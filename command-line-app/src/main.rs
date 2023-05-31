use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    impl Config {
        fn build(args: &[String]) -> Result<Config, &'static str> {
            if args.len() < 3 {
                return Err("No enogh arguments!");
            }
            let query = args[1].clone();
            let file_path = args[2].clone();

            Ok(Config { query, file_path })
        }
    }

    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        println!("err: {}", err);
        process::exit(-1);
    });
    println!("query word: {}", config.query);
    println!("file name: {}", config.file_path);

    fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(config.file_path);
        println!("results\n{:?}", contents.unwrap());
        Ok(())
    }

    // run(config).unwrap_or_else(|err| {
    //     println!("{:?}", err);
    //     process::exit(-1);
    // });

    if let Err(e) = run(config) {
        println!("{:?}", e);
        process::exit(-1);
    }
}

struct Config {
    query: String,
    file_path: String,
}
// For now, just know that Box<dyn Error> means the function will return a type
// that implements the Error trait, 
// but we don’t have to specify what particular type the return value will be.

fn parse_config(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
        return Err("No enogh arguments!");
    }
    let query = args[1].clone();
    let file_path = args[2].clone();

    Ok(Config { query, file_path })
}
