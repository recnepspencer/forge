# Computed

## What This Feature Is

A computed surface is retained derived runtime state. It declares what upstream
live or computed surfaces it depends on, which aspects it reads, which aspects
it produces, and whether it maintains incremental patches or falls back to
whole-refresh posture.

## Why You Use It

- you need runtime state that should derive automatically from live truth
- you want nested derived surfaces without hand-wiring invalidation
- you need a durable handle that can be materialized, observed, inspected, and
  reused by effects, previews, or later computed surfaces

## Stable Entry Points

- `workspace.computed(...)`
- `workspace.computed_view(...)`
- `workspace.computed_definition(...)`
- `workspace.materialize(...)`
- `workspace.inspect(...)`
- `workspace.state(...)`

Preferred ordinary DX is `workspace.computed(...)`. Compatibility surfaces such
as `computed_view(...)` and `computed_definition(...)` still exist for narrower
cases.

For downstream crates that need domain-owned declaration vocabularies before a
workspace is assembled, `ForgeQueryComputedBuilder::surface(...)` is also
public. That seam is for declaration reuse, while ordinary runtime code should
still prefer the workspace surface.

## Core Mental Model

Computed state is not authoritative truth. It lives in the derived runtime lane
and must always be rebuildable from the authoritative surfaces it depends on.

What the handle means:

- it points at a retained derived surface
- it has a stable dependency contract
- it carries materialization and patch posture the runtime can inspect later

What the runtime tracks automatically:

- upstream live/computed dependencies
- read and produced aspect contracts
- incremental versus refresh-fallback posture
- materialized rows and pending derived patches

When a computed surface cannot honestly derive from one mutation delta alone,
whole-refresh fallback can rebuild from retained upstream rows instead of
pretending a local incremental update was sufficient.

## How It Executes

1. You declare the computed surface with a name, dependency contract, and
   maintainer.
2. The runtime validates that the upstream surfaces exist and that dependency
   structure is legal.
3. Relevant authoritative writes wake the computed surface through its declared
   upstreams.
4. The maintainer updates derived materialization and pending derived patches.
   Whole-refresh maintainers may rebuild from retained upstream live/computed
   rows when delta-only maintenance would be dishonest. When a whole-refresh
   computed declares more than one upstream live surface, the retained snapshot
   includes rows from every declared upstream live view, not only the view that
   happened to receive the triggering authoritative write. This also holds for
   downstream whole-refresh computeds reached through other computed surfaces:
   the runtime preserves the declared live siblings needed by the downstream
   rebuild contract rather than forcing the maintainer to reconstruct them
   manually. Whole-refresh maintainers also receive retained mutation context:
   commit identity, snapshot token, touched aspect paths, and any authored
   mutation metadata the write carried.
5. `workspace.materialize(...)` returns current derived rows.
6. `workspace.inspect(...)` explains dependencies, produced aspects, and patch
   posture.

Nested computed surfaces execute in dependency order rather than in accidental
call order.

## Small Example

```rust
use forge_query::facade::{ForgeQueryDerivedViewHandle, ForgeQueryLiveView};
use serde_json::Value;

let mut workspace = runtime.workspace("tasks").unwrap();

let live: ForgeQueryLiveView<Value> = workspace
    .live_view("tasks.table", |q| {
        q.from("Task")
            .select(["identity.id", "title.value"])
            .schema_basis("tasks-table")
    })
    .unwrap();

let titles: ForgeQueryDerivedViewHandle<Value> = workspace
    .computed(
        "tasks.titles",
        |c| {
            c.depends_on_live(&live)
                .reads(["title.value"])
                .produces(["title.summary"])
        },
        TitleListMaintainer,
    )
    .unwrap();

let rows = workspace.materialize(&titles);
```

This is the smallest honest example because it shows the core pieces:
upstream dependency, read contract, produced contract, and derived
materialization.

## Real Example

```rust
use forge_query::facade::{ForgeQueryDerivedViewHandle, ForgeQueryInspection, ForgeQueryLiveView};
use serde_json::Value;

let mut workspace = runtime.workspace("workflow").unwrap();

let live: ForgeQueryLiveView<Value> = workspace
    .live_view("tasks.table", |q| {
        q.from("Task")
            .select(["identity.id", "title.value"])
            .schema_basis("tasks-table")
    })
    .unwrap();

let titles: ForgeQueryDerivedViewHandle<Value> = workspace
    .computed(
        "computed.titles",
        |c| {
            c.depends_on_live(&live)
                .reads(["title.value"])
                .produces(["title.summary"])
        },
        TitleListMaintainer,
    )
    .unwrap();

let summary: ForgeQueryDerivedViewHandle<Value> = workspace
    .computed(
        "computed.summary",
        |c| {
            c.depends_on_computed(&titles)
                .reads(["title.summary"])
                .produces(["validation.state"])
        },
        SummaryMaintainer,
    )
    .unwrap();

workspace
    .insert("Task", |task| {
        task.aspect("identity.id", "task-1")
            .aspect("title.value", "Nested title")
    })
    .unwrap();

let rows = workspace.materialize(&summary);
let inspection = workspace.inspect(&summary).unwrap();

match inspection {
    ForgeQueryInspection::DerivedView(derived) => {
        assert_eq!(derived.upstream_derived_views(), &["computed.titles".to_string()]);
        assert_eq!(derived.produced_aspects(), &["validation.state".to_string()]);
    }
    other => panic!("expected derived inspection, got {other:?}"),
}
```

What is authoritative:

- `Task` truth and the live view over it

What is derived:

- `computed.titles`
- `computed.summary`

What gets retained:

- dependency digests
- produced-aspect digests
- materialized rows
- pending incremental or refresh-fallback patch posture
- retained mutation context for whole-refresh rebuilds

What gets inspected:

- upstream live/computed membership
- dependency and produced aspect contracts
- materialization and pending patch evidence

## How It Relates To Other Features

- Use [Live Views](./live-views.md) as the normal authoritative upstream.
- Use [Effects](./effects.md) when a change in computed output should deliver
  something or stage pending work.
- Use workspace state snapshots when you need readiness/posture rather than the
  derived rows themselves.

Computed surfaces are the normal place to keep runtime-derived readiness,
validation, rollups, or UI-friendly derived state. They are not the place to
hide authoritative writes.

## Inspection And Debugging

`workspace.inspect(&computed)` tells you:

- authority lane
- upstream live and computed dependencies
- dependency aspects and produced aspects
- whether delivery is incremental or whole-refresh fallback
- materialized row count
- pending patch counts and digests

This is the main way to verify whether a computed surface is wired correctly.

## Anti-Patterns

- Using computed state to smuggle authoritative mutations.
- Declaring hidden dependencies outside the `reads(...)` and
  `depends_on_live(...)` / `depends_on_computed(...)` contract.
- Assuming nested computeds are just callback chains instead of ordered retained
  runtime surfaces.
- Ignoring refresh-fallback posture and pretending it is incremental.
- Rebuilding from host-side shadow state instead of using retained upstream
  runtime rows when refresh fallback is declared.
- Reconstructing sibling upstream truth yourself because a write touched only
  one live view. The runtime already hands whole-refresh maintainers the
  retained rows for every declared upstream live surface.

## Current Limits

- Computed surfaces are stable for runtime-backed synchronous derived state.
- They must remain rebuildable from authority; they are not persistent truth.
- Async/resource-derived state and temporal execution lanes are deferred future
  neighbors, not part of the stabilized computed contract yet.

## Related Docs

- [Workspace Overview](./workspace-overview.md)
- [Live Views](./live-views.md)
- [Effects](./effects.md)
