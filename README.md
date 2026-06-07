# vector-clock-rs

Vector clocks and causality tracking: Lamport timestamps, vector clocks, conflict detection, and causal ordering.

## Features

- **Lamport Timestamps** — Simple logical clocks with send/receive semantics
- **Vector Clocks** — Per-process causal history tracking
- **Conflict Detection** — Identify concurrent events
- **Causal Ordering** — Happens-before relation and topological sort
- **Clock Merging** — Component-wise max merge with dominance detection

## Modules

| Module | Description |
|--------|-------------|
| `lamport` | Lamport logical timestamps |
| `vector` | Vector clock implementation |
| `conflict` | Conflict detection between concurrent events |
| `ordering` | Causal ordering relations (happens-before) |
| `merge` | Clock merging and synchronization |

## Usage

```rust
use vector_clock_rs::vector::VectorClock;

let mut vc1 = VectorClock::new();
vc1.increment(1);
let mut vc2 = vc1.clone();
vc2.increment(2);
assert!(vc1.happened_before(&vc2));
```

## Testing

```bash
cargo test    # 28 tests
cargo clippy  # zero warnings
```

## License

MIT
