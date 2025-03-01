use std::error::Error;
use std::fs;

pub struct Config<'a> {
    query: String,
    file_pathes: &'a [String],
}

impl Config<'_> {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() <= 2 {
            return Err("missing arguments");
        }

        let query = args[1].clone().to_lowercase();
        let file_pathes = &args[2..];
        Ok(Config {query, file_pathes})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    for file_path in config.file_pathes {
        let contents = fs::read_to_string(file_path)?;
        
        let find = search(&config.query, &contents);
        if find.len() > 0 {
            println!("File {file_path} {count}", count = find.len());
            println!("{find:#?}\n");
        }
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut search_result = vec![];
    for line in contents.lines() {
        if line.to_lowercase().contains(query) {
            search_result.push(line);
        }
    }
    search_result
}
