## rust 학습용 사이드 프로젝트

### 전체 로드맵
[ROADMAP.md](../ROADMAP.md)

---

### 02. 숫자 통계 CLI

Rust Book의 기본 타입과 컬렉션 챕터를 학습한 직후 1~2시간 안에 만들어 보는 CLI 프로젝트입니다. 표준 라이브러리만 사용하며, 외부 크레이트를 추가하지 않는 것을 목표로 합니다.

#### 선행 챕터
rustlings 기준으로 다음 두 챕터를 마친 직후가 적합합니다.

- `04_primitive_types` — 정수/부동소수점 타입, 튜플, 배열
- `05_vecs` — `Vec<T>` 생성, 요소 접근, 순회

#### 기능 명세
표준 입력(stdin) 또는 명령줄 인자(args)로 공백으로 구분된 숫자 여러 개를 받아, 평균/중앙값/최대/최소를 계산해 출력합니다.

```bash
# 명령줄 인자로 전달
cargo run -- 1 2 3 4 5

# stdin으로 전달 (한 줄에 공백 구분)
echo "1 2 3 4 5" | cargo run
```

출력 형식 예시

```
mean=3, median=3, max=5, min=1
```

#### 핵심 활용 포인트
이 프로젝트에서 의식적으로 연습할 Rust 요소들입니다.

- `Vec<f64>` — 가변 길이 숫자 컬렉션을 만들고 채우기
- slice (`&[f64]`) — 통계 계산 함수의 매개변수로 슬라이스를 받아 소유권 없이 다루기
- 튜플로 결과 묶기 — `fn stats(xs: &[f64]) -> (f64, f64, f64, f64)` 형태로 평균/중앙값/최대/최소를 한 번에 반환
- `iter()` 맛보기 — `.sum::<f64>()`, `.max_by(...)`, `.min_by(...)` 등 이터레이터 메서드를 살짝 경험
- 부동소수점 비교 주의 — `f64`는 `Ord`가 아니므로 `max_by`/`min_by`에 `partial_cmp`를 사용해야 함

#### 검증 예시
다음 케이스가 기대대로 동작하면 1차 목표 달성으로 간주합니다.

```bash
echo "1 2 3 4 5" | cargo run
# → mean=3, median=3, max=5, min=1
```

#### 디렉토리 구조

```
my_02_stats/
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
cargo run -- 1 2 3 4 5

# stdin으로 실행
echo "1 2 3 4 5" | cargo run

# 릴리스 빌드
cargo build --release
./target/release/my_02_stats 1 2 3 4 5
```
