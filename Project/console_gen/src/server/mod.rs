
pub mod server {
    use reqwest;
    use std::collections::HashMap;

    pub struct Server {
        pub ip_addr_target : String,
        pub port_target : String
    }
    impl Server 
    {
        pub async fn perform_post_to_flask(&mut self, send_str : String) -> Result<(), Box<dyn std::error::Error>>
        {   
            let mut map = HashMap::new();
            map.insert("data", send_str );
            let client = reqwest::Client::new();

            print!("\nSending.....\n");
            
            let mut post_req = format!("http://{}:{}/data", self.ip_addr_target, self.port_target); 
            
            print!("\nSending: {}\n", post_req);
            let res = client.post(post_req)
                .json(&map)
                .send()
                .await?;
            Ok(())
        }
    } // server
}