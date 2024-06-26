use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    // dbg!(&args);

    // unwrap_or_else는 panic!이 아닌 애러를 처리할 수 있도록 해줌
    // Result가 OK를 반환하면 unwrap가 동일한 동작을 함.
    let config = Config::build(&args).unwrap_or_else(|err| {
        // 여기는 클로저로서 err를 받아서 처리하는 부분
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
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
    // Result를 사용해서 성공시 Config, 실패시 &'static str(lifetime은 'static)를 갖는 Result를 반환
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let target = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { target, file_path })
    }
}
