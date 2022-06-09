use std::process;
use std::time::Duration;
extern crate serde_json;
use serde_json::from_slice;
extern crate log;
use log::{error, info};
extern crate paho_mqtt as mqtt;
use std::thread::sleep;
use crate::messages::connexion::Payload as ConnexionPayload;
use crate::messages::heartbeat::Payload as HeartbeatPayload;

pub fn consume_connexion(msg: mqtt::Message) {
    match from_slice::<ConnexionPayload>(msg.payload()) {
        Ok(m) => {
            info!("Connexion: {:?}", m);
        },
        Err(e) => {
            error!("Wrong message format: {:?}", e);
        }
    }
}

pub fn consume_heartbeat(msg: mqtt::Message) {
    match from_slice::<HeartbeatPayload>(msg.payload()) {
        Ok(m) => {
            info!("Heartbeat: {:?}", m);
        },
        Err(e) => {
            error!("Wrong message format: {:?}", e);
        }
    }
}

fn try_reconnect(cli: &mqtt::Client) -> bool {
    println!("Connection lost. Waiting to retry connection");
    for _ in 0..12 {
        sleep(Duration::from_millis(5000));
        if cli.reconnect().is_ok() {
            println!("Successfully reconnected");
            return true;
        }
    }
    println!("Unable to reconnect after several attempts.");
    false
}

pub fn run() {
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri("tcp://test.mosquitto.org:1883")
        .finalize();
    
    let client = mqtt::Client::new(create_opts).unwrap_or_else(|err| {
        error!("Error creating the client: {:?}", err);
        process::exit(1)
    });

    let ctrlc_client = client.clone();
    ctrlc::set_handler(move || { ctrlc_client.stop_consuming(); })
        .expect("Error setting Ctrl-C handler");

    let rx = client.start_consuming();
    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(true)
        .finalize();
    
    let subscriptions = ["connexion", "heartbeat"];
    let qos = [1, 1];

    info!("Connecting to the MQTT broker...");
    match client.connect(conn_opts) {
        Ok(response) => {
            if let Some(connexion_response) = response.connect_response() {
                info!(
                    "Connected to '{}' with MQTT version {}",
                    connexion_response.server_uri, connexion_response.mqtt_version
                );
                if connexion_response.session_present {
                    info!("With client session already present in broker.");
                } else {
                    info!("Subscribing to topics with requested QoS: {:?}", qos);

                    client.subscribe_many(&subscriptions, &qos)
                        .and_then(|response| {
                            response.subscribe_many_response()
                                .ok_or(mqtt::Error::General("Bad response."))
                        })
                        .and_then(|vqos| {
                            info!("QoS granted: {:?}", vqos);
                            Ok(())
                        })
                        .unwrap_or_else(|err| {
                            error!("Error subscribing to topics: {:?}", err);
                            client.disconnect(None).unwrap();
                        });
                }
            }
        }
        Err(e) => {
            error!("Error connecting to the broker: {:?}", e);
        }
    }

    info!("Waiting for messages on topics {:?}", subscriptions);
    for msg in rx.iter() {
        if let Some(msg) = msg {
            // Do something based on the topic
            match msg.topic() {
                "connexion" => {
                    consume_connexion(msg)
                },
                "heartbeat" => {
                    consume_heartbeat(msg)
                },
                &_ => {}
            }
        } else if client.is_connected() || !try_reconnect(&client) {
            break;
        }
    }

    if client.is_connected() {
        info!("\nDisconnecting...");
        client.unsubscribe_many(&subscriptions).unwrap();
        client.disconnect(None).unwrap();
    }
    info!("Exiting");
}