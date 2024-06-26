# my-tiny-grep-by-rust

rust 공식문서의 I/O 프로젝트

## 정리

### 12.1

- (pending)

### 12.2

- (pending)

### 12.3

- 프로그램의 구조적, 잠재적 에러처리 방식과 유지보수를 위한 코드 구조화에 대한 고민
  - 역활에 따라 코드를 분리하는 것이 좋다.(1)
  - 의미있는 변수들끼리 구조체로 묶어서 관리하는 것이 좋다.(2)
  - 에러 케이스별로 에러 메세지를 출력하는 것이 좋다.(3)
  - 에러 처리의 유지보수를 위해 한 곳에서 관리하는 것이 좋다.(4)

## 초과 목표

- Search 결과마다 라인넘버 출력하기 (-n, --line-number)
- Search 결과의 일정 라인수를 추가로 출력하기 (-A, --after-context NUM)
- Search 결과 이전의 일정 라인수를 추가로 출력하기 (-B, --before-context NUM)
- Case sensitivity (-s, --case-sensitive)
- ✅ Ignore case (-i, --ignore-case)
- ✅ Search의 결과물들을 치환 (-r, --replace REPLACEMENT_TEXT)

## 궁금한 점

- main 함수의 입력부에는 별도의 변수가 없는건가? (ex. c 언어의 argc, argv)

- clone을 남발하면 결국 기존 C계열의 메모리 관리와 다를바 없지 않을까? // 그러고도 Rust를 사용할 이유가 있을까?
  => 세세한 부분에서도 잠재적 문제에 대해 에러를 발생시켜주기 때문에라도 사용 할 만하다(?)
