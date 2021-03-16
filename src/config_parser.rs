use std::collections::HashMap;

pub fn parse() -> HashMap<String, bool> {
    let mut config_map = HashMap::new();
    let config_file = include_str!("default-config.ini");

    let mut parse = |line: &str| -> Option<()> {
        if !line.contains('=') || line.starts_with('#') {
            return None;
        }

        let mut iterator = line.split(" = ");
        let option = iterator.next()?;
        let value = match iterator.next()? {
            "yes" => true,
            "no" => false,
            _ => return None,
        };

        config_map.insert(option.to_string(), value);
        None
    };

    for line in config_file.lines() {
        parse(line);
    }

    config_map
}
