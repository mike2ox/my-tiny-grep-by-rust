use my_tiny_grep_by_rust::Config;
use std::env;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();

    // dbg!(&args);

    // unwrap_or_else는 panic!이 아닌 애러를 처리할 수 있도록 해줌
    // Result가 OK를 반환하면 unwrap가 동일한 동작을 함.
    let config = Config::build(&args).unwrap_or_else(|err| {
        // 여기는 클로저로서 err를 받아서 처리하는 부분
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    // println!("Searching for {}", target);
    // println!("In file {}", file_path);

    if let Err(e) = my_tiny_grep_by_rust::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
