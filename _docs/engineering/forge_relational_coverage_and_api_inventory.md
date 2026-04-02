# Forge Relational Coverage And API Inventory

This document exists to keep `forge-relational` performance hardening and future DX work systematic.

It has two goals:

1. show which runtime and workload categories are already covered by performance certification versus only partially covered or still missing
2. inventory the current public surface so we know exactly where runtime methods, authority methods, facades, and configuration knobs live before we start consolidating or redesigning them

The source scan for this document was taken from:

- [performance_profiles.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/tests/performance_profiles.rs)
- the pre-hardening scope memo in [forge_relational_pre_hardening_scope.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/engineering/forge_relational_pre_hardening_scope.md)
- [facade.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/facade.rs)
- [api.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/presentation/api.rs)
- [builder.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/logic/builder.rs)
- [runtime_state.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/logic/runtime/state/runtime_state.rs)
- access and authority modules under `history`, `durability`, `indexes`, `inspection`, `publication`, `simulation`, `storage`, `merge`, and `visibility`
- the current baseline workbook in [forge_relational_performance_baseline.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/engineering/forge_relational_performance_baseline.md)
- the complexity registry in [forge_relational_complexity_budgets.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/engineering/forge_relational_complexity_budgets.md)
- the adversarial matrix in [forge_relational_adversarial_testing_matrix.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/engineering/forge_relational_adversarial_testing_matrix.md)

## Coverage Summary

Current certified perf lane breadth:

- 23 performance families
- 72 benchmark cases
- 385 tracked metrics in the committed baseline workbook

Current structural support beyond elapsed timing:

- runtime complexity contracts and counter proofs
- adversarial semantic coverage
- profile-aware baselines
- per-case regression contracts
- phase timing on selected commit, merge, replay, retention, query, and workflow cases

Short read:

- primitive runtime coverage is strong
- structural certification coverage is strong
- workflow coverage is good
- inspection, index parity, and mixed-load coverage are now good
- rocketship-scale resident graph coverage is now strong, with both zero-diagnostics and geometry-profile propagation companions at 100k-node scale
- geometry-kernel realism is now good-to-strong
- chip-simulator realism is now good, with both thin and rich diagnostic event-wave churn companions
- sustained long-run realism is now good
- hot-path vs cold-path / replay-recoverable cost is now explicitly measured, including richer publication and diagnostic split on geometry and chip-shaped paths
- artifact recoverability is now explicitly certified for geometry diagnostics summaries and chip compiled views
- large-scale geometry artifact-class decomposition is now explicitly measured at 100k-node pseudo-realistic scale
- merged relational-plus-signal kernel wave coverage now exists in both operational and development profiles
- hot-path, deferred, and replay-reconstructable policy boundaries are now certified as first-class budgets
- profile-boundary drift is now visible in the committed baseline across synthetic profile, workflow, chip, and rocketship suites

## Performance Coverage Matrix

| Category | Status | Current Coverage | Main Source |
| --- | --- | --- | --- |
| Commit-path cost | Covered | `commit_delta_matrix` for narrow create burst, cross-partition relation burst, persisted single entity create | [performance_profiles.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/tests/performance_profiles.rs) |
| Durability append cost | Covered | `durability_append_matrix` for fresh-store append and existing-segment append | [performance_profiles.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/tests/performance_profiles.rs) |
| Query packetization and execution shape | Covered | `query_packet_matrix` for explicit targets, entity kind scan, cross-partition connectivity traversal, packet count, scope count, planning and execution timing | [performance_profiles.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/tests/performance_profiles.rs) |
| Snapshot and materialization cost | Covered | `snapshot_materialization_matrix` for current snapshot, historical version, projection identity surface | [performance_profiles.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/tests/performance_profiles.rs) |
| Retention and reclaim | Covered | `retention_reclaim_matrix` plus workflow reclaim round trip | [performance_profiles.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/tests/performance_profiles.rs) |
| Replay and recovery | Covered | `replay_recovery_matrix` and persisted recovery replay workflow | [performance_profiles.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/tests/performance_profiles.rs) |
| Merge and lineage execution | Covered | `merge_lineage_matrix` including planning, execute, verify vs execute, prepare vs execute, lineage breadth, phase timing | [performance_profiles.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/tests/performance_profiles.rs) |
| Profile and observability cost | Covered | `profile_matrix` for certification rich, geometry rich, zero-diagnostics thin profile | [performance_profiles.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/tests/performance_profiles.rs) |
| End-to-end workflows | Covered | `workflow_matrix` for trade correction, intraday risk, audit, persistence/recovery/replay, retention | [performance_profiles.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/tests/performance_profiles.rs) |
| Complexity-budget proof coverage | Covered | runtime clone, snapshot pin maintenance, visible scans, retention pass, adjacency lookup, invariant materialization | [forge_relational_complexity_budgets.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/engineering/forge_relational_complexity_budgets.md) |
| Adversarial semantic coverage | Covered | replay, derived indexes, lineage, durability/recovery, branch/history, MVCC/retention | [forge_relational_adversarial_testing_matrix.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/engineering/forge_relational_adversarial_testing_matrix.md) |
| Geometry-kernel domain perf | Covered | `geometry_kernel_matrix` covers persisted topology identity recovery plus topology bridge waves under rich and zero-diagnostics geometry profiles, and `cad_topology_matrix` now adds a pseudo-realistic CAD assembly interface bridge workload for local explicit reads plus topology connectivity | [performance_profiles.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/tests/performance_profiles.rs) |
| Rocketship-scale resident graph perf | Covered | `rocketship_scale_matrix` now certifies a 100k-node resident world in four shapes: flat narrow hot-update plus explicit-query paths under zero-diagnostics and rich geometry profiles, a pseudo-realistic 12-subsystem rocketship assembly with mixed explicit and traversal queries, and a deeper subsystem propagation wave that spans multiple interfaces | [performance_profiles.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/tests/performance_profiles.rs) |
| Chip-simulator domain perf | Covered | `chip_simulator_matrix` now covers dense fanout compile waves in standard and rich diagnostics, checkpoint-window recover-and-compile, branch/savepoint rollback compile stepping, and sustained event-wave compile churn under repeated stepping | [performance_profiles.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/tests/performance_profiles.rs) |
| CAD topology perf realism | Covered | `cad_topology_matrix` now certifies a pseudo-realistic assembly interface bridge workload with bounded local explicit reads and connectivity inspection over a multi-subassembly topology | [performance_profiles.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/tests/performance_profiles.rs) |
| Sustained-load and drift | Covered | `sustained_load_matrix` now includes long-window commit/query churn stability, replay-window drift, retention-pass drift, and mixed topology query churn across repeated updates, explicit reads, and traversal waves | [performance_profiles.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/tests/performance_profiles.rs) |
| Hot-path vs cold-path / replayable recovery cost | Covered | `hot_cold_path_matrix` now certifies narrow hot commit/query and hot commit/compile work separately from checkpoint, recover, replay, and cold verification/query/compile work for geometry and chip-shaped surfaces, plus rich publication/diagnostic companions that expose what certification payload is riding the hot path | [performance_profiles.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/tests/performance_profiles.rs) |
| Mixed-load concurrency perf | Covered | `mixed_load_matrix` now certifies concurrent snapshot/version pressure and concurrent relation-index parity pressure under scheduler contention | [performance_profiles.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/tests/performance_profiles.rs) |
| Index-assisted vs fallback perf parity | Covered | `index_parity_matrix` now certifies warm entity-field generations, build-failed fallback, and persisted recovery parity | [performance_profiles.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/tests/performance_profiles.rs) |
| Inspection API perf | Covered | `inspection_budget_matrix` now certifies graph/kind/connectivity bundles, structural identity historical windows, and retention plus commit inspection windows | [performance_profiles.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/tests/performance_profiles.rs) |
| Invariant materialization perf | Covered | `invariant_materialization_matrix` now isolates touched-surface custom invariant preparation and execution work, including invariant pre/post timing and traversal counters on a structural commit wave | [performance_profiles.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/tests/performance_profiles.rs) |
| Artifact recoverability | Covered | `artifact_recoverability_matrix` now certifies which geometry diagnostics summaries and chip compiled outputs can be reconstructed cleanly through checkpoint, recover, and replay without replay mismatches | [performance_profiles.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/tests/performance_profiles.rs) |
| Large-scale geometry artifact decomposition | Covered | `geometry_artifact_decomposition_matrix` now isolates artifact counts, entry counts, artifact kinds, artifact scopes, and selected entry codes for a 100k-node pseudo-realistic rich geometry workload | [performance_profiles.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/tests/performance_profiles.rs) |
| Runtime-bridge mock kernel wave perf | Covered | `runtime_bridge_mock_matrix` now certifies narrow, medium-region, and mixed-locality relational commit plus query waves feeding a downstream mocked bridge invalidation and recomputation wave in operational and development profiles without crossing the crate boundary | [performance_profiles.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/tests/performance_profiles.rs) |
| Explicit hot, deferred, and reconstructable policy budgets | Covered | `recoverability_policy_matrix` now certifies must-be-hot truth surfaces, deferred trace surfaces, and replay-reconstructable summaries and compiled artifacts for geometry and chip-shaped workloads | [performance_profiles.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/tests/performance_profiles.rs) |
| Profile-boundary drift visibility | Covered | the committed perf lane now emits `profile_execution_lane_code`, `profile_diagnostics_boundary_code`, and `profile_matches_defaults` for synthetic profile, workflow, chip, and rocketship suites, and the summary report surfaces profile drift as a first-class hotspot class | [performance_profiles.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/tests/performance_profiles.rs) |

## Remaining Depth Work

The high-priority depth categories that were previously missing are now covered. The remaining work is no longer about proving that whole surfaces exist; it is about tightening the policy and hotspot story inside the covered surfaces.

### Current Focus

- geometry-rich diagnostics policy
  - classify which artifact classes must remain synchronous
  - cap or sample reconstructable trace-heavy output at rocketship scale
  - keep certification-rich truth without letting `400k+` rich artifacts dominate hot geometry commits

- merged relational-plus-signal realism
  - broaden beyond the current single wave into larger invalidation regions
  - measure packet locality and downstream recomputation width together
  - turn merged-kernel measurements into explicit target envelopes for the geometry kernel

- recoverability policy hardening
  - turn the current recoverability proofs into stronger artifact-class policy tables
  - document what is `must-be-hot`, `can-defer`, and `reconstructable-from-replay`
  - use those tables to drive profile cleanup later during DX hardening

- targeted hotspot debugging
  - rich geometry diagnostics/publication remains the clearest scale tax
  - merged relational-plus-signal waves need broader locality envelopes
  - long-run rocketship and simulator realism should now be optimized with the full matrix rather than ad hoc spot fixes

## Public Surface Inventory

This section inventories the user-facing and operator-facing surface that currently matters most for runtime use and later DX cleanup.

It is intentionally organized by access pattern:

- entrypoints
- builder and configuration
- runtime methods
- read access facets
- authority facets
- exported data surfaces

## Top-Level Entry Points

Primary entrypoint:

- [RelationalRuntimeApi](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/presentation/api.rs)
  - `builder()`
  - `runtime()`

Primary crate facade:

- [facade.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/facade.rs)
  - `config`
  - `commit_strategies`
  - `diagnostics`
  - `durability`
  - `errors`
  - `history`
  - `identity`
  - `inspection`
  - `indexes`
  - `lineage`
  - `merge`
  - `runtime`
  - `payloads`
  - `harness`
  - `publication`
  - `query`
  - `replay`
  - `schema`
  - `snapshots`
  - `storage`
  - `symbols`
  - `transactions`

Boundary rule from [lib.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/lib.rs):

- the crate exposes a single public facade boundary through `pub mod facade`
- most other top-level crate modules are intentionally internal

## Runtime Construction Surface

Primary builder:

- [RelationalRuntimeBuilder](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/logic/builder.rs)

Builder methods:

- `new`
- `profile`
- `runtime_name`
- `execution_model`
- `planning`
- `commit_authority`
- `durability_mode`
- `diagnostics`
- `schema_registry`
- `invariant_catalog`
- `custom_invariant`
- `commit_strategy`
- `commit_strategy_executor`
- `entity_capacity`
- `relation_capacity`
- `mvcc`
- `storage_layout`
- `publication`
- `payload_policy`
- `symbol_policy`
- `visibility_cache_policy`
- `durable_log_policy`
- `durability_policy`
- `durable_store_layout`
- `adjacency_policy`
- `cross_context_policy`
- `cascade_delete_policy`
- `compiled_lane_policy`
- `relation_integrity_scope_budget`
- `build`

This builder is already the main DX control surface for runtime construction.

## Runtime Object Surface

Core runtime type:

- [RelationalRuntime](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/logic/runtime/state/runtime_state.rs)

Direct public methods on `RelationalRuntime`:

- `config`
- `commit_strategy_registry`
- `commit_strategies`
- `commit_strategies_authority`

Additional public runtime methods are attached from subsystem access and authority impls:

- `history_access`
- `history_authority`
- `durability_access`
- `durability_authority`
- `index_access`
- `index_authority`
- `inspection_access`
- `performance_access`
- `publication_access`
- `replay_access`
- `simulation_access`
- `simulation_authority`
- `storage_access`
- `storage_authority`
- `retention_authority`
- `merge_access`
- `prepare_merge_execution`
- `execute_prepared_merge`

## Read Access Facets

### History

- [HistoryAccess](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/history/logic/access.rs)
  - `latest_commit`
  - `branch_head`
  - `branches`
  - `version_graph`
  - `ancestor_closure_by_commit_id_order`
  - `latest_common_ancestor_between_branches`
  - `can_merge_branch_into`
  - `inspect_merge`
  - `entity_aspect_history`
  - `relation_aspect_history`
  - `entity_aspect_history_with_trace`
  - `relation_aspect_history_with_trace`
  - `lineage_entity_aspect_history`
  - `lineage_entity_aspect_history_with_trace`

### Durability

- [DurabilityAccess](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/durability/access.rs)
  - `recovery_plan`
  - `durable_log`
  - `durable_branch_heads`

### Indexes

- [IndexAccess](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/indexes/logic/access.rs)
  - `latest_generation`
  - `generations_for_version`
  - `execute_query_plan_with_fallback_parity`

### Inspection

- [InspectionAccess](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/inspection/logic/access.rs)
  - root accessor only lives here

Additional inspection methods are spread across focused modules under [inspection/logic](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/inspection/logic):

- [graph.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/inspection/logic/graph.rs)
  - `graph_summary`
  - `kind_summary`
- [connectivity.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/inspection/logic/connectivity.rs)
  - `connectivity_summary`
  - `neighbors`
- [historical.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/inspection/logic/historical.rs)
  - `open_historical_view`
  - `inspect_historical_record`
- [retention.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/inspection/logic/retention.rs)
  - `retention_summary`
  - `inspect_record_retention`
  - `inspect_snapshot_pinning`
  - `inspect_retention_execution`
- [commit.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/inspection/logic/commit.rs)
  - `inspect_commit`
  - `inspect_recent_commits`
  - `inspect_branch_head`
- [structural_identity.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/inspection/logic/structural_identity.rs)
  - `structural_identity`
  - `compare_structural_identity`
  - `query_structural_identity`

### Performance

- [PerformanceAccess](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/performance/logic/access.rs)
  - `contracts`
  - `counters`
  - `reset_counters`

### Publication

- [PublicationAccess](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/publication/logic/access.rs)
  - `diagnostics`
  - `diagnostic_artifacts`
  - `diagnostics_since`
  - `latest_bundle`
  - `latest_patch`
  - `latest_replay`
  - `read_patch_stream`
  - `read_subscriber_stream`

### Replay

- [ReplayAccess](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/replay/logic/access.rs)
  - `canonical_commit_envelope`
  - `compare_outcome`

### Simulation

- [SimulationAccess](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/simulation/logic/access.rs)
  - `compiled_artifact`
  - `compiled_artifact_compatibility`

### Storage

- [StorageAccess](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/storage/logic/access.rs)
  - `partition_ids`
  - `partition_storage_stats`
  - `storage_stats`
  - `chunked_storage_summary`
  - `chunk_diagnostics`
  - `plan_read_explicit_query_packet`
  - `outgoing_relations_for_entity`
  - `incoming_relations_for_entity`
  - `all_relations_for_entity`

## Authority And Mutating Facets

### HistoryAuthority

- [authority.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/history/logic/authority.rs)
  - `create_branch`
  - `retain_version_for_replay`
  - `release_version_replay_retention`

### DurabilityAuthority

- [authority.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/durability/authority.rs)
  - `checkpoint`
  - `compact_store`
  - `recover`

### IndexAuthority

- [authority.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/indexes/logic/authority.rs)
  - `register`
  - `build_for_commit`

### SimulationAuthority

- [authority.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/simulation/logic/authority.rs)
  - `compile_execution_artifact`

### VisibilityRetentionAuthority

- [retention_authority.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/visibility/retention/retention_authority.rs)
  - `inspect_plan`
  - `run_pass`

### Merge

- [facade.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/merge/facade.rs)
  - runtime methods:
  - `merge_access`
  - `prepare_merge_execution`
  - `execute_prepared_merge`

- [merge/logic/mod.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/merge/logic/mod.rs)
  - `MergeAccess::runtime`
  - `MergeAccess::inspect_history_scope`
  - `MergeAccess::inspect_planning_scope`

### Commit Strategy Facets

- [commit_strategies/facade.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/commit_strategies/facade.rs)

Read-side:

- `CommitStrategiesFacade::canonicalize_request`
- `CommitStrategiesFacade::execute`

Authority-side:

- `CommitStrategiesAuthorityFacade::lower_execution`
- `CommitStrategiesAuthorityFacade::execute_lowered_commit`
- `CommitStrategiesAuthorityFacade::validate_lowered_plan`
- `CommitStrategiesAuthorityFacade::execute_validated_commit`

## Transactions Surface

The transaction surface is currently exposed primarily through re-exported data plus `RelationalTransaction`.

Primary public transaction types are exported from [transactions::facade](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/transactions/facade.rs) and [facade.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/facade.rs).

The main input and outcome categories include:

- transaction options
- worker intent batches
- mutation intents
- bulk entity and bulk relation create intents
- update, replace, delete intents
- commit result and commit outcome
- commit phase timing
- commit structural, schema, publication, and history summaries
- merge execution outcome and merge execution summaries
- rollback outcome and rollback summary

This is one of the broadest user-facing data surfaces in the crate.

## Configuration Surface Inventory

Primary config root:

- [RelationalRuntimeConfig](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/config/data/runtime_config.rs)

Primary config profile and policy types:

- [policies.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/config/data/policies.rs)
  - `RelationalRuntimeProfile`
  - `MvccConfig`
  - `StorageLayoutConfig`
  - `PublicationConfig`
  - related durability, retention, visibility, adjacency, and publication policies re-exported through [facade.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/facade.rs)

Sectioned runtime config:

- [sections.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/config/data/sections.rs)
  - `ExecutionConfig`
  - `DiagnosticsConfig`
  - `HistoryConfig`
  - `SchemaConfig`
  - `CommitStrategiesConfig`
  - `IdentityConfig`
  - `StorageConfig`
  - `VisibilityConfig`
  - `PublicationRuntimeConfig`
  - `DurabilityConfig`
  - `RelationIntegrityScopeBudget`

Override surface:

- [overrides.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/config/data/overrides.rs)
  - `ExecutionConfigOverride`
  - `DiagnosticsConfigOverride`
  - `HistoryConfigOverride`
  - `SchemaConfigOverride`
  - `CommitStrategiesConfigOverride`
  - `IdentityConfigOverride`
  - `StorageConfigOverride`
  - `VisibilityConfigOverride`
  - `PublicationConfigOverride`
  - `DurabilityConfigOverride`
  - `RelationalConfigOverride`

Config provenance surface:

- [provenance.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/config/data/provenance.rs)
  - `ConfigValueSource`
  - `ConfigProvenanceEntry`
  - `ConfigProvenance`
  - `ConfigProvenance::source_for`

Resolved profile preset surface:

- [profiles/mod.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/config/data/profiles/mod.rs)
  - `RelationalRuntimeConfig::resolved`

## Re-Exported Public Domains

These domains are part of the externally reachable public surface even when most of their call sites are currently data-driven rather than method-heavy.

- diagnostics
- durability
- errors
- history
- identity
- inspection
- indexes
- lineage
- merge
- payloads
- publication
- query
- replay
- schema
- snapshots
- storage
- symbols
- transactions

Validation-related concepts are still publicly reachable, but today they are exposed mostly through:

- `runtime` re-exports such as invariant catalog and invariant rule types
- schema and transaction summaries
- builder registration surfaces for custom invariants

The most consequential public data-heavy areas right now are:

- schema
- merge
- transactions
- commit strategies
- durability
- inspection

## What This Means For DX Later

The API surface is already broad enough that DX hardening should not start with random renames or piecemeal method cleanup.

The main consolidation opportunities appear to be:

1. runtime entrypoint clarity
- `RelationalRuntimeApi`
- `RelationalRuntimeBuilder`
- direct `RelationalRuntime` methods
- subsystem accessors and authorities

2. inspection surface cohesion
- inspection methods are intentionally split by concern, but the user-facing surface is spread across multiple implementation files

3. transaction and commit-strategy overlap
- there is a lot of power here, but the distinction between raw transaction flow, lowered strategy flow, and validated strategy flow will likely need a clearer DX story

4. config layering clarity
- profile
- resolved config
- overrides
- builder overrides
- provenance

That layering is strong for correctness and observability, but it is a future DX simplification target.

## Recommended Next Planning Move

Before the next optimization wave:

1. turn the coverage matrix above into an explicit checklist
2. add dedicated perf families for geometry, chip, and sustained load
3. keep this API inventory updated whenever a new public facet or config section appears
4. use this document as the reference map for later DX cleanup so performance and API simplification stay aligned

## Execution Checklist

This is the current execution-order checklist for closing the highest-value coverage gaps.

### Phase 1

- [x] Add `geometry_kernel_matrix`
- [x] Add a persisted topology identity and recovery round trip case
- [x] Add an in-memory topology edit plus connectivity summary case
- [x] Add thin versus rich geometry-profile variants where the workload shape stays the same

### Phase 2

- [x] Add `chip_simulator_matrix`
- [x] Add dense fanout update wave coverage
- [x] Add branch and rollback stepping coverage
- [x] Add checkpoint-window stepping coverage
- [x] Add thin versus audit-rich simulator-profile variants

### Phase 3

- [x] Add `sustained_load_matrix`
- [x] Add repeated commit and query churn
- [x] Add replay-window drift tracking
- [x] Add retention-pass drift tracking
- [ ] Add broader long-run packet and scope stability checks

### Phase 4

- [x] Add `inspection_budget_matrix`
- [x] Add `index_parity_matrix`
- [x] Add `mixed_load_matrix`
- [ ] Add dedicated invariant-materialization perf isolation

### Maintenance Rule

- [x] Whenever a new performance family lands, add its status and cases back into this document
- [ ] Whenever a new public config or facade surface lands, update the API inventory in this document in the same change
