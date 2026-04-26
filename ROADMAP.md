# Rust 학습 로드맵: 챕터 묶음 단위 작은 프로젝트

## Context

The Rust Book과 rustlings로 Rust를 학습 중인데, rustlings 길이로 인해 흥미가 떨어지고 있다. 흥미를 유지하면서 학습한 내용을 즉시 활용하는 작은 프로젝트(1~2시간) 단위로 학습 흐름을 재구성한다.

**원칙:**
- rustlings 챕터 순서를 따름. `00_intro` 직후부터 시작 (처음부터 다시 진행)
- 한 묶음당 rustlings 챕터 1~3개를 학습한 **직후**, 그 묶음에서 배운 개념을 활용하는 작은 프로젝트 1개 완성 (사후 활용형)
- 한 프로젝트는 1~2시간 안에 끝낼 수 있는 규모로 한정
- 외부 크레이트 의존성 최소화 (표준 라이브러리 위주)
- 흥미 분야: CLI 도구 + 네트워크/웹 서버 (12번에서 TCP 서버로 보상)
- 학습자 배경: JavaScript/TypeScript 주 사용자

## 디렉토리 구조

각 프로젝트는 독립된 cargo 프로젝트로 생성한다. workspace는 사용하지 않는다 — 폴더 단위로 진도가 한눈에 보이고 각자 독립 빌드/실행이 가능하다.

```
rust_project/
├── 01_unit_converter/       # 단위 변환기 CLI
├── 02_stats/                # 숫자 통계 CLI
├── 03_word_counter/         # 단어 카운터 (wc -w 클론)
├── 04_rps/                  # 가위바위보 CLI
├── 05_caesar_cipher/        # 시저 암호 CLI
├── 06_addressbook/          # 인메모리 주소록 CLI
├── 07_csv_summary/          # CSV 요약 도구
├── 08_shapes/               # 도형 면적/둘레 계산기
├── 09_tokenizer/            # 간단한 토크나이저
├── 10_roman_numerals/       # 로마숫자 변환기 + 테스트
├── 11_bst/                  # 이진 탐색 트리
├── 12_echo_server/          # 멀티스레드 TCP 에코 서버
└── 13_mini_logger/          # 미니 로거 매크로 + Config
```

생성 명령 예: `cd rust_project && cargo new 01_unit_converter`

## 프로젝트 13개 상세

### 01. 단위 변환기 CLI
- **선행 챕터:** `01_variables`, `02_functions`, `03_if`
- **기능:** `cargo run -- <값> <단위>` 형식. 섭씨↔화씨, kg↔lb 변환
- **핵심 활용:** `let mut`, fn 시그니처/반환값, `if/else`, `match` 맛보기
- **검증:** `cargo run -- 0 c` → `32 °F`, `cargo run -- 100 lb` → 약 `45.36 kg`

### 02. 숫자 통계 CLI
- **선행 챕터:** `04_primitive_types`, `05_vecs`
- **기능:** stdin 또는 args로 숫자 여러 개 받아 평균/중앙값/최대/최소 출력
- **핵심 활용:** `Vec<f64>`, slice, tuple로 결과 묶기, 기본 `iter()`
- **검증:** `echo "1 2 3 4 5" | cargo run` → mean=3, median=3, max=5, min=1

### 03. 단어 카운터 CLI (wc -w 클론)
- **선행 챕터:** `06_move_semantics`
- **기능:** 파일 경로를 받아 단어 수/줄 수/바이트 수 출력
- **핵심 활용:** 소유권/빌림, `String` vs `&str`, 함수 분리하며 borrow checker 의식
- **검증:** 임의 텍스트 파일에 대해 시스템 `wc -w`와 결과 비교

### 04. 가위바위보 CLI
- **선행 챕터:** `07_structs`, `08_enums`
- **기능:** 사람 입력 vs 무작위 컴퓨터, 매 라운드 결과 출력. 점수 누적, `quit` 입력 시 최종 결과
- **핵심 활용:** `enum Move`, `enum Outcome`, `struct Score`, `match`로 승자 판정, `impl` 메서드
- **검증:** 수동 플레이로 정상 동작/누적/종료 흐름 확인

### 05. 시저 암호 CLI
- **선행 챕터:** `09_strings`, `10_modules`
- **기능:** `encrypt`/`decrypt` 서브커맨드 + shift 값. 알파벳만 시프트, 그 외 통과
- **핵심 활용:** 문자열/`char` 메서드, `chars().map().collect()`, `mod cipher` / `mod cli` 파일 분리, `pub`/`use`
- **검증:** `encrypt "hello" 3` → `khoor`, `decrypt "khoor" 3` → `hello`

### 06. 인메모리 주소록 CLI
- **선행 챕터:** `11_hashmaps`, `12_options`
- **기능:** REPL 형태로 `add`/`find`/`remove`/`list`. 메모리만 사용
- **핵심 활용:** `HashMap<String, Contact>`, `Option<&Contact>`, `entry().or_insert()`, `unwrap_or_else`
- **검증:** REPL로 add → find → remove → find(없음) 흐름 수동 확인

### 07. CSV 요약 도구
- **선행 챕터:** `13_error_handling`
- **기능:** CSV 파일 경로 + 컬럼 인덱스를 받아 평균/합 출력. 파일 없음/파싱 실패/인덱스 범위 초과를 모두 적절한 에러로 처리
- **핵심 활용:** `Result<T, E>`, `?`, 사용자 정의 `enum Error` + `From` 변환, `panic!` 대신 명시적 에러
- **검증:** 정상 CSV / 잘못된 CSV / 존재하지 않는 파일에 대해 각각 적절한 에러 메시지 출력 확인

### 08. 도형 면적/둘레 계산기
- **선행 챕터:** `14_generics`, `15_traits`
- **기능:** `Circle`, `Rectangle`, `Triangle` + `Shape` 트레잇(`area()`, `perimeter()`). `Vec<Box<dyn Shape>>` 또는 제네릭 함수로 합계 계산
- **핵심 활용:** `trait Shape`, `impl Trait for Type`, `fn total_area<T: Shape>(...)`, `impl Display`
- **검증:** 단위 테스트로 각 도형 면적 검증

### 09. 간단한 토크나이저
- **선행 챕터:** `16_lifetimes`
- **기능:** 문자열을 입력으로 받아 `Vec<&str>` (단어 슬라이스 모음) 반환. 구분자 지정 옵션
- **핵심 활용:** 명시적 lifetime `fn tokenize<'a>(input: &'a str) -> Vec<&'a str>`, 입력 참조 수명을 출력에 전파
- **검증:** `tokenize("hello world")` → `["hello", "world"]`, 단위 테스트

### 10. 로마숫자 ↔ 아라비아숫자 변환기
- **선행 챕터:** `17_tests`, `18_iterators`
- **기능:** `to_roman(u32) -> String`, `from_roman(&str) -> Option<u32>`
- **핵심 활용:** iterator 체인(`map`, `fold`, `filter`, `take_while`), `#[test]`, `assert_eq!`
- **검증:** 단위 테스트 10개 이상 통과

### 11. 이진 탐색 트리
- **선행 챕터:** `19_smart_pointers`
- **기능:** `insert`, `contains`, `inorder_traversal` (`Vec` 반환)
- **핵심 활용:** `Option<Box<Node>>` 재귀 구조, 재귀 메서드
- **검증:** 단위 테스트로 inorder 결과가 정렬되어 나오는지 확인

### 12. 멀티스레드 TCP 에코 서버
- **선행 챕터:** `20_threads`
- **기능:** `127.0.0.1:7878`에서 listen. 클라이언트 연결마다 스레드 spawn, 받은 줄을 그대로 돌려보냄. 활성 클라이언트 수를 `Arc<Mutex<usize>>`로 공유
- **핵심 활용:** `std::net::TcpListener`, `thread::spawn`, `Arc<Mutex<T>>`, mpsc 채널(옵션)
- **검증:** `nc 127.0.0.1 7878` 한 개로 입력 줄이 돌아오는지 확인 + 두 개의 nc 동시 접속도 확인

### 13. 미니 로거 매크로 + Config 빌더
- **선행 챕터:** `21_macros`, `22_clippy`, `23_conversions`
- **기능:** `info!`/`warn!`/`error!` 매크로 직접 작성. `Config` struct + `From<HashMap<String, String>>` 구현. clippy 경고 0개 통과
- **핵심 활용:** `macro_rules!`, `From`/`Into`, clippy 권고 적용
- **검증:** `cargo clippy -- -D warnings` 통과, 매크로 호출이 의도대로 출력되는지 수동 확인

## 진행 방식

1. rustlings 해당 챕터(들)을 끝낸다
2. `cd rust_project && cargo new <번호>_<이름>`로 새 프로젝트 생성
3. 위 정의된 기능을 구현
4. 검증 항목을 직접 수행해 통과 확인
5. 다음 묶음으로 이동

## 검증 (전체 공통)

각 프로젝트는 다음을 모두 만족하면 완료:
- `cargo build` 경고 없이 빌드
- 각 프로젝트별 검증 항목 사람이 직접 수행해 통과
- 8번 이후 묶음은 가능하면 단위 테스트 1개 이상 추가

13번까지 끝나면 표준 라이브러리만으로 Rust의 핵심 개념을 한 번 훑은 셈. 이후 axum/tokio 등 외부 크레이트 단계로 자연스럽게 진입할 준비가 된다.
