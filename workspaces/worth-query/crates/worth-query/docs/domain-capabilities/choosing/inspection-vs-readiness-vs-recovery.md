# Inspection Vs Readiness Vs Recovery

## What This Page Helps You Choose

Use this page when you need to answer one of these questions:

- "is this family or surface supported here?"
- "what truth did Query retain after that run?"
- "what should I do next after the stop?"

Those questions map to different Query surfaces.

## Why You Use It

- choose readiness before you run something
- choose inspection after you already have retained declaration truth
- choose recovery when an ordinary, checked, or proof-visible lane stopped and
  you need a next action

## Surfaces Compared

- [Declaration Entry Readiness](../declaration-entry-readiness.md)
- [Declaration Entry Inspection](../declaration-entry-inspection.md)
- [Recovery Boundary](../recovery-boundary.md)

## Core Mental Model

Think of the three surfaces this way:

1. readiness: "what is supported before I commit to this run?"
2. inspection: "what retained truth do I have after crossing this seam?"
3. recovery: "who owns the fix, and what should I do next?"

They are intentionally not the same thing.

## How To Choose

Choose **readiness** when:

- you want family-level support posture
- you need executable crossing rows before one concrete run
- you want to know whether lower-authority or grouped support is available

Choose **inspection** when:

- you already have a retained declaration-entry artifact
- you need to read route, receipt, envelope, or lower-authority posture
- you want one unified read surface over retained seam truth

Choose **recovery** when:

- an ordinary, checked, or proof-visible surface stopped
- you need one machine-readable next-step action
- you need basis posture, aspect posture, source family, or grouped member
  context for the stop

## Small Example

Use readiness before a run:

```rust
let report = handle.declaration_entry_readiness::<AttachFaceMaterial>();
```

Use inspection after a run:

```rust
let inspection = workspace.inspections()?.inspect(&subject)?;
```

Use recovery after a stop:

```rust
let brief = handle.recover_from_outcome(&outcome);
```

## Real Example

If a grouped neighborhood edit stopped:

- use readiness first if you need to know whether grouped contributions or
  grouped products are even supported for that family
- use inspection when you need to read the retained route, receipt, envelope,
  or lower-authority posture after one successful grouped or declaration-entry
  crossing
- use recovery when the grouped checked or proof lane stopped and your app must
  decide whether to refresh, rebind one member, or inspect a conflict

## How It Relates To Other Features

- [Ordinary Outcomes](../ordinary-outcomes.md) is the compact source for many
  recovery entry points.
- [Grouped Authoring](../grouped-authoring.md) and [Grouped Products](../grouped-products.md)
  feed richer grouped truth into inspection and recovery.
- [Signal Compatibility Orchestration](../signal-compatibility-orchestration.md)
  and [Continuation Pipeline](../continuation-pipeline.md) both produce stops
  that recovery can project into next-step actions.

## Inspection And Debugging

Use readiness when you need:

- family support rows
- declarative availability before a run

Use inspection when you need:

- retained digests
- route, receipt, envelope, relational, bridge, or signal posture

Use recovery when you need:

- `recommended_action()`
- `source_family()`
- `basis_posture()`
- `aspect_posture()`
- grouped member context on grouped stops

## Anti-Patterns

- using readiness as if it were a post-run inspection artifact
- using inspection as if it recommended the next repair action
- skipping recovery and guessing next steps from denial strings
- treating a successful inspection artifact as proof that later execution is
  ready

## Current Limits

- readiness is support posture, not execution proof
- inspection is a read surface, not an action planner
- recovery preserves rich stop context, but it does not automatically resolve
  collaborative conflicts for you

## Related Docs

- [Declaration Entry Readiness](../declaration-entry-readiness.md)
- [Declaration Entry Inspection](../declaration-entry-inspection.md)
- [Recovery Boundary](../recovery-boundary.md)
- [Grouped Authoring](../grouped-authoring.md)
