use std::env;
extern crate log;
extern crate paho_mqtt as mqtt;
use embagent::{cloud,edge};

fn main() {
    env_logger::init();
    
    let action = env::args()
        .nth(1)
        .unwrap_or_else(|| "edge".to_string());
    
    match &action[..] {
        "edge" => {
            edge::agent::Agent::new("banana".to_string()).run();
        },
        "cloud" => {
            cloud::agent::run();
        },
        &_ => {}
    }

}
