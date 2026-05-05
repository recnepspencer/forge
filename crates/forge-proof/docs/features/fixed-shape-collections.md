# Fixed-Shape Collections

## What This Feature Is

Fixed-shape collection wrappers make small structural invariants explicit. Instead of treating "exactly one," "exactly two," "non-empty," or "disjoint pair" as comments on raw collections, `forge-proof` gives them named carriers.

## Why You Use It

- a function really needs exactly one item, not "a vector that should have one"
- a function really needs a fixed ordered pair
- a function really needs a non-empty collection
- a pair must also carry a disjointness proof

## Stable Entry Points

- `ExactlyOne<T>`
- `Pair<T>`
- `NonEmpty<T>`
- `DisjointPair<T>`

Key methods:

- `ExactlyOne::new(...)`
- `ExactlyOne::get()`
- `Pair::new(left, right)`
- `Pair::left()`
- `Pair::right()`
- `NonEmpty::new(head, tail)`
- `NonEmpty::try_from_vec(items)`
- `DisjointPair::left()`
- `DisjointPair::right()`
- `DisjointPair::proof()`

Good to know:

- `DisjointPair` construction is sealed because it carries a real proof fact

## DX Posture

This feature has a partial pleasant lane.

- pleasant helpers exist for `pair(...)` and `non_empty(...)`
- direct wrapper constructors such as `ExactlyOne::new(...)` and `Pair::new(...)` remain the raw substrate truth
- when teaching or using the wrappers directly, prefer `use forge_proof::raw::*;`

## Core Mental Model

These wrappers are for invariants that deserve to be part of the type, not runtime folklore.

Use them when shape matters to correctness or progression clarity:

- cardinality
- ordering
- non-emptiness
- disjointness

## How It Executes

Typical usage:

1. construct the smallest honest shape wrapper
2. use its typed accessors instead of raw tuple or vector indexing
3. preserve or extract owned state explicitly
4. for proof-bearing wrappers like `DisjointPair`, let stronger surfaces construct them

## Small Example

```rust
use forge_proof::{ExactlyOne, Pair};

let only = ExactlyOne::new("only");
let pair = Pair::new("left", "right");

assert_eq!(only.get(), &"only");
assert_eq!(pair.left(), &"left");
```

This is the smallest honest example because both wrappers are public, direct, and encode real shape constraints.

## Real Example

```rust
use forge_proof::{NonEmpty, Pair};

fn collect_inputs() {
    let queued = NonEmpty::new("first", vec!["second", "third"]);
    let lanes = Pair::new("primary", "secondary");

    assert_eq!(queued.first(), &"first");
    assert_eq!(queued.as_slice().len(), 3);
    assert_eq!(lanes.right(), &"secondary");
}
```

What this shows:

- `NonEmpty` keeps the "must have at least one" rule visible
- `Pair` preserves ordered fixed-arity structure
- downstream code no longer needs to recover these invariants from raw collections

## How It Relates To Other Features

- Pair this with [Proven Vectors](./proven-vectors.md) when the collection also needs proof-bearing order or uniqueness.
- Pair this with [Structural Facts](./structural-facts.md) because `DisjointPair` carries `Disjointness`.
- Pair this with [Recipes And Stages](./recipes-and-stages.md) when fixed-arity shape participates in staged progression.

## Inspection And Debugging

- use `left()`, `right()`, `get()`, `first()`, and `as_slice()` for honest inspection
- `into_inner()`, `into_array()`, and `into_vec()` are the owned extraction surfaces
- `DisjointPair::proof()` makes the carried disjointness fact explicit

## Anti-Patterns

- Do not pass raw `Vec<T>` when a function semantically requires `NonEmpty<T>`.
- Do not pass a raw tuple where `Pair<T>` is the intended stable boundary.
- Do not construct a disjoint pair through comments or convention when the proof-bearing wrapper is the real invariant.

## Current Limits

- fixed-shape wrappers are intentionally small and static
- `DisjointPair` sealing means public callers consume it more often than they mint it
- these wrappers do not replace higher-level composition flows; they support them

## Related Docs

- [Proven Vectors](./proven-vectors.md)
- [Structural Facts](./structural-facts.md)
- [Recipes And Stages](./recipes-and-stages.md)
