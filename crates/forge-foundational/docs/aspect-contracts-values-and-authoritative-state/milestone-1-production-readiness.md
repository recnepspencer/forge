# Milestone 1 Production Readiness

## What This Feature Is

This feature is the machine-checkable closure contract for Milestone 1. It
freezes the public API inventory, compatibility debt, and proof-seed inventory
that later milestones are allowed to assume.

## Why You Use It

- Use this when you need to know exactly what Milestone 1 guarantees today.
- Use this when an adopting crate needs the frozen public API inventory instead
  of inferring it from exports.
- Use this when you need the proof-bearing readiness artifact, not just the
  plain report.

## Stable Entry Points

Readiness report:

- `milestone1_migration_readiness_report()`

Stronger readiness lane:

- `certify_milestone1_production_test_readiness()`
- `require_milestone1_production_test_readiness(...)`

Supporting types:

- `Milestone1MigrationReadinessReport`
- `Milestone1ProductionTestReadyArtifact`
- `Milestone1PublicApiSurface`
- `Milestone1CompatibilityDebt`
- `Milestone1ProofSeed`

## Core Mental Model

The readiness report is not a summary paragraph. It is the milestone closure
artifact.

It answers:

- which public API surfaces Milestone 1 claims as shipped
- which compatibility debt is still explicit
- which proof seeds certify the hard boundary work

The public API surfaces frozen here are:

- `values`
- `aspect_contracts`
- `authoritative_state`
- `authoritative_patches`
- `aspect_common_path`
- `identity_categories`
- `locators`
- `compatibility_bridges`
- `compatibility_common_path`
- `digest_preparation`

The named proof seeds frozen here are:

- `contract_validation`
- `evolution_classification`
- `authoritative_state_admission`
- `patch_admissibility`
- `mask_mode_typing`
- `identity_and_locator_categories`
- `compatibility_lowering`
- `aspect_common_path_front_doors`
- `compatibility_common_path_front_doors`
- `digest_preparation_readiness`

## How It Executes

The report is built from three exact inventories:

1. public API surfaces
2. compatibility debt rows
3. proof seeds

The stronger readiness artifact then wraps that report in a proof-bearing
artifact for production-test closure.

## Small Example

```rust
use forge_foundational::milestone1_migration_readiness_report;

let report = milestone1_migration_readiness_report();
assert_eq!(report.public_api().len(), 10);
```

This is the smallest honest example because most consumers first need the
inventory itself.

## Real Example

```rust
use forge_foundational::{
    certify_milestone1_production_test_readiness,
    milestone1_migration_readiness_report,
};

let report = milestone1_migration_readiness_report();

for surface in report.public_api() {
    println!("{} -> {}", surface.name(), surface.adoption_use());
}

let certified = certify_milestone1_production_test_readiness();
let exact = certified.payload();

assert_eq!(exact.proof_seeds().len(), 10);
```

What is authoritative here is the readiness inventory, not milestone folklore.
This is the named boundary later work is allowed to depend on.

## How It Relates To Other Features

- [Grouped Public Lanes And Common-Path Usage](./grouped-public-lanes-and-common-path-usage.md)
  explains the hardened `aspects()` and `compatibility()` public journeys.
- [Digest Preparation And Canonical Basis](./digest-preparation-and-canonical-basis.md)
  explains the separate public surface frozen here as `digest_preparation`.
- the other docs in this folder describe the surfaces the readiness report
  freezes.

## Inspection And Debugging

Inspect these first:

- `report.public_api()` to see the frozen Milestone 1 public surfaces
- `report.compatibility_debt()` to see what is still transitional
- `report.proof_seeds()` to see the named proof obligations and evidence lanes
- the `json_compatibility_lowering` debt row when you need the exact migration
  boundary and exit condition
- the certified readiness artifact when an API requires stronger proof than the
  plain report

If later work claims Milestone 1 guarantees something else, compare that claim
to the exact inventory here.

## Anti-Patterns

- Do not treat prose docs or closeout notes as stronger than the readiness
  artifact.
- Do not assume JSON compatibility debt is already retired just because the
  bridge exists.
- Do not smuggle the plain readiness report into APIs that require the
  certified readiness artifact.

## Current Limits

- The readiness artifact freezes Milestone 1 itself, not later profile,
  transition, or diagnostics ontology.
- This page tells you what is frozen. It is not the detailed API guide for each
  frozen surface.
- Compatibility lowering is still explicit debt for adopting crates, not the
  ideal forever shape.

## Related Docs

- [Grouped Public Lanes And Common-Path Usage](./grouped-public-lanes-and-common-path-usage.md)
- [Digest Preparation And Canonical Basis](./digest-preparation-and-canonical-basis.md)
- [Compatibility Lowering And JSON Bridges](./compatibility-lowering-and-json-bridges.md)
