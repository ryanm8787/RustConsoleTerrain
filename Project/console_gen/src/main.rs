use std::io;

use std::collections::HashMap;

fn main() {
    println!("Please enter a a width:");   

    let mut input_string = String::new(); 
    
    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");

    // NOTE: this trim is needed to remove whitespace from console.
    let input_width : i32 = input_string.trim().parse().expect("");

    // generate terrian 
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);


}