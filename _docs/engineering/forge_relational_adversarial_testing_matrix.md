# Forge Relational Adversarial Testing Matrix

## Purpose

This document exists to prevent a repeat of the `forge-signal` failure mode where plausible happy-path coverage coexisted with missing adversarial tests for critical semantic boundaries.

For `forge-relational`, test coverage is not considered adequate unless each high-risk contract has explicit coverage for:

- success behavior
- failure behavior
- determinism behavior
- replay or recovery behavior where applicable
- "must not happen" negative-space behavior

These tests should use production runtime surfaces wherever possible:

- `RelationalRuntime`
- `RelationalTransaction`
- canonical commit envelopes
- snapshots and version reads
- diagnostics artifacts
- patch artifacts
- replay artifacts
- lineage graphs
- recovery plans and recovery outcomes
- `forge_harness`

## Contract Matrix

### Replay

Required adversarial cases:

- replay success reproduces profile-promised observable surfaces
- wrong branch context fails explicitly
- missing parent chain fails explicitly
- schema mismatch fails explicitly
- replay does not mutate authoritative runtime state
- repeated replay remains deterministic

Current contract tests:

- `tests::replay_contracts::replay_contract_success_reproduces_canonical_surfaces`
- `tests::replay_contracts::replay_contract_failure_wrong_branch_is_explicit`
- `tests::replay_contracts::replay_contract_failure_missing_parent_chain_is_explicit`
- `tests::replay_contracts::replay_contract_success_preserves_merge_parent_order`

### Derived index

Required adversarial cases:

- storage-visible reads remain correct with no index present
- index generation is branch-scoped
- index build failure does not alter truth semantics
- index lag does not alter truth semantics
- repeated index build over same source commit is deterministic
- recovery can omit or rebuild indexes without changing truth reads

Current contract tests:

- `tests::index_contracts::derived_index_contract_success_branch_scoped_build_keeps_storage_fallback`
- `tests::index_contracts::derived_index_contract_failure_unknown_index_keeps_truth_reads_correct`

### Lineage

Required adversarial cases:

- create events are authoritative and deterministic
- correspondence remains advisory until promotion
- invalid references fail explicitly
- promotion is the only advisory-to-authoritative path
- branch-local divergence preserves separate lineage histories
- replay and diagnostics include lineage where promised

Current contract tests:

- `tests::lineage_contracts::lineage_contract_correspondence_stays_advisory_until_promoted`
- `tests::lineage_contracts::lineage_contract_failure_invalid_references_do_not_promote`

### Durability and recovery

Required adversarial cases:

- checkpoint and recovery reproduce branch heads and latest commit
- schema mismatch blocks recovery explicitly
- missing parent chain blocks recovery explicitly
- partial/corrupt durable input fails explicitly
- recovery does not depend on snapshot materialization artifacts
- replay equivalence holds before and after recovery

Current contract tests:

- `tests::durability_contracts::durability_contract_recovery_rebuilds_branch_heads_and_latest_commit`
- `tests::durability_contracts::durability_contract_failure_schema_mismatch_is_explicit`
- `tests::durability_contracts::durability_contract_failure_missing_parent_chain_is_explicit`
- `tests::durability_contracts::durability_contract_recovery_preserves_merge_parent_order`

### Branch and history

Required adversarial cases:

- branch creation from current head is deterministic
- branch-targeted commits advance only the selected branch
- history remains merge-ready at the representation level
- parent ordering is deterministic
- failed publication does not advance history

Current runtime tests:

- `tests::branch_creation_and_branch_targeted_commits_build_a_version_graph`
- `tests::duplicate_branch_creation_is_rejected`
- `tests::merge_commit_uses_deterministic_parent_order_and_advances_target_branch`
- `tests::merge_commit_requires_existing_parent_branch_heads`

### MVCC and retention

Required adversarial cases:

- pinned snapshots block reclaim
- released snapshots allow reclaim when policy permits
- historical reads survive later updates
- chunk-aware retention remains deterministic
- version-visible reads do not depend on current live payload

Current runtime tests:

- `tests::snapshot_pins_block_reclaim_until_release`
- `tests::snapshots_resolve_historical_entity_payloads_by_version`
- `tests::chunked_storage_summary_tracks_visibility_boundaries`
- `tests::chunk_diagnostics_and_packet_plans_are_public_and_stable`

## Enforcement

The matrix is enforced through three mechanisms:

1. contract-tagged test modules in `crates/forge-relational/src/tests`
2. focused CI guards that run critical adversarial lanes directly
3. architectural review against semantic-risk categories, not only line coverage

CI should fail if:

- a required contract module is missing
- a contract module lacks declared lane coverage comments
- critical adversarial tests are renamed away or removed
- deterministic/failure/recovery lanes stop passing

## Review Rule

Every new high-risk subsystem or contract must add:

- at least one success-path adversarial test
- at least one failure-path adversarial test
- at least one determinism/parity test
- at least one replay or recovery test if the subsystem affects history, publication, or durability

If those tests do not exist, the subsystem is not considered ready regardless of implementation completeness.
