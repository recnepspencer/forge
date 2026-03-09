# Quick Start: Game Engines

Use the game-engine preset when frame-time stability matters and you want low-overhead operational diagnostics with parallelism available for broader recompute waves.

## Recommended configuration

```rust
use forge_signal::facade::{
    SignalDeploymentPreset, SignalGraph, SignalRuntime,
};

let graph = SignalGraph::new();
let plan = SignalDeploymentPreset::GameEngine.recommended();
let runtime = SignalRuntime::builder(graph)
    .runtime_policy(plan.runtime_policy)
    .build();
let executor = plan.executor;
```

## Why this preset

- operational-first runtime policy
- earlier staged/full parallel admission than web-oriented defaults
- replay stays authoritative while rich artifacts can be reconstructed on demand
- suitable for repeated frame-like invalidation waves

## Recommended local certification

```bash
bash scripts/ci/run_signal_local_certification.sh game-engine
```

## Practical guidance

- Keep day-to-day play/editor runs on the game-engine preset.
- Use `Development` or `Forensic` only when tracking down a hard determinism or rollback issue.
