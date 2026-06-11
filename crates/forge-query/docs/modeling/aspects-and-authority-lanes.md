# Aspects And Authority Lanes

## What This Feature Is

Aspects are the auditable names Forge Query uses for fields and derived
surfaces. Authority lanes are the auditable places where truth, derived state,
delivery residue, preview residue, and future temporal or async state live.

Together they answer two questions:

- what data contract does this surface read, produce, trigger on, or write
- which lane is allowed to own the resulting state

## Why You Use It

- you want computed and effect contracts to stay explicit instead of becoming
  spooky reactivity
- you want preview and branch work to reuse declarations without leaking into
  authoritative truth
- you want inspection and state snapshots to make support posture obvious
- you want future async work to extend the same semantics instead of inventing
  parallel vocabulary

## Stable Entry Points

- computed builder contracts such as `reads(...)` and `produces(...)`
- effect builder contracts such as `when_live(...)`, `when_computed(...)`, and
  `condition_expression(...)`
- `workspace.state(...)`
- `workspace.inspect(...)`
- `ForgeQueryAuthorityLane`
- `ForgeQueryPreviewOptions`
- `ForgeQueryBranchOptions`
- `workspace.public_handle_contract()`

Authority lanes are part of the stabilized public runtime vocabulary. Some
lanes are stable today and some exist as explicit future neighbors that are
visible before they are implemented.

## Core Mental Model

Aspects describe semantic data dependencies. Lanes describe ownership and
execution authority.

Aspects tell the runtime:

- what a live or computed surface reads
- what a computed surface produces
- what an effect watches
- what a patch or delta is about

Authority lanes tell the runtime:

- where authoritative truth lives
- whether a write is branch-local or preview-local
- whether a surface is derived runtime state instead of truth
- whether residue belongs to delivery, pending intent, or an external bridge

Current public lane vocabulary:

- `AuthoritativeTruth`
- `BranchLocalTruth`
- `PreviewTruth`
- `DerivedRuntimeState`
- `EffectDeliveryState`
- `PendingWriteIntent`
- `BridgeExternalState`
- `TemporalExecutionState`
- `AsyncResourceState`

## How It Executes

1. You declare a live, computed, or effect surface with explicit aspects.
2. The runtime binds that surface to one or more authority lanes.
3. Writes and delivery route through the admitted lane for that surface.
4. `workspace.state(...)` and `workspace.inspect(...)` report the resulting lane
   and the aspect contract that produced it.
5. Preview and branch sessions can redirect, mute, or sandbox side effects by
   changing lane admission rather than changing the declaration itself.

## Small Example

```rust
let titles = workspace
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

let state = workspace.state(&titles).unwrap();

assert_eq!(state.authority_lane().as_str(), "derived-runtime-state");
```

This is the smallest honest example because it shows aspects and lanes playing
different roles: `reads` and `produces` name the semantic contract, while the
state snapshot names the authority lane.

## Real Example

```rust
use forge_query::facade::ForgeQueryPreviewOptions;

let titles = workspace
    .computed(
        "workflow.titles",
        |c| {
            c.depends_on_live(&tasks)
                .reads(["title.value", "status.value"])
                .produces(["runtime.title_rollup", "validation.state"])
        },
        WorkflowTitleMaintainer,
    )
    .unwrap();

let readiness = workspace
    .computed(
        "workflow.readiness",
        |c| {
            c.depends_on_computed(&titles)
                .reads(["validation.state"])
                .produces(["readiness.state"])
        },
        WorkflowReadinessMaintainer,
    )
    .unwrap();

let publish = workspace
    .effect("workflow.publish-readiness", |e| {
        e.when_computed(&readiness, ["readiness.state"])
            .condition_expression(
                "expr.ready-to-publish",
                ["readiness.state"],
                ["delivery.publish"],
            )
            .deliver("workflow.delivery")
            .meaningful_change_suppression()
    })
    .unwrap();

workspace
    .insert("Task", |task| {
        task.aspect("identity.id", "task-1")
            .aspect("title.value", "Approval form")
            .aspect("status.value", "open")
    })
    .unwrap();

let derived_state = workspace.state(&readiness).unwrap();
let effect_inspection = workspace.inspect(&publish).unwrap();
let preview_label =
    forge_query::facade::ForgeQuerySessionLabel::scoped_strs("workflow", ["approval-preview"])
        .unwrap();

let mut preview = workspace
    .preview_with_options(
        preview_label,
        ForgeQueryPreviewOptions::sandboxed_write_intent(),
    )
    .unwrap();
let preview_binding = preview.use_effect(&publish).unwrap();

assert_eq!(derived_state.authority_lane().as_str(), "derived-runtime-state");

match effect_inspection {
    forge_query::facade::ForgeQueryInspection::Effect(effect) => {
        assert_eq!(effect.target_lane().as_str(), "effect-delivery-state");
        assert_eq!(effect.trigger_aspects(), &["readiness.state".to_string()]);
    }
    other => panic!("expected effect inspection, got {other:?}"),
}

let binding = workspace.inspect(&preview_binding).unwrap();
match binding {
    forge_query::facade::ForgeQueryInspection::PreviewBinding(binding) => {
        assert_eq!(binding.preview_lane().as_str(), "preview-truth");
        assert!(binding.pending_write_intent_admitted());
        assert!(!binding.authoritative_side_effect_admitted());
    }
    other => panic!("expected preview binding inspection, got {other:?}"),
}
```

What is authoritative:

- the live `Task` truth

What is derived:

- `runtime.title_rollup`
- `validation.state`
- `readiness.state`

What lane each piece lives in:

- live truth: `AuthoritativeTruth`
- computed readiness: `DerivedRuntimeState`
- ordinary effect delivery: `EffectDeliveryState`
- sandboxed preview write-intent staging: `PreviewTruth` and
  `PendingWriteIntent`

## How It Relates To Other Features

- [Computed](../runtime-surfaces/computed.md) uses aspects to make derived dependencies explicit.
- [Effects](../execution/effects.md) uses aspects plus lanes to constrain routing and
  feedback.
- [Branches And Previews](../foundations/branches-and-previews.md) changes lane admission
  without rewriting declarations.
- [State](../foundations/state.md) exposes lane posture as a stable public snapshot.
- [Inspection](../capabilities/inspection.md) makes both the aspect contract and lane posture
  visible.

## Inspection And Debugging

The most important places to check lanes and aspects are:

- computed inspection:
  dependency aspects, produced aspects, derived-runtime lane
- effect inspection:
  trigger aspects, condition inputs and outputs, target lane, effect policy
- live view inspection:
  authoritative lane, basis digest, active lane digest
- preview and branch inspection:
  source lane, preview or branch lane, admitted side-effect posture
- state snapshots:
  current lane plus stable/pending/unsupported posture

## Anti-Patterns

- Treating aspect names as optional decoration instead of contract.
- Writing logic that assumes derived runtime state is authoritative truth.
- Letting preview or branch flows silently write into authoritative truth.
- Introducing new public lane vocabulary for async work instead of extending the
  current lane set.
- Hiding dependencies outside `reads(...)`, trigger aspects, or produced
  aspects.

## Current Limits

- `AuthoritativeTruth`, `DerivedRuntimeState`, preview, branch, delivery, and
  pending write-intent lanes are part of the current runtime-backed story.
- `TemporalExecutionState` and `AsyncResourceState` are explicit future lanes,
  visible now so downstream code can plan around them without pretending they
  execute yet.
- Authority lanes are infrastructure vocabulary. They should not be replaced
  with domain-specific lane names in public DX.

## Declaration-entry retrofit

The aspect story extends beyond runtime reads, computed surfaces, and
effects. Declaration-entry artifacts such as progression, foundational
evidence, route plans, receipts, envelopes, relational routing, bridge
routing, and signal compatibility now need to carry aspect-aware semantic
granularity too. The same discipline applies there: aspects are contracts, not
optional decoration, and later binding/orchestration phases should consume
retained aspect truth instead of re-inferring it from broad artifact classes.

In the currently shipped declaration-entry retrofit, that means:

- envelopes publish the public semantic slice
- relational routing lowers only the relational slice from that publication
- bridge routing lowers only the bridge slice and separately freezes what
  actually mapped into bridge continuation semantics
- signal compatibility checks dependency and produced-aspect posture against
  the envelope slice before later execution exists

So aspects are now doing the same kind of load-bearing work on the
declaration-entry side that they already do in runtime, relational, bridge,
and signal execution surfaces.

## Related Docs

- [Computed](../runtime-surfaces/computed.md)
- [Effects](../execution/effects.md)
- [Branches And Previews](../foundations/branches-and-previews.md)
- [State](../foundations/state.md)
- [Inspection](../capabilities/inspection.md)


