# Proven Vectors

## What This Feature Is

Proven vectors are `Vec<T>` wrappers that carry explicit proof facts about the collection. `CanonicalVec<T>` carries canonical order. `UniqueVec<T>` carries uniqueness.

## Why You Use It

- you want canonical ordering to survive as a reusable fact
- you want uniqueness to remain explicit after the initial check
- you need downstream APIs to see the proof-bearing collection directly instead of rechecking it

## Stable Entry Points

- `CanonicalVec<T>`
- `UniqueVec<T>`
- `CanonicalVec::as_slice()`
- `CanonicalVec::proof()`
- `CanonicalVec::into_parts()`
- `UniqueVec::as_slice()`
- `UniqueVec::proof()`
- `UniqueVec::into_parts()`

Important boundary:

- public construction is sealed because these wrappers carry real proof facts

## Core Mental Model

These are not "nicer vectors." They are vectors whose structure has already been proven.

That matters because downstream code can then rely on:

- canonical order being established
- uniqueness being established

without turning those into rechecked conventions.

## How It Executes

Typical usage:

1. some trusted progression surface establishes canonical order or uniqueness
2. the resulting wrapper carries both the vector and the proof fact
3. later code reads through `as_slice()` or consumes through `into_parts()`
4. downstream APIs can require the proven wrapper directly

## Small Example

```rust
use forge_proof::{CanonicalOrder, CanonicalVec, Proof};

type OrderedItems = CanonicalVec<u64>;
type OrderedProof = Proof<CanonicalOrder>;

let _ = std::any::type_name::<OrderedItems>();
let _ = std::any::type_name::<OrderedProof>();
```

This is the smallest honest public example because it shows the stable wrapper vocabulary without pretending callers can mint proof-bearing vectors directly.

## Real Example

```rust
use forge_proof::{CanonicalVec, UniqueVec};

fn inspect<T>(ordered: &CanonicalVec<T>, unique: &UniqueVec<T>) {
    let _ordered_items = ordered.as_slice();
    let _ordered_proof = ordered.proof();

    let _unique_items = unique.as_slice();
    let _unique_proof = unique.proof();
}
```

What this shows:

- the collection and the proof travel together
- inspection stays explicit
- no dynamic proof registry is required to know what has been established

## How It Relates To Other Features

- Pair this with [Proof Markers And Sets](./proof-markers-and-sets.md) because the vector wrappers are concrete proof-bearing containers.
- Pair this with [Structural Facts](./structural-facts.md) because they use `CanonicalOrder` and `Uniqueness`.
- Pair this with [Fixed-Shape Collections](./fixed-shape-collections.md) when you need smaller structural wrappers rather than proof-bearing variable-length collections.

## Inspection And Debugging

- `as_slice()` gives you the collection contents
- `proof()` makes the carried fact visible
- `into_parts()` is the honest owned extraction point when lower-level code needs the vector and proof separately

## Anti-Patterns

- Do not use `CanonicalVec<T>` when canonical order has not actually been established.
- Do not throw away the wrapper and then expect downstream code to remember what was proven.
- Do not re-run uniqueness or ordering checks everywhere if the upstream surface can carry the proven wrapper honestly.

## Current Limits

- construction is sealed
- the wrappers expose proof-bearing structure, not auto-repair logic
- they intentionally model only a small core set of reusable collection facts

## Related Docs

- [Proof Markers And Sets](./proof-markers-and-sets.md)
- [Structural Facts](./structural-facts.md)
- [Fixed-Shape Collections](./fixed-shape-collections.md)
