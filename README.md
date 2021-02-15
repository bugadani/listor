Listor
======

Listor is both a list and a vector at the same time. It is a linked list implemented in contiguous
memory, with constant-time[^1] operations. Listor is internally implemented with a slab allocator.

Unlike [slab] or [stable_vec], Listor implements a doubly-linked list.

Examples
--------

### Unbounded listor

```rust
use listor::Listor;

// Create a new listor with unbounded capacity.
let mut listor = Listor::new();

listor.push_back(1).unwrap();
listor.push_back(2).unwrap();
listor.push_back(3).unwrap();

assert_eq!(3, listor.len());
```

### Unbounded listor with initial capacity

```rust
use listor::Listor;

// Create a new listor with unbounded capacity with an initial capacity for 2 elements.
let mut listor = Listor::with_capacity(2);

listor.push_back(1).unwrap();
listor.push_back(2).unwrap();

// Pushing an element after the listor is full grows the listor.
listor.push_back(3).unwrap();

assert_eq!(3, listor.len());
```

### Bounded listor

```rust
use listor::Listor;

// Create a new listor with capacity for 2 elements.
let mut listor = Listor::bounded(2);

listor.push_back(1).unwrap();
listor.push_back(2).unwrap();

// Pushing an element after the listor is full fails.
assert!(listor.push_back(3).is_none());

assert_eq!(2, listor.len());
```

[slab]: https://crates.io/crates/slab
[stable_vec]: https://crates.io/crates/stable-vec
[^1]: as long as the data structure does not have to reallocate.
