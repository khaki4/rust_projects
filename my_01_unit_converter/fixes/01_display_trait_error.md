# 01. `Unit` 타입에 `Display` 미구현 (컴파일 에러)

## 증상

`cargo check` 실행 시 다음과 같은 에러가 발생합니다.

```
error[E0277]: `Unit` doesn't implement `std::fmt::Display`
  --> src/main.rs:12:29
   |
12 |     println!("{:.2} {}", v, u);
   |                     --      ^ `Unit` cannot be formatted with the default formatter
```

## 원인

`println!`의 `{}`(기본 포맷)는 `std::fmt::Display`가 구현된 타입만 출력할 수 있습니다.
`Unit` enum에는 `Display`가 구현되어 있지 않아서, `u`를 그대로 `{}`로 넘기면 컴파일이 되지 않습니다.

코드에는 이미 사람이 읽을 수 있는 라벨을 돌려주는 `Unit::label(&self) -> &str` 메서드가 정의되어 있습니다 (`src/main.rs:22~31`). `&str`은 `Display`가 구현된 타입이므로 이걸 그대로 활용하면 됩니다.

## 수정 방법

`src/main.rs:12` 의 `println!` 호출에서 `u` 대신 `u.label()`을 넘기도록 변경합니다.

**Before**
```rust
println!("{:.2} {}", v, u);
```

**After**
```rust
println!("{:.2} {}", v, u.label());
```

## 다른 선택지 (참고)

학습 차원에서 알아 두면 좋은 다른 방법들입니다. 지금 단계에서는 위의 수정 한 줄이면 충분합니다.

- `Unit`에 직접 `impl std::fmt::Display for Unit` 을 구현해서 `{}` 그대로 쓰게 만들기
- `#[derive(Debug)]`를 붙이고 `{:?}` 로 디버그 출력하기 (Display와 의미가 다름에 주의)
