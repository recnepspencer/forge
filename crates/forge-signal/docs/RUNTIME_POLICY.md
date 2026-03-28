# Runtime Policy

`SignalRuntimePolicy` controls how heavy or light the runtime feels.

It mainly decides how much detail the runtime keeps around and how much extra
work it does to support diagnostics, replay, and investigation.

## Presets

Most people should start with one of these:

- `SignalRuntimePolicy::operational()`
- `SignalRuntimePolicy::development()`
- `SignalRuntimePolicy::forensic()`

Use:

- `operational()` for leaner production behavior
- `development()` as the normal default while building
- `forensic()` when retained detail matters more than cost

There are also workload-shaped presets:

- `web_development()`
- `game_engine()`
- `fintech()`
- `kernel()`

## What policy affects

Policy mainly controls:

- how much explanation and provenance detail gets retained
- replay and history richness
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

## Rule of thumb

Start with `development()` unless you already know you need something lighter or
heavier.

If you are optimizing for production cost, move toward `operational()`.

If you are investigating correctness, replay, or hard runtime bugs, move toward
`forensic()`.
