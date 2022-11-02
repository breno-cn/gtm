use std::{fs};

use serde::Deserialize;
use serde_yaml::Value;

pub fn read_from_file(filepath: String) -> Value {
    let file = fs::File::open(filepath).unwrap();

    let data = serde_yaml::Deserializer::from_reader(file);

    Value::deserialize(data).unwrap()
}
