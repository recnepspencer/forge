# Storage Foundation S.5 Engineering Spec: Physical Isolation, Latches, Epochs, And Stable Read Plans

> **Status:** Planned
>
> **Roadmap parent:** [physical-database-roadmap.md](/Users/Esther/Documents/Programming/worth_workspace/worth/_docs/worth-store/physical-database-roadmap.md)
>
> **Primary prerequisite:** `S.4.5 Physical Database Simulation Harness`
>
> **Follow-on storage-foundation sequence:** `S.6 Hardware-Aware I/O, QoS, And Background Work Pacing`
>
> **Primary architectural driver:** make physical byte reads stable while
> checkpointing, compaction, reclaim, tier movement, and future blob migration
> move physical structures underneath readers, without pretending Store owns
> semantic MVCC visibility.

## Goal

Make Worth Store physical reads stable under concurrent maintenance.

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
deterministic after crash. S.4.5 made hostile physical simulation reusable,
deterministic, aspect-native, production-boundary-backed, and
certification-owned. S.5 is the next required boundary: while the store is
online, readers need physical byte stability even as maintenance rewrites,
moves, publishes, and reclaims physical structures.

This is not semantic MVCC. `worth-relational` owns transaction visibility,
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
- `physical-database-roadmap.md`
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
- `storage-foundation-s4-5.md`
  gives S.5 the reusable physical simulation harness: deterministic schedules,
  maintenance actors, production-boundary drivers, certification-owned oracles,
  exact counters, replayable transcripts, forbidden-shortcut denials, and
  `S5SimulationHarnessReadiness`.

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
- S.5 consumes `S5PhysicalIsolationRecoveryReadiness` and
  `S5SimulationHarnessReadiness`; it must not accept copied S.4 closeout
  fields, live runtime cache state, semantic MVCC snapshots, or a generic local
  runner as physical stability authority.
- Stable read plans are admitted, proof-bearing physical artifacts. They are not
  collections of page ids plus comments.
- Physical latches protect mutation of physical structures. Epochs detect stale
  observations inside declared stability scopes; hazards, leases, guards,
  pins, latches, and COW/RCU preservation keep bytes reachable and valid.
- Copy-on-write or an equivalent publication protocol is mandatory wherever
  maintenance rewrites reachable physical structure.
- Reclaim is a proof consumer, not a scavenger. It may reclaim only bytes that
  are unreachable by every admitted read, checkpoint, recovery, scrub, and
  future blob reachability barrier.
- S.5 may expose assumptions to S.6 about latch waits, background interference,
  and protected byte footprints, but it does not claim hardware I/O QoS.

## S.4.5 Harness Consumption Rules

- Every hostile S.5 interleaving suite must enter through the completed S.4.5
  scenario-authoring, plan-lowering, schedule, execution, observer, oracle,
  transcript, coverage, and evidence pipeline. A local S.5-only runner is a
  shortcut, even if it produces the same-looking rows.
- S.5 entry must consume `S5SimulationHarnessReadiness` and
  `S5HarnessReadinessReceipt` before any S.5 simulation lane can count as
  certification evidence.
- S.5 may extend `PhysicalSimulationScenarioFamily` with S.5-owned physical
  isolation families, but those families must lower through
  `PhysicalScenarioBuilder`, `PhysicalSimulationScenarioDefinition`,
  `lower_physical_simulation_plan`, `PhysicalSimulationPlan`,
  `PhysicalInterleavingSchedule`, `ReplaySeed`, `StateSpaceBudget`, and the
  existing partial-order-reduction posture rather than bypassing them.
- S.5 suites must use certification-owned `PhysicalProofOracle` families and
  `PhysicalProofOracleVerdict` values. Test support may drive production
  boundaries and build fixtures; it may not decide whether a read was stable.
- S.5 must reuse the S.4.5 transcript and evidence closure:
  `PhysicalSimulationTranscript`, `SimulationReplayBundle`,
  `PhysicalCertificationEvidenceBundle`, generated coverage rows, mutation
  evidence, and forbidden-shortcut rejection. Logs, summaries, elapsed time,
  same-run self-comparison, fixture labels, private mutation, JSON authority,
  and copied readiness fields remain non-evidence.
- Every phase that introduces a new hostile family must name its required
  actors, production-facing drivers, observer traces, oracle family, counter
  contract profile, replay transcript fields, and mutation-validation row in
  that phase rather than relying on the Phase 12 closeout to remember them.

## Physical Isolation Laws

- Physical/Semantic Isolation Separation Law: semantic MVCC visibility cannot
  admit, extend, or release physical byte stability. Store read plans must carry
  Store-owned root, page, segment, extent, generation, epoch, and reachability
  proof.
- Stable Read Plan Law: every nontrivial physical read must be admitted through
  a plan that names root epoch, manifest epoch, physical references, generation
  proofs, latch requirements, reachability barriers, footprint counters, and
  retry posture before execution.
- Protect-Before-Observe Law: a reader may not rely on a current root, manifest,
  page, extent, segment, or future chunk reference until it has published a
  hazard, lease, or reader epoch that can block reclaim, then revalidated the
  epoch/generation basis under the declared stability scope. An admitted
  implementation may instead use a double-collect protocol that reloads and
  validates the root/epoch after protection is published, denying or retrying
  on drift.
- Epoch Honesty Law: root, manifest, segment, extent, page, and future chunk
  epochs may be compared only inside a declared stability scope. A successful
  comparison outside that scope is a projection, not authority.
- Physical Byte Guard Law: a reachability lease prevents reclaim; it is not
  automatically permission to dereference bytes. Execution may read bytes only
  through a guard proving the frame, mmap view, extent window, or owned read
  buffer remains valid until the read receipt is completed or denied.
- Traversal Admission Law: any traversal needed to discover the protected
  footprint is part of read-plan admission. Traversal must use temporary guards,
  scoped epoch/generation validation, and retry or denial receipts before it can
  lower into an execution-ready plan or stepwise cursor.
- Latch Order Law: latch acquisition must follow a declared partial order or
  deny with typed deadlock-prevention evidence before waiting can create a
  cycle.
- No Hidden Latch-I/O Law: ordinary read execution may not hold high-level
  structural latches across blocking storage I/O unless the plan declares that
  cost and the S.6 handoff receives the exact wait/interference surface.
- Copy-On-Write Publication Law: maintenance may publish rewritten physical
  structure only by creating a new reachable version, durably publishing the
  new root or manifest, and preserving old reachability until admitted readers
  release or expire.
- Root Kind Separation Law: `CurrentPhysicalRoot`, `CheckpointPublicationRoot`,
  `RecoveryRoot`, and `ManifestLocatorRoot` are distinct authority surfaces.
  A read plan must name which root kind it admits against, and no checkpoint or
  recovery root may masquerade as the current foreground read root.
- Reclaim Reachability Law: reclaim may consume only executed reachability
  evidence and live hazard/lease tables. Backend residue, directory listing,
  last-observed page ids, and copied read-plan fields cannot prove reclaim
  eligibility.
- Lease Expiry Non-Authority Law: expiry is not reclaim authority unless the
  system has also proven the expired handle can no longer dereference protected
  bytes, or the read has been completed, revoked, or converted into an owned
  copy-stable representation.
- Free/Reuse Generation Fence Law: a page, extent, segment, or future chunk
  identity may not be reused until prior reachability removal, reclaim
  eligibility, generation advancement, and allocator publication have been
  admitted in one crash-stable free/reuse posture.
- Quarantine Stability Law: quarantined or unresolved physical damage remains
  movement-blocking or read-denying until a later repair sequence admits a new
  posture. S.5 cannot make damaged bytes stable by moving them.
- Restart Stability Law: restart during physical cutover must recover either
  the old stable root or the new stable root with typed cutover posture; it may
  not expose a mixed tree.
- Publication Memory Ordering Law: root swaps, hazard slot publication, reader
  epoch publication, generation advancement, allocator publication, and read
  validation must name acquire/release or stronger ordering rules. Relaxed or
  backend-ambient ordering cannot satisfy S.5 authority.
- Diagnostic Non-Interference Law: rich latch, epoch, wait, and reachability
  diagnostics may be materialized by policy, but they must not change read-plan
  admission or publication outcome.

## Planned Directory Skeleton

`workspaces/worth-store/crates/worth-store-physical-isolation/src/`

- `lib.rs`
  exposes the crate facade and re-exports only proof-bearing S.5 boundary
  types.
- `readiness.rs`
  consumes `S5PhysicalIsolationRecoveryReadiness` and produces physical
  isolation entry authority.
- `physical_snapshot_boundary.rs`
  keeps semantic snapshot identifiers out of physical stability admission while
  preserving explicit cross-layer correlation.
- `root_protocol/`
  owns current, checkpoint-publication, recovery, and manifest-locator root
  authority types plus protect-before-observe admission sequencing.
- `epoch/`
  owns root, manifest, segment, extent, page, and future chunk epoch tokens,
  comparison scopes, retry decisions, and stale-plan denials.
- `latch/`
  owns latch classes, latch order, acquisition plans, wait counters, and
  deadlock prevention or detection reports.
- `byte_guard/`
  owns frame pins, mmap view guards, extent-window guards, owned read-buffer
  guards, and guard release receipts.
- `read_plan/`
  owns stable read plan admission, protected reference sets, read-plan
  footprint accounting, execution-ready read handles, and release receipts.
- `traversal_admission/`
  owns guarded traversal, temporary footprint discovery, stepwise read cursors,
  and traversal retry or denial receipts.
- `publication/`
  owns copy-on-write publication plans, root/manifest swap receipts, old-root
  preservation, and crash-restart cutover posture.
- `maintenance_interlock/`
  owns read-during-compaction, read-during-checkpoint, read-during-reclaim,
  read-during-tier-movement, and future read-during-blob-migration safety
  rules.
- `reachability/`
  owns hazard, lease, protected-reference, and reclaim eligibility tables.
- `free_reuse/`
  owns crash-stable free/reuse generation fences and allocator publication
  posture.
- `memory_ordering.rs`
  owns declared acquire/release or stronger ordering requirements for root
  swaps, hazard slot publication, reader epochs, generation advancement,
  allocator publication, and read validation.
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

`workspaces/worth-store/crates/worth-store-certification/src/`

- `s5_physical_isolation_harness/`
  owns S.5 certification registration over the S.4.5 harness: physical
  isolation scenario families, suite lane declarations, oracle selection,
  mutation-validation matrices, generated coverage expectations, and closeout
  evidence assembly. It must not implement a new runner or duplicate S.4.5
  lowering, scheduling, transcript, or evidence machinery.

`workspaces/worth-store/crates/worth-store-test-support/src/`

- `s5_physical_isolation/`
  owns reusable mechanics that plug into the S.4.5 harness: production-facing
  maintenance actors, latch scheduler adapters, epoch drift injectors, reclaim
  adversaries, restart-at-cutover fixtures, and deterministic yieldpoint
  bindings. It may expose drivers and fixtures, not oracle meaning or
  certification verdicts.

## Phase Plan

### Phase 1: Admit S.4 Recovery Readiness Into Physical Isolation Entry

Phase 1 freezes the S.4-to-S.5 boundary. It admits only recovered physical roots,
pageLSN frontiers, replay receipts, source-precedence traces, recovery counters,
and explicit stability assumptions from S.4.

**Relevant subsystems**
- `worth-store-recovery-physics`
- `worth-store-physical-isolation`
- `worth-store-readiness`
- `worth-store-certification`

**Relevant APIs**
- `S5PhysicalIsolationRecoveryReadiness`
- `S5SimulationHarnessReadiness`
- `S5HarnessReadinessReceipt`
- `S5SimulationHarnessReadinessDenial`
- `RecoveredPhysicalState`
- `RecoverySourceDecisionTrace`
- `RedoExecutionReceipt`
- `RecoveryCounterSnapshot`
- `PhysicalIsolationEntryAdmission`
- `PhysicalIsolationEntryDenial`
- Foundational boundary evidence: `FoundationalBoundaryEvidenceExecutedReceiptArtifact`,
  `FoundationalBoundaryEvidenceProvenanceArtifact`,
  `FoundationalBoundaryEvidenceSourceBasis`, and
  `FoundationalBoundaryEvidenceFreshnessPosture` for S.4 receipts, source
  basis, and freshness disclosure at the entry boundary.
- Proof progression: `Recipe<Unresolved, S5EntryRequest>`,
  `Recipe<Resolved, S5EntryBasis>`, `AuthorityWitness<S5EntryAuthority>`,
  `AssumptionBasis<S4RecoveryReadinessBasis>`, and checked entry outcomes for
  admitted, denied, stale, and rebind-required recovery handoffs.

**Warnings**
- Do not reconstruct S.5 entry from copied S.4 closeout fields.
- Do not reconstruct S.5 harness entry from copied S.4.5 readiness fields,
  generated reports, coverage rows, terminal projections, or test logs.
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
- Harness admission proof: S.5 certification lanes cannot register until
  `S5SimulationHarnessReadiness` and its readiness receipt admit the completed
  S.4.5 harness; copied readiness fields, wrong-sequence maturity evidence,
  unsupported-profile evidence, and missing S.5 correctness non-claim evidence
  deny before any S.5 scenario can run.
- Foundational/Proof proof: S.4 receipt, provenance, source-basis, and
  freshness fields lower into the named Foundational evidence surfaces, while
  S.5 entry progression carries a Proof assumption basis and cannot advance
  without the Store-owned entry authority witness.

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
- `worth-store-physical-isolation`
- `worth-store-authority`
- `worth-relational`
- `worth-store-certification`

**Relevant APIs**
- `PhysicalSnapshotCorrelation`
- `SemanticVisibilityReference`
- `PhysicalReadStabilityAuthority`
- `SemanticVisibilityCannotMintPhysicalStability`
- Foundational boundary roles: `AuthoritativeCurrentRole`,
  `DerivedProjectionRole`, `SupportOnlyRole`, `ReceiptEvidenceRole`, and
  `FoundationalBoundaryRoleClaim` to label semantic visibility references,
  diagnostic projections, support reports, and executed Store authority without
  collapsing them.
- Proof witnesses: `AuthorityWitness<PhysicalReadStabilityAuthority>`,
  `CapabilityWitness<SemanticCorrelationCapability>`, and checked transition
  outcomes that deny projection-to-authority promotion.

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
- Role proof: every cross-layer surface is tagged as authoritative current,
  derived projection, support-only, or receipt evidence, and only the Store
  physical stability authority role can enter read-plan admission.

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
- `worth-store-physical-isolation`
- `worth-store-physical-format`
- `worth-store-recovery-physics`
- `worth-store-certification`

**Relevant APIs**
- `CurrentPhysicalRoot`
- `CheckpointPublicationRoot`
- `RecoveryRoot`
- `ManifestLocatorRoot`
- `RootEpoch`
- `ManifestEpoch`
- `SegmentEpoch`
- `ExtentEpoch`
- `PageEpoch`
- `ChunkEpoch`
- `GenerationCountedPhysicalReference`
- `PhysicalEpochVector`
- `PhysicalReferenceGenerationMismatch`
- `RootKindMismatchDenial`
- `PhysicalOrderingContract`
- `EpochComparisonScope`
- `EpochRetryDecision`
- `StalePhysicalReadPlanDenial`
- Foundational canonicalization: `CanonicalBasisEntry`,
  `CanonicalBasisSequence`, `CanonicalEquivalenceBasis`, and
  `prepare_canonical_basis_sequence(...)` for epoch comparison evidence and
  reproducible stale-plan diagnostics.
- Proof basis/freshness: `AssumptionBasis<PhysicalEpochBasis>`,
  `FreshnessScopedBasis<CurrentValidity, _>`, `RebindRequired`,
  `BoundaryBridged<_>`, and checked freshness outcomes for epoch drift.

**Warnings**
- Do not compare epochs outside a declared stability scope.
- Do not collapse generation identity, LSN, pageLSN, and epoch into one value.
- Do not pass a checkpoint, recovery, or manifest-locator root to APIs
  requiring current foreground read-root authority.
- Do not make chunk epoch semantics claim S.7 blob lifecycle.
- Do not accept raw `PageId`, `ExtentId`, `SegmentId`, or `ChunkId` in any
  S.5 authority path after generation-counted references are available.
- Do not use relaxed or ambient memory ordering for root publication, hazard
  publication, generation advancement, allocator publication, or validation.

**Test requirements**
- Adversarial equivalence: repeated admission against unchanged root, manifest,
  segment, extent, and page epochs produces the same stable-read epoch basis.
- Adversarial denial: stale root epochs, page generation reuse, manifest epoch
  drift, extent replacement, and future chunk epoch mismatch deny or retry
  before bytes are read.
- Scope proof: epoch comparisons outside their declared scope fail rather than
  becoming ordinary boolean equality.
- Freshness proof: unchanged epoch bases remain current-valid Proof bases,
  drifted bases downgrade to stale or rebind-required forms, and no raw epoch
  equality may substitute for the declared Foundational canonical basis.
- ABA proof: stale physical references with matching ids but mismatched
  generations deny before plan admission, latch acquisition, or reclaim
  eligibility.
- Root-kind proof: current roots, checkpoint publication roots, recovery roots,
  and manifest locator roots cannot be substituted for each other without an
  explicit readmission transition.
- Memory-ordering proof: root swaps, hazard publication, reader epochs,
  generation advancement, allocator publication, and validation declare
  acquire/release or stronger ordering and expose tests that fail if the
  ordering is weakened.

**Engineering decisions**
- Epochs detect whether an observed physical stability basis is still current;
  they do not preserve bytes without
  latches, pins/guards, COW/RCU preservation, and reachability leases.
- Generations identify reused physical identities; epochs identify observed
  publication stability.
- S.5 uses generation-counted physical references to prevent ABA bugs from
  page, extent, segment, or future chunk reuse.
- Epoch comparison is represented as a small ordered physical epoch vector over
  declared stability scopes, not as ad hoc map lookup or unscoped equality.
- Root kinds are distinct typestate authorities, and read-plan admission must
  consume the exact root kind it admits against.
- S.5 memory ordering is a named contract because lock-free or RCU-like root
  publication is unsound without declared publication and validation ordering.
- Future chunk epochs exist only as stability placeholders until S.7 owns blob
  chunk lifecycle.

**Open questions**
- None.

### Phase 4: Define Latch Classes, Acquisition Order, And Deadlock Policy

Phase 4 makes physical latch behavior explicit and mechanically auditable.

**Relevant subsystems**
- `worth-store-physical-isolation`
- `worth-store-buffer-pool`
- `worth-store-certification`

**Relevant APIs**
- `PhysicalLatchClass`
- `PhysicalLatchMode`
- `PhysicalLatchKey`
- `CanonicalLatchAcquisitionOrder`
- `LatchAcquisitionPlan`
- `LatchOrderProof`
- `LatchWaitForGraph`
- `LatchWaitCounterSnapshot`
- `DeadlockPreventionDenial`
- `DeadlockDetectionReport`
- Proof structural collections: `CanonicalVec<LatchAcquisitionStep>`,
  `NonEmpty<LatchAcquisitionStep>`, `Pair<PhysicalLatchClass>`, and
  `Proof<CanonicalOrder, StructuralProofAuthority>` for canonical latch order.
- Foundational performance: `FoundationalCounterBackedPerformanceReceipt`,
  `FoundationalPerformanceCounterSpec`, `FoundationalPerformanceCounterRow`,
  and `FoundationalPerformanceContractName` for latch-attempt and wait evidence.

**Warnings**
- Do not rely on comments or convention for latch ordering.
- Do not let read plans acquire latches in execution order discovered after
  seeing pages.
- Do not hide blocking behind an ordinary read method.
- Do not protect S.5 latches with one global mutex, unordered lock sets, or
  runtime hash iteration order.

**Test requirements**
- Adversarial equivalence: two callers that request the same protected
  physical footprint in different input orders lower to the same canonical
  latch acquisition order.
- Adversarial denial: cyclic latch plans, mixed hierarchy inversions, upgrade
  attempts without upgrade authority, and execution-time latch discovery deny
  before waiting.
- Deadlock proof: deterministic hostile schedules either cannot form a wait
  cycle or emit typed deadlock detection evidence with exact wait counters.
- Structural proof: latch acquisition plans carry canonical-order proof through
  Proof structural wrappers, and the same execution emits Foundational
  counter-backed performance rows for attempts, waits, denied upgrades, and
  detected cycles.
- Algorithm proof: canonical latch order is stable across insertion order,
  hash seed, platform, and restart; if detection is selected for a latch family,
  the wait-for graph is bounded, typed, and counter-backed.

**Engineering decisions**
- S.5 must pick either prevention or detection per latch family and make that
  policy explicit.
- The default algorithm is deadlock prevention through canonical acquisition
  order over `PhysicalLatchKey` sorted by root/manifest/segment/extent/page or
  future chunk hierarchy plus latch class and mode.
- Deadlock detection is allowed only for latch families whose cost or
  compatibility makes strict ordering insufficient; those families must use a
  bounded wait-for graph with exact edge, wait, and cycle counters.
- Latch acquisition is a lowered plan, not executor discretion.
- Wait counters are part of the result surface because S.6 will later consume
  them for foreground interference accounting.

**Open questions**
- None.

### Phase 5: Admit Stable Physical Read Plans

Phase 5 defines the proof-bearing physical read plan that execution may consume
without rediscovering root, latch, epoch, or reachability strategy.

**Relevant subsystems**
- `worth-store-physical-isolation`
- `worth-store-buffer-pool`
- `worth-store-physical-format`
- `worth-store-certification`

**Relevant APIs**
- `UnprotectedReadIntent`
- `PublishedReaderHazard`
- `ProtectedRootObservation`
- `ValidatedRootObservation`
- `TraversalAdmissionGuard`
- `TraversalAdmissionReceipt`
- `SeedStableReadPlan`
- `StepwiseStableReadCursor`
- `StablePhysicalReadPlan`
- `StablePhysicalReadPlanAdmission`
- `StablePhysicalReadHandle`
- `PhysicalReadPlanFootprint`
- `CompactProtectedReferenceSet`
- `ProtectedReferenceRangeSet`
- `ReadPlanAdmissionScratchArena`
- `ProtectedPhysicalReferenceSet`
- `PhysicalReadPlanReleaseReceipt`
- `ReadPlanCounterSnapshot`
- Proof carriers: `Recipe<Resolved, PhysicalReadPlanFootprint>`,
  `Recipe<Lowered, LoweredStableReadPlan>`,
  `Recipe<Resolved, ProtectedRootObservation>`,
  `Recipe<Lowered, TraversalAdmissionReceipt>`,
  `ExecutionReadyRecipe<StablePhysicalReadPlan>`,
  `CanonicalVec<ProtectedPhysicalReference>`,
  `UniqueVec<ProtectedPhysicalReference>`, and
  `NonEmpty<ProtectedPhysicalReference>` for admitted read-plan shape.
- Foundational aspects and canonical basis: `AspectKey`,
  `ContractValidatedAspectValue`, `AuthoritativeRecordAspectState`,
  `CanonicalBasisBundle`, and `CanonicalDerivedDigest` for native plan
  evidence, diagnostic locators, and reproducible footprint identity.

**Warnings**
- Do not observe or cache a root pointer before publishing the reader hazard,
  lease, or epoch required to prevent reclaim.
- Do not let execution assemble protected page sets after admission.
- Do not hide footprint-discovery traversal inside execution.
- Do not let a read plan look cheap if it protects a broad physical footprint.
- Do not admit plans that omit release semantics.
- Do not allow a plan to cross quarantine, generation, or stale epoch denials.
- Do not represent large protected footprints as unbounded raw `Vec` scans when
  segment, extent, or page ranges can preserve the same proof more compactly.
- Do not allocate per protected reference on the foreground admission path.

**Test requirements**
- Adversarial equivalence: the same root, references, generation proofs, and
  epoch basis produce the same canonical read-plan footprint regardless of input
  order.
- Adversarial denial: missing release semantics, broad unbounded footprints,
  quarantined references, stale page generations, and execution-time reference
  discovery deny before read handles are issued.
- Cost proof: plan admission emits exact protected-reference, latch, epoch,
  resident-byte, and allocation counters.
- Native/proof proof: admitted plans are authored from native Store and
  Foundational aspect values, not JSON or projection rows, and they preserve
  Proof canonical/unique/non-empty reference-set wrappers through the execution
  readiness boundary.
- Harness proof: stable-read-plan admission scenarios are authored through the
  S.4.5 public scenario builder and lower into replayable schedules with
  `CounterContractOracle`, `TranscriptReplayOracle`, and mutation lanes for
  missing release semantics, stale generations, execution-time discovery, and
  unbounded protected-reference footprints.
- Structure proof: protected footprints use compact canonical range/set
  representations where the physical references are contiguous or
  extent-local, preserve uniqueness and order proofs, and expose exact range,
  reference, resident-byte, and scratch-allocation counters.
- Protect-before-observe proof: admission cannot produce
  `ProtectedRootObservation` until `PublishedReaderHazard` exists, and cannot
  produce `ValidatedRootObservation` until root/epoch/generation validation is
  repeated after protection publication or a declared reader-epoch protocol is
  proven.
- Traversal proof: index/tree/graph traversal needed to discover a footprint
  happens inside admission through temporary guards and produces either
  `StablePhysicalReadPlan`, `StepwiseStableReadCursor`, or a typed retry/denial
  receipt; execution cannot discover new footprint authority.

**Engineering decisions**
- Stable read plans are the only ordinary input to physical read execution.
- S.5 encodes admission as a Proof/typestate chain:
  `UnprotectedReadIntent -> PublishedReaderHazard ->
  ProtectedRootObservation -> ValidatedRootObservation ->
  TraversalAdmissionReceipt -> StablePhysicalReadPlan ->
  ExecutionReadyPhysicalReadPlan`.
- Read-plan admission uses pre-sized or arena-scoped scratch storage for
  sorting, deduplication, range coalescing, and proof construction; heap growth
  during normal foreground admission is a tested violation unless explicitly
  admitted by the workload envelope.
- Protected-reference sets are stored as compact canonical ranges plus
  singleton references when that preserves the declared footprint exactly;
  callers cannot observe a broad plan as if it were scalar.
- Direct known-footprint reads admit to `SeedStableReadPlan`; access paths that
  discover leaves or pages during guarded traversal admit to
  `StepwiseStableReadCursor` until they can lower to an execution-ready plan.
- The plan carries exactly the proofs established at admission.
- Release receipts are first-class so reclaim and maintenance can consume them.

**Open questions**
- None.

### Phase 6: Execute Stable Reads Without Re-Deciding Isolation Strategy

Phase 6 executes admitted plans and proves the executor cannot re-plan, widen,
or silently retry outside declared epoch policy.

**Relevant subsystems**
- `worth-store-physical-isolation`
- `worth-store-buffer-pool`
- `worth-store-physical-integrity`
- `worth-store-certification`

**Relevant APIs**
- `ExecutionReadyPhysicalReadPlan`
- `PhysicalByteGuard`
- `FramePinGuard`
- `MmapViewGuard`
- `ExtentWindowGuard`
- `OwnedReadBufferGuard`
- `ByteGuardReleaseReceipt`
- `StablePhysicalReadExecution`
- `StablePhysicalReadReceipt`
- `EpochRetryReceipt`
- `PhysicalReadExecutionDenial`
- Proof execution surfaces: `ExecutionReadyRecipe<StablePhysicalReadPlan>`,
  `ExecutedRecipe<StablePhysicalReadReceipt>`,
  `ExecuteReadyRecipeTransition`, `ProofOutcome`, and
  `TransitionOutcome` variants for success, denial, deferment, stale, retry,
  and rebind-required execution.
- Foundational receipts and diagnostics:
  `FoundationalBoundaryEvidenceExecutedReceiptArtifact`,
  `FoundationalBoundaryEvidenceCompletedReceiptArtifact`,
  `FoundationalDiagnosticRow`, `FoundationalDiagnosticOutcomeKind`, and
  provenance-ready diagnostic rows for executed read receipts and localized
  read denials.

**Warnings**
- Do not let execution widen the footprint because a page moved.
- Do not silently retry on epoch drift without recording retry posture.
- Do not read bytes after latch release or plan expiry.
- Do not dereference a frame, mmap region, extent window, or read buffer through
  a reachability lease alone.
- Do not hold high-level structural latches across blocking storage I/O unless
  the plan declares the latch/I/O cost surface.
- Do not treat integrity failures as isolation failures; consume S.3 damage
  posture distinctly.
- Do not perform allocation-heavy planning, latch ordering, reference-set
  expansion, or diagnostics materialization inside the read executor.

**Test requirements**
- Adversarial convergence: a stable read executing while maintenance publishes a
  new root either reads the old protected bytes or retries into a newly admitted
  plan with typed epoch-retry evidence.
- Adversarial denial: expired plans, released handles, stale epochs after retry
  budget, latch loss, quarantined bytes, and widened execution footprints deny
  with typed locality.
- Non-redecision proof: mutation testing verifies the executor cannot choose a
  new latch strategy, root, reference set, or reachability barrier.
- Outcome proof: execution uses Proof checked outcomes instead of flattening
  stale, retry, denial, quarantine, and rebind states into a generic error, and
  executed receipts lower into Foundational receipt/diagnostic rows only after
  Store execution has occurred.
- Hot-path proof: execution counters prove the executor consumes the lowered
  plan with zero plan allocations, zero broad footprint scans, and no hidden
  diagnostic materialization under minimal evidence policy.
- Byte-guard proof: every decoded or copied byte range is accessed through a
  guard whose lifetime covers the read receipt or denial, and tests fail if a
  reachability lease is used as a byte guard.
- Latch/I/O proof: ordinary read execution does not hold structural latches
  across blocking storage I/O; any admitted exception is named in the plan and
  exported to S.6 as wait/interference evidence.

**Engineering decisions**
- Execution consumes the lowered plan and may not choose isolation policy.
- Epoch retry is an explicit transition, not a loop hidden inside reads.
- The executor is a data-plane consumer of an execution-ready plan; it may only
  follow precomputed latch keys, protected ranges, epoch retry policy, and
  release obligations.
- Reachability, mutation exclusion, and byte-memory validity are separate
  authorities: lease/barrier, latch, and guard/pin respectively.
- The ordinary read path prefers guard/pin plus RCU/COW reachability over
  holding broad latches while bytes are fetched or decoded.
- S.3 integrity denials remain physically localized and do not become generic
  read failures.

**Open questions**
- None.

### Phase 7: Publish Copy-On-Write Physical Updates

Phase 7 defines how maintenance rewrites reachable physical structures without
invalidating admitted readers.

**Relevant subsystems**
- `worth-store-physical-isolation`
- `worth-store-recovery-physics`
- `worth-store-physical-format`
- `worth-store-certification`

**Relevant APIs**
- `CopyOnWritePublicationPlan`
- `PhysicalPublicationIntent`
- `ReadCopyUpdateRootPublication`
- `AtomicPhysicalRootSwap`
- `RootSwapOrderingContract`
- `RootPublicationEpoch`
- `ManifestPublicationEpoch`
- `PhysicalPublicationReceipt`
- `OldReachabilityPreservation`
- `CrashStableFreeReusePosture`
- `AllocatorPublicationFence`
- Proof composition: `Recipe<Lowered, CopyOnWritePublicationPlan>`,
  `AuthorityWitness<PhysicalPublicationAuthority>`,
  `join_ready(...)`, `Pair<OldRootProof, NewRootProof>`, and
  `DisjointPair<OldReachabilitySet>` where old and new structures must remain
  separated.
- Foundational transition/boundary evidence:
  `FoundationalBoundaryEvidenceExecutedReceiptArtifact`,
  `FoundationalBoundaryEvidenceLineageSubjectSet`,
  `FoundationalBoundaryEvidenceContinuityAttachmentScope`, and canonical
  basis entries for root/manifest publication receipts.

**Warnings**
- Do not overwrite reachable bytes in place while an admitted reader may still
  hold them.
- Do not publish a root or manifest without preserving old reachability.
- Do not treat checkpoint cutover receipts as copy-on-write publication
  receipts; S.4 and S.5 prove different things.
- Do not make publication depend on readers seeing mutable in-place root state
  or backend directory residue.
- Do not reuse a physical identity in the same breath as root publication unless
  reclaim eligibility, generation advancement, and allocator publication are
  admitted in a crash-stable free/reuse posture.

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
- Composition proof: Proof join surfaces combine old-root preservation,
  new-root publication, latch, and epoch readiness into one execution-ready
  publication, and Foundational evidence records lineage/continuity without
  becoming publication authority.
- Publication proof: root and manifest updates behave as RCU-style copy-on-write
  publication: readers admitted before the swap retain old reachability,
  readers admitted after the swap observe the new epoch, and reclaim waits for
  release proof.
- Ordering proof: COW/RCU root and manifest swaps declare acquire/release or
  stronger memory ordering for publication and validation, and weak-ordering
  mutants fail under deterministic interleavings.
- Free/reuse proof: crash before, during, and after free/reuse publication
  never admits both an old generation and reused identity as current authority.

**Engineering decisions**
- Copy-on-write is the default publication law for moved or rewritten physical
  structures.
- Publication uses an RCU/COW-style root or manifest swap discipline with an
  explicit old-root preservation record; in-place mutation of reachable
  structures is outside the ordinary S.5 lane.
- Free/reuse is not a side effect of publication. It is a separate
  crash-stable posture that joins reclaim eligibility, generation advancement,
  and allocator publication.
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
- `worth-store-physical-isolation`
- `worth-store-recovery-physics`
- `worth-store-physical-integrity`
- `worth-store-certification`

**Relevant APIs**
- `CompactionReadInterlockPlan`
- `CompactionRewritePublication`
- `CompactionCutoverStabilityProof`
- `ReadDuringCompactionVerdict`
- `CompactionProtectedReferenceSet`
- `CompactionCandidateRangeSet`
- `CompactionCutoverDelta`
- `CompactionDeferredReclaimQueue`
- Proof surfaces: `Recipe<Resolved, CompactionReadInterlockPlan>`,
  `CapabilityWitness<CompactionMaintenanceCapability>`,
  `CanonicalVec<CompactionProtectedReference>`,
  `DisjointPair<ReadProtectedSet>`, and checked transition outcomes for
  blocked, deferred, denied, and admitted compaction interlocks.
- Foundational diagnostics/performance:
  `FoundationalDiagnosticSubject`, `FoundationalDiagnosticDenialClass`,
  `FoundationalCounterBackedPerformanceReceipt`, and
  `FoundationalPerformanceSupportingEvidenceRow` for blocked compaction,
  copied pages, swaps, and retry counters.

**Warnings**
- Do not let compaction choose read-visible roots by directory residue.
- Do not move quarantined or unresolved damaged bytes as if they were stable.
- Do not let compaction reclaim old pages immediately after new root
  publication.
- Do not claim S.8 layout/index discipline; S.5 owns only stability under
  movement.
- Do not make compaction scan all active readers or all store pages to find
  protected overlap when candidate ranges and hazard leases can bound the
  decision.

**Test requirements**
- Adversarial convergence: a read admitted before compaction cutover reads the
  old protected structure while a read admitted after cutover reads the new
  structure, and both converge on valid physical bytes for their admitted plan.
- Adversarial denial: compaction over quarantined regions, stale source epoch,
  missing old-root preservation, backend residue candidate selection, and early
  page reuse deny at named interlock boundaries.
- Counter proof: compaction/read lanes expose exact protected pages, copied
  pages, publication swaps, blocked reclaims, and epoch retries.
- Interlock proof: Proof disjointness and canonical protected-set wrappers
  must survive from compaction planning into cutover, while Foundational
  diagnostic/performance rows materialize the executed interlock counters only
  after Store-owned compaction decisions close.
- Range-interlock proof: compaction candidate ranges intersect admitted
  protected-reference ranges through bounded range-set operations, not
  full-store scans or reader-by-reader folklore.
- Harness proof: read-during-compaction lanes reuse S.4.5 maintenance actors,
  production-facing drivers, deterministic yieldpoints, observers, transcripts,
  `NoMixedRootOracle`, `OldReaderSeesOldRootOracle`,
  `PostSwapReaderSeesNewRootOracle`, `BlockedReclaimUntilReleaseOracle`, and
  mutation rows for in-place overwrite, early reclaim, stale epoch reuse, and
  backend-residue candidate selection.

**Engineering decisions**
- Compaction is a maintenance actor constrained by read plans.
- Compaction candidates are represented as canonical physical range sets and
  lowered into cutover deltas; blocked old-structure retirement enters a typed
  deferred reclaim queue rather than a generic retry list.
- S.5 may block or defer compaction; S.6 later paces its I/O.
- Compaction cutover must be restart-stable and read-stable separately.

**Open questions**
- None.

### Phase 9: Interlock Reads With Checkpoint Publication

Phase 9 makes checkpoint publication visible to readers only through admitted
root and manifest epoch transitions.

**Relevant subsystems**
- `worth-store-physical-isolation`
- `worth-store-recovery-physics`
- `worth-store-certification`

**Relevant APIs**
- `CheckpointReadInterlockPlan`
- `CurrentPhysicalRoot`
- `CheckpointPublicationRoot`
- `RecoveryRoot`
- `ManifestLocatorRoot`
- `CheckpointPublicationStabilityProof`
- `CheckpointRootEpochTransition`
- `ReadDuringCheckpointVerdict`
- Proof readmission/freshness: `BoundaryBridged<CheckpointRootBasis>`,
  `AuthorityWitness<CheckpointReadmissionAuthority>`,
  `readmit_with(...)`, `FreshnessTransitionOutcome`, and
  checked checkpoint-root retry outcomes.
- Foundational canonical/readmission surfaces:
  `CurrentBasisBoundaryArtifact`, `BoundaryBridgedCurrentBasisBoundaryArtifact`,
  `readmit_current_basis_boundary_artifact_after_boundary(...)`, and canonical
  basis entries for checkpoint-root publication evidence.

**Warnings**
- Do not reopen S.4 checkpoint validity.
- Do not expose a checkpoint manifest as current before its physical
  publication epoch is admitted.
- Do not let a read plan mix old root pages with a new checkpoint frontier.
- Do not let checkpoint publication roots, recovery roots, or manifest locator
  roots satisfy APIs requiring `CurrentPhysicalRoot` without an explicit
  readmission transition.

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
- Readmission proof: checkpoint-root evidence crossing restart or cutover is
  boundary-bridged and must be explicitly readmitted before it can participate
  in current read-plan admission.
- Root separation proof: foreground reads, checkpoint publication, recovery,
  and manifest location each consume their own root authority type; tests deny
  mixed root/frontier plans even when the underlying locator bytes match.
- Harness proof: read-during-checkpoint and restart-during-publication lanes
  execute through S.4.5 crash/interleaving events, transcript replay, and
  `CrashRecoversOldOrNewNeverMixedOracle`; copied checkpoint reports and
  same-run self-comparison remain forbidden shortcut evidence.

**Engineering decisions**
- S.4 proves checkpoint recovery; S.5 proves online read stability during
  checkpoint publication.
- Checkpoint publication is an epoch transition for readers.
- `CurrentPhysicalRoot` is the foreground read authority. A
  `CheckpointPublicationRoot` can become current only through the S.5 admitted
  publication/readmission path; a `RecoveryRoot` remains S.4 recovery evidence
  until S.5 entry admission consumes it.
- PageLSN frontier remains physical replay metadata, not semantic visibility.

**Open questions**
- None.

### Phase 10: Enforce Reclaim Reachability And Hazard Barriers

Phase 10 defines when old pages, extents, and future chunks may be reclaimed
after publication or movement.

**Relevant subsystems**
- `worth-store-physical-isolation`
- `worth-store-buffer-pool`
- `worth-store-physical-integrity`
- `worth-store-certification`

**Relevant APIs**
- `ReachabilityBarrier`
- `HazardLeaseTable`
- `HazardLeaseSlot`
- `HazardLeaseGeneration`
- `HazardLeaseEpochIndex`
- `LeaseExpiryPosture`
- `ReadHandleRevocationReceipt`
- `OwnedCopyStableReadReceipt`
- `ProtectedReferenceLease`
- `ReclaimEligibilityProof`
- `CrashStableFreeReusePosture`
- `GenerationAdvanceReceipt`
- `AllocatorPublicationReceipt`
- `BlockedReclaimReport`
- `ReclaimDenial`
- Proof structural surfaces: `UniqueVec<ProtectedReferenceLease>`,
  `CanonicalVec<ReachabilityBarrier>`, `DisjointPair<ReadProtectedSet>`,
  `Proof<Disjointness, StructuralProofAuthority>`, and checked reclaim
  eligibility transitions.
- Foundational evidence: `FoundationalBoundaryEvidenceCompletedReceiptArtifact`,
  `FoundationalBoundaryEvidenceSupportCloseoutArtifact`,
  `FoundationalBoundaryEvidenceCloseoutDisposition`, and
  `FoundationalBoundaryEvidenceSupportResidualDebtSet` for blocked reclaim,
  completed release, and residual hazard debt.

**Warnings**
- Do not reclaim from backend residue or absence from current root alone.
- Do not drop old structures before every admitted reader, scrubber, verifier,
  checkpoint, and recovery barrier releases them.
- Do not let lease expiry silently free bytes without a typed expiry posture.
- Do not implement reclaim eligibility as a global scan of raw reader handles,
  reference-counted page objects, or unbounded hash maps.
- Do not treat time passing, task cancellation, or process disappearance as
  reclaim authority unless the read handle is proven unable to dereference
  protected bytes or the read has been converted to owned bytes.
- Do not reuse page, extent, segment, or future chunk identities until
  generation advancement and allocator publication are crash-stable.

**Test requirements**
- Adversarial equivalence: identical admitted read leases and publication
  receipts produce the same reclaim eligibility decision regardless of release
  order.
- Adversarial denial: active read leases, unreleased scrub windows, quarantine
  holds, checkpoint preservation, recovery verifier holds, and future chunk
  migration holds block reclaim with exact protected-reference counters.
- Leak proof: released plans eventually make reclaim eligible without
  accumulating unbounded hazard-table entries.
- Eligibility proof: reclaim consumes Proof unique/canonical/disjoint
  reachability wrappers and emits Foundational completed or blocked closeout
  evidence, but neither support closeout nor residual debt can authorize byte
  reclamation.
- Data-structure proof: hazard/lease records live in generation-counted slots
  or an equivalent arena/slab index so acquire, release, stale-release denial,
  leak detection, and reclaim lookup expose exact slot, generation, epoch, and
  protected-reference counters.
- Expiry proof: expired leases block reclaim unless accompanied by handle
  revocation, completed release, or owned-copy conversion proof.
- Generation-fence proof: free/reuse tests crash around reachability removal,
  generation advancement, and allocator publication and never admit ambiguous
  old/new generation authority.
- Harness proof: reclaim lanes run through the S.4.5 delayed-release,
  maintenance-actor, replay-bundle, and generated-coverage surfaces with
  `BlockedReclaimUntilReleaseOracle`, `CounterContractOracle`, and mutants for
  expiry-as-authority, stale lease release, raw reader-handle scans, and
  generation reuse before allocator publication.

**Engineering decisions**
- Reclaim consumes reachability proof; it does not infer it from storage shape.
- Hazard and lease tables are physical authority, not diagnostics, and their
  ordinary representation is a slab/arena of generation-counted lease slots
  indexed by epoch and protected-reference range.
- Reclaim eligibility is computed from canonical reachability barriers plus
  indexed hazard leases; a full-reader scan is a certification failure unless
  explicitly admitted for a bounded test-only lane.
- Lease expiry is a liveness signal, not an authority signal. Reclaim requires
  either release, revocation, or owned-copy conversion before freed bytes may be
  reused.
- Physical identity reuse is fenced by a crash-stable free/reuse posture that
  joins reachability removal, generation advancement, and allocator
  publication.
- Future chunk barriers exist as stability placeholders until S.7 owns blob
  lifecycle and retention.

**Open questions**
- None.

### Phase 11: Reserve Tier Movement And Blob Migration Stability Without Claiming S.7

Phase 11 gives S.5 enough typed stability vocabulary for tier movement and
future blob migration reads without implementing native blob lifecycle or S.6
I/O QoS.

**Relevant subsystems**
- `worth-store-physical-isolation`
- `worth-store-physical-format`
- `worth-store-certification`

**Relevant APIs**
- `TierMovementReadInterlockPlan`
- `ChunkMigrationReadInterlockPlan`
- `MovablePhysicalRefKind`
- `PhysicalChunkStabilityPlaceholder`
- `TierMovementStabilityVerdict`
- `FutureBlobMigrationNonClaim`
- Proof non-claim surfaces: `Recipe<Resolved, ChunkMigrationReadInterlockPlan>`,
  `AssumptionBasis<FutureChunkStabilityBasis>`,
  `CapabilityWitness<TierMovementStabilityCapability>`, and checked denial
  outcomes for unsupported S.7 lifecycle claims.
- Foundational boundary roles and diagnostics: `PlannedWorkRole`,
  `SupportOnlyRole`, `FoundationalBoundarySurfaceDisposition`,
  `FoundationalDiagnosticBreachClass`, and `FoundationalDiagnosticArtifactKind`
  for future blob/tier movement placeholders and non-claim reports.

**Warnings**
- Do not implement S.7 chunk trees or blob retention in S.5.
- Do not claim cold-tier performance, hardware QoS, or chunk dedupe behavior.
- Do not allow future chunk placeholders to become blob authority.
- Do not force chunk-specific placeholder fields through ordinary page, extent,
  or segment read APIs before S.7 introduces real chunk lifecycle semantics.

**Test requirements**
- Adversarial equivalence: stable chunk or extent placeholders preserve
  reference, generation, epoch, and reachability proof across independent
  tier-movement read-plan admissions.
- Adversarial denial: missing chunk epoch, stale extent generation, copied
  migration labels, unsupported tier movement, and blob-lifecycle claims deny
  before physical read stability is admitted.
- Non-claim proof: S.5 evidence explicitly reports that blob chunk lifecycle,
  dedupe, resumable writes, and blob retention remain S.7 scope.
- Scope proof: future chunk placeholders may carry Proof assumption bases for
  stability only, and Foundational planned/support surfaces must deny any
  promotion into blob authority, retention authority, or cold-tier QoS claims.
- Hot-path containment proof: ordinary page/extent/segment reads do not carry
  chunk-only fields; future chunks appear only behind generic movable physical
  reference kind surfaces or explicitly named S.7 placeholder APIs.

**Engineering decisions**
- S.5 may protect future chunk reads; it does not own blob semantics.
- S.5 represents future chunks through an extensible movable physical object
  shape, such as `MovablePhysicalRefKind::{Page, Extent, Segment, FutureChunk}`,
  so S.7 can attach blob lifecycle semantics later without widening ordinary
  page-read APIs now.
- Tier movement is treated as physical structure movement under read stability
  law.
- S.6 will later decide I/O pacing and media capability for tier movement.

**Open questions**
- None.

### Phase 12: Consume And Extend The S.4.5 Harness For S.5 Interleaving Families

Phase 12 converts the S.4.5 readiness shape probe into real S.5 physical
isolation certification families. It reuses the completed Roadmap 2 simulation
harness as infrastructure and adds only S.5-owned scenario vocabulary, suite
registration, oracle selection, mutation expectations, and closeout evidence.

**Relevant subsystems**
- `worth-store-certification`
- `worth-store-test-support`
- `worth-store-physical-certification`
- `worth-store-physical-isolation`
- `worth-store-readiness`
- `worth-proof`

**Relevant APIs**
- `S5SimulationHarnessReadiness`
- `S5HarnessReadinessReceipt`
- `S5SimulationHarnessReadinessDenial`
- `PhysicalScenarioBuilder`
- `PhysicalSimulationScenarioDefinition`
- `PhysicalSimulationScenarioFamily`
- `PhysicalSimulationScenarioFamily::S5ReadinessShapeProbe`
- new S.5 physical isolation scenario-family variants for stable read plans,
  compaction, checkpoint publication, reclaim, tier movement, future chunk
  stability, and restart during cutover
- `lower_physical_simulation_plan`
- `PhysicalSimulationPlan`
- `PhysicalInterleavingSchedule`
- `ReplaySeed`
- `StateSpaceBudget`
- `PartialOrderReductionPosture`
- production-facing driver contracts and deterministic yieldpoints from S.4.5
- maintenance actor contracts for foreground reads, compaction, checkpoint,
  reclaim, tier movement, crash/restart, and future blob movement placeholders
- `PhysicalSimulationObserver`
- `ObservedPhysicalTrace`
- `PhysicalProofOracle`
- `PhysicalProofOracleVerdict`
- `OracleVerdictBasis`
- reusable oracle families: `NoMixedRootOracle`,
  `OldReaderSeesOldRootOracle`, `PostSwapReaderSeesNewRootOracle`,
  `BlockedReclaimUntilReleaseOracle`,
  `CrashRecoversOldOrNewNeverMixedOracle`, `NoPrivateMutationOracle`,
  `NoJsonAuthorityOracle`, `CounterContractOracle`,
  `TranscriptReplayOracle`, and `IndependentVerifierAgreementOracle`
- counter contract surfaces for exact, zero, positive, bounded, and monotonic
  expectations
- `PhysicalSimulationTranscript`
- `SimulationReplayBundle`
- `PhysicalCertificationEvidenceBundle`
- generated coverage and mutation-coverage surfaces from S.4.5
- Proof harness surfaces: `recipe(...)`, `proof_flow()`,
  `CanonicalVec<PhysicalInterleavingStep>`,
  `NonEmpty<MaintenanceActorPlan>`, `Pair<ScenarioDriver, ScenarioObserver>`,
  `ProofOutcomeKind`, and `ReadyJoinSummary`.
- Foundational harness evidence: `FoundationalBoundaryEvidenceHarnessExpansionPoint`,
  `CanonicalFixtureManifestEvidence`, `FoundationalPerformanceHarnessExpansionPoint`,
  and canonical basis/digest entries for scenario definitions, schedules,
  transcripts, oracles, and mutation matrices.

**Warnings**
- Do not create a second S.5-only runner that bypasses the Roadmap 2 harness.
- Do not keep using `S5ReadinessShapeProbe` as the final S.5 proof family once
  the S.5-owned isolation family is available; the probe proves harness shape,
  not physical isolation correctness.
- Do not put oracle meaning in test support drivers.
- Do not make interleavings random without replayable schedules and transcript
  identity.
- Do not allow scenario definitions to omit drivers, observers, or expected
  counters.
- Do not rely on wall-clock race discovery, thread sleep timing, or same-run
  self-comparison as proof of interleaving safety.
- Do not convert scenario definitions, transcripts, evidence bundles, or
  coverage rows to JSON except through terminal projection or hostile
  readmission lanes.

**Test requirements**
- Adversarial equivalence: the same scenario definition and seed lower to the
  same interleaving schedule, actor plan, oracle set, expected counters, and
  transcript identity across independent harness runs.
- Adversarial denial: S.5 scenario definitions missing actor roles, production
  driver capability, latch order, epoch basis, observer trace, oracle family,
  counter expectations, forbidden-shortcut expectations, transcript identity,
  mutation row, or S.4.5 readiness receipt fail before plan admission.
- Mutation proof: required S.5 mutants for early reclaim, stale epoch reuse,
  latch inversion, in-place compaction overwrite, and mixed-root read all fail
  their intended suite lanes.
- Scaling proof: adding a new interleaving family requires registering a lane,
  drivers, observers, oracles, transcript fields, and mutation expectations in
  one coherent harness topology.
- Harness adoption proof: S.5 lanes consume `S5SimulationHarnessReadiness` and
  reuse `PhysicalScenarioBuilder`, scenario lowering, schedules, production
  drivers, observers, reusable oracles, transcript replay, evidence bundles,
  generated coverage, mutation evidence, and forbidden-shortcut rejection
  rather than rebuilding those surfaces locally.
- Probe graduation proof: the old S.4.5
  `PhysicalSimulationScenarioFamily::S5ReadinessShapeProbe` remains as a
  non-claim regression lane, while real S.5 closeout uses S.5-owned scenario
  families with actual physical isolation assertions.
- Oracle ownership proof: test-support drivers can emit observations and
  counters, but only certification-owned `PhysicalProofOracle` families can
  produce `PhysicalProofOracleVerdict` values for S.5 suite closure.
- Scheduler proof: hostile interleavings are generated by a deterministic
  scheduler with replayable seeds, actor steps, partial-order reduction or an
  equivalent bounded exploration rule, state-space budget counters, and
  transcript identity.
- Native evidence proof: scenario definitions, counter contracts, transcripts,
  replay bundles, generated coverage, and certification evidence remain
  aspect-native. JSON-shaped scenario authority, logs, summaries, fixture
  labels, and copied evidence rows deny through the existing S.4.5 shortcut
  rejection surfaces.

**Engineering decisions**
- S.5 does not build a harness; it is the first heavy consumer of the completed
  S.4.5 harness.
- S.5 extends scenario-family vocabulary and oracle registration only where
  physical isolation introduces new proof meaning.
- The harness uses deterministic interleaving schedules and bounded
  state-space exploration; random stress may supplement it but cannot satisfy
  any S.5 proof obligation.
- Test support supplies mechanics; certification owns proof meaning.
- Every suite must keep positive, hostile, forbidden-shortcut, reopen/retry, and
  mutant lanes explicit.

**Open questions**
- None.

### Phase 13: Materialize Foundational And Proof Evidence From Executed Isolation

Phase 13 exports S.5 evidence through Foundational and Proof vocabulary without
letting those exports become Store physical stability authority.

**Relevant subsystems**
- `worth-store-physical-isolation`
- `worth-foundational`
- `worth-proof`
- `worth-store-certification`

**Relevant APIs**
- `PhysicalIsolationEvidenceBundle`
- `PhysicalIsolationBoundaryRoleClaim`
- `PhysicalIsolationCounterBackedPerformanceReceipt`
- `StableReadPlanProofArtifact`
- `PhysicalIsolationProofProgression`
- `ProjectionCannotMintPhysicalStabilityDenial`
- Foundational aspects/evidence/performance:
  `AuthoritativeRecordAspectState`, `CanonicalBasisBundle`,
  `CanonicalDerivedDigest`, `FoundationalBoundaryArtifactRole`,
  `FoundationalBoundaryEvidenceExecutedReceiptArtifact`,
  `FoundationalBoundaryEvidenceCompletedReceiptArtifact`,
  `FoundationalDiagnosticRow`, `FoundationalCounterBackedPerformanceReceipt`,
  and `FoundationalPerformanceBundle`.
- Proof progression/evidence: `Artifact<P, T, S, A>`,
  `Proof<CanonicalOrder, StructuralProofAuthority>`,
  `Proof<Uniqueness, StructuralProofAuthority>`,
  `Proof<Disjointness, StructuralProofAuthority>`,
  `PhysicalIsolationProofProgression`, checked transitions, runtime
  readmission, and ready-join summaries for executed S.5 evidence.

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
- Authority-denial proof: Foundational authoritative-current, derived
  projection, support-only, planned-work, receipt-evidence, diagnostics, and
  performance surfaces are exhaustively tested against Store APIs that require
  `StablePhysicalReadPlan`, `LatchOrderProof`, `PhysicalEpochBasis`, or
  `ReclaimEligibilityProof`.

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
- `worth-store-physical-isolation`
- `worth-store-physical-backend`
- `worth-store-certification`
- `worth-store-readiness`

**Relevant APIs**
- `S6IoQosIsolationReadiness`
- `PhysicalIsolationCloseoutReport`
- `PhysicalIsolationCounterSnapshot`
- `ForegroundInterferenceSurface`
- `BackgroundMaintenanceIsolationAssumption`
- `UnsupportedQoSClaim`
- Foundational handoff surfaces: `FoundationalPerformanceCounterSpec`,
  `FoundationalCounterBackedPerformanceReceipt`,
  `FoundationalPerformanceReportPlan`,
  `FoundationalBoundaryEvidenceSupportBasisDisclosure`, and
  `FoundationalBoundaryEvidenceResidualDebt` for wait, retry, blocked
  maintenance, and unsupported-QoS disclosure.
- Proof handoff surfaces: `BoundaryBridged<S6IoQosIsolationReadinessBasis>`,
  `AssumptionBasis<S5PhysicalIsolationCloseoutBasis>`,
  `AuthorityWitness<S6ReadinessPublicationAuthority>`, and checked
  readmission/rebind outcomes for S6 consumers.

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
- Handoff proof: S6 readiness is a typed Proof/Foundational handoff with
  explicit assumption basis, performance counters, support-basis disclosure,
  residual debt, and unsupported-claim denials; logs and terminal projections
  cannot reconstruct it.

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
- `worth-store-physical-isolation`
- `worth-store-certification`
- `worth-store-test-support`
- `worth-store-recovery-physics`
- `worth-store-physical-integrity`
- `worth-foundational`
- `worth-proof`

**Relevant APIs**
- `PhysicalIsolationCloseoutSuite`
- `PhysicalIsolationCertificationBundle`
- `PhysicalIsolationCloseoutReport`
- `SyntheticPhysicalIsolationShortcutRejectionReport`
- `S6IoQosIsolationReadiness`
- Foundational closeout surfaces: `FoundationalBoundaryEvidenceAttachmentBundle`,
  `FoundationalBoundaryArtifactCertifiedSurface`,
  `FoundationalPerformanceCertifiedSurface`,
  `FoundationalDiagnosticCertifiedSurface`,
  canonical basis/digest bundles, and boundary-evidence readmission surfaces
  for closeout artifacts that cross process or trust boundaries.
- Proof closeout surfaces: `Artifact<PhysicalIsolationClosed, _, _, _>`,
  proof sets for canonical order, uniqueness, and disjointness,
  `ExecutedRecipe<PhysicalIsolationCloseoutReport>`, `join_ready(...)`, and
  checked runtime readmission for resumed certification evidence.

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
- Algorithm/data-structure proof: final suites prove generation-counted
  references, canonical latch ordering, compact protected-reference range
  sets, RCU/COW publication, indexed hazard lease slots, deferred reclaim
  queues, and deterministic interleaving schedules are used where the phase
  plan requires them.
- Protocol-order proof: final suites prove protect-before-observe ordering,
  traversal admission, byte guard usage, no hidden latch/I/O, root-kind
  separation, lease-expiry non-authority, free/reuse generation fences, and
  publication memory ordering are encoded as phase/proof types rather than
  comments or runtime folklore.
- Harness proof: every S.5 suite consumes S.4.5
  `S5SimulationHarnessReadiness`, starts from the public scenario builder,
  lowers to `PhysicalSimulationPlan`, executes a replayable
  `PhysicalInterleavingSchedule`, emits `PhysicalSimulationTranscript`,
  packages `SimulationReplayBundle` and `PhysicalCertificationEvidenceBundle`,
  contributes generated coverage rows, and maps to positive, hostile,
  forbidden-shortcut, retry/reopen, and mutation-validation lanes.
- Shadow-runner denial: any suite that tries to close on a local S.5 runner,
  fixture label, log summary, private mutation, same-run self-comparison,
  copied readiness row, JSON authority, or test-support-owned verdict fails
  certification even if the visible read result is correct.
- Handoff proof: S.6 receives typed physical-stability assumptions,
  foreground-interference surfaces, wait/retry counters, blocked-maintenance
  counters, and explicit unsupported-QoS claims.
- Foundational/Proof closeout proof: final certification bundles include
  Foundational canonical, diagnostic, evidence, and performance surfaces plus
  Proof progression/readmission traces for every closed S.5 lane, while every
  Store authority API rejects those surfaces as substitutes for Store-owned
  stable read plans, latch proofs, epoch scopes, and reclaim proofs.
- Line-cap and composition proof: production, test, and support files stay
  within workspace line-cap rules and keep latch, epoch, read-plan,
  publication, reclaim, harness, evidence, and handoff responsibilities
  separate.

**Engineering decisions**
- S.5 closeout proves online physical read stability only.
- S.5 is architecture-performance critical: the ordinary foreground read path
  must consume pre-lowered plans, compact proof-bearing data structures,
  bounded scratch allocation, and indexed lease/reclaim structures rather than
  generic runtime rediscovery.
- Arch Law 41 applies directly: S.5 phase outputs must be sealed
  proof-carrying types, and out-of-order movement such as observing before
  protection, executing before traversal admission, reading bytes without a
  guard, or reclaiming from expiry alone must be uncallable.
- S.5 explicitly reserves hardware QoS, native blob lifecycle, layout/index
  strategy, repair, security, and full database certification for later
  sequences.
- The closeout must be strong enough for S.6 to begin foreground/background I/O
  pacing without reopening physical isolation.
- The closeout must also be strong enough for later S.6-S.12 suites to extend
  the S.4.5 harness shape by adding new scenario families, not by forking new
  proof machinery.

**Open questions**
- None.

## Must Ship

- typed consumption of `S5PhysicalIsolationRecoveryReadiness`
- physical/semantic isolation separation with explicit cross-layer correlation
- distinct root authority types for current physical roots, checkpoint
  publication roots, recovery roots, and manifest-locator roots
- root, manifest, segment, extent, page, and future chunk epoch vocabulary
- generation-counted physical references and scoped epoch vectors that prevent
  ABA/stale-reuse authority bugs
- protect-before-observe admission chain with hazard/lease/reader-epoch
  publication before root reliance and post-protection validation
- declared latch classes, modes, acquisition order, wait counters, and deadlock
  prevention or detection policy
- canonical latch acquisition over physical latch keys, with bounded wait-for
  graphs only for families that explicitly choose detection
- stable physical read-plan admission carrying root epoch, manifest epoch,
  physical references, generation proofs, latch requirements, reachability
  barriers, footprint counters, retry posture, and release semantics
- traversal admission protocol for direct seed plans and stepwise read cursors,
  with temporary guards and retry/denial receipts
- compact protected-reference range/set representations plus pre-sized or
  arena-scoped read-plan admission scratch storage
- execution-ready read handles that cannot re-decide root, latch, epoch,
  footprint, or reachability strategy
- physical byte guards for frame pins, mmap views, extent windows, or owned read
  buffers, separate from reachability leases
- no-hidden-latch-I/O enforcement and S.6-visible wait/interference evidence
  for any declared exception
- copy-on-write or equivalent physical publication for moved or rewritten
  reachable structures
- RCU/COW-style root and manifest publication with old-root preservation and
  release-gated reclaim
- declared acquire/release or stronger memory-ordering contracts for root
  swaps, hazard publication, reader epochs, generation advancement, allocator
  publication, and validation
- read-during-compaction, read-during-checkpoint, read-during-reclaim,
  read-during-tier-movement, and read-during-future-blob-migration interlocks
- canonical compaction candidate range sets, cutover deltas, and deferred
  reclaim queues
- restart-during-cutover behavior that exposes either the old stable root or
  the new stable root, never a mixed tree
- reachability, hazard, lease, protected-reference, and reclaim eligibility
  barriers for pages, extents, and future chunks
- generation-counted hazard lease slots indexed by epoch and protected
  reference range
- lease-expiry posture where expiry blocks reclaim unless paired with release,
  revocation, or owned-copy conversion proof
- crash-stable free/reuse generation fences joining reachability removal,
  generation advancement, and allocator publication
- movable physical reference kind abstraction that keeps future chunk
  placeholders out of ordinary page/extent/segment hot paths
- quarantine interlock consuming S.3 damage locality so damaged bytes cannot be
  stabilized by movement
- exact latch, wait, epoch-retry, stale-plan-denial, protected-reference,
  blocked-reclaim, copied-page, publication-swap, and read-plan footprint
  counters
- typed consumption of S.4.5 `S5SimulationHarnessReadiness` and
  `S5HarnessReadinessReceipt` before any S.5 certification lane can close
- S.5 physical isolation scenario-family extensions registered through the
  S.4.5 Roadmap 2 harness, including maintenance actor plans, deterministic
  schedules, production-facing drivers, observers, certification-owned oracles,
  transcripts, generated coverage, and mutation validation
- S.4.5 deterministic scheduler reuse with replayable seeds, bounded
  state-space exploration, partial-order-reduction posture, and transcript
  identity
- Foundational and Proof-compatible evidence from executed Store isolation
  findings, plus projection-authority denials
- concrete `S6IoQosIsolationReadiness` handoff payload

## Must Preserve

- Store owns physical byte stability.
- `worth-relational` owns semantic MVCC, transaction visibility, branch truth,
  and snapshot meaning.
- `worth-store-recovery-physics` owns S.4 recovery correctness; S.5 consumes
  recovered roots and stability assumptions.
- `worth-store-physical-integrity` owns S.3 damage detection and quarantine
  locality; S.5 consumes that posture and does not repair damage.
- `worth-foundational` owns shared evidence and boundary vocabulary, not Store
  physical stability authority.
- `worth-proof` owns progression law, not latch, epoch, read-plan, or reclaim
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
- `S5SimulationHarnessReadiness`
- `S5HarnessReadinessReceipt`
- `s5_physical_isolation_simulation_replay_bundle`
- `s5_physical_isolation_certification_evidence_bundle`
- `s5_physical_isolation_generated_coverage_matrix`
- `physical_interleaving_schedule`
- `deterministic_interleaving_scheduler_trace`
- `maintenance_actor_plan`
- `stable_read_plan_trace`
- `protect_before_observe_trace`
- `root_kind_separation_trace`
- `traversal_admission_trace`
- `generation_counted_reference_trace`
- `physical_byte_guard_trace`
- `protected_reference_range_set_trace`
- `latch_order_trace`
- `latch_wait_for_graph_trace`
- `latch_wait_counter_trace`
- `epoch_comparison_trace`
- `epoch_retry_trace`
- `copy_on_write_publication_trace`
- `rcu_root_publication_trace`
- `publication_memory_ordering_trace`
- `read_during_compaction_trace`
- `compaction_candidate_range_set_trace`
- `read_during_checkpoint_trace`
- `read_during_reclaim_trace`
- `hazard_lease_slot_trace`
- `lease_expiry_posture_trace`
- `free_reuse_generation_fence_trace`
- `deferred_reclaim_queue_trace`
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

- `s45_harness_consumption_suite`
  proves every S.5 hostile lane consumes the completed S.4.5 harness readiness,
  public scenario authoring API, lowering, deterministic schedule, production
  driver contracts, observers, certification-owned oracles, replay bundle,
  evidence bundle, generated coverage, mutation evidence, and forbidden
  shortcut rejection.
- `s5_entry_authority_suite`
  proves S.5 consumes typed S.4 readiness and rejects copied recovery fields,
  live runtime state, terminal projections, and semantic snapshots.
- `physical_semantic_isolation_separation_suite`
  proves semantic MVCC tokens correlate with but cannot mint Store physical
  stability.
- `epoch_scope_and_stale_plan_suite`
  proves root, manifest, segment, extent, page, and future chunk epochs are
  scoped and stale-plan denials occur before byte reads.
- `generation_counted_reference_suite`
  proves reused page, extent, segment, and future chunk identifiers cannot
  satisfy S.5 authority when their generation evidence is stale or mismatched.
- `latch_order_deadlock_suite`
  proves canonical latch ordering, wait accounting, and deadlock prevention or
  detection under hostile schedules.
- `latch_algorithm_shape_suite`
  proves canonical physical latch keys produce deterministic order across input
  order, hash seed, platform, and restart, and that any selected deadlock
  detection lane uses a bounded wait-for graph.
- `stable_read_plan_admission_suite`
  proves plans carry protected references, latch requirements, epoch basis,
  footprint counters, reachability barriers, retry posture, and release
  semantics.
- `protect_before_observe_suite`
  proves root observation and reliance are uncallable before reader hazard,
  lease, or epoch publication plus post-protection validation.
- `root_kind_separation_suite`
  proves current, checkpoint-publication, recovery, and manifest-locator roots
  cannot satisfy each other's authority APIs without explicit readmission.
- `traversal_admission_suite`
  proves footprint-discovery traversal happens in admission through temporary
  guards and lowers only into seed plans, stepwise cursors, or typed
  retry/denial receipts.
- `protected_footprint_data_structure_suite`
  proves read-plan footprints use compact canonical range/set representations
  and arena-scoped scratch storage instead of unbounded raw vectors or
  per-reference foreground allocation.
- `stable_read_execution_non_redecision_suite`
  proves execution consumes admitted plans and cannot re-plan isolation
  strategy after seeing moved or changed bytes.
- `physical_byte_guard_suite`
  proves execution cannot dereference bytes unless a frame pin, mmap view
  guard, extent-window guard, or owned read-buffer guard covers the read
  receipt or denial.
- `no_hidden_latch_io_suite`
  proves ordinary read execution does not hold high-level structural latches
  across blocking storage I/O unless the declared plan and S.6 handoff expose
  the wait/interference surface.
- `copy_on_write_publication_suite`
  proves moved or rewritten reachable structures publish through old-root
  preservation and stable epoch transitions.
- `rcu_publication_and_old_root_preservation_suite`
  proves root and manifest publication behave as RCU/COW-style swaps where
  pre-swap readers retain old reachability and post-swap readers observe the
  new epoch.
- `publication_memory_ordering_suite`
  proves root swaps, hazard publication, reader epochs, generation advancement,
  allocator publication, and validation use declared acquire/release or
  stronger ordering.
- `read_during_compaction_suite`
  proves readers admitted before and after compaction cutover observe stable
  physical structures for their admitted plan.
- `compaction_range_interlock_suite`
  proves compaction candidate range sets intersect protected-reference range
  sets through bounded operations and never through full-store or all-reader
  scans.
- `read_during_checkpoint_suite`
  proves checkpoint publication does not expose mixed old/new root or pageLSN
  frontier state to admitted readers.
- `reclaim_reachability_suite`
  proves active reads, scrub windows, recovery verifiers, checkpoints,
  quarantine holds, and future chunk holds block reclaim until release.
- `hazard_lease_data_structure_suite`
  proves hazard leases use generation-counted arena/slab slots indexed by epoch
  and protected-reference range, with exact acquire, release, stale-release,
  leak, and reclaim lookup counters.
- `lease_expiry_non_authority_suite`
  proves expired leases cannot authorize reclaim without completed release,
  handle revocation, or owned-copy conversion proof.
- `free_reuse_generation_fence_suite`
  proves reuse of page, extent, segment, or future chunk identities is blocked
  until reachability removal, reclaim eligibility, generation advancement, and
  allocator publication are crash-stably admitted.
- `tier_and_future_blob_migration_stability_suite`
  proves S.5 protects physical movement placeholders without claiming S.7 blob
  lifecycle or S.6 QoS.
- `interleaving_harness_scaling_suite`
  proves S.5 scenarios lower into deterministic schedules with actors,
  observers, oracles, transcripts, counters, and mutation expectations.
- `deterministic_scheduler_suite`
  proves hostile interleavings are replayable from scenario identity, seed,
  actor steps, state-space budget, and partial-order-reduction or equivalent
  bounded exploration metadata.
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
- protect-before-observe admission ordering
- root kind separation
- latch ordering and deadlock policy
- stable read-plan admission
- traversal admission for discovered footprints
- execution non-redecision
- physical byte guards separate from reachability leases
- no hidden structural latch holds across blocking I/O
- copy-on-write or equivalent publication for reachable structure rewrites
- publication memory ordering contracts
- read-during-compaction safety
- read-during-checkpoint safety
- read-during-reclaim safety
- reachability and hazard barriers
- lease expiry non-authority
- crash-stable free/reuse generation fences
- quarantine movement/reclaim interlock
- restart-during-cutover stability
- exact latch, epoch, read-plan, publication, and reclaim counters
- generation-counted physical references
- canonical latch ordering over physical latch keys
- compact protected-reference range/set representations
- bounded read-plan scratch allocation
- RCU/COW-style publication and old-root preservation
- indexed hazard lease slots and deferred reclaim queues
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
