use std::{env, error::Error, fs, process};

fn main() {
    println!("minigrep start");

    let args: Vec<String> = env::args().collect();

    assert!(args.len() >= 1);
    assert!(args[0].contains("minigrep"));

    let config = Config::build(&args).unwrap_or_else(|err| {
        // println!("Problem parsing arguments: {err}");
        // eprintln! 宏：标准错误输出
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Search for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        // println!("Application error: {e}");
        eprintln!("Application error: {e}");
        process::exit(1);
    }

    // dbg!(args);
}

struct Config {
    query: String,
    filename: String,
    ignore_case: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            filename,
            ignore_case,
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }
    Ok(())
}

fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
