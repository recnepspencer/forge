# S9.16 Acceptance Map

## Purpose

This document is the closeout map for `S9.16`.

It exists to prevent a familiar failure mode:

- implementation moves forward
- architecture language sounds right
- a few local tests pass
- but the milestone has no explicit acceptance owner, no named invariant, and no proof surface

`S9.16` is a performance-truth program. That means each sub-batch must name:

- the invariant being hardened
- the runtime counter, summary, or surface that proves it
- the tests or certification lanes that own it
- the negative-space condition that must stay impossible

This map does not replace the architecture plan in
[signal_architecture2.md](/C:/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge_signal/signal_architecture2.md).
It is the implementation-side acceptance companion for that plan.

## Program Rules

- Every `S9.16.x` claim must be backed by named runtime surfaces, not narrative.
- Performance claims must prefer breadth/counter invariants over elapsed-time assertions.
- If a batch depends on future certification harness work, the gap must be named explicitly.
- Cold reconstruction may be exposed, but only through explicitly named materialization or reconstruction APIs.
- No acceptance row is complete until both success and negative-space behavior are covered.
- For work that closes `S9.9`, `S9.10`, `S9.12`, or `S9.15`, acceptance must also name the retired legacy surface, not only the new target surface.
- "Supported" means "representable through the public or owned internal execution path," not "documented as preferred."

## Implementation Control Addendum For `S9.9` / `S9.10` / `S9.12` / `S9.15`

These workstreams need tighter acceptance scaffolding than ordinary batch work
because they are substrate-replacement programs, not feature additions.

### Required Acceptance Questions

Every implementation batch touching one of these workstreams must answer all of
the following:

1. What legacy execution surface is being retired?
2. What proof-bearing type now owns admissibility?
3. What phase boundary now emits the named proof?
4. What counter proves the new path stayed bounded?
5. What exact invalid path is now impossible or rejected before construction?

If a batch cannot answer those five questions, it has not yet produced an
acceptance-quality architectural change.

### Required Migration Tracking Shape

Implementation notes or closeout notes for these workstreams must include a row
using this schema:

| Workstream | Legacy surface retired | Bridge surface remaining | Final proof-bearing surface | Counter surface | Negative-space proof |
| --- | --- | --- | --- | --- | --- |

### Required Closeout Impossibility Checks

The following must be tracked explicitly as negative-space acceptance items:

- reconstructability without bounded journal proof
- restore from snapshot-like bundles
- whole-live candidate scope on supported merge paths
- grouped concurrent apply that serializes semantic work under a parallel label
- worker writes to shared runtime surfaces
- rollback via baseline bundle semantics
- branch restore without reconstructability proof
- routine lifecycle access to heavy capture

## Acceptance Table

| Slice | Invariant | Proof surface | Current coverage | Negative space |
| --- | --- | --- | --- | --- |
| `S9.16.1` hot/cold separation | Hot operational paths run from `RuntimeArtifactState` alone | merge comparability, effect application write lanes, observer/runtime artifact access, reconstruction telemetry | `tests::merge_adoption::merge_branch_equivalent_runtime_state_ignores_retained_artifact_richness`; `tests::phase1_api::observer_exposes_runtime_and_retained_artifacts_separately`; `tests::observability::explicit_retained_and_reconstructed_artifact_apis_match_policy` | retained richness must not change merge/apply/runtime outcome |
| `S9.16.1` cold access honesty | Cold reconstruction is explicit by API name and policy result | `materialize_explanation_artifact`, `materialize_provenance_artifact`, `reconstruct_*`, `DiagnosticsAvailability`, hot-path reconstruction counter | `tests::observability::explicit_omit_policy_surfaces_unavailable_artifacts`; `tests::observability::explicit_retained_and_reconstructed_artifact_apis_match_policy`; `tests::harness_bridge::*artifact*` | ordinary observer/operational queries must not silently reconstruct broad cold views |
| `S9.16.2` shared snapshot storage | Snapshot sharing changes storage strategy, never semantic meaning | snapshot ids, snapshot artifact-retention metadata, explicit restore intents, canonical dependency snapshot update derivation, shared-node restore batch planning, restore breadth planning, dependency snapshot contracts, `SnapshotStorageStrategy`, `SnapshotDeltaRecord`, storage telemetry counters, restore behavior, checkpoint telemetry including restore breadth counters | `tests::phase1_api::shared_dependency_snapshot_reports_storage_sharing_without_implying_semantics`; `tests::phase1_api::snapshot_storage_telemetry_distinguishes_replacement_from_version_only_delta`; `tests::phase1_api::set_dep_snapshot_uses_version_only_delta_when_snapshot_shape_is_stable`; `tests::phase1_api::derive_dependency_snapshot_restore_batch_uses_version_only_delta_for_shared_shape`; `tests::phase5_state::snapshot_artifact_retention_policy_changes_richness_not_restore_truth`; `tests::phase5_state::branch_snapshot_records_explicit_artifact_retention_for_non_active_branches`; `tests::phase5_state::restore_snapshot_with_active_policy_prunes_cold_richness_without_changing_operational_truth`; `tests::phase5_state::restore_snapshot_rejects_seed_recomputation_intent_before_mutation`; `tests::phase5_state::snapshot_restore_plan_reports_shared_delta_and_coarse_requirements`; existing snapshot delta tests | pointer-sharing/backing reuse must not become identity or restore semantics |
| `S9.16.3` locality-first invalidation | Invalidation breadth is bounded by canonical mutation-time delta | `FrontierPlan`, `FrontierExecutionSummary`, entry-level direct-dirty vs maybe-stale classification, frontier telemetry counters, touched-scope summaries, retained frontier trace records, flow invalidation summaries projected from frontier execution truth | `tests::invalidation_bugs::frontier_execution_summary_exposes_direct_dirty_and_maybe_stale_entries`; `tests::invalidation_bugs::frontier_runtime_counters_are_derived_from_execution_summary`; `tests::invalidation_bugs::frontier_tracing_policy_changes_retained_richness_not_invalidation_truth`; `tests::invalidation_bugs::reachable_cycle_detection_fails_before_false_frontier_commit`; `tests::invalidation_bugs::one_node_with_multiple_justifications_collapses_to_stable_canonical_entry`; `tests::invalidation_bugs::repeated_identical_inputs_produce_deterministic_frontier_summary`; `tests::invalidation_bugs::transitive_wave_contains_only_nodes_reachable_from_planned_roots`; `tests::invalidation_bugs::frontier_transitive_wave_count_stays_zero_when_no_transitive_entries_realize`; `tests::diagnostics::flow_diagnostics_report_zero_realized_transitive_waves_when_frontier_has_none`; existing invalidation/adversarial propagation tests | post hoc graph scans must not become the source of invalidation truth, and flow invalidation reporting must not depend on a second legacy semantic owner |
| `S9.16.4` reuse contracts | Reuse occurs only through explicit equivalence contract and typed strategy/origin truth | `ArtifactEquivalenceContract`, `ReuseBoundaryContext`, `ReuseBasis`, `ReuseOrigin`, typed rejection taxonomy, retained certification proof count, lineage/replay/history reuse surfaces, cross-identity correspondence evidence, partial-splice composition provenance | `tests::phase3_semantics::defined_computation_evaluate_cross_identity_reuses_cached_result_via_public_api`; `tests::phase3_semantics::defined_computation_evaluate_partial_splice_uses_public_api`; `tests::phase3_semantics::cross_identity_lineage_and_history_preserve_correspondence_family`; `tests::phase3_semantics::branch_local_cross_identity_rejection_preserves_main_correspondence_and_lineage`; `tests::phase3_semantics::branch_local_partial_splice_rejection_preserves_main_mixed_provenance`; `tests::phase5_state::snapshot_restore_preserves_advanced_reuse_history_truth`; `tests::telemetry_contract::runtime_metrics_surface_exposes_typed_advanced_reuse_counts`; `tests::observability::ordinary_summary_and_history_reads_do_not_materialize_cold_artifacts`; `tests::diagnostics::diagnostics_history_and_replay_preserve_typed_advanced_reuse_origins` | reuse must not come from ad hoc field comparison, continuity-token coincidence, observer reconstruction, or retained-richness availability |
| `S9.16.5` diagnostics tiering | Tier changes richness, not semantics | `DiagnosticsTier`, `RetentionBudget`, `ReconstructionBudget`, `DiagnosticsAvailability`, replay/lineage boundaries, bounded reconstruction telemetry with retained/reconstructed/denied attribution | `tests::observability::artifact_access_counters_attribute_lane_api_and_denial_reason`; `tests::observability::ordinary_summary_surfaces_do_not_trigger_artifact_reconstruction`; `tests::observability::tier_matrix_public_observer_surfaces_preserve_truth_while_availability_changes`; `tests::observability::ordinary_observer_access_never_increments_cold_or_denial_counters_across_tiers`; `tests::observability::branch_and_snapshot_churn_respect_retention_budget_under_all_tiers`; `tests::observability::long_session_branch_churn_with_mixed_reads_keeps_bounds_and_cold_work_honest`; `tests::observability::ordinary_summary_and_history_rendering_respect_retained_detail_limits`; `tests::diagnostics::diagnostics_profiles_control_retention_bounds`; `tests::phase5_state::replay_and_lineage_overlap_stay_equivalent_across_runtime_policy_matrix`; existing branch/snapshot/history budget tests | lower tier must not trigger hidden broad reconstruction for ordinary access, exceed retained envelopes under churn, or rewrite canonical replay/lineage/reuse truth |
| `S9.16.6` certification harness | Geometry-readiness claims come from workload-shaped certification, not anecdote | runtime counters and canonical summaries consumed by harness | future work | no log-scraping, no microbench overclaiming |

## Batch Tracking

### Substrate Completion Tracking

The following rows bind the completion spec to acceptance ownership.

| Workstream | Invariant | Required proof surface | Required counters | Negative space |
| --- | --- | --- | --- | --- |
| `S9.12` reconstructability completion | checkpoint + bounded journal + required derived rebuild is the only supported restore truth | `CheckpointBoundary`, `BoundedJournalSegment`, `RequiredDerivedRebuildSet`, `ReconstructabilityProof` | `journal_replay_span`, `journal_suffix_breadth`, `restore_authority_breadth`, `restore_required_derived_breadth`, `restore_diagnostic_richness_breadth` | no restore from snapshot bundle, no optional journal proof, no diagnostics-driven semantic rebuild |
| `S9.15` bounded merge completion | supported merge candidate construction is purely proof-driven and bounded | `MergeBoundaryWitness`, `StructuralMergeJournalSlice`, `ProofMinimalOverlapBasis`, `ConservativeOverlapExpansion`, `LoweredMergePlan` | `boundary_witness_kind`, `source_slice_breadth`, `proof_minimal_overlap_breadth`, `conservative_overlap_expansion_breadth`, `final_candidate_breadth`, `reconciliation_breadth` | no `MergeCandidateScope`, no whole-live supported candidate scope, no ambient branch-state discovery |
| `S9.9` true parallel apply completion | grouped concurrent apply is real on proof-safe static stages and all other full-parallel requests lower honestly to serial | `DisjointApplyProof`, `GroupLocalApplyPacket`, `ConcurrentApplyReductionPlan`, `LoweredApplyPlan` | `group_local_packet_breadth`, `reduction_packet_breadth`, `reduction_group_count`, `shared_surface_publication_breadth`, `parallel_admission_rejection_reason` | no fake `FullParallel`, no worker access to shared surfaces, no reduction-side semantic recomputation |
| `S9.10` rollback and lifecycle completion | rollback is effect-derived and lifecycle transfer is type-separated and cost-honest | `TransactionRollbackPacket`, `AuthorityTransferPacket`, `ExplicitBranchForkPacket`, `HeavyCaptureWitness`, `BranchLifecycleTransfer` | `rollback_packet_breadth`, `rollback_packet_count_by_subsystem`, `move_transfer_count`, `explicit_fork_count`, `restore_transfer_count`, `heavy_capture_count` | no baseline-bundle rollback truth, no implicit duplication on branch switch, no raw branch-bundle restore |

### `S9.16.1` Current Batch

Targeted invariant:

- hot/cold artifact separation is real in storage lanes, write lanes, and hot-path comparability

Landed proof surfaces:

- merge comparability consumes only runtime artifact state
- effect application uses `ArtifactWriteDelta`
- merge adoption uses `ArtifactWriteDelta`
- observer exposes explicit cold assembly helpers
- cold reconstruction-capable access is named `materialize_*` instead of masquerading as a neutral getter

Named tests:

- `tests::phase1_api::observer_exposes_runtime_and_retained_artifacts_separately`
- `tests::merge_adoption::merge_branch_equivalent_runtime_state_ignores_retained_artifact_richness`
- `tests::observability::explicit_omit_policy_surfaces_unavailable_artifacts`
- `tests::observability::explicit_retained_and_reconstructed_artifact_apis_match_policy`
- `tests::harness_bridge::operational_profile_reconstructs_rich_artifacts_without_retaining_facts`

Still open for `S9.16.1`:

- audit remaining history/reuse-certification reads to ensure they are cold-only or explicitly materialized
- reduce any remaining production dependence on history-shaped setters or convenience reconstruction
- decide whether cold observational summaries need stricter naming split beyond current materialization helpers

### `S9.16.2` Pre-acceptance conditions

Before `S9.16.2` can close, it must prove:

- narrow dependency changes do not require whole snapshot clones
- retention policy changes richness only
- restore and merge semantics remain stable under shared snapshot backing

The proof must come from runtime snapshot counters and semantic equality tests, not storage anecdotes.

Current starting proof:

- `DependencySnapshot::shares_storage_with(...)` and
  `SharedDependencySnapshot::shares_storage_with(...)` make storage sharing
  explicit on the type surface
- `DependencySnapshotUpdate::storage_strategy()` distinguishes shared
  replacement from version-only delta updates
- `tests::phase1_api::shared_dependency_snapshot_reports_storage_sharing_without_implying_semantics`
  proves that shared backing is an explicit storage fact rather than an implied
  semantic identity claim
- storage telemetry now distinguishes full shared replacement boundaries from
  version-only delta boundaries, which gives later certification harness work a
  canonical measurement surface instead of log inference
- snapshot capture now records explicit explanation/provenance retention in
  `SignalSnapshotMeta.artifact_retention`, and
  `tests::phase5_state::snapshot_artifact_retention_policy_changes_richness_not_restore_truth`
  proves that snapshot richness can change while restore truth stays stable
- restore now consumes explicit `SnapshotRestoreIntent`, so “rewind active
  state,” “restore captured richness,” and “apply active runtime policy after
  restore” are no longer bundled implicitly behind one restore helper
- dependency snapshot rewrites now derive a canonical narrow update form from
  explicit previous/next snapshot contracts, which lets stable-shape rewrites
  narrow to version-only delta updates instead of defaulting to shared replacement
- restore now also has a shared-node dependency snapshot batch planning surface,
  which gives future in-place restore work a canonical delta packet instead of
  forcing it to rediscover snapshot differences from whole graph state
- restore can now emit a proof-bearing `SnapshotRestorePlan` that distinguishes
  shared-node dependency snapshot delta work from the coarse replacement reasons
  that still remain, which keeps restore-breadth honesty explicit before the
  in-place restore engine exists
- restore execution now also records shared-delta breadth and coarse-reason
  counts into canonical checkpoint telemetry, so certification can consume
  runtime counters directly instead of relying only on the planning API

### `S9.16.3` Pre-acceptance conditions

Before `S9.16.3` can close, it must prove:

- frontier seeds come only from canonical mutation-time delta packets or batch summaries
- localized edits remain localized in propagation breadth
- disjoint invalidations remain disjoint under repeated churn
- entry-level direct-dirty vs maybe-stale truth is visible in canonical frontier summaries
- cycle preflight failure does not falsely commit frontier state
- repeated identical inputs produce deterministic frontier summaries
- multiple justifications collapse to one canonical entry with stable precedence
- transitive waves stay bounded to nodes reachable from planned roots
- tracing richness can vary by policy without changing invalidation meaning

The proof must come from frontier counters/summaries and adversarial propagation tests.

## Relationship To Existing Certification Docs

This map should be read alongside:

- [forge_signal_adversarial_testing_matrix.md](/C:/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/engineering/forge_signal_adversarial_testing_matrix.md)
- [forge_signal_fintech_certification_matrix.md](/C:/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/engineering/forge_signal_fintech_certification_matrix.md)
- [forge_harness_workflow_certification_design.md](/C:/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/engineering/forge_harness_workflow_certification_design.md)

Those docs define certification philosophy and domain workflow expectations.
This map binds `S9.16` implementation batches to concrete acceptance ownership inside `forge-signal`.

For `S9.9`, `S9.10`, `S9.12`, and `S9.15`, this map should be read together
with [s9_missing_substrate_completion.md](/C:/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge_signal/s9_missing_substrate_completion.md).
That document defines the proof chain and migration discipline; this map defines
the acceptance owner, counters, and negative-space obligations.
