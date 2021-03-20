use std::collections::HashMap;

static CONFIG_FILE: &str = include_str!("default-config.ini");

pub fn parse() -> HashMap<&'static str, bool> {
    let mut config_map = HashMap::new();

    let mut parse = |line: &'static str| -> Option<()> {
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

        config_map.insert(option, value);
        None
    };

    for line in CONFIG_FILE.lines() {
        parse(line);
    }

    config_map
}
