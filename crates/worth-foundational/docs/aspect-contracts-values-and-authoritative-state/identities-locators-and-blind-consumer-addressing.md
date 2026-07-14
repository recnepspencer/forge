# Identities, Locators, And Blind-Consumer Addressing

## What This Feature Is

This feature gives Milestone 1 a typed addressing surface. It covers identity
categories, aspect locators, field locators, mask locators, source locators,
value locators, boundary-artifact locators, and mismatch locators.

## Why You Use It

- Use this when later code needs to point at the same aspect, field, or
  boundary meaning without producer-private coordinates.
- Use this when representation-equal ids should still stay semantically
  distinct.
- Use this when a blind consumer needs typed locations for support, debugging,
  or boundary reporting.
- Use this when source and mismatch loci must stay distinct from ordinary aspect
  or field addressing.

## Stable Entry Points

Common path:

- `aspects().vocabulary().key(...)`
- `aspects().vocabulary().field_key(...)`
- `aspects().vocabulary().field_path(...)`
- `AspectLocator::new(...)`
- `AspectFieldLocator::new(...)`
- `AspectFieldLocator::from_aspect(...)`
- `AspectValueLocator`
- `AspectValueLocator::whole_aspect(...)`
- `AspectValueLocator::struct_field(...)`
- `AspectContractLocator::new(...)`
- `AspectMaskLocator::projection(...)`
- `AspectMaskLocator::mutation(...)`
- `AspectMaskLocator::diagnostic(...)`
- `BoundarySourceLocator::aspect(...)`
- `BoundarySourceLocator::aspect_field(...)`
- `BoundarySourceLocator::boundary_artifact(...)`
- `BoundaryMismatchLocator::aspect(...)`
- `BoundaryMismatchLocator::aspect_field(...)`
- `BoundaryMismatchLocator::boundary_artifact(...)`
- `BoundaryArtifactLocator`
- `BoundarySourceLocator`
- `BoundaryMismatchLocator`
- canonical basis preparation for value and boundary locators through the named
  locator-specific canonicalization basis entry points

Lower lane:

- `BoundaryArtifactId`
- `BoundaryHandle`
- `EquivalenceBasisId`
- `BoundaryEpoch`
- `CanonicalDigestId`
- `AspectLocator`
- `AspectFieldLocator`
- `AspectValueLocator`
- `AspectContractLocator`
- `AspectMaskLocator<_>`
- `BoundarySourceLocator`
- `BoundaryMismatchLocator`
- `LocatorAuthority`

Good to know:

- ids and locators are part of Milestone 1 meaning, not helper clutter.
- field paths are shared between struct law, masks, patches, and locators.
- `aspects().vocabulary().field_path(...)` is the single-field common path for
  Milestone 1 struct targeting; broader canonical field paths remain lower-lane
  locator vocabulary.

## Core Mental Model

Milestone 1 separates:

- identity categories: typed ids that must not collapse into each other
- locators: typed ways to point at aspect or field meaning

That distinction is important. A locator tells you where a piece of meaning is.
An identity category tells you what kind of stable id you are holding.

Milestone 1 also keeps several locator families distinct on purpose:

- value and field locators for aspect-native meaning
- mask locators for projection, mutation, or diagnostic scope
- boundary source locators for where external input came from
- boundary mismatch locators for where parity or admission failed
- value locators for whole-aspect versus struct-field semantic value addressing

## How It Executes

The normal flow is:

1. build canonical aspect keys and field paths through `aspects().vocabulary()`
2. create typed locators for aspects, fields, contracts, or masks
3. use typed ids where a later lane needs stable category-separated identity
4. use source or mismatch locators when boundary-originated work or parity work
   needs blind-consumer-readable addressing
5. pass those locators and ids into compatibility, canonicalization, support,
   or later milestone surfaces

For locator work, this means there are two honest levels of field-path surface:

- common-path `field_path(...)` for one declared struct field
- lower-lane `CanonicalFieldPath::new(...)` when a locator or canonical basis
  lane needs a broader typed path vocabulary

## Small Example

```rust
use worth_foundational::{aspects, AspectLocator, LocatorAuthority};

let task_key = aspects().vocabulary().key("task.summary")?;
let locator = AspectLocator::new(LocatorAuthority::SupportOnly, task_key);
```

This is the smallest honest example because it shows the basic typed aspect
addressing surface directly.

## Real Example

```rust
use worth_foundational::{
    aspects, AspectFieldLocator, AspectLocator, AspectMaskLocator, LocatorAuthority,
};

let vocabulary = aspects().vocabulary();
let aspect = AspectLocator::new(
    LocatorAuthority::SupportOnly,
    vocabulary.key("task.summary")?,
);
let path = vocabulary.field_path(["label"])?;

let field_locator = AspectFieldLocator::from_aspect(aspect.clone(), path.clone());
let mask = aspects().diagnostic_mask().fields(["label"])?;
let diagnostic_mask_locator =
    AspectMaskLocator::diagnostic(aspect.authority(), aspect.aspect_key().clone(), &mask);

assert_eq!(field_locator.aspect(), &aspect);
assert_eq!(diagnostic_mask_locator.aspect_key(), aspect.aspect_key());
```

What is authoritative here is the typed locator and its category, not a local
path string a producer happened to invent.

## How It Relates To Other Features

- [Struct Contracts, Fields, And Field Paths](./struct-contracts-fields-and-field-paths.md)
  defines the field-path surface locators reuse.
- [Projection, Mutation, And Diagnostic Masks](./projection-mutation-and-diagnostic-masks.md)
  uses related mask-path targeting law.
- [Compatibility Lowering And JSON Bridges](./compatibility-lowering-and-json-bridges.md)
  uses `BoundarySourceLocator` to keep JSON-originated input explicitly sourced.

## Inspection And Debugging

Inspect these first:

- the locator category when two locations look structurally similar but behave
  differently
- `LocatorAuthority` when a locator is unexpectedly rejected or downgraded
- the exact field path when field-level targeting appears to drift
- whether the situation really needs an aspect or field locator, a source
  locator, or a mismatch locator

If a blind consumer cannot interpret a location, the first suspect should be
category confusion, not missing payload data.

## Anti-Patterns

- Do not collapse different id categories into one generic integer or byte
  array API.
- Do not use free-form strings where a typed locator exists.
- Do not treat source and mismatch loci as the same concept.
- Do not treat boundary-artifact locators as if they were the same thing as
  aspect-value locators.

## Current Limits

- This layer standardizes Milestone 1 addressing and category identity. Later
  milestones add more specialized locator families.
- Locator meaning is typed, but later runtimes still decide how to present it.

## Related Docs

- [Struct Contracts, Fields, And Field Paths](./struct-contracts-fields-and-field-paths.md)
- [Compatibility Lowering And JSON Bridges](./compatibility-lowering-and-json-bridges.md)
