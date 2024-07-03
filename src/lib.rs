use std::error::Error;
use std::{env, fs};

// 대소문자 구분하는 검색 함수
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let _query = query.to_lowercase(); // 작업 추가 가능
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&_query) {
            results.push(line);
        }
    }

    results
}

// 파일을 읽어서 출력하는 함수(SoC)
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // let contents =
    //     fs::read_to_string(config.file_path).expect("Should have been able to read the file");
    let contents = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.target, &contents)
    } else {
        search(&config.target, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    Ok(())
}
// 1. run()의 반환 타입이 ()에서 Result<(), Box<dyn Error>로 변경(Box는 트레이트 객체, dyn은 동적)
// 2. ? 연산자를 사용해서 expect호출을 제거(panic!대신 호출하는 쪽에서 처리할 수 있도록 함수에서 직접 에러값을 반환)
// 3. ()를 사용한다는건 run을 호출하여 side effect에 대해서만 처리하겠다는 표현
pub struct Config {
    pub target: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    // Result를 사용해서 성공시 Config, 실패시 &'static str(lifetime은 'static)를 갖는 Result를 반환
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let target = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            target,
            file_path,
            ignore_case,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

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
