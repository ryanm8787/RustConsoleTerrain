mod terrgen;
mod server;

use std::fs;
use crate::terrgen::terrgen::Generator;
use crate::server::server::Server;
use futures::executor::block_on;
use tokio;

fn to_my_type(value: serde_json::Value) -> i32 {
    serde_json::from_value(value).unwrap()
}

// this will run an example demo
fn main() {
    let json_file_path = String::from("/home/rust/config/config.json"); 

    let data = fs::read_to_string(&json_file_path)
        .expect("Unable to read file");

    let json: serde_json::Value = serde_json::from_str(&data)
        .expect("JSON does not have correct format.");

    let mut generator = Generator 
        {ceiling : to_my_type(json["ceiling"].clone()), width : to_my_type(json["width"].clone()), height : to_my_type(json["height"].clone()), 
        floor : to_my_type(json["floor"].clone()), current_gradient : 0, map : Vec::new(), indx_map : Vec::new()}; 

    let mut ans = generator.generate_terrain();

    // TODO: config this port
    let mut client = Server 
        { ip_addr_target : String::from("172.17.0.2"), port_target : String::from("5000") }; 
    
    tokio::runtime::Builder::new_current_thread()
    .enable_all()
    .build()
    .unwrap()
    .block_on(
        client.perform_post_to_flask(ans)
    );
}