use std::collections::HashMap;

pub mod config_parser;

pub fn parse_config() -> (
    Box<HashMap<String, config_parser::config_structs::TaskParams>>,
    Box<HashMap<String, Vec<String>>>,
) {
    return config_parser::parse();
}
