# worth-runtime-bridge

`worth-runtime-bridge` is the causal protocol boundary between
`worth-relational` truth and `worth-signal` computation.

It exists to do one job well:

- take authoritative truth changes
- route them deterministically into compute invalidation
- evaluate against explicit truth views
- support speculative branch-local work without contaminating main
- retain replayable diagnostics and certification evidence

If `worth-relational` owns truth and `worth-signal` owns derived computation,
the bridge owns the boundary between them.

## What It Should Feel Like

The everyday bridge path should be boringly obvious:

- build
- route
- evaluate
- speculate
- discard or promote
- inspect

For ordinary usage, start with [`worth_runtime_bridge::facade`].

## First Example

```rust
use worth_runtime_bridge::facade::RuntimeBridge;

fn demo(
    truth_source: impl worth_runtime_bridge::facade::RelationalBridgeSource + Clone + 'static,
    branch_heads: impl worth_runtime_bridge::facade::TruthBranchHeadSource + Clone + 'static,
    compute_sink: impl worth_runtime_bridge::facade::SignalBridgeSink + Clone + 'static,
) -> Result<(), Box<dyn std::error::Error>> {
    let bridge = RuntimeBridge::builder()
        .with_truth_source(truth_source)
        .with_truth_branch_head_source(branch_heads)
        .with_compute_sink(compute_sink)
        .register_mapping(worth_runtime_bridge::facade::BridgeMappingRegistration::new(
            worth_runtime_bridge::facade::BridgeMappingId::new("pricing:steel"),
            worth_runtime_bridge::facade::TruthPatchScope::new(
                worth_runtime_bridge::facade::MappingSelector::exact("component:steel"),
                worth_runtime_bridge::facade::MappingSelector::exact("cost"),
                worth_runtime_bridge::facade::MappingSelector::exact("usd"),
            ),
            worth_runtime_bridge::facade::SignalInvalidationScope::new("price:bicycle"),
            worth_runtime_bridge::facade::CoarseRoutingMode::Direct,
        ))
        .build()?;

    let route = bridge.route(crate::facade::TruthCommitIdentity::new("commit:steel-main"))?;
    let evaluation = bridge.evaluate_current(route.target())?;
    let explanation = bridge.diagnostics().explain_last_route();

    let _ = evaluation;
    let _ = explanation;
    Ok(())
}
```

That example is intentionally small.
The important part is the shape:

- one builder
- one route call
- one evaluation call
- one diagnostics door

## Standard Path

Use [`worth_runtime_bridge::facade`] when you want the canonical public
workflow:

- `RuntimeBridge::builder()`
- `bridge.route(...)`
- `bridge.evaluate_current(...)`
- `bridge.evaluate(...)`
- `bridge.speculate(...)`
- `session.discard(...)`
- `session.promote(...)`
- `bridge.diagnostics().explain_*()`

That single facade also exposes the deeper explicit-control and retained-proof
surfaces when the job genuinely requires them, but callers stay on the same
import path.

## What The Bridge Owns

The bridge owns:

- deterministic truth-change routing
- explicit truth-view evaluation
- branch-local speculative sessions
- discard versus promotion boundaries
- replayable diagnostics and certification evidence

The bridge does not own:

- relational truth semantics
- signal compute semantics
- arbitrary host policy outside the bridge contract

## Next Steps

- Read [`QUICKSTART.md`](./QUICKSTART.md) for the smallest honest setup path.
- Use `worth_runtime_bridge::facade` as the public import path for new code.
- Start with the standard-path methods on that facade, and only then reach for
  deeper explicit-control or retained-proof types when the job actually
  requires them.

## Documentation Map

Getting started:

- [`QUICKSTART.md`](./QUICKSTART.md)
- [`DAILY_WORKFLOWS.md`](./DAILY_WORKFLOWS.md)
- [`API_OVERVIEW.md`](./API_OVERVIEW.md)
- [`DIAGNOSTICS.md`](./DIAGNOSTICS.md)
- [`REFERENCE_MAP.md`](./REFERENCE_MAP.md)

Core concepts:

- [`ROUTING_AND_EVALUATION.md`](./ROUTING_AND_EVALUATION.md)
- [`BRANCHING_AND_SPECULATION.md`](./BRANCHING_AND_SPECULATION.md)
- [`WRITEBACK_AND_PROMOTION.md`](./WRITEBACK_AND_PROMOTION.md)
- [`HISTORY_AND_REPLAY.md`](./HISTORY_AND_REPLAY.md)

Advanced integration and trust:

- [`RUNTIME_POLICY.md`](./RUNTIME_POLICY.md)
- [`CHANGE_STREAMS_AND_SOURCES.md`](./CHANGE_STREAMS_AND_SOURCES.md)
- [`MAPPING_CONTINUITY_AND_REMAP.md`](./MAPPING_CONTINUITY_AND_REMAP.md)
- [`MERGE_AND_STRUCTURAL_COMPARISON.md`](./MERGE_AND_STRUCTURAL_COMPARISON.md)
- [`CERTIFICATION_AND_HARNESS.md`](./CERTIFICATION_AND_HARNESS.md)
- [`CAUSAL_BUNDLES_AND_GUARANTEES.md`](./CAUSAL_BUNDLES_AND_GUARANTEES.md)
- [`HOST_ADAPTERS.md`](./HOST_ADAPTERS.md)
