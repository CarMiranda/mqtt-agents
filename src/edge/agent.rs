use std::process;
use std::time::Duration;
extern crate log;
use log::{error, info};
extern crate paho_mqtt as mqtt;
use std::thread::sleep;
use chrono::Utc;
use crate::messages::connexion::Payload as ConnexionPayload;
use crate::messages::heartbeat::Payload as HeartbeatPayload;
extern crate serde_json;
use serde_json::to_vec;

pub struct Agent {
    id: String,
    client: mqtt::Client
}

impl Agent {

    pub fn new(id: String) -> Agent {
        let client = mqtt::Client::new("tcp://test.mosquitto.org:1883").unwrap_or_else(|err| {
            error!("Error creating the client: {:?}", err);
            process::exit(1)
        });
    
        let conn_opts = mqtt::ConnectOptionsBuilder::new()
            .keep_alive_interval(Duration::from_secs(20))
            .clean_session(true)
            .finalize();
        
        if let Err(e) = client.connect(conn_opts) {
            error!("Unable to connect: {:?}", e);
            process::exit(1)
        }

        Agent { id, client }
    }

    fn publish_connexion(&self, payload: &ConnexionPayload) {
        let message = mqtt::Message::new("connexion", to_vec(payload).unwrap(), 2);
        let tok = self.client.publish(message);
        if let Err(e) = tok {
            error!("Error publishing message: {:?}", e);
        } else {
            info!("Succesfully published connexion message.");
        }
    }

    fn publish_heartbeat(&self, payload: &HeartbeatPayload) {
        let message = mqtt::Message::new("heartbeat", to_vec(payload).unwrap(), 2);
        let tok = self.client.publish(message);
        if let Err(e) = tok {
            error!("Error publishing message: {:?}", e);
        } else {
            info!("Succesfully published heartbeat message.");
        }
    }

    pub fn run(&self) -> () {
        let ctrlc_client = self.client.clone();
        ctrlc::set_handler(move || { ctrlc_client.disconnect(None).unwrap(); })
            .expect("Error setting Ctrl-C handler");

        self.publish_connexion(&ConnexionPayload::new(self.id.to_string()));
        for _ in 0..10 {
            self.publish_heartbeat(&HeartbeatPayload::new(self.id.to_string(), Utc::now()));
            sleep(Duration::from_secs(5));
        }
        self.client.disconnect(None).unwrap();
    }
}