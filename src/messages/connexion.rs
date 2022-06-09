extern crate serde;
use serde::{Deserialize, Serialize};
extern crate log;
extern crate paho_mqtt as mqtt;

pub const TOPIC: &str = "connexion";

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
    id: String
}

impl Payload {
    pub fn new(id: String) -> Payload {
        Payload { id }
    }
}