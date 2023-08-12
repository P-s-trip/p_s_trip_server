### Arc

Arc는 데이터를 힙에 할당하고, 데이터를 가리키는 참조자들의 개수를 추적합니다. 참조자 개수를 원자적으로 증가/감소시키기 때문에 여러 스레드가 동시에 안전하게 접근할 수 있습니다. Arc는 Rc와 비슷하지만, Arc는 여러 스레드에서 사용할 수 있도록 동시성을 보장하는 반면, Rc는 단일 스레드에서만 사용할 수 있습니다.

### destructuring

```
let p = Point { x: 0, y: 7 };

let Point { x: a, y: b } = p;
```

### r#

r#은 Rust에서 "예약어" 또는 "키워드"를 변수 이름 또는 식별자로 사용하려고 할 때 충돌을 피하기 위한 특별한 접두사입니다.

### where T: std::fmt::Debug

와 같은 구문을 사용하여 해당 타입 T가 Debug 트레이트를 구현하는지 여부를 체크할 수 있습니다. 이는 디버깅 및 로깅 등의 목적으로 코드를 작성할 때 유용합니다.

Rust-analyzer › Inlay Hints › Parameter Hints: Enable
Rust-analyzer › Inlay Hints › Chaining Hints: Enable
