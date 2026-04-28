# Forge Query Workspace Overview

## What This Feature Is

`ForgeQueryWorkspace` is the stabilized public runtime facade for ordinary
runtime-backed `forge-query` work. It is the place where product code declares
live views, computed state, and effects, performs reads and writes, opens
preview or branch sessions, snapshots state, and inspects retained runtime
evidence.

## Why You Use It

- you want one public context for runtime-backed query work
- you want to compose live views, computed state, and effects without reaching
  into lower-runtime plumbing
- you want branch/preview/state/inspection surfaces to line up around one
  stable mental model

## Stable Entry Points

Stable runtime-backed entry points:

- `runtime.workspace(...)`
- `workspace.live_view(...)`
- `workspace.computed(...)`
- `workspace.effect(...)`
- `workspace.preview(...)` / `workspace.branch(...)`
- `workspace.write(...)`
- `workspace.read(...)`
- `workspace.observe(...)`
- `workspace.materialize(...)`
- `workspace.state(...)`
- `workspace.inspect(...)`
- `workspace.public_api_contract()`
- `workspace.public_handle_contract()`
- `workspace.public_support_matrix()`
- `workspace.admit_public_api_family(...)`

Compatibility entry points still exist, but new code should prefer the names
above.

Good to know:

- `workspace.intent(...)` is public vocabulary, but it is not in the stable
  compatibility support set yet.
- Method presence is not a support claim. Use the support matrix and admission
  gate when you are near deferred or unsupported families.

## Core Mental Model

The workspace is not the owner of truth. `forge-relational` still owns truth
semantics, and `forge-signal` still owns reactive evaluation. The workspace is
the public context that lets you declare what you want from those runtimes in a
coherent, inspectable shape.

Think of it this way:

- authoritative truth lives below the workspace
- the workspace declares durable runtime surfaces over that truth
- handles returned by the workspace are retained runtime objects, not raw data
- `read`, `observe`, `materialize`, `state`, and `inspect` let you ask
  different questions about those retained surfaces

If you are building ordinary runtime-backed product features, the workspace is
the place you should start from.

## How It Executes

The typical workspace lifecycle looks like this:

1. Open a named workspace from a configured runtime.
2. Declare live views, computed surfaces, and effects.
3. Write authoritative changes through `workspace.write(...)`.
4. Read current rows, drain patches, or materialize derived rows from retained
   handles.
5. Inspect handles or snapshot state when you need explanations or readiness.
6. Open preview or branch sessions when you need isolated experimentation.

The workspace keeps the durable handles and their retained evidence aligned with
the same runtime. A handle from one runtime is not portable into another
runtime's inspection or state APIs.

## Small Example

```rust
use forge_query::facade::{ForgeQueryLiveView, ForgeQueryWriteCommand};
use serde_json::{json, Value};

let mut workspace = runtime.workspace("editor").unwrap();

let tasks: ForgeQueryLiveView<Value> = workspace
    .live_view("tasks.table", |q| {
        q.from("Task")
            .select(["identity.id", "title.value"])
            .order_by("title.value")
            .schema_basis("editor-task-table")
    })
    .unwrap();

workspace
    .write(ForgeQueryWriteCommand::Insert {
        collection: "Task".to_string(),
        payload: json!({
            "identity": { "id": "" },
            "title": { "value": "Buy milk" },
        }),
    })
    .unwrap();

let rows = workspace.read(&tasks);
let patches = workspace.observe(&tasks);
```

This is the smallest honest example because it shows the full loop:
declaration, authoritative write, snapshot read, and incremental observation.

## Real Example

```rust
use forge_query::facade::{
    ForgeQueryDerivedViewHandle, ForgeQueryEffectHandle, ForgeQueryInspection,
    ForgeQueryLiveView, ForgeQueryWriteCommand,
};
use serde_json::{json, Value};

let mut workspace = runtime.workspace("workflow.editor").unwrap();

let canvas: ForgeQueryLiveView<Value> = workspace
    .live_view("workflow.editor.canvas", |q| {
        q.from("WorkflowNode")
            .select(["identity.id", "layout.frame", "validation.state"])
            .order_by("identity.id")
            .schema_basis("workflow-editor-canvas")
    })
    .unwrap();

let readiness: ForgeQueryDerivedViewHandle<Value> = workspace
    .computed(
        "workflow.editor.readiness",
        |c| {
            c.depends_on_live(&canvas)
                .reads(["validation.state", "layout.frame"])
                .produces(["runtime.readiness"])
        },
        WorkflowReadinessMaintainer,
    )
    .unwrap();

let badges: ForgeQueryEffectHandle<Value> = workspace
    .effect("workflow.editor.badges", |e| {
        e.when_computed(&readiness, ["runtime.readiness"])
            .condition_expression(
                "expr.workflow.badges",
                ["runtime.readiness"],
                ["ui.badges"],
            )
            .deliver("ui.badges")
            .meaningful_change_suppression()
    })
    .unwrap();

workspace
    .write(ForgeQueryWriteCommand::Insert {
        collection: "WorkflowNode".to_string(),
        payload: json!({
            "identity": { "id": "" },
            "layout": { "frame": "0,0,320,80" },
            "validation": { "state": "ready" },
        }),
    })
    .unwrap();

let canvas_patches = workspace.observe(&canvas);
let readiness_rows = workspace.materialize(&readiness);
let badge_explanation = workspace.inspect(&badges).unwrap();

match badge_explanation {
    ForgeQueryInspection::Effect(effect) => {
        assert_eq!(effect.trigger_source(), "workflow.editor.readiness");
    }
    other => panic!("expected effect inspection, got {other:?}"),
}
```

What is authoritative:

- `workspace.write(...)` changes authoritative truth

What is derived:

- `readiness` is derived runtime state
- `badges` is effect delivery state over derived output

What gets retained:

- live subscription installation for `canvas`
- materialized derived rows and patch posture for `readiness`
- delivery and phase evidence for `badges`

What gets inspected:

- one unified `workspace.inspect(...)` call can explain each retained surface

## How It Relates To Other Features

- Use [Live Views](./live-views.md) when you need query-shaped current truth.
- Use [Computed](./computed.md) when runtime state should derive from live or
  other computed surfaces.
- Use [Effects](./effects.md) when a surface should deliver or stage something
  because another surface changed.
- Use preview or branch sessions when the work should remain isolated from
  current truth.

`workspace.read(...)` and `workspace.materialize(...)` are snapshot-style reads.
`workspace.observe(...)` is the incremental patch path. `workspace.state(...)`
and `workspace.inspect(...)` are explanation surfaces, not substitutes for
domain data access.

## Inspection And Debugging

The workspace gives you two main explanation paths:

- `workspace.state(...)` for a typed readiness/supported/pending snapshot
- `workspace.inspect(...)` for retained evidence about a handle or receipt

Use the public support and handle contracts when you need to understand whether
a family is stable, deferred, or unsupported before exposing it in another
runtime.

## Anti-Patterns

- Treating the workspace as the owner of truth semantics.
- Reaching for lower-runtime APIs in ordinary product code when the workspace
  already owns the public surface.
- Assuming method presence means feature support without checking the support
  matrix or admission gate.
- Passing handles across runtimes and expecting inspection or state APIs to work.

## Current Limits

- The stabilized workspace surface is safe for runtime-backed, synchronous
  composition.
- Temporal basis, async/resource execution, mixed-cause delivery, store-backed
  execution, and durable artifact reload remain deferred.
- `workspace.intent(...)` is public vocabulary but not yet part of the stable
  compatibility support set.

## Related Docs

- [Live Views](./live-views.md)
- [Computed](./computed.md)
- [Effects](./effects.md)
