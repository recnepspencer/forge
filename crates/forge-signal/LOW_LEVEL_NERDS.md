# LOW_LEVEL_NERDS.md

This file is for the people who see a default setting and immediately think:

"Interesting. But what if I changed eight of them at once?"

That instinct is sometimes valuable and sometimes how you end up benchmarking a self-inflicted problem for three days.

## The knobs you are actually allowed to touch

### 1. Runtime policy richness

Main surface:

- `SignalRuntimePolicy::operational()`
- `SignalRuntimePolicy::development()`
- `SignalRuntimePolicy::forensic()`
- `SignalRuntimePolicy::web_development()`
- `SignalRuntimePolicy::game_engine()`
- `SignalRuntimePolicy::fintech()`
- `SignalRuntimePolicy::kernel()`

Useful overrides:

- `.with_explanation_retention(...)`
- `.with_provenance_retention(...)`
- `.with_replay_detail(...)`
- `.with_semantic_retention(...)`
- `.with_parallel_admission(...)`

What this really means:

- `Operational`: cheap truth, reconstruct richer artifacts later
- `Development`: retain richer artifacts now
- `Forensic`: keep more stuff because Future You is angry and wants receipts

### 2. Executor policy

Main surface:

- `StageExecutor::Serial`
- `StageExecutor::parallel(min_stage_width)`
- `StageExecutor::full_parallel(min_stage_width)`
- `.with_parallel_policy(ParallelExecutionPolicy::new(...).with_worker_count(...).with_chunk_size(...).with_apply_group_min_width(...).with_max_concurrent_apply_groups(...))`

What the lower-level parameters do:

- `min_stage_width`: don’t even consider parallelism below this width
- `worker_count`: how many worker lanes you want the executor to use
- `chunk_size`: how coarse precompute partitioning should be
- `apply_group_min_width`: how wide a conflict-free group should be before grouped concurrent apply is worth attempting
- `max_concurrent_apply_groups`: cap on concurrent apply waves

If you are not sure:

- start with a deployment preset
- run certification
- only then get clever

### 3. Core storage profile

Build-time features:

- default: `profile-standard`
- `profile-compact`
- `profile-extended`

What changes with profile:

- `MAX_ASPECTS`
- `AspectMaskBits`
- `HOT_VEC_INLINE_CAPACITY`
- `StableHashValue`
- `STABLE_HASH_WIDTH_BITS`

This is not a runtime knob. It is a build profile.

That is intentional.

Per-runtime dynamic storage width sounds flexible right up until you want deterministic memory behavior, predictable serialization, and a certification story that does not smell like regret.

## Recommended ways to be a responsible low-level nerd

### Want more parallelism?

Do this:

1. start from `SignalDeploymentPreset::*`
2. inspect `stage.parallel_admission_reason`
3. inspect the perf report
4. lower thresholds deliberately

Do not do this:

1. set every threshold to `1`
2. declare victory
3. wonder why your synthetic benchmark got slower

### Want richer explanation/provenance?

Do this:

- switch to `Development` or `Forensic`
- or keep `Operational` and reconstruct explicitly with:
  - `graph.reconstruct_explanation_artifact(node)`
  - `graph.reconstruct_provenance_artifact(node)`

Do not do this:

- assume eager retention is free
- then act surprised when semantic finalization becomes your personality

### Want to certify the weird tuning you just invented?

Run:

```bash
bash scripts/ci/run_signal_local_certification.sh full
```

Or use a deployment-specific lane:

```bash
bash scripts/ci/run_signal_local_certification.sh web
bash scripts/ci/run_signal_local_certification.sh game-engine
bash scripts/ci/run_signal_local_certification.sh fintech
bash scripts/ci/run_signal_local_certification.sh kernel
```

## Things you should not "optimize" casually

- determinism contracts
- rollback semantics
- replay event truth
- retained vs reconstructed artifact semantics
- canonical ordering

If you break any of those, you are not doing performance work anymore. You are writing folklore.

## Final wisdom

There are only three valid reasons to touch the low-level presets:

1. you measured something
2. you are targeting a specific workload shape
3. you are willing to rerun certification after touching the knob

Anything else is just artisanal chaos.
