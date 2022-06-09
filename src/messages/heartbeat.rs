extern crate serde;
use serde::{Deserialize, Serialize};
extern crate chrono;
use chrono::{DateTime, Utc};
use chrono::serde::ts_nanoseconds;
extern crate log;
extern crate paho_mqtt as mqtt;

pub const TOPIC: &str = "heartbeat";

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
    id: String,
    #[serde(with="ts_nanoseconds")]
    time: DateTime<Utc>
}

impl Payload {

    pub fn new(id: String, time: DateTime<Utc>) -> Payload {
        Payload { id, time }
    }
}