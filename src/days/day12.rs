use crate::types::AppState;
use axum::{
    extract::{self, Path, State},
    Json,
};
use chrono::{DateTime, Datelike, Utc};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use std::{collections::HashMap, time::UNIX_EPOCH};
use ulid::Ulid;
use uuid::Builder;

#[derive(Serialize)]
pub struct ParseUlidsResponse {
    #[serde(rename = "christmas eve")]
    christmas_eve: u16,
    weekday: u16,
    #[serde(rename = "in the future")]
    future: u16,
    #[serde(rename = "LSB is 1")]
    lsb_is_1: u16,
}

#[derive(Deserialize)]
pub struct WeekdayParam {
    weekday: u8,
}

pub async fn store_packet(
    Path(params): Path<HashMap<String, String>>,
    State(state): State<AppState>,
) {
    let packet = params
        .get("packet")
        .expect("Packet string expected")
        .to_owned();

    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("current_time expected")
        .as_secs();

    let mut data = state
        .record_last_updated
        .lock()
        .expect("mutex was poisoned");
    let _ = *data.entry(packet.clone()).or_insert(current_time);

    println!("adding {} at {}", packet, current_time);
}

pub async fn load_packet(
    Path(params): Path<HashMap<String, String>>,
    State(state): State<AppState>,
) -> String {
    let packet_query = params.get("packet").expect("Packet string expected");

    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("current_time expected")
        .as_secs();

    let data = state
        .record_last_updated
        .lock()
        .expect("mutex was poisoned");
    let packet_time = data.get(packet_query).expect("packet id not found");

    format!("{}", current_time - packet_time)
}

pub async fn ulids_to_uuids(
    extract::Json(payload): extract::Json<Vec<String>>,
) -> Json<Vec<String>> {
    Json(
        payload
            .iter()
            .map(|ulid| ulid_to_uuid(ulid))
            .rev()
            .collect::<Vec<String>>(),
    )
}

pub async fn parse_ulids(
    Path(WeekdayParam { weekday }): Path<WeekdayParam>,
    Json(payload): Json<Vec<String>>,
) -> Json<ParseUlidsResponse> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let target_weekday = chrono::Weekday::try_from(weekday)
        .unwrap_or_else(|_| panic!("Invalid weekday: {}", weekday));

    let counts = payload
        .into_iter()
        .filter_map(|s| Ulid::from_string(&s).ok())
        .fold(
            ParseUlidsResponse {
                christmas_eve: 0,
                weekday: 0,
                future: 0,
                lsb_is_1: 0,
            },
            |mut acc, ulid| {
                let datetime: DateTime<Utc> = ulid.datetime().into();
                if datetime.month() == 12 && datetime.day() == 24 {
                    acc.christmas_eve += 1;
                }

                if ulid.to_bytes()[15] & 1 == 1 {
                    acc.lsb_is_1 += 1;
                }

                if (ulid.timestamp_ms() / 1000) > now {
                    acc.future += 1;
                }

                if datetime.weekday() == target_weekday {
                    acc.weekday += 1;
                }

                acc
            },
        );

    Json(counts)
}

fn ulid_to_uuid(s: &str) -> String {
    let ulid = Ulid::from_string(s).unwrap();
    let uuid = Builder::from_bytes(ulid.to_bytes());

    uuid.as_uuid().to_string()
}
