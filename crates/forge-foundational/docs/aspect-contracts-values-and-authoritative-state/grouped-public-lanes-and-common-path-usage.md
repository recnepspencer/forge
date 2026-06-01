# Grouped Public Lanes And Common-Path Usage

## What This Feature Is

This feature is the DX-hardened public journey for Milestone 1. It gives you a
named `aspects()` lane for native authoring and a named `compatibility()` lane
for JSON-originated lowering.

## Why You Use It

- Use this when you want the supported first path into Milestone 1.
- Use this when you need one discoverable authoring journey for scalar,
  struct, mask, validation, state, and patch flow.
- Use this when compatibility lowering must stay visibly separate from native
  authoring.

## Stable Entry Points

Native common path:

- `aspects().contract()`
- `aspects().struct_fields()`
- `aspects().mask_contract()`
- `aspects().projection_mask()`
- `aspects().mutation_mask()`
- `aspects().diagnostic_mask()`
- `aspects().validate()`
- `aspects().authoritative_state()`
- `aspects().patch()`
- `aspects().vocabulary()`

Compatibility common path:

- `compatibility().json()`
- `.input(...)`
- `.lower_value(...)`
- `.lower_state(...)`

Readiness-facing grouped surfaces:

- `aspect_common_path`
- `compatibility_common_path`

## Core Mental Model

Milestone 1 now has two named public journeys:

- `aspects()` for native aspect meaning and authority flow
- `compatibility()` for explicit JSON bridge lowering

They are related, but they are not interchangeable. The whole point of the DX
hardening work was to make that obvious from the public surface.

This page is about the grouped first-contact lanes only. It is not the
reference for canonical basis preparation, readiness closure, or lower-level
type inventories.

## How It Executes

The practical rule is:

1. start at `aspects()` for native contract, value, validation, state, and
   patch work
2. start at `compatibility().json()` only when the input really is
   compatibility-shaped JSON
3. keep JSON lowering explicit until the output lands on native validated or
   authoritative meaning

The `aspect_common_path` surface specifically exists so one public lane teaches:

- contract authoring
- struct-field authoring
- mask authoring
- value and field-path vocabulary
- validation
- authoritative state admission
- patch construction and application

The `compatibility_common_path` surface exists so JSON lowering cannot quietly
pretend to be the same lane.

If you later need canonical ordering or digest input, leave these common paths
and move into the `canonicalization()` surface explicitly.

## Small Example

```rust
use forge_foundational::aspects;

let vocabulary = aspects().vocabulary();
let key = vocabulary.key("retry.count")?;
```

This is the smallest honest example because it shows the intended first-contact
lane for native Milestone 1 work.

## Real Example

```rust
use forge_foundational::{aspects, compatibility, ScalarAspectType};

let vocabulary = aspects().vocabulary();
let contract = aspects()
    .contract()
    .for_key(vocabulary.key("retry.count")?)
    .identified_by(vocabulary.identity(7))
    .at_revision(vocabulary.revision(1))
    .scalar(ScalarAspectType::Int64);

let native = aspects().validate().against(&contract);
let json_lane = compatibility().json();

let _ = native;
let _ = json_lane;
```

What is authoritative here is not the folder layout or the milestone closeout.
It is the shipped public journey the readiness artifact names as
`aspect_common_path` and `compatibility_common_path`.

## How It Relates To Other Features

- [Compatibility Lowering And JSON Bridges](./compatibility-lowering-and-json-bridges.md)
  covers the explicit bridge lane in detail.
- [Digest Preparation And Canonical Basis](./digest-preparation-and-canonical-basis.md)
  covers the separate canonicalization surface that starts after Milestone 1
  meaning is already established.
- [Milestone 1 Production Readiness](./milestone-1-production-readiness.md)
  freezes the public API inventory and proof seeds for this milestone.
- the other docs in this folder explain the specific capability seams behind the
  common lanes.

## Inspection And Debugging

Inspect these first:

- the native `aspects()` lane when a workflow should be contract-first and
  authoritative-state-first
- the `compatibility()` lane when legacy JSON is entering the system
- the Milestone 1 readiness report when you need the named public API and proof
  seed inventory

If first-contact usage still feels confusing, check whether the code is
skipping back to flat exports too early instead of following the named lanes.

## Anti-Patterns

- Do not teach new callers from the full flat export wall when `aspects()` and
  `compatibility()` are the hardened public journeys.
- Do not run native authoring through `compatibility()` out of convenience.
- Do not assume `aspects()` or `compatibility()` already explain canonical
  digest preparation just because readiness names those grouped surfaces.
- Do not hide patch application or state admission behind one giant helper.

## Current Limits

- The common paths improve discoverability. They do not eliminate the lower
  milestone-owned types behind them.
- The readiness artifact still names explicit JSON compatibility debt, so this
  is not the end state for every adopting runtime.

## Related Docs

- [Compatibility Lowering And JSON Bridges](./compatibility-lowering-and-json-bridges.md)
- [Digest Preparation And Canonical Basis](./digest-preparation-and-canonical-basis.md)
- [Milestone 1 Production Readiness](./milestone-1-production-readiness.md)
