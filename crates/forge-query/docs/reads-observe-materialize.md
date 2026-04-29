# Reads, Observation, and Materialization

## What This Feature Is

These are the primary data-consumption surfaces for retained runtime handles.
They answer different questions:

- `read(...)` gives the current snapshot rows of a live view
- `observe(...)` drains incremental patch batches for a live view
- `materialize(...)` gives the current rows of a computed surface

They are not interchangeable. Each one reflects a different retained runtime
surface and cost shape.

## Why You Use It

- you need the current rows for a live surface
- you need query-shaped incremental updates rather than broad mutation noise
- you need the current derived rows for a computed surface

## Stable Entry Points

- `workspace.read(...)`
- `workspace.observe(...)`
- `workspace.materialize(...)`

Related explanation surfaces:

- `workspace.inspect(...)`
- `workspace.state(...)`

Narrower helper:

- `workspace.observe_computed(...)` exists, but the stabilized closeout names
  `read`, `observe`, and `materialize` as the primary stable consumption set.

## Core Mental Model

These APIs do not create new runtime surfaces. They consume surfaces you already
declared.

Think in terms of retained handles:

- a live view handle can be read as a snapshot or observed as patch batches
- a computed handle can be materialized as derived rows

The runtime keeps the handle identity, patch sequencing, and materialization
state. These methods only let you consume what is already retained.

## How It Executes

1. Declare a live view or computed surface first.
2. Route authoritative writes through `workspace.insert(...)`,
   `workspace.update(...)`, or `workspace.delete(...)`.
3. Consume current live rows with `read(...)`.
4. Drain live patch batches with `observe(...)`.
5. Consume current derived rows with `materialize(...)`.

`read(...)` and `materialize(...)` are snapshot reads. `observe(...)` is a
drain of retained incremental evidence. It only gives you what has accumulated
for that live surface since the last drain.

## Small Example

```rust
use forge_query::facade::ForgeQueryLiveView;
use serde_json::Value;

let mut workspace = runtime.workspace("tasks").unwrap();

let table: ForgeQueryLiveView<Value> = workspace
    .live_view("tasks.table", |q| {
        q.from("Task")
            .select(["identity.id", "title.value"])
            .schema_basis("tasks-table")
    })
    .unwrap();

workspace
    .insert("Task", |task| {
        task.aspect("identity.id", "task-1")
            .aspect("title.value", "Buy milk")
    })
    .unwrap();

let rows = workspace.read(&table);
let patches = workspace.observe(&table);
```

This is the smallest honest example because it shows the two different live
consumption paths on the same handle.

## Real Example

```rust
use forge_query::facade::{ForgeQueryDerivedViewHandle, ForgeQueryLiveView};
use serde_json::Value;

let mut workspace = runtime.workspace("builder").unwrap();

let view: ForgeQueryLiveView<Value> = workspace
    .live_view("tasks.builder-table", |q| {
        q.from("Task")
            .select(["identity.id", "title.value"])
            .order_by("title.value")
            .schema_basis("runtime-task-builder")
            .as_surface("tasks.builder-table")
    })
    .unwrap();

let titles: ForgeQueryDerivedViewHandle<Value> = workspace
    .computed(
        "tasks.builder-title-list",
        |c| {
            c.depends_on_live(&view)
                .reads(["title.value"])
                .produces(["runtime.title_list"])
        },
        TitleListMaintainer,
    )
    .unwrap();

workspace
    .insert("Task", |task| {
        task.aspect("identity.id", "task-1")
            .aspect("title.value", "Builder DX")
    })
    .unwrap();

let live_rows = workspace.read(&view);
let live_patches = workspace.observe(&view);
let computed_rows = workspace.materialize(&titles);
```

What is authoritative:

- the `Task` truth and the live surface over it

What is derived:

- the computed title list

What gets retained:

- live patch batches for `view`
- materialized derived rows for `titles`

What gets consumed:

- `read(&view)` returns current live rows
- `observe(&view)` drains incremental live delivery
- `materialize(&titles)` returns current derived rows

## How It Relates To Other Features

- Use [Live Views](./live-views.md) to declare the retained live surface.
- Use [Computed](./computed.md) to declare the retained derived surface.
- Use [State and Readiness Surfaces](./state.md) when you need typed posture
  rather than data rows.

These APIs are consumption boundaries. They should come after declaration, not
instead of it.

## Inspection And Debugging

If the data does not look right:

- inspect the live view to verify subscription and installation posture
- inspect the computed handle to verify dependencies, produced aspects, and
  patch posture
- snapshot state when you need a typed explanation of ready versus pending

## Anti-Patterns

- Treating `observe(...)` like a snapshot read.
- Treating `read(...)` like an event stream.
- Calling `materialize(...)` on a live view mental model.
- Assuming a drain returns historical truth forever instead of retained
  incremental evidence since the last drain.

## Current Limits

- These APIs are stable for runtime-backed synchronous live and derived
  surfaces.
- They reflect retained runtime state, not store-backed replay or durable
  continuation.
- Temporal and async/resource consumption semantics remain deferred future
  work.

## Related Docs

- [Live Views](./live-views.md)
- [Computed](./computed.md)
- [State and Readiness Surfaces](./state.md)
