extern crate core;

use std::env;
use std::fs::File;
use std::process::exit;
use std::str::FromStr;
use home::home_dir;

use serde_json::{Map, Value};
use wol::MacAddr;

const CONFIG_FILENAME: &str = ".wake.json";

fn main() {
    let mut config_file_absolute_path = home_dir().unwrap();
    config_file_absolute_path.push(CONFIG_FILENAME);

    //println!("Configuration file: {}", config_file_absolute_path.display());

    let config_file = File::open(config_file_absolute_path.as_path())
        .unwrap_or_else(|e| {
            eprintln!("Failed to read configuration file {}: {}", config_file_absolute_path.display(), e);
            exit(1);
        });

    let config_object: Value = serde_json::from_reader(config_file)
        .unwrap_or_else(|e| {
            eprintln!("Could not parse configuration file JSON: {}", e);
            exit(1);
        });

    let computer_name = env::args().nth(1)
        .unwrap_or_else(|| {
            eprintln!("Usage: {} <computerName>", env::current_exe().unwrap().file_name().unwrap().to_str().unwrap());
            exit(1);
        });

    let computer: &Map<String, Value> = config_object[&computer_name].as_object()
        .unwrap_or_else(|| {
            eprintln!("Configuration object in JSON file {} does not contain a {} key", config_file_absolute_path.display(), &computer_name);
            exit(1);
        });

    println!("Waking up {}...", &computer["label"].as_str().get_or_insert(&computer_name));

    for mac_address_entry in computer["macAddresses"].as_array().unwrap() {
        let mac_address_string = mac_address_entry.as_str()
            .unwrap_or_else(|| {
                eprintln!("entry in macAddresses array is not a string: {}", mac_address_entry);
                exit(1);
            });

        let mac_address: MacAddr = MacAddr::from_str(mac_address_string)
            .unwrap_or_else(|e| {
                eprintln!("Invalid MAC address {}: {}", &mac_address_string, e);
                exit(1);
            });

        wol::send_wol(mac_address, None, None).unwrap();
        println!("Sent wake-on-LAN packet to {}", &mac_address_string);
    }

}