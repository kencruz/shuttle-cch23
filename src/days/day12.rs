use std::collections::HashMap;
use std::time::SystemTime;

use crate::types::AppState;
use axum::{
    extract::{self, Path, State},
    Json,
};
use ulid::Ulid;
use uuid::Builder;

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

fn ulid_to_uuid(s: &str) -> String {
    let ulid = Ulid::from_string(s).unwrap();
    let uuid = Builder::from_bytes(ulid.to_bytes());

    uuid.as_uuid().to_string()
}
