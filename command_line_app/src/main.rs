use commandapp::*;
use std::env;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        println!("err: {}", err);
        process::exit(-1);
    });
    println!("query word: {}", config.query);
    println!("file name: {}", config.file_path);

    // run(config).unwrap_or_else(|err| {
    //     println!("{:?}", err);
    //     process::exit(-1);
    // });

    if let Err(e) = run(config) {
        // println!("{:?}", e);
        eprintln!("{}", e);
        process::exit(-1);

    }

    // let ignore_case = env::var("IGNORE_CASE").is_ok();
    // println!("{}", ignore_case);
}
