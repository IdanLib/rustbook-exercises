use std::error::Error;
use std::fs;
use std::env; 

pub struct Config  {
    pub query:String,
    pub filename: String,
    pub ignore_case: bool
}

//Option 1
// impl Config {
//     pub fn build (args: &Vec<String>) -> Result<Config, &str> { //implementing a constuctor
//         if args.len() <3 {
//             return Err("Not enough arguments!");
//         }

//         return Ok(Config {
//             query: args[1].to_string(), 
//             filename: args[2].to_string(), 
//             ignore_case: env::var("IGNORE_CASE").is_ok()
//         })
//     }
// }

// Option 2
impl Config {
    pub fn build (mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> { 
        //Here, instead of handling a vector with vector-based logic, checking its length etc., we are treating an iterator, using iterator-based methods and iterator methods.

        args.next(); //moving the iterator once forward - the first argument is the name of the program, which we want to ignore. 

        let query: String = match args.next() {
            Some(qr) => qr,
            None => return Err("No query specified: What do you want to search?")
        };

        let filename: String = match args.next() {
            Some(f_name) => f_name,
            None => return Err("No filename included: Where do you want to search?")
        };

        return Ok(Config {
            query,
            filename, 
            ignore_case: env::var("IGNORE_CASE").is_ok()
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    if config.ignore_case {
        println!("{:?}", search_case_insensitive(&config.query, &contents));
    } else {
        println!("{:?}", search(&config.query, &contents));
    }

    return Ok(());
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    
    //1. Using for loop

    // let mut results = vec![];

    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }

    // return results;

    //2. Using filer()
    return contents
        .lines()
        .filter(|line| {
            line.contains(query)
        })
        .collect();
}


fn search_case_insensitive<'a>(query: &str, contents:&'a str) -> Vec<&'a str> {

    let lower_case_query = query.to_lowercase();

    return contents
        .lines()
        .filter(|line| {
            line.to_lowercase().contains(&lower_case_query)
        })
        .collect();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() { //should return line in which query appears
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three."
        ;

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape."
        ;
        
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query= "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }

}
