# Worth UI AI Discovery

Worth UI is the product-facing UI platform boundary. Start with the named
`worth_ui::facade` audiences. Read lower crates only when maintaining the
implementation that owns that boundary.

Canonical reading order:

1. [Architecture](./docs/architecture.md)
2. [Authored composition](./docs/authored-composition.md)
3. [Interaction and intents](./docs/interaction-and-intents.md)
4. [Application lifecycle](./docs/application-lifecycle.md)
5. [Application inspection](./docs/inspection.md)
6. [Runtime subsystem map](./docs/runtime-subsystems.md)
7. [Query-backed UI views](./docs/query-binding.md)
8. [Milestone 3.10.1 migration](./docs/migration-3.10.1.md)

The longer contributor orientation remains in
[worth-ui-readme.md](./docs/worth-ui-readme.md).

## Ordinary Product Path

```text
WorthUi::app()
-> WorthUiApplicationBuilder
-> freeze()
-> WorthUiApp
-> launch()
-> WorthUiActiveApplicationSession
-> execute_mounted_frame(...)
-> typed outcome
```

`WorthUiApp` is one prepared generation. Launch consumes it. The active session
keeps application, graph, planning, Query, host, mounted publication, and
inspection identities coherent.

```rust
use worth_ui::facade::app::{
    UiMountedFrameRequest, UiPresentationDeadline, WorthUi,
};

let app = WorthUi::app()
    .freeze()
    .expect("application preparation should succeed");
let mut session = app.launch().expect("application should launch");
let outcome = session
    .execute_mounted_frame(
        UiMountedFrameRequest::all_bound_surfaces(),
        UiPresentationDeadline::at_tick(1),
        0,
        |_sources| {},
    )
    .expect("mounted-frame transition should start");
```

Treat the returned mounted-frame outcome exhaustively. Do not import an
intermediate runtime phase to skip a denial or recover a raw executor.

For Query-backed content, discover from
`worth_ui::facade::query_binding`. Follow the concrete type progression:

```text
host installation plan
-> installed scalar or collection registration
-> Query-issued projection observation
-> ordinary projection rebind
-> shape-specific affine fact receipt
-> mounted semantic text and host publication
```

Availability, currency/activity, stop posture, compatibility, native value,
and collection continuation are separate typed axes. Reporting identities and
inspection projections explain this progression but cannot enter it.

For human actions, keep this progression equally explicit:

```text
loss-aware native observations
-> presentation-bound semantic interaction
-> typed route, payload, and operability proof
-> move-only UI admission
-> exact typed provider execution
-> separate product or Query admission
-> declared consequence through ordinary rebind and mounted publication
```

Start with [Interaction and intents](./docs/interaction-and-intents.md). Native
input is not intent authority, UI admission is not domain admission, and a
diagnostic or visible posture cannot be promoted into either.

## Authority Boundaries

- `worth-ui-dsl` owns authored syntax, source structure, language diagnostics,
  normalization, and the sealed semantic package.
- `facade::source` owns file transport, watcher settlement, and runtime ingress.
- `worth-ui-runtime` owns active application, graph, planning, execution,
  interaction and intent admission, mounting, publication, host exchange, and
  operational inspection state.
- `facade::query_binding` and `worth-ui-query-binding` are the only
  Query-to-UI product route.
- `worth-ui-host-contract` and host adapters own native mechanics, not UI
  meaning.
- `facade::inspection` exposes read-only queries and receipts. It cannot mutate
  or reconstruct operational state.

IDs, digests, reports, and inspection receipts explain authority. They do not
grant it.

When investigating Query-backed output, start with
[Query-backed UI views](./docs/query-binding.md), then inspect the public
facade, the binding owner, the runtime projection-rebind entry, and only then
the Query substrate. Do not infer a workspace extension, literal field lookup,
or renderer-side query lane from older names.

## Authored Inputs

Use `WorthUiApplicationBuilder::with_rust_authored_input(...)` for typed Rust
composition. Use the production filesystem provider/watcher plus DSL compiler
to obtain one complete `WorthUiWatchedCandidateSubmission`, then pass it to
`with_candidate_submission(...)`.

Do not serialize typed authoring through JSON, split a candidate into loose
declarations, or parse source on a frame.

## Runtime Placement

Place runtime work using [Runtime subsystems](./docs/runtime-subsystems.md).
There are seven owners: application, graph, planning, mounting, observation,
inspection, and the thin session composition root. The map includes exact
future homes for Milestones 3.11, 3.12, 3.17, and 3.18.

## Verification

Before claiming a WORTH UI change is complete, run the focused evidence and the
repository-owned topology, compiler-contract, strict Clippy, formatting,
WORTH UI line-cap, boundary, and agent-context checks. A behavioral pass does
not override a red ownership or reachability gate.
