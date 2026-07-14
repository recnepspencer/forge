# Artifact

## What This Feature Is

`Artifact<P, T, S, A>` is the crate's general proof-bearing carrier for phase-tagged values. It lets you carry a payload together with an explicit phase, proof set, and assumption basis without hiding any of that structure behind runtime lookup.

## Why You Use It

- you need a proof-bearing carrier that is not specifically a staged recipe
- you want phase, proofs, and basis to travel together as one typed value
- you need borrowed inspection and owned extraction without inventing another wrapper

## Stable Entry Points

- `Artifact::<P, T>::new(payload)`
- `Artifact::payload()`
- `Artifact::proofs()`
- `Artifact::basis()`
- `Artifact::view()`
- `Artifact::into_parts()`
- `ArtifactView<'a, P, T, S, A>`
- `ArtifactParts<T, S, A>`

Good to know:

- `Artifact::with_state(...)` exists only for crate-internal minting and progression
- public callers use stronger artifact forms returned by other surfaces instead of constructing them directly

## DX Posture

This is mostly substrate/reference material.

- there is no separate pleasant-lane `artifact(...)` flow today
- when you work directly with `Artifact`, prefer `use worth_proof::raw::*;`
- if the real problem is staged progression, the pleasant lane starts at `recipe(...)` instead

## Core Mental Model

An artifact is the most general "typed truth bundle" in `worth-proof`.

It always answers four questions:

- what phase is this in
- what is the payload
- what proofs travel with it
- what basis does it currently depend on

`ArtifactView` is the borrowed inspection surface. `ArtifactParts` is the owned extraction surface.

## How It Executes

Artifact usage is simple:

1. construct a payload-only artifact when no proof or basis exists yet
2. inspect it through borrowed accessors or `view()`
3. consume it through `into_parts()` when a lower-level operation needs owned state
4. let crate-managed progression surfaces return stronger artifact forms as needed

## Small Example

```rust
use worth_proof::Artifact;

struct RawPhase;

let artifact = Artifact::<RawPhase, _>::new("payload");

assert_eq!(artifact.payload(), &"payload");
```

This is the smallest honest example because it shows the public constructor that exists for ordinary callers: a payload-only artifact with no extra proof or basis state.

## Real Example

```rust
use worth_proof::{Artifact, ArtifactParts};

struct ValidatedPhase;

fn inspect_and_consume() {
    let artifact = Artifact::<ValidatedPhase, _>::new(String::from("payload"));

    let view = artifact.view();
    assert_eq!(view.payload(), "payload");

    let parts: ArtifactParts<_, _, _> = artifact.into_parts();
    let (payload, _proofs, _basis) = parts.into_parts();

    assert_eq!(payload, "payload");
}
```

What is happening here:

- the artifact owns the payload
- the borrowed view lets you inspect without destructuring
- `ArtifactParts` is the explicit owned extraction boundary
- no hidden clone or proof drop occurs

## How It Relates To Other Features

- Pair it with [Assumption Basis](./assumption-basis.md) when the artifact should carry basis state.
- Pair it with [Proof Markers And Sets](./proof-markers-and-sets.md) when the artifact should carry explicit proof facts.
- Pair it with [Freshness And Downgrade](./freshness-and-downgrade.md) when the carried basis can degrade over time.
- Use [Recipes And Stages](./recipes-and-stages.md) instead when the thing you are modeling is specifically a staged progression flow.

## Inspection And Debugging

- `payload()`, `proofs()`, and `basis()` tell you exactly what the carrier currently holds
- `view()` is the best borrowed inspection surface when you want to observe the whole state honestly
- `into_parts()` is the clearest way to see the owned representation that a lower-level operation will receive

## Anti-Patterns

- Do not treat `Artifact<P, ...>` as a generic escape hatch for every progression problem. Use recipes when stage progression is the real model.
- Do not expect to mint stronger proof-bearing artifact states directly from public code.
- Do not bypass `ArtifactParts` by inventing ad hoc destructuring wrappers.

## Current Limits

- public construction is intentionally minimal
- stronger-state construction is crate-internal
- artifacts encode phase but do not provide the staged recipe lifecycle on their own

## Related Docs

- [Assumption Basis](./assumption-basis.md)
- [Proof Markers And Sets](./proof-markers-and-sets.md)
- [Recipes And Stages](./recipes-and-stages.md)
