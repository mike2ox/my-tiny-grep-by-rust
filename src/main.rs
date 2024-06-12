use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    // dbg!(&args);

    let target = &args[1]; // 찾고자 하는 문자열
    let file_path = &args[2]; // 파일 경로

    // println!("Searching for {}", query);
    // println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
