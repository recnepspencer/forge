# forge-relational Current Architecture

This document is the current-reference description of how `forge-relational`
works today.

It is not a refactor spec, a future-phase sketch, or a historical journal. It
describes the runtime as it exists in code now: its public surface, its major
internal contracts, the shape of its artifacts, and the boundaries that are
important for future work.

Use this document when you need to understand:

- what the runtime owns
- which types are authoritative
- how callers are expected to enter and read the system
- what commit, replay, diagnostics, lineage, and durability artifacts exist
- which surfaces are canonical
- which future-looking concepts are real contracts today and which are only
  declaration surfaces

If this document and older milestone language disagree, treat this document as
the current behavioral reference.

## Status

As of March 12, 2026:

- the architecture program through Phase F is closed enough to treat the core
  runtime shape as established
- Milestone 4 (Invariant Engine), Milestone 5 (Commit Architecture), and
  Milestone 6 (API Surface) are closed
- the next major work is feature expansion, hardening, scale proof, and
  parallel-preparation work, not more foundational cleanup

## Core Runtime Model

`forge-relational` is a serialized-authority truth runtime.

The governing rule is:

parallelize disposable work, serialize authority.

In current implementation terms, that means:

- authoritative truth mutation flows through transactions and one logical commit
  boundary
- committed snapshots are immutable and readable while later mutation proceeds
- observable outputs are canonical and deterministic
- commit publication is coherent across snapshot, patch, diagnostics, and replay
  artifacts
- replay reconstructs from canonical commit envelopes rather than from internal
  heap state

The runtime is meant to be reusable as a standalone truth-state library. It is
not defined as a bridge helper, a kernel-only implementation detail, or a
reactive scheduler.

## Public Surface

The main public boundary is [`src/facade.rs`](../../crates/forge-relational/src/facade.rs).

The namespaced facade is the public surface.

### Namespaced facade groups

The current domain namespaces are:

- `config`
- `diagnostics`
- `durability`
- `errors`
- `history`
- `identity`
- `indexes`
- `lineage`
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

These namespaces map closely to actual ownership or contract domains. New code
should import through them.

### Runtime entrypoints

The simplest public entrypoints are:

- `RelationalRuntimeApi::builder()`
- `RelationalRuntimeApi::runtime()`

`RelationalRuntimeApi::builder()` returns `RelationalRuntimeBuilder`, which is
the intended construction API.

The builder currently exposes explicit configuration setters for:

- profile selection
- runtime name
- execution model
- planning contract
- commit authority contract
- durability mode
- diagnostics profile
- schema registry
- invariant catalog
- entity/relation capacity
- MVCC config
- storage layout
- publication policy
- payload policy
- symbol policy
- visibility cache policy
- durable log policy
- durability policy
- durable store layout
- adjacency policy
- cross-context policy
- cascade delete policy
- compiled lane policy

The builder resolves a `RelationalRuntimeConfig` and constructs a
`RelationalRuntime`.

## Configuration Model

Configuration is sectioned by subsystem rather than flattened into one large
bag.

The important exported configuration families are:

- `ExecutionConfig`
- `DiagnosticsConfig`
- `DurabilityConfig`
- `HistoryConfig`
- `IdentityConfig`
- `PublicationRuntimeConfig`
- `SchemaConfig`
- `StorageConfig`
- `VisibilityConfig`

Policy types are grouped under `config`, including:

- `RelationalRuntimeProfile`
- `MvccConfig`
- `PublicationConfig`
- `DurabilityPolicy`
- `DurableLogPolicy`
- `VisibilityCachePolicy`
- `StorageLayoutConfig`
- `AdjacencyPolicy`
- `CrossContextPolicy`
- `CascadeDeletePolicy`
- `SnapshotReleasePolicy`
- `RetentionPolicy`
- `RetentionBackend`
- `CompiledLanePolicy`

Configuration provenance is also first-class:

- `ConfigProvenance`
- `ConfigProvenanceEntry`
- `ConfigValueSource`

That provenance shows whether a value came from a profile default, builder
override, or another explicit resolution source.

## Identity Model

Identity is generational, partition-aware, and split by record class.

Current public identity types include:

- `EntityId`
- `RelationId`
- `EntityStorageId`
- `RelationStorageId`
- `PartitionId`
- `Generation`
- `KindId`
- `VersionId`
- `LineageId`
- `LocalSlot`
- `StructuralFingerprint`
- `VersionBound`

Important architectural truths:

- entity identity and relation identity are separate
- generational reuse is part of the authoritative identity model
- partition locality is embedded in the identity form
- stale-handle rejection is part of normal correctness, not an edge case

Performance-critical code should continue to treat these compact generational
IDs as the authoritative identity form.

## Runtime Shape and Subsystem Boundaries

`RelationalRuntime` is the main runtime object, but it is no longer intended to
be treated as one undifferentiated god object.

The crate exports subsystem-shaped access and authority surfaces through the
runtime, and new code should prefer those surfaces over direct field reach
through.

Important subsystem families present in the runtime include:

- history
- visibility
- publication
- durability
- lineage
- indexing
- instrumentation / performance

Representative access patterns used throughout the codebase today are:

- `history_access()` / `history_authority()`
- `visibility_reads()` / `visibility_authority()`
- `publication_access()`
- `durability_access()` / `durability_authority()`
- `lineage_access()` / `lineage_authority()`
- `index_access()` / `index_authority()`
- `storage_access()`
- `retention_access()`
- `performance_access()`

The architectural rule is:

- use access surfaces for committed observation
- use authority surfaces for mutation or publication/recovery transitions
- do not blur committed reads, speculative reads, and mutation authority into
  one generic state bag

## Transaction and Commit Model

Authoritative mutation flows through transactions.

The main public transaction-side types are:

- `RelationalTransaction`
- `TransactionId`
- `TransactionOptions`
- `SavepointId`
- `WorkerIntentBatch`
- `MutationIntent`
- `CreateIntent`
- `EntityMutationIntent`
- `RelationMutationIntent`
- `BulkEntityCreateIntent`
- `BulkRelationCreateIntent`
- `UpdateEntityIntent`
- `DeleteEntityIntent`
- `DeleteRelationIntent`
- `ReplaceEntityIntent`
- `RecordRef`

### Bulk-first write shape

Bulk work is represented explicitly in the public intent model. That matters
because the runtime treats bulk domains as real bulk domains rather than as
scalar loops hidden behind a cheap-looking API.

Examples:

- `WorkerIntentBatch`
- bulk create intents
- merged commit planning
- commit structural summaries
- patch and change summaries

## Commit Artifacts

Milestone 5 made commit output reconstructive instead of flattening it into one
bag.

### CommitOutcome

`CommitOutcome` is the baseline successful commit result:

- `transaction_id`
- `commit: CommitReference`
- `version_id`
- `snapshot: SnapshotHandle`
- `changed_records: Vec<RecordRef>`
- `publication_status`
- `commit_log: CommitLog`

This is the minimal authoritative outcome shape for successful commit
publication.

### CommitResult

`CommitResult` is the richer reconstructive envelope layered on top of
`CommitOutcome`.

It contains:

- `outcome: CommitOutcome`
- `summary: CommitSummary`
- `structural_summary: CommitStructuralSummary`
- `publication: CommitPublication`
- `validation: CommitValidation`
- `execution: CommitExecution`

Important accessors exposed directly from `CommitResult` include:

- `commit_log()`
- `commit_summary()`
- `publication()`
- `structural_summary()`
- `history_summary()`
- `patch_budget_summary()`
- `change_summary()`
- `publication_summary()`
- `validation()`
- `execution()`
- `diagnostics()`
- `patch()`
- `envelope()`
- `invariant_executions()`
- `validation_summary()`
- `phase_timing()`
- `complexity_delta()`
- `patch_position()`
- `final_snapshot_id()`
- `merge_parent_count()`

The important design rule is that downstream consumers should usually consume
these result artifacts directly rather than rescanning raw checks, raw patch
records, or raw trace events.

### Commit structural and summary families

Current summary artifact families include:

- `CommitStructuralSummary`
- `CommitHistorySummary`
- `CommitPatchBudgetSummary`
- `CommitChangeSummary`
- `CommitPublicationSummary`
- `CommitValidationSummary`
- `CommitSummary`

`CommitStructuralSummary` currently carries:

- `invariant_groups`
- `commit_topology`
- `touched_partitions`
- reserved bulk entity slot count
- reserved bulk relation slot count

These summaries are intended to be producer-owned or framework-owned proof
forms, not downstream re-derivations.

### Commit publication artifacts

`CommitPublication` currently contains:

- `diagnostics: Vec<RelationalDiagnosticArtifact>`
- `envelope: Arc<CanonicalCommitEnvelope>`

The canonical envelope is the important durable/replay/publication artifact. It
is shared where appropriate and exposed by reference at the public boundary.

### Commit log

`CommitLog` is still present as the forensic trace surface.

It owns a `CommitSummary` and a trace-event stream (`CommitTraceEvent`), but the
current architecture avoids duplicating heavy summary artifacts into both places
where unnecessary. The running summary is the main summary authority; the event
stream exists for ordered forensic tracing.

### Rollback artifacts

Rollback is also modeled reconstructively:

- `RollbackEffect`
- `RollbackSummary`
- `RollbackOutcome`

`RollbackSummary` is derived from explicit rollback effects and exposes counts
for restored records and discarded creations. Consumers should prefer the
summary for routine interpretation and the effects vector for deeper inspection.

## Validation and Invariant Model

The invariant path is explicit, phase-typed, and reconstructive.

Important public/runtime-facing invariant types include:

- `InvariantCatalog`
- `InvariantRegistration`
- `InvariantRule`
- `InvariantClass`
- `InvariantExecutionPoint`
- `InvariantFailureEffect`
- `InvariantCheckResult`

The engine now returns `InvariantExecutionResult`, not just a raw list of check
results.

### Invariant execution metadata

`InvariantExecutionMetadata` includes:

- execution point
- observation kind
- target version
- current version
- consumed groups
- applicable groups
- max cost
- execution disposition
- optional plan contract
- whether a merged plan backed the request

Execution disposition is explicit:

- `Executed`
- `SkippedByPlanContract`
- `SkippedByMayBreakMask`

Observation kind is explicit:

- committed
- speculative

This is a real architectural contract, not just a convention over one generic
state-plus-enum API.

### Invariant execution summary

`InvariantExecutionSummary` carries:

- result count
- advisory count
- violation count
- first blocking failure, if any
- first publication failure, if any

This allows commit/publication consumers to route failures without rescanning
raw invariant checks themselves.

### Important honesty point

`ProjectionAspect` and projection `required_aspects()` are real declaration
surfaces today.

What they are not yet:

- an already-wired bridge invalidation system
- an active framework-owned projection cache invalidation contract
- a completed operational read/write intersection path outside the invariant
  declaration story

The current code establishes explicit read-contract declaration. It does not yet
implement the full bridge-style invalidation system implied by that declaration.

## Snapshot and Read Model

Committed reads are immutable.

The primary snapshot types are:

- `SnapshotId`
- `SnapshotHandle`
- `SnapshotReadPolicy`
- `SnapshotInspectionSummary`

`SnapshotHandle` contains:

- `snapshot_id`
- `version_id`
- `read_policy`

Current snapshot read policies exported publicly are:

- `ImmutablePinned`
- `ImmutablePinnedNoLazyMutation`

### Storage-visible read records

The canonical read records are:

- `EntityReadRecord`
- `RelationReadRecord`

They carry:

- typed record ID
- kind resolution
- lifecycle state
- created-at version
- optional retired-at version
- payload
- for relations, explicit `source` and `target`

### Record lifecycle

The public lifecycle enum is explicit:

- `Live`
- `DeletedRetained`
- `RetainedDanglingForAudit`
- `PinnedBySnapshot`
- `PinnedByBranch`
- `PinnedByReplayRetention`
- `Reclaimable`
- `Reusable`

That lifecycle vocabulary is part of the reference model. It should not be
collapsed back into vague tombstone language.

### RelationalReadView

`RelationalReadView` still exists and remains a valid committed read surface.

It currently provides:

- `snapshot()`
- `entities()`
- `relations()`
- `execute_packet(...)`
- `get_entity(...)`
- `get_relation(...)`

This is still the honest surface for callers that truly need a whole snapshot
read materialized as entities plus relations.

## Query and Packet Model

The primary public query surface is packetized.

Important query types are:

- `QueryWorkPacket`
- `PartitionHint`
- `QueryExecutionShape`
- `ReductionDiscipline`
- `ReadPacketPlan`
- `PacketResult`

The important architectural direction is:

- bulk packetized reads are the main public read shape
- per-ID convenience exists, but should not replace the packet model

`QueryWorkPacket::bulk(...)` creates a bulk packet with:

- label
- optional partition hint
- bulk execution shape
- deterministic reduction discipline
- target record refs

`ReadPacketPlan` exposes the storage planning summary for a packet, including
chunk indexes and target count.

## Projection Read Surface

The type-driven projection read API is now the primary ergonomic read surface.

### Projection declarations

The relevant public/runtime types are:

- `ProjectionAspect`
- `EntityRecordProjection`
- `RelationRecordProjection`
- `VisibilityProjectionView`

`ProjectionAspect` is a lightweight named declaration:

- it is constructed with `ProjectionAspect::new("name")`
- it exposes `name()`

Projection traits declare:

- `const KIND: KindId`
- `required_aspects() -> &'static [ProjectionAspect]`
- `from_record(...) -> Option<Self>`

### VisibilityProjectionView

`VisibilityProjectionView` is version-scoped.

It currently exposes:

- `version_id()`
- `entities::<T>()`
- `entities_in::<T>(partition_id)`
- `entity::<T>(entity_id)`
- `relations::<T>()`
- `relations_in::<T>(partition_id)`
- `relation::<T>(relation_id)`
- `entity_records(kind_id)`
- `entity_records_in(partition_id, kind_id)`
- `relation_records(kind_id)`
- `relation_records_in(partition_id, kind_id)`
- `all_entity_records()`
- `all_relation_records()`

Important ordering/cost truths:

- typed aggregate projections preserve deterministic ordering
- `all_entity_records()` and `all_relation_records()` are honest escape hatches
  for consumers that really need full visible record sets
- lower-level record scans still exist underneath, but they are subordinate
  infrastructure now rather than the preferred API

`visibility_reads()` currently exposes:

- `project_version(version_id)`
- `project_snapshot(snapshot_handle)`

Those are the primary entrypoints for projection reads.

## Publication and Patch Model

Publication is coherent and commit-native.

### Publication bundle

The main publication types are:

- `PublicationBundle<ReplayRecord>`
- `PublicationStatus`
- `PublicationStage`
- `PublicationError`

A publication bundle contains:

- `commit`
- `snapshot`
- `diagnostics_summary`
- `patch`
- `replay`
- `status`

### Patch model

Patch-related public types include:

- `RelationalPatchRecord`
- `PatchRecord`
- `PatchRecordKind`
- `PatchDetail`
- `AspectKey`
- `PatchOrdering`
- `PatchPublicationMode`
- `PatchStreamPosition`
- `PatchFragmentBudget`
- `PatchStreamRequest`
- `PatchStreamBatch`
- `PatchStreamReadError`
- `PatchStreamReadErrorClass`

Current patch truths:

- patch ordering is explicit (`CanonicalCommitOrder`)
- patch publication mode is explicit (`CommitNative`)
- patch compatibility is explicit
- patch records carry target record ref, kind, aspects, and detail
- canonicalization of patch content is a first-class concern

The patch stream supports explicit resume semantics:

- `after_position`
- `max_commits`
- returned `resumed_after`
- returned `next_position`
- returned `latest_position`
- explicit `UnknownResumePosition` and `InvalidBatchSize` failures

## Replay Model

Replay is canonical-envelope driven.

### Canonical commit envelope

`CanonicalCommitEnvelope` currently contains:

- `commit: CommitReference`
- `branch_context`
- `merge_parent_branches`
- `merge_base_commits`
- `schema_version`
- `schema_registry`
- `merged_plan`
- `patch`
- `diagnostics_summary`
- `lineage_event_ids`
- `index_generation_ids`

This is the canonical replay and durable truth artifact. It is not a dump of
internal heap state.

### Replay request and outcome

The main replay types are:

- `RelationalReplayRequest`
- `RelationalReplayOutcome`
- `ReplayExecutionMode`
- `ReplayObservableSurface`
- `ReplayMismatch`
- `ReplayMismatchClass`
- `ReplayError`
- `ReplayFailureClass`
- `ReplaySnapshotSurface`

`RelationalReplayRequest` carries:

- target commit ID
- target branch ID
- execution mode

`RelationalReplayOutcome` carries:

- requested replay request
- optional resolved commit
- reconstructed parent chain
- optional snapshot version
- compared surfaces
- mismatch list
- optional failure class

Current replay observable surfaces are explicit:

- snapshot
- patch
- diagnostics
- history
- branch head
- lineage
- derived indexes

Replay mismatch classes are explicit too:

- `PatchDrift`
- `DiagnosticsDrift`
- `HistoryDrift`
- `SnapshotDrift`
- `BranchHeadDrift`
- `LineageDrift`
- `DerivedIndexDrift`

`ReplaySnapshotSurface` is the current full observable snapshot comparison
surface used for replay drift checks. Replay intentionally uses a dedicated full
observable-state surface rather than pretending that a generic query read view
is the exact same contract.

## History and Branching

History is branch-aware and merge-ready.

Important public history types include:

- `CommitId`
- `BranchId`
- `CommitReference`
- `BranchHead`
- `VersionNode`
- `VersionGraphSnapshot`
- `MergeConflictRecord`
- `MergeInspection`
- `VersionGraphPolicy`
- `HistoryRetentionClass`

Important current truths:

- `CommitReference` includes ordered parent commit IDs
- branch heads are explicit
- version graph snapshots are explicit
- merge inspection is structured rather than ad hoc
- merge-ready parent ordering is already part of the model even before any
  broader future merge work

`MergeInspection` currently includes:

- source branch
- target branch
- optional source head
- optional target head
- optional merge base
- source-only commits
- target-only commits
- conflicting records
- `can_merge`

## Lineage and Correspondence

Lineage is modeled as a real graph domain, not as freeform event logging.

Current public lineage types include:

- `LineageEventKind`
- `LineageNode`
- `LineageEventRecord`
- `LineageInvariant`
- `LineageResolutionStatus`
- `CorrespondenceCandidate`
- `CorrespondenceResolution`
- `LineageGraphSnapshot`
- `LineageDivergenceSummary`
- `HistoricalLineageResolution`

Current event kinds are:

- `Create`
- `Replace`
- `Split`
- `Merge`
- `Retire`
- `Correspond`

Important truths:

- storage identity and lineage identity remain separate
- correspondence starts advisory and can later be promoted
- lineage snapshots are branch-scoped
- historical lineage resolution is explicit

## Indexing Model

Derived indexes are non-authoritative read-side helpers.

Current public index families include:

- `DerivedIndexDefinition`
- `DerivedIndexId`
- `DerivedIndexKind`
- `DerivedIndexGeneration`
- `DerivedIndexGenerationId`
- `DerivedIndexPayload`
- `DerivedIndexPublicationStatus`
- `DerivedIndexBuildRequest`
- `DerivedIndexBuildOutcome`
- `DerivedIndexCompatibility`
- `ReadWithStorageFallbackOutcome`

The important contract is unchanged:

- indexes may accelerate reads
- indexes are not authority
- storage-visible fallback remains available
- index absence or mismatch must not change truth semantics

## Durability and Recovery

Durability persists canonical truth artifacts rather than transient arena
layout.

Current public durability types include:

- `DurabilityMode`
- `DurableStoreLayout`
- `DurableStore`
- `DurableSegmentManifest`
- `DurableCheckpointManifest`
- `DurableCheckpoint`
- `DurableCheckpointId`
- `DurableSegmentId`
- `RecoveryPlan`
- `RecoveryCursor`
- `RecoveryCoverage`
- `RecoveryCompatibilityCheck`
- `RecoveryIntegrityReport`
- `RecoveryFailureClass`
- `CompactionPlan`
- `CompactionOutcome`
- `CompactionPolicy`
- `CheckpointCoverage`
- `SegmentRetentionClass`

Current durability truths:

- recovery rebuilds from canonical envelopes and durable history artifacts
- persisted durability does not define truth differently than in-memory
  authority; it changes storage mode, not semantic authority
- checkpoint and tail-log recovery are explicit
- corruption fallback is structured and diagnosable

## Diagnostics

Diagnostics are production infrastructure.

The main exported diagnostics families are:

- `RelationalDiagnosticArtifact`
- `RelationalDiagnosticsEntry`
- `RelationalDiagnosticsProfile`
- `DiagnosticsScope`
- `DiagnosticsArtifactKind`
- `DiagnosticCode`
- `DeterminismExpectation`
- `RelationalDiagnosticsFacade`

Diagnostics artifacts are used throughout commit, publication, replay, and
other subsystem flows. The runtime treats them as stable structured artifacts,
not as debug-only strings.

## Harness Surface

The relational harness is part of the runtime contract, not a side test helper.

Current harness exports include:

- `RelationalHarnessAdapter`
- `RelationalHarnessPlan`
- `RelationalHarnessExpectations`
- `RelationalHarnessError`
- `RelationalFixture`
- `FixtureEntity`
- `FixtureRelation`
- `default_harness_expectations()`

The harness surface exists to drive:

- fixture setup
- mutation batch execution
- target capture
- parity and replay-oriented scenario work

The fintech domain harness built in tests is currently one of the most serious
examples of that acceptance path.

## Performance and Introspection Surfaces

The runtime exports performance/introspection data rather than forcing all
inspection through ad hoc debugging.

Important exported families include:

- `RuntimeComplexityCounters`
- `ComplexityContract`
- `ComplexityStatus`
- `StorageStats`
- `PartitionStorageStats`
- `RetentionPlan`
- `RetentionPassOutcome`
- `ChunkDiagnostics`
- `ChunkVisibilitySummary`
- `ChunkedStorageSummary`

These surfaces are part of how the runtime proves cost boundaries and storage
behavior.

## Observation, Authority, and Phase Honesty

The codebase now follows a stronger phase distinction than older versions:

- committed reads use committed read surfaces
- speculative/working-state observation is explicit in validation and write-path
  flows
- mutation authority remains explicit and serialized
- replay, publication, and recovery have their own artifact surfaces

Avoid reintroducing APIs that erase these boundaries into one generic
state-and-mode object.

## Current Limits

### Projection invalidation

`ProjectionAspect` is real today as a declaration mechanism.

Current limit:

- the runtime does not yet expose a completed projection-cache invalidation or
  bridge-layer intersection system that operationally uses those declarations

Do not document or implement as if this already exists.

### Lower-level helper survival

Some lower-level read and scan helpers still exist underneath the projection and
packet layers.

Current policy:

- they remain valid infrastructure and escape hatches
- they are not the preferred ergonomic boundary anymore

## What Future Work Should Assume

Future work should assume the following as stable architectural ground:

- namespaced facade is the primary public and internal default surface
- commit results are reconstructive artifact envelopes
- replay is envelope-driven
- publication is coherent
- lineage and derived indexes are explicit domains
- projection reads are the primary ergonomic read API
- bulk packetized reads are the primary query API
- authority, committed observation, and speculative observation must stay
  explicit

Future work should not assume the following are already solved:

- full projection/bridge invalidation wiring
- completed parallel validation and parallel preparation runtime machinery
- complete scale proof for geometry-kernel, chip-simulator, or AI-scale loads
- full hardening coverage implied by the long-term vision

## Practical Reading Order

If you are new to the crate and want to understand the current system quickly,
read in this order:

1. [`src/facade.rs`](../../crates/forge-relational/src/facade.rs)
2. [`src/transactions/data/outcomes.rs`](../../crates/forge-relational/src/transactions/data/outcomes.rs)
3. [`src/transactions/data/commit_log.rs`](../../crates/forge-relational/src/transactions/data/commit_log.rs)
4. [`src/replay/data/mod.rs`](../../crates/forge-relational/src/replay/data/mod.rs)
5. [`src/visibility/materialization/read_records/projection.rs`](../../crates/forge-relational/src/visibility/materialization/read_records/projection.rs)
6. [`src/storage/data/mod.rs`](../../crates/forge-relational/src/storage/data/mod.rs)
7. [`src/history/data/mod.rs`](../../crates/forge-relational/src/history/data/mod.rs)
8. [`src/lineage/data/mod.rs`](../../crates/forge-relational/src/lineage/data/mod.rs)
9. [`src/config/data/mod.rs`](../../crates/forge-relational/src/config/data/mod.rs)
10. the highest-signal domain tests, especially the fintech workflow harness

That sequence gives the current runtime story with minimal archaeological work.
