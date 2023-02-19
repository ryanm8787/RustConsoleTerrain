
pub mod server {
    use reqwest;
    use std::collections::HashMap;

    pub struct Server {
        pub ip_addr_target : String,
        pub port_target : String
    }
    impl Server 
    {
        pub async fn perform_post_to_flask(&mut self, send_str : String, width : i32, height : i32) -> Result<(), Box<dyn std::error::Error>>
        {   
            let mut map = HashMap::new();
            let len : usize = send_str.len();

            map.insert("data", send_str );
            map.insert("width", width.to_string());
            map.insert("height", height.to_string());
            let client = reqwest::Client::new();

            print!("\nSending.....\n");
            
            let mut post_req = format!("http://{}:{}/data", self.ip_addr_target, self.port_target); 
            
            print!("\nSending: W:{} H:{}\n", width.to_string(), height.to_string());
            print!("String size: {}", len);
            let res = client.post(post_req)
                .json(&map)
                .send()
                .await?;
            Ok(())
        }
    } // server
}