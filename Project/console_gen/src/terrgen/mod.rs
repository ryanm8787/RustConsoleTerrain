pub mod terrgen {

use rand::Rng;

pub struct Generator {
    pub floor : i32,
    pub ceiling : i32,
    pub width : i32,
    pub height : i32,
    pub current_gradient : i32,
    pub map : Vec<Vec<String>>,
    pub indx_map : Vec<usize>,
}

impl Generator {

    fn update_gradient(&mut self)
    {
        let mut rng = rand::thread_rng();
            
        self.current_gradient = rng.gen_range(-1..2);
    }

    fn generate_empty_string(&mut self)
    {
        let mut vec_counter : i32 = 0;
    
        // create vectors
        loop 
        {
            if vec_counter == self.height
            {
                break;
            }
            let mut new_vec : Vec<String> = Vec::new();

            let mut vec_element_counter : i32 = 0;
            loop 
            {
                if vec_element_counter == self.width
                {
                    break;
                }
                vec_element_counter += 1;
                new_vec.push(" ".to_string());
            }
            new_vec.push("\n".to_string());

            self.map.push(new_vec);
            vec_counter += 1;
        };
    }

    // TODO
    fn fill_empty_terrain(&mut self)
    {
        let mut x_counter : usize = 0;
        
        loop {
            if x_counter == self.width as usize
            {
                break;
            }
            let mut y_counter : i32 = 0;

            if (self.indx_map[x_counter] as i32) < (self.height as i32) - 1
            {
                y_counter = (self.indx_map[x_counter] as i32) + 1;
            }

            loop 
            {
                if y_counter >= self.height
                {
                    break;
                }

                self.map[y_counter as usize][x_counter].clear();
                self.map[y_counter as usize][x_counter].insert_str(0, "0");

                y_counter += 1; 
            }

            x_counter += 1;
        };
    }
    
    fn generate_floor(&mut self)
    {
        self.indx_map = Vec::new();
        let mut x_counter : usize = 0;
        let mut height_current : i32 = self.ceiling;
        loop {
            if x_counter == self.width as usize
            {
                break;
            }
            
            self.update_gradient();

            height_current += self.current_gradient;
            if height_current > self.ceiling
            {
                height_current = self.ceiling;   
            }
            else if height_current < self.floor
            {
                height_current = self.floor;
            }

            self.map[height_current as usize][x_counter].clear();
            self.map[height_current as usize][x_counter].insert_str(0, "1");
            self.indx_map.push(height_current as usize);
            x_counter += 1;
        };
    }

    fn convert_map_to_str(&mut self) -> String
    {
        let mut str_map = String::from("");

        let mut counter_x : usize = 0; 
        let mut counter_y : usize = 0;
        loop
        {
            if counter_x >= self.map[counter_y].len()
            {
                counter_y += 1;
                counter_x = 0;
            }

            if counter_y == self.map.len()
            {
                break;
            }

            str_map.push_str(&self.map[counter_y][counter_x]); 
            counter_x += 1;
        };
        return str_map;
    }

    pub fn generate_terrain(&mut self) -> String
    {
        self.generate_empty_string();
        self.generate_floor();
        self.fill_empty_terrain();
        let mut ans = self.convert_map_to_str();
        return ans;
    }
}

} // terrgen