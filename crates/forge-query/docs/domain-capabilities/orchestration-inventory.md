# Orchestration Inventory

## What This Feature Is

The orchestration inventory is the Query-owned registry for the public
orchestration surface.

Use it when you need to answer questions like:

- which admitted-handle orchestration verbs are actually shipped
- which family a verb belongs to
- whether a verb has ordinary, checked, and proof-visible lanes
- which transcript and checked topology back that verb
- which doc page and certification suite are supposed to cover it

This is not a second orchestration engine. It is the anti-drift boundary that
keeps exported verbs, transcripts, docs, and certification rows synchronized.

## Why You Use It

- inspect the live shipped orchestration surface without guessing from examples
- audit whether exported verbs are missing inventory, docs, or certification
- distinguish declaration-entry, continuation, signal, and contribution
  orchestration families cleanly
- keep ordinary-outcome lanes visible as first-class public rows
- project the older declaration-entry grammar inventory from one canonical
  cross-family source instead of maintaining a second list by hand

## Stable Entry Points

- `ForgeQueryOrchestrationSurfaceInventory::current()`
- `ForgeQueryOrchestrationSurfaceInventory::rows()`
- `ForgeQueryOrchestrationSurfaceInventory::row_for_public_name(...)`
- `ForgeQueryOrchestrationSurfaceInventory::rows_for_family(...)`
- `ForgeQueryOrchestrationInventoryAudit::current()`
- `ForgeQueryOrchestrationInventoryAudit::from_inventory(...)`
- `ForgeQueryOrchestrationSurfaceRow`
- `ForgeQueryOrchestrationSurfaceFamily`
- `ForgeQueryOrchestrationSurfaceVisibility`
- `ForgeQueryOrchestrationTranscriptFamily`
- `ForgeQueryOrchestrationCheckedTopologyKind`
- `ForgeQueryOrchestrationSupportSurface`
- `ForgeQueryOrchestrationBindingProjection`
- `ForgeQueryOrchestrationProofContract`
- `ForgeQueryOrchestrationSurfaceDocReference`
- `ForgeQueryOrchestrationSurfaceCertificationReference`

## Core Mental Model

Think of this feature as the public surface ledger for orchestration.

Each row says:

- what verb exists
- what family owns it
- what visibility lane it belongs to
- what proof and checked surface back it
- what support/readiness surface it depends on
- what docs page should teach it
- what certification suite should prove it

The current shipped families are:

- declaration-entry orchestration
- progressed route orchestration
- progressed receipt orchestration
- progressed envelope orchestration
- continuation preparation from target
- continuation preparation from context
- prepared continuation execution
- signal-compatibility orchestration
- contribution-composed orchestration

Helper verbs are also registered here. They keep their underlying family
ownership. A geometry preview helper still inventories as
signal-compatibility orchestration, a geometry material-attachment helper
still inventories as contribution-composed orchestration, and a grouped
local-neighborhood helper still inventories through the helper registry instead
of hiding as undocumented surface drift.

The current visibility lanes are:

- `Ordinary`
- `OrdinaryOutcome`
- `Checked`
- `ProofVisible`

The important rule is:

- inventory rows are not comments about the surface
- inventory rows are the surface contract for audit purposes

## How It Executes

`ForgeQueryOrchestrationSurfaceInventory::current()` builds one canonical row
set for the shipped orchestration verbs.

Those rows carry:

- public verb name
- canonical base name
- orchestration family
- visibility lane
- whether ordinary-outcome projection is supported
- shared binding projection posture
- proof contract
- doc reference
- certification reference

`ForgeQueryOrchestrationInventoryAudit::current()` then checks that inventory
against the actual admitted-handle public source surface.

Today the audit verifies at least these drift classes:

- duplicate public names
- exported verbs that have no inventory row
- inventory rows whose doc reference does not resolve
- rows with missing checked/proof type references
- rows with missing certification references
- rows with missing support-surface linkage
- rows that lie about shared binding projection
- ordinary-outcome rows that do not actually declare ordinary support
- family/visibility groups that are missing ordinary, checked, or proof lanes

This is what turns the inventory into a real boundary instead of a doc list.

## Small Example

```rust
let inventory = ForgeQueryOrchestrationSurfaceInventory::current();

let row = inventory
    .row_for_public_name("orchestrate_signal_compatibility_outcome")
    .expect("signal orchestration outcome row should exist");

assert_eq!(
    row.family(),
    ForgeQueryOrchestrationSurfaceFamily::SignalCompatibilityOrchestration
);
assert_eq!(
    row.visibility(),
    ForgeQueryOrchestrationSurfaceVisibility::OrdinaryOutcome
);
let _ = row.proof_contract().transcript_family();
let _ = row.doc_reference().path();
```

## Real Example

Use the audit when you need to treat public orchestration coverage as a real
contract in tooling or certification code.

```rust
let audit = ForgeQueryOrchestrationInventoryAudit::current();

assert!(audit.duplicate_public_names().is_empty());
assert!(audit.uninventoried_public_verbs().is_empty());
assert!(audit.undocumented_exports().is_empty());
assert!(audit.missing_transcript_rows().is_empty());
assert!(audit.missing_certification_rows().is_empty());
assert!(audit.missing_support_rows().is_empty());
assert!(audit.missing_binding_projection_rows().is_empty());
assert!(audit.ordinary_projection_mismatches().is_empty());
assert!(audit.family_visibility_gaps().is_empty());
```

You can also inspect one family slice directly:

```rust
let continuation_rows = ForgeQueryOrchestrationSurfaceInventory::current()
    .rows_for_family(ForgeQueryOrchestrationSurfaceFamily::ContinuationExecute);

for row in continuation_rows {
    let _ = row.public_name();
    let _ = row.binding_projection();
    let _ = row.proof_contract().checked_topology_kind();
}
```

## How It Relates To Other Features

- [Declaration Entry Orchestration](./declaration-entry-orchestration.md) owns
  the declaration-entry verbs that the inventory now tracks canonically.
- [Continuation Pipeline](./continuation-pipeline.md) owns the continuation
  prepare/execute surfaces that the inventory records as separate families.
- [Signal Compatibility Orchestration](./signal-compatibility-orchestration.md)
  owns the signal-facing orchestration verbs recorded here.
- [Contribution-Composed Orchestration](./contribution-composed-orchestration.md)
  owns the declaration-plus-contribution orchestration verbs recorded here.
- [Family Helpers](./family-helpers.md) own the family-native helper verbs that
  are also registered here as ordinary, checked, and proof-visible public rows.
- [Ordinary Outcomes](./ordinary-outcomes.md) owns the compact outcome surface
  that the inventory treats as a first-class visibility lane rather than as a
  comment on the checked lane.
- [Declaration Entry Readiness](./declaration-entry-readiness.md) owns part of
  the support/readiness truth that declaration-entry inventory rows point back
  to.

## Inspection And Debugging

Useful row-level accessors:

- `public_name()`
- `canonical_base_name()`
- `family()`
- `visibility()`
- `ordinary_outcome_supported()`
- `binding_projection()`
- `proof_contract()`
- `doc_reference()`
- `certification_reference()`
- `row_digest()`

Useful proof-contract accessors:

- `checked_type_name()`
- `proof_type_name()`
- `transcript_family()`
- `checked_topology_kind()`
- `support_surface()`

Useful audit accessors:

- `duplicate_public_names()`
- `uninventoried_public_verbs()`
- `undocumented_exports()`
- `missing_doc_rows()`
- `missing_transcript_rows()`
- `missing_certification_rows()`
- `missing_support_rows()`
- `missing_binding_projection_rows()`
- `ordinary_projection_mismatches()`
- `family_visibility_gaps()`

## Anti-Patterns

- treating this inventory as a generated report instead of a public contract
- merging continuation preparation and execution into one generic continuation
  row family
- treating signal orchestration or contribution-composed orchestration as if
  they were just declaration-entry aliases
- assuming docs coverage is fine because a path string exists without checking
  that the referenced page actually teaches the surface
- adding a new public orchestration method without registering it here
- using this surface as a substitute for the stronger checked or proof-visible
  runtime artifacts it points to

## Current Limits

- the inventory currently covers the shipped orchestration families plus the
  registered helper lanes that project onto them
- the audit checks the real admitted-handle source surface, but it is still a
  code-level certification boundary rather than a rendered docs inventory
- the older declaration-entry grammar inventory is now a projection of this
  boundary, not a separate authority

## Related Docs

- [Domain Capabilities](./README.md)
- [Declaration Entry Orchestration](./declaration-entry-orchestration.md)
- [Ordinary Outcomes](./ordinary-outcomes.md)
- [Continuation Pipeline](./continuation-pipeline.md)
- [Signal Compatibility Orchestration](./signal-compatibility-orchestration.md)
- [Contribution-Composed Orchestration](./contribution-composed-orchestration.md)
- [Family Helpers](./family-helpers.md)
- [Grouped Authoring](./grouped-authoring.md)
- [Declaration Entry Readiness](./declaration-entry-readiness.md)
