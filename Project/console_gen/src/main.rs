mod terrgen;

use std::fs;
use crate::terrgen::terrgen::Generator;

fn to_my_type(value: serde_json::Value) -> i32 {
    serde_json::from_value(value).unwrap()
}

fn main() {
    let json_file_path = String::from("/home/gen_terr/Project/console_gen/config/config.json"); 

    let data = fs::read_to_string(&json_file_path)
        .expect("Unable to read file");

    let json: serde_json::Value = serde_json::from_str(&data)
        .expect("JSON does not have correct format.");

    let mut generator = Generator 
             {ceiling : to_my_type(json["ceiling"].clone()), width : to_my_type(json["width"].clone()), height : to_my_type(json["height"].clone()), 
             floor : to_my_type(json["floor"].clone()), current_gradient : 0, map : String::from("")}; 

    generator.generate_terrain();
}