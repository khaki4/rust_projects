## rust 학습용 사이드 프로젝트

### 전체 로드맵
[ROADMAP.md](../ROADMAP.md)

---

### 06. 인메모리 주소록 CLI

`HashMap`과 `Option<T>`을 익힌 직후 1~2시간 안에 만들어 보는 작은 REPL 형태의 CLI 프로젝트입니다. 데이터는 메모리에만 보관하며 파일 저장은 다루지 않습니다. 표준 라이브러리만 사용하며, 외부 크레이트를 추가하지 않는 것을 목표로 합니다.

#### 선행 챕터
rustlings 기준으로 다음 두 챕터를 마친 직후가 적합합니다.

- `11_hashmaps` — `HashMap` 생성/조회/삽입, `entry().or_insert()`
- `12_options` — `Option<T>`, `unwrap_or_else`, `match`로 분기 처리

#### 기능 명세
실행하면 프롬프트가 뜨고, 한 줄 단위로 명령을 입력받아 처리하는 REPL입니다. 데이터는 프로세스 메모리에만 보관되며, 종료하면 사라집니다.

```bash
cargo run
```

지원하는 명령은 다음 네 가지입니다.

| 명령                       | 동작                                                  |
|----------------------------|-------------------------------------------------------|
| `add <name> <phone>`       | 이름을 키로 연락처를 추가/갱신                        |
| `find <name>`              | 이름으로 검색해 연락처 출력, 없으면 안내 메시지       |
| `remove <name>`            | 이름으로 연락처 삭제, 없으면 안내 메시지              |
| `list`                     | 저장된 모든 연락처를 출력                             |

#### 핵심 활용 포인트
이 프로젝트에서 의식적으로 연습할 Rust 요소들입니다.

- `HashMap<String, Contact>` — 이름을 키로 한 인메모리 저장소
- `Option<&Contact>` — `get()`이 돌려주는 참조 옵션 다루기
- `entry().or_insert()` — 키가 없을 때만 기본값을 넣는 패턴
- `unwrap_or_else` — 검색 실패 시 기본 동작/메시지 처리
- `match` — `Some` / `None` 분기와 명령 문자열 분기 모두에 활용
- 표준 입력(`std::io::stdin().lines()`)으로 한 줄씩 읽어 명령 파싱

#### 검증 예시
REPL을 직접 실행해 다음 흐름이 의도대로 동작하면 1차 목표 달성으로 간주합니다.

```text
> add alice 010-1234-5678
> find alice          # → 연락처 출력
> remove alice
> find alice          # → "찾을 수 없음" 류 메시지
```

#### 디렉토리 구조

```
my_06_addressbook/
├── Cargo.toml
├── README.md
└── src/
    └── main.rs
```

#### 실행 방법

```bash
# 빌드만
cargo build

# 한 번에 빌드 + 실행 (REPL 진입)
cargo run

# 릴리스 빌드
cargo build --release
./target/release/my_06_addressbook
```
