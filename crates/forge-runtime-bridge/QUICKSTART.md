# Quickstart

This is the essentials page for `forge-runtime-bridge`.

If you remember only one mental model, remember this:

- build
- route
- evaluate
- speculate
- inspect

The goal is to get you to first success quickly, not to explain every bridge
subsystem.

## The Smallest Mental Model

The bridge sits between:

- `forge-relational` truth
- `forge-signal` computation

Its standard path is:

```rust
let bridge = RuntimeBridge::builder()
    .with_truth_source(relational_source)
    .with_compute_sink(signal_sink)
    .build()?;

let route = bridge.route(change)?;
let evaluation = bridge.evaluate_current(route.target())?;
let diagnostics = bridge.diagnostics().explain_last();
```

Everything else in this guide is just the honest minimum needed to make that
shape real.

## 1. Import The Facade

```rust
use forge_runtime_bridge::facade::{
    BridgeMappingId, BridgeMappingRegistration, BridgeTruthViewEvaluationRequest,
    CoarseRoutingMode, MappingSelector, RuntimeBridge, SignalInvalidationScope,
    TruthBranchIdentity, TruthPatchScope,
};
```

Use `forge_runtime_bridge::facade` for bridge work.
Start with the standard-path methods on that facade. Only drop to the deeper
explicit-control or retained-proof types when the job actually calls for them.

## 2. Fastest Real Example

If you already have a bound truth source, branch-head source, compute sink, and
at least one mapping, the standard path is:

```rust
let route = bridge.route(crate::facade::TruthCommitIdentity::new("commit:steel-main"))?;
let evaluation = bridge.evaluate_current(route.target())?;
let diagnostics = bridge.diagnostics().explain_last();
```

That is the bridge equivalent of Angular's "read, update, observe" signals
story: one small loop that establishes the product.

## 3. Build A Bridge

```rust
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

Normal setup should feel like:

- bind truth
- bind branch-head reads
- bind compute
- register mappings
- build

## 4. Route A Truth Change

```rust
let route = bridge.route(crate::facade::TruthCommitIdentity::new("commit:steel-main"))?;
```

For ordinary work, `route(...)` is the front door.
You should not need to manually orchestrate ingest, plan, delivery, and
preparation phases just to route one truth change.

## 5. Evaluate Against The Current Truth View

```rust
let evaluation = bridge.evaluate_current(route.target())?;
```

This is the default "show me the current bridge-visible result" path.

If you need a specific truth view, use `evaluate(...)` instead:

```rust
let branch_eval = bridge.evaluate(
    BridgeTruthViewEvaluationRequest::for_branch_head(TruthBranchIdentity::new("main")),
)?;
```

## 6. Open A Speculative Session

```rust
let session = bridge.speculate(spec_request)?;
let comparison = session.compare_to_main();
```

The standard path treats speculation as a session, not as a pile of ids and
phase calls.

If you need explicit truth-view reads for both sides, the comparison object can
hand you the requests:

```rust
let main_eval = bridge.evaluate(
    comparison.main_evaluation_request(TruthBranchIdentity::new("main")),
)?;
let speculative_eval = bridge.evaluate(
    comparison.speculative_evaluation_request(),
)?;
```

## 7. Discard Or Promote

Discard:

```rust
let discarded = session.discard(vec![
    forge_runtime_bridge::facade::BridgePreviewResidueClass::PreviewExecutionRetained,
    forge_runtime_bridge::facade::BridgePreviewResidueClass::ReplayRetainedNonAuthoritative,
])?;
```

Promote:

```rust
let promoted = session.promote()?;
```

The important part is the boundary:

- discard stays non-authoritative
- promotion is explicit and derives authority from the active preview proof

## 8. Inspect What Happened

```rust
let diagnostics = bridge.diagnostics();

let route_explanation = diagnostics.explain_last_route();
let evaluation_explanation = diagnostics.explain_last_evaluation();
let session_explanation = diagnostics.explain_session("pricing:preview-promote");
```

Start from the diagnostics door.
Only drop into raw record-family queries if you are doing specialist replay or
forensics on purpose.

## Common Pitfalls

- Reaching for low-level proof or control types too early. Start with the
  standard-path facade methods unless the job truly needs explicit policy,
  stream, structural, or replay control.
- Thinking of speculation as ambient mode. It is a session with explicit
  compare, discard, and promote boundaries.
- Treating diagnostics as optional support output. Diagnostics are part of the
  bridge contract and should be part of normal usage.
- Treating `evaluate(...)` as the default path. Use `evaluate_current(...)`
  first and only move to explicit truth-view requests when the basis matters.

## Everyday Checklist

If your normal bridge code is getting longer than this mental model, pause and
check whether you accidentally drifted into specialist surfaces:

- `builder`
- `route`
- `evaluate`
- `speculate`
- `discard` or `promote`
- `diagnostics`

That is the intended everyday memory shape.
