use std::{print, println, vec};


pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let lines: Vec<&str> = contents.lines().collect();
    let mut results = vec![];
    for line in lines{
        if line.contains(query){
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

}
