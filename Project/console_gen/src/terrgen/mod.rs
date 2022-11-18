pub mod terrgen {

use rand::Rng;
use serde_json::Result;

pub struct Generator {
    pub floor : i32,
    pub ceiling : i32,
    pub width : i32,
    pub height : i32,
    pub current_gradient : i32,
    pub map : String,
}

impl Generator {

    fn update_gradient(&mut self)
    {
        let mut rng = rand::thread_rng();
            
        self.current_gradient = rng.gen_range(0..2);
    }

    fn generate_empty_string(&mut self)
    {
        let mut current_x_ct : i32 = 0;
        let mut current_y_ct : i32 = 0;
        
        loop {
            if current_x_ct == self.width
            {
                current_x_ct = 0;
                current_y_ct += 1;
                self.map.push_str("\n");
                if current_y_ct == self.height 
                {
                    break;
                }
            }
            
            self.map.push_str(" ");
            current_x_ct += 1;
        };

        println!("map: \n{}", self.map)
    }


    pub fn generate_terrain(&mut self)
    {
        self.generate_empty_string();
    }
}

} // terrgen