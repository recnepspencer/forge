# Forge Relational Phase 4 Hotspots And Runtime Endurance

This memo starts Phase 4 by translating the current certified matrix into:

- a ranked hotspot list
- a plain-language endurance read for intense workloads
- the next domain hardening order

This document is grounded in the committed baseline in [forge_relational_performance_baseline.jsonl](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/engineering/forge_relational_performance_baseline.jsonl).

## What Phase 3 Told Us

Phase 3 did not reveal a hidden bridge-scaling disaster.

- Narrow mock bridge wave:
  - `runtime_bridge_mock_matrix/geometry_commit_bridge_wave_operational`
  - median elapsed `81us`
  - `affected_bridge_sources = 3`
  - `bridge_nodes_recomputed = 10`
- Medium bridge region:
  - `runtime_bridge_mock_matrix/geometry_commit_bridge_wave_medium_region_operational`
  - median elapsed `187us`
  - `affected_bridge_sources = 16`
  - `bridge_nodes_recomputed = 49`
- Mixed locality bridge wave:
  - `runtime_bridge_mock_matrix/geometry_commit_bridge_wave_mixed_locality_operational`
  - median elapsed `208us`
  - `affected_bridge_sources = 15`
  - `bridge_tasks_scheduled = 31`

The bridge seam is therefore not the primary Phase 4 bottleneck. The domain workloads are.

## Ranked Hotspots

These are the highest-value hotspots to attack next, ordered by practical impact.

1. 100k-world geometry query and propagation latency
- `rocketship_scale_matrix/hundred_k_nodes_pseudorealistic_propagation_wave`
- `propagation_execution_micros = 169629`
- `explicit_query_micros = 169661`
- `hot_update_micros = 127423`

Why it matters:
- once the 100k-node world is already resident, a single realistic propagation-plus-query cycle is still on the order of a few hundred milliseconds
- that is acceptable for heavyweight engineering interaction, but not yet for highly iterative kernel-style work

2. 100k-world geometry bootstrap cost
- `rocketship_scale_matrix/hundred_k_nodes_pseudorealistic_propagation_wave`
- `bootstrap_entity_commit_micros = 4203676`
- `bootstrap_relation_commit_micros = 4800771`
- total initial world seeding is roughly `9.0s`

Why it matters:
- long-lived sessions are fine
- repeated rebuild-from-scratch workflows are still too expensive

3. 100k-world rich geometry path still costs more than thin operational
- `rocketship_scale_matrix/hundred_k_nodes_geometry_profile_propagation_wave`
- `hot_update_micros = 141416`
- `propagation_execution_micros = 184255`
- `explicit_query_micros = 182222`

Why it matters:
- Phase 1 removed the catastrophic trace flood
- but rich geometry is still measurably slower than the thinner operational shape at large scale

4. Geometry hot truth durability tail
- `hot_cold_path_matrix/geometry_rich_publication_hot_vs_replay_truth`
- `hot_commit_micros = 667`
- `durable_append_micros = 573`
- `publication_micros = 18`

Why it matters:
- the remaining hot geometry cost is now mostly real truth/durability cost, not trace spam
- that is exactly the kind of wall Phase 4 should profile more deeply

5. Chip recovery/checkpoint overhead, not compile cost
- `hot_cold_path_matrix/chip_hot_compile_vs_recovery_compile`
- `hot_compile_micros = 7`
- `hot_commit_micros = 638`
- `checkpoint_micros = 1476`
- `recover_micros = 341`

Why it matters:
- chip-style inner compile work is not the current problem
- checkpoint/recovery cadence and commit shape are more important than compile itself

6. Replay-window endurance under sustained pressure
- `sustained_load_matrix/replay_window_drift_stability`
- `average_replay_micros = 4633`
- `max_replay_micros = 9309`
- `replayed_commit_count = 32`

Why it matters:
- replay is healthy enough for bounded windows
- we have not yet proven hour-scale or very large sliding windows

## Runtime Endurance In Plain Terms

This section translates the current numbers into operational language.

## Comfortable Right Now

These are workloads I would consider supportable today with good confidence.

- Localized geometry edits and bounded topology waves
  - Example evidence: `geometry_kernel_matrix/topology_bridge_connectivity_wave_zero_diagnostics`
  - median `78us`
- Chip stepping with local fanout and repeated branch/savepoint use
  - Example evidence: `chip_simulator_matrix/branch_rollback_compile_step_window`
  - median `74us`
- Sustained localized mixed topology churn over dozens of iterations
  - Example evidence: `sustained_load_matrix/mixed_topology_query_churn_stability`
  - total `9764us` over repeated updates and queries
  - `max_packets_per_iteration = 3`
  - `max_scope_units_per_iteration = 3`

Plain English:
- the runtime is already comfortable doing many small, region-local truth updates in a row
- it is not showing an immediate drift cliff in these certified windows

## Caution But Plausible Right Now

These are workloads the runtime can clearly do, but where the current numbers say we should harden further before making very strong promises.

- 100k-node resident geometry worlds with realistic subsystem propagation
  - hot update about `127ms`
  - propagation about `170ms`
  - explicit read about `170ms`
- medium-width bridge invalidation regions
  - `16` affected bridge sources
  - `49` recomputed nodes
  - still region-local, but noticeably wider than the narrow case
- chip event-wave churn windows
  - `chip_simulator_matrix/event_wave_compile_churn_window`
  - total `1729us` across the certified window
  - average update `67us`

Plain English:
- this is good enough for serious heavyweight engineering interaction
- it is not yet where we want it for relentless high-frequency kernel iteration

## Not Yet Proven

These are the cases I would not claim confidently yet.

- hour-plus continuous 100k-node broad-wave geometry execution
- game-engine-like frame loops with wide mixed read/write scene churn
- chip-simulator-grade long-running global stepping over much denser fanout than the current certified windows
- large replay windows far beyond the current `32`-commit drift certification

Plain English:
- the runtime does not look like it “falls over” quickly
- but we have only certified intense runs over dozens of iterations and bounded replay windows, not marathon-scale operation

## How Long Can We Run Before It Stops Being Supportable?

The honest answer is:

- there is no current evidence of a short-run collapse
- there is also not yet enough evidence to claim indefinite heavy execution at the hardest scales

Translated into operational terms:

- For localized intense work:
  - supportable now
  - the current drift tests suggest many repeated operations without structural degradation in the certified windows
- For 100k-node broad-wave work:
  - supportable as bursts and interactive heavy operations
  - not yet proven as an always-on, high-frequency, hour-scale loop
- For hardest-task chip or geometry workloads:
  - likely viable only if the hot loop stays region-local and the system checkpoints intelligently
  - not yet proven for continuous globally wide execution

So the current “supportability wall” is not obviously memory exhaustion or some immediate runtime collapse. It is more likely:

- throughput ceiling under broad-wave workloads
- recovery/checkpoint cadence cost
- unproven long-horizon drift at scale

## Practical Translation

If you asked me to summarize the current state in simple terms:

- We are already good at sustained local work.
- We are plausibly good at heavy interactive 100k-node work.
- We are not yet certified for marathon-scale globally broad workloads.

That means Phase 4 should aim to answer:

1. Can we keep 100k-node geometry broad-wave cycles comfortably below roughly `100-150ms` for the hot operational path?
2. Can we keep chip event-wave windows cheap when fanout and step windows get much larger?
3. Can we run those workloads for far longer than the current dozens-of-iterations proof windows without packet, scope, or replay drift?

## Phase 4 Hardening Order

The best order now is:

1. Geometry broad-wave hardening
- focus on 100k-node propagation and explicit read cost
- this is the clearest user-facing wall

2. Long-horizon endurance lanes
- extend sustained-load and replay-window certification far beyond current windows
- this answers the “how long can we keep going?” question with evidence instead of inference

3. Chip denser fanout and longer step windows
- compile cost is already good
- the next question is global stepping realism and checkpoint cadence

4. First game-engine frame-shaped workloads
- this fills the last major domain-shaped realism gap

## What Success Looks Like

Phase 4 will have done its job when:

- the hotspot list is narrower and more domain-specific
- we have endurance evidence that covers much longer high-pressure windows
- we know whether the next real wall is locality, durability cadence, or eventual layout pressure
