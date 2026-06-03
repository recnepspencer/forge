# Routing And Evaluation

This guide covers the bridge's core computational job:

- accept authoritative truth change
- route it deterministically into compute invalidation
- evaluate against an explicit truth basis

If `forge-relational` owns truth and `forge-signal` owns derived computation,
the bridge owns the causal transfer between them.

## Start From The Facade

For most work, begin here:

```rust
use forge_runtime_bridge::facade::*;
```

This guide goes deeper than the quickstart, but it is still about normal
runtime work rather than low-level protocol forensics.

## The Routing Mental Model

Routing is the bridge answer to:

- "which compute targets are affected by this truth change?"

The bridge should make that answer:

- deterministic
- explicit
- diagnostics-backed
- replay-safe

In ordinary code, that begins with one call:

```rust
let route = bridge.route(crate::facade::TruthCommitIdentity::new("commit:steel-main"))?;
```

That route is the canonical bridge view of the truth change, not just a casual
notification.

## How Routing Fits Into The Product Story

Routing should be thought of in three layers:

1. truth identifies what changed
2. bridge maps that change into invalidation targets
3. compute evaluates the affected targets against a truth view

The bridge does not own the truth semantics or the compute semantics.
It owns the transfer contract.

## Deterministic Mapping

Routing depends on the mapping registry you register at build time.

At a high level, each mapping says:

- when this truth scope changes
- invalidate this compute scope

Example:

```rust
use forge_runtime_bridge::facade::{
    BridgeMappingId, BridgeMappingRegistration, CoarseRoutingMode, MappingSelector,
    SignalInvalidationScope, TruthPatchScope,
};

let bridge = RuntimeBridge::builder()
    .with_truth_source(relational_source.clone())
    .with_truth_branch_head_source(relational_source)
    .with_compute_sink(signal_sink)
    .register_mapping(BridgeMappingRegistration::new(
        BridgeMappingId::new("pricing:steel"),
        TruthPatchScope::new(
            MappingSelector::exact("component:steel"),
            MappingSelector::exact("cost"),
            MappingSelector::exact("usd"),
        ),
        SignalInvalidationScope::new("price:bicycle"),
        CoarseRoutingMode::Direct,
    ))
    .build()?;
```

The important product rule is that the mapping registry should produce the same
routing meaning for the same truth change every time.

## Fanout Is A First-Class Bridge Story

One truth change may affect many compute targets.

That is not an edge case.
It is part of the bridge's normal job.

In the pricing-shock reference workload, one shared component like `steel` can
invalidate many final-price targets at once.

The bridge should preserve:

- exact scope matching when available
- safe fallback behavior when configured
- deterministic fanout when multiple specific targets legitimately match

## Evaluating The Current Result

After a route, the most common next question is:

- "what does the compute side see now for this target?"

That is the role of `evaluate_current(...)`:

```rust
let route = bridge.route(crate::facade::TruthCommitIdentity::new("commit:steel-main"))?;
let evaluation = bridge.evaluate_current(route.target())?;
```

This is the easiest way to evaluate using the bridge's default current truth
view.

## Evaluating An Explicit Truth View

Use `evaluate(...)` when the truth basis matters explicitly.

That includes jobs like:

- inspect the head of a branch
- inspect a specific snapshot
- inspect historical commit-bound truth

The bridge provides request constructors for those cases:

```rust
use forge_runtime_bridge::facade::{
    BridgeTruthViewEvaluationRequest, TruthBranchIdentity, TruthCommitIdentity,
    TruthSnapshotIdentity,
};

let branch_eval = bridge.evaluate(
    BridgeTruthViewEvaluationRequest::for_branch_head(TruthBranchIdentity::new("main")),
)?;

let snapshot_eval = bridge.evaluate(
    BridgeTruthViewEvaluationRequest::for_branch_snapshot(
        TruthBranchIdentity::new("pricing-main"),
        TruthSnapshotIdentity::new("snapshot:pricing-main"),
    ),
)?;

let historical_eval = bridge.evaluate(
    BridgeTruthViewEvaluationRequest::for_historical_commit(
        TruthBranchIdentity::new("main"),
        TruthCommitIdentity::new("commit:steel-main"),
    ),
)?;
```

This is how the bridge keeps truth-view selection explicit without making the
ordinary current-view path heavy.

## Branch-Local Evaluation

Branch-local evaluation matters whenever you are doing speculative or preview
work.

The key point is that:

- branch-local truth view is part of bridge semantics

It is not just a UI convenience.
It is how the bridge preserves causal isolation while still making comparisons
possible.

## Diagnostics For Routing And Evaluation

Diagnostics should be attached to these jobs directly:

```rust
let route = bridge.route(crate::facade::TruthCommitIdentity::new("commit:steel-main"))?;
let evaluation = bridge.evaluate_current(route.target())?;

let diagnostics = bridge.diagnostics();
let route_explanation = diagnostics.explain_last_route();
let evaluation_explanation = diagnostics.explain_last_evaluation();
```

That is the normal path for answering questions like:

- why did this target get invalidated?
- which truth basis was evaluated?
- was this current, branch-head, snapshot, or historical?

## Routing Under Load

Milestone 13 deliberately pressure-tests routing with:

- high-fanout shared inputs
- repeated live updates
- main-branch churn during speculative branch work

The point of those tests is not just speed.
It is semantic stability:

- the same truth change should keep the same routing meaning
- the same truth-view request should keep the same evaluation meaning
- diagnostics-tier differences should not change causal meaning

## Everyday Rule Of Thumb

If the job is:

- route a normal truth change
- evaluate current result
- evaluate an explicit branch or historical truth view

stay in the standard path:

- `route(...)`
- `evaluate_current(...)`
- `evaluate(...)`
- `diagnostics().explain_*()`

If you find yourself reaching for phase-by-phase protocol verbs just to do
ordinary routing or evaluation, you have likely drifted below the intended
surface.

## Common Pitfalls

- Treating routing as a best-effort notification instead of a canonical bridge
  decision.
- Forgetting that fanout is normal and trying to force one truth change into
  one target mentally.
- Using explicit historical or snapshot requests when the ordinary current-view
  path would have been simpler and clearer.
- Assuming diagnostics-tier changes are allowed to change routing or evaluation
  meaning. They are not.
