# Grouped Public Lanes And Front-Door Usage

## What This Feature Is

This feature is the DX-hardened public surface for Milestone 2. It gives you
one common lane, one grouped lower lane, and one stronger lane so first
contact does not require spelunking the flat export wall.

## Why You Use It

- Use this when you want the supported first path into canonicalization.
- Use this when you need to inspect the lower-lane vocabulary by capability
  seam.
- Use this when readiness or proof-bearing closure should stay visibly
  stronger than ordinary authoring.

## Stable Entry Points

Common lane:

- `forge_foundational::canonicalization_api::common_path`

Lower lane:

- `forge_foundational::canonicalization_api::lower_lane::basis`
- `forge_foundational::canonicalization_api::lower_lane::comparison`
- `forge_foundational::canonicalization_api::lower_lane::export`
- `forge_foundational::canonicalization_api::lower_lane::digest`

Stronger lane:

- `forge_foundational::canonicalization_api::stronger_lane`
- `forge_foundational::canonicalization_api::stronger_lane::readiness`

Supporting front doors:

- `canonicalization()`
- `canonicalization().basis()`
- `canonicalization().compare()`
- `canonicalization().export()`
- `canonicalization().digest()`
- `canonicalization().readiness()`

## Core Mental Model

Think of the grouped public surface as three lanes:

- common path: the first thing most callers should use
- lower lane: the inspectable vocabulary when you need exact basis,
  comparison, export, or digest types
- stronger lane: the readiness and proof-bearing closure boundary

Each lane teaches something different on purpose. The grouped inventory in the
readiness artifact even records what each lane teaches and what it must not
hide.

At a high level:

- `common_path` teaches the staged happy path and should be the first thing
  most callers see
- `lower_lane::{basis,comparison,export,digest}` teaches the exact inspectable
  vocabulary by capability seam
- `stronger_lane::readiness` teaches the proof-bearing closure boundary and
  should not be confused with ordinary authoring

## How It Executes

The practical rule is:

1. start at `canonicalization_api::common_path`
2. drop to the matching lower lane only when you need exact vocabulary or
   exact artifacts
3. move to the stronger lane only when you are dealing with readiness
   certification

This keeps the common path easy to discover without pretending the lower or
stronger boundaries do not exist.

## Small Example

```rust
use forge_foundational::canonicalization_api::common_path;

let basis = common_path::canonicalization()
    .basis()
    .at(rule_version)
    .from_state(state)?;
```

This is the smallest honest example because it shows the intended first-contact
surface without dropping straight into lower-lane exports.

## Real Example

```rust
use forge_foundational::canonicalization_api::{common_path, lower_lane, stronger_lane};

let report = common_path::canonicalization().readiness().report();

if common_path::canonicalization().readiness().passes(&report) {
    let lower_basis_type = std::any::type_name::<lower_lane::basis::CanonicalBasisReadyArtifact>();
    let stronger_readiness =
        std::any::type_name::<stronger_lane::readiness::CanonicalProductionTestReadyArtifact>();

    println!("{lower_basis_type}");
    println!("{stronger_readiness}");
}
```

What is authoritative here is not the folder layout alone. The grouped public
surface is frozen in the readiness artifact as part of what Milestone 2 claims
as shipped.

## How It Relates To Other Features

- [Canonical Basis Sequences And Entry Grammar](./canonical-basis-sequences-and-entry-grammar.md)
  is the first capability most common-path callers actually use.
- [Canonical Production Readiness](./canonical-production-readiness.md)
  freezes the grouped public-surface inventory and its supporting proof.
- all the other docs in this folder explain the semantic seams behind each
  lower-lane home.

## Inspection And Debugging

Inspect these first:

- the grouped inventory in the readiness report when you need the frozen lane
  contract
- `canonicalization_api::lower_lane::*` when the common path feels too high
  level
- `canonicalization_api::stronger_lane::readiness` when a plain report is not
  strong enough for the API you are calling

If discoverability still feels weak, check whether you are using the grouped
surface at all. Many problems come from falling back to flat exports too early.

## Anti-Patterns

- Do not teach new callers from the flat root export wall when the grouped
  public surface is the supported first-contact lane.
- Do not use the stronger readiness lane as if it were just another namespace
  for ordinary basis work.
- Do not bypass the common path unless you actually need lower-lane control.

## Current Limits

- The grouped public surface improves discoverability. It does not replace the
  lower-lane types or make their constraints disappear.
- Compatibility flat exports still exist, so older code can remain noisier than
  the preferred grouped path.

## Related Docs

- [Canonical Production Readiness](./canonical-production-readiness.md)
- [Canonical Basis Sequences And Entry Grammar](./canonical-basis-sequences-and-entry-grammar.md)
