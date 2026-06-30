# Storage Foundation S.5 Engineering Spec: Physical Isolation, Latches, Epochs, And Stable Read Plans

> **Status:** Planned
>
> **Roadmap parent:** [forge_store_roadmap_2.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/forge_store_roadmap_2.md)
>
> **Primary prerequisite:** `S.4 WAL, Checkpoint, LSN, And Recovery Physics`
>
> **Follow-on storage-foundation sequence:** `S.6 Hardware-Aware I/O, QoS, And Background Work Pacing`
>
> **Primary architectural driver:** make physical byte reads stable while
> checkpointing, compaction, reclaim, tier movement, and future blob migration
> move physical structures underneath readers, without pretending Store owns
> semantic MVCC visibility.

## Goal

Make Forge Store physical reads stable under concurrent maintenance.

S.5 turns recovered physical roots, pageLSN frontiers, recovery receipts,
source-precedence traces, generation proofs, latches, epochs, read leases,
copy-on-write publication, and reachability barriers into explicit physical
isolation law. It is complete when an admitted physical read plan can survive
interleaved compaction, checkpointing, reclaim, tier movement, restart during
cutover, and future blob-chunk movement without observing half-published roots,
missing pages, stale generation reuse, or reclaimed bytes that were still
protected by the read plan.

## Why This Sequence Exists

S.1 gave Store physical addresses. S.2 made physical byte access bounded. S.3
made physical bytes integrity-vetted. S.4 made recovered physical state
deterministic after crash. S.5 is the next required boundary: while the store is
online, readers need physical byte stability even as maintenance rewrites,
moves, publishes, and reclaims physical structures.

This is not semantic MVCC. `forge-relational` owns transaction visibility,
branch meaning, snapshot truth, and semantic read isolation. S.5 answers a lower
question: once a Store physical read is admitted against a root, page, segment,
extent, or future chunk, do the bytes remain stable long enough to read them
honestly?

## Governing Summaries

- `MENTALITY.md`
  protects hard-problem-first design. S.5 must start from hostile maintenance
  interleavings, stale generations, and reclaim pressure rather than a friendly
  latch wrapper.
- `arch_laws.md`
  protects proof-bearing phase progression, phase-typed observation, and
  authority/derivation separation. S.5 must lower recovered S.4 state into
  stable physical read plans and must not let execution re-decide root, latch,
  epoch, or reachability strategy.
- `composition_laws.md`
  protects named semantic steps. S.5 must not collapse latch ordering, epoch
  admission, read planning, copy-on-write publication, reclaim barriers,
  deadlock handling, and diagnostics into one isolation manager.
- `domain_structure_laws.md`
  protects topology by responsibility. Latches, epochs, stable read plans,
  publication barriers, reclaim reachability, deadlock policy, maintenance
  interleaving harnesses, and S.6 handoff evidence fail differently and need
  separate Store-owned modules.
- `perf_laws.md`
  protects visible cost. S.5 must expose exact latch attempts, latch waits,
  epoch retries, stale-plan denials, protected references, blocked reclaim,
  copied pages, publication swaps, and read-plan footprint counters.
- `forge_store_roadmap_2.md`
  places S.5 after S.4 recovery and before S.6 I/O QoS. The roadmap requires
  stable physical reads under maintenance before the store can honestly claim
  foreground latency isolation under hardware-aware I/O pressure.
- `storage-foundation-s3.md`
  gives S.5 physical corruption locality and quarantine posture. S.5 may use
  that locality to block movement or reclaim, but it does not repair damage.
- `storage-foundation-s4.md`
  gives S.5 recovered roots, pageLSN frontiers, replay receipts,
  source-precedence traces, recovery counters, and explicit stability
  assumptions. S.5 must consume that typed handoff instead of reopening recovery
  physics.

## Adversarial Constraint

S.5 must survive this hostile condition:

> Foreground reads, recovery reads, scrub reads, compaction planning,
> compaction cutover, checkpoint publication, root publication, reclaim, tier
> movement, future blob migration, and crash/restart during cutover all
> interleave against a store larger than memory. No admitted read may observe
> half-published roots, reused stale generations, moved pages without stable
> references, reclaimed extents, quarantined bytes as ordinary data, or
> execution-time strategy changes hidden behind a cheap-looking read API.

If a read plan can be admitted from semantic snapshot meaning alone, if a latch
order is convention-only, if a root epoch can change invisibly under a reader,
if reclaim can free protected bytes, if compaction can publish moved pages
without copy-on-write or equivalent reachability proof, or if deadlock behavior
is not typed, S.5 is not closed.

## Product Decision Lock

- S.5 owns physical byte stability, not semantic visibility.
- S.5 consumes `S5PhysicalIsolationRecoveryReadiness`; it must not accept copied
  S.4 closeout fields, live runtime cache state, or semantic MVCC snapshots as
  physical stability authority.
- Stable read plans are admitted, proof-bearing physical artifacts. They are not
  collections of page ids plus comments.
- Physical latches protect mutation of physical structures. Epochs prove root,
  manifest, segment, extent, page, and future chunk stability across the read
  plan.
- Copy-on-write or an equivalent publication protocol is mandatory wherever
  maintenance rewrites reachable physical structure.
- Reclaim is a proof consumer, not a scavenger. It may reclaim only bytes that
  are unreachable by every admitted read, checkpoint, recovery, scrub, and
  future blob reachability barrier.
- S.5 may expose assumptions to S.6 about latch waits, background interference,
  and protected byte footprints, but it does not claim hardware I/O QoS.

## Physical Isolation Laws

- Physical/Semantic Isolation Separation Law: semantic MVCC visibility cannot
  admit, extend, or release physical byte stability. Store read plans must carry
  Store-owned root, page, segment, extent, generation, epoch, and reachability
  proof.
- Stable Read Plan Law: every nontrivial physical read must be admitted through
  a plan that names root epoch, manifest epoch, physical references, generation
  proofs, latch requirements, reachability barriers, footprint counters, and
  retry posture before execution.
- Epoch Honesty Law: root, manifest, segment, extent, page, and future chunk
  epochs may be compared only inside a declared stability scope. A successful
  comparison outside that scope is a projection, not authority.
- Latch Order Law: latch acquisition must follow a declared partial order or
  deny with typed deadlock-prevention evidence before waiting can create a
  cycle.
- Copy-On-Write Publication Law: maintenance may publish rewritten physical
  structure only by creating a new reachable version, durably publishing the
  new root or manifest, and preserving old reachability until admitted readers
  release or expire.
- Reclaim Reachability Law: reclaim may consume only executed reachability
  evidence and live hazard/lease tables. Backend residue, directory listing,
  last-observed page ids, and copied read-plan fields cannot prove reclaim
  eligibility.
- Quarantine Stability Law: quarantined or unresolved physical damage remains
  movement-blocking or read-denying until a later repair sequence admits a new
  posture. S.5 cannot make damaged bytes stable by moving them.
- Restart Stability Law: restart during physical cutover must recover either
  the old stable root or the new stable root with typed cutover posture; it may
  not expose a mixed tree.
- Diagnostic Non-Interference Law: rich latch, epoch, wait, and reachability
  diagnostics may be materialized by policy, but they must not change read-plan
  admission or publication outcome.

## Planned Directory Skeleton

`workspaces/forge-store/crates/forge-store-physical-isolation/src/`

- `lib.rs`
  exposes the crate facade and re-exports only proof-bearing S.5 boundary
  types.
- `readiness.rs`
  consumes `S5PhysicalIsolationRecoveryReadiness` and produces physical
  isolation entry authority.
- `physical_snapshot_boundary.rs`
  keeps semantic snapshot identifiers out of physical stability admission while
  preserving explicit cross-layer correlation.
- `epoch/`
  owns root, manifest, segment, extent, page, and future chunk epoch tokens,
  comparison scopes, retry decisions, and stale-plan denials.
- `latch/`
  owns latch classes, latch order, acquisition plans, wait counters, and
  deadlock prevention or detection reports.
- `read_plan/`
  owns stable read plan admission, protected reference sets, read-plan
  footprint accounting, execution-ready read handles, and release receipts.
- `publication/`
  owns copy-on-write publication plans, root/manifest swap receipts, old-root
  preservation, and crash-restart cutover posture.
- `maintenance_interlock/`
  owns read-during-compaction, read-during-checkpoint, read-during-reclaim,
  read-during-tier-movement, and future read-during-blob-migration safety
  rules.
- `reachability/`
  owns hazard, lease, protected-reference, and reclaim eligibility tables.
- `quarantine_interlock.rs`
  consumes S.3 quarantine and damage-locality evidence so unstable or damaged
  physical regions cannot be moved or reclaimed as ordinary bytes.
- `counters.rs`
  owns exact latch, wait, epoch, retry, stale-plan, protected-reference,
  publication, copy-on-write, and blocked-reclaim counters.
- `evidence/`
  maps executed Store isolation findings into Foundational and Proof-compatible
  evidence without replacing Store physical stability authority.
- `s6_handoff.rs`
  publishes `S6IoQosIsolationReadiness` with physical-stability assumptions,
  latch/wait surfaces, protected byte footprints, and unsupported QoS claims.

`workspaces/forge-store/crates/forge-store-certification/src/`

- `s5_physical_isolation_harness/`
  extends the Roadmap 2 harness with scenario definitions, lowered plans,
  interleaving schedules, observers, oracles, mutation evidence, and story
  transcripts for S.5.

`workspaces/forge-store/crates/forge-store-test-support/src/`

- `s5_physical_isolation/`
  owns deterministic interleaving drivers, maintenance actors, latch schedulers,
  epoch drift injectors, reclaim adversaries, and restart-at-cutover fixtures.

## Phase Plan

### Phase 1: Admit S.4 Recovery Readiness Into Physical Isolation Entry

Phase 1 freezes the S.4-to-S.5 boundary. It admits only recovered physical roots,
pageLSN frontiers, replay receipts, source-precedence traces, recovery counters,
and explicit stability assumptions from S.4.

**Relevant subsystems**
- `forge-store-recovery-physics`
- `forge-store-physical-isolation`
- `forge-store-readiness`
- `forge-store-certification`

**Relevant APIs**
- `S5PhysicalIsolationRecoveryReadiness`
- `RecoveredPhysicalState`
- `RecoverySourceDecisionTrace`
- `RedoExecutionReceipt`
- `RecoveryCounterSnapshot`
- `PhysicalIsolationEntryAdmission`
- `PhysicalIsolationEntryDenial`

**Warnings**
- Do not reconstruct S.5 entry from copied S.4 closeout fields.
- Do not treat semantic MVCC snapshots as physical read stability.
- Do not accept live cache, buffer-pool, mmap, or same-process runtime state as
  S.5 entry proof.

**Test requirements**
- Adversarial equivalence: independently materialized S.4 readiness over the
  same persisted recovery outcome admits to the same S.5 physical isolation
  entry identity and root epoch basis.
- Adversarial denial: copied closeout fields, live runtime handles, semantic
  snapshot tokens, debug strings, terminal projections, and stale recovery
  reports cannot satisfy S.5 entry admission.
- Boundary proof: S.5 entry cannot reopen S.4 WAL replay, source precedence, or
  checkpoint validation; it consumes only typed S.4 proof.

**Engineering decisions**
- S.5 starts from recovered physical state, not arbitrary store files.
- S.4 remains the owner of recovery correctness.
- S.5 entry must already know what stability assumptions S.4 did and did not
  prove.

**Open questions**
- None.

### Phase 2: Separate Physical Read Stability From Semantic MVCC Visibility

Phase 2 names the cross-layer seam so Store can correlate semantic readers with
physical reads without letting semantic visibility mint physical stability.

**Relevant subsystems**
- `forge-store-physical-isolation`
- `forge-store-authority`
- `forge-relational`
- `forge-store-certification`

**Relevant APIs**
- `PhysicalSnapshotCorrelation`
- `SemanticVisibilityReference`
- `PhysicalReadStabilityAuthority`
- `SemanticVisibilityCannotMintPhysicalStability`

**Warnings**
- Do not duplicate relational MVCC in Store.
- Do not let branch, transaction, snapshot, or semantic commit identifiers
  authorize page, root, extent, or chunk stability.
- Do not hide the distinction behind generic `snapshot` names.

**Test requirements**
- Adversarial equivalence: the same semantic read correlated to the same
  recovered physical root produces the same physical plan only when the Store
  physical root and epoch proofs match.
- Adversarial denial: semantic transaction ids, branch ids, relational snapshot
  tokens, projection masks, and current-basis exports cannot admit or extend a
  physical read plan without Store physical stability authority.
- Naming proof: public APIs distinguish semantic visibility, physical snapshot
  correlation, and physical read stability in type names and denial kinds.

**Engineering decisions**
- Semantic truth and physical stability are separate authorities.
- Store may expose correlation evidence for diagnostics, but correlation is not
  admission.
- This phase prevents S.5 from becoming a second MVCC runtime.

**Open questions**
- None.

### Phase 3: Define Root, Manifest, Segment, Extent, Page, And Chunk Epochs

Phase 3 defines the epoch vocabulary S.5 uses to detect stale read plans and
maintenance publication races.

**Relevant subsystems**
- `forge-store-physical-isolation`
- `forge-store-physical-format`
- `forge-store-recovery-physics`
- `forge-store-certification`

**Relevant APIs**
- `RootEpoch`
- `ManifestEpoch`
- `SegmentEpoch`
- `ExtentEpoch`
- `PageEpoch`
- `ChunkEpoch`
- `EpochComparisonScope`
- `EpochRetryDecision`
- `StalePhysicalReadPlanDenial`

**Warnings**
- Do not compare epochs outside a declared stability scope.
- Do not collapse generation identity, LSN, pageLSN, and epoch into one value.
- Do not make chunk epoch semantics claim S.7 blob lifecycle.

**Test requirements**
- Adversarial equivalence: repeated admission against unchanged root, manifest,
  segment, extent, and page epochs produces the same stable-read epoch basis.
- Adversarial denial: stale root epochs, page generation reuse, manifest epoch
  drift, extent replacement, and future chunk epoch mismatch deny or retry
  before bytes are read.
- Scope proof: epoch comparisons outside their declared scope fail rather than
  becoming ordinary boolean equality.

**Engineering decisions**
- Epochs prove physical stability windows.
- Generations identify reused physical identities; epochs identify observed
  publication stability.
- Future chunk epochs exist only as stability placeholders until S.7 owns blob
  chunk lifecycle.

**Open questions**
- None.

### Phase 4: Define Latch Classes, Acquisition Order, And Deadlock Policy

Phase 4 makes physical latch behavior explicit and mechanically auditable.

**Relevant subsystems**
- `forge-store-physical-isolation`
- `forge-store-buffer-pool`
- `forge-store-certification`

**Relevant APIs**
- `PhysicalLatchClass`
- `PhysicalLatchMode`
- `LatchAcquisitionPlan`
- `LatchOrderProof`
- `LatchWaitCounterSnapshot`
- `DeadlockPreventionDenial`
- `DeadlockDetectionReport`

**Warnings**
- Do not rely on comments or convention for latch ordering.
- Do not let read plans acquire latches in execution order discovered after
  seeing pages.
- Do not hide blocking behind an ordinary read method.

**Test requirements**
- Adversarial equivalence: two callers that request the same protected
  physical footprint in different input orders lower to the same canonical
  latch acquisition order.
- Adversarial denial: cyclic latch plans, mixed hierarchy inversions, upgrade
  attempts without upgrade authority, and execution-time latch discovery deny
  before waiting.
- Deadlock proof: deterministic hostile schedules either cannot form a wait
  cycle or emit typed deadlock detection evidence with exact wait counters.

**Engineering decisions**
- S.5 must pick either prevention or detection per latch family and make that
  policy explicit.
- Latch acquisition is a lowered plan, not executor discretion.
- Wait counters are part of the result surface because S.6 will later consume
  them for foreground interference accounting.

**Open questions**
- None.

### Phase 5: Admit Stable Physical Read Plans

Phase 5 defines the proof-bearing physical read plan that execution may consume
without rediscovering root, latch, epoch, or reachability strategy.

**Relevant subsystems**
- `forge-store-physical-isolation`
- `forge-store-buffer-pool`
- `forge-store-physical-format`
- `forge-store-certification`

**Relevant APIs**
- `StablePhysicalReadPlan`
- `StablePhysicalReadPlanAdmission`
- `StablePhysicalReadHandle`
- `PhysicalReadPlanFootprint`
- `ProtectedPhysicalReferenceSet`
- `PhysicalReadPlanReleaseReceipt`
- `ReadPlanCounterSnapshot`

**Warnings**
- Do not let execution assemble protected page sets after admission.
- Do not let a read plan look cheap if it protects a broad physical footprint.
- Do not admit plans that omit release semantics.
- Do not allow a plan to cross quarantine, generation, or stale epoch denials.

**Test requirements**
- Adversarial equivalence: the same root, references, generation proofs, and
  epoch basis produce the same canonical read-plan footprint regardless of input
  order.
- Adversarial denial: missing release semantics, broad unbounded footprints,
  quarantined references, stale page generations, and execution-time reference
  discovery deny before read handles are issued.
- Cost proof: plan admission emits exact protected-reference, latch, epoch,
  resident-byte, and allocation counters.

**Engineering decisions**
- Stable read plans are the only ordinary input to physical read execution.
- The plan carries exactly the proofs established at admission.
- Release receipts are first-class so reclaim and maintenance can consume them.

**Open questions**
- None.

### Phase 6: Execute Stable Reads Without Re-Deciding Isolation Strategy

Phase 6 executes admitted plans and proves the executor cannot re-plan, widen,
or silently retry outside declared epoch policy.

**Relevant subsystems**
- `forge-store-physical-isolation`
- `forge-store-buffer-pool`
- `forge-store-physical-integrity`
- `forge-store-certification`

**Relevant APIs**
- `ExecutionReadyPhysicalReadPlan`
- `StablePhysicalReadExecution`
- `StablePhysicalReadReceipt`
- `EpochRetryReceipt`
- `PhysicalReadExecutionDenial`

**Warnings**
- Do not let execution widen the footprint because a page moved.
- Do not silently retry on epoch drift without recording retry posture.
- Do not read bytes after latch release or plan expiry.
- Do not treat integrity failures as isolation failures; consume S.3 damage
  posture distinctly.

**Test requirements**
- Adversarial convergence: a stable read executing while maintenance publishes a
  new root either reads the old protected bytes or retries into a newly admitted
  plan with typed epoch-retry evidence.
- Adversarial denial: expired plans, released handles, stale epochs after retry
  budget, latch loss, quarantined bytes, and widened execution footprints deny
  with typed locality.
- Non-redecision proof: mutation testing verifies the executor cannot choose a
  new latch strategy, root, reference set, or reachability barrier.

**Engineering decisions**
- Execution consumes the lowered plan and may not choose isolation policy.
- Epoch retry is an explicit transition, not a loop hidden inside reads.
- S.3 integrity denials remain physically localized and do not become generic
  read failures.

**Open questions**
- None.

### Phase 7: Publish Copy-On-Write Physical Updates

Phase 7 defines how maintenance rewrites reachable physical structures without
invalidating admitted readers.

**Relevant subsystems**
- `forge-store-physical-isolation`
- `forge-store-recovery-physics`
- `forge-store-physical-format`
- `forge-store-certification`

**Relevant APIs**
- `CopyOnWritePublicationPlan`
- `PhysicalPublicationIntent`
- `RootPublicationEpoch`
- `ManifestPublicationEpoch`
- `PhysicalPublicationReceipt`
- `OldReachabilityPreservation`

**Warnings**
- Do not overwrite reachable bytes in place while an admitted reader may still
  hold them.
- Do not publish a root or manifest without preserving old reachability.
- Do not treat checkpoint cutover receipts as copy-on-write publication
  receipts; S.4 and S.5 prove different things.

**Test requirements**
- Adversarial equivalence: independently planned rewrites over the same stable
  old root and new physical structure produce the same publication epoch and
  preservation basis.
- Adversarial denial: in-place overwrite of reachable pages, missing old-root
  preservation, stale publication epoch, copied checkpoint receipt, and
  publication without reachability evidence deny before cutover.
- Crash proof: restart before, during, and after publication recovers either
  the old stable physical structure or the new stable physical structure, never
  a mixed tree.

**Engineering decisions**
- Copy-on-write is the default publication law for moved or rewritten physical
  structures.
- S.4 recovery receipts can be prerequisites, but S.5 owns online physical
  publication stability.
- Old reachability is retained until read-plan release or expiry proves it can
  be dropped.

**Open questions**
- None.

### Phase 8: Interlock Reads With Compaction

Phase 8 protects foreground and recovery reads while compaction selects,
rewrites, publishes, and retires physical structures.

**Relevant subsystems**
- `forge-store-physical-isolation`
- `forge-store-recovery-physics`
- `forge-store-physical-integrity`
- `forge-store-certification`

**Relevant APIs**
- `CompactionReadInterlockPlan`
- `CompactionRewritePublication`
- `CompactionCutoverStabilityProof`
- `ReadDuringCompactionVerdict`
- `CompactionProtectedReferenceSet`

**Warnings**
- Do not let compaction choose read-visible roots by directory residue.
- Do not move quarantined or unresolved damaged bytes as if they were stable.
- Do not let compaction reclaim old pages immediately after new root
  publication.
- Do not claim S.8 layout/index discipline; S.5 owns only stability under
  movement.

**Test requirements**
- Adversarial convergence: a read admitted before compaction cutover reads the
  old protected structure while a read admitted after cutover reads the new
  structure, and both converge on valid physical bytes for their admitted plan.
- Adversarial denial: compaction over quarantined regions, stale source epoch,
  missing old-root preservation, backend residue candidate selection, and early
  page reuse deny at named interlock boundaries.
- Counter proof: compaction/read lanes expose exact protected pages, copied
  pages, publication swaps, blocked reclaims, and epoch retries.

**Engineering decisions**
- Compaction is a maintenance actor constrained by read plans.
- S.5 may block or defer compaction; S.6 later paces its I/O.
- Compaction cutover must be restart-stable and read-stable separately.

**Open questions**
- None.

### Phase 9: Interlock Reads With Checkpoint Publication

Phase 9 makes checkpoint publication visible to readers only through admitted
root and manifest epoch transitions.

**Relevant subsystems**
- `forge-store-physical-isolation`
- `forge-store-recovery-physics`
- `forge-store-certification`

**Relevant APIs**
- `CheckpointReadInterlockPlan`
- `CheckpointPublicationStabilityProof`
- `CheckpointRootEpochTransition`
- `ReadDuringCheckpointVerdict`

**Warnings**
- Do not reopen S.4 checkpoint validity.
- Do not expose a checkpoint manifest as current before its physical
  publication epoch is admitted.
- Do not let a read plan mix old root pages with a new checkpoint frontier.

**Test requirements**
- Adversarial equivalence: readers admitted before and after checkpoint
  publication see stable physical roots that match their admitted epoch basis
  and pageLSN frontier.
- Adversarial denial: half-published checkpoint manifests, stale root epochs,
  mismatched pageLSN frontiers, copied checkpoint fields, and mixed old/new
  root reads deny or retry.
- Restart proof: crash during checkpoint publication recovers one stable
  checkpoint-root posture that read-plan admission can consume without
  ambiguous roots.

**Engineering decisions**
- S.4 proves checkpoint recovery; S.5 proves online read stability during
  checkpoint publication.
- Checkpoint publication is an epoch transition for readers.
- PageLSN frontier remains physical replay metadata, not semantic visibility.

**Open questions**
- None.

### Phase 10: Enforce Reclaim Reachability And Hazard Barriers

Phase 10 defines when old pages, extents, and future chunks may be reclaimed
after publication or movement.

**Relevant subsystems**
- `forge-store-physical-isolation`
- `forge-store-buffer-pool`
- `forge-store-physical-integrity`
- `forge-store-certification`

**Relevant APIs**
- `ReachabilityBarrier`
- `HazardLeaseTable`
- `ProtectedReferenceLease`
- `ReclaimEligibilityProof`
- `BlockedReclaimReport`
- `ReclaimDenial`

**Warnings**
- Do not reclaim from backend residue or absence from current root alone.
- Do not drop old structures before every admitted reader, scrubber, verifier,
  checkpoint, and recovery barrier releases them.
- Do not let lease expiry silently free bytes without a typed expiry posture.

**Test requirements**
- Adversarial equivalence: identical admitted read leases and publication
  receipts produce the same reclaim eligibility decision regardless of release
  order.
- Adversarial denial: active read leases, unreleased scrub windows, quarantine
  holds, checkpoint preservation, recovery verifier holds, and future chunk
  migration holds block reclaim with exact protected-reference counters.
- Leak proof: released plans eventually make reclaim eligible without
  accumulating unbounded hazard-table entries.

**Engineering decisions**
- Reclaim consumes reachability proof; it does not infer it from storage shape.
- Hazard and lease tables are physical authority, not diagnostics.
- Future chunk barriers exist as stability placeholders until S.7 owns blob
  lifecycle and retention.

**Open questions**
- None.

### Phase 11: Reserve Tier Movement And Blob Migration Stability Without Claiming S.7

Phase 11 gives S.5 enough typed stability vocabulary for tier movement and
future blob migration reads without implementing native blob lifecycle or S.6
I/O QoS.

**Relevant subsystems**
- `forge-store-physical-isolation`
- `forge-store-physical-format`
- `forge-store-certification`

**Relevant APIs**
- `TierMovementReadInterlockPlan`
- `ChunkMigrationReadInterlockPlan`
- `PhysicalChunkStabilityPlaceholder`
- `TierMovementStabilityVerdict`
- `FutureBlobMigrationNonClaim`

**Warnings**
- Do not implement S.7 chunk trees or blob retention in S.5.
- Do not claim cold-tier performance, hardware QoS, or chunk dedupe behavior.
- Do not allow future chunk placeholders to become blob authority.

**Test requirements**
- Adversarial equivalence: stable chunk or extent placeholders preserve
  reference, generation, epoch, and reachability proof across independent
  tier-movement read-plan admissions.
- Adversarial denial: missing chunk epoch, stale extent generation, copied
  migration labels, unsupported tier movement, and blob-lifecycle claims deny
  before physical read stability is admitted.
- Non-claim proof: S.5 evidence explicitly reports that blob chunk lifecycle,
  dedupe, resumable writes, and blob retention remain S.7 scope.

**Engineering decisions**
- S.5 may protect future chunk reads; it does not own blob semantics.
- Tier movement is treated as physical structure movement under read stability
  law.
- S.6 will later decide I/O pacing and media capability for tier movement.

**Open questions**
- None.

### Phase 12: Scale The Roadmap 2 Harness For Interleaving Families

Phase 12 improves the current S3/S4 harness pattern so S.5 and later milestones
can add hostile interleavings without producing one-off test piles.

**Relevant subsystems**
- `forge-store-certification`
- `forge-store-test-support`
- `forge-store-physical-isolation`
- `forge-proof`

**Relevant APIs**
- `PhysicalScenarioDefinition`
- `PhysicalScenarioPlan`
- `PhysicalInterleavingSchedule`
- `MaintenanceActorPlan`
- `PhysicalProofOracleVerdict`
- `PhysicalStoryTranscript`
- `MutationValidationMatrix`
- `HarnessLaneRegistry`

**Warnings**
- Do not create a second S.5-only runner that bypasses the Roadmap 2 harness.
- Do not put oracle meaning in test support drivers.
- Do not make interleavings random without replayable schedules and transcript
  identity.
- Do not allow scenario definitions to omit drivers, observers, or expected
  counters.

**Test requirements**
- Adversarial equivalence: the same scenario definition and seed lower to the
  same interleaving schedule, actor plan, oracle set, expected counters, and
  transcript identity across independent harness runs.
- Adversarial denial: scenario definitions missing actor roles, latch order,
  epoch basis, forbidden shortcut expectations, counter expectations, or
  transcript identity fail plan admission.
- Mutation proof: required S.5 mutants for early reclaim, stale epoch reuse,
  latch inversion, in-place compaction overwrite, and mixed-root read all fail
  their intended suite lanes.
- Scaling proof: adding a new interleaving family requires registering a lane,
  drivers, observers, oracles, transcript fields, and mutation expectations in
  one coherent harness topology.

**Engineering decisions**
- S.5 upgrades the harness around interleaving schedules because S.6-S12 will
  need the same shape for I/O pressure, blob movement, repair, security, and
  certification.
- Test support supplies mechanics; certification owns proof meaning.
- Every suite must keep positive, hostile, forbidden-shortcut, reopen/retry, and
  mutant lanes explicit.

**Open questions**
- None.

### Phase 13: Materialize Foundational And Proof Evidence From Executed Isolation

Phase 13 exports S.5 evidence through Foundational and Proof vocabulary without
letting those exports become Store physical stability authority.

**Relevant subsystems**
- `forge-store-physical-isolation`
- `forge-foundational`
- `forge-proof`
- `forge-store-certification`

**Relevant APIs**
- `PhysicalIsolationEvidenceBundle`
- `PhysicalIsolationBoundaryRoleClaim`
- `PhysicalIsolationCounterBackedPerformanceReceipt`
- `StableReadPlanProofArtifact`
- `PhysicalIsolationProofProgression`
- `ProjectionCannotMintPhysicalStabilityDenial`

**Warnings**
- Do not build evidence from plans, logs, labels, expected errors, or copied
  read-plan fields.
- Do not let Foundational diagnostics, performance receipts, boundary role
  claims, or Proof artifacts satisfy Store APIs requiring stable read plans,
  latch proofs, epoch scopes, or reclaim eligibility.
- Do not materialize rich diagnostics on the hot path unless policy admits it.

**Test requirements**
- Adversarial equivalence: two executed isolation lanes with the same read-plan
  receipt and counter basis materialize the same Foundational diagnostics,
  performance receipt, canonical basis, and Proof progression trace.
- Adversarial denial: logs, copied counters, projection rows, same-run
  self-comparison, support-only reports, and planned-work artifacts cannot mint
  Store stable read plans or reclaim eligibility.
- Profile proof: reduced-richness evidence profiles elide optional forensic
  material while preserving read outcome, epoch retry posture, latch counters,
  and reclaim decision.

**Engineering decisions**
- Store physical isolation findings are the authority.
- Foundational standardizes exported boundary meaning and performance evidence.
- Proof standardizes progression shape where S.5 states need sealed movement.
- Evidence policy must preserve hot-path cost boundaries.

**Open questions**
- None.

### Phase 14: Publish S.6 I/O And QoS Isolation Readiness

Phase 14 publishes the typed S.6 handoff: physical-stability assumptions, latch
wait surfaces, protected-footprint counters, background-interference hints, and
unsupported QoS claims.

**Relevant subsystems**
- `forge-store-physical-isolation`
- `forge-store-physical-backend`
- `forge-store-certification`
- `forge-store-readiness`

**Relevant APIs**
- `S6IoQosIsolationReadiness`
- `PhysicalIsolationCloseoutReport`
- `PhysicalIsolationCounterSnapshot`
- `ForegroundInterferenceSurface`
- `BackgroundMaintenanceIsolationAssumption`
- `UnsupportedQoSClaim`

**Warnings**
- Do not claim p99/p999 latency or hardware queue-depth behavior in S.5.
- Do not hide latch waits or blocked maintenance from S.6.
- Do not pass S.6 generic logs; handoff fields must be typed and
  counter-backed.

**Test requirements**
- Adversarial equivalence: identical executed S.5 closeout evidence produces
  the same S.6 readiness payload, isolation assumptions, unsupported-QoS list,
  and foreground-interference surface.
- Adversarial denial: copied closeout reports, logs, same-process metrics,
  synthetic wait labels, and missing latch/reclaim counters cannot satisfy S.6
  readiness.
- Non-claim proof: S.6 handoff names the exact QoS, hardware, media, queue,
  and latency claims that S.5 does not make.

**Engineering decisions**
- S.6 consumes physical-stability and wait/interference surfaces.
- S.5 does not certify hardware-aware I/O behavior.
- The handoff must be concrete enough for S.6 to pace background work without
  reopening S.5 read stability.

**Open questions**
- None.

### Phase 15: Close Physical Isolation, Latches, Epochs, And Stable Read Plans

Phase 15 runs the named S.5 suites, rejects synthetic shortcuts, verifies
interleaving safety, and records S.6 readiness.

**Relevant subsystems**
- `forge-store-physical-isolation`
- `forge-store-certification`
- `forge-store-test-support`
- `forge-store-recovery-physics`
- `forge-store-physical-integrity`
- `forge-foundational`
- `forge-proof`

**Relevant APIs**
- `PhysicalIsolationCloseoutSuite`
- `PhysicalIsolationCertificationBundle`
- `PhysicalIsolationCloseoutReport`
- `SyntheticPhysicalIsolationShortcutRejectionReport`
- `S6IoQosIsolationReadiness`

**Warnings**
- Do not close S.5 on single-threaded read success.
- Do not close S.5 on latch wrappers without hostile interleavings.
- Do not claim S.6 I/O QoS, S.7 blob lifecycle, S.8 layout discipline, S.10
  repair, S.11 security, or S.12 aerospace-grade certification from S.5.
- Do not leave S.6 with untyped latch waits, hidden background interference, or
  ambiguous protected-footprint surfaces.

**Test requirements**
- Adversarial closeout: hostile interleavings for read-during-compaction,
  read-during-checkpoint, read-during-reclaim, read-during-tier-movement,
  read-during-future-blob-migration, and restart during cutover all preserve
  stable physical read outcomes or typed retry/denial.
- Adversarial denial: semantic snapshot-as-physical-authority, raw page lists,
  copied read-plan fields, latch-order inversion, stale epoch reuse, early
  reclaim, backend residue, same-run self-comparison, and fixture labels deny
  or fail certification.
- Boundedness proof: exact counters prove latch attempts, waits, epoch retries,
  stale-plan denials, protected references, blocked reclaim, publication swaps,
  copied pages, and read-plan footprints remain within declared envelopes.
- Harness proof: every S.5 suite maps to scenario definition, lowered plan,
  interleaving schedule, drivers, observers, oracles, transcript families,
  positive lane, hostile lane, forbidden-shortcut lane, retry/reopen lane, and
  mutation-validation lane.
- Handoff proof: S.6 receives typed physical-stability assumptions,
  foreground-interference surfaces, wait/retry counters, blocked-maintenance
  counters, and explicit unsupported-QoS claims.
- Line-cap and composition proof: production, test, and support files stay
  within workspace line-cap rules and keep latch, epoch, read-plan,
  publication, reclaim, harness, evidence, and handoff responsibilities
  separate.

**Engineering decisions**
- S.5 closeout proves online physical read stability only.
- S.5 explicitly reserves hardware QoS, native blob lifecycle, layout/index
  strategy, repair, security, and full database certification for later
  sequences.
- The closeout must be strong enough for S.6 to begin foreground/background I/O
  pacing without reopening physical isolation.

**Open questions**
- None.

## Must Ship

- typed consumption of `S5PhysicalIsolationRecoveryReadiness`
- physical/semantic isolation separation with explicit cross-layer correlation
- root, manifest, segment, extent, page, and future chunk epoch vocabulary
- declared latch classes, modes, acquisition order, wait counters, and deadlock
  prevention or detection policy
- stable physical read-plan admission carrying root epoch, manifest epoch,
  physical references, generation proofs, latch requirements, reachability
  barriers, footprint counters, retry posture, and release semantics
- execution-ready read handles that cannot re-decide root, latch, epoch,
  footprint, or reachability strategy
- copy-on-write or equivalent physical publication for moved or rewritten
  reachable structures
- read-during-compaction, read-during-checkpoint, read-during-reclaim,
  read-during-tier-movement, and read-during-future-blob-migration interlocks
- restart-during-cutover behavior that exposes either the old stable root or
  the new stable root, never a mixed tree
- reachability, hazard, lease, protected-reference, and reclaim eligibility
  barriers for pages, extents, and future chunks
- quarantine interlock consuming S.3 damage locality so damaged bytes cannot be
  stabilized by movement
- exact latch, wait, epoch-retry, stale-plan-denial, protected-reference,
  blocked-reclaim, copied-page, publication-swap, and read-plan footprint
  counters
- Roadmap 2 interleaving harness extensions for maintenance actor plans,
  deterministic schedules, observers, oracles, transcripts, and mutation
  validation
- Foundational and Proof-compatible evidence from executed Store isolation
  findings, plus projection-authority denials
- concrete `S6IoQosIsolationReadiness` handoff payload

## Must Preserve

- Store owns physical byte stability.
- `forge-relational` owns semantic MVCC, transaction visibility, branch truth,
  and snapshot meaning.
- `forge-store-recovery-physics` owns S.4 recovery correctness; S.5 consumes
  recovered roots and stability assumptions.
- `forge-store-physical-integrity` owns S.3 damage detection and quarantine
  locality; S.5 consumes that posture and does not repair damage.
- `forge-foundational` owns shared evidence and boundary vocabulary, not Store
  physical stability authority.
- `forge-proof` owns progression law, not latch, epoch, read-plan, or reclaim
  semantics.
- S.5 does not claim S.6 hardware-aware I/O QoS, S.7 blob lifecycle, S.8 layout
  strategy, S.10 repair, S.11 security, or S.12 certification.
- Diagnostic richness does not alter read-plan admission, execution, or
  publication outcomes.

## Acceptance Evidence

S.5 is complete only when the store satisfies the Roadmap 2 named suite:

- `Physical isolation, latch, epoch, and stable-read-plan test`

Required machine-checkable outputs:

- `physical_isolation_story_transcript`
- `physical_isolation_scenario_definition`
- `physical_isolation_scenario_plan`
- `physical_interleaving_schedule`
- `maintenance_actor_plan`
- `stable_read_plan_trace`
- `latch_order_trace`
- `latch_wait_counter_trace`
- `epoch_comparison_trace`
- `epoch_retry_trace`
- `copy_on_write_publication_trace`
- `read_during_compaction_trace`
- `read_during_checkpoint_trace`
- `read_during_reclaim_trace`
- `read_during_tier_movement_trace`
- `future_blob_migration_non_claim_trace`
- `reachability_barrier_trace`
- `blocked_reclaim_report`
- `quarantine_interlock_trace`
- `restart_during_cutover_trace`
- `foreground_interference_surface`
- `foundational_isolation_evidence_bundle`
- `proof_progression_isolation_trace`
- `projection_authority_denial_trace`
- `synthetic_isolation_shortcut_rejection_report`
- `physical_isolation_mutation_validation_report`
- `S6IoQosIsolationReadiness`

Required acceptance suites:

- `s5_entry_authority_suite`
  proves S.5 consumes typed S.4 readiness and rejects copied recovery fields,
  live runtime state, terminal projections, and semantic snapshots.
- `physical_semantic_isolation_separation_suite`
  proves semantic MVCC tokens correlate with but cannot mint Store physical
  stability.
- `epoch_scope_and_stale_plan_suite`
  proves root, manifest, segment, extent, page, and future chunk epochs are
  scoped and stale-plan denials occur before byte reads.
- `latch_order_deadlock_suite`
  proves canonical latch ordering, wait accounting, and deadlock prevention or
  detection under hostile schedules.
- `stable_read_plan_admission_suite`
  proves plans carry protected references, latch requirements, epoch basis,
  footprint counters, reachability barriers, retry posture, and release
  semantics.
- `stable_read_execution_non_redecision_suite`
  proves execution consumes admitted plans and cannot re-plan isolation
  strategy after seeing moved or changed bytes.
- `copy_on_write_publication_suite`
  proves moved or rewritten reachable structures publish through old-root
  preservation and stable epoch transitions.
- `read_during_compaction_suite`
  proves readers admitted before and after compaction cutover observe stable
  physical structures for their admitted plan.
- `read_during_checkpoint_suite`
  proves checkpoint publication does not expose mixed old/new root or pageLSN
  frontier state to admitted readers.
- `reclaim_reachability_suite`
  proves active reads, scrub windows, recovery verifiers, checkpoints,
  quarantine holds, and future chunk holds block reclaim until release.
- `tier_and_future_blob_migration_stability_suite`
  proves S.5 protects physical movement placeholders without claiming S.7 blob
  lifecycle or S.6 QoS.
- `interleaving_harness_scaling_suite`
  proves S.5 scenarios lower into deterministic schedules with actors,
  observers, oracles, transcripts, counters, and mutation expectations.
- `foundational_proof_isolation_evidence_suite`
  proves executed Store isolation findings materialize into Foundational and
  Proof-compatible evidence while projection/support/planned/receipt surfaces
  cannot become Store physical stability authority.
- `synthetic_isolation_test_rejection_suite`
  proves logs, fixture labels, same-run self-comparison, copied fields,
  semantic snapshots, backend residue, and test-support-owned oracles cannot
  close S.5.
- `s6_io_qos_readiness_handoff_suite`
  proves S.6 receives typed physical-stability assumptions, wait/retry
  counters, foreground-interference surfaces, blocked-maintenance counters, and
  explicit unsupported-QoS claims.

Every suite must map to its scenario definitions, lowered plans, interleaving
schedules, required drivers, observers, proof oracles, transcript families,
evidence bundle fields, positive control lane, hostile lane,
forbidden-shortcut lane, retry/reopen lane, and mutation-validation lane. Every
suite must name exact counters that must be positive, exact counters that must
remain zero, and the mutants it is expected to kill.

## Allowed Debt

S.5 may reserve advanced lock-free read paths, backend-specific latch
acceleration, richer operator-facing wait visualizations, and fully native blob
chunk migration lifecycle for later sequences when the ordinary physical
isolation law already exists.

S.5 may not mark these as debt:

- typed S.4 readiness consumption
- physical/semantic isolation separation
- epoch vocabulary and stale-plan denials
- latch ordering and deadlock policy
- stable read-plan admission
- execution non-redecision
- copy-on-write or equivalent publication for reachable structure rewrites
- read-during-compaction safety
- read-during-checkpoint safety
- read-during-reclaim safety
- reachability and hazard barriers
- quarantine movement/reclaim interlock
- restart-during-cutover stability
- exact latch, epoch, read-plan, publication, and reclaim counters
- deterministic interleaving harness support
- mutation validation for required S.5 defects
- synthetic shortcut rejection
- Foundational/Proof evidence from executed Store isolation findings
- projection-versus-authority denial for all non-Store evidence surfaces
- concrete S.6 readiness handoff

## Sequencing Notes

S.5 belongs immediately after S.4 because physical isolation needs recovered
roots, pageLSN frontiers, replay receipts, source-precedence traces, recovery
counters, and explicit stability assumptions before it can admit stable read
plans. It belongs before S.6 because hardware-aware I/O and QoS cannot protect
foreground latency honestly until the foreground read's physical stability
window, latch waits, blocked maintenance, and protected footprint are explicit.

Later sequences consume S.5 as follows:

- S.6 consumes latch wait, foreground-interference, background-blocking,
  protected-footprint, and stability-assumption surfaces for I/O pacing.
- S.7 consumes future chunk stability and reachability placeholders when native
  blob chunk lifecycle becomes authoritative.
- S.8 consumes stable read plans and publication/reclaim barriers when access
  path layout families declare read/write amplification and rebuild behavior.
- S.9 formalizes physical read lease, reclaim, compaction cutover, and
  checkpoint/read interleaving state machines.
- S.10 consumes reachability, quarantine interlock, and stable read evidence for
  backup, offline verification, PITR, and repair planning.
- S.11 consumes physical isolation boundaries when tenant, encryption, audit,
  and secure-delete behavior must respect stable physical reads.
- S.12 consumes S.5 as one required physical database certification lane.

## Required Self-Check

- Does S.5 solve a real structural problem? Yes: it makes physical byte reads
  stable while maintenance moves, publishes, and reclaims storage underneath
  readers.
- Is the adversarial constraint precise and load-bearing? Yes: it names the
  hostile interleavings, stale-generation failures, half-publication failures,
  reclaim failures, and authority confusions S.5 must survive.
- Does the roadmap justify this milestone now? Yes: Roadmap 2 places S.5 after
  recovery physics and before I/O QoS because stable read windows must exist
  before foreground/background I/O pacing can be meaningful.
- Does the spec preserve crate authority boundaries? Yes: Store owns physical
  stability, Relational owns semantic MVCC, S.4 owns recovery, S.3 owns
  integrity, Foundational owns shared evidence vocabulary, and Proof owns
  progression law.
- Are the phases carrying most of the real design information? Yes.
- Is each phase centered on one conceptual detail or boundary? Yes.
- Does each phase contain at least two adversarial tests? Yes.
- Could a competent engineer map this spec into honest types, modules, and
  tests? Yes: phases name subsystems, APIs, warnings, tests, decisions, and
  harness outputs.
- Does the milestone belong in this roadmap sequence? Yes: S.5 is the required
  bridge from deterministic recovery to hardware-aware foreground/background
  I/O behavior.
