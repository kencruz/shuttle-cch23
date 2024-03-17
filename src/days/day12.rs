use std::time::SystemTime;
use std::collections::HashMap;

use axum::extract::{Path, State};
use crate::types::AppState;

pub async fn store_packet(Path(params): Path<HashMap<String, String>>, State(state): State<AppState>) {
    let packet = params
        .get("packet").expect("Packet string expected").to_owned();

    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("current_time expected").as_secs();

    let mut data = state.record_last_updated.lock().expect("mutex was poisoned");
    let _ = *data.entry(packet.clone()).or_insert(current_time);


    println!("adding {} at {}", packet, current_time);
}

pub async fn load_packet(Path(params): Path<HashMap<String, String>>, State(state): State<AppState>) -> String {
    let packet_query = params
        .get("packet").expect("Packet string expected");

    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("current_time expected").as_secs();

    let data = state.record_last_updated.lock().expect("mutex was poisoned");
    let packet_time = data.get(packet_query).expect("packet id not found");


    format!("{}", current_time - packet_time)
}
