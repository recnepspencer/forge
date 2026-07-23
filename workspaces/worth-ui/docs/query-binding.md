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
- `WorthUiInstalledSnapshotQueryView::read()` and
  `WorthUiInstalledSnapshotQueryView::project(...)`
- `WorthUiInstalledLiveQueryView::open_using(...)`
- `WorthUiQueryLiveResource::read(...)`, `project(...)`, and `close(...)`
- `WorthUiApp::launch()`
- `WorthUiActiveApplicationSession::execute_framework_turn(...)`

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

Registration is shared, but execution deliberately splits by lifecycle:

```text
Query runtime installs the Worth UI domain package
-> application resolves the installed Worth UI domain
-> application derives and registers an installed view
-> the prepared application launches one active application session

snapshot view
-> read -> run -> snapshot projection
-> framework turn admit_and_submit

live view
-> open_using -> Query-owned live resource
-> resource read -> live projection
-> framework turn admit_live_and_submit(resource, projection)
-> active binding retains the resource until retirement, shutdown, or explicit close
```

`WorthUiInstalledQueryView` is the common registration envelope. It intentionally
does not expose snapshot reads or live opens. The lifecycle-specific types keep
those operations unambiguous at compile time. Query remains responsible for
live-resource activation, maintenance, recovery, and disposal; Worth UI only
admits the resource and projection atomically and coordinates retirement with
the active application lifecycle.

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
        .expect("the application must prepare as one authority")
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
    app::WorthUiActiveApplicationSession,
    query_binding::{
        WorthUiInstalledSnapshotQueryView, WorthUiQuerySnapshotProjectionOutcome,
    },
};

fn measurement_projection(
    workspace: &mut WorthQueryWorkspace,
    view: &WorthUiInstalledSnapshotQueryView,
) -> WorthUiQuerySnapshotProjectionOutcome {
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

fn submit_projection(
    session: &mut WorthUiActiveApplicationSession,
    outcome: WorthUiQuerySnapshotProjectionOutcome,
) {
    let mut submission = None;
    let _completion = session.execute_framework_turn(|turn| {
        turn.query_projection(|query| {
            submission = Some(query.admit_and_submit(outcome));
        });
    });
    submission
        .expect("the Query source lane ran")
        .expect("the projection matched the registered installed view");
}
```

The Query completion remains authoritative. `WorthUiQuerySnapshotProjectionOutcome`
preserves it together with the installed UI definition. The framework turn
settles and submits it; application code never reconstructs a basis digest or
converts the native value through a representation format.

For a live view, call `open_using(...)`, read and project through the returned
`WorthUiQueryLiveResource`, then pass both the resource and
`WorthUiQueryLiveProjectionOutcome` to `admit_live_and_submit(...)` in the same
framework turn. Do not submit the projection separately: the resource is the
Query-owned lifecycle authority that the active binding must retain. Successful
replacement returns exact retirement authority for removed or superseded live
resources; close it against the owning Query workspace and preserve typed stop
outcomes rather than treating disposal as a boolean.

The active application session is intentional. It keeps Query submission,
application generation, graph/allocation authority, host session, inspection,
and replacement cutover coherent. Raw runtime launch and submission seams are
certification or owner-implementation concerns, not the ordinary application
workflow.

## How It Relates To Other Features

- Use Query-free UI registration for surfaces that have no domain-backed state.
- Pair installed views with graph touches when projected facts can invalidate a
  mounted neighborhood.
- Snapshot views provide one completed projection. Live views add Query-owned
  activation, retention, retirement, and disposal while preserving the same UI
  registration grammar.
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
- Do not split either lifecycle-specific projection outcome into locally trusted
  basis, receipt, fact, support, source-label, or digest fields.
- Do not route snapshot execution through a live resource or live execution
  through the snapshot read path.
- Do not drop a nonempty live retirement merely because plan publication
  succeeded; consume its exact Query close receipts.
- Do not implement live-view activation, subscription, recovery, or disposal in
  the UI runtime.
- Do not import Query into `worth-ui-runtime`; translation belongs only in
  `worth-ui-query-binding`.

## Current Limits

- The current public domain package exposes Worth UI measurement snapshot and
  live views.
- Virtualized execution consumes already-admitted visible ranges. Collection
  cursor policy, pagination, and general live-patch product semantics remain
  outside the current surface.
- Measurement refinement currently admits canonical `Float32` values for the
  declared measurement field and preserves Query denial for absent or
  unsupported native shapes.
- Richer UI projection families should be added as installed-domain capabilities,
  not as detached UI-side constructors.

## Related Docs

- [Worth UI runtime orientation](./worth-ui-readme.md)
- [Worth Query orientation](../../../crates/worth-query/docs/AI_README.md)
