use std::env;
use std::fs;

fn main() {
    
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("in file {file_path}");

    let contents: String = fs::read_to_string(file_path).expect("Should have been able to read the file");
    println!("With text: \n{contents}");

    let mut n = 0;

    for line in contents.lines() { 
        for word in line.split(' ') {
            if word == query {
                n += 1;
            }
        }   
    }

    println!("Number of {query} in {file_path} is: {n}");
}
