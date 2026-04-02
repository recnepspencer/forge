# Forge Relational Pre-Hardening Scope

This memo closes the last focused scoping pass before the systemic hardening plan.

It answers five questions:

1. which diagnostics are actually hot-path truth versus trace richness
2. what the current merged relational-plus-signal envelope looks like
3. whether the next wall is locality/layout or something else
4. where the runtime profile boundaries should land
5. whether the perf harness is charging us for measurement overhead

Primary sources:

- [performance_profiles.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/tests/performance_profiles.rs)
- [performance_support.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/tests/performance_support.rs)
- [mod.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/diagnostics/data/mod.rs)
- [policies.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/config/data/policies.rs)
- [forge_relational_performance_baseline.jsonl](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/engineering/forge_relational_performance_baseline.jsonl)

## Bottom Line

We do not need another broad discovery pass.

The remaining pre-plan scoping conclusions are now clear:

- the old dominant scale tax was rich geometry diagnostics/publication, not basic storage truth
- that hot-path trace flood has now been removed by explicit artifact policy enforcement
- merged relational-plus-signal execution is already narrow and healthy at small invalidation widths
- packetization and locality are no longer the first thing to optimize
- profile boundaries should become an explicit hot/cold policy, not just preset names
- profile-boundary drift is now measurable in the committed lane and currently clean against the promoted baseline
- the harness is mostly clean, but we still want one more hygiene sweep over hand-rolled timers

This memo still keeps the pre-Phase-1 diagnosis because it explains why the policy work mattered. The current post-Phase-1 state is:

- `rocketship_scale_matrix/hundred_k_nodes_geometry_profile_narrow_round_trip`
  - `hot_update_micros = 113304`
  - `diagnostic_artifact_count = 30`
  - `detailed_trace_entries = 0`
- `rocketship_scale_matrix/hundred_k_nodes_geometry_profile_propagation_wave`
  - `hot_update_micros = 153551`
  - `diagnostic_artifact_count = 32`
  - `detailed_trace_entries = 0`
- `geometry_artifact_decomposition_matrix/hundred_k_nodes_pseudorealistic_rich_artifact_classes`
  - `artifact_count_total = 32`
  - `artifact_kind_detailed_trace_count = 0`

## 1. Diagnostics Taxonomy

The public diagnostics surface is already rich enough to reason about policy directly.

From [mod.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/diagnostics/data/mod.rs):

- scopes: `Schema`, `Transaction`, `Snapshot`, `Retention`, `History`, `Replay`, `PatchPublication`, `Lineage`, `QueryPlanning`, `Invariant`
- artifact kinds: `MinimalSummary`, `DetailedTrace`, `Failure`, `Rollback`, `Comparison`

The relevant measured cases now give us a practical split.

### Must Be Hot

These carry truth or immediate boundary outcomes and should stay on the synchronous path:

- patch publication summaries
- replay compatibility and recovery summaries
- changed-record truth surfaces
- compiled artifact compatibility and record-count truth

Evidence:

- `recoverability_policy_matrix/geometry_hot_truth_vs_deferred_trace_policy`
  - `must_be_hot_changed_records = 1`
  - `reconstructable_summary_entries = 1`
  - `deferred_trace_entries = 0`
- `recoverability_policy_matrix/chip_compile_reconstructable_policy`
  - `must_be_hot_changed_records = 1`
  - `hot_compiled_record_count = 1`
  - `reconstructable_compiled_record_count = 1`

### Can Defer

These are valuable but not required to preserve immediate authoritative truth:

- detailed traces
- history-scope traces
- query-planning traces
- snapshot-path traces
- rich entry fanout beyond summary publication

Evidence:

- `artifact_recoverability_matrix/geometry_diagnostics_summary_vs_trace_recoverability`
  - `hot_total_artifacts = 18`
  - `hot_detailed_trace_artifact_count = 0`
  - `hot_detailed_trace_entry_count = 0`
  - `hot_summary_entry_count = 1`

### Reconstructable From Replay

These can be regenerated or validated later:

- geometry minimal summaries
- replay comparison surfaces
- chip compiled compatibility surfaces

Evidence:

- `artifact_recoverability_matrix/geometry_diagnostics_summary_vs_trace_recoverability`
  - `summary_digest_match = true`
  - `replay_mismatch_count = 0`
- `artifact_recoverability_matrix/chip_compiled_artifact_recoverability`
  - hot and cold compatibility both `Compatible`
  - hot and cold compiled record count both `1`

### Practical Policy Read

The right policy split is:

- `must-be-hot`: minimal truth summaries and compatibility surfaces
- `can-defer`: rich traces and most explanatory artifacts
- `reconstructable-from-replay`: summary parity and compiled-view parity checks

That is now a real config and publication policy during hardening, not just an interpretation of benchmark output.

## 2. Runtime Bridge Mock Envelopes

We now have a mocked bridge case inside `forge-relational`, not a real cross-crate dependency.

From `runtime_bridge_mock_matrix` in [forge_relational_performance_baseline.jsonl](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/engineering/forge_relational_performance_baseline.jsonl):

- `geometry_commit_bridge_wave_operational`
  - median elapsed `70us`
  - `relational_commit_micros = 51`
  - `relational_query_micros = 19`
  - `bridge_micros = 0`
  - `affected_bridge_sources = 3`
  - `bridge_nodes_recomputed = 10`
  - `bridge_tasks_scheduled = 7`
- `geometry_commit_bridge_wave_development`
  - median elapsed `78us`
  - `relational_commit_micros = 61`
  - `relational_query_micros = 17`
  - `bridge_micros = 0`
  - `affected_bridge_sources = 3`
  - `bridge_nodes_recomputed = 10`
  - `bridge_tasks_scheduled = 7`

What that means:

- the intended runtime-to-reactive handoff shape is already structurally narrow
- the crate boundary is now clean again because the bridge wave is mocked locally
- the future bridge crate still needs its own real certification lane before we treat these numbers as integration truth

What is still not fully scoped:

- larger invalidation regions
- merged locality pressure under broad downstream recomputation
- packet width versus recomputation width coupling

That is no longer a missing category. It is a next-depth expansion inside a covered category.

## 3. Locality And Layout Pressure

The current evidence does not say "layout first."

It says:

- packet fragmentation was real and improved
- locality still matters
- but the biggest current wall was rich geometry observability, and Phase 1 has now moved that flood off the synchronous path

Key evidence:

- `query_packet_matrix/connectivity_traversal_cross_partition`
  - median `127us`
  - packet count `3`
  - scope unit count `12`
- historical pre-policy geometry-rich state
  - `rocketship_scale_matrix/hundred_k_nodes_geometry_profile_narrow_round_trip`
    - `diagnostic_artifact_count` was about `403k`
    - `detailed_trace_entries` was about `403k`
  - `geometry_artifact_decomposition_matrix/hundred_k_nodes_pseudorealistic_rich_artifact_classes`
    - `artifact_count_total` was about `414k`
    - `artifact_kind_detailed_trace_count` was about `414k`
- current post-policy geometry-rich state
  - `rocketship_scale_matrix/hundred_k_nodes_geometry_profile_narrow_round_trip`
    - `hot_update_micros = 113304`
    - `diagnostic_artifact_count = 30`
    - `detailed_trace_entries = 0`
  - `rocketship_scale_matrix/hundred_k_nodes_geometry_profile_propagation_wave`
    - `hot_update_micros = 153551`
    - `diagnostic_artifact_count = 32`
    - `detailed_trace_entries = 0`
  - `geometry_artifact_decomposition_matrix/hundred_k_nodes_pseudorealistic_rich_artifact_classes`
    - `artifact_count_total = 32`
    - `artifact_kind_detailed_trace_count = 0`
    - `artifact_kind_minimal_summary_count = 32`

Interpretation:

- the hottest wall was never "we cannot traverse or update the resident graph"
- the hottest wall was "rich geometry diagnostics explode the commit surface at scale"
- Phase 1 removed that wall from the hot path
- the next likely walls are merged recompute width and then materialization/locality pressure
- storage-layout refactor, including hybrid AoSoA, should remain conditional on what the post-diagnostics numbers look like

Current locality/layout conclusion:

- keep AoSoA as a live option
- do not make it the first hardening move
- first hardening wave has already removed the obvious hot-path publication and diagnostics waste

## 4. Profile Boundary Policy

The current runtime profiles in [policies.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/config/data/policies.rs) are:

- `CertificationCore`
- `GeometryKernel`
- `ChipSimulation`
- `AiWorkflow`

Measured behavior shows these should be treated as policy boundaries, not just labels.

### Operational Thin

Use for hot kernel execution:

- no detailed traces
- minimal summaries only
- bounded artifact count
- replay and checkpoint kept available, but not trace-heavy by default

Evidence:

- `profile_matrix/certification_core_zero_diagnostics_commit_query_round_trip`
  - median `223us`
  - `diagnostic_artifact_count = 5`
  - `detailed_trace_entries = 0`

### Geometry Rich

Use for bounded certification and interactive debugging:

- minimal summaries hot by default
- detailed traces available through rich or replay-backed paths rather than forced synchronous emission
- richer publication artifacts
- acceptable at small scale and now materially healthier at rocketship scale after Phase 1 policy enforcement

Evidence:

- `profile_matrix/geometry_kernel_rich_commit_query_round_trip`
  - median `316us`
  - `diagnostic_artifact_count = 2`
  - `detailed_trace_entries = 0`
- `rocketship_scale_matrix/hundred_k_nodes_geometry_profile_propagation_wave`
  - `hot_update_micros = 153551`
  - `diagnostic_artifact_count = 32`
  - `detailed_trace_entries = 0`

### Audit Or Replay Heavy

Use for recoverability, certification, and offline forensics:

- persisted durability
- replay and recovery verification
- summary parity and compiled-view parity
- not appropriate as the default hot loop

Evidence:

- `artifact_recoverability_matrix`
- `recoverability_policy_matrix`
- `replay_recovery_matrix`

Practical profile conclusion:

- the grand hardening plan should explicitly separate hot operational geometry from rich geometry certification
- rich geometry should survive, but much more of it should move behind deferred or replay-reconstructable boundaries

## 5. Measurement Hygiene

The harness is now in better shape than before, but this is still worth tracking deliberately.

What is already safe:

- JSON emission happens after timing
- summary aggregation happens after timing
- baseline checks happen after timing
- `perf_harness_measurement_matrix` in [performance_profiles.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/tests/performance_profiles.rs) proves a heavy metrics payload can cost `7332us` while the recorded elapsed remains `0us`

What changed:

- [performance_support.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/tests/performance_support.rs) now has `measurement_from(...)`
- shared commit-path measurement in [performance_profiles.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/tests/performance_profiles.rs) now uses that helper

What remains:

- a large hand-rolled `PerfMeasurement { ... }` surface still exists in `performance_profiles.rs`
- there is still value in a one-time sweep converting the remaining hand-rolled timing blocks onto `measurement_from(...)`

Measurement hygiene conclusion:

- the harness-level risk is under control
- the sweep should be part of hardening, but it is no longer a blocker for planning

## What We Now Know Well Enough To Plan

We now have enough scoping confidence to move to the systemic hardening plan.

The hardening plan should optimize in this order:

1. profile boundary enforcement on top of the new artifact policy
2. geometry-rich deferred and replay-reconstructable artifact strategy refinement
3. merged relational-plus-signal target envelopes
4. post-policy locality and materialization pressure
5. final measurement-hygiene sweep

What we should not do first:

- broad new coverage building
- premature storage-layout rewrite
- optimizing packet counts in isolation while profile policy and merged recompute scaling are still the clearer next walls
