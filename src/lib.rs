use std::env;
use std::fs;
use std::error::Error;

#[derive(Debug)]
pub struct Config {
	pub search_query: String,
	pub file_path: String,
	pub ignore_case: bool,
}

impl Config {
		pub fn parse_config(args: &[String])-> Result<Config, &'static str> {
		if args.len() != 3 {
			return Err("length of args is expected to be 3");
		} 	 
		let search_query = args[1].clone();
	    let file_path = args[2].clone();
	    
	    let ignore_case_str = env::var("IGNORE_CASE").unwrap_or_default();
	    let ignore_case;
	    if ignore_case_str == "true" || 
	    ignore_case_str == "TRUE" ||
	    ignore_case_str == "1"
	    {
	    	ignore_case = true;
	    } else {
	    	ignore_case = false;
	    }

	    let config = Config {
	    	search_query: search_query, 
	    	file_path: file_path,
	    	ignore_case: ignore_case,
	    };
	    return Ok(config);
	}
}

pub fn search_content(config: Config) -> Result<(), Box<dyn Error>> {
	//println!("SearchQuery: {}, Searching in File: {}", config.search_query, config.file_path);

	let contents = fs::read_to_string(config.file_path)
		.expect("Should be able to read the file contents");

	let results;
	if config.ignore_case {
	  results = search_case_insensitive(&config.search_query, &contents);
	}  else {
	  results = search_case_sensitive(&config.search_query, &contents);	
	}  	
	for line in results {
		println!("{}", line);
	}
	
	Ok(())
}

pub fn search_case_sensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
    	if line.contains(query) {
    		results.push(line);
    	}
    }
    results
}

pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query_lowercase = query.to_lowercase();
    for line in contents.lines() {
    	if line.to_lowercase().contains(&query_lowercase) {
    		results.push(line);
    	}
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "productive";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Productive day.";

        assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}