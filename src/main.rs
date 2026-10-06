use core::panic;
use std::{dbg, env};

use std::fs;

struct Config{
    query: String,
    file: String
}

impl Config{
    fn new_cfg(args: &[String])->Self{
        if args.len() < 3{
            panic!("Not Enough Arguments");
        }
        Config { query: args[1].clone(), file: args[2].clone() }
    }
}

fn file_to_string(filename:&str)->String{
    return fs::read_to_string(filename).expect("NO SUCH FILE {file}.");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);

    // let query: &String = &args[1];
    // let file = &args[2];

    let cfg = Config::new_cfg(&args);
    
    println!("Searching for {}", cfg.query);
    println!("In file {}", cfg.file);

    let contents = file_to_string(&cfg.file);


    println!("With text:\n{contents}");
}
