// use core::panic;
use std::{env, println, process};
use std::error::Error;

use std::fs;

use rust_cli::search;

struct Config{
    query: String,
    file: String
}

impl Config{
    fn new_cfg(args: &[String])->Result<Config, &'static str>{
        if args.len() < 3{
            return Err("Not Enough Arguments");
        }
        Ok(Config { query: args[1].clone(), file: args[2].clone() })
    }
}

fn run(config:Config)->Result<(), Box<dyn Error>>{
    let contents =  fs::read_to_string(config.file)?;
    // println!("With text:\n{contents}");

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);

    // let query: &String = &args[1];
    // let file = &args[2];

    let cfg = Config::new_cfg(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    
    // println!("Searching for '{}' in file {}", cfg.query, cfg.file);

    // let contents = file_to_string(&cfg.file);

    if let Err(e) = run(cfg){
        println!("Application Error: {e}");
        process::exit(1);
    }

}
