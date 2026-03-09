# Quick Start: Fintech

Use the fintech preset when replay fidelity, deterministic auditability, and rich investigation data matter more than absolute minimum runtime overhead.

## Recommended configuration

```rust
use forge_signal::facade::{
    SignalDeploymentPreset, SignalGraph, SignalRuntime,
};

let graph = SignalGraph::new();
let plan = SignalDeploymentPreset::Fintech.recommended();
let runtime = SignalRuntime::builder(graph)
    .runtime_policy(plan.runtime_policy)
    .build();
let executor = plan.executor;
```

## Why this preset

- `Development`-rich runtime policy
- stronger replay detail for audit and reconstruction
- conservative deterministic parallel admission
- explicit retained vs reconstructed artifact semantics

## Recommended local certification

```bash
bash scripts/ci/run_signal_local_certification.sh fintech
```

## When to move richer

- If you need maximal retained provenance during incident response, switch to `SignalRuntimePolicy::kernel()`.
- If normal production traffic is paying too much observability overhead, keep the fintech preset for certification and run `Operational` in the hottest path while retaining replay.
