# Proof Markers And Sets

## What This Feature Is

Proof markers and proof sets are how `forge-proof` represents established facts without hiding them in comments or dynamic registries. A proof-bearing form can carry one fact with `Proof<P, A>` or a statically known group of facts with `ProofSetCons<Head, Tail>`, where `A` is the authority that is allowed to prove `P`.

## Why You Use It

- you need to preserve that a fact was established
- you want multiple facts to travel together without a dynamic proof bag
- you need stronger forms to carry explicit structural truth

## Stable Entry Points

- `ProofMarker`
- `AuthorityProves<P>`
- `NoProofs`
- `Proof<P, A>`
- `ProofSet`
- `ProofSetAuthorizedBy<Auth>`
- `ProofSetCons<Head, Tail>::new(head, tail)`
- `ProofSetCons::head()`
- `ProofSetCons::tail()`

Important boundary:

- public code can observe and carry `Proof<P, A>`
- public code cannot mint stronger proof-bearing forms directly
- proof minting requires an authority that implements `AuthorityProves<P>`

## DX Posture

This is raw-substrate reference material.

- there is no separate pleasant-lane proof-construction surface
- the pleasant lane teaches proof carriage indirectly through stronger forms and grouped reads
- when you work directly with proof carriers or proof-set structure, prefer `use forge_proof::raw::*;`

## Core Mental Model

A proof is not a capability token and not just a marker trait.

It means:

- a specific fact has already been established

Examples include:

- canonical order
- uniqueness
- disjointness
- normalization

Proof sets let you compose those facts statically. The crate deliberately avoids a dynamic "proof map" model.

## How It Executes

The usual flow is:

1. a crate-managed progression surface establishes a fact
2. the resulting form carries `Proof<P, A>` or a proof set
3. later code inspects or preserves that proof-bearing state
4. owned extraction keeps proof carriage explicit instead of silently dropping it

## Small Example

```rust
use forge_proof::{NoProofs, ProofSetCons};

let proofs = ProofSetCons::new(NoProofs, NoProofs);
let _ = proofs.head();
```

This is the smallest honest public example because it demonstrates proof-set structure without implying that ordinary callers can mint stronger proof facts.

## Real Example

```rust
use forge_proof::{Artifact, ArtifactParts, NoProofs};

struct RawPhase;

fn preserve_carried_state() {
    let artifact = Artifact::<RawPhase, _>::new("payload");
    let parts: ArtifactParts<_, _, _> = artifact.into_parts();

    let (_payload, _proofs, _basis) = parts.into_parts();
}
```

What this shows:

- proof-bearing state is always part of the owned extraction surface
- even a proof-free form still exposes its proof position honestly
- higher-level stronger forms use that same explicit structure rather than magic side channels

## How It Relates To Other Features

- Pair this with [Structural Facts](./structural-facts.md) to see the built-in proof kinds.
- Pair this with [Proven Vectors](./proven-vectors.md) and [Fixed-Shape Collections](./fixed-shape-collections.md) to see public proof-carrying containers.
- Pair this with [Artifact](./artifact.md) when a phase-tagged carrier should transport proofs explicitly.

## Inspection And Debugging

- `head()` and `tail()` let you inspect proof-set structure without flattening it
- proof-bearing container APIs typically expose `proof()` or keep proofs available through owned extraction
- if you cannot point to where a proof is carried, the abstraction is probably too blurry

## Anti-Patterns

- Do not describe proof facts only in prose when the API can carry them explicitly.
- Do not invent a dynamic proof registry for surfaces that should remain statically known.
- Do not use witnesses where a carried proof fact is the real invariant.

## Current Limits

- proof minting is sealed
- proof sets are intentionally static and explicit
- the crate ships only a small core vocabulary of structural proof kinds

## Related Docs

- [Structural Facts](./structural-facts.md)
- [Artifact](./artifact.md)
- [Proven Vectors](./proven-vectors.md)
