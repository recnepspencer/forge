# Public Doc Coverage

## What This Feature Is

The public doc coverage boundary is Query's published registry for the teaching
surface of its public product.

Use it when you need to answer questions like:

- which public orchestration surfaces are actually documented
- which golden readout is supposed to certify a surface
- whether a surface is discoverable from the docs index
- whether a surface participates in a named user-facing journey
- whether the current docs/goldens set drifted away from the live public API

This is not another orchestration registry. It is the documentation-and-goldens
layer that projects from the live orchestration inventory and turns "we wrote
some pages" into a real contract.

## Why You Use It

- inspect the real teaching coverage for the public Query surface
- audit missing docs, missing goldens, README discovery gaps, and journey gaps
- distinguish surface goldens from coverage-boundary readouts
- keep helper and grouped-authoring surfaces visible as first-class documented
  rows instead of letting them hide under generic prose
- treat docs and goldens as part of the public product instead of as loose
  supporting files

## Stable Entry Points

- `ForgeQueryPublicDocCoverageInventory::current()`
- `ForgeQueryPublicDocCoverageInventory::rows()`
- `ForgeQueryPublicDocCoverageInventory::row_for_public_name(...)`
- `ForgeQueryPublicDocCoverageAudit::current()`
- `ForgeQueryPublicDocCoverageAudit::from_inventory(...)`
- `ForgeQueryPublicDocCoverageRow`
- `ForgeQueryPublicDocReference`
- `ForgeQueryPublicGoldenTranscript`
- `ForgeQueryPublicGoldenTranscriptKind`
- `ForgeQueryPublicJourneyKind`
- `forge_query_public_doc_coverage_golden_transcripts()`
- `forge_query_public_doc_coverage_golden_transcript_digest()`

## Core Mental Model

Think of this feature as the public teaching ledger for Query.

The orchestration inventory tells you what public surface exists. Public doc
coverage tells you how that surface is taught and certified.

Each coverage row says:

- which public verb or public surface row it belongs to
- which feature page should teach it
- which README label should make it discoverable
- which golden readout is supposed to cover it
- which user-facing journey it belongs to

The current journey families are:

- `PlatformEntry`
- `Continuation`
- `SignalFacing`
- `ContributionComposed`
- `HelperProjection`
- `GroupedAuthoring`

The current golden kinds are:

- `SurfaceCoverage`
  - a golden readout for one real public surface family
- `CoverageBoundaryReadout`
  - a golden readout for the docs-coverage boundary itself

The important rule is:

- coverage rows are not a loose docs index
- coverage rows are the audit contract for feature docs, golden readouts, and
  discovery coverage

## How It Executes

`ForgeQueryPublicDocCoverageInventory::current()` starts from
`ForgeQueryOrchestrationSurfaceInventory::current()`.

For each live orchestration row, it builds one coverage row with:

- the live public name and canonical base name
- orchestration family and visibility
- the source surface digest
- the feature-doc reference
- the README discovery label
- the golden transcript reference
- the journey classification

`ForgeQueryPublicDocCoverageAudit::current()` then checks that published
coverage against the live public surface and the checked-in docs/golden files.

Today the audit verifies at least these drift classes:

- live public surfaces with no valid feature-doc coverage
- live public surfaces with no valid surface-coverage golden
- coverage rows for surfaces that no longer exist
- surface-coverage golden manifest rows that are no longer used by live rows
- README discovery labels that are missing from the docs index
- journey mismatches between a coverage row and its golden

That is what makes this a real anti-drift boundary instead of just another
markdown page.

## Small Example

```rust
let coverage = ForgeQueryPublicDocCoverageInventory::current();

let row = coverage
    .row_for_public_name("orchestrate_signal_compatibility_outcome")
    .expect("signal outcome surface should be documented");

assert_eq!(
    row.doc_reference().path(),
    "crates/forge-query/docs/domain-capabilities/signal-compatibility-orchestration.md"
);
assert_eq!(row.readme_discovery_label(), "Signal Compatibility Orchestration");
assert!(row.has_golden_transcript());
assert!(row.has_journey_coverage());
```

## Real Example

Use the audit when you need to treat docs and goldens as part of release-grade
surface certification.

```rust
let audit = ForgeQueryPublicDocCoverageAudit::current();

assert!(audit.undocumented_public_surfaces().is_empty());
assert!(audit.surfaces_missing_goldens().is_empty());
assert!(audit.orphan_doc_rows().is_empty());
assert!(audit.orphan_golden_rows().is_empty());
assert!(audit.readme_discovery_gaps().is_empty());
assert!(audit.journey_coverage_gaps().is_empty());
```

You can also inspect the golden manifest directly:

```rust
for golden in forge_query_public_doc_coverage_golden_transcripts() {
    let _ = golden.label();
    let _ = golden.path();
    let _ = golden.dx_focus();
    let _ = golden.kind();
    let _ = golden.journey();
}
```

## How It Relates To Other Features

- [Orchestration Inventory](./orchestration-inventory.md) is the authority for
  what public orchestration surfaces exist. Public doc coverage projects from
  it.
- [Family Helpers](./family-helpers.md) and [Grouped Authoring](./grouped-authoring.md)
  now have explicit documented coverage rows instead of being treated as side
  notes on generic orchestration pages.
- [Recovery Boundary](./recovery-boundary.md) participates in the public
  teaching story through journey coverage even though this inventory is focused
  on orchestration-surface publication.
- [Certification Surface And Closeout Bundle](./certification/certification-surface-and-closeout-bundle.md)
  and [Goldens, Boundaries, And Hostile Certification](./certification/goldens-boundaries-and-hostile-certification.md)
  remain the broader certification docs around the proof surface that these
  rows point into.
- [Platform Entry Closeout](./platform-entry-closeout.md) consumes this
  coverage inventory and its audit directly when it certifies the broader
  public product bundle.

## Inspection And Debugging

Useful inventory accessors:

- `source_inventory_digest()`
- `coverage_digest()`
- `rows()`
- `row_for_public_name(...)`

Useful row accessors:

- `public_name()`
- `canonical_base_name()`
- `orchestration_family()`
- `visibility()`
- `surface_row_digest()`
- `doc_reference()`
- `readme_discovery_label()`
- `golden_transcript()`
- `journey()`
- `has_golden_transcript()`
- `has_readme_discovery()`
- `has_journey_coverage()`
- `coverage_digest()`

Useful audit accessors:

- `coverage_digest()`
- `undocumented_public_surfaces()`
- `surfaces_missing_goldens()`
- `orphan_doc_rows()`
- `orphan_golden_rows()`
- `readme_discovery_gaps()`
- `journey_coverage_gaps()`

Useful golden-manifest accessors:

- `label()`
- `path()`
- `dx_focus()`
- `kind()`
- `journey()`

## Anti-Patterns

- treating "the doc file exists" as equivalent to real feature coverage
- adding a surface golden file without registering it in the golden manifest
- letting helper or grouped surfaces hide under generic parent-page prose
- using this inventory as a substitute for the stronger orchestration inventory
  or certification surfaces it depends on
- assuming README discoverability is fine without checking the actual label
  coverage
- treating the coverage-boundary readout golden as if it were itself a live
  surface golden

## Current Limits

- this boundary currently covers the public orchestration/helper/grouped
  surface published through the orchestration inventory
- the audit proves file existence, section presence, golden-kind alignment, and
  journey alignment, but it is still a code-level certification surface rather
  than a rendered docs site pipeline
- the current golden catalog is focused on surface readouts and coverage
  boundary readouts, not on every possible tutorial or narrative example

## Related Docs

- [Domain Capabilities](./README.md)
- [Orchestration Inventory](./orchestration-inventory.md)
- [Family Helpers](./family-helpers.md)
- [Grouped Authoring](./grouped-authoring.md)
- [Signal Compatibility Orchestration](./signal-compatibility-orchestration.md)
- [Contribution-Composed Orchestration](./contribution-composed-orchestration.md)
- [Declaration Entry Orchestration](./declaration-entry-orchestration.md)
- [Recovery Boundary](./recovery-boundary.md)
- [Platform Entry Closeout](./platform-entry-closeout.md)
- [Certification Surface And Closeout Bundle](./certification/certification-surface-and-closeout-bundle.md)
- [Goldens, Boundaries, And Hostile Certification](./certification/goldens-boundaries-and-hostile-certification.md)
