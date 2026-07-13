# Projection Consumption And Downstream Authority

## What This Feature Is

Projection consumption lets application code carry facts produced by Query
without rebuilding their authority from IDs, digests, or rows. When another
runtime needs those facts, Query returns one sealed authority object that keeps
the basis, source lineage, consumed facts, receipt, and consumer requirements
together.

## Why You Use It

- Carry Query-derived identities, labels, memberships, or source references
  into another runtime.
- Reject stale, mismatched, partial, or unsupported consumption before it can
  become downstream authority.
- Inspect typed facts without parsing raw result payloads.
- Keep compatibility and lineage checks in Query instead of duplicating them
  in each consumer.

## Stable Entry Points

### Ordinary fluent path

The ordinary path is result-attached:

- `WorthQueryReadResult::consume_projection_authority(...)`
- `WorthQueryWriteReceipt::consume_projection_authority(...)`
- `QueryContextExecutionArtifact::consume_projection_authority(...)`
- `WorthQueryDerivedArtifactBinding::consume_projection_authority(...)`
- `WorthQueryLiveArtifactBinding::consume_projection_authority(...)`
- `ProjectionAuthorityContract::declare()`
- `ProjectionAuthorityOutcome`
- `WorthQueryConsumedProjectionAuthority`

### Contract reference

Build a `ProjectionAuthorityContract` with only the guarantees the consumer
actually requires: settled consumption, source authority, target identity, and
source references. Query checks those requirements during the one canonical
transition. For a durable handoff, serialize the declaration with
`to_terminal_json_document()` and reload it with
`load_projection_authority_contract_document(...)`; replay still enters the
same transition and unknown schemas fail closed.

### Denial and inspection

`ProjectionAuthorityOutcome` distinguishes admitted authority, admitted
authority with warnings, authority denial, consumption denial, deferral, and
source mismatch. Inspect the typed outcome; never recover by assembling raw
parts.

### Advanced lifecycle

Use `consume_projection_facts(...)` only for immediate typed inspection. Its
completion type is deliberately absent from the curated facade: decomposed
facts, receipts, and digests are not a second downstream-authority API.

The explicit lifecycle remains available for framework code that needs to
observe declaration, eligibility, contract binding, extraction, and receipt
issuance separately. Start with
`declare_projection_fact_consumption(...)` and
`evaluate_projection_consumption_eligibility(...)`.

Support discovery is available through source-local
`discover_projection_fact_consumption_support(...)` methods and
`consumed_projection_authority_support_matrix()`.

### Migration history

Older consumers retained completed consumption parts or compared basis and
receipt digests locally. Those patterns are no longer curated facade APIs.
Retain the sealed authority object and use its getters only for observation or
indexing.

## Core Mental Model

A **basis** is the admitted world of truth used by the Query operation. A
projection authority is Query's sealed proof that requested facts were
consumed from one source in that world and satisfy one declared downstream
contract.

Source runtimes still own their underlying truth. Query does not promote a
rendered source ID or receipt digest into that truth. It binds the source
runtime's evidence to the Query basis and consumption receipt, then exposes one
non-cloneable authority product. Getters are for indexing and inspection; they
cannot recreate the product.

## How It Executes

1. Declare the facts and guarantees the consumer requires.
2. Call `consume_projection_authority(...)` on the source result or binding.
3. Query performs source extraction and the canonical authority transition.
4. Receive one `ProjectionAuthorityOutcome`.
5. Continue only with its admitted authority; otherwise handle its typed
   warning, denial, deferral, or source mismatch.

The fluent and explicit paths use the same transition. Adapters may extract
from different source artifacts, but they do not own separate authority logic.

## Small Example

```rust
use worth_query::facade::{
    AuthorizedProjectionArtifact, ProjectionAuthorityContract,
    ProjectionAuthorityOutcome, ProjectionFactConsumptionPathError,
    WorthQueryWriteReceipt,
};

fn consume_write_authority(
    receipt: &WorthQueryWriteReceipt,
    projection: &AuthorizedProjectionArtifact,
) -> Result<ProjectionAuthorityOutcome, ProjectionFactConsumptionPathError> {
    receipt.consume_projection_authority(
        "result-shape:profile",
        projection,
        ProjectionAuthorityContract::declare()
            .require_settled_consumption()
            .require_source_authority()
            .require_target_identity()
            .require_source_references(),
    )
}
```

This is the smallest honest path because the consumer states its requirements
and receives the indivisible authority product directly.

## Real Example

```rust
use worth_query::facade::{
    AuthorizedProjectionArtifact, ProjectionAuthorityContract,
    ProjectionAuthorityOutcome, WorthQueryConsumedProjectionAuthority,
    WorthQueryWriteReceipt,
};

fn admitted_authority(
    receipt: &WorthQueryWriteReceipt,
    projection: &AuthorizedProjectionArtifact,
) -> Result<WorthQueryConsumedProjectionAuthority, String> {
    let outcome = receipt
        .consume_projection_authority(
            "result-shape:profile",
            projection,
            ProjectionAuthorityContract::declare()
                .require_settled_consumption()
                .require_source_authority()
                .require_target_identity()
                .require_source_references(),
        )
        .map_err(|error| error.to_string())?;

    match outcome {
        ProjectionAuthorityOutcome::Admitted(authority)
        | ProjectionAuthorityOutcome::AdmittedWithWarnings(authority, _) => Ok(authority),
        ProjectionAuthorityOutcome::AuthorityDenied(denial) => {
            Err(format!("authority denied: {:?}", denial.kind()))
        }
        ProjectionAuthorityOutcome::ConsumptionDenied(denial) => {
            Err(format!("consumption denied: {:?}", denial.reason()))
        }
        ProjectionAuthorityOutcome::Deferred(deferred) => {
            Err(format!("deferred: {:?}", deferred.reason()))
        }
        ProjectionAuthorityOutcome::SourceMismatch(mismatch) => {
            Err(format!("source mismatch: {:?}", mismatch.source_family()))
        }
    }
}
```

The returned object retains the authoritative relationship. A downstream
runtime may inspect its basis, receipt, facts, source identity, requirements,
and counters, but should store or pass the authority object itself whenever a
later operation depends on that relationship.

## How It Relates To Other Features

- Use [Read Composition](../authoring/read-composition.md) to produce the read
  result whose facts are consumed.
- Use [Basis Capability Lifecycle](basis-capability-lifecycle.md) when you need
  to author or inspect the operation's truth-world capability itself.
- Use [Inspection](inspection.md) when explanation is the goal rather than
  carrying operational authority.
- Use [Async Resources And Result State](async-resources-and-result-state.md)
  for the surrounding retained or async state model.
- Use [Support Matrix And Admission](../foundations/support-matrix-and-admission.md)
  to distinguish admitted, deferred, and unsupported source families.

## Inspection And Debugging

On an admitted authority, inspect:

- `basis()` and `source_identity()` for lineage
- `contract()` for consumer requirements
- `facts()` and `receipt()` for what was consumed
- `evidence()` for a derived diagnostic projection
- `counters()` for bounded-work evidence

Before consumption, inspect
`consumed_projection_authority_support_matrix()` when source support is
unclear. A denial, deferral, or mismatch is a terminal typed outcome; do not
fall back to raw facts.

## Anti-Patterns

- Passing a basis digest, receipt digest, source label, and fact list as an
  authority tuple.
- Comparing consumer-local digests to decide whether Query authority is valid.
- Reconstructing a projection contract or source identity in downstream code.
- Importing Query's internal `projection_consumption` module.
- Treating evidence getters as constructors or promotion inputs.
- Falling back to raw rows after a typed denial or unsupported posture.

## Current Limits

- Read results, write receipts, Query-context executions, derived bindings,
  and live bindings are the supported ordinary source families; exact posture
  depends on the requested facts and source evidence.
- Store-backed and durable neighbors remain deferred or unsupported where the
  support matrix says so. Query does not guess missing lineage.
- Immediate typed-fact inspection remains supported through opaque
  `consume_projection_facts(...)` results, but those decomposed parts cannot be
  named through the curated facade as downstream authority.
- Projection authority does not replace `read`, `observe`, or `materialize`.
  Use those when rows are the actual product.

## Related Docs

- [Downstream Runtime Integration](../foundations/downstream-runtime-integration.md)
- [Consumer Kit](../foundations/consumer-kit.md)
- [Basis Capability Lifecycle](basis-capability-lifecycle.md)
- [Projection Consumption Vs Inspection](../domain-capabilities/choosing/projection-consumption-vs-inspection.md)
- [Support Matrix And Admission](../foundations/support-matrix-and-admission.md)
