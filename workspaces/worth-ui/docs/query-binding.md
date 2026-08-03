# Query-Backed UI Views

## What This Feature Is

Query-backed views let Worth UI mount native values produced by an installed
Worth Query runtime. Query owns execution, result posture, compatibility,
recovery, and resource lifecycle. Worth UI owns the declared projection
requirement, UI consequence, mounted identity, and presentation.

## Why You Use It

- Present scalar or keyed collection data without copying Query truth into UI.
- Preserve pending, current, stale, revalidating, and exact stop posture.
- Carry native values into mounted semantic text without JSON or string
  reconstruction.
- Rebind only declared consumers when Query meaning changes.
- Correlate Query evidence, a projection fact, mounted identity, and pixels.

## Stable Entry Points

- `worth_ui::facade::query_binding::WorthUiScalarProjectionHostPlan`
- `UiScalarProjectionRegistration`
- `UiCollectionProjectionRegistration`
- `UiScalarProjectionObservation`
- `UiCollectionProjectionObservation`
- `UiScalarProjectionFactReceipt`
- `UiCollectionProjectionFactReceipt`
- `UiProjectionAvailability`
- `UiPresentProjection`
- `WorthUiApplicationBuilder::register_scalar_projection(...)`
- `WorthUiApplicationBuilder::register_collection_projection(...)`
- `WorthUiActiveApplicationSession::begin_projection_rebind(...)`

There is no product `WorthUiQueryWorkspaceExt` import. Query-free applications
do not install a dummy Query runtime or register a dummy projection.

## Core Mental Model

A projection has orthogonal contracts:

- shape: scalar or collection;
- schema: selected fields, native family, and row identity where applicable;
- lifecycle: snapshot or live;
- availability: unavailable, present, or stopped;
- currency/activity: current or retained stale, optionally revalidating;
- compatibility: exact admitted replacement or a typed stop;
- budget: bounded accesses, rows, bytes, and retained resources.

These are types, not fields an application may reassemble. A reporting identity
cannot become a binding, an inspection projection cannot become a fact, and a
collection fact cannot enter a scalar consumer.

## Register A Projection

An installation owner first obtains an `UiInstalledProjectionView` from Query
authority. Product application code then declares the required shape and
registers it before `freeze()`.

The following fragment is compiled inside the public facade contract.

<!-- compile-pass-fragment:register_scalar_projection -->
```rust
fn register_scalar(view: UiInstalledProjectionView) {
    let registration = UiScalarProjectionRegistration::text(
        view,
        UiProjectionFieldRequirement::declared("status").expect("valid selected field"),
    );
    let _app = worth_ui::facade::app::WorthUi::app()
        .with_change_profile(
            worth_ui::facade::rebind::UiChangeProfile::platform_pulse(),
        )
        .register_scalar_projection(registration)
        .expect("installed scalar projection registration")
        .freeze()
        .expect("application preparation should succeed");
}
```

For a collection, use `UiCollectionProjectionRegistration::text(...)` or
`native(...)` with an explicit row-identity field, selected fields,
completeness requirement, and continuation posture. Registration never
executes Query or creates UI-local result state.

## Install The Production Query Runtime

Platform Pulse demonstrates the concrete hosted scalar route:

```text
WorthUiScalarProjectionHostPlan::prepare()
-> split request and completion
-> WorthQueryExecutionRuntimeInstaller::install(...)
-> completion.complete(installation)
-> split installed registration and initial projection advance
-> register the scalar projection on WorthUi::app()
```

The completion verifies the Query Consumer Kit support contract for the actual
backend before opening the live projection. The host installer remains the
Query authority boundary; Worth UI does not emulate it.

## Observe And Rebind

The live owner issues one affine observation. Submit it through
`begin_projection_rebind(...)`, exhaust the typed rebind outcome, and return the
released shape-specific fact to the Query lifecycle owner. Publication
completion is what allows that owner to advance again.

```text
Query-issued observation
-> ordinary 3.12 observation admission and classification
-> indexed affected-scope plan
-> mounted semantic text
-> host presentation and atomic publication
-> released scalar or collection fact
-> Query publication admission
```

`UiProjectionAvailability` must be matched exhaustively. `Present(Current)`
carries current native value. `Present(RetainedStale { ... })` carries the
predecessor plus activity. `Unavailable` and `Stopped` carry exact typed
posture and mint no fabricated value.

## Replacement And Invalidation

Compatible replacement preserves logical binding identity only through
Query-issued compatibility proof. Schema, native-family, payload-shape, row
identity, world, generation, basis, and budget mismatches stop before a
successor. The predecessor remains available only when the typed outcome says
it was retained.

Changed scalar and collection facts enter the same source/viewport observation
ordering used by the rest of the application. Runtime follows declared
consumer indexes; it does not poll Query, scan the mounted graph, or introduce a
second executor.

## Cost And Lifecycle

- Query-free and unchanged turns perform zero projection/content work.
- Scalar access is bounded by declared selected fields.
- Collection work scales with selected or changed rows, not total collection
  size; completeness and continuation remain explicit.
- Diagnostic detail and closure stress belong to separate cost lanes.
- Cancellation, denial, replacement, retry, continuation, reset, close, and
  shutdown dispose their governed resources exactly once.

## Inspection And Debugging

Correlate the Query transition or attempt identity, projection fact identity,
application generation, mounted frame/node identity, and presentation evidence.
Request compact evidence first and expand detail under an explicit disclosure,
retention, and byte budget. None of those reporting values can execute Query,
construct a fact, or publish.

## Anti-Patterns

- Importing a nonexistent workspace-extension convenience API.
- Copying Query result state into a UI cache or local loading/error enum.
- Selecting operational values by a literal field lookup after admission.
- Converting native values through JSON, debug text, or widened numbers.
- Reassembling authority from identities, digests, reports, or inspection.
- Querying from a renderer or host adapter.
- Treating collection continuation as scalar posture.
- Retrying or replacing without consuming the returned affine owner.

## Current Limits

The direct grammar supports declared scalar and keyed collection projections.
Rich tables, joins, authored expressions and formatting, mutation intents, and
general composition remain additive successor work. Milestone 3.14 may consume
these projection facts for admitted intents; it must not replace this binding,
observation, or publication path.

## Related Docs

- [Application lifecycle](./application-lifecycle.md)
- [Worth UI architecture](./architecture.md)
- [Runtime subsystems](./runtime-subsystems.md)
- [Application inspection](./inspection.md)
