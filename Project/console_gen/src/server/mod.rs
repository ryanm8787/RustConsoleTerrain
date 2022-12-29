
pub mod server {
    use reqwest;
    use std::collections::HashMap;

    pub struct Server {
        pub ip_addr_target : String,
        pub port_target : String
    }
    impl Server 
    {
        pub async fn perform_post_to_flask(&mut self) -> Result<(), Box<dyn std::error::Error>>
        {   
            let mut map = HashMap::new();
            map.insert("lang", "rust");
            map.insert("body", "json");

            let client = reqwest::Client::new();
            let res = client.post("http://0.0.0.0:5000/data")
                .json(&map)
                .send()
                .await?;
            
            Ok(())
        }
    } // server
}