use std::io;

pub mod terrgen;

fn main() {
    println!("Please enter a a width:");   

    let mut input_string_width = String::new(); 
    
    io::stdin()
        .read_line(&mut input_string_width)
        .expect("Failed to read line");

    // NOTE: this trim is needed to remove whitespace from console.
    let input_width : i32 = input_string_width.trim().parse().expect("Failed to convert str->int.");

    println!("Please enter a a height:");   

    let mut input_string_height = String::new(); 

    io::stdin()
        .read_line(&mut input_string_height)
        .expect("Failed to read line");

    let input_height : i32 = input_string_height.trim().parse().expect("Failed to convert str->int.");
    println!("{}, {}", input_width, input_height);   
}