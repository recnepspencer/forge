# Structural Facts

## What This Feature Is

Structural facts are the built-in proof kinds that `worth-proof` uses to represent common zero-cost invariants. They are named proof markers that can travel in `Proof<P, StructuralProofAuthority>` or larger proof-bearing forms.

## Why You Use It

- you need a stable vocabulary for common proven structure
- you want containers and composed forms to carry real proof facts instead of comments
- you need a shared invariant language across crates without dynamic proof lookup

## Stable Entry Points

- `CanonicalOrder`
- `Uniqueness`
- `Disjointness`
- `Normalization`
- `StructuralProofAuthority`

These all implement `ProofMarker` and are carried through proof-bearing surfaces such as:

- `Proof<CanonicalOrder, StructuralProofAuthority>`
- `CanonicalVec<T>`
- `UniqueVec<T>`
- `DisjointPair<T>`
- `LoweredFamilyProgram2<...>`

## DX Posture

This is raw-substrate/reference vocabulary.

- there is no separate pleasant-lane structural-fact constructor story
- the pleasant lane teaches these facts through carriers like `family_pair(...).lower_by(...)` and ready/proven wrappers
- when you work directly with structural fact names or proof carriers, prefer `use worth_proof::raw::*;`

## Core Mental Model

Structural facts are reusable proof names for "this shape property has already been established."

Use them when the fact is:

- stable
- reusable downstream
- valuable enough that re-proving it repeatedly would be wasteful or blurry

## How It Executes

A structural fact usually appears in one of two ways:

1. directly, as a carried proof like `Proof<CanonicalOrder, StructuralProofAuthority>`
2. indirectly, embedded in a stronger container or lowered program that exposes a `proof()` accessor

The public API is mostly about consuming or preserving these facts, not minting them directly.
`StructuralProofAuthority` is a nameable zero-variant enum: it can identify the
authority in proof types, but no safe runtime value of the authority can exist.
Its public-value exemption is justified entirely by that representation; uses
of the name do not need a second source-level posture model.

## Small Example

```rust
use worth_proof::{CanonicalOrder, Proof, StructuralProofAuthority};

type CanonicalOrderProof = Proof<CanonicalOrder, StructuralProofAuthority>;
let _ = std::any::type_name::<CanonicalOrderProof>();
```

This is the smallest honest example because the structural fact itself is stable and public, even though public proof minting is not.

## Real Example

```rust
use worth_proof::{
    lower_deterministic_family_pair, AuthoritativeFamilyMember, CompositionFamilySymbol,
    FamilyLifecycleAction, Pair,
};

fn family_action_key(
    action: &FamilyLifecycleAction<u8, u16, &'static str>,
) -> (u8, Option<u8>, Option<u16>) {
    match action {
        FamilyLifecycleAction::Retire { target } => (0, None, Some(*target.value())),
        FamilyLifecycleAction::Rewrite { target, .. } => (1, None, Some(*target.value())),
        FamilyLifecycleAction::Supersede { target, .. } => (2, None, Some(*target.value())),
        FamilyLifecycleAction::Create { symbol, .. } => (3, Some(*symbol.value()), None),
    }
}

fn lower() {
    let lowered = lower_deterministic_family_pair(
        Pair::new(
            FamilyLifecycleAction::Create {
                symbol: CompositionFamilySymbol::new(2_u8),
                payload: "create",
            },
            FamilyLifecycleAction::Retire {
                target: AuthoritativeFamilyMember::new(11_u16),
            },
        ),
        family_action_key,
    );

    let _canonical_order = lowered.proof();
}
```

What this shows:

- deterministic family lowering produces a canonical-order proof-bearing result
- the proof fact is part of the public result shape
- callers do not need to guess whether ordering was established

## How It Relates To Other Features

- Pair this with [Proof Markers And Sets](./proof-markers-and-sets.md) for the carrier model.
- Pair this with [Fixed-Shape Collections](./fixed-shape-collections.md) and [Proven Vectors](./proven-vectors.md) for concrete containers that carry these facts.
- Pair this with [Recipes And Stages](./recipes-and-stages.md) when stage progression yields structurally stronger forms.

## Inspection And Debugging

- look for `proof()` accessors on proof-bearing wrappers
- look for explicit proof positions in owned extraction APIs
- if downstream code depends on a structural fact, prefer a surface that carries it instead of relying on re-sorting or re-checking

## Anti-Patterns

- Do not use a structural fact name when the underlying invariant was not actually established.
- Do not re-prove canonical order or uniqueness locally when the upstream surface can carry it honestly.
- Do not turn these facts into string labels or runtime flags.

## Current Limits

- the built-in set is intentionally small
- the crate does not provide a generic runtime proof engine
- structural facts are reusable vocabulary, not a replacement for domain-specific semantic law

## Related Docs

- [Proof Markers And Sets](./proof-markers-and-sets.md)
- [Fixed-Shape Collections](./fixed-shape-collections.md)
- [Proven Vectors](./proven-vectors.md)
