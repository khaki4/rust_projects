## rust 학습용 사이드 프로젝트

### 전체 로드맵
[ROADMAP.md](../ROADMAP.md)

---

### 05. 시저 암호 CLI

문자열과 모듈 시스템을 학습한 직후 1~2시간 안에 만들어 보는 작은 CLI 프로젝트입니다. 알파벳을 정해진 칸수만큼 시프트하는 고전적인 시저 암호를 구현하고, 코드를 여러 모듈 파일로 분리하는 연습을 함께 합니다. 표준 라이브러리만 사용합니다.

#### 선행 챕터
rustlings 기준으로 다음 두 챕터를 마친 직후가 적합합니다.

- `09_strings` — `String` / `&str`, `chars()` / `bytes()`, 문자열 슬라이스와 소유권
- `10_modules` — `mod` / `pub` / `use`로 코드를 여러 파일로 분리하기

#### 기능 명세
서브커맨드와 시프트 값을 명령줄 인자로 받아 입력 문자열을 암호화/복호화합니다.

```bash
cargo run -- <encrypt|decrypt> <문자열> <shift>
```

지원하는 서브커맨드는 다음 두 가지입니다.

| 서브커맨드 | 동작                          | 비고                              |
|------------|-------------------------------|-----------------------------------|
| `encrypt`  | 알파벳을 shift만큼 앞으로 이동 | 그 외 문자(공백/숫자/기호)는 통과 |
| `decrypt`  | 알파벳을 shift만큼 뒤로 이동   | 대소문자는 각각 유지              |

처리 규칙

- `'a'..='z'`와 `'A'..='Z'` 범위만 시프트 대상
- 시프트 결과는 알파벳 26자 안에서 순환 (`khoor` ↔ `hello` 같은 wrap-around 동작)
- 알파벳이 아닌 문자는 변환하지 않고 그대로 출력
- 사용 예: `cargo run -- encrypt "hello" 3` → `khoor`

#### 핵심 활용 포인트
이 프로젝트에서 의식적으로 연습할 Rust 요소들입니다.

- `String` vs `&str` — 인자에서 받은 입력을 어떤 타입으로 다루고, 결과를 어떻게 돌려줄지 결정
- `char` 메서드 — `is_ascii_alphabetic()`, `is_ascii_uppercase()` 등으로 분기
- `chars().map().collect()` 패턴 — 이터레이터로 문자별 변환을 표현하고 `String`으로 다시 모으기
- `mod cipher` / `mod cli` 분리 — 암호화 로직과 인자 파싱 로직을 별도 파일로 분리
- `pub` / `use` — 모듈 경계를 의식하며 외부에 노출할 함수만 `pub`으로 표시

#### 검증 예시
다음 두 케이스가 기대대로 동작하면 1차 목표 달성으로 간주합니다.

```bash
cargo run -- encrypt "hello" 3   # → khoor
cargo run -- decrypt "khoor" 3   # → hello
```

#### 디렉토리 구조

```
my_05_caesar_cipher/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs
    ├── cipher.rs
    └── cli.rs
```

`cipher.rs`와 `cli.rs`는 모듈 분리 연습을 위한 예시 구조입니다. 직접 만들면서 `mod` / `pub` / `use`로 연결합니다.

#### 실행 방법

```bash
# 빌드만
cargo build

# 한 번에 빌드 + 실행 (인자는 -- 뒤에)
cargo run -- encrypt "hello" 3

# 릴리스 빌드
cargo build --release
./target/release/my_05_caesar_cipher encrypt "hello" 3
```
