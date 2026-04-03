# Forge Relational Phase 4 Hotspots And Runtime Endurance

This memo now captures the end-state of Phase 4 rather than its kickoff assumptions. It translates the certified matrix into:

- a current hotspot list
- a plain-language endurance read for intense workloads
- the remaining hardening questions before the AoSoA decision gate

This document is grounded in the committed baseline in [forge_relational_performance_baseline.jsonl](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/engineering/forge_relational_performance_baseline.jsonl).

## What Phase 3 Told Us

Phase 3 did not reveal a hidden bridge-scaling disaster.

- Narrow mock bridge wave:
  - `runtime_bridge_mock_matrix/geometry_commit_bridge_wave_operational`
  - median elapsed `67us`
  - `affected_bridge_sources = 3`
  - `bridge_nodes_recomputed = 10`
- Medium bridge region:
  - `runtime_bridge_mock_matrix/geometry_commit_bridge_wave_medium_region_operational`
  - median elapsed `122us`
  - `affected_bridge_sources = 16`
  - `bridge_nodes_recomputed = 49`
- Mixed locality bridge wave:
  - `runtime_bridge_mock_matrix/geometry_commit_bridge_wave_mixed_locality_operational`
  - median elapsed `167us`
  - `affected_bridge_sources = 15`
  - `bridge_tasks_scheduled = 31`

The bridge seam is therefore not the primary blocker going into Phase 6. The remaining walls are inside the large domain workloads themselves.

## Ranked Hotspots

These are the highest-value remaining hotspots, ordered by practical impact.

1. 100k-world bootstrap cost
- `rocketship_scale_matrix/hundred_k_nodes_pseudorealistic_propagation_wave`
- `bootstrap_entity_commit_micros = 1500878`
- `bootstrap_relation_commit_micros = 2609789`
- total initial world seeding is still roughly `4.1s`

Why it matters:
- the resident-world hot path is now much healthier than bootstrap
- repeated rebuild-from-scratch workflows are still materially more expensive than steady-state work

2. Touched-partition clone and publication width on 100k-world geometry edits
- `rocketship_scale_matrix/hundred_k_nodes_pseudorealistic_propagation_wave`
- `hot_update_micros = 16655`
- `phase_timing.draft_preparation_micros = 2059`
- `phase_timing.publication_storage_commit_micros = 846`

Why it matters:
- propagation and explicit reads are no longer the scary part
- the remaining hot-path wall is mostly overlay/publish cost inside the commit pipeline

3. Remaining rich-vs-thin geometry spread at large scale
- `rocketship_scale_matrix/hundred_k_nodes_geometry_profile_propagation_wave`
- `hot_update_micros = 16126`
- `propagation_execution_micros = 209`
- `explicit_query_micros = 53`
- `publication_micros = 943`

Why it matters:
- Phase 1 successfully removed the catastrophic trace flood
- rich geometry is now close enough to the thin lane that the remaining gap is mostly real publication/layout work, not observability spam

4. Geometry durability and checkpoint tail
- `hot_cold_path_matrix/geometry_rich_publication_hot_vs_replay_truth`
- `hot_commit_micros = 536`
- `durable_append_micros = 3964`
- `checkpoint_micros = 1666`

Why it matters:
- the remaining expensive geometry tail is now genuine durability and recovery work
- this is the right kind of cost to weigh in the AoSoA gate instead of blaming diagnostics

5. Chip recovery/checkpoint overhead, not compile cost
- `hot_cold_path_matrix/chip_hot_compile_vs_recovery_compile`
- `hot_compile_micros = 7`
- `hot_commit_micros = 714`
- `checkpoint_micros = 1412`
- `recover_micros = 268`

Why it matters:
- chip-style inner compile work is not the current problem
- checkpoint/recovery cadence and commit shape are more important than compile itself

6. Longer-horizon replay-window endurance beyond the current proof window
- `sustained_load_matrix/replay_window_drift_stability`
- `average_replay_micros = 4112`
- `max_replay_micros = 8603`
- `replayed_commit_count = 32`

Why it matters:
- replay is healthy enough for bounded windows
- we still have not certified marathon-scale replay windows

## Runtime Endurance In Plain Terms

This section translates the current numbers into operational language.

## Comfortable Right Now

These are workloads I would consider supportable today with good confidence.

- Localized geometry edits and bounded topology waves
  - Example evidence: `geometry_kernel_matrix/topology_bridge_connectivity_wave_zero_diagnostics`
  - median `59us`
- Chip stepping with local fanout and repeated branch/savepoint use
  - Example evidence: `chip_simulator_matrix/branch_rollback_compile_step_window`
  - median `66us`
- Sustained localized mixed topology churn over dozens of iterations
  - Example evidence: `sustained_load_matrix/mixed_topology_query_churn_stability`
  - total `4936us` over repeated updates and queries
  - `max_packets_per_iteration = 3`
  - `max_scope_units_per_iteration = 3`

Plain English:
- the runtime is already comfortable doing many small, region-local truth updates in a row
- it is not showing an immediate drift cliff in these certified windows

## Caution But Plausible Right Now

These are workloads the runtime can clearly do, but where the current numbers say we should harden further before making very strong promises.

- 100k-node resident geometry worlds with realistic subsystem propagation
  - hot update about `16-17ms`
  - propagation about `210us`
  - explicit read about `58us`
- medium-width bridge invalidation regions
  - `16` affected bridge sources
  - `49` recomputed nodes
  - still region-local, but noticeably wider than the narrow case
- chip event-wave churn windows
  - `chip_simulator_matrix/event_wave_compile_churn_window`
  - total `1265us` across the certified window
  - average update `48us`

Plain English:
- this is good enough for serious heavyweight engineering interaction
- it is much closer to relentless iterative work than it was at the start of Phase 4
- the remaining concern is long-horizon cadence and bootstrap, not single-wave propagation explosions

## Not Yet Proven

These are the cases I would not claim confidently yet.

- hour-plus continuous 100k-node broad-wave geometry execution
- game-engine-like frame loops much wider than the new bounded scene-graph windows
- chip-simulator-grade long-running global stepping over much denser fanout than the current certified windows
- large replay windows far beyond the current `32`-commit drift certification

Plain English:
- the runtime does not look like it falls over quickly
- but we have only certified intense runs over bounded windows, not marathon-scale operation

## How Long Can We Run Before It Stops Being Supportable?

The honest answer is:

- there is no current evidence of a short-run collapse
- there is also not yet enough evidence to claim indefinite heavy execution at the hardest scales

Translated into operational terms:

- For localized intense work:
  - supportable now
  - the current drift tests suggest many repeated operations without structural degradation in the certified windows
- For 100k-node broad-wave work:
  - supportable now as repeated heavy operations and bounded endurance windows
  - `rocketship_hot_update_endurance` stayed stable over `256` hot updates
  - `average_update_micros = 1677`
  - `first_window_average_update_micros = 1943`
  - `last_window_average_update_micros = 1776`
  - `rocketship_propagation_endurance` stayed stable over `96` full update-plus-propagation-plus-query cycles
  - `average_update_micros = 3361`
  - `average_propagation_micros = 205`
  - `average_explicit_query_micros = 52`
  - `first_window_average_cycle_micros = 9173`
  - `last_window_average_cycle_micros = 2608`
- For frame-shaped interactive work:
  - supportable now on bounded scene-graph windows
  - `game_engine_matrix/local_scene_graph_propagation_wave` certifies a local frame-like update at `525us` median
  - `game_engine_matrix/mixed_read_write_frame_churn_window` certifies `48` mixed update-plus-propagation-plus-query cycles with:
  - `average_update_micros = 367`
  - `average_propagation_micros = 40`
  - `average_explicit_query_micros = 12`
  - `first_window_average_cycle_micros = 427`
  - `last_window_average_cycle_micros = 421`
- For denser chip stepping:
  - `sustained_load_matrix/chip_global_step_endurance` certifies `128` repeated steps with:
  - `average_update_micros = 216`
  - `average_compile_micros = 2`
  - `first_window_average_cycle_micros = 172`
  - `last_window_average_cycle_micros = 265`
- For hardest-task chip or geometry workloads:
  - likely viable only if the hot loop stays region-local and the system checkpoints intelligently
  - not yet proven for continuous globally wide execution

So the current supportability wall is not obviously memory exhaustion or some immediate runtime collapse. It is more likely:

- bootstrap/import throughput under large worlds
- recovery/checkpoint cadence cost
- unproven long-horizon drift at scale

## Practical Translation

If you asked me to summarize the current state in simple terms:

- We are already good at sustained local work.
- We are now convincingly good at repeated heavy interactive 100k-node work over bounded windows.
- We are not yet certified for marathon-scale globally broad workloads.

That means the remaining questions before the AoSoA gate are:

1. Is touched-partition clone and publication width still the main structural wall after all the pre-AoSoA hardening?
2. Is bootstrap/import throughput important enough to justify separate bulk-loading work before layout changes?
3. Do we need longer-horizon endurance evidence before we trust a layout decision?

## Remaining Phase 4 Hardening Order

The remaining order is:

1. Bootstrap throughput hardening
- focus on large entity and relation seed/import cost
- steady-state 100k work is now much healthier than world construction

2. Longer-horizon endurance lanes
- extend the current `256` and `96` iteration rocketship windows further
- this answers the “how long can we keep going?” question with stronger evidence

3. Chip denser fanout and longer step windows
- compile cost is already good
- the next question is uglier global stepping and checkpoint cadence beyond the new bounded endurance case

4. Broader game-engine frame-shaped workloads
- the first frame-shaped workload gap is now covered
- the next question is much wider scene churn and larger frame regions

## What Success Looks Like

Phase 4 has effectively done its job when:

- the hotspot list is much narrower and more domain-specific than it was at kickoff
- we have endurance evidence that covers real repeated heavy-work windows instead of only burst snapshots
- we know that the next real wall is bootstrap/import throughput, durability cadence, or layout pressure rather than generic query execution

Current read:

- the hotspot list is now much narrower than it was at the start of Phase 4
- the steady-state 100k geometry path is no longer the primary reason to hesitate
- the remaining open questions are bootstrap/import throughput, longer-horizon endurance, and whether touched-partition clone cost is enough to justify AoSoA later
