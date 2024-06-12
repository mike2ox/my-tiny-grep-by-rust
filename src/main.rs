use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    // dbg!(&args);

    let config = Config::new(&args);
    // println!("Searching for {}", target);
    // println!("In file {}", file_path);

    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

struct Config {
    target: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let target = args[1].clone();
        let file_path = args[2].clone();

        Config { target, file_path }
    }
}
