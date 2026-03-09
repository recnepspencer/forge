# Quick Start: Web Development

Use the web-development preset when request/interaction latency and small incremental updates matter more than maximum retained forensic detail.

## Recommended configuration

```rust
use forge_signal::facade::{
    SignalDeploymentPreset, SignalGraph, SignalRuntime,
};

let graph = SignalGraph::new();
let plan = SignalDeploymentPreset::WebDevelopment.recommended();
let runtime = SignalRuntime::builder(graph)
    .runtime_policy(plan.runtime_policy)
    .build();
let executor = plan.executor;
```

## Why this preset

- cheap operational semantics by default
- conservative parallel admission for small request-driven workloads
- replay and stable semantic IDs remain authoritative
- explanation/provenance can be reconstructed when needed instead of always being retained

## Recommended local certification

```bash
bash scripts/ci/run_signal_local_certification.sh web
```

## Practical guidance

- Start here unless you already know you need richer retained diagnostics.
- If a route or invalidation wave becomes compute-heavy, the executor can still admit parallelism when the work is worth it.
