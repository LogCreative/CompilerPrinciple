use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // File for input
    let filename = &args[1];
    // contents of the file
    let contents = fs::read_to_string(filename).expect("No such file."); 

    println!("Text in file:\n{}",contents);
}
