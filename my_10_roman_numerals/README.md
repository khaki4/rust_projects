## rust 학습용 사이드 프로젝트

### 전체 로드맵
[ROADMAP.md](../ROADMAP.md)

---

### 10. 로마숫자 ↔ 아라비아숫자 변환기

이터레이터 어댑터 체인과 단위 테스트를 본격적으로 연습하기 위한 1~2시간 규모의 작은 라이브러리/CLI 프로젝트입니다. 표준 라이브러리만 사용하며, 외부 크레이트를 추가하지 않는 것을 목표로 합니다.

#### 선행 챕터
rustlings 기준으로 다음 두 챕터를 마친 직후가 적합합니다.

- `17_tests` — `#[test]`, `assert_eq!`, `cargo test`
- `18_iterators` — 이터레이터 어댑터 체인 (`map`, `fold`, `filter`, `take_while` 등)

#### 기능 명세
아라비아숫자(`u32`)와 로마숫자(`&str`) 사이의 양방향 변환 함수를 제공합니다.

```rust
fn to_roman(n: u32) -> String;
fn from_roman(s: &str) -> Option<u32>;
```

| 함수            | 입력         | 출력             | 비고                                         |
|-----------------|--------------|------------------|----------------------------------------------|
| `to_roman`      | `u32`        | `String`         | 아라비아 → 로마                              |
| `from_roman`    | `&str`       | `Option<u32>`    | 로마 → 아라비아. 잘못된 입력은 `None` 반환  |

변환 규칙

- 기본 기호: `I=1`, `V=5`, `X=10`, `L=50`, `C=100`, `D=500`, `M=1000`
- 감산 표기 쌍: `IV=4`, `IX=9`, `XL=40`, `XC=90`, `CD=400`, `CM=900`
- 잘못된 문자, 잘못된 조합 등 변환 불가능한 입력은 `from_roman`에서 `None` 반환

#### 핵심 활용 포인트
이 프로젝트에서 의식적으로 연습할 Rust 요소들입니다.

- 이터레이터 체인 — `map`, `fold`, `filter`, `take_while` 등을 조합해 변환 로직을 표현
- `#[test]` — 함수 단위로 검증하는 단위 테스트 작성
- `assert_eq!` — 기대값과 실제값 비교로 결과 검증
- `Option<T>` — 변환 실패를 `None`으로 표현하고 호출 측에서 다루기
- `cargo test` 워크플로 — 테스트 주도로 구현해 보기

#### 검증 예시
다음 조건을 만족하면 1차 목표 달성으로 간주합니다.

- `to_roman` / `from_roman` 각각에 대한 단위 테스트를 합쳐서 10개 이상 작성
- `cargo test` 실행 시 모든 테스트 통과

```bash
cargo test
```

#### 디렉토리 구조

```
my_10_roman_numerals/
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

# 단위 테스트 실행 (이 프로젝트의 핵심 검증)
cargo test

# 릴리스 빌드
cargo build --release
./target/release/my_10_roman_numerals
```
