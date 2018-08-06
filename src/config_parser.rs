use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub fn parse() -> HashMap<String, bool> {
    let mut config_map = HashMap::new();
    let mut config_file = include_str!("default-config.ini");

    for line in config_file.lines() {
        if !line.contains('=') || line.starts_with('#') {
            continue;
        }

        let mut iterator = line.split(" = ");
        let option = iterator.next().unwrap();
        let value = match iterator.next().unwrap() {
            "yes" => true,
            "no" => false,
            e => panic!("value i, config not supported: {}", e),
        };

        config_map.insert(option.to_string(), value);
    }

    config_map
}
