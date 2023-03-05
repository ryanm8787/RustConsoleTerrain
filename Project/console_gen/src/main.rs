mod terrgen;
mod server;

use std::fs;
use crate::terrgen::terrgen::Generator;
use crate::server::server::Server;
use futures::executor::block_on;

use tokio;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;

fn to_my_type(value: serde_json::Value) -> i32 {
    serde_json::from_value(value).unwrap()
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";

        let response = format!(
            "{status_line}"
        );

        stream.write_all(response.as_bytes()).unwrap();
    }
}

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
        { ip_addr_target : String::from("172.17.0.3"), port_target : String::from("5000") }; 
    
    tokio::runtime::Builder::new_current_thread()
    .enable_all()
    .build()
    .unwrap()
    .block_on(
        client.perform_post_to_flask(ans, to_my_type(json["width"].clone()), to_my_type(json["height"].clone()))
    );

    // just listen on this port for now 
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();

    for stream in listener.incoming() {
        println!("it fucking wimdy!\n");
        let stream = stream.unwrap();

        // handle_connection(stream);
    }
}