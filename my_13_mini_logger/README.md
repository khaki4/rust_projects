## rust 학습용 사이드 프로젝트

### 전체 로드맵
[ROADMAP.md](../ROADMAP.md)

---

### 13. 미니 로거 매크로 + Config 빌더

Rust Book 후반의 매크로/변환 트레잇을 학습한 직후 1~2시간 안에 만들어 보는 로드맵 마지막 프로젝트입니다. 자체 로깅 매크로와 `Config` 변환 로직을 작성하고, clippy 권고를 반영해 깔끔한 코드로 마무리하는 것을 목표로 합니다.

#### 선행 챕터
rustlings 기준으로 다음 세 챕터를 마친 직후가 적합합니다.

- `21_macros` — `macro_rules!` 패턴 매칭과 반복(`$($x:expr),*`) 문법
- `22_clippy` — clippy 사용법과 경고 메시지 해석
- `23_conversions` — `From`/`Into`, `TryFrom`/`TryInto` 변환 트레잇

#### 기능 명세
`info!`, `warn!`, `error!` 매크로를 직접 작성하고, `HashMap<String, String>`을 `Config` struct로 변환하는 기능을 구현합니다.

```bash
cargo run
cargo clippy -- -D warnings
```

요구 사항

- `info!` / `warn!` / `error!` 매크로를 `macro_rules!`로 직접 정의 (외부 로깅 크레이트 사용 금지)
- 출력에는 로그 레벨과 타임스탬프가 포함되어야 함
- `Config` struct를 정의하고, `impl From<HashMap<String, String>> for Config`를 구현해 키/값 맵에서 설정을 읽어 들임
- `cargo clippy -- -D warnings`가 경고 0개로 통과해야 함

매크로 출력 예시

```
[INFO][2026-04-26T12:00:00] hello world
[WARN][2026-04-26T12:00:01] disk almost full
[ERROR][2026-04-26T12:00:02] connection refused
```

#### 핵심 활용 포인트
이 프로젝트에서 의식적으로 연습할 Rust 요소들입니다.

- `macro_rules!` — 패턴 매칭(`$($arg:tt)*`)과 반복으로 가변 인자 매크로 정의
- `From` / `Into` — `HashMap<String, String>` → `Config` 자동 변환 흐름 체득
- clippy 권고 적용 — `-D warnings`로 경고를 에러로 승격해 모든 경고를 끄는 습관 들이기
- `format_args!` / `println!` 위임 패턴 — 매크로 안에서 표준 포맷 매크로를 재사용

#### 검증 예시
다음 두 항목이 모두 만족되면 1차 목표 달성으로 간주합니다.

```bash
# 1) clippy 경고 없이 통과
cargo clippy -- -D warnings

# 2) 매크로 호출이 의도한 형식으로 출력되는지 수동 확인
#    예: info!("hello {}", name) → [INFO][...] hello <name>
cargo run
```

#### 디렉토리 구조

```
my_13_mini_logger/
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

# clippy 경고 0개 검증
cargo clippy -- -D warnings

# 릴리스 빌드
cargo build --release
./target/release/my_13_mini_logger
```
