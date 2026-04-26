## rust 학습용 사이드 프로젝트

### 전체 로드맵
[ROADMAP.md](../ROADMAP.md)

---

### 11. 이진 탐색 트리 (BST)

스마트 포인터 챕터를 학습한 직후, `Box<T>`로 재귀 자료구조를 직접 만들어 보는 1~2시간 규모의 작은 프로젝트입니다. 표준 라이브러리만 사용하며, 외부 크레이트를 추가하지 않는 것을 목표로 합니다.

#### 선행 챕터
rustlings 기준으로 다음 챕터를 마친 직후가 적합합니다.

- `19_smart_pointers` — `Box<T>`, 재귀 타입과 heap allocation, `Deref`/`Drop` 맛보기

#### 기능 명세
정수(또는 비교 가능한 임의 타입)를 보관하는 이진 탐색 트리(BST)를 구현합니다. 다음 세 가지 메서드를 제공합니다.

| 메서드                                     | 설명                                            |
|--------------------------------------------|-------------------------------------------------|
| `insert(value)`                            | 트리에 값을 삽입 (중복 처리 정책은 자유)        |
| `contains(&value) -> bool`                 | 값이 트리에 존재하는지 확인                     |
| `inorder_traversal() -> Vec<T>`            | 중위 순회 결과를 `Vec<T>`로 반환                |

BST의 불변식: 어떤 노드의 왼쪽 서브트리에는 더 작은 값, 오른쪽 서브트리에는 더 큰 값만 들어갑니다. 따라서 중위 순회 결과는 항상 오름차순으로 정렬되어 나와야 합니다.

#### 핵심 활용 포인트
이 프로젝트에서 의식적으로 연습할 Rust 요소들입니다.

- `Option<Box<Node>>` 형태의 재귀 자료구조 — `Node`가 자기 자신을 자식으로 가지려면 컴파일 타임에 크기를 알 수 없으므로, heap에 할당하는 `Box`로 감싸야 한다는 점을 직접 체감
- 재귀 메서드 — `insert`, `contains`, `inorder_traversal`을 모두 자식 노드에 위임하는 재귀 호출로 구현
- `match`로 트리 분기 처리 — `Option<Box<Node>>`에 대해 `Some` / `None` 분기, 값 비교 결과(`Ordering`)에 따른 분기를 `match`로 깔끔하게 표현
- 소유권/빌림 — `&self` vs `&mut self`, 자식 노드를 어떻게 빌릴지 borrow checker와 협상

#### 검증 예시
다음 단위 테스트가 통과하면 1차 목표 달성으로 간주합니다.

```rust
#[test]
fn inorder_returns_sorted_sequence() {
    let mut tree = Bst::new();
    for v in [5, 2, 8, 1, 3, 7, 9] {
        tree.insert(v);
    }

    assert!(tree.contains(&3));
    assert!(!tree.contains(&4));
    assert_eq!(tree.inorder_traversal(), vec![1, 2, 3, 5, 7, 8, 9]);
}
```

임의 순서로 삽입하더라도 `inorder_traversal` 결과가 항상 오름차순이어야 합니다.

#### 디렉토리 구조

```
my_11_bst/
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

# 단위 테스트
cargo test

# 릴리스 빌드
cargo build --release
./target/release/my_11_bst
```
