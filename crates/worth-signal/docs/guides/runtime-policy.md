# Runtime Policy

`SignalRuntimePolicy` controls the runtime's execution objective, observation
activation, retention, and diagnostic behavior.

It decides both what the runtime is optimizing for and how much optional work
it is allowed to do for debugging, replay, and later inspection. The shared
meaning for objective and activation comes from `worth-foundational`; Signal
owns execution and session lifecycle.

## Presets

Most people should start with one of these:

- `SignalRuntimePolicy::operational()` — public Throughput + OnDemand
  production posture
- `SignalRuntimePolicy::development()`
- `SignalRuntimePolicy::forensic()`

If you are already on the builder path and just want a stock preset, you can
use the named builder helpers too:

- `.operational_policy()`
- `.development_policy()`
- `.forensic_policy()`
- `.web_development_policy()`
- `.fintech_policy()`

Use:

- `operational()` for lower production overhead
- `development()` as the normal default while building
- `forensic()` when keeping more detail matters more than cost

## What policy affects

Policy mostly controls:

- execution objective (`LatencyBounded`, `Balanced`, or `Throughput`)
- observation activation (`Continuous` or `OnDemand`)

- how much explanation and history detail gets kept
- how much replay and run history gets kept
- failure and rollback detail
- parallel admission thresholds

## Common overrides

Use overrides when a preset is close but not quite right:

- `.with_explanation_retention(...)`
- `.with_provenance_retention(...)`
- `.with_replay_detail(...)`
- `.with_semantic_retention(...)`
- `.with_parallel_admission(...)`
- `.with_history_limit(...)`
- `.with_detail_limit(...)`
- `.with_history_details(...)`

If you want to adjust a preset instead of rebuilding it from scratch, use:

- builder path: `.adjust_runtime_policy(...)`
- runtime path: `runtime.try_adjust_runtime_policy(...)` (typed denial)
- known-valid runtime convenience: `runtime.adjust_runtime_policy(...)`
- checkpoint tuning: `.adjust_checkpoints(...)` or `runtime.adjust_checkpoint_policy(...)`
- comparator fallback tuning: `.adjust_fallback_comparator(...)`

## One Important Rule

`set_runtime_policy(...)` is the real owner once you move past stock presets.

The governed progression is compiler-visible:

```text
SignalRuntimePolicyRequest
    -> AdmittedSignalRuntimePolicy
    -> ResolvedSignalRuntimePolicy
    -> InstalledSignalRuntimePolicy
```

Planner and execution code consume the installed/resolved policy. A raw request
is not execution authority. `Throughput` is an objective, not a correctness or
durability downgrade; later observation-session phases decide when optional
counters, diagnostic facts, lineage, provenance, and replay sidecars are
actually admitted.

`reset_runtime_policy_to_tier(...)` is the convenience reset back to a named
tier preset.

New integration code should use `try_set_runtime_policy(...)` or
`try_adjust_runtime_policy(...)`, handle its typed admission denial, and never
bypass the compiler. The corresponding
`set_runtime_policy(...)` and `adjust_runtime_policy(...)` methods are
infallible conveniences for callers that already hold a known-valid preset or
compiled configuration.

Objective and activation are independent axes. `operational()` uses
`OnDemand`, but callers may select `Continuous` observation explicitly without
changing the Throughput execution objective.

Either way, the reset replaces the full policy bundle. If you already tuned
retention, replay, or history limits, use `set_runtime_policy(...)` instead.
