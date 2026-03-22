# S9 Missing-Substrate Completion Spec

## Summary

This document defines the completion work for the remaining unfinished substrate across `S9.9`, `S9.10`, `S9.12`, and `S9.15`.

These are not polish tasks. They are the remaining places where the runtime is architecturally honest about incompleteness, but not yet complete:

- `S9.12`: reconstructability is improved, but restore still is not fully forced through a canonical proof chain of `checkpoint + bounded journal + required derived rebuild`
- `S9.15`: whole-live merge fallback is gone, but bounded merge proof is not yet complete enough to make all supported merge flows purely proof-driven
- `S9.9`: disjoint parallel structure exists, but true grouped concurrent apply on the mutable engine is not yet complete
- `S9.10`: rollback and lifecycle are improved, but rollback is not yet fully reduced to typed inverse authority effects and lifecycle transfers are not yet completely type-separated

This spec is governed by:

- [MENTALITY.md](/C:/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/coding_guidelines/MENTALITY.md)
- [architectural_guidelines.md](/C:/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/coding_guidelines/architectural_guidelines.md)
- [performance_guidelines.md](/C:/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/coding_guidelines/performance_guidelines.md)
- [signal_architecture2.md](/C:/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge_signal/signal_architecture2.md)

Primary adversarial constraint:

- under long-lived branch/snapshot churn, geometry-kernel-scale graphs, hostile restore/merge/replay workloads, and disjoint parallel batches, the runtime must preserve identical authoritative truth and semantically required derived truth while keeping hot-path breadth bounded by semantic delta and forbidding whole-live fallback, fake parallelism, policy-driven semantic drift, or baseline-bundle rollback as supported behavior

Implementation order:

1. shared proof substrate laws
2. `S9.12` reconstructability completion
3. `S9.15` bounded merge completion
4. `S9.9` true parallel apply completion
5. `S9.10` rollback and lifecycle completion
6. cross-phase certification closeout

## Shared Proof Substrate Laws

These laws are mandatory for every proof-bearing object introduced by this work. This section exists to prevent each subsystem from inventing its own private notion of "proof."

### 1. Proofs are boundary products, not late orchestration guesses

A proof object must be produced directly from canonical mutation-time, checkpoint-time, or lifecycle-time artifacts. It must not be synthesized later from arbitrary current state.

Allowed:

- merge proof derived from mutation ledger boundaries, merge-base checkpoint truth, and proof-authorized overlap indexes maintained from the same journal semantics
- reconstructability proof derived from checkpoint capture plus bounded retained journal segment
- rollback packets emitted during effect application
- disjoint apply proof derived from lowered task footprints against a specific planning basis

Forbidden:

- proof assembled by broad branch inspection after the fact
- proof inferred from convenience caches
- proof reconstructed from diagnostic summaries
- proof that depends on arbitrary ambient whole-live state discovery

### 2. Every proof type must encode five things

Every proof-bearing type must explicitly encode or imply through its type family:

- authority scope
  - what authority surface, branch, checkpoint, stage, or mutation basis it certifies
- boundedness scope
  - what exact breadth, cursor window, candidate set, or touched-surface set it certifies
- continuity basis
  - what chain or boundary makes the proof valid
- invalidation rule
  - what mutation or lifecycle event makes it stale
- reuse model
  - whether it is consumable once, inspectable, replay-stable, persistent until mutation, or snapshot-stable

### 3. Descriptive metadata is never admissibility proof

A struct with fields and labels is not proof unless supported execution can proceed from it without broad rediscovery.

If execution still needs:

- whole-branch inspection
- whole-graph rescans
- speculative runtime validation
- hidden "just to be safe" broad reconstruction

then the object is descriptive metadata, not admissibility proof.

### 4. Supported execution may consume only proof-authorized indexes

A proof-bearing path may consume indexes only if those indexes are themselves maintained from the same canonical mutation, checkpoint, or journal semantics that define the proof.

Allowed:

- branch overlap index maintained from mutation journal semantics
- dependency snapshot index maintained from authoritative dependency snapshot updates
- replay suffix index maintained from retained replay/journal packets

Forbidden:

- convenience branch index that silently widens merge planning
- whole-live helper index that effectively reintroduces broad branch scans
- diagnostic-only index used to satisfy semantic admissibility

### 5. Proofs must be replay-verifiable where the phase requires replay truth

For reconstructability, merge, and rollback, proof must be verifiable against retained canonical packets. Planner confidence is not enough.

### 6. Supported execution must consume lowered proof-bearing packets only

This follows Architectural Law 30 directly.

For every phase:

- phase `K` output type must be the only acceptable input to phase `K+1`
- raw collections, mutable state bundles, and convenience structs must not bypass the proof chain
- executors must not re-decide strategy, admissibility, or boundedness

### 7. Proof-bearing types must not silently unify cost or semantic boundaries

This follows Architectural Laws 29 and 40.

Do not collapse:

- bounded proof-minimal overlap and conservative expansion
- retained authority truth and policy-rich diagnostics
- concurrent group-local work and serial reduction work
- inverse authority effects and imperative undo commands

## Shared Runtime Classification

Every retained, rebuilt, or restored surface must belong to exactly one class.

### 1. Authoritative retained truth

Examples:

- graph authority
- node authority state
- checkpoint authority
- branch ancestry authority
- mutation journal authority
- merge-base authority truth

### 2. Semantically required derived truth

Examples:

- canonical dependency/topology indexes required for valid execution
- replay suffix structures required for reconstructability or merge truth
- merge support structures required for supported bounded reconciliation
- any derived structure whose absence would change admissibility or semantic outcome

### 3. Performance-only derived state

Examples:

- acceleration indexes
- bounded caches
- convenience lookup structures
- precomputed candidate maps that can be rebuilt without changing truth

### 4. Diagnostic richness

Examples:

- explanation artifacts
- provenance detail
- forensic summaries
- convenience inspection packets
- retained diagnostics beyond what semantic correctness requires

Hard rules:

- policy may affect diagnostic richness only
- policy may not affect authoritative truth
- policy may not affect semantically required derived truth
- policy may not affect merge/apply/restore admissibility
- restore, merge, rollback, and planning may depend only on authoritative or semantically required derived truth

## Workstream 1: `S9.12` Reconstructability Completion

### Adversarial constraint

A checkpointed, branch-heavy runtime under long history and repeated snapshot restore must be reconstructable from checkpoint plus a bounded journal suffix only. Restoring from snapshot-like state bundles, broad retained history, or diagnostic surfaces is architecturally invalid.

### Current defect

The runtime is now more honest:

- snapshot reconstructability records retained replay span
- journal proof is no longer silently absent

But reconstructability is still too record-like:

- restore is not yet forced to consume one canonical proof object
- the separation between authority restore, semantically required derived rebuild, and policy-rich diagnostics is not yet pinned tightly enough
- `RequiredDerivedRebuildSet` does not yet exist as a closed semantic class

### Required target forms

```rust
pub struct CheckpointBoundary {
    pub checkpoint_id: CheckpointId,
    pub authority_digest: AuthorityDigest,
    pub branch_identity: BranchIdentity,
    pub checkpoint_kind: CheckpointKind,
}

pub struct BoundedJournalSegment {
    pub start_cursor: ReplayCursor,
    pub end_cursor: ReplayCursor,
    pub retained_events: JournalEventSet,
    pub continuity_digest: JournalContinuityDigest,
    pub replay_stable: bool,
}

pub enum RequiredDerivedRebuildSet {
    DependencyIndexes(DependencyIndexRebuildProof),
    ReplaySuffix(ReplaySuffixRebuildProof),
    MergeSupport(MergeSupportRebuildProof),
}

pub struct ReconstructabilityProof {
    pub checkpoint: CheckpointBoundary,
    pub journal: BoundedJournalSegment,
    pub required_rebuild: Vec<RequiredDerivedRebuildSet>,
}
```

### Proof semantics

`CheckpointBoundary`

- producer: checkpoint subsystem at capture time
- certifies: authority boundary and branch identity
- invalidated by: mutation beyond the captured checkpoint state
- reuse model: persistent and inspectable

`BoundedJournalSegment`

- producer: retained journal/replay capture at snapshot time
- certifies: bounded suffix continuity and retained replay window
- invalidated by: continuity break or truncation beyond supported retained span
- replay-verifiable: yes
- reuse model: inspectable and replay-stable

`ReconstructabilityProof`

- producer: checkpoint capture plus bounded journal capture
- certifies: authority reconstruction scope and semantically required rebuild scope
- invalidated by: authority mutation beyond `journal.end_cursor` or mismatched checkpoint continuity
- replay-verifiable: yes
- reuse model: inspectable and restore-consumable

### Required classification lock

`RequiredDerivedRebuildSet` may include only semantically required derived truth:

- dependency snapshot/index structures required for later valid execution
- replay suffix structures required for equivalence or merge support
- bounded merge-support structures if they are required for supported merge semantics

It must exclude:

- convenience caches
- rich diagnostics
- policy-tier summaries
- optional forensic materializations

### Restore phase chain

Restore must become exactly three explicit phases:

1. `restore_authority_from_checkpoint`
   - input: `CheckpointBoundary`
   - output: authoritative runtime truth only

2. `rebuild_required_derived_from_authority`
   - input: authoritative runtime truth plus `RequiredDerivedRebuildSet`
   - output: semantically required derived truth only

3. `apply_diagnostic_policy_richness`
   - input: authority + semantically required derived truth + active diagnostics policy
   - output: policy-shaped diagnostic richness only

Hard rule:

- phase 3 may not mutate authority
- phase 3 may not mutate semantically required derived truth
- phase 3 may not alter merge/apply/restore admissibility
- phase 3 may not introduce quasi-authoritative caches required by later planning or execution

### Compile-time enforcement

- restore APIs accept `ReconstructabilityProof` only
- no supported restore helper accepts raw snapshot bundles or partially descriptive snapshot metadata
- `RequiredDerivedRebuildSet` is a closed type family owned by reconstructability code
- policy-rich diagnostic rebuild APIs are structurally separate from reconstructability restore APIs
- no supported snapshot path may produce `journal: None`

### Measurement boundaries

Checkpoint/reconstructability facade must expose:

- `journal_replay_span`
- `journal_event_count`
- `journal_retained_bytes`
- `restore_authority_breadth`
- `restore_required_derived_breadth`
- `restore_diagnostic_richness_breadth`

### Acceptance tests

- `checkpoint_plus_bounded_journal_is_sufficient_for_restore`
- `destroying_all_derived_state_and_rebuilding_from_reconstructability_proof_preserves_truth`
- `required_derived_rebuild_set_contains_no_policy_optional_richness`
- `restore_under_tier_matrix_preserves_authority_and_required_derived_truth`
- `discontinuous_or_truncated_journal_proof_fails_before_restore_mutation`
- `replay_suffix_equivalence_holds_across_long_snapshot_restore_churn`

### Closeout criteria

- checkpoint + bounded journal is the only supported reconstructability truth
- authority, semantically required derived truth, performance-only derived state, and diagnostic richness are structurally distinct
- restore no longer accepts snapshot-like bundles as authority proof

## Workstream 2: `S9.15` Bounded Merge Completion

### Adversarial constraint

Under long-lived branch divergence, repeated snapshot restore, and hostile overlapping mutation journals, merge planning must remain bounded by carried proof rather than ambient whole-branch knowledge. Supported merge flows must either lower to a bounded merge plan or fail before candidate construction.

### Current defect

The whole-live fallback has been removed, which is correct. But merge still is not fully proof-complete:

- merge still captures or consults branch-state-adjacent structures too freely
- the semantic status of target overlap is not sharp enough
- candidate construction is not yet specified as a pure function of lowered planning inputs plus proof-authorized indexes

### Required target forms

```rust
pub struct MergeBoundaryWitness {
    pub source_boundary: BranchMutationBoundaryId,
    pub target_boundary: BranchMutationBoundaryId,
    pub merge_base: CheckpointBoundary,
    pub continuity_digest: MergeContinuityDigest,
}

pub struct StructuralMergeJournalSlice {
    pub records: Vec<StructuralMergeRecord>,
    pub source_boundary: BranchMutationBoundaryId,
}

pub struct ProofMinimalOverlapBasis {
    pub overlapping_nodes: CanonicalNodeSet,
    pub overlapping_dependencies: CanonicalDependencySet,
}

pub struct ConservativeOverlapExpansion {
    pub expanded_nodes: CanonicalNodeSet,
    pub expansion_reason: OverlapExpansionReason,
}

pub struct PlannedMergeCandidateSet {
    pub candidates: CanonicalNodeSet,
}

pub struct LoweredMergePlan {
    pub boundary: MergeBoundaryWitness,
    pub source_slice: StructuralMergeJournalSlice,
    pub overlap_basis: ProofMinimalOverlapBasis,
    pub overlap_expansion: ConservativeOverlapExpansion,
    pub candidates: PlannedMergeCandidateSet,
    pub resolution_basis: MergeDecisionBasis,
}
```

### Semantic split of overlap

This is mandatory and must not collapse into one slice concept.

`ProofMinimalOverlapBasis`

- the smallest authority-relevant overlap implied directly by:
  - source journal slice
  - merge-base proof
  - target continuity semantics
- not a heuristic
- not a convenience expansion
- not an execution-ready candidate set

`ConservativeOverlapExpansion`

- the explicit bounded expansion needed for sound reconciliation beyond proof-minimal overlap
- must name why each expansion class is sound and necessary
- must remain bounded by proof-authorized indexes maintained from the same mutation journal semantics

`PlannedMergeCandidateSet`

- final candidate membership
- pure function of:
  - `MergeBoundaryWitness`
  - `StructuralMergeJournalSlice`
  - `ProofMinimalOverlapBasis`
  - `ConservativeOverlapExpansion`
  - proof-authorized bounded indexes only

### Hard merge law

For supported merge flows, candidate construction must be a pure function of lowered merge planning inputs plus canonical proof-authorized indexes whose maintenance is themselves bounded by the same mutation journal semantics.

Forbidden:

- whole-live branch scans
- convenience indexes that effectively encode whole-branch truth
- hidden ambient branch-state discovery during candidate construction

### Proof semantics

`MergeBoundaryWitness`

- producer: branch lifecycle + mutation ledger + merge-base authority proof
- certifies: one source-target-merge-base bounded merge relationship
- invalidated by: mutation past source or target boundary
- replay-verifiable: yes
- reuse model: inspectable until either side mutates past boundary

`StructuralMergeJournalSlice`

- producer: mutation ledger
- certifies: bounded structural source delta since boundary
- invalidated by: further source mutation
- reuse model: inspectable

`ProofMinimalOverlapBasis`

- producer: merge lowering from journal proof and merge-base truth
- certifies: minimal authority-relevant overlap only
- invalidated by: boundary invalidation or changed overlap indexes
- reuse model: planning-only

`ConservativeOverlapExpansion`

- producer: merge lowering using proof-authorized indexes
- certifies: bounded sound expansion beyond minimal overlap
- invalidated by: same as overlap basis
- reuse model: planning-only

### Merge planning chain

1. establish `MergeBoundaryWitness`
2. derive `StructuralMergeJournalSlice`
3. derive `ProofMinimalOverlapBasis`
4. derive `ConservativeOverlapExpansion`
5. derive `PlannedMergeCandidateSet`
6. lower `LoweredMergePlan`
7. execute merge from `LoweredMergePlan` only

### Compile-time enforcement

- merge executor accepts `LoweredMergePlan` only
- candidate-construction helpers require `MergeBoundaryWitness`
- overlap basis, expansion, and final candidates are distinct types
- supported merge variants cannot encode whole-live candidate scope
- unsupported merge families must reject before candidate construction

### Measurement boundaries

Merge planning/execution must expose:

- `source_slice_breadth`
- `proof_minimal_overlap_breadth`
- `conservative_overlap_expansion_breadth`
- `final_candidate_breadth`
- `reconciliation_breadth`
- `boundary_witness_kind`

### Acceptance tests

- `supported_merge_candidate_construction_is_purely_proof_driven`
- `proof_minimal_overlap_and_conservative_expansion_remain_distinct_and_bounded`
- `supported_merge_never_constructs_whole_live_scope`
- `merge_candidate_construction_is_identical_with_and_without_convenience_branch_indexes`
- `restore_after_merge_preserves_future_boundary_validity`
- `unsupported_merge_families_fail_before_candidate_construction`

### Closeout criteria

- supported merge flows are always bounded by carried proof
- candidate construction is pure and proof-driven
- whole-live merge is unrepresentable on supported paths

## Workstream 3: `S9.9` True Parallel Apply Completion

### Adversarial constraint

Under wide disjoint stages on geometry-style graphs, the runtime must either execute real grouped concurrent apply with identical semantic outcomes to serial execution, or lower honestly to serial execution. It must not advertise full parallelism while funneling the majority of semantic work through a hidden serial phase.

### Current defect

The plan structure exists:

- disjoint groups
- lowering
- admission/reporting

But the execution substrate is incomplete:

- apply still serializes groups on the mutable engine
- hidden shared outputs are the likely failure point
- reduction is not yet sharply constrained

### Required target forms

```rust
pub struct DisjointApplyProof {
    pub planning_basis: PlanningAuthorityDigest,
    pub mutation_domain: MutationDomain,
    pub group_footprints: Vec<ApplyFootprint>,
    pub shared_surface_policy: SharedSurfacePolicy,
}

pub enum LoweredApplyPlan {
    Serial(SerialApplyPlan),
    GroupedConcurrent(ConcurrentApplyPlan),
}

pub struct GroupLocalApplyPacket {
    pub authority_delta: AuthorityDeltaPacket,
    pub topology_delta: TopologyDeltaPacket,
    pub replay_packet: LocalReplayPacket,
    pub lineage_packet: LocalLineagePacket,
    pub telemetry_packet: LocalTelemetryPacket,
    pub snapshot_packet: LocalSnapshotPacket,
}

pub struct ConcurrentApplyPlan {
    pub groups: Vec<DisjointApplyGroup>,
    pub proof: DisjointApplyProof,
    pub reduction: ConcurrentApplyReductionPlan,
}

pub struct ConcurrentApplyReductionPlan {
    pub ordering_contract: ReductionOrderingContract,
    pub allowed_work: ReductionWorkClass,
}
```

### Hard concurrency law

This plan adopts the safer model:

- workers may write only to group-local isolated buffers
- all shared runtime surfaces are reduction-only

That means `DisjointApplyProof` does not need to certify global disjointness over every shared surface. It must certify:

- disjointness over worker-writable authority/mutation space
- that all non-disjoint shared surfaces are excluded from worker writes and deferred to deterministic reduction

### Worker and reduction contract

Worker-allowed work:

- group-local authority mutation on disjoint slices
- local dependency reconciliation already proven local
- local topology/artifact/runtime deltas
- local replay/lineage/telemetry packets
- local snapshot/update packets

Reduction-allowed work:

- deterministic publication ordering
- insertion of replay packets into shared retained structures
- lineage publication
- telemetry aggregation
- snapshot commit publication
- merge-relevant derived index publication that cannot be isolated per worker

Reduction-forbidden work:

- broad dependency discovery
- recomputing disjointness
- late execution strategy selection
- majority of semantic authority work
- convenience artifact construction
- hidden rich-path reconstruction

### Parallel equivalence scope

Serial vs concurrent equivalence must cover:

- authoritative state
- replay packet ordering contract
- lineage/provenance graph
- semantically required diagnostic conclusions
- checkpoint eligibility and reconstructability consequences
- downstream merge-relevant truth

Matching only coarse summaries is insufficient.

### Proof semantics

`DisjointApplyProof`

- producer: lowered planning from canonical task footprints and planning basis
- certifies: group concurrency safety for one lowered stage under one mutation domain and shared-surface policy
- invalidated by: changed planning basis, changed footprints, or changed shared-surface policy
- replay-verifiable: indirectly through equivalence and packet-ordering tests
- reuse model: single-use per lowered stage

### Compile-time enforcement

- planner emits `LoweredApplyPlan::Serial` or `LoweredApplyPlan::GroupedConcurrent`
- executor consumes lowered plan variant only
- grouped concurrent path requires `DisjointApplyProof`
- worker code cannot access shared surfaces directly
- reduction consumes only `GroupLocalApplyPacket`s
- `SerialFallback` cannot remain as the semantic truth for grouped concurrent execution

### Measurement boundaries

Planner/apply must expose:

- `group_local_authority_breadth`
- `group_local_packet_breadth`
- `reduction_packet_breadth`
- `reduction_group_count`
- `shared_surface_publication_breadth`
- `parallel_admission_rejection_reason`

### Acceptance tests

- `grouped_concurrent_and_serial_apply_are_equivalent_across_all_semantic_surfaces`
- `workers_write_only_group_local_buffers`
- `reduction_breadth_scales_with_publication_packets_not_semantic_work`
- `no_grouped_concurrent_plan_exists_without_disjoint_apply_proof`
- `unsupported_mutable_engine_parallelism_lowers_to_serial_honestly`

### Closeout criteria

- `FullParallel` means real grouped concurrent apply or does not exist
- reduction is narrow publication, not a backdoor serial semantic engine
- planner, lowered plan, executor, and reporting all agree on execution truth

## Workstream 4: `S9.10` Rollback and Lifecycle Completion

### Adversarial constraint

Under frequent transactions, injected failures, branch switching, branch creation, and repeated restore cycles, rollback and lifecycle transfer must preserve exact pre-operation semantic truth while avoiding eager broad baseline cloning and avoiding heavyweight branch-state capture as a routine orchestration primitive.

### Current defect

We improved the system:

- rollback baseline capture is lazy
- branch switching is move-based
- heavy capture is at least named honestly

But the remaining substrate is incomplete:

- rollback can still decay into a broad undo engine if not typed precisely
- rollback target truth boundary is not yet pinned tightly enough
- lifecycle paths still need full type separation for move vs duplicate vs restore vs heavy capture

### Rollback truth boundary

After rollback, the runtime must be observationally equivalent to the pre-transaction state with respect to:

- authoritative truth
- semantically required derived truth

Diagnostic richness may be restored:

- exactly, or
- by deterministic rebuild according to policy

Rollback packets must not carry policy-optional forensic richness unless required for deterministic semantic rebuild.

### Required target forms

```rust
pub enum TransactionRollbackPacket {
    Authority(AuthorityRollbackDelta),
    Topology(TopologyRollbackDelta),
    Config(ConfigRollbackDelta),
    DiagnosticsRequired(DiagnosticsRollbackDelta),
}

pub struct AuthorityTransferPacket {
    pub branch_id: SignalBranchId,
    pub authority: AuthorityState,
    pub required_derived: DerivedSemanticState,
}

pub struct ExplicitBranchForkPacket {
    pub source_branch: SignalBranchId,
    pub forked_authority: AuthorityState,
    pub forked_required_derived: DerivedSemanticState,
}

pub struct HeavyCaptureWitness(());

pub enum BranchLifecycleTransfer {
    Move(AuthorityTransferPacket),
    Duplicate(ExplicitBranchForkPacket),
    Restore(ReconstructabilityProof),
}
```

### Rollback packet grain

Rollback packets are:

- typed minimal inverse authority effects
- subsystem-local
- effect-derived
- ordered only where subsystem causality demands ordering

Rollback packets are not:

- imperative reverse commands
- generic serialized undo scripts
- baseline state bundles with a new name

### Lifecycle transfer semantics

`Move`

- transfer active authority to/from stored branch state
- no duplication implied

`Duplicate`

- explicit branch fork with real second branch-owned truth surface
- only this path duplicates authority

`Restore`

- reconstructive restore from `ReconstructabilityProof`
- not a copy of an arbitrary branch bundle

### Heavy capture policy

Witness-gating alone is not enough unless witness construction is tightly owned.

Rules:

- `HeavyCaptureWitness` construction must be owned by one sealed internal lifecycle module only
- routine branch switch/create/restore paths cannot construct it
- tests may access it only through explicitly test-scoped helper entrypoints
- every heavy capture increments a dedicated runtime counter

### Proof semantics

`TransactionRollbackPacket`

- producer: mutation/effect application
- certifies: minimal inverse authority effect for one subsystem mutation
- invalidated by: application against the wrong authority baseline or wrong transaction order
- replay-verifiable: yes, against effect log and pre/post authority truth
- reuse model: single-use per failed transaction

`AuthorityTransferPacket`

- producer: lifecycle move path
- certifies: single-consumer transfer of authority and required derived truth
- invalidated by: mutation after transfer creation
- reuse model: single-use move-only

`ExplicitBranchForkPacket`

- producer: branch fork path
- certifies: explicit duplication for a second branch-owned truth surface
- invalidated by: mutation after fork packet creation
- reuse model: single-use

### Compile-time enforcement

- rollback/finalize consumes `TransactionRollbackPacket`s only
- branch switch APIs accept `AuthorityTransferPacket` only
- branch restore APIs accept `ReconstructabilityProof` only
- branch fork APIs accept `ExplicitBranchForkPacket` only
- heavy capture APIs require `HeavyCaptureWitness` from a sealed constructor
- rollback and transfer packets remain move-only unless a real second consumer exists

### Measurement boundaries

Transaction/lifecycle surfaces must expose:

- `rollback_packet_breadth`
- `rollback_packet_count_by_subsystem`
- `move_transfer_count`
- `explicit_fork_count`
- `restore_transfer_count`
- `heavy_capture_count`

### Acceptance tests

- `read_only_and_no_op_transactions_emit_zero_rollback_packets`
- `rollback_packets_restore_pre_transaction_truth_without_baseline_bundles`
- `branch_switch_is_move_only`
- `branch_fork_duplicates_only_explicit_branch_owned_truth`
- `branch_restore_requires_reconstructability_proof`
- `heavy_capture_requires_internal_witness_and_increments_counter`

### Closeout criteria

- rollback is effect-derived inverse authority restoration, not generic undo
- branch lifecycle operations are type-separated and cost-honest
- heavy capture is narrow, counted, and non-routine

## Cross-Phase Measurement and Certification

### Required measurement boundaries

These are mandatory and must be surfaced at subsystem facades and phase boundaries:

Reconstructability:

- `journal_replay_span`
- `journal_suffix_breadth`
- `restore_authority_breadth`
- `restore_required_derived_breadth`
- `restore_diagnostic_richness_breadth`

Merge:

- `boundary_witness_kind`
- `source_slice_breadth`
- `proof_minimal_overlap_breadth`
- `conservative_overlap_expansion_breadth`
- `final_candidate_breadth`
- `reconciliation_breadth`

Parallel apply:

- `lowered_apply_kind`
- `group_local_authority_breadth`
- `group_local_packet_breadth`
- `reduction_packet_breadth`
- `shared_surface_publication_breadth`
- `parallel_admission_rejection_reason`

Rollback/lifecycle:

- `rollback_packet_breadth`
- `rollback_packet_count_by_subsystem`
- `move_transfer_count`
- `explicit_fork_count`
- `restore_transfer_count`
- `heavy_capture_count`

### Required adversarial certification scenarios

- checkpoint + bounded journal reconstruction equivalence under long branch/snapshot churn
- merge-after-restore boundedness under overlapping mutation journals
- serial vs concurrent equivalence on disjoint geometry-style subgraphs
- repeated branch switch/fork/restore under allocator/catalog churn
- rollback exactness under failures injected at multiple commit phases
- negative-space certification:
  - unsupported merge without boundary witness fails before candidate construction
  - concurrent apply without `DisjointApplyProof` cannot compile/lower
  - restore without `ReconstructabilityProof` cannot compile/lower
  - routine lifecycle paths cannot call heavy capture helpers

## Document and Closeout Updates

When implemented, the following docs must be updated together:

- [signal_architecture2.md](/C:/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge_signal/signal_architecture2.md)
- [test-requirements.md](/C:/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge_signal/test-requirements.md)
- [forge_signal_adversarial_testing_matrix.md](/C:/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/engineering/forge_signal_adversarial_testing_matrix.md)
- [forge_signal_fintech_certification_matrix.md](/C:/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/engineering/forge_signal_fintech_certification_matrix.md)

Required closeout statement:

- `S9.12`, `S9.15`, `S9.9`, and `S9.10` are complete only when supported execution is proof-driven, no forbidden fallback path remains representable, and the certification suite proves breadth, equivalence, and exactness through named counters and proof-bearing runtime surfaces.

## Assumptions and Defaults

- explicit failure remains preferable to broad fallback whenever proof is missing
- proof-authorized indexes are allowed only when maintained from the same canonical semantics they serve
- diagnostic policy remains strictly post-semantic and may not affect authority, required derived truth, or admissibility
- `S10` is out of scope for this document; no merge-forward feature expansion belongs in this batch
