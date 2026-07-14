# Runtime Policy

`RuntimePolicy` controls how light or detailed the runtime feels.

It decides how much detail the runtime keeps and how much extra work it does
for debugging, replay, and later inspection.

## Presets

Most people should start with one of these:

- `RuntimePolicy::operational()`
- `RuntimePolicy::development()`
- `RuntimePolicy::forensic()`

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
- runtime path: `runtime.adjust_runtime_policy(...)`
- checkpoint tuning: `.adjust_checkpoints(...)` or `runtime.adjust_checkpoint_policy(...)`
- comparator fallback tuning: `.adjust_fallback_comparator(...)`

## One Important Rule

`set_runtime_policy(...)` is the real owner once you move past stock presets.

`reset_runtime_policy_to_tier(...)` is the convenience reset back to a named
tier preset.

`set_diagnostics_profile(...)` is only the older transitional name for that
same reset behavior.

Either way, the reset replaces the full policy bundle. If you already tuned
retention, replay, or history limits, use `set_runtime_policy(...)` instead.
