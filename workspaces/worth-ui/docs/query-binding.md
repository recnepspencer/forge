# Query-Backed UI Views

## What This Feature Is

Query-backed UI views let a Worth UI application render and measure data from a
Worth Query workspace without rebuilding Query state inside the UI runtime. You
install Worth UI's domain package in Query, derive an installed view from that
workspace, and register the view with the UI application.

## Why You Use It

- Bind a control or surface to measurements produced by a Query workspace.
- Keep snapshot and live-view lifecycle attached to the Query runtime that owns it.
- Carry native Foundational values into allocation without JSON, text parsing,
  or locally invented identity.

## Stable Entry Points

- `worth_ui::facade::query_binding::worth_ui_domain_package()`
- `WorthUiQueryWorkspaceExt::worth_ui()`
- `WorthUiInstalledQueryDomain::measurement_view(...)`
- `WorthUiInstalledQueryDomain::live_measurement_view(...)`
- `WorthUi::app().register_query_view(...)`
- `WorthUiInstalledQueryView::read()` and `WorthUiInstalledQueryView::project(...)`
- `WorthUiRuntime::execute_framework_turn(...)`

The UI facade exposes the binding and runtime handoff. Query still owns workspace
construction, read execution, projection declarations, basis selection, and
installed-domain lifecycle.

## Core Mental Model

The Query workspace owns the data and the right to read it. Calling
`workspace.worth_ui()` resolves a runtime-affine installed-domain handle: a
handle that is valid only for the Query runtime and installation generation that
minted it.

Calling `measurement_view(...)` derives one installed UI view from that handle.
The view keeps two things together:

- the stable UI declaration used for registration and invalidation
- the exact installed Query authority used for execution and projection

Worth UI registers that object as a whole. The runtime may derive UI indexes and
diagnostic summaries, but those derived artifacts cannot replace the installed
authority.

## How It Executes

```text
Query runtime installs the Worth UI domain package
-> application resolves the installed Worth UI domain
-> application derives and registers an installed view
-> the installed view executes a Query read
-> the same view wraps the Query projection outcome
-> a Worth UI framework turn admits the outcome
-> worth-ui-query-binding refines native measurement facts
-> allocation consumes the opaque settlement
```

Snapshot and live views follow the same path. Their lifecycle is part of the
installed view definition, not a UI-side flag.

## Small Example

```rust
use worth_query::facade::runtime::WorthQueryWorkspace;
use worth_ui::facade::{
    app::{WorthUi, WorthUiApp},
    query_binding::WorthUiQueryWorkspaceExt,
};

fn build_ui(workspace: &WorthQueryWorkspace) -> WorthUiApp {
    let installed = workspace
        .worth_ui()
        .expect("the Query runtime must install the Worth UI domain package");
    let measurements = installed
        .measurement_view("inspector.measurements")
        .expect("the installed domain must support measurement reads");

    WorthUi::app()
        .register_query_view(measurements)
        .expect("the installed view must be registered once")
        .freeze()
}
```

This is the smallest honest example because the builder receives an installed
view, not a detached definition or a collection of UI-authored Query metadata.

## Real Example

The application executes through the installed view, then submits the resulting
projection during the framework-owned allocation turn:

```rust
use worth_foundational::facade::{CanonicalFieldPath, FieldKey};
use worth_query::facade::{
    domain,
    foundation::ProjectionFactFieldPath,
    runtime::WorthQueryWorkspace,
};
use worth_ui::facade::{
    query_binding::{WorthUiInstalledQueryView, WorthUiQueryProjectionOutcome},
    runtime::WorthUiRuntime,
};

fn measurement_projection(
    workspace: &mut WorthQueryWorkspace,
    view: &WorthUiInstalledQueryView,
) -> WorthUiQueryProjectionOutcome {
    let completion = view
        .read()
        .expect("declare the installed read")
        .using(domain::current())
        .run(workspace)
        .expect("the view and workspace must share installed authority")
        .into_result()
        .expect("the read must complete");

    let field = ProjectionFactFieldPath::from_canonical_field_path(
        CanonicalFieldPath::new([
            FieldKey::new("measurement").unwrap(),
            FieldKey::new("value").unwrap(),
        ])
        .unwrap(),
    );

    view.project(
        &completion,
        domain::project_facts().display_field(field),
    )
    .expect("the completion must come from this installed view")
}

fn submit_projection(runtime: &mut WorthUiRuntime, outcome: WorthUiQueryProjectionOutcome) {
    let mut submission = None;
    let _completion = runtime.execute_framework_turn(|turn| {
        turn.query_projection(|query| {
            submission = Some(query.admit_and_submit(outcome));
        });
    });
    submission
        .expect("the Query source lane ran")
        .expect("the projection matched the registered installed view");
}
```

The Query completion remains authoritative. `WorthUiQueryProjectionOutcome`
preserves it together with the installed UI definition. The framework turn
settles and submits it; application code never reconstructs a basis digest or
converts the native value through a representation format.

## How It Relates To Other Features

- Use Query-free UI registration for surfaces that have no domain-backed state.
- Pair installed views with graph touches when projected facts can invalidate a
  mounted neighborhood.
- Snapshot views provide one completed projection. Live views add Query-owned
  activation and disposal while preserving the same UI registration grammar.
- Allocation consumes binding-owned settlements. Host measurement remains a
  separate runtime source and cannot impersonate Query authority.

## Inspection And Debugging

Inspect the view definition before launch through `view.definition()`. It
exposes the stable identity, lifecycle, expected shape, and canonical digest.

At runtime, typed denials distinguish missing installation, foreign or stale
installed authority, unregistered views, Query-free runtimes, and projection
settlement failures. Refinement receipts retain exact counters for declared
measurement facts, projected facts, refinement attempts, and admitted
observations. The declared counter uses exact requested facts, not deduplicated
fact-family labels.

## Anti-Patterns

- Do not create a `ViewBindingDescriptor` for Query-backed UI directly.
- Do not register a bare `WorthUiQueryViewDefinition`; register the installed view.
- Do not copy Query status, basis, result shape, or live posture into UI enums.
- Do not hash reporting text or digest strings to recreate operational identity.
- Do not stringify native projection values and parse them again for allocation.
- Do not import Query into `worth-ui-runtime`; translation belongs only in
  `worth-ui-query-binding`.

## Current Limits

- The current public domain package exposes Worth UI measurement snapshot and
  live views.
- Measurement refinement currently admits canonical `Float32` values for the
  declared measurement field and preserves Query denial for absent or
  unsupported native shapes.
- Richer UI projection families should be added as installed-domain capabilities,
  not as detached UI-side constructors.

## Related Docs

- [Worth UI runtime orientation](./worth-ui-readme.md)
- [Worth Query orientation](../../../crates/worth-query/docs/AI_README.md)
