# Platform Entry Closeout

## What This Feature Is

The platform-entry closeout boundary is Query's machine-checkable certification
surface for the public platform-entry product bundle.

Use it when you need one authoritative answer to questions like:

- did the live public surface, docs coverage, parity proofs, hostile proofs,
  and UI boundary suite all stay aligned
- which digest represents the current closed public product
- which proof class is missing when a closeout claim is no longer honest

This is not another feature-runtime surface like orchestration or recovery.
It is the certification layer that consumes those public surfaces and proves
that they close together as one public product.

## Why You Use It

- inspect one certification readout instead of checking inventory, docs coverage,
  trybuild, parity tests, and hostile tests separately
- get one bundle of digests for release-grade certification or tooling
- verify that helper, grouped, continuation, signal, contribution-composed,
  and recovery surfaces are still covered by the same public closure story
- detect drift between the live surface and the proof surfaces that are
  supposed to certify it

## Stable Entry Points

- `forge_query_platform_entry_closeout_surface()`
- `certify_platform_entry_closeout()`
- `ForgeQueryPlatformEntryCloseoutSurface`
- `ForgeQueryPlatformEntryCloseoutBundle`
- `ForgeQueryPlatformEntryCloseoutOutput`
- `ForgeQueryPlatformEntryAlignmentAudit`
- `ForgeQueryPlatformEntryCompileFailManifest`
- `ForgeQueryPlatformEntryCompileFailAudit`
- `ForgeQueryPlatformEntryUiProofRow`
- `ForgeQueryPlatformEntryUiProofKind`
- `ForgeQueryPlatformEntryParityManifest`
- `ForgeQueryPlatformEntryParityRow`
- `ForgeQueryPlatformEntryParityLane`
- `ForgeQueryPlatformEntryParityAssertionClass`
- `ForgeQueryPlatformEntryParityAudit`
- `ForgeQueryPlatformEntryHostileManifest`
- `ForgeQueryPlatformEntryHostileRow`
- `ForgeQueryPlatformEntryHostileDivergenceClass`
- `ForgeQueryPlatformEntryHostileAudit`
- `forge_query_platform_entry_compile_fail_manifest()`
- `forge_query_platform_entry_compile_fail_boundary_digest()`
- `forge_query_platform_entry_parity_manifest()`
- `forge_query_platform_entry_hostile_manifest()`

## Core Mental Model

Think of this boundary as the certification ledger for Query's platform-entry
product bundle.

It consumes five proof classes:

- live public breadth from orchestration inventory
- docs and golden breadth from public doc coverage
- compile-checked UI proofs from the domain-handle golden/boundary suite
- parity rows for surfaces that must converge
- hostile rows for surfaces that must stay observably different

The closeout surface tells you whether those proof classes still agree.

The certification bundle turns that agreement into one output manifest with stable
digest keys.

The current output manifest is:

- `public_surface_digest`
- `compile_fail_boundary_digest`
- `parity_digest`
- `hostile_digest`
- `docs_coverage_digest`
- `milestone_closeout_digest`

## How It Executes

`forge_query_platform_entry_closeout_surface()` does not rediscover the world
by scanning docs or tests heuristically.

It consumes the authority surfaces directly:

1. `ForgeQueryOrchestrationSurfaceInventory::current()`
2. `ForgeQueryPublicDocCoverageInventory::current()`
3. `forge_query_platform_entry_compile_fail_manifest()`
4. `forge_query_platform_entry_parity_manifest()`
5. `forge_query_platform_entry_hostile_manifest()`

Then it builds:

- inventory alignment
- docs-coverage alignment
- compile-fail audit
- parity audit
- hostile audit
- one final certification digest over those digests

`certify_platform_entry_closeout()` then emits the stable output manifest over
that surface.

## Small Example

```rust
let closeout = forge_query_platform_entry_closeout_surface();

assert!(closeout.inventory_alignment().is_aligned());
assert!(closeout.docs_coverage_alignment().is_aligned());
assert!(closeout.compile_fail_audit().missing_surfaces().is_empty());
assert!(closeout.parity_audit().missing_equivalence_rows().is_empty());
assert!(closeout.hostile_audit().missing_divergence_rows().is_empty());
```

## Real Example

Use the bundle when you need a release-grade readout over the current
platform-entry proof surface.

```rust
let bundle = certify_platform_entry_closeout();

assert_eq!(
    bundle.output_manifest(),
    &[
        "public_surface_digest",
        "compile_fail_boundary_digest",
        "parity_digest",
        "hostile_digest",
        "docs_coverage_digest",
        "milestone_closeout_digest",
    ]
);

let final_digest = bundle
    .output_digest("milestone_closeout_digest")
    .expect("final certification digest should exist");

assert!(!final_digest.is_empty());
```

If one proof class drifts, inspect the surface first:

- `inventory_alignment().gaps()`
- `docs_coverage_alignment().gaps()`
- `compile_fail_audit().missing_surfaces()`
- `parity_audit().missing_proof_anchors()`
- `hostile_audit().missing_proof_anchors()`

That gives you the failing proof class before you dig through the underlying
test or inventory boundary.

## How It Relates To Other Features

- [Orchestration Inventory](./orchestration-inventory.md) owns the live public
  breadth this closeout surface closes over.
- [Public Doc Coverage](./public-doc-coverage.md) owns the doc/golden teaching
  breadth this closeout surface closes over.
- [Recovery Boundary](./recovery-boundary.md), [Family Helpers](./family-helpers.md),
  [Grouped Authoring](./grouped-authoring.md), [Continuation Pipeline](./continuation-pipeline.md),
  [Signal Compatibility Orchestration](./signal-compatibility-orchestration.md),
  and [Contribution-Composed Orchestration](./contribution-composed-orchestration.md)
  are all part of the public product this surface certifies.
- [Certification Surface And Closeout Bundle](./certification/certification-surface-and-closeout-bundle.md)
  describes the broader certification bundle patterns this page builds on.
- [Goldens, Boundaries, And Hostile Certification](./certification/goldens-boundaries-and-hostile-certification.md)
  explains the proof styles this closeout surface consumes.

## Inspection And Debugging

Useful closeout-surface accessors:

- `public_surface_digest()`
- `docs_coverage_digest()`
- `compile_fail_boundary_digest()`
- `parity_digest()`
- `hostile_digest()`
- `inventory_alignment()`
- `docs_coverage_alignment()`
- `compile_fail_audit()`
- `parity_audit()`
- `hostile_audit()`
- `closeout_surface_digest()`

Useful bundle accessors:

- `output_manifest()`
- `surface()`
- `outputs()`
- `output_digest(...)`
- `milestone_closeout_digest()`

Useful audit details:

- compile-fail:
  - `missing_surfaces()`
  - `missing_paths()`
  - `orphan_rows()`
- parity:
  - `missing_equivalence_rows()`
  - `unknown_surfaces()`
  - `missing_proof_paths()`
  - `missing_proof_anchors()`
- hostile:
  - `missing_divergence_rows()`
  - `unknown_surfaces()`
  - `missing_proof_paths()`
  - `missing_proof_anchors()`

## Anti-Patterns

- treating a green `cargo test` run as equivalent to a stable certification bundle
- adding a new public orchestration/helper/grouped surface without updating the
  underlying inventory or docs coverage surfaces this closeout seam consumes
- certifying a parity claim that is broader than the current real behavior
- pointing a parity or hostile row at a file that exists but does not contain
  the named proof
- using this closeout bundle as a substitute for the lower proof surfaces when
  you actually need to inspect the exact drift class

## Current Limits

- this boundary currently certifies the public platform-entry product bundle,
  not every Query certification surface
- the compile-fail manifest is intentionally tied to the domain-handle UI
  golden/boundary suite rather than a second trybuild world
- parity and hostile manifests are explicit proof ledgers; they do not
  synthesize new equivalence or divergence claims automatically

## Related Docs

- [Domain Capabilities](./README.md)
- [Orchestration Inventory](./orchestration-inventory.md)
- [Public Doc Coverage](./public-doc-coverage.md)
- [Recovery Boundary](./recovery-boundary.md)
- [Family Helpers](./family-helpers.md)
- [Grouped Authoring](./grouped-authoring.md)
- [Continuation Pipeline](./continuation-pipeline.md)
- [Signal Compatibility Orchestration](./signal-compatibility-orchestration.md)
- [Contribution-Composed Orchestration](./contribution-composed-orchestration.md)
- [Certification Surface And Closeout Bundle](./certification/certification-surface-and-closeout-bundle.md)
- [Goldens, Boundaries, And Hostile Certification](./certification/goldens-boundaries-and-hostile-certification.md)
