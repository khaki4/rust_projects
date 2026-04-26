## rust 학습용 사이드 프로젝트

### 전체 로드맵
[ROADMAP.md](../ROADMAP.md)

---

### 09. 간단한 토크나이저

문자열 입력을 단어 단위 슬라이스로 잘라 반환하는 작은 토크나이저입니다. 새로운 `String`을 만드는 대신 입력 문자열의 일부를 그대로 가리키는 `&str` 슬라이스를 모아 반환하는 것이 핵심입니다. 이 과정에서 명시적 lifetime annotation으로 입력과 출력의 참조 수명 관계를 컴파일러에 알려 주는 연습을 합니다.

#### 선행 챕터
rustlings 기준으로 다음 챕터를 마친 직후가 적합합니다.

- `16_lifetimes` — 명시적 lifetime annotation, 입력/출력 참조 수명 관계, lifetime elision 규칙

#### 기능 명세
입력 문자열을 단어 슬라이스(`Vec<&str>`)로 분리합니다. 기본 구분자는 공백이며, 옵션으로 다른 구분자(공백/구두점 등)를 지정할 수 있어야 합니다.

```rust
fn tokenize<'a>(input: &'a str) -> Vec<&'a str>;
fn tokenize_by<'a>(input: &'a str, is_delim: impl Fn(char) -> bool) -> Vec<&'a str>;
```

- 반환되는 각 `&str`은 `input`의 부분 슬라이스이어야 하며, 새 `String` 할당이 일어나면 안 됩니다.
- 연속된 구분자는 빈 토큰을 만들지 않도록 처리합니다.
- 입력이 비어 있으면 빈 `Vec`을 반환합니다.

#### 핵심 활용 포인트
이 프로젝트에서 의식적으로 연습할 Rust 요소들입니다.

- 명시적 lifetime annotation — `fn tokenize<'a>(input: &'a str) -> Vec<&'a str>` 처럼 입력 참조의 lifetime을 출력에 그대로 전파
- 슬라이스 기반 설계 — 새 `String`을 만들지 않고 입력의 부분 슬라이스만 반환하도록 구현
- lifetime elision이 적용되는 경우 vs 명시가 필요한 경우의 차이를 직접 체감
- 클로저를 인자로 받는 함수 시그니처(`impl Fn(char) -> bool`)와 lifetime의 조합
- 단위 테스트(`#[cfg(test)]` 모듈)로 동작 검증

#### 검증 예시
다음 케이스가 기대대로 동작하면 1차 목표 달성으로 간주합니다.

```rust
assert_eq!(tokenize("hello world"), vec!["hello", "world"]);
assert_eq!(tokenize(""), Vec::<&str>::new());
assert_eq!(tokenize("  many   spaces  "), vec!["many", "spaces"]);
```

단위 테스트로 위 케이스들을 모두 통과시키면 됩니다.

#### 디렉토리 구조

```
my_09_tokenizer/
├── Cargo.toml
├── README.md
└── src/
    └── main.rs
```

#### 실행 방법

```bash
# 빌드만
cargo build

# 한 번에 빌드 + 실행
cargo run

# 단위 테스트 실행
cargo test

# 릴리스 빌드
cargo build --release
./target/release/my_09_tokenizer
```
