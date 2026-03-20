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

## Acceptance Table

| Slice | Invariant | Proof surface | Current coverage | Negative space |
| --- | --- | --- | --- | --- |
| `S9.16.1` hot/cold separation | Hot operational paths run from `RuntimeArtifactState` alone | merge comparability, effect application write lanes, observer/runtime artifact access, reconstruction telemetry | `tests::merge_adoption::merge_branch_equivalent_runtime_state_ignores_retained_artifact_richness`; `tests::phase1_api::observer_exposes_runtime_and_retained_artifacts_separately`; `tests::observability::explicit_retained_and_reconstructed_artifact_apis_match_policy` | retained richness must not change merge/apply/runtime outcome |
| `S9.16.1` cold access honesty | Cold reconstruction is explicit by API name and policy result | `materialize_explanation_artifact`, `materialize_provenance_artifact`, `reconstruct_*`, `ArtifactMaterializationMode`, hot-path reconstruction counter | `tests::observability::explicit_omit_policy_surfaces_unavailable_artifacts`; `tests::observability::explicit_retained_and_reconstructed_artifact_apis_match_policy`; `tests::harness_bridge::*artifact*` | ordinary observer/operational queries must not silently reconstruct broad cold views |
| `S9.16.2` shared snapshot storage | Snapshot sharing changes storage strategy, never semantic meaning | snapshot ids, snapshot artifact-retention metadata, explicit restore intents, canonical dependency snapshot update derivation, shared-node restore batch planning, restore breadth planning, dependency snapshot contracts, `SnapshotStorageStrategy`, `SnapshotDeltaRecord`, storage telemetry counters, restore behavior, checkpoint telemetry including restore breadth counters | `tests::phase1_api::shared_dependency_snapshot_reports_storage_sharing_without_implying_semantics`; `tests::phase1_api::snapshot_storage_telemetry_distinguishes_replacement_from_version_only_delta`; `tests::phase1_api::set_dep_snapshot_uses_version_only_delta_when_snapshot_shape_is_stable`; `tests::phase1_api::derive_dependency_snapshot_restore_batch_uses_version_only_delta_for_shared_shape`; `tests::phase5_state::snapshot_artifact_retention_policy_changes_richness_not_restore_truth`; `tests::phase5_state::branch_snapshot_records_explicit_artifact_retention_for_non_active_branches`; `tests::phase5_state::restore_snapshot_with_active_policy_prunes_cold_richness_without_changing_operational_truth`; `tests::phase5_state::restore_snapshot_rejects_seed_recomputation_intent_before_mutation`; `tests::phase5_state::snapshot_restore_plan_reports_shared_delta_and_coarse_requirements`; existing snapshot delta tests | pointer-sharing/backing reuse must not become identity or restore semantics |
| `S9.16.3` locality-first invalidation | Invalidation breadth is bounded by canonical mutation-time delta | invalidation counters, frontier summaries, touched-node/touched-scope summaries | pre-existing invalidation tests plus future frontier certification | post hoc graph scans must not become the source of invalidation truth |
| `S9.16.4` reuse contracts | Reuse occurs only through explicit equivalence contract | `ArtifactEquivalenceContract`, `ReuseBasis`, retained certification proof count, lineage/replay reuse surfaces | partial existing reuse tests; full acceptance not complete yet | reuse must not come from ad hoc field comparison or retained-richness availability |
| `S9.16.5` diagnostics tiering | Tier changes richness, not semantics | diagnostics policy, artifact materialization mode, replay/lineage boundaries, bounded reconstruction telemetry | partial observability coverage; full tier matrix not complete yet | lower tier must not trigger hidden broad reconstruction for ordinary access |
| `S9.16.6` certification harness | Geometry-readiness claims come from workload-shaped certification, not anecdote | runtime counters and canonical summaries consumed by harness | future work | no log-scraping, no microbench overclaiming |

## Batch Tracking

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

The proof must come from frontier counters/summaries and adversarial propagation tests.

## Relationship To Existing Certification Docs

This map should be read alongside:

- [forge_signal_adversarial_testing_matrix.md](/C:/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/engineering/forge_signal_adversarial_testing_matrix.md)
- [forge_signal_fintech_certification_matrix.md](/C:/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/engineering/forge_signal_fintech_certification_matrix.md)
- [forge_harness_workflow_certification_design.md](/C:/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/engineering/forge_harness_workflow_certification_design.md)

Those docs define certification philosophy and domain workflow expectations.
This map binds `S9.16` implementation batches to concrete acceptance ownership inside `forge-signal`.
