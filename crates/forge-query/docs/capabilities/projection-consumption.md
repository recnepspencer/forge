# Projection Consumption

## What This Feature Is

Projection consumption turns a materialized Query artifact into typed facts you
can trust without reopening source authority yourself.

It is also one of the admitted families that uses the shared intent admission
vocabulary without crossing a runtime execution seam.

Use it when you already have one of these:

- a `ForgeQueryReadResult`
- a `ForgeQueryWriteReceipt`
- a `QueryContextExecutionArtifact`

and you want stable, named facts such as:

- entity identities
- display fields
- grouped memberships
- source references
- write-target identities
- continuity aftermath

instead of re-parsing rows, indexing into payload bags, or reconstructing
meaning from lower-runtime artifacts in caller code.

## Why You Use It

- you want typed facts instead of raw payload walking
- you want the runtime to prove whether a fact family is admitted, denied,
  deferred, or only admitted with warnings
- you want a receipt and envelope for consumed facts, not just a row vector
- you want to keep Query-owned declaration and eligibility rules at the right
  boundary instead of rebuilding them in product code

## Stable Entry Points

Common path:

- `ForgeQueryReadResult::consume_projection_facts(...)`
- `ForgeQueryWriteReceipt::consume_projection_facts(...)`
- `QueryContextExecutionArtifact::consume_projection_facts(...)`
- `ProjectMaterializedFacts::declare()`
- `ProjectionFactConsumptionAttempt`
- `CompletedProjectionFactConsumption`

Support discovery:

- `ForgeQueryReadReceipt::discover_projection_fact_consumption_support(...)`
- `ForgeQueryWriteReceipt::discover_projection_fact_consumption_support()`
- `QueryContextExecutionArtifact::discover_projection_fact_consumption_support()`
- `discover_projection_consumption_support(...)`

Advanced path:

- `ForgeQueryReadReceipt::declare_projection_fact_consumption(...)`
- `ForgeQueryWriteReceipt::declare_projection_fact_consumption(...)`
- `QueryContextExecutionArtifact::declare_projection_fact_consumption(...)`
- `evaluate_projection_consumption_eligibility(...)`
- `AdmittedProjectionConsumption::bind_contract()`
- `MaterializedProjectionContract`
- `ConsumedProjectionFactSet`
- `ProjectionConsumptionReceipt`
- `forge_query_projection_consumption_intent(...)`

Expert lower-source path:

- `ProjectionConsumptionSource`
- `ProjectionConsumptionAuthoringSurface`
- `declare_projection_consumption(...)`
- `MaterializedProjectionContract::extract_from_read_result(...)`
- `MaterializedProjectionContract::extract_from_write_receipt(...)`
- `MaterializedProjectionContract::extract_from_query_context_execution(...)`
- `MaterializedProjectionContract::extract_from_relational_row_set(...)`
- `MaterializedProjectionContract::extract_from_relational_grouped_projection(...)`

Good to know:

- the common path is the right default for app code
- the advanced path is for code that needs to inspect or persist each stage
- the lower-source path is an expert seam, not the first thing product code
  should reach for
- the shared intent-admission path exists so projection consumption uses the
  same admitted vocabulary as the runtime-backed families
- retained derived artifact bindings are not yet first-class projection-
  consumption sources; when a caller needs typed scalar evidence from a named
  retained derived artifact, the current runtime-owned seam is
  `consume_scalar_fields(...)` on the retained artifact binding, and when a
  caller needs a small typed pack from that same named artifact the current
  runtime-owned seam is `decode_row_pair(...)` or `decode_row_triple(...)`;
  when a caller needs correspondence proof across two retained rows in that
  same named artifact, the current runtime-owned seam is
  `verify_scalar_alignment(...)`
- live artifact bindings are also runtime-owned pack/bind seams, not yet full
  projection-consumption source families; when a caller needs one named live
  snapshot pack across several live views, the current runtime-owned seams are
  `read_live_artifact_bundle(...)`, `bind_live_artifact(...)`, and
  `read_live_artifact_binding(...)`

## Core Mental Model

Projection consumption is not "give me rows and I'll figure it out."

It is a typed lifecycle:

1. declare which fact families you want
2. ask Query whether that request is admitted, denied, deferred, or mismatched
3. bind an admitted request into one contract
4. extract one typed fact set from one source artifact
5. issue a receipt and envelope over that consumed fact set

The important boundary is this:

- `forge-relational` and `forge-runtime-bridge` still own source truth and
  source artifacts
- `forge-query` owns declaration, eligibility, contract binding, typed fact
  extraction, receipts, and envelopes

So if a caller needs "the identity facts from this read result" or "the source
references from this write receipt," the caller should ask Query to consume
those facts, not reopen the underlying materialization and reinterpret it by
hand.

Projection consumption still terminates in a bound contract and typed fact
extraction, not in route or evaluate execution.

## How It Executes

The common path is:

1. declare requested fact families with `ProjectMaterializedFacts::declare()`
2. call `consume_projection_facts(...)` on the source artifact
3. receive one `ProjectionFactConsumptionAttempt`
4. if admitted, use `CompletedProjectionFactConsumption`
5. inspect the typed fact set, receipt, or envelope

The advanced path is the same lifecycle, but explicit:

1. declare with `declare_projection_fact_consumption(...)`
2. evaluate with `evaluate_projection_consumption_eligibility(...)`
3. bind admitted meaning with `bind_contract()`
4. extract typed facts from the contract
5. issue a `ProjectionConsumptionReceipt`
6. derive a `SelfDescribingProjectionConsumptionEnvelope`

There is also a shared admitted-family entry point when you want projection
consumption to read like the rest of the covered lattice:

1. author `forge_query_projection_consumption_intent(...)`
2. `review()?` or `admit()?`
3. `bind_contract()`
4. extract typed facts from that bound contract

Typed postures matter:

- `Admitted`
- `AdmittedWithWarnings`
- `Denied`
- `Deferred`
- `SourceMismatch`

Warnings only decorate admitted meaning. If the caller cannot honestly proceed
to contract binding and extraction, the result is not "advisory"; it is denied,
deferred, or mismatched.

## Small Example

```rust
use forge_query::facade::{
    AuthorizedProjectionArtifact, CanonicalResultShapeArtifact, ForgeQueryReadResult,
    ProjectMaterializedFacts, ProjectionFactConsumptionAttempt,
};

fn consume_identity_and_label(
    read_result: &ForgeQueryReadResult,
    result_shape: &CanonicalResultShapeArtifact,
    authorized_projection: &AuthorizedProjectionArtifact,
) -> String {
    let attempt = read_result
        .consume_projection_facts(
            result_shape,
            authorized_projection,
            ProjectMaterializedFacts::declare()
                .entity_identities()
                .display_field("profile.display_name"),
        )
        .expect("declaration or extraction should stay typed");

    match attempt {
        ProjectionFactConsumptionAttempt::Admitted(completed)
        | ProjectionFactConsumptionAttempt::AdmittedWithWarnings(completed, _) => {
            let facts = completed.facts();
            let entity = &facts.entity_identities()[0];
            let label = &facts.display_fields()[0];
            format!("{}:{}", entity.entity_identity(), label.value())
        }
        ProjectionFactConsumptionAttempt::Denied(denied) => {
            format!("denied:{:?}", denied.reason())
        }
        ProjectionFactConsumptionAttempt::Deferred(deferred) => {
            format!("deferred:{:?}", deferred.reason())
        }
        ProjectionFactConsumptionAttempt::SourceMismatch(mismatch) => {
            format!("mismatch:{:?}", mismatch.source_family())
        }
    }
}
```

This is the smallest honest example because it shows the common path, typed
fact access, and typed non-admitted handling without exposing raw rows.

The shared admitted-family path is intentionally smaller because this family
does not cross a runtime execution seam:

```rust
let contract = forge_query_projection_consumption_intent(declaration)?
    .review()?
    .admit()?
    .bind_contract();
```

## Real Example

```rust
use forge_query::facade::{
    evaluate_projection_consumption_eligibility, AuthorizedProjectionArtifact,
    CanonicalResultShapeArtifact, ForgeQueryReadReceipt, ForgeQueryReadResult,
    ProjectMaterializedFacts, ProjectionConsumptionEligibility,
};

fn certify_read_fact_consumption(
    read_receipt: &ForgeQueryReadReceipt,
    read_result: &ForgeQueryReadResult,
    result_shape: &CanonicalResultShapeArtifact,
    authorized_projection: &AuthorizedProjectionArtifact,
) -> String {
    let declaration = read_receipt
        .declare_projection_fact_consumption(
            result_shape,
            authorized_projection,
            ProjectMaterializedFacts::declare()
                .entity_identities()
                .display_field("profile.display_name"),
        )
        .expect("declaration should stay typed");

    match evaluate_projection_consumption_eligibility(&declaration) {
        ProjectionConsumptionEligibility::Admitted(admitted) => {
            let contract = admitted.bind_contract();
            let facts = contract
                .extract_from_read_result(read_result)
                .expect("extraction should stay typed");
            let receipt = facts.issue_receipt();
            let envelope = receipt.projection_consumption_envelope();

            format!(
                "{}:{}:{}",
                contract.contract_digest(),
                receipt.receipt_digest(),
                envelope.envelope_digest()
            )
        }
        ProjectionConsumptionEligibility::AdmittedWithWarnings(admitted, warnings) => {
            let contract = admitted.bind_contract();
            let facts = contract
                .extract_from_read_result(read_result)
                .expect("extraction should stay typed");

            format!("{}:{}", facts.fact_set_digest(), warnings.warning_kinds().len())
        }
        ProjectionConsumptionEligibility::Denied(denied) => {
            format!("denied:{:?}", denied.reason())
        }
        ProjectionConsumptionEligibility::Deferred(deferred) => {
            format!("deferred:{:?}", deferred.reason())
        }
        ProjectionConsumptionEligibility::SourceMismatch(mismatch) => {
            format!("mismatch:{:?}", mismatch.source_family())
        }
    }
}
```

Use the advanced path when:

- you need the declaration digest
- you need to keep eligibility and extraction separate
- you need the bound contract as its own artifact
- you are writing framework code, certification code, or integration code at a
  sharper boundary than normal app code

## How It Relates To Other Features

- Pair this with [Read Composition](../authoring/read-composition.md) when you need typed
  consumed facts from a `ForgeQueryReadResult` rather than only its payload.
- Pair it with [Intent Admission](../execution/intent-admission.md) when you
  want the shared admitted-family story for projection consumption itself.
- Pair it with [Reads, Observation, and Materialization](../runtime-surfaces/reads-observe-materialize.md)
  when you need to decide whether rows are enough or whether a caller really
  needs typed fact consumption.
- Pair it with [Writes And Intents](../execution/writes-and-intents.md) when a write
  receipt needs typed target, provenance, or continuity aftermath facts.
- Pair it with [Inspection](inspection.md) when you need explanation surfaces
  around the same read/write/query-context flow. Projection consumption uses
  receipt-first inspection rather than `workspace.inspect(...)`.
- Pair it with [Support Matrix And Admission](../foundations/support-matrix-and-admission.md)
  when you need to explain why a family is deferred or unsupported at the
  runtime-facade level. Projection consumption also has its own source-local
  support discovery helpers.

## Inspection And Debugging

The first debugging surface here is the receipt, not `workspace.inspect(...)`.

Important things to inspect on `CompletedProjectionFactConsumption` or
`ProjectionConsumptionReceipt`:

- `source_family()`
- `source_identity()`
- `support_posture()`
- `warning_kinds()`
- `admitted_fact_family_count()`
- `extracted_fact_count()`
- `authority_reopen_count()`
- `deferred_neighbors()`
- `transition_rules()`
- `projection_consumption_envelope()`

Use source-local support discovery before consumption when you are unsure what a
source can prove:

- read receipts require a `CanonicalResultShapeArtifact`
- write receipts expose support from carried mutation evidence
- query-context executions expose support from their execution posture and
  carried metadata

Framework or certification code can also inspect:

- `projection_consumption_support_matrix()`
- `projection_consumption_public_boundary_audit()`
- `projection_consumption_proof_shape_audit()`
- `certify_projection_consumption_closeout_core()`

Those are framework-level artifacts, not the normal app entry point.

## Anti-Patterns

- Treating projection consumption as a prettier row iterator.
- Reopening raw rows, payload maps, or lower-runtime artifacts in caller code
  when Query already exposes the fact family you need.
- Assuming a warning-bearing admission is the same as a denied or deferred
  posture.
- Using projection consumption when you only need plain live rows or computed
  rows and no typed fact contract.
- Treating `workspace.inspect(...)` as the projection-consumption inspection
  path. This feature uses receipts and envelopes directly.
- Reaching for the lower-source expert path in ordinary app code when the
  common path already fits.

## Current Limits

- The common path is currently centered on `ForgeQueryReadResult`,
  `ForgeQueryWriteReceipt`, and `QueryContextExecutionArtifact`.
- The shared admitted-family path is real, but this family still ends in a
  bound contract and typed extraction rather than runtime execution.
- Query-context field facts may be admitted with warnings because they can be
  payload-bound instead of receipt-perfect.
- Write-receipt fact families are limited to evidence actually carried by the
  receipt. Missing provenance or continuity evidence stays typed as deferred or
  mismatched instead of being guessed.
- Lower-source relational and bridge extraction seams exist, but they are still
  expert boundaries rather than the primary app-facing path.
- Retained derived artifact bindings currently stop at the runtime-owned
  retained-scalar evidence seam; they do not yet participate as full
  projection-consumption source families with authorized-projection binding
  semantics.
- This feature does not replace `workspace.read(...)`, `workspace.observe(...)`,
  or `workspace.materialize(...)`. Use those when rows are the real product
  surface.

## Related Docs

- [Workspace Overview](../foundations/workspace-overview.md)
- [Read Composition](../authoring/read-composition.md)
- [Reads, Observation, and Materialization](../runtime-surfaces/reads-observe-materialize.md)
- [Writes And Intents](../execution/writes-and-intents.md)
- [Inspection](inspection.md)
- [Support Matrix And Admission](../foundations/support-matrix-and-admission.md)


