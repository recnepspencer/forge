# Profile Production Readiness

## What This Feature Is

This feature is the machine-checkable closure contract for Milestone 3. It
freezes what the profile milestone claims as certified, what compile-fail
boundaries are required, which proof surfaces are allowed, and which runtime
assumptions are still only assumptions.

## Why You Use It

- Use this when you need to know exactly what Milestone 3 guarantees today.
- Use this when an adopting crate wants the frozen public-surface inventory,
  phase gates, or hostile-proof ownership.
- Use this when you need to distinguish shipped profile guarantees from later
  diagnostics, provenance, or adopting-runtime work.

## Stable Entry Points

Readiness report:

- `foundational_profile_milestone3_readiness_report()`

Stronger readiness lane:

- `certify_foundational_profile_milestone3_production_test_readiness()`
- `require_foundational_profile_milestone3_production_test_readiness(...)`

Supporting types:

- `FoundationalProfileProductionReadinessReport`
- `FoundationalProfileProductionTestReadyArtifact`
- `FoundationalProfileCertifiedSurface`
- `FoundationalProfileCompileFailBoundary`
- `FoundationalProfileMilestone3PhaseGate`

## Core Mental Model

The readiness report is not an overview page. It is a closure artifact.

It answers:

- which Milestone 3 surfaces are certified
- which compile-fail boundaries must exist
- which grouped public lanes are frozen as the teachable API
- which proof surfaces are required or forbidden
- which assumptions and residual debt still remain

The grouped public lanes frozen here are:

- `profiles_api::common_path`
- `profiles_api::lower_lane::{composition, progression, attachment, materialization, identity, certification}`
- `profiles_api::stronger_lane`
- `profiles_api::stronger_lane::readiness`

## How It Executes

The report is built from exact milestone inventories:

1. certified surfaces
2. certified-surface evidence
3. hostile pressures
4. compile-fail boundaries
5. proof-lane requirements and forbidden surfaces
6. phase gates
7. grouped public-surface inventory

The stronger readiness artifact then wraps that report in a proof-bearing
artifact for production-test closure.

## Small Example

```rust
use worth_foundational::foundational_profile_milestone3_readiness_report;

let report = foundational_profile_milestone3_readiness_report();
assert!(report.passes_readiness_checklist());
```

This is the smallest honest example because most consumers first need the exact
report, not the stronger artifact.

## Real Example

```rust
use worth_foundational::{
    certify_foundational_profile_milestone3_production_test_readiness,
    foundational_profile_milestone3_readiness_report,
};

let report = foundational_profile_milestone3_readiness_report();

for entry in report.public_surface_inventory() {
    println!("{} -> {:?}", entry.path(), entry.lane());
}

let certified = certify_foundational_profile_milestone3_production_test_readiness();
let exact = certified.payload();

assert!(exact.passes_readiness_checklist());
```

What is authoritative here is the readiness inventory, not a closeout note or
doc summary. The report tells you the exact grouped profile lanes, exact phase
gates, and exact hostile-proof ownership that Milestone 3 is claiming as
shipped.

## How It Relates To Other Features

- [Profile Families And Composed Profile Sets](./profile-families-and-composed-profile-sets.md)
  and the other docs in this folder describe the surfaces the readiness artifact
  freezes.
- `profiles_api::common_path`, `profiles_api::lower_lane::*`, and
  `profiles_api::stronger_lane::readiness` are part of the frozen grouped
  public surface inventory.

## Inspection And Debugging

Inspect these first:

- `report.certified_surfaces()` to see what Milestone 3 actually certifies
- `report.certified_surface_evidence()` to find the owning cert test and
  compile-fail proof
- `report.public_surface_inventory()` to see the frozen common/lower/stronger
  lanes
- `report.phase_gates()` when you need the milestone's linear closure order
- `report.worth_proof_required_surfaces()` and
  `report.worth_proof_forbidden_surfaces()` when you need the exact proof
  boundary, not just the narrative summary
- `report.residual_debt()` when you need to know what was intentionally left
  for later work

## Anti-Patterns

- Do not treat prose docs or closeout notes as stronger than the readiness
  artifact.
- Do not assume later ontology work, adopting-runtime parity, or runtime policy
  lowering is already closed just because Milestone 3 itself is ready.
- Do not smuggle plain readiness reports into APIs that require the certified
  readiness artifact.

## Current Limits

- The readiness artifact freezes Milestone 3, not later diagnostics or
  provenance milestones.
- Residual debt is explicit and should be read as real deferred work, not as
  "probably fine."

## Related Docs

- [Proof-Bearing Profile Certification And Readmission](./proof-bearing-profile-certification-and-readmission.md)
- [Profile Families And Composed Profile Sets](./profile-families-and-composed-profile-sets.md)
