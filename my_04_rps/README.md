## rust 학습용 사이드 프로젝트

### 전체 로드맵
[ROADMAP.md](../ROADMAP.md)

---

### 04. 가위바위보 CLI

Rust Book의 구조체와 열거형 챕터를 학습한 직후 1~2시간 안에 만들어 보는 작은 REPL CLI 프로젝트입니다. 표준 라이브러리만 사용하며, 외부 크레이트를 추가하지 않는 것을 목표로 합니다.

#### 선행 챕터
rustlings 기준으로 다음 두 챕터를 마친 직후가 적합합니다.

- `07_structs` — 필드 정의, `impl` 블록, 메서드 vs 연관 함수
- `08_enums` — `enum` variant, `match`로 모든 경우 다루기

#### 기능 명세
사람이 입력한 수와 무작위로 결정된 컴퓨터의 수를 비교해 매 라운드 결과를 출력합니다. REPL 형태로 동작하며, 점수가 누적됩니다. `quit`을 입력하면 최종 결과를 출력하고 종료합니다.

```bash
cargo run
> rock
You: Rock, CPU: Scissors → Win (You 1 - 0 CPU)
> paper
You: Paper, CPU: Paper → Draw (You 1 - 0 CPU, Draws 1)
> quit
Final: You 1 - 0 CPU, Draws 1
```

지원하는 입력은 다음과 같습니다.

| 입력            | 의미       |
|-----------------|------------|
| `rock` / `r`    | 바위       |
| `paper` / `p`   | 보         |
| `scissors` / `s`| 가위       |
| `quit` / `q`    | 종료       |

승패 규칙

- `Rock` vs `Scissors` → Rock 승
- `Scissors` vs `Paper` → Scissors 승
- `Paper` vs `Rock` → Paper 승
- 같은 수끼리는 무승부

#### 핵심 활용 포인트
이 프로젝트에서 의식적으로 연습할 Rust 요소들입니다.

- `enum Move { Rock, Paper, Scissors }` — 가능한 수를 타입으로 표현
- `enum Outcome { Win, Lose, Draw }` — 라운드 결과를 타입으로 표현
- `struct Score { wins: u32, losses: u32, draws: u32 }` — 누적 점수를 한 묶음으로
- `match`로 승자 판정 — 모든 (사람, 컴퓨터) 조합을 빠짐없이 다루기
- `impl` 메서드 — `Move::from_str(&str) -> Option<Move>`, `Score::record(&mut self, Outcome)` 등 동작을 타입에 묶기
- 무작위성 — 외부 크레이트 없이 시간 기반 의사 난수(`std::time::SystemTime`) 정도로 시작해도 충분

#### 검증 예시
다음 흐름이 기대대로 동작하면 1차 목표 달성으로 간주합니다.

```bash
cargo run
# rock / paper / scissors 입력 시 매 라운드 결과가 출력되는지
# 점수가 정상적으로 누적되는지
# quit 입력 시 최종 점수가 출력되며 종료되는지
```

#### 디렉토리 구조

```
my_04_rps/
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

# 릴리스 빌드
cargo build --release
./target/release/my_04_rps
```
