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


    pub fn generate_terrain(&mut self)
    {

    }
}

} // terrgen