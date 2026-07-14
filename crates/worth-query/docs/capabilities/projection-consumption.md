# Projection Consumption And Downstream Authority

## What This Feature Is

Projection consumption lets application code move selected facts from a
completed Query read into another subsystem without treating raw IDs, rows, or
digests as authority. The completed read already knows its source, basis, and
authorized result shape. You declare which facts you need, and Query returns a
sealed authority object or a typed non-admitted outcome.

## Why You Use It

- Carry entity identities, memberships, relation endpoints, or display fields
  into UI, workflow, and other downstream runtimes.
- Reject stale, mismatched, partial, or unavailable facts before they become
  downstream authority.
- Keep fact extraction and source-lineage checks inside Query.
- Avoid parallel consumer-side structs that can accidentally pair facts from
  one read with authority from another.

## Stable Entry Points

Import the ordinary surface from `worth_query::facade::read`:

- `project_facts()` begins a fact declaration.
- `WorthQueryReadCompletion::consume_projection(...)` consumes it against the
  completed read.
- `WorthQueryProjectionOutcome` preserves completed, advisory, violation,
  deferred, and unavailable postures.
- `WorthQueryProjectionOutcome::into_admitted()` moves admitted authority to a
  downstream owner.
- `WorthQueryConsumedProjectionAuthority` is the sealed admitted product.

Available fact declarations include `entity_identities()`,
`view_local_identities()`, `target_identity()`, `source_references()`,
`effect_continuity_facts()`, `memberships()`, `relation_endpoints()`,
`display_field(...)`, and `derived_scalar_field(...)`.

## Core Mental Model

A **basis** is the admitted world of truth used by the read. Projection
consumption does not rediscover that world from an identifier. It uses the
authority already sealed into `WorthQueryReadCompletion`.

The projection declaration names facts, not safety switches. Query always
requires settled consumption, source authority, and matching basis generation.
The caller cannot opt out of those checks. The resulting authority keeps the
source lineage, facts, receipt, and requirements together as one product.

## How It Executes

1. Build and run an ordinary read with `read::declare(...).using(...).run(...)`.
2. Keep the `WorthQueryReadCompletion`, not just its row payload.
3. Declare the facts with `read::project_facts()`.
4. Call `completion.consume_projection(...)`.
5. Move completed or advisory authority with `into_admitted()`, or handle the
   typed non-admitted outcome.

Extraction and authority binding happen inside Query. No consumer-visible
canonicalization, planning, extraction, or success-envelope step exists.

## Small Example

```rust
use worth_query::facade::read::{
    project_facts, WorthQueryProjectionOutcome, WorthQueryReadCompletion,
};

fn identity_authority(
    completion: &WorthQueryReadCompletion,
) -> WorthQueryProjectionOutcome {
    completion.consume_projection(project_facts().entity_identities())
}
```

This is the smallest honest example because the completed read supplies source
and basis authority while the consumer supplies only its requested facts.

## Real Example

```rust
use worth_query::facade::read::{
    project_facts, ProjectionFactFieldPath,
    WorthQueryConsumedProjectionAuthority, WorthQueryProjectionOutcome,
    WorthQueryReadCompletion,
};

fn ui_measurement_authority(
    completion: &WorthQueryReadCompletion,
    display_field: ProjectionFactFieldPath,
) -> Result<Box<WorthQueryConsumedProjectionAuthority>, WorthQueryProjectionOutcome> {
    completion
        .consume_projection(
            project_facts()
                .entity_identities()
                .display_field(display_field),
        )
        .into_admitted()
        .map(|(authority, _warnings)| authority)
}
```

The UI receives a sealed authority that proves both identity and display-field
facts came from the same completed read. Advisory warnings remain attached to
the admission result. A violation, deferral, or unavailable source stays typed
and cannot be converted into authority by extracting its evidence.

## How It Relates To Other Features

- Use [Read Composition](../authoring/read-composition.md) to define the read and
  result shape that make facts available.
- Use [Declarative Query Experience](declarative-query-experience.md) for the
  complete declaration/context/execution flow.
- Use [Basis Capability Lifecycle](basis-capability-lifecycle.md) when the
  operation needs an explicit historical, preview, policy, or tenant world.
- Use [Inspection](inspection.md) when explanation is the goal rather than
  carrying operational authority.

## Inspection And Debugging

Before moving authority, inspect the outcome category:

- `authority()` is present for completed and advisory outcomes.
- `advisory()` exposes non-fatal warnings.
- `violation()` preserves authority, consumption, source-mismatch, and
  declaration violations.
- `deferred()` means the declared work cannot proceed yet.
- `unavailable()` preserves authority-binding or fact-extraction failure.

After admission, inspect the authority's facts, receipt, source identity, basis,
and counters. These accessors explain an authority; they cannot recreate one.

## Anti-Patterns

- Passing a basis digest, receipt digest, source label, and fact list as an
  authority tuple.
- Calling `into_result()` before projection facts have been consumed.
- Rebuilding projection facts from rows in consumer code.
- Comparing local digests to decide whether Query authority is valid.
- Importing internal projection-consumption or lower-runtime modules.
- Falling back to raw IDs after a typed violation or unavailable outcome.
- Creating a consumer-local extraction, planning, or success-envelope layer.

## Current Limits

- The ordinary projection journey starts from a completed ordinary read.
- Fact availability depends on the read's result shape and retained source
  evidence; Query returns a typed non-admitted outcome when either is missing.
- Projection authority is operational proof, not a persistence DTO.
- Store-backed durable restore remains governed by the support matrix and must
  enter through the same ordinary declaration and outcome contracts.
- Projection consumption does not replace reading rows or rich inspection. Use
  it only when another operation depends on the authority of selected facts.

## Related Docs

- [Declarative Query Experience](declarative-query-experience.md)
- [Downstream Runtime Integration](../foundations/downstream-runtime-integration.md)
- [Consumer Kit](../foundations/consumer-kit.md)
- [Basis Capability Lifecycle](basis-capability-lifecycle.md)
- [Support Matrix And Admission](../foundations/support-matrix-and-admission.md)
