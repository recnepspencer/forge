# Forge Signal Adversarial Testing Matrix

## Purpose

This document exists to keep `forge-signal` honest against its own promises.

The failure mode we are explicitly avoiding is:

- architecture sounds right
- happy-path behavior looks correct
- some deterministic and rollback tests exist
- but a core contract is under-tested, so a real bug survives in optimization paths, rollback edges, lifecycle asymmetry, or retained-state behavior

Coverage is not considered adequate unless each high-risk contract has explicit coverage for:

- success behavior
- failure behavior
- determinism behavior
- boundedness / retention behavior where applicable
- negative-space behavior: what must not happen

These tests should prefer production runtime surfaces wherever possible:

- `SignalGraph`
- `SignalRuntime`
- `SignalTransaction`
- execution reports
- diagnostics summaries
- explanation / provenance artifacts
- replay artifacts
- snapshots, branches, and lineage records
- harness fixtures and certification artifacts

## Vision Promise Validation

The current signal vision makes five core promises. Each promise must stay mapped to concrete contract lanes.

### 1. Deterministic execution

Required evidence:

- serial, staged-parallel, and full-parallel agree on canonical retained surfaces
- replay ordering is canonical
- explanation, provenance, lineage, and report surfaces remain stable across executor-policy churn

Primary enforcement:

- `tests::adversarial_parallel`
- `tests::determinism`
- `tests::diagnostics`
- `tests::phase5_state`
- `bash scripts/ci/check_signal_parallel_determinism_cert.sh 4 "$DIR"`

### 2. Transactional rollback semantics

Required evidence:

- failures rewind graph-visible state
- failures do not leak semantic artifacts
- dynamic dependency capture does not leave ghost edges
- config/keyed state mutated during aborted work does not leak forward
- event-bus rollback remains lifecycle-symmetric

Primary enforcement:

- `logic::transaction::tests`
- `tests::phase3_semantics`
- `tests::diagnostics`
- `tests::adversarial_edges`
- `logic::events::tests`
- `bash scripts/ci/check_signal_failure_matrix.sh`

### 3. Aspect-aware invalidation granularity

Required evidence:

- aspect and partition scoping dirties only the intended downstreams
- reconverging invalidation paths do not fabricate cycles
- invalidation without replacement remains inspectable through lineage/replay where promised

Primary enforcement:

- `tests::multi_aspect`
- `tests::phase3_semantics`
- `tests::adversarial_edges`
- `tests::phase5_state`

### 4. Explicit separation from truth-state storage

Required evidence:

- snapshots capture signal-runtime evaluation state, not host truth payload ownership
- replay is evaluation-state replay, not arbitrary host side-effect replay
- branch/snapshot metadata stays runtime-local and deterministic

Primary enforcement:

- `tests::phase5_state`
- `tests::observability`
- `crates/forge-signal/BOUNDARY_CONTRACT.md`
- `crates/forge-signal/docs/SNAPSHOTS_BRANCHES_AND_REPLAY.md`

### 5. First-class runtime self-inspection

Required evidence:

- explanations and provenance are queryable or reconstructable according to policy
- replay and lineage can answer “what changed, why, and when”
- diagnostics remain bounded and recent under churn
- artifact access semantics are explicit under each runtime policy

Primary enforcement:

- `tests::observability`
- `tests::diagnostics`
- `tests::adversarial_diagnostics`
- `tests::harness_adapter`
- `tests::phase5_state`

## Contract Matrix

### Planner and invalidation

Required adversarial cases:

- `MaybeStale` nodes validate clean without running compute when meaningful inputs did not change
- `ForceOnDemand` recomputes clean targets explicitly
- deep linear chains do not recurse explosively
- reconverging DAG invalidation does not falsely report cycles
- dirty/maybe-stale propagation remains deterministic under repeated frontier churn

Current contract tests:

- `tests::phase4_planner::maybe_stale_requested_target_validates_clean_without_running_compute`
- `tests::phase4_planner::force_on_demand_plans_and_executes_clean_target`
- `tests::phase4_planner::build_evaluation_plan_handles_deep_linear_chain_without_recursion`
- `tests::adversarial_edges::reconverging_invalidation_path_is_not_reported_as_a_cycle`
- `tests::graph_core::maybe_stale_transitive_dependent`

### Transactions, rollback, and isolation

Required adversarial cases:

- rollback restores touched graph state and leaves untouched state intact
- dynamically discovered dependencies do not leave ghost subscribers after abort
- aborted keyed evaluation does not leak key-registry or memoized state
- poisoned transactions rewind deterministically
- failure during event begin or flush does not commit state

Current contract tests:

- `logic::transaction::patch_buffer::tests::rollback_clears_only_touched_entries_and_preserves_untouched`
- `logic::transaction::tests::rollback_removes_dynamic_dependency_capture_ghost_subscribers`
- `tests::phase3_semantics::aborted_keyed_evaluation_does_not_leak_key_registry_growth`
- `logic::transaction::tests::poisoned_transaction_returns_poisoned_outcome`
- `logic::transaction::tests::failure_during_event_begin_rewinds_graph`
- `logic::transaction::tests::failed_event_flush_does_not_commit_graph_state`

### Event lifecycle symmetry

Required adversarial cases:

- checkpoint order is deterministic
- partial flush failure rolls back only subscribers that actually checkpointed
- reverse-order rollback remains stable

Current contract tests:

- `logic::events::tests::deterministic_order_independent_of_registration`
- `logic::events::tests::rollback_only_unwinds_successfully_checkpointed_subscribers_after_partial_flush_failure`
- `logic::events::tests::rollback_runs_reverse_order`

### Parallel semantics

Required adversarial cases:

- apply failure leaks no partial graph or semantic state
- dynamic rewiring preserves semantic parity
- logically equivalent dependency/region orders canonicalize to the same retained artifacts
- worker-count / chunk-size / apply-group churn remains deterministic
- narrow stages stay serial when the admission policy says they should

Current contract tests:

- `tests::adversarial_parallel::full_parallel_apply_failure_does_not_leak_partial_semantic_state`
- `tests::adversarial_parallel::full_parallel_rewires_dynamic_dependencies_without_losing_parity`
- `tests::adversarial_parallel::logically_equivalent_region_orders_produce_identical_provenance_and_replay`
- `tests::adversarial_parallel::repeated_executor_policy_churn_keeps_tolerance_boundary_artifacts_stable`
- `tests::adversarial_parallel::many_thin_stages_remain_serial_under_parallel_threshold`
- `bash scripts/ci/check_signal_parallel_determinism_cert.sh 4 "$DIR"`

### Diagnostics, observability, and artifact access

Required adversarial cases:

- diagnostics tier changes richness, not runtime/reuse/lineage/replay truth
- retained-vs-reconstructed semantics are explicit and policy-correct
- typed availability distinguishes retained, reconstructed, omitted-by-tier, denied-by-budget, and unavailable states
- diagnostics history prefers recency, not arena-index accident
- repeated failure/rollback churn keeps latest retained state current and bounded
- explanation/provenance surfaces remain deterministic under mixed upstream states
- history/detail retention budget overrides are enforced
- ordinary summary/history/replay access performs zero cold reconstruction
- cold artifact access counters attribute retained reads, reconstructed reads, and denial reasons

Current contract tests:

- `tests::observability::explicit_retained_and_reconstructed_artifact_apis_match_policy`
- `tests::observability::artifact_access_counters_attribute_lane_api_and_denial_reason`
- `tests::adversarial_diagnostics::execution_history_prefers_most_recent_records_over_low_arena_indices`
- `tests::adversarial_diagnostics::repeated_failure_and_rollback_loops_preserve_explanation_after_churn`
- `tests::observability::explanation_is_deterministic_with_multiple_upstreams_and_mixed_states`
- `tests::diagnostics::runtime_policy_history_budget_overrides_are_enforced`
- `tests::observability::ordinary_summary_surfaces_do_not_trigger_artifact_reconstruction`

### Resource boundedness and stale-state pressure

Required adversarial cases:

- edge stores compact after churn
- dependency snapshot storage compacts after churn
- unregister and slot reuse do not leave ghost edges or tombstoned tier metadata
- operational diagnostics stay bounded under repeated snapshot/dependency churn
- branch/snapshot restore loops do not accumulate stale lineage or replay debris

Current contract tests:

- `tests::adversarial_edges::gc_epoch_compacts_edge_and_snapshot_storage_after_churn`
- `tests::adversarial_edges::unregister_and_slot_reuse_after_churn_leave_no_ghost_edges`
- `logic::transaction::tests::unregister_clears_tier_metadata_tombstones`
- `tests::adversarial_diagnostics::operational_profile_stays_bounded_under_snapshot_and_dependency_churn`
- `tests::phase5_state::repeated_snapshot_restore_loops_do_not_leak_non_restore_lineage_or_branch_state`

### Keyed and memoized execution

Required adversarial cases:

- memoization stays scoped by family
- aborted transactions do not promote memoized results
- many keys and many families remain distinct and stable
- replay and lineage remain coherent around memoized reuse

Current contract tests:

- `tests::phase3_semantics::memoization_is_scoped_by_family`
- `tests::phase3_semantics::memoization_write_is_discarded_on_rollback`
- `tests::adversarial_keyed::many_families_sharing_same_key_remain_distinct`
- `tests::adversarial_keyed::stress_thousands_of_keyed_lookups_and_memo_keys`
- `tests::phase5_state::lineage_distinguishes_replacement_refresh_and_memoized_reuse`

### Snapshot, branch, replay, and lineage

Required adversarial cases:

- snapshot capture/restore round-trips preserve canonical evaluation state
- cross-branch restore is rejected explicitly
- branch-local history remains isolated under switch/restore churn
- replay slices are stable by branch, node, artifact, cursor range, and snapshot neighborhood
- lineage distinguishes refresh, replace, restore, invalidation-without-replacement, branch transitions, and memoized reuse
- continuity-token semantics remain generic and domain-agnostic

Current contract tests:

- `tests::phase5_state::graph_snapshot_round_trip_restores_versions_and_emits_restore_replay_and_lineage`
- `tests::phase5_state::restore_branch_snapshot_rejects_cross_branch_payloads_and_keeps_catalog_consistent`
- `tests::phase5_state::branch_switch_and_restore_churn_preserve_branch_local_heads_and_replay_isolation`
- `tests::phase5_state::replay_slices_and_lineage_chains_are_branch_and_snapshot_queryable`
- `tests::phase5_state::lineage_distinguishes_replacement_refresh_and_memoized_reuse`
- `tests::phase5_state::invalidation_emits_lineage_without_replacement_and_branch_restore_is_local`
- `tests::phase5_state::continuity_token_preserves_lineage_without_requiring_output_identity`
- `tests::observability::tier_matrix_public_observer_surfaces_preserve_truth_while_availability_changes`
- `tests::observability::branch_and_snapshot_churn_respect_retention_budget_under_all_tiers`
- `tests::observability::long_session_branch_churn_with_mixed_reads_keeps_bounds_and_cold_work_honest`
- `bash scripts/ci/check_signal_phase5_contracts.sh`

### Public API and contract surfaces

Required adversarial cases:

- public contract markers remain exported and discoverable
- runtime builder defaults remain stable
- easy-mode APIs compile down to the same truthful runtime semantics
- harness capture surfaces replay/explanation/provenance according to contract

Current contract tests:

- `tests::contracts::boundary_contract_markers_are_public`
- `tests::phase1_api::runtime_builder_uses_expected_defaults`
- `tests::phase1_api::easy_mode_supports_input_computed_get_set_and_batch`
- `tests::harness_adapter::signal_harness_adapter_captures_v2_replay_summary`
- `tests::harness_platform::signal_harness_platform_runs_serial_parallel_parity`
- `tests::observability::ordinary_observer_access_never_increments_cold_or_denial_counters_across_tiers`
- `tests::observability::ordinary_summary_and_history_rendering_respect_retained_detail_limits`

## Enforcement

The matrix is enforced through four mechanisms:

1. contract-oriented test modules in `crates/forge-signal/src/tests`
2. focused CI lanes that run critical contract groups directly
3. certification scripts that compare canonical artifacts rather than only checking exit status
4. architectural review against semantic-risk categories, not only line coverage

CI should fail if:

- a required contract lane is removed
- a contract module referenced here stops passing
- determinism, failure, phase-5, or resource-bounds lanes regress
- a new high-risk subsystem lands without an explicit contract category and adversarial coverage

## Required CI Lanes

These lanes are the minimum enforcement surface for the current runtime:

- `bash scripts/ci/check_signal_failure_matrix.sh`
- `bash scripts/ci/check_signal_contract_matrix.sh`
- `bash scripts/ci/check_signal_resource_bounds.sh`
- `bash scripts/ci/check_signal_phase5_contracts.sh`
- `bash scripts/ci/check_signal_core_profiles.sh`
- `bash scripts/ci/check_signal_semantic_snapshots.sh "$DIR"`
- `bash scripts/ci/check_signal_parallel_determinism_cert.sh 4 "$DIR"`
- `bash scripts/ci/run_signal_perf_lane.sh`

## Review Rule

Every new high-risk subsystem or contract must add:

- at least one success-path adversarial test
- at least one failure-path adversarial test
- at least one determinism or parity test
- at least one boundedness/retention test if the subsystem stores historical or retained state
- at least one replay, restore, or recovery test if the subsystem affects history, publication, snapshots, or branch state

If those tests do not exist, the subsystem is not considered ready regardless of implementation completeness.
