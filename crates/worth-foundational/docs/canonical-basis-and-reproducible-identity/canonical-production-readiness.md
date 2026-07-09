# Canonical Production Readiness

## What This Feature Is

This feature is the machine-checkable closure contract for Milestone 2. It
freezes which canonicalization surfaces are certified, which hostile pressures
and compile-fail boundaries exist, and which grouped public lanes are part of
the shipped teachable API.

## Why You Use It

- Use this when you need to know exactly what Milestone 2 guarantees today.
- Use this when an adopting crate needs the frozen grouped public-surface
  inventory or the exact hostile-proof ownership.
- Use this when you need to distinguish shipped canonical guarantees from later
  runtime adoption or later ontology work.

## Stable Entry Points

Readiness report:

- `canonical_milestone2_production_readiness_report()`

Stronger readiness lane:

- `certify_canonical_milestone2_production_readiness()`
- `require_canonical_production_test_readiness(...)`

Supporting types:

- `CanonicalProductionReadinessReport`
- `CanonicalProductionTestReadyArtifact`
- `CanonicalCertifiedSurface`
- `CanonicalCompileFailBoundary`
- `CanonicalMilestone2PhaseGate`
- `CanonicalPublicSurfaceEntry`

## Core Mental Model

The readiness report is not a narrative summary. It is a closure artifact.

It answers:

- which canonicalization surfaces are certified
- which hostile pressures were used to prove them
- which compile-fail boundaries are required
- which golden artifacts, property seeds, and harness lanes are frozen
- which grouped public lanes are part of the shipped surface
- which assumptions, non-assumptions, and residual debt remain

The certified surfaces frozen here are:

- basis grammar
- Milestone 1 basis builders
- equivalence basis
- mismatch basis
- export bundles
- digest algorithm slots

The grouped public lanes frozen here are:

- `canonicalization_api::common_path`
- `canonicalization_api::lower_lane::{basis, comparison, export, digest}`
- `canonicalization_api::stronger_lane`
- `canonicalization_api::stronger_lane::readiness`

## How It Executes

The report is built from exact milestone inventories:

1. certified surfaces and their evidence
2. hostile pressures
3. compile-fail boundaries
4. golden artifacts and fixture manifest coverage
5. property seeds and harness expansion points
6. phase gates
7. grouped public-surface inventory
8. assumptions, non-assumptions, and residual debt

The stronger readiness artifact then wraps that report in a proof-bearing
artifact for production-test closure.

## Small Example

```rust
use worth_foundational::canonical_milestone2_production_readiness_report;

let report = canonical_milestone2_production_readiness_report();
assert!(report.passes_readiness_checklist());
```

This is the smallest honest example because most consumers first need the
exact report, not the stronger artifact.

## Real Example

```rust
use worth_foundational::{
    canonical_milestone2_production_readiness_report,
    certify_canonical_milestone2_production_readiness,
};

let report = canonical_milestone2_production_readiness_report();

for entry in report.public_surface_inventory() {
    println!("{} -> {:?}", entry.path(), entry.lane());
}

let certified = certify_canonical_milestone2_production_readiness();
let exact = certified.payload();

assert!(exact.passes_readiness_checklist());
```

What is authoritative here is the readiness inventory, not a closeout note or
doc summary. The report tells you the exact grouped lanes, exact phase gates,
and exact hostile-proof ownership that Milestone 2 is claiming as shipped.

## How It Relates To Other Features

- the other docs in this folder describe the capabilities the readiness report
  freezes
- `canonicalization_api::common_path`,
  `canonicalization_api::lower_lane::*`, and
  `canonicalization_api::stronger_lane::readiness` are part of the frozen
  grouped public surface inventory

## Inspection And Debugging

Inspect these first:

- `report.certified_surfaces()` to see what Milestone 2 actually certifies
- `report.certified_surface_evidence()` to find the owning cert test and
  compile-fail proof
- `report.public_surface_inventory()` to see the frozen common, lower, and
  stronger lanes
- `report.golden_artifacts()` and `report.property_seeds()` when you need the
  exact proof fixtures and hostility dimensions
- `report.harness_expansion_points()` when you need the named replay,
  hostility, parity, or grouped-surface harness lanes
- `report.compile_fail_boundaries()` when you need the exact misuse boundaries
  the milestone promises to fail closed
- `report.phase_gates()` when you need the milestone's linear closure order
- `report.assumptions()`, `report.non_assumptions()`, and
  `report.residual_debt()` when you need the real boundary of what is and is
  not closed

If a readiness claim feels too broad, compare it to the exact inventory here.
The report is the stronger source.

## Anti-Patterns

- Do not treat prose docs or closeout notes as stronger than the readiness
  artifact.
- Do not assume real runtime adoption parity is already proven just because
  canonicalization itself is ready.
- Do not smuggle plain readiness reports into APIs that require the certified
  readiness artifact.

## Current Limits

- The readiness artifact freezes Milestone 2, not later diagnostics, profile,
  or provenance milestones.
- Residual debt is explicit and should be treated as real deferred work, not
  "probably fine."

## Related Docs

- [Grouped Public Lanes And Front-Door Usage](./grouped-public-lanes-and-front-door-usage.md)
- [Digest Derivation And Slot Semantics](./digest-derivation-and-slot-semantics.md)
