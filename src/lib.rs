use dotenv::dotenv;
use std::error::Error;
use std::{env, fs};

// 대소문자 구분안하고 의미상 일치하는지를 검색하는 함수
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
    is_line_number: bool,
) -> Vec<&'a str> {
    let _query = query.to_lowercase(); // 작업 추가 가능
    let mut results = Vec::new();

    let mut line_number = 1;

    for line in contents.lines() {
        if line.to_lowercase().contains(&_query) {
            if is_line_number {
                results.push(format!("{}: {}", line_number, line));
            } else {
                results.push(line);
            }
        }
        line_number += 1;
    }

    results
}

// 파일을 읽어서 출력하는 함수(SoC)
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // let contents =
    //     fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case || !config.case_sensitive {
        search_case_insensitive(&config.target, &contents, &config.line_number)
    } else {
        search(&config.target, &contents, &config.line_number)
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
    pub line_number: bool,
    pub case_sensitive: bool,
    pub ignore_case: bool,
}

enum Flag {
    LineNumber,
    IgnoreCase,
}

impl Config {
    // Result를 사용해서 성공시 Config, 실패시 &'static str(lifetime은 'static)를 갖는 Result를 반환
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // 첫번째 인수는 프로그램 이름이므로 무시

        dotenv().ok();

        // let flag = match args.next() {
        //     Some(arg) => match arg.as_str() {
        //         "-n" => Flag::LineNumber,
        //         "--line-number" => Flag::LineNumber,
        //         "-i" => Flag::IgnoreCase,
        //         "--ignore-case" => Flag::IgnoreCase,
        //         _ => return Err("Invalid flag"),
        //     },
        //     None => return Err("Didn't get a flag"),
        // };

        let target = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        let case_sensitive = env::var("CASE_SENSITIVE").is_ok();
        let line_number = env::var("LINE_NUMBER").is_ok();

        Ok(Config {
            target,
            file_path,
            ignore_case,
            case_sensitive,
            line_number,
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
    // let mut results = Vec::new();

    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }

    // results

    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
