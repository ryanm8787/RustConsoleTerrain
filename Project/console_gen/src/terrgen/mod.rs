mod terrgen {

use rand::Rng;

pub struct Generator {
    max_height : i32,
    max_width : i32,
    current_gradient : i32,
    map : String,
}

impl Generator {

    fn update_gradient(&mut self)
    {
        let mut rng = rand::thread_rng();
            
        self.current_gradient = rng.gen_range(0..2);
        println!("current gradeint {}", self.current_gradient);
    }

}

} // terrgen