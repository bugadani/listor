Listor
======

Listor is both a list and a vector at the same time. It is a linked list implemented in contiguous
memory, with constant-time[^1] operations. Listor is internally implemented with a slab allocator.

Unlike [slab] or [stable_vec], Listor implements a doubly-linked list.

[slab]: https://crates.io/crates/slab
[stable_vec]: https://crates.io/crates/stable-vec
[^1]: as long as the data structure does not have to reallocate.
