use std::env;
use std::error::Error;
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

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

// 파일을 읽어서 출력하는 함수(SoC)
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // let contents =
    //     fs::read_to_string(config.file_path).expect("Should have been able to read the file");
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{contents}");
    Ok(())
}
// 1. run()의 반환 타입이 ()에서 Result<(), Box<dyn Error>로 변경(Box는 트레이트 객체, dyn은 동적)
// 2. ? 연산자를 사용해서 expect호출을 제거(panic!대신 호출하는 쪽에서 처리할 수 있도록 함수에서 직접 에러값을 반환)
// 3. ()를 사용한다는건 run을 호출하여 side effect에 대해서만 처리하겠다는 표현
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
