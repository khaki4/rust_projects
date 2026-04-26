## rust 학습용 사이드 프로젝트

### 전체 로드맵
[ROADMAP.md](../ROADMAP.md)

---

### 07. CSV 요약 도구

Rust Book의 에러 처리 챕터를 학습한 직후 1~2시간 안에 만들어 보는 작은 CLI 프로젝트입니다. 표준 라이브러리만 사용하며, 외부 크레이트를 추가하지 않는 것을 목표로 합니다.

#### 선행 챕터
rustlings 기준으로 다음 챕터를 마친 직후가 적합합니다.

- `13_error_handling` — `Result<T, E>`, `?` 연산자, 에러 타입 설계, `From` 변환

#### 기능 명세
명령줄 인자로 CSV 파일 경로와 컬럼 인덱스를 받아 해당 컬럼의 합과 평균을 출력합니다.

```bash
cargo run -- <파일경로> <컬럼인덱스>
```

사용 예시

```bash
cargo run -- data.csv 2
```

다음 세 가지 실패 케이스는 모두 `panic!` 대신 명시적인 에러로 처리해야 합니다.

- 파일 없음 — 지정한 경로의 파일을 열 수 없는 경우
- CSV 파싱 실패 — 셀의 값을 숫자로 변환할 수 없는 경우 등
- 컬럼 인덱스 범위 초과 — 행에 해당 인덱스의 컬럼이 존재하지 않는 경우

#### 핵심 활용 포인트
이 프로젝트에서 의식적으로 연습할 Rust 요소들입니다.

- `Result<T, E>` — 실패할 수 있는 모든 함수의 반환 타입을 `Result`로 통일
- `?` 연산자 — 중첩된 `match` 대신 에러를 호출자로 전파
- 사용자 정의 `enum Error` — `Io`, `Parse`, `OutOfRange` 등 도메인 에러를 한 타입으로 묶기
- `impl From<...> for Error` — `std::io::Error`, `std::num::ParseFloatError` 등을 자동 변환해 `?`와 자연스럽게 연동
- `panic!` 대신 명시적 에러 반환 — 사용자 입력/외부 입력에 대한 실패는 모두 `Result`로 표현

#### 검증 예시
다음 세 케이스가 기대대로 동작하면 1차 목표 달성으로 간주합니다.

```bash
# 정상 CSV — 합/평균이 정상 출력되어야 함
cargo run -- data.csv 2

# 깨진 CSV (숫자 변환 실패 등) — 파싱 에러 메시지 출력
cargo run -- broken.csv 2

# 존재하지 않는 파일 — 파일 없음 에러 메시지 출력
cargo run -- no_such_file.csv 2
```

세 경우 모두 패닉 없이 사람이 읽을 수 있는 에러 메시지로 종료되어야 합니다.

#### 디렉토리 구조

```
my_07_csv_summary/
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
cargo run -- data.csv 2

# 릴리스 빌드
cargo build --release
./target/release/my_07_csv_summary data.csv 2
```
