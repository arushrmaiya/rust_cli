use std::{vec};


pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    // let lines: Vec<&str> = contents.lines().collect();
    // let mut results = vec![];
    // for line in lines{
    //     if line.contains(query){
    //         results.push(line);
    //     }
    // }
    // results
    contents
    .lines()
    .filter(|line| line.contains(query))
    .collect()
}

pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let lines: Vec<&str> = contents.lines().collect();
    let mut results = vec![];
    for line in lines{
        if line.to_lowercase().contains(&query.to_lowercase()){
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]

    fn case_sensitive(){

        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust Me.";
        assert_eq!(vec!["Rust:", "Trust Me."], search_case_insensitive(query, contents));


    }

}
