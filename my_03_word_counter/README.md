### 03. 단어 카운터 CLI (wc -w 클론)

Rust Book의 소유권 챕터를 학습한 직후 1~2시간 안에 만들어 보는 작은 CLI 프로젝트입니다. 표준 라이브러리만 사용하며, 외부 크레이트를 추가하지 않는 것을 목표로 합니다.

#### 선행 챕터
rustlings 기준으로 다음 챕터를 마친 직후가 적합합니다.

- `06_move_semantics` — 소유권 이동(move), 빌림(borrow), 참조(`&`, `&mut`)

#### 기능 명세
명령줄 인자로 파일 경로를 받아 해당 파일의 단어 수, 줄 수, 바이트 수를 출력합니다.

```bash
cargo run -- <파일경로>
```

출력 형식 예시

```
lines: 12
words: 87
bytes: 524
```

세부 규칙

- 단어는 공백류(스페이스, 탭, 개행 등)로 구분되는 토큰 하나를 의미합니다.
- 줄 수는 개행 문자(`\n`)를 기준으로 셉니다.
- 바이트 수는 파일의 실제 바이트 길이를 사용합니다(문자 수가 아님).

#### 핵심 활용 포인트
이 프로젝트에서 의식적으로 연습할 Rust 요소들입니다.

- 소유권 이동(move)과 빌림(borrow) — 파일에서 읽어 온 `String`을 함수로 어떻게 넘길지 직접 결정해 보기
- `String` vs `&str` — 카운트 함수의 시그니처를 `&str`로 받아 빌림으로 처리
- 함수 분리하면서 borrow checker 의식하기 — `count_lines`, `count_words`, `count_bytes`처럼 분리해 같은 데이터를 여러 함수에 빌려 주기
- `std::fs::read_to_string` 등 표준 라이브러리 파일 입출력 맛보기

#### 검증 예시
임의 텍스트 파일을 만들어 시스템의 `wc -w` 결과와 비교해 일치하면 1차 목표 달성으로 간주합니다.

```bash
echo "hello rust world" > sample.txt
cargo run -- sample.txt   # → words: 3
wc -w sample.txt          # → 3 sample.txt
```

#### 디렉토리 구조

```
my_03_word_counter/
├── Cargo.toml
├── README.md
└── src/
    └── main.rs
```

#### 실행 방법

```bash
# 빌드만
cargo build

# 한 번에 빌드 + 실행 (인자는 -- 뒤에)
cargo run -- sample.txt

# 릴리스 빌드
cargo build --release
./target/release/my_03_word_counter sample.txt
```
