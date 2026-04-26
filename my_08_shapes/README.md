## rust 학습용 사이드 프로젝트

### 전체 로드맵
[ROADMAP.md](../ROADMAP.md)

---

### 08. 도형 면적/둘레 계산기

`generics`와 `traits`를 학습한 직후 1~2시간 안에 만들어 보는 작은 라이브러리/CLI 프로젝트입니다. 공통 행위를 트레잇으로 추상화하고, 제네릭 함수로 여러 도형을 한 번에 다뤄 보는 것이 목표입니다. 표준 라이브러리만 사용합니다.

#### 선행 챕터
rustlings 기준으로 다음 두 챕터를 마친 직후가 적합합니다.

- `14_generics` — 제네릭 함수, 제네릭 구조체, 단일 타입 파라미터 `<T>`
- `15_traits` — `trait` 정의/구현, 디폴트 구현, trait bound (`T: Trait`), `dyn Trait`

#### 기능 명세
세 가지 도형(`Circle`, `Rectangle`, `Triangle`)을 정의하고, 공통 `Shape` 트레잇으로 면적과 둘레를 통일된 인터페이스로 다룹니다.

```rust
trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}
```

여러 도형을 한 번에 다루는 두 가지 접근을 모두 연습합니다.

- `Vec<Box<dyn Shape>>` — 동적 디스패치로 서로 다른 타입의 도형을 한 컬렉션에 담기
- `fn total_area<T: Shape>(shapes: &[T]) -> f64` — 정적 디스패치 제네릭 함수로 동일 타입 슬라이스 합계 계산

도형별 입력 값:

| 도형        | 필요한 값                              |
|-------------|----------------------------------------|
| `Circle`    | 반지름 `r`                             |
| `Rectangle` | 너비 `w`, 높이 `h`                     |
| `Triangle`  | 세 변 `a`, `b`, `c` (둘레 = a+b+c, 면적은 헤론의 공식) |

헤론의 공식

- `s = (a + b + c) / 2`
- `area = sqrt(s × (s − a) × (s − b) × (s − c))`

#### 핵심 활용 포인트
이 프로젝트에서 의식적으로 연습할 Rust 요소들입니다.

- `trait Shape` — 공통 행위 정의, 필요시 디폴트 구현 추가
- `impl Shape for Circle` / `impl Shape for Rectangle` / `impl Shape for Triangle` — 타입별 트레잇 구현
- `fn total_area<T: Shape>(shapes: &[T]) -> f64` — 제네릭 함수 + trait bound
- `Vec<Box<dyn Shape>>` — 트레잇 객체로 이종(heterogeneous) 컬렉션 다루기
- `impl std::fmt::Display for Circle` 등 — 도형마다 사람이 읽기 좋은 출력 형식 제공
- 정적 디스패치 vs 동적 디스패치 차이를 코드 레벨에서 체감

#### 검증 예시
단위 테스트로 각 도형의 면적/둘레 값을 검증합니다.

```rust
#[test]
fn circle_area() {
    let c = Circle { r: 1.0 };
    assert!((c.area() - std::f64::consts::PI).abs() < 1e-9);
}

#[test]
fn rectangle_perimeter() {
    let r = Rectangle { w: 3.0, h: 4.0 };
    assert_eq!(r.perimeter(), 14.0);
}

#[test]
fn triangle_area_3_4_5() {
    let t = Triangle { a: 3.0, b: 4.0, c: 5.0 };
    assert!((t.area() - 6.0).abs() < 1e-9);
}
```

`cargo test`로 모든 테스트가 통과하면 1차 목표 달성으로 간주합니다.

#### 디렉토리 구조

```
my_08_shapes/
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
./target/release/my_08_shapes
```
