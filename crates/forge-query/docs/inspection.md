# Inspection

## What This Feature Is

Inspection is Forge Query's explanation surface. It lets you ask the runtime
why a live view, computed view, effect, write receipt, batch write receipt,
intent artifact, preview artifact, or branch artifact looks the way it does
without reaching through private runtime plumbing.

## Why You Use It

- you need to confirm that a live view really installed the subscription shape
  you thought it did
- you need to debug computed dependencies, pending patches, or refresh-fallback
  posture
- you need to explain effect routing, suppression, pending write-intents, or
  feedback phases
- you need a trustworthy record for preview promotion, branch-local staging, or
  intent denial behavior

## Stable Entry Points

- `workspace.inspect(...)`
- `ForgeQueryInspection`
- `ForgeQueryInspectionTarget`

The inspection family is part of the stabilized public runtime facade. It is
safe to build against now for synchronous runtime-backed surfaces.

## Core Mental Model

Inspection does not execute the feature again. It reads retained runtime
evidence that already exists because the feature was declared, installed,
executed, or closed out.

What you are holding:

- a typed explanation artifact chosen by the target you inspect
- evidence that is lane-aware, digest-bound, and specific to one runtime
  artifact family

What the runtime keeps track of automatically:

- authority lane and basis posture
- declaration digests
- support/admission evidence
- authored mutation metadata on write receipts when the write declared it
- structured declared and resolved target evidence on write receipts and batch
  components when mutation-family routing needs more than bare touched fallout
- existing-truth binding evidence on write receipts and batch components when a
  mutation intentionally targeted admitted authoritative preexisting truth
- canonical existing-truth binding digests on write receipts and aggregated
  binding digests on batch/session inspection when you need one stable session
  explanation instead of component-by-component reconstruction
- canonical naming mutation digests on batch/session inspection when one
  ordered authority session mixes attach, rebind, and remove outcomes and you
  need one stable aggregate explanation instead of re-summarizing components
- continuity-aware authority evidence on write receipts and batch components
  when an admitted update-existing mutation carries predecessor and successor
  identity through the bridge-backed authority lane
- same-batch symbolic target reference evidence on write receipts and batch
  components when an ordered batch intentionally targets truth created earlier
  in that same batch
- authoritative causality evidence on write receipts and batch components when
  the mutation crossed the bridge-backed authority lane
- authoritative provenance evidence on write receipts and batch components when
  the mutation crossed the bridge-backed authority lane
- aggregate batch mutation evidence on batch write receipts so session-wide
  target and authority breadth does not collapse into a final-component shadow
- retained counters and residue counts
- unified inspection digests for later auditing

## How It Executes

1. You pass a handle or receipt into `workspace.inspect(...)`.
2. The runtime converts it into a `ForgeQueryInspectionTarget`.
3. The runtime chooses the correct explanation family.
4. It derives a sealed inspection artifact from retained runtime evidence.
5. You pattern-match on `ForgeQueryInspection` and read the relevant fields.

Inspection is unified at the entry point, but specialized in the result.

## Small Example

```rust
use forge_query::facade::{ForgeQueryInspection, ForgeQueryLiveView};
use serde_json::Value;

let mut workspace = runtime.workspace("tasks").unwrap();

let view: ForgeQueryLiveView<Value> = workspace
    .live_view("tasks.table", |q| {
        q.from("Task")
            .select(["identity.id", "title.value"])
            .schema_basis("tasks-table")
    })
    .unwrap();

let inspection = workspace.inspect(&view).unwrap();

match inspection {
    ForgeQueryInspection::LiveView(live) => {
        assert_eq!(live.view_name(), "tasks.table");
        assert_eq!(
            live.authority_lane().as_str(),
            "authoritative-truth"
        );
    }
    other => panic!("expected live inspection, got {other:?}"),
}
```

This is the smallest honest example because it proves inspection is unified at
the call site but typed at the result.

## Real Example

```rust
use forge_query::facade::{
    ForgeQueryInspection, ForgeQueryLiveView, ForgeQueryPreviewOptions,
};
use serde_json::Value;

let mut workspace = runtime.workspace("workflow").unwrap();

let live: ForgeQueryLiveView<Value> = workspace
    .live_view("tasks.table", |q| {
        q.from("Task")
            .select(["identity.id", "title.value", "status.value"])
            .schema_basis("tasks-table")
    })
    .unwrap();

let titles = workspace
    .computed(
        "tasks.title-rollup",
        |c| {
            c.depends_on_live(&live)
                .reads(["title.value", "status.value"])
                .produces(["runtime.title_rollup"])
        },
        TitleListMaintainer,
    )
    .unwrap();

let effect = workspace
    .effect("tasks.publish-rollup", |e| {
        e.when_computed(&titles, ["runtime.title_rollup"])
            .condition_expression(
                "expr.publish-rollup",
                ["runtime.title_rollup"],
                ["ui.rollup"],
            )
            .deliver("ui.rollup")
            .meaningful_change_suppression()
    })
    .unwrap();

workspace
    .insert("Task", |task| {
        task.aspect("identity.id", "task-1")
            .aspect("title.value", "Inspection target")
            .aspect("status.value", "open")
    })
    .unwrap();

let effect_inspection = workspace.inspect(&effect).unwrap();

let mut preview = workspace
    .preview_with_options(
        "rollup preview",
        ForgeQueryPreviewOptions::redirected_delivery(),
    )
    .unwrap();
let preview_binding = preview.use_effect(&effect).unwrap();
let preview_outcome = preview.discard().unwrap();

let binding_inspection = workspace.inspect(&preview_binding).unwrap();
let outcome_inspection = workspace.inspect(&preview_outcome).unwrap();

match effect_inspection {
    ForgeQueryInspection::Effect(effect) => {
        assert_eq!(effect.trigger_source(), "tasks.title-rollup");
        assert_eq!(effect.target(), "ui.rollup");
        assert!(effect.pending_delivery_count() >= 1);
    }
    other => panic!("expected effect inspection, got {other:?}"),
}

match binding_inspection {
    ForgeQueryInspection::PreviewBinding(binding) => {
        assert_eq!(binding.label(), "rollup preview");
        assert_eq!(binding.effect_policy().as_str(), "redirected");
    }
    other => panic!("expected preview binding inspection, got {other:?}"),
}

match outcome_inspection {
    ForgeQueryInspection::PreviewOutcome(outcome) => {
        assert!(outcome.discarded());
        assert_eq!(outcome.promoted_write_count(), 0);
    }
    other => panic!("expected preview outcome inspection, got {other:?}"),
}
```

What is authoritative:

- live truth for `Task`

What is derived:

- computed rollup state
- effect routing evidence
- preview-local binding and closeout evidence

What gets retained:

- live installation digests

Write receipt and batch-component inspection now also preserve typed authority
evidence for:

- existing-truth target bindings
- same-batch symbolic target references
- naming-aware attachment, rebind, and removal outcomes
- causality and provenance bundles carried from the bridge
- computed dependency and produced-aspect evidence
- effect trigger, condition, phase, and pending-delivery evidence
- preview basis, policy, and residue evidence

What gets inspected:

- family-specific explanation artifacts through one entry point
- write receipt declared aspect operations, retained mutation metadata, and
  resolved target evidence when you need to certify authored meaning instead of
  only touched fallout
- write receipt existing-truth binding evidence when you need to prove which
  authoritative identity selected a preexisting target and which canonical
  binding artifact the runtime admitted
- write receipt and batch-component symbolic target evidence when you need to
  prove which same-batch declaration a later mutation resolved through
- write receipt and batch-component continuity evidence when you need to prove
  which authoritative predecessor continued as which successor set, which
  existing-truth binding basis anchored the mutation, and which
  lineage/continuity digests the bridge preserved
- typed continuity denials on preview lanes when authored intent asks for
  continuity evidence that only the authoritative bridge-backed lane can mint
- batch write receipt aggregate existing-truth and symbolic digests when you
  need one inspectable session summary for mixed authoritative-import lanes
- batch write receipt aggregate continuity digest when one session contains
  multiple continuity-aware updates and you need one inspectable continuity
  summary instead of reconstructing it from component artifacts
- batch write receipt aggregated touched aspects, affected live/derived
  surfaces, and ordered component operations when one semantic import or
  workflow had to execute as multiple data-dependent writes

## How It Relates To Other Features

- Pair this with [Live Views](./live-views.md) to explain subscription
  installation and active-lane posture.
- Pair it with [Computed](./computed.md) to inspect dependencies, materialized
  rows, and pending derived patches.
- Pair it with [Effects](./effects.md) to inspect routing, suppression, pending
  write-intents, and feedback phases.
- Pair it with [Branches And Previews](./branches-and-previews.md) to inspect
  policy, residue, and promotion closeout.

Inspection is the trust surface that keeps the rest of the runtime usable.

## Inspection And Debugging

`workspace.inspect(...)` can currently return:

- `LiveView`
- `DerivedView`
- `Effect`
- `WriteReceipt`
- `BatchWriteReceipt`
- `IntentReceipt`
- `IntentDenial`
- `EffectIntentReceipt`
- `PreviewBinding`
- `PreviewOutcome`
- `PreviewIntentReceipt`
- `BranchIntentReceipt`

Some especially important things to look for:

- live view: subscription family, basis digest, active lane digest, consumer
  attachment digest, budget policies, counter digests
- computed: upstream live/computed dependencies, dependency aspects, produced
  aspects, incremental posture, pending patch counts
- effect: trigger source, condition descriptor, target lane, effect policy,
  pending delivery counts, latest phase evidence, feedback graph
- preview: effect policy, basis evidence, admitted side-effect posture, closeout
  kind, residue counts, promotion/discard posture
- intent artifacts: source and target lanes, strategy identity/version, outcome
  digests, invariant evidence, denial stage

## Anti-Patterns

- Treating inspection as a cheap replacement for reads or materialization.
- Assuming the unified entry point means all artifact families expose the same
  fields.
- Reading private meaning into digest strings instead of using the typed accessors.
- Using inspection as permission to bypass support admission or effect policy.

## Current Limits

- Inspection is stable for the runtime-backed synchronous artifact families
  listed above.
- Future temporal and async families must extend this explanation surface rather
  than creating a second debugging API.
- Inspection explains runtime artifacts. It does not turn unsupported families
  into admitted ones.

## Related Docs

- [Workspace Overview](./workspace-overview.md)
- [Live Views](./live-views.md)
- [Computed](./computed.md)
- [Effects](./effects.md)
- [Branches And Previews](./branches-and-previews.md)
