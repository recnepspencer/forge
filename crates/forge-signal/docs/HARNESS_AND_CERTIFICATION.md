# Harness And Certification

This is the part of `forge-signal` you use when "it works on my machine" is not an acceptable quality bar.

## Scenario-driven harness usage

Main types:

- `SignalScenario`
- `SignalMutationBatch`
- `SignalHarnessAdapter`
- `signal_parity_suite(...)`
- `signal_bench(...)`

### Example: scenario with mutation batch

```rust
use forge_signal::facade::*;

let mut scenario = SignalScenario::new("demo");
let source = scenario.node("source");
let target = scenario.node("target");
scenario
    .dependency("target", "source", Aspect::new(0))?
    .with_evaluator(|node, view| {
        let result = if node == source {
            view.finish(NodeEvaluationResult::from_version(
                AspectVersion::from_updates([(Aspect::new(0), 1)]),
            ))
        } else {
            let version = view.read_aspect_version(source, Aspect::new(0))?;
            view.finish(NodeEvaluationResult::from_version(version))
        };
        Ok(result)
    });

let fixture = scenario
    .input("source")
    .observe("target")
    .fixture()?;

let mutation = SignalMutationBatch::new("dirty-source")
    .mark_dirty("source", Aspect::new(0))
    .build();
let request = ExecutionRequest::target("pull-target", "target".to_string());
# let _ = (fixture, mutation, request);
# Ok::<(), SignalError>(())
```

## What the harness captures that ordinary execution does not

- diagnostics summary records
- explanation records
- provenance records
- replay records
- artifact materialization mode
- runtime policy metadata
- core storage profile metadata
- stage admission reasons in run extensions

That makes it the right surface for CI and regression certification, not just for demos.

## Recommended local commands

Minimal docs-friendly lane:

```bash
bash scripts/ci/run_signal_local_certification.sh web
```

Stronger parallel/determinism lane:

```bash
bash scripts/ci/run_signal_local_certification.sh game-engine
```

Audit/perf-heavy lane:

```bash
bash scripts/ci/run_signal_local_certification.sh fintech
```

Everything:

```bash
bash scripts/ci/run_signal_local_certification.sh full
```

## Important certification scripts

- `bash scripts/ci/check_signal_core_profiles.sh`
- `bash scripts/ci/check_signal_failure_matrix.sh`
- `bash scripts/ci/check_signal_contract_matrix.sh`
- `bash scripts/ci/check_signal_resource_bounds.sh`
- `bash scripts/ci/check_signal_semantic_snapshots.sh "$DIR"`
- `bash scripts/ci/check_signal_parallel_determinism_cert.sh 2 "$DIR"`
- `bash scripts/ci/run_signal_perf_lane.sh`

## What each one proves

- `core_profiles`: build-profile assumptions are not accidentally hard-coded
- `failure_matrix`: rollback/failure paths do not leak semantic artifacts
- `contract_matrix`: the runtime’s major semantic promises each have a direct adversarial lane
- `resource_bounds`: churn, GC, retention, and restore loops stay bounded instead of quietly accumulating stale state
- `semantic_snapshots`: serial/staged/full-parallel stay byte-stable under each runtime policy
- `determinism_cert`: stability holds across repeated runs and hostile ignored loops
- `perf_lane`: phase costs and admission reasons are inspectable instead of guessed

The durable matrix that ties those scripts back to the signal vision lives in [_docs/engineering/forge_signal_adversarial_testing_matrix.md](/Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/engineering/forge_signal_adversarial_testing_matrix.md).
