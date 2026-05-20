# Live Views

## What This Feature Is

A live view is a durable, query-shaped runtime surface over authoritative
truth. It defines what collection or entity surface you care about, which
projected aspects belong in the surface, what view shape it uses, and how
incremental updates should be delivered back to the consumer.

## Why You Use It

- you need current truth as rows or view-shaped records
- you want writes to produce query-shaped patch batches instead of broad raw
  mutation noise
- you need a retained surface that computed state, effects, preview sessions,
  and inspection can all reuse

## Stable Entry Points

- `workspace.live_view(...)`
- `workspace.live_view_request(...)`
- `workspace.read(...)`
- `workspace.observe(...)`
- `workspace.inspect(...)`
- `workspace.state(...)`

Compatibility names such as `declare_live_view` still exist lower in the
runtime, but ordinary code should prefer the workspace surface.

For downstream crates that need domain-owned declaration vocabularies outside a
specific workspace instance, `ForgeQueryLiveViewBuilder::surface(...)` is also
public. That seam exists for declaration reuse, not as a replacement for the
ordinary workspace DX.

Good to know:

- `allow_traversal_relation(...)` is a real declaration input, not just schema
  metadata. When you use it, the declared bounded traversal is carried into the
  lower live-query request as well as the schema admission view.
- Traversal relations must be declared once each with a non-zero max depth.
- If you use the lower-level `workspace.live_view_request(...)` path, the
  runtime still checks that request traversal and schema traversal stay aligned.
  A traversal-bearing request with missing or narrower schema relation support
  is rejected before admission.
- The lower declarative request can now preserve hidden query-only projection,
  delivered result fields, non-equality predicates, traversal, and ordering.
  That same lower request shape is also what `compose_read(...)` uses before
  canonicalization.

## Core Mental Model

A live view is not just a query result. It is a retained runtime installation.
When you declare one, the runtime admits a query-shaped surface, installs the
subscription/lifecycle machinery for that surface, and keeps patch delivery
aligned with the declared projection and view shape.

What the handle means:

- it names one durable surface
- it carries the identity of the admitted query/view-shape installation
- it can be read, observed, inspected, preview-bound, and used as a dependency
  for computed state

What the runtime tracks automatically:

- installation and support evidence
- active lane and basis binding
- query-shaped patch grouping
- patch sequence for the surface

## How It Executes

1. You declare the live view with a builder or request object.
2. The runtime admits the live surface against schema/source/runtime support.
3. Authoritative writes route relevance against the declared surface.
4. `workspace.read(...)` gives the current snapshot rows.
5. `workspace.observe(...)` drains query delivery batches for the surface.
6. `workspace.inspect(...)` reconstructs the retained subscription proof chain.

Relevant writes produce live patch batches only when the changed meaning touches
the declared projection or grouping basis.

If the live surface includes declared traversal relations, the runtime keeps
that traversal posture attached to the admitted query identity instead of
asking downstream domain code to reconstruct it later.

## Small Example

```rust
use forge_query::facade::ForgeQueryLiveView;
use serde_json::Value;

let mut workspace = runtime.workspace("tasks").unwrap();

let table: ForgeQueryLiveView<Value> = workspace
    .live_view("tasks.table", |q| {
        q.from("Task")
            .select(["identity.id", "title.value"])
            .order_by("title.value")
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

This is the smallest honest example because it shows both consumer paths:
snapshot reads and incremental observation.

## Real Example

```rust
use forge_query::facade::{ForgeQueryInspection, ForgeQueryLiveView};
use serde_json::Value;

let mut workspace = runtime.workspace("tasks.grouped").unwrap();

let grouped: ForgeQueryLiveView<Value> = workspace
    .live_view("tasks.grouped", |q| {
        q.from("Task")
            .grouped_by("status.value")
            .select(["identity.id", "title.value", "status.value"])
            .schema_basis("tasks-grouped")
    })
    .unwrap();

let receipt = workspace
    .insert("Task", |task| {
        task.aspect("identity.id", "task-1")
            .aspect("title.value", "Seed task")
            .aspect("status.value", "todo")
    })
    .unwrap();

let grouped_patches = workspace.observe(&grouped);
let inspection = workspace.inspect(&grouped).unwrap();

assert!(receipt
    .affected_live_view_ids()
    .contains(&"tasks.grouped".to_string()));

match inspection {
    ForgeQueryInspection::LiveView(live) => {
        assert_eq!(live.view_name(), "tasks.grouped");
        assert_eq!(live.authority_lane(), forge_query::facade::ForgeQueryAuthorityLane::AuthoritativeTruth);
    }
    other => panic!("expected live inspection, got {other:?}"),
}
```

Traversal-bearing declaration example:

```rust
let topology = workspace
    .live_view("topology.half-edge-detail", |q| {
        q.from("WorthTopologyEntity")
            .detail()
            .select(["identity.id", "half_edge.kind"])
            .allow_traversal_relation("HalfEdgeNext", 2)
            .schema_basis("topology-half-edge-detail")
    })
    .unwrap();
```

Use this when the surface itself needs bounded graph reach. Do not treat it as
an after-the-fact schema note.

What is authoritative:

- the underlying `Task` truth

What is derived:

- the grouped patch shape and installation evidence are derived from the live
  declaration

What gets retained:

- subscription family and declaration digests
- basis binding and active lane digests
- budget and consumer attachment evidence

What gets observed:

- grouped membership batches rather than broad mutation noise

## How It Relates To Other Features

- Use [Computed](computed.md) when another runtime surface should derive from
  this live view.
- Use [Effects](../execution/effects.md) when a change in this surface should deliver UI
  output or stage pending work.
- Use [Intent Admission](../execution/intent-admission.md) when the caller
  needs the covered live-read intent path rather than `workspace.read(...)`
  convenience alone.
- Use the workspace overview when you need the whole runtime story around live
  views.

Live views are the normal upstream dependency for computed surfaces. They are
also one of the main things preview and branch sessions bind to.

## Inspection And Debugging

`workspace.inspect(&view)` tells you:

- the declared surface name
- query and view-shape digests
- subscription family and bridge declaration digests
- admission, activation, basis binding, and active lane digests
- budget and consumer attachment posture

Use this when you need to prove why a surface exists or why a write did or did
not affect it.

## Anti-Patterns

- Treating a live view as a one-shot query result instead of a retained runtime
  surface.
- Declaring a broad projection and then expecting narrow patch behavior.
- Treating `allow_traversal_relation(...)` as documentation-only metadata.
- Using lower-runtime installation helpers in ordinary product code.
- Expecting irrelevant aspect churn to affect the view when the declared
  projection does not include that meaning.

## Current Limits

- Live views are stable for runtime-backed synchronous use.
- They are the foundation for query-shaped delivery, but temporal/async mixed
  delivery is still deferred.
- Support posture still matters: future neighbors are exposed through the
  support matrix, not admitted automatically.

## Related Docs

- [Workspace Overview](../foundations/workspace-overview.md)
- [Computed](computed.md)
- [Effects](../execution/effects.md)


