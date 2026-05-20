# Branches and Previews

## What This Feature Is

Preview and branch sessions are isolated runtime contexts built from the same
workspace model. They let you reuse retained live, computed, and effect
surfaces while moving writes and intent-like work into preview-local or
branch-local authority lanes instead of mutating current truth directly.

## Why You Use It

- you want to try changes without touching authoritative truth
- you need isolated preview-local writes and closeout behavior
- you need branch-local intent staging that never silently executes against the
  authoritative lane

## Stable Entry Points

Stable session entry points:

- `workspace.preview(...)`
- `workspace.preview_with_options(...)`
- `workspace.branch(...)`
- `workspace.branch_with_options(...)`

Stable option constructors:

- `ForgeQueryPreviewOptions::derive_only()`
- `ForgeQueryPreviewOptions::muted()`
- `ForgeQueryPreviewOptions::redirected_delivery()`
- `ForgeQueryPreviewOptions::sandboxed_write_intent()`
- `ForgeQueryBranchOptions::derive_only()`
- `ForgeQueryBranchOptions::muted()`
- `ForgeQueryBranchOptions::redirected_delivery()`
- `ForgeQueryBranchOptions::sandboxed_write_intent()`

Stable preview-local operations:

- bind live/computed/effect handles into a preview
- stage preview-local writes
- discard or promote a preview outcome

Support-gated neighbors:

- preview-local or branch-local intent execution still depends on admitted
  intent support and the correct sandboxed policy

## Core Mental Model

Preview and branch sessions are not separate products. They are lane-shifted
contexts over the same retained runtime surfaces.

Preview:

- binds existing handles into a preview lane
- can stage preview-local writes
- can discard or promote staged work
- defaults to `derive_only`

Branch:

- opens a branch-local lane
- is primarily about branch-local intent staging in the current public surface
- defaults to `derive_only`

The key idea is isolation by authority lane, not by reimplementing the whole
runtime.

## How It Executes

Preview path:

1. Open a preview session from the workspace.
2. Bind live, computed, and optional effect handles.
3. Stage preview-local writes or preview-local intent work, depending on
   policy and support.
4. Discard or promote the preview.

Branch path:

1. Open a branch session from the workspace.
2. Use the declared branch effect policy.
3. Stage branch-local intent work when support and policy admit it.

`derive_only` is the default for both preview and branch sessions. That means
derived behavior is allowed, but delivery and write-intent work are denied or
muted unless you explicitly choose a broader policy.

## Small Example

```rust
use forge_query::facade::ForgeQueryPreviewOptions;

let mut workspace = runtime.workspace("preview").unwrap();

let mut preview = workspace
    .preview_with_options(
        "draft create",
        ForgeQueryPreviewOptions::sandboxed_write_intent(),
    )
    .unwrap();

preview
    .insert("Task", |task| {
        task.aspect("identity.id", "preview-1")
            .aspect("title.value", "Preview-only task")
    })
    .unwrap();

let outcome = preview.discard();
```

This is the smallest honest example because it shows preview-local staging with
an explicit closeout result instead of pretending preview writes are ordinary
truth writes.

## Real Example

```rust
use forge_query::facade::{
    ForgeQueryBranchOptions, ForgeQueryInspection, ForgeQueryLiveView, ForgeQueryPreviewOptions,
};
use serde_json::{json, Value};

let mut workspace = runtime.workspace("workflow").unwrap();

let live: ForgeQueryLiveView<Value> = workspace
    .live_view("tasks.preview-bind", |q| {
        q.from("Task")
            .select(["identity.id", "title.value"])
            .schema_basis("tasks-preview")
    })
    .unwrap();

let preview_outcome = {
    let mut preview = workspace
        .preview_with_options(
            "preview execution",
            ForgeQueryPreviewOptions::redirected_delivery(),
        )
        .unwrap();
    preview.use_view(&live);
    preview
        .insert("Task", |task| {
            task.aspect("identity.id", "preview-task")
                .aspect("title.value", "Preview execution task")
        })
        .unwrap();
    preview.discard()
};

let branch_result = {
    let mut branch = workspace
        .branch_with_options(
            "branch local intent",
            ForgeQueryBranchOptions::sandboxed_write_intent(),
        )
        .unwrap();
    branch.execute_intent(forge_query::facade::ForgeQueryIntentDeclaration::strategy_commit(
        "branch-reconcile",
        "strategy.intent.reconcile",
        "1.0",
        "intent.reconcile.input.v1",
        json!({ "entity": "task-1", "title": "branch title" }),
    ))
};
```

What is authoritative:

- the workspace's current truth lane

What is isolated:

- preview writes land in `PreviewTruth`
- branch-local admitted intents land in `BranchLocalTruth`

What gets retained:

- binding evidence
- execution evidence
- closeout evidence
- preview or branch-local intent receipts when admitted

What closeout means:

- `discard()` proves staged work did not leak into authoritative truth
- `promote()` attempts an authoritative handoff and can fail typed and early

## How It Relates To Other Features

- Use [Live Views](../runtime-surfaces/live-views.md) and [Computed](../runtime-surfaces/computed.md) as the
  retained handles you bind into previews.
- Use [Writes and Intent Boundaries](../execution/writes-and-intents.md) when deciding
  whether work belongs in direct writes, preview-local staging, or branch-local
  intent paths.
- Use [State and Readiness Surfaces](state.md) when you need typed posture
  snapshots around stable versus deferred families.

Preview and branch sessions are lane-control features, not alternate truth
engines.

## Inspection And Debugging

The runtime can explain:

- preview handle binding evidence
- preview execution evidence
- preview outcome and closeout residue
- preview-local and branch-local intent receipts

Look for:

- effect policy
- source lane versus target lane
- residue class counts
- basis snapshot tokens and denial digests on failed promotion

## Anti-Patterns

- Treating preview writes as if they are already authoritative.
- Assuming `derive_only` allows delivery or write-intent work.
- Expecting branch-local or preview-local intents to bypass support admission.
- Treating preview or branch sessions as separate domain runtimes rather than
  authority-lane shifts over retained surfaces.

## Current Limits

- Preview and branch sessions are stable as runtime-backed isolation contexts.
- Preview-local writes, binding, discard, and typed promotion denials are part
  of the current surface.
- Preview-local and branch-local intent work still depends on admitted intent
  support and the correct sandboxed policy.
- Durable preview replay, store-backed branch semantics, and temporal/async
  branch behavior remain future work.

## Related Docs

- [Workspace Overview](workspace-overview.md)
- [Writes and Intent Boundaries](../execution/writes-and-intents.md)
- [State and Readiness Surfaces](state.md)


