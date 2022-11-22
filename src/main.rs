extern crate core;

use std::env;
use std::fs::File;
use std::str::FromStr;

use serde_json::{Map, Value};

const CONFIG_FILENAME: &str = ".wake.json";

fn main() {
    let mut config_file_absolute_path = env::home_dir().unwrap();
    config_file_absolute_path.push(CONFIG_FILENAME);
    println!("Config file = {}", config_file_absolute_path.display());

    let config_file = File::open(config_file_absolute_path.as_path())
        .unwrap_or_else(|e| { panic!("Failed to read configuration file {}: {}", config_file_absolute_path.display(), e) });

    let config_object: Value = serde_json::from_reader(config_file)
        .unwrap_or_else(|e| { panic!("Could not parse configuration file JSON: {}", e) });

    let computer_name = env::args().nth(1)
        .unwrap_or_else(|| { panic!("First argument must be a computer name") });

    let computer: &Map<String, Value> = config_object[&computer_name].as_object()
        .unwrap_or_else(|| { panic!("Configuration object in JSON file {} does not contain a {} key", config_file_absolute_path.display(), &computer_name) });

    println!("Waking up {}", &computer["label"].as_str().get_or_insert(&computer_name));

    for str in computer["macAddresses"].as_array().unwrap() {
        let mac_address_string = str.as_str().unwrap();

        let mac_address: wol::MacAddr = wol::MacAddr::from_str(mac_address_string)
            .unwrap_or_else(|e| { panic!("Invalid MAC address {}: {}", &mac_address_string, e) });

        wol::send_wol(mac_address, None, None).unwrap();
        println!("Sent wake-on-LAN packet to {}", &mac_address_string);
    }
}