// use core::panic;
use std::{env, println, process};
use std::error::Error;

use std::fs;

use rust_cli::{search, search_case_insensitive};

pub struct Config{
    pub query: String,
    pub file: String,
    pub ignore_case: bool,
}

impl Config{
    fn new_cfg(mut args: impl Iterator<Item = String>)->Result<Config, &'static str>{

        args.next();

        let query: String = match args.next(){
            Some(arg)=>arg,
            None=>return Err("Didn't get a query string")
        };

        let file: String = match args.next(){
            Some(arg)=>arg,
            None=>return Err("Didn't get a file string")
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {query: query, file: file, ignore_case:ignore_case})
    }
}

fn run(config:Config)->Result<(), Box<dyn Error>>{
    let contents =  fs::read_to_string(config.file)?;
    // println!("With text:\n{contents}");

    let result = if config.ignore_case{
        search_case_insensitive(&config.query, &contents)   
    } else{
        search(&config.query, &contents)
    };


    for line in result {
        println!("{line}");
    }

    Ok(())
}

fn main() {
    // let args: Vec<String> = env::args().collect();
    // dbg!(&args);

    // let query: &String = &args[1];
    // let file = &args[2];

    let cfg = Config::new_cfg(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    
    // println!("Searching for '{}' in file {}", cfg.query, cfg.file);

    // let contents = file_to_string(&cfg.file);

    if let Err(e) = run(cfg){
        eprintln!("Application Error: {e}");
        process::exit(1);
    }

}
