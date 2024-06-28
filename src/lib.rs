use std::error::Error;
use std::fs;

// 파일을 읽어서 출력하는 함수(SoC)
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // let contents =
    //     fs::read_to_string(config.file_path).expect("Should have been able to read the file");
    let contents = fs::read_to_string(config.file_path)?;
    for line in search(&config.target, &contents) {
        println!("{}", line);
    }
    Ok(())
}
// 1. run()의 반환 타입이 ()에서 Result<(), Box<dyn Error>로 변경(Box는 트레이트 객체, dyn은 동적)
// 2. ? 연산자를 사용해서 expect호출을 제거(panic!대신 호출하는 쪽에서 처리할 수 있도록 함수에서 직접 에러값을 반환)
// 3. ()를 사용한다는건 run을 호출하여 side effect에 대해서만 처리하겠다는 표현
pub struct Config {
    target: String,
    file_path: String,
}

impl Config {
    // Result를 사용해서 성공시 Config, 실패시 &'static str(lifetime은 'static)를 갖는 Result를 반환
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let target = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { target, file_path })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
// 라이프타임 매개변수는 어떤 인수의 라이프타임이 반환 값의 라이프타임과 연결되는지를 특정한다
// 러스트에게 search 함수에 의해 반환된 데이터가 search 함수의 contents 인수로 전달된 데이터만큼 오래 살 것이라는 것을 말해줌
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
