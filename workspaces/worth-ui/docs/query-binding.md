# Query-Backed UI Views

## What This Feature Is

Query-backed views let a Worth UI application present data owned by a Worth
Query workspace without copying Query state into the UI runtime. You install
the Worth UI domain package in Query, create an installed view, and register
that view on the application builder.

## Why You Use It

- Present collection or detail data from an installed Query workspace.
- Keep snapshot and live-resource lifecycle with the Query runtime that owns it.
- Carry native Foundational values into UI planning without JSON or text.
- Preserve exact Query denials and settlement identity for inspection.

## Stable Entry Points

- `worth_ui::facade::query_binding::worth_ui_domain_package()`
- `WorthUiQueryWorkspaceExt::worth_ui()`
- `WorthUiInstalledQueryDomain::measurement_view(...)`
- `WorthUiInstalledQueryDomain::live_measurement_view(...)`
- `WorthUiApplicationBuilder::register_query_view(...)`
- `WorthUiApp::resolve_query_view(...)`
- `WorthUiQueryInspection`

Query-free applications do not register a dummy domain or view.

## Core Mental Model

Worth Query owns the installed domain, operation attempt, live resource,
settlement, and exact result. Worth UI owns where and how an admitted result is
presented. The binding crate carries a compact installed reference across that
boundary; it does not create a second Query runtime.

## How It Executes

```text
install Worth UI domain package in Query
-> create installed measurement view
-> register view before application freeze
-> resolve installed binding reference
-> execute and settle through the Query workspace
-> submit the settled projection in a mounted-frame source closure
-> plan, mount, present, and publish through the ordinary application path
```

## Small Example

```rust
use worth_ui::facade::app::WorthUi;
use worth_ui::facade::query_binding::WorthUiQueryWorkspaceExt;

let view = workspace
    .worth_ui()?
    .measurement_view("inspector.measurements")?;
let app = WorthUi::app()
    .register_query_view(view)?
    .freeze()?;
```

This registers Query authority before application preparation. It does not
execute the view or create UI-local result state.

## Real Example

```rust
let reference = app
    .resolve_query_view(&view_identity, view_shape)
    .expect("the prepared app owns this installed view");
let settled = settle_projection(reference, requirements, &mut workspace)?;

let outcome = session.execute_mounted_frame(
    request,
    deadline,
    now,
    |sources| {
        sources.query_projection(|query| {
            query
                .admit_settled(settled)
                .expect("the exact registered projection should admit");
            query
                .submit_settled(&plan_link)
                .expect("the active plan link should resolve");
        });
    },
)?;
```

`settle_projection` must enter through
`WorthUiInstalledQueryBindingReference::enter_snapshot_attempt`, prepare the
consumer contract, and exhaust the typed execution, publication, consumption,
and settlement outcomes. The exact plan link depends on the admitted
application plan. The important boundary is stable: Query produces the settled
projection, and the ordinary mounted-frame closure admits it into UI
execution.

## How It Relates To Other Features

- Register the view on the same builder used by
  [Application lifecycle](./application-lifecycle.md).
- File and Rust-authored declarations can reference the same registered view.
- Query inspection cites exact attempt or settlement artifacts without copying
  them into UI-owned truth.

## Inspection And Debugging

Use `WorthUiQueryInspection` to inspect an exact settled projection or denial.
Minimal and rich evidence policies share the same underlying artifact. A
wrong-world attempt remains a typed Query denial.

## Anti-Patterns

- Copying Query result state into a UI cache.
- Recreating support, denial, recovery, or live-resource posture locally.
- Converting native values through JSON or text.
- Using an inspection receipt as a Query operation capability.
- Registering dummy Query state for a Query-free application.

## Current Limits

Only registered, installed view shapes and operations are available. Treat a
missing support row or typed denial as real boundary truth rather than falling
back to a local query implementation.

## Related Docs

- [Application lifecycle](./application-lifecycle.md)
- [Authored composition](./authored-composition.md)
- [Application inspection](./inspection.md)
