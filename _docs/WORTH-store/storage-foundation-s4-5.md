# Storage Foundation S.4.5 Engineering Spec: Physical Database Simulation Harness

> **Status:** Planned
>
> **Roadmap parent:** [worth_store_roadmap_2.md](/Users/Esther/Documents/Programming/WORTH_workspace/WORTH/_docs/worth-store/worth_store_roadmap_2.md)
>
> **Primary prerequisite:** `S.4 WAL, Checkpoint, LSN, And Recovery Physics`
>
> **Follow-on storage-foundation sequence:** `S.5 Physical Isolation, Latches, Epochs, And Stable Read Plans`
>
> **Primary architectural driver:** make Roadmap 2 physical database testing
> scale through a reusable, aspect-native, deterministic, production-boundary
> simulation substrate before S.5 begins hostile maintenance interleaving work.

## Goal

Build the physical database simulation harness that S.5 through S.12 can reuse
instead of rebuilding bespoke hostile tests inside each milestone.

S.4.5 turns the broad Roadmap 2 harness doctrine into typed Store architecture:
scenario definitions, deterministic schedules, actors, production-facing
drivers, fault and corruption events, observers, certification-owned oracles,
counter expectations, transcripts, evidence bundles, coverage matrices, and
simulation profiles. It is complete when S.5 can consume
`S5SimulationHarnessReadiness` and express physical isolation attacks through
the shared harness without depending on logs, timing luck, private mutation,
same-run self-comparison, JSON-shaped authority, or test-support-owned proof
meaning.

## Why This Sequence Exists

S.4 built recovery physics and already introduced useful crash/fault harness
pieces. S.5 is the first milestone where hostile concurrency, maintenance
actors, root publication, byte guards, reclaim, restart during cutover, and
deterministic interleavings become central rather than supporting tests.

If S.5 builds that harness locally, later S.6 through S.12 will either copy it,
weaken it, or bolt on simulation after the physical protocols have already
ossified. S.4.5 exists to make adversarial simulation a Store substrate before
the next physical protocol depends on it.

## Governing Summaries

- `MENTALITY.md`
  protects hard-problem-first design. S.4.5 builds the adversarial test
  substrate before S.5, because physical isolation cannot be certified by
  ad hoc interleaving tests added after implementation.
- `arch_laws.md`
  protects proof-bearing progression, phase-typed execution, and authority
  separation. S.4.5 must make unresolved scenarios, resolved plans, lowered
  schedules, admitted executions, executed transcripts, oracle verdicts, and
  certified evidence mechanically distinct.
- `composition_laws.md`
  protects named semantic steps and test-code architecture. S.4.5 must not
  collapse scenario authoring, fixture construction, scheduling, driver
  behavior, observation, oracle judgment, and evidence materialization into
  one mega harness file.
- `domain_structure_laws.md`
  protects tree topology by responsibility. Harness mechanics, production
  drivers, fixtures, scenario definitions, oracle verdicts, transcripts,
  evidence, and certification closeout fail differently and need separate
  homes.
- `perf_laws.md`
  protects visible cost. The harness must record exact counters and enforce
  resource envelopes during execution, not infer cost from elapsed time or
  final success.
- `worth_store_roadmap_2.md`
  places S.4.5 between recovery and isolation so Roadmap 2's physical harness
  requirements become concrete before stable-read interleavings need them.
- `storage-foundation-s3.md`
  supplies corruption and quarantine semantics that simulation fixtures must
  preserve without moving corruption proof meaning into test support.
- `storage-foundation-s4.md`
  supplies crash, storage-interposer, fresh-runtime, recovery-determinism, and
  S.5 handoff patterns that S.4.5 generalizes rather than discards.
- `storage-foundation-s5.md`
  supplies the first heavy consumer: deterministic interleavings for
  protect-before-observe, root publication, byte guards, reclaim, latches, and
  restart during cutover.
- `test-requirements-2.md`
  defines the full Roadmap 2 adversarial harness doctrine. S.4.5 turns the
  immediately required substrate into implementable Store modules and handoff
  evidence.
- `FOUNDATIONAL_README.md`
  protects the rule that Store keeps stronger physical authority while lowering
  only boundary, canonical, diagnostic, provenance, profile, and performance
  evidence into Foundational shared vocabulary.
- `worth-proof README` and `worth_proof_roadmap.md`
  protect proof-bearing progression law. S.4.5 should use Proof for staged
  scenario progression, freshness, witnesses, outcomes, and structural
  collections, not as a runtime simulation engine.

## Adversarial Constraint

S.4.5 must survive this hostile testing condition:

> A later Store milestone claims physical database behavior under crashes,
> byte corruption, stale generations, checkpointing, compaction, reclaim,
> maintenance interference, I/O pressure, future blob movement, backup,
> repair, tenant pressure, and restart loops. The claim must be tested through
> typed aspect-native scenarios, deterministic schedules, production-boundary
> drivers, exact counters, replayable transcripts, independent observers, and
> certification-owned oracles. It must not be closable by logs, elapsed time,
> same-run self-comparison, private state mutation, fixture labels, synthetic
> in-memory stores, JSON scenario authority, or test-support-owned verdicts.

If a suite can pass because the harness says "the expected thing happened"
without an executed production-boundary trace and certification-owned oracle
verdict, S.4.5 is not closed.

## Product Decision Lock

- S.4.5 owns reusable simulation and certification mechanics, not physical
  isolation, recovery, compaction, I/O QoS, blob lifecycle, repair, security,
  or final S.12 database certification.
- Store production crates own physical behavior. Harness crates may drive,
  distort, observe, and replay behavior; they may not define the product law.
- `worth-store-test-support` owns reusable mechanics and fixtures.
- `worth-store-certification` owns oracle meaning, verdicts, coverage matrices,
  closeout reports, and certification evidence.
- `worth-store-physical-certification` owns reusable physical certification
  vocabulary and evidence bundle types that are not milestone-specific.
- Foundational surfaces describe exported evidence and canonical identity; they
  do not satisfy Store physical authority APIs.
- Proof surfaces encode stage progression and checked outcomes; they do not
  own simulation runtime semantics.
- Ordinary scenarios, fixtures, transcripts, and evidence are aspect-native.
  JSON may appear only in explicit terminal projection or hostile/readmission
  tests.
- S.4.5 is a test operating system for physical database claims, not a
  certification bureaucracy. The strict proof machinery must sit underneath an
  ergonomic authoring surface that engineers can use repeatedly without
  hand-building internal evidence types.

## Simulation Harness Laws

- Golden Path Authoring Law: every certification lane must be expressible
  through a small ergonomic scenario-builder API that lowers into the full
  proof-bearing pipeline. If ordinary scenario authoring requires direct
  construction of internal Proof, Foundational, evidence, or transcript types,
  the harness is not closed.
- Yieldpoint Law: deterministic schedules may control interleavings only at
  named production-boundary yieldpoints. A schedule that cannot prove which
  production seam it paused, faulted, resumed, or observed is not certification
  evidence.
- Counter Strength Law: counters must use the weakest expectation strong
  enough to prove the claim. Exact counters are mandatory for forbidden
  behavior, deterministic event structure, fault delivery, actor steps, and
  transcript fields when exactness is the claim. Performance-sensitive
  implementation counters should use zero, positive, bounded, monotonic, or
  profile-scoped expectations unless exactness itself is being certified.
- Oracle Library Law: S.4.5 must ship reusable oracle families for S.5
  readiness shape probes, not merely an oracle trait. Future suites should
  compose existing oracle families wherever the proof meaning is shared.
- Harness Dogfood And Readiness Probe Law: S.4.5 cannot close until at least
  one S.4 recovery scenario and one shortcut-rejection scenario dogfood the
  public authoring API, and until one S.5 readiness shape probe runs through
  the same scenario-to-evidence pipeline. The S.5 probe proves harness
  expressiveness only; it must carry an explicit non-claim posture for S.5
  physical isolation correctness because S.5 is not implemented yet.
- Extension Slot Law: future S.6 through S.12 actors, faults, oracles, and
  profiles may be declared as typed extension slots, but S.4.5 must not
  implement future domain behavior before a consuming milestone exists.
- Generated Coverage Law: coverage maturity rows must be generated from
  registered scenarios, drivers, actors, yieldpoints, counters, oracle
  families, transcripts, and mutation results. Hand-authored coverage prose
  cannot satisfy coverage.
- Test Scope Separation Law: ordinary unit tests and simple integration tests
  remain valid where they prove local pure logic or simple production paths.
  S.4.5 owns certification scenarios for physical database claims under
  hostile conditions; it must not become mandatory ceremony for every test.

## Planned Directory Skeleton

`workspaces/worth-store/crates/worth-store-physical-certification/src/`

- `lib.rs`
  exposes the physical certification facade and re-exports only proof-bearing
  boundary types.
- `authoring/`
  owns the public golden-path scenario-builder API that ordinary test authors
  use before the harness lowers into internal Proof, Foundational, canonical,
  transcript, and evidence machinery.
- `scenario/`
  owns aspect-native scenario definitions, canonical scenario identity,
  scenario families, and scenario admission denials.
- `planning/`
  resolves scenarios into required capabilities, actors, drivers, observers,
  oracle families, counter contracts, and evidence policy.
- `schedule/`
  owns deterministic schedules, actor-step ordering, seeds, state-space
  budgets, partial-order-reduction posture, and replay identity.
- `execution/`
  owns execution admission, executed simulation receipts, runtime isolation
  posture, and transcript production.
- `actors/`
  owns actor contracts for foreground operations, recovery, checkpoint,
  compaction, reclaim, scrub, offline verification, future blob movement,
  backup, repair, security, and tenant pressure.
- `faults/`
  owns fault and interleaving event vocabulary, including crash, torn write,
  dropped flush, byte corruption, stale generation, delayed release, I/O stall,
  and wrong-authority attempts.
- `drivers/`
  owns production-boundary driver traits, capability profiles, and named
  deterministic yieldpoint declarations.
- `observation/`
  owns observer registration, observed traces, independent verifier linkage,
  and forbidden-observation denials.
- `oracles/`
  owns reusable certification oracle families, oracle contracts, and verdict
  shapes.
- `counters/`
  owns exact counter specifications, zero/positive/exact/monotonic/bounded
  expectation forms, and mismatch evidence.
- `transcript/`
  owns replayable story transcripts, canonical transcript basis, and shrink
  metadata.
- `fixtures/`
  owns fixture capability declarations and production-backed fixture receipts,
  not private-state mutation.
- `profiles/`
  owns smoke, CI certification, soak, release certification, hardware
  qualification, crash matrix, corruption matrix, and interleaving profiles.
- `coverage/`
  owns generated Roadmap 2 coverage matrix rows and maturity ladder evidence
  derived from registered scenarios, oracles, counters, transcripts, and
  mutation results.
- `evidence/`
  owns Foundational boundary materialization, canonical basis, diagnostics,
  provenance, receipts, profile surfaces, performance receipts, and projection
  authority denials.
- `proof_progression/`
  owns Store-specific aliases over Proof recipes, witnesses, outcomes,
  assumptions, and structural collections.
- `s5_readiness.rs`
  publishes `S5SimulationHarnessReadiness`.

`workspaces/worth-store/crates/worth-store-test-support/src/`

- `physical_simulation/`
  exposes reusable mechanics only: deterministic scheduler adapters,
  production-backed fixture builders, adversarial storage wrappers, crash
  runtime isolation helpers, corruption injectors, memory/I/O pressure
  drivers, actor runners, transcript replay helpers, and shrink helpers.
- `s4_recovery_physics/`
  remains the existing S.4-specific harness support and is either consumed by
  the generic physical simulation substrate or wrapped through compatibility
  adapters without becoming the generic authority.
- `native_aspect_fixtures.rs`
  remains the ordinary native fixture authoring entry point; S.4.5 expands it
  for physical scenario inputs rather than routing scenario meaning through
  JSON.

`workspaces/worth-store/crates/worth-store-certification/tests/`

- `s4_5_simulation_harness_*.rs`
  owns runtime and certification tests for each S.4.5 phase.
- `ui/s4_5_simulation_harness/`
  owns compile-fail tests for WORTHd scenarios, skipped progression, JSON
  authority, private mutation, test-support oracle meaning, and projection
  authority substitution.
- later `s5_*`, `s6_*`, and `s7_*` suites consume the S.4.5 substrate instead
  of introducing new local harness skeletons.

## Existing Test Architecture To Preserve And Integrate

- The older `crates/worth-store/src/tests/harness` tree contains semantic
  harness modules for certification, corruption, fixtures, and scenarios. S.4.5
  treats this as historical/semantic context, not as the new physical authority
  for Roadmap 2.
- The dedicated `workspaces/worth-store/crates/worth-store-test-support` crate
  already owns native fixtures, memory pressure, allocation sentinels, resident
  pressure, terminal/hostile JSON fixture quarantines, and S.4 recovery
  mechanics.
- The dedicated `workspaces/worth-store/crates/worth-store-certification`
  tests already include aspect-native compile-fail lanes, authority/projection
  readmission lanes, terminal projection quarantine, S.4 crash/recovery suites,
  S.4 Foundational/Proof evidence suites, and S.4 synthetic shortcut denials.
- S.4.5 must extend the dedicated workspace architecture and migrate reusable
  S.4 mechanics upward only when the shared responsibility is real.

## Phase Plan

### Phase 1: Admit S.4 Closeout And Freeze The Harness Boundary

Phase 1 freezes the entry boundary. S.4.5 consumes S.4 recovery closeout,
S.4-specific harness lessons, Roadmap 2 harness requirements, and S.5 consumer
needs without reopening recovery physics or implementing isolation.

**Relevant subsystems**
- `worth-store-recovery-physics`
- `worth-store-physical-certification`
- `worth-store-test-support`
- `worth-store-certification`
- `worth-store-readiness`

**Relevant APIs**
- `S4RecoveryPhysicsCloseout`
- `S5PhysicalIsolationRecoveryReadiness`
- `S45SimulationHarnessEntry`
- `S45HarnessBoundaryDenial`
- `S45RoadmapHarnessRequirementSet`
- `S45ExistingHarnessInventory`
- Foundational evidence: `FoundationalBoundaryEvidenceExecutedReceiptArtifact`,
  `FoundationalBoundaryEvidenceSourceBasis`,
  `FoundationalBoundaryEvidenceFreshnessPosture`, and
  `FoundationalBoundaryEvidenceRuntimeAssumption`
- Proof progression: `Recipe<Unresolved, S45HarnessEntryRequest>`,
  `Recipe<Resolved, S45HarnessEntryBasis>`,
  `AuthorityWitness<S45HarnessEntryAuthority>`,
  `AssumptionBasis<S4CloseoutBasis>`, and checked entry outcomes

**Warnings**
- Do not reopen WAL replay, checkpoint source precedence, pageLSN, or durable
  acknowledgment law.
- Do not treat S.4's crash harness as the final generic Roadmap 2 harness.
- Do not let S.5 isolation requirements leak into S.4.5 as implementation of
  latches, epochs, byte guards, or reclaim.
- Do not accept old semantic harness files as Roadmap 2 physical harness
  authority just because they have similar names.

**Test requirements**
- Adversarial equivalence: independently materialized S.4 closeout evidence
  over the same persisted recovery outcome admits to the same S.4.5 entry
  identity and requirement set.
- Adversarial denial: copied S.4 report fields, logs, old semantic harness
  labels, same-run self-comparison, and terminal projections cannot satisfy
  S.4.5 entry.
- Boundary proof: S.4.5 entry APIs cannot invoke S.4 recovery execution or
  mint S.5 isolation authority.
- Inventory proof: the existing dedicated workspace support and certification
  surfaces are classified as reusable mechanics, milestone-local mechanics,
  certification meaning, or obsolete semantic context.

**Engineering decisions**
- S.4.5 starts from S.4 closeout and Roadmap 2 harness doctrine.
- The first artifact is not a scenario runner; it is the typed entry and
  boundary inventory that prevents authority drift.
- Existing S.4 mechanics may be generalized only through named shared
  responsibilities.

**Open questions**
- None.

### Phase 2: Define Aspect-Native Scenario Definitions And Identity

Phase 2 defines the scenario authoring language. Scenarios are typed native
Store/Foundational values with canonical identity; JSON and terminal text can
appear only as hostile/readmission inputs. The public authoring experience must
start from an ergonomic golden-path builder that lowers into native scenario
authority rather than asking test authors to construct internal proof carriers
by hand.

**Relevant subsystems**
- `worth-store-physical-certification::authoring`
- `worth-store-physical-certification::scenario`
- `worth-store-aspect-native`
- `worth-store-test-support`
- `worth-foundational`
- `worth-proof`

**Relevant APIs**
- `physical_scenario(...)`
- `PhysicalScenarioBuilder`
- `ScenarioBuilderFixtureStep`
- `ScenarioBuilderActorStep`
- `ScenarioBuilderScheduleStep`
- `ScenarioBuilderExpectationStep`
- `CertifiedPhysicalScenario`
- `PhysicalSimulationScenarioDefinition`
- `PhysicalSimulationScenarioFamily`
- `PhysicalScenarioIntent`
- `PhysicalScenarioCanonicalIdentity`
- `PhysicalScenarioAuthorityWitness`
- `JsonScenarioAuthorityDenied`
- `TerminalProjectionScenarioDenied`
- Foundational aspects/canonicalization: `AspectValue`,
  `StructAspectValue`, `AspectKey`, `ContractValidatedAspectValue`,
  `AuthoritativeRecordAspectState`, `CanonicalBasisSequence`,
  `EquivalenceBasisId`, `derive_canonical_digest(...)`
- Proof shapes: `Recipe<Unresolved, PhysicalSimulationScenarioDefinition>`,
  `Proof<ScenarioCanonicalBasis, StoreScenarioAuthority>`, `CanonicalVec`,
  `NonEmpty`, and checked denial outcomes

**Warnings**
- Do not invent a string DSL whose strings become authority.
- Do not author ordinary scenarios through `serde_json::Value`.
- Do not let fixture names, expected error text, or comments carry oracle
  meaning.
- Do not flatten operation intent, data shape, fault posture, profile, and
  expected proof into one bag of fields.
- Do not expose internal Foundational bundles, Proof witnesses, evidence
  policies, or transcript readmission types as mandatory ordinary authoring
  inputs.

**Test requirements**
- Adversarial replay: the same native scenario authored through independent
  builders produces the same canonical scenario identity, digest basis, and
  unresolved Proof recipe payload.
- Adversarial denial: raw strings, JSON values, terminal projection documents,
  fixture labels, and copied digest strings cannot satisfy scenario authority.
- Aspect-native proof: scenario fields lower through Foundational aspect
  contracts and validated state rather than JSON-shaped intermediate state.
- Structural proof: scenario definitions use proof-bearing collections where
  non-empty actor sets, canonical operation order, unique actor ids, or
  disjoint fault scopes are required.
- Golden-path proof: an S.5 readiness shape probe can be authored through the
  public builder using fixture, actor, schedule, expectation, and counter
  clauses, then lower into the same native scenario definition as the fully
  explicit internal construction without claiming implemented S.5 behavior.
- Ergonomics denial: a certification lane that requires ordinary authors to
  hand-construct Proof recipes, Foundational materialization bundles,
  canonical digest rows, transcript readmission wrappers, or evidence-bundle
  internals fails S.4.5 closeout.

**Engineering decisions**
- Scenario identity is canonical meaning, not a test filename.
- Scenario definitions declare physical intent and pressure shape; they do not
  decide execution outcomes.
- The scenario family vocabulary is shared enough for S.5-S.12, but
  milestone-specific scenario extensions remain in the consuming milestone.
- The public authoring API is the daily driver; internal proof and evidence
  machinery is the lowered representation.

**Open questions**
- None.

### Phase 3: Lower Scenarios Into Explicit Simulation Plans

Phase 3 resolves scenario definitions into simulation plans that name every
required capability before execution: actors, drivers, observers, oracle
families, counter contracts, fixture classes, evidence policy, and forbidden
shortcuts.

**Relevant subsystems**
- `worth-store-physical-certification::planning`
- `worth-store-physical-certification::profiles`
- `worth-store-physical-certification::coverage`
- `worth-store-test-support::physical_simulation`
- `worth-proof`

**Relevant APIs**
- `PhysicalSimulationPlan`
- `PhysicalSimulationCapabilitySet`
- `RequiredPhysicalDriverSet`
- `RequiredObserverSet`
- `RequiredOracleFamilySet`
- `SimulationEvidencePolicy`
- `ForbiddenShortcutSet`
- `SimulationPlanDenial`
- Foundational performance/profile surfaces:
  `FoundationalProfileSet`, `CertificationPostureProfile`,
  `FoundationalPerformanceBoundary`,
  `FoundationalPerformanceCounterName`,
  `FoundationalPolicyAdmissionReceipt`
- Proof progression:
  `Recipe<Resolved, PhysicalSimulationScenarioBasis>`,
  `Recipe<Lowered, PhysicalSimulationPlan>`,
  `CapabilityWitness<SimulationPlanningCapability>`,
  `UniqueVec`, `CanonicalVec`, `DisjointPair`, and checked lower outcomes

**Warnings**
- Do not let the executor choose drivers, observers, or oracles after seeing
  results.
- Do not hide unsupported capabilities behind skipped tests.
- Do not mark a scenario executable without naming forbidden shortcuts.
- Do not let a plan use a weaker profile than the scenario requested.

**Test requirements**
- Adversarial equivalence: semantically identical scenario definitions lower
  into the same simulation plan identity, required capability set, counter
  contract set, and evidence policy.
- Adversarial denial: missing production driver, missing observer, missing
  oracle family, unsupported profile, ambiguous fault scope, or absent
  forbidden-shortcut set denies before execution.
- Plan/execute separation proof: execution APIs accept only lowered/admitted
  plans and cannot rediscover plan meaning.
- Profile proof: developer smoke, CI certification, local soak, release
  certification, and hardware qualification profiles preserve the same proof
  model while changing scale and cost envelope.

**Engineering decisions**
- Planning is where scenario intent becomes executable harness shape.
- Unsupported physical capability is a typed denial, not a skipped assertion.
- Counter expectations and evidence policy are plan inputs, not post-run
  decoration.

**Open questions**
- None.

### Phase 4: Define Actors And Production-Facing Driver Contracts

Phase 4 defines the actor and driver surfaces that make simulations touch the
same boundary shapes production will use. It also freezes the yieldpoint
contract that makes deterministic scheduling real rather than timing theater.

**Relevant subsystems**
- `worth-store-physical-certification::actors`
- `worth-store-physical-certification::drivers`
- `worth-store-physical-backend`
- `worth-store-recovery-physics`
- `worth-store-offline-verifier`
- `worth-store-test-support::physical_simulation`

**Relevant APIs**
- `PhysicalSimulationActor`
- `ForegroundReadActor`
- `ForegroundWriteActor`
- `RecoveryActor`
- `CheckpointActor`
- `CompactionActor`
- `ReclaimActor`
- `ScrubActor`
- `OfflineVerifierActor`
- `PhysicalSimulationDriver`
- `ProductionStorageBoundaryDriver`
- `AdversarialStorageBoundaryDriver`
- `CrashRuntimeIsolationDriver`
- `MemoryPressureDriver`
- `IoPressureDriver`
- `DriverCapabilityProfile`
- `PhysicalBoundaryYieldpoint`
- `YieldpointDeclaration`
- `YieldpointPauseReceipt`
- `YieldpointResumeReceipt`
- `YieldpointObservationReceipt`
- `YieldpointScheduleBinding`
- `UnboundYieldpointScheduleDenied`
- `PrivateMutationDriverDenied`
- Foundational boundary role claims for driver evidence:
  `FoundationalBoundaryRoleClaim`, `SupportOnlyRole`, `ReceiptEvidenceRole`
- Proof witnesses:
  `CapabilityWitness<ProductionBoundaryDriverCapability>`,
  `AuthorityWitness<DriverAdmissionAuthority>`, and checked admission outcomes

**Warnings**
- Do not mutate private structs to simulate disk, crash, corruption, or
  maintenance behavior.
- Do not let test support decide whether a behavior is correct.
- Do not use one "world" driver that hides storage, recovery, verifier,
  memory, I/O, and actor responsibilities behind one mutable handle.
- Do not let actor names imply semantic authority they do not own.
- Do not schedule a pause, fault, crash, corruption, resume, or observation
  against an unnamed seam.
- Do not rely on sleeps or ambient thread timing where a production-boundary
  yieldpoint is required.

**Test requirements**
- Adversarial parity: a simulation actor using the production-facing storage
  boundary produces the same admitted boundary trace as the corresponding
  production operation under a no-fault control schedule.
- Adversarial denial: private mutation drivers, fake in-memory-only drivers,
  direct field pokes, bypassed storage boundaries, and test-support verdict
  drivers cannot satisfy driver admission.
- Capability proof: each driver declares capability profile, supported fault
  classes, unsupported fault classes, and evidence surfaces before planning can
  consume it.
- Boundary proof: storage, crash, verifier, memory, I/O, and future
  blob/security/repair drivers remain separate enough to replace or deny each
  capability independently.
- Yieldpoint proof: drivers declare named deterministic yieldpoints for the
  seams relevant to their capability profile, including before/after WAL
  append, flush, root load, root swap, page pin, lease publish, reclaim
  eligibility, checkpoint manifest write, compaction cutover, crash seam, and
  future extension seams where applicable.
- Yieldpoint denial: a schedule that pauses, faults, resumes, or observes a
  driver without a declared yieldpoint binding cannot enter execution.

**Engineering decisions**
- Actors describe who takes a step; drivers describe how the step crosses a
  production-like boundary.
- Test support can provide hostile mechanics but cannot own the proof verdict.
- Driver capability profiles are part of the plan basis.
- Yieldpoints belong to production-facing driver contracts, not to ad hoc test
  sleeps.

**Open questions**
- None.

### Phase 5: Build Deterministic Schedules And Replay Identity

Phase 5 defines deterministic simulation ordering, replayable seeds, actor step
identity, state-space budgets, and shrink/minimization metadata.

**Relevant subsystems**
- `worth-store-physical-certification::schedule`
- `worth-store-physical-certification::execution`
- `worth-store-test-support::physical_simulation`
- `worth-proof`
- `worth-foundational`

**Relevant APIs**
- `PhysicalInterleavingSchedule`
- `PhysicalActorStep`
- `ReplaySeed`
- `ScheduleReplayIdentity`
- `StateSpaceBudget`
- `PartialOrderReductionPosture`
- `ScheduleShrinkTrace`
- `ScheduleReplayDenial`
- Foundational canonicalization:
  `CanonicalBasisSequence`, `CanonicalExportBundle`,
  `compare_canonical_basis(...)`, and `derive_canonical_digest(...)`
- Proof collections:
  `CanonicalVec<PhysicalActorStep>`, `NonEmpty<PhysicalActorStep>`,
  `UniqueVec<PhysicalActorId>`, `Proof<CanonicalOrder, _>`, and
  `Proof<Uniqueness, _>`

**Warnings**
- Do not rely on thread timing, sleeps, wall-clock order, hash iteration order,
  or platform map ordering as schedule authority.
- Do not call random stress replayable unless seed, actor steps, event choices,
  and profile are captured.
- Do not let schedule shrinking erase the physical event that proves the
  failure.
- Do not use broad dynamic graph machinery where fixed actor-step sequences are
  enough.

**Test requirements**
- Adversarial replay: the same scenario, plan, seed, profile, and actor set
  reproduce the same schedule, actor steps, event choices, and schedule digest
  across independent runs.
- Adversarial denial: schedules depending on wall-clock time, unordered maps,
  ambient threads, missing seed, missing actor ids, or unbounded exploration
  cannot enter execution.
- Shrink proof: minimized failing schedules preserve the original failure
  class, fault locus, counter mismatch, and oracle verdict.
- Cost proof: schedule generation records state-space budget, explored steps,
  pruned steps, and partial-order-reduction posture where used.

**Engineering decisions**
- Determinism is a product of scenario identity, plan identity, seed, actor
  step order, and profile, not a hope that a stress test repeats.
- Random campaigns may exist only as deterministic seeded campaigns.
- S.5 consumes this phase directly for hostile maintenance interleavings.

**Open questions**
- None.

### Phase 6: Define Fault, Corruption, Crash, And Interleaving Events

Phase 6 defines the event vocabulary the scheduler can deliver through
production-facing drivers.

**Relevant subsystems**
- `worth-store-physical-certification::faults`
- `worth-store-physical-certification::drivers`
- `worth-store-test-support::physical_simulation`
- `worth-store-physical-integrity`
- `worth-store-recovery-physics`

**Relevant APIs**
- `PhysicalFaultEvent`
- `CrashEvent`
- `TornWriteEvent`
- `DroppedFlushEvent`
- `ReorderedPersistenceEvent`
- `ByteCorruptionEvent`
- `StaleGenerationEvent`
- `DelayedReleaseEvent`
- `BlockedReclaimEvent`
- `IoStallEvent`
- `FaultDeliveryReceipt`
- `FaultDeliveryDenial`
- `PhysicalArtifactFaultLocus`
- Foundational locators:
  `BoundaryArtifactLocator`, `BoundarySourceLocator`,
  `BoundaryMismatchLocator`, `FoundationalBoundaryEvidenceLocality`
- Proof progression:
  `Recipe<Lowered, FaultDeliveryPlan>`,
  `ExecutionReadyRecipe<FaultDeliveryPlan, _>`, `ExecutedRecipe<_>`, and
  checked execution outcomes

**Warnings**
- Do not corrupt arbitrary bytes without declaring artifact class, field kind,
  offset, and expected localization posture.
- Do not simulate crashes by keeping live heap, caches, buffer pools, mmap
  views, or singletons alive.
- Do not deliver a fault outside the production-facing driver boundary.
- Do not accept "some error occurred" as corruption proof.

**Test requirements**
- Adversarial localization: declared fault events produce delivery receipts
  naming artifact kind, field kind, operation seam, expected localization, and
  actual observed boundary.
- Adversarial denial: arbitrary byte scribbles, private object mutation,
  same-process crash simulation, post-decode corruption, and ambiguous fault
  loci deny before execution.
- Fresh-runtime proof: crash events force runtime isolation evidence and cannot
  reuse live heap, handles, cache state, buffer-pool state, mmap views,
  singletons, arenas, or registries.
- Event parity proof: no-fault control events leave production boundary traces
  equivalent to the corresponding ordinary production operation.

**Engineering decisions**
- Faults are scheduled physical events, not test helper mutations.
- Corruption events know the physical boundary they attack.
- Crash events require fresh-runtime evidence.

**Open questions**
- None.

### Phase 7: Define Observers And Certification-Owned Oracles

Phase 7 separates observation mechanics from proof judgment. Observers collect
facts; oracles decide certification verdicts.

**Relevant subsystems**
- `worth-store-physical-certification::observation`
- `worth-store-physical-certification::oracles`
- `worth-store-certification`
- `worth-store-offline-verifier`
- `worth-foundational`
- `worth-proof`

**Relevant APIs**
- `PhysicalSimulationObserver`
- `ObservedPhysicalTrace`
- `IndependentVerifierObservation`
- `PhysicalProofOracle`
- `PhysicalProofOracleVerdict`
- `OracleVerdictBasis`
- `ReusablePhysicalOracleFamily`
- `NoMixedRootOracle`
- `OldReaderSeesOldRootOracle`
- `PostSwapReaderSeesNewRootOracle`
- `BlockedReclaimUntilReleaseOracle`
- `CrashRecoversOldOrNewNeverMixedOracle`
- `NoPrivateMutationOracle`
- `NoJsonAuthorityOracle`
- `CounterContractOracle`
- `TranscriptReplayOracle`
- `IndependentVerifierAgreementOracle`
- `TestSupportOracleDenied`
- `SameRunSelfComparisonDenied`
- `LogOnlyEvidenceDenied`
- Foundational diagnostics/evidence:
  `FoundationalDiagnosticDecisionRow`,
  `FoundationalDiagnosticFailureRow`,
  `FoundationalCertifiedDiagnosticBundle`,
  `FoundationalBoundaryEvidenceCompletedReceiptArtifact`,
  `FoundationalBoundaryEvidenceSupportBasisDisclosure`
- Proof outcomes:
  `TransitionOutcome`, `ProofOutcome`, `SuccessfulTransitionOutcome`,
  `DenialTransitionOutcome`, `DeferredTransitionOutcome`,
  `FreshnessTransitionOutcome`

**Warnings**
- Do not let `worth-store-test-support` own expected truth or final verdicts.
- Do not compare a runtime only to itself when an independent verifier lane is
  required.
- Do not accept logs, printed traces, expected error strings, or successful
  completion as oracle evidence.
- Do not flatten success, denial, deferment, stale, rebind-required, and failure
  into one boolean.
- Do not force every scenario to hand-roll a custom oracle when a reusable
  oracle family expresses the proof meaning.
- Do not let reusable oracle families become generic mega-oracles whose names
  no longer predict the invariant they prove.

**Test requirements**
- Adversarial convergence: independent observers over the same executed
  simulation produce oracle-consumable traces that converge on the same
  certification verdict where the scenario declares convergence.
- Adversarial denial: test-support-owned verdicts, logs, expected error text,
  same-run self-comparison, and fixture labels cannot satisfy oracle verdict
  APIs.
- Offline verifier proof: where a scenario requires an independent verifier,
  runtime and verifier observations are both present and disagreement becomes
  typed evidence rather than hidden failure.
- Outcome topology proof: oracle verdicts preserve success, denial, deferment,
  stale, rebind-required, and failure categories where the underlying Proof
  progression exposes them.
- Oracle library proof: the initial S.5 readiness oracle set includes reusable
  families for mixed-root denial, old-reader/new-root behavior, blocked reclaim
  until release, crash old-or-new-never-mixed, private mutation denial, JSON
  authority denial, counter contracts, transcript replay, and independent
  verifier agreement.
- Composition proof: a public scenario can compose multiple reusable oracle
  families without custom oracle code while still producing a precise verdict
  basis for each family.

**Engineering decisions**
- Observers are mechanics; oracles are certification meaning.
- Certification owns verdicts and closeout evidence.
- Foundational diagnostics explain verdicts; they do not replace Store oracle
  authority.
- Reusable oracle families are the scaling unit for later milestone tests.

**Open questions**
- None.

### Phase 8: Define Exact Counter Contracts And Resource Profiles

Phase 8 makes cost and resource claims testable through exact counter
expectations and profile-scoped envelopes.

**Relevant subsystems**
- `worth-store-physical-certification::counters`
- `worth-store-physical-certification::profiles`
- `worth-store-budgets`
- `worth-store-buffer-pool`
- `worth-store-io-scheduler`
- `worth-foundational`

**Relevant APIs**
- `PhysicalCounterContract`
- `PhysicalCounterExpectation`
- `CounterExpectationKind::{Zero, Positive, Exact, Monotonic, Bounded}`
- `CounterStrengthPosture`
- `CounterStrengthJustification`
- `OverExactCounterDenied`
- `PhysicalResourceEnvelope`
- `SimulationProfile`
- `DeveloperSmokeProfile`
- `CiCertificationProfile`
- `LocalSoakProfile`
- `ReleaseCertificationProfile`
- `HardwareQualificationProfile`
- `CounterMismatchEvidence`
- Foundational performance:
  `FoundationalPerformanceCounterRow`,
  `FoundationalCounterBackedPerformanceReceipt`,
  `FoundationalPerformanceBudgetKind`,
  `FoundationalPerformanceEvidenceStrength`,
  `FoundationalCertifiedPerformanceBundle`

**Warnings**
- Do not assert counters are merely nonzero when the claim requires exact zero,
  exact count, monotonicity, or boundedness.
- Do not use elapsed time alone as performance proof.
- Do not let smoke profile success imply CI, release, or hardware
  qualification readiness.
- Do not inspect resource usage only after execution if the forbidden behavior
  could have happened mid-run.
- Do not make implementation-sensitive performance counters exact merely
  because exactness is easy to assert today.
- Do not weaken forbidden behavior counters below exact zero.

**Test requirements**
- Adversarial equivalence: the same executed scenario under the same profile
  emits the same counter rows, budget rows, and Foundational performance basis
  across independent runs.
- Adversarial denial: missing counter specs, duplicate counter rows,
  unexpected rows, nonzero forbidden counters, under-strength evidence, and
  profile mismatch deny certification.
- Envelope proof: memory, allocation, page pins, dirty pages, I/O queue,
  latency/interference, and future blob streaming envelopes are enforced during
  the run, not only after completion.
- Profile proof: smoke, CI, soak, release, and hardware profiles are distinct
  certified postures and cannot certify each other by projection.
- Counter strength proof: forbidden behavior counters and deterministic event
  structure counters use exact expectations where exactness is the claim, while
  implementation-sensitive counters such as latch waits, retry counts, page
  pins, allocation counts, I/O queue depth, replayed pages, and blocked reclaim
  attempts use the weakest sufficient zero, positive, monotonic, bounded, or
  profile-scoped expectation.
- Brittleness denial: a counter contract that over-specifies an
  implementation-sensitive cost without declaring exactness as the proof claim
  denies as over-exact.

**Engineering decisions**
- Counter contracts are part of the lowered plan.
- Foundational performance surfaces package executed counter evidence; they do
  not measure or certify Store behavior by themselves.
- Profiles change scale and envelope, not the proof model.
- Counter strength is part of the contract; stronger is not automatically
  better when it makes tests brittle without proving more truth.

**Open questions**
- None.

### Phase 9: Build Production-Backed Fixture Builders

Phase 9 defines fixtures as production-backed physical worlds, not synthetic
state sketches. Fixture builders create persisted stores and physical artifact
sets through admitted production-facing paths, then attach mutation/corruption
capabilities only at declared physical seams.

**Relevant subsystems**
- `worth-store-physical-certification::fixtures`
- `worth-store-test-support::physical_simulation`
- `worth-store-physical-format`
- `worth-store-buffer-pool`
- `worth-store-recovery-physics`
- `worth-store-physical-integrity`

**Relevant APIs**
- `ProductionBackedPhysicalFixture`
- `PhysicalFixtureBuilder`
- `FixtureCapabilityDeclaration`
- `FixtureAuthorityReceipt`
- `FixtureMutationBoundary`
- `SyntheticFixtureAuthorityDenied`
- `PersistedStoreFixtureManifest`
- `LargeStoreFixtureProfile`
- `PhysicalArtifactFixtureCatalog`
- Foundational provenance/canonicalization:
  `FoundationalBoundaryEvidenceProvenanceArtifact`,
  `FoundationalBoundaryEvidenceSourceBasis`,
  `CanonicalBasisBundle`, and `BoundaryArtifactId`
- Proof witnesses:
  `AuthorityWitness<FixtureConstructionAuthority>`,
  `Proof<FixtureProvenance, StoreFixtureAuthority>`, and
  `FreshnessScopedBasis<CurrentValidity, _>`

**Warnings**
- Do not create physical authority by hand-filling structs.
- Do not use small in-memory stores to certify larger-than-memory behavior.
- Do not let corruption injectors create expected truth by labeling a fixture.
- Do not let fixture builders bypass page, WAL, checkpoint, manifest, or
  buffer-pool APIs that production behavior depends on.

**Test requirements**
- Adversarial parity: a fixture built through production-facing paths reopens
  through the same physical authority and semantic digest expected by the
  independent fixture manifest.
- Adversarial denial: hand-filled physical structs, private storage mutation,
  synthetic in-memory stores, copied fixture receipts, and fixture labels
  cannot satisfy fixture authority.
- Scale proof: `store_larger_than_memory`, `checkpoint_heavy`,
  `compaction_heavy`, `foreground_under_background_io`, and future
  `blob_larger_than_memory` fixture profiles declare size relative to budgets.
- Mutation-boundary proof: fixtures expose only declared physical mutation
  seams and record whether the seam attacks pages, frames, WAL, manifests,
  indexes, chunks, audit records, key envelopes, tenant metadata, or later
  repair artifacts.

**Engineering decisions**
- Fixtures are starting worlds with provenance, not proof verdicts.
- Test support may construct worlds and expose seams; certification oracles
  decide whether executed behavior satisfies the claim.
- Fixture scale classes become reusable across S.5-S.12.

**Open questions**
- None.

### Phase 10: Emit Replayable Transcripts And Evidence Bundles

Phase 10 defines the durable output of every simulation run: transcripts,
replay identity, counter snapshots, observer traces, oracle verdicts, evidence
bundles, and terminal projections.

**Relevant subsystems**
- `worth-store-physical-certification::transcript`
- `worth-store-physical-certification::evidence`
- `worth-store-physical-certification::execution`
- `worth-store-certification`
- `worth-foundational`
- `worth-proof`

**Relevant APIs**
- `PhysicalSimulationTranscript`
- `PhysicalStoryTranscript`
- `SimulationReplayBundle`
- `PhysicalCertificationEvidenceBundle`
- `SimulationRunIdentity`
- `SimulationFailureDigest`
- `TranscriptReplayDenial`
- `TerminalProjectionOnlyEvidenceDenied`
- Foundational surfaces:
  `FoundationalBoundaryMaterializationBundle`,
  `FoundationalCertifiedDiagnosticBundle`,
  `FoundationalBoundaryEvidenceCompletedReceiptArtifact`,
  `FoundationalBoundaryEvidenceExecutedReceiptArtifact`,
  `FoundationalCounterBackedPerformanceReceipt`,
  `BoundaryBridgedCanonicalExportArtifact`,
  `readmit_canonical_export_after_boundary(...)`
- Proof execution:
  `ExecutionReadyRecipe<PhysicalSimulationExecution, _>`,
  `ExecutedRecipe<PhysicalSimulationExecution, _>`,
  `ExecuteReadyRecipeTransition`, and checked readmission outcomes

**Warnings**
- Do not make logs the evidence bundle.
- Do not include artifacts in JSON as semantic authority; terminal JSON is a
  projection only.
- Do not omit seed, profile, source revision, format version, driver profile,
  actor steps, faults, counters, oracle verdicts, or verifier comparison when
  the scenario depends on them.
- Do not let a transcript replay require the original process or live heap.

**Test requirements**
- Adversarial replay: an executed transcript can replay from its scenario,
  plan, schedule, driver profiles, fault events, fixture manifest, and seed
  without access to original live runtime state.
- Adversarial denial: loose logs, terminal JSON, missing seeds, missing driver
  profiles, copied transcript fields, same-run self-comparison, and missing
  oracle verdicts cannot satisfy evidence bundle APIs.
- Foundational materialization proof: transcripts and bundles lower into
  Foundational canonical, diagnostic, evidence, profile, and performance
  surfaces without becoming Store physical authority.
- Readmission proof: boundary-bridged transcripts require explicit readmission
  before supporting a later certification comparison.

**Engineering decisions**
- Transcript is the replayable story; evidence bundle is the certification
  envelope.
- Terminal projections are for humans and terminals only.
- Canonical transcript identity is required before cross-run comparison.

**Open questions**
- None.

### Phase 11: Define Coverage Matrix And Harness Maturity Ladder

Phase 11 turns Roadmap 2 coverage expectations into typed rows and maturity
evidence that later milestones can consume. Coverage must be generated from
registered executable surfaces, not hand-maintained as certification prose.

**Relevant subsystems**
- `worth-store-physical-certification::coverage`
- `worth-store-physical-certification::profiles`
- `worth-store-certification`
- `worth-store-readiness`
- `worth-foundational`

**Relevant APIs**
- `Roadmap2PhysicalCoverageMatrix`
- `PhysicalCoverageMatrixRow`
- `GeneratedCoverageMatrix`
- `RegisteredScenarioCoverageRow`
- `RegisteredOracleCoverageRow`
- `RegisteredCounterCoverageRow`
- `RegisteredTranscriptCoverageRow`
- `MutationResultCoverageRow`
- `HarnessMaturityLevel`
- `HarnessMaturityEvidence`
- `SequenceHarnessDependency`
- `CoverageGapDenial`
- `CoverageRowSatisfiedReceipt`
- `Roadmap2HarnessReadinessReport`
- Foundational diagnostics:
  `FoundationalDiagnosticNamedGap`,
  `FoundationalDiagnosticCertifiedCoverageClass`,
  `FoundationalDiagnosticSurfaceAvailability`,
  `FoundationalDiagnosticSupportReport`

**Warnings**
- Do not use coverage prose as closeout evidence.
- Do not treat `Exists` or `SmokeWorks` as CI certification readiness.
- Do not let one suite family satisfy unrelated artifact classes, publication
  seams, background subsystems, tenant/security surfaces, or repair workflows.
- Do not hide missing coverage behind allowed debt unless the consuming
  milestone names and owns that debt.
- Do not let a manually edited coverage file or markdown checklist satisfy a
  coverage row.
- Do not claim coverage for an artifact class, seam, actor set, driver profile,
  oracle family, counter contract, transcript output, or mutation result unless
  a registered scenario or suite produced it.

**Test requirements**
- Adversarial completeness: S.5's declared harness dependencies map to
  satisfied coverage rows for deterministic scheduling, actors, faults,
  production drivers, observers, oracles, transcripts, counters, and mutation
  validation readiness.
- Adversarial denial: missing coverage row, smoke-only maturity,
  wrong-sequence maturity evidence, unsupported profile, or prose-only
  certification cannot satisfy closeout.
- Matrix proof: rows distinguish artifact class, publication seam, fault phase,
  resource envelope, background interference, authority family, offline
  verifier, and mutation validation posture.
- Foundational diagnostics proof: coverage gaps materialize as support
  diagnostics and named gaps without becoming Store authority or hiding the
  missing lane.
- Generated coverage proof: coverage rows are derived from actual registered
  scenario definitions, actor sets, yieldpoints, fault classes, driver
  profiles, observer sets, oracle families, counter contracts, transcript
  outputs, and mutation results.
- Manual coverage denial: hand-authored coverage prose, edited matrix rows,
  unchecked maturity claims, and missing registration evidence cannot satisfy
  the coverage matrix.

**Engineering decisions**
- Coverage rows are typed obligations, not markdown checkboxes.
- S.4.5 must close enough harness maturity for S.5 CI certification; later
  rows may remain planned when their consuming milestone has not started.
- Maturity is per subsystem and sequence, not global vibe.
- The coverage matrix is an output generated from registered proof surfaces.

**Open questions**
- None.

### Phase 12: Prove Forbidden Shortcut Rejection

Phase 12 builds compile-fail and runtime denial lanes for every shortcut the
harness exists to prevent.

**Relevant subsystems**
- `worth-store-physical-certification`
- `worth-store-test-support`
- `worth-store-certification`
- `worth-store-aspect-native`
- `worth-proof`

**Relevant APIs**
- `SyntheticHarnessShortcutRejectionReport`
- `ForbiddenShortcutSet`
- `PrivateMutationDenied`
- `JsonScenarioAuthorityDenied`
- `LogOnlyEvidenceDenied`
- `SameRunSelfComparisonDenied`
- `TestSupportOracleDenied`
- `TerminalProjectionAuthorityDenied`
- `ProjectionAuthorityDenied`
- `SkippedProgressionDenied`
- Proof privacy/progression compile-fail surfaces for scenario, plan,
  schedule, execution, transcript, verdict, evidence, and readiness stages

**Warnings**
- Do not rely only on review discipline to prevent shortcut tests.
- Do not let terminal projection quarantine cover ordinary scenario authority
  if a stronger native denial is needed.
- Do not let Foundational or Proof artifacts be accepted by Store APIs that
  require Store-owned simulation readiness.
- Do not skip compile-fail coverage where the shortcut should be uncallable.

**Test requirements**
- Adversarial denial: raw JSON scenario authority, terminal projection text,
  logs, same-run self-comparison, private mutation, fixture labels,
  test-support oracle meaning, copied evidence fields, and skipped Proof
  stages all fail at named boundaries.
- Adversarial parity: legitimate native scenario, lowered plan, admitted
  schedule, executed transcript, certification oracle verdict, and evidence
  bundle can close the same lane the shortcuts fail.
- Compile-fail proof: constructors for stronger scenario, plan, schedule,
  execution, transcript, oracle verdict, evidence bundle, and S.5 readiness
  forms are sealed against external minting.
- Projection-authority proof: Foundational reports, diagnostics, receipts,
  performance bundles, and Proof traces cannot satisfy Store physical
  simulation authority APIs without the Store-owned witness type.

**Engineering decisions**
- The forbidden-shortcut suite is part of the product harness, not an optional
  QA extra.
- Arch Law 41 applies directly: out-of-order progression must be uncallable.
- Positive control lanes must prove the denial suite is not just blocking
  everything.

**Open questions**
- None.

### Phase 13: Publish S.5 Simulation Harness Readiness

Phase 13 publishes the typed handoff S.5 needs: deterministic interleaving
support, maintenance actors, crash/restart support, production drivers,
observers, reusable oracle families, counters, transcripts, yieldpoints, and
forbidden shortcut rejection.

**Relevant subsystems**
- `worth-store-physical-certification`
- `worth-store-physical-isolation`
- `worth-store-test-support`
- `worth-store-certification`
- `worth-store-readiness`
- `worth-foundational`
- `worth-proof`

**Relevant APIs**
- `S5SimulationHarnessReadiness`
- `S5InterleavingHarnessCapability`
- `S5MaintenanceActorCapabilitySet`
- `S5IsolationOracleFamilySet`
- `S5CounterContractSet`
- `S5RequiredYieldpointSet`
- `S5ReadinessShapeProbeScenario`
- `S5IsolationCorrectnessNonClaim`
- `S5ReusableOracleReadiness`
- `S5HarnessCoverageReceipt`
- `S5HarnessReadinessDenied`
- Foundational evidence/performance:
  `FoundationalBoundaryEvidenceCompletedReceiptArtifact`,
  `FoundationalBoundaryEvidenceRuntimeNonAssumption`,
  `FoundationalCertifiedPerformanceBundle`,
  `FoundationalCertifiedDiagnosticBundle`
- Proof progression:
  `Recipe<Admitted, S5HarnessReadinessCandidate>`,
  `ExecutionReadyRecipe<S5HarnessReadinessCandidate, _>`,
  `ExecutedRecipe<S5HarnessReadinessCloseout, _>`,
  `AuthorityWitness<S5HarnessReadinessAuthority>`,
  and checked execution/readmission outcomes

**Warnings**
- Do not claim S.5 physical isolation is implemented.
- Do not hand S.5 a generic runner without named support for
  protect-before-observe, root swaps, byte guards, reclaim barriers,
  deterministic schedules, restart during cutover, and shortcut rejection.
- Do not omit exact counter contract support for latch waits, epoch retries,
  protected references, blocked reclaim, publication swaps, and future
  S.5-specific counters.
- Do not let S.5 define a second scenario/scheduler/oracle/transcript skeleton.
- Do not hand S.5 only extension slots; it must receive the concrete
  ready-now actor, yieldpoint, oracle, counter, transcript, and shortcut
  surfaces needed for its readiness shape probes.
- Do not claim any S.5 isolation law has passed because an S.4.5 shape probe
  could be expressed or executed against stubbed/non-authoritative actors.

**Test requirements**
- Adversarial readiness: S.5 can express protect-before-observe, root-kind
  separation, traversal admission, byte guard usage, no-hidden-latch-I/O,
  publication memory ordering, lease-expiry non-authority, and free/reuse
  generation fence scenarios through S.4.5 harness surfaces.
- Adversarial denial: a generic runner missing deterministic schedules,
  maintenance actors, production-boundary drivers, S.5 oracle families, counter
  contracts, or forbidden-shortcut suites cannot produce
  `S5SimulationHarnessReadiness`.
- Handoff replay proof: an S.5 readiness shape probe can lower, schedule,
  execute, transcript, verdict, and evidence-bundle through S.4.5 with
  stubbed/non-authoritative actors and explicit non-claim evidence, without
  implementing or certifying S.5 product behavior.
- Foundational/Proof proof: readiness evidence materializes into Foundational
  receipt, diagnostic, runtime-assumption, non-assumption, and performance
  surfaces, while the handoff itself is a Store-owned Proof-progressed artifact.
- Public authoring proof: the S.5 readiness shape probe uses
  the golden-path scenario builder rather than direct internal proof/evidence
  construction.
- Extension-slot denial: future blob lifecycle, tenant/security, repair/PITR,
  hardware qualification, and full S.12 campaign slots are visible as typed
  extension slots but cannot masquerade as implemented S.5-ready behavior.

**Engineering decisions**
- S.5 readiness proves harness capability, not isolation correctness.
- The S.5 spec remains the authority for which physical laws S.5 must prove.
- S.4.5 hands over enough substrate that S.5 implementation can focus on
  physical isolation rather than harness invention.
- S.4.5 implements the S.5 harness-readiness slice and declares future slices
  as extension slots only. The S.5 slice proves expressiveness, not S.5
  correctness.

**Open questions**
- None.

### Phase 14: Close The Physical Simulation Harness Milestone

Phase 14 runs the S.4.5 acceptance suites, rejects synthetic shortcuts, records
coverage maturity, dogfoods the public authoring API on already-implemented
lanes, runs the S.5 readiness shape probe with non-claim evidence, and freezes
the handoff contract for S.5.

**Relevant subsystems**
- `worth-store-physical-certification`
- `worth-store-test-support`
- `worth-store-certification`
- `worth-store-readiness`
- `worth-foundational`
- `worth-proof`

**Relevant APIs**
- `PhysicalSimulationHarnessCloseoutSuite`
- `PhysicalSimulationHarnessCertificationBundle`
- `PhysicalSimulationHarnessCloseoutReport`
- `SyntheticHarnessShortcutRejectionReport`
- `Roadmap2HarnessReadinessReport`
- `S5SimulationHarnessReadiness`
- `S45HarnessDogfoodReport`
- `S4RecoveryDogfoodScenario`
- `S5ReadinessShapeProbeScenario`
- `S5CorrectnessNonClaimEvidence`
- `ShortcutRejectionDogfoodScenario`

**Warnings**
- Do not close on one successful run.
- Do not close on S.4-only recovery tests.
- Do not close on prose coverage or planned coverage.
- Do not claim S.12 certification or release/aerospace posture from S.4.5.
- Do not leave S.5 with any need to invent the core harness skeleton locally.
- Do not close if the public authoring API is bypassed by all vertical slices.
- Do not treat the S.5 readiness shape probe as S.5 implementation evidence.
- Do not make S.4.5 implement future S.6-S.12 behavior just to prove the
  extension points exist.

**Test requirements**
- Adversarial closeout: every S.4.5 phase produces named positive, hostile,
  forbidden-shortcut, replay, and readiness evidence.
- Adversarial denial: logs, JSON authority, private mutation, fixture labels,
  same-run self-comparison, test-support oracle meaning, copied evidence,
  projection authority, and skipped Proof stages cannot close S.4.5.
- Integration proof: at least one S.4 recovery scenario and one S.5 readiness
  shape probe run through the shared harness pipeline from native definition to
  evidence bundle.
- Vertical-slice proof: the S.4 recovery slice and shortcut rejection slice
  dogfood the public golden-path authoring API. The readiness shape-probe
  slice uses that same API only to prove harness expressiveness for the next
  milestone. All three produce deterministic replay bundles and generated
  coverage rows, and the probe keeps correctness non-claim evidence explicit.
- Mutation validation proof: controlled harness mutants such as ignored
  scheduler seed, skipped fault delivery, stale fixture accepted, missing
  counter row, test-support oracle accepted, and private mutation accepted fail
  the intended S.4.5 suites.
- Extension-slot proof: future blob lifecycle, security/tenant, repair/PITR,
  hardware qualification, and full S.12 certification families are declared as
  typed extension slots without implemented future-domain behavior or false
  readiness claims.
- Composition proof: production, support, and certification modules stay under
  the workspace line-cap rules unless explicitly exempted and keep scenario,
  planning, scheduling, driver, observer, oracle, counter, transcript, evidence,
  coverage, and handoff responsibilities separate.

**Engineering decisions**
- S.4.5 is the Roadmap 2 physical simulation substrate milestone.
- It closes on reusable harness capability and S.5 readiness, not on the
  correctness of every future S.6-S.12 lane.
- Later milestones may extend actor, fault, fixture, oracle, and coverage
  families without changing the core scenario-to-evidence progression.
- The closeout has to feel usable in practice: the vertical slices are the
  evidence that the harness is a working test operating system, not only a
  certification schema.

**Open questions**
- None.

## Must Ship

- typed `S45SimulationHarnessEntry` from S.4 closeout and Roadmap 2 harness
  requirements
- public golden-path scenario-builder API for ordinary certification authoring
- aspect-native `PhysicalSimulationScenarioDefinition` and canonical scenario
  identity
- scenario resolution and lowering into explicit `PhysicalSimulationPlan`
- deterministic `PhysicalInterleavingSchedule`, replay seed, actor-step order,
  state-space budget, and shrink metadata
- named production-boundary yieldpoints for deterministic schedule control
- production-facing actor and driver contracts for storage, crash, corruption,
  memory, I/O, offline verification, maintenance, and future blob/security/
  repair lanes
- declared fault, corruption, crash, and interleaving event vocabulary with
  delivery receipts
- observer registry and certification-owned oracle registry
- reusable S.5-ready oracle families, including mixed-root, old-reader,
  post-swap-reader, blocked-reclaim, crash old-or-new-never-mixed,
  private-mutation denial, JSON-authority denial, counter-contract,
  transcript-replay, and independent-verifier agreement oracles
- exact counter contracts with zero, positive, exact, monotonic, and bounded
  expectation forms
- counter-strength posture requiring weakest-sufficient expectation strength
- production-backed physical fixture builders and fixture manifests
- replayable transcripts, evidence bundles, failure digests, and terminal
  projection quarantine
- generated coverage matrix rows and harness maturity evidence
- forbidden shortcut rejection across compile-fail and runtime denial lanes
- dogfooded S.4 recovery and shortcut rejection vertical slices through the
  public authoring API
- S.5 readiness shape-probe vertical slice through the public authoring API,
  with S.5 correctness non-claim evidence
- typed extension slots for future S.6-S.12 actors, faults, oracles, profiles,
  and coverage rows without future behavior implementation
- Foundational evidence, diagnostics, canonical basis, profile, provenance,
  receipt, and counter-backed performance materialization from executed Store
  simulation findings
- Proof recipes, witnesses, checked outcomes, freshness/readmission, and
  proof-bearing collections for scenario-to-evidence progression
- concrete `S5SimulationHarnessReadiness` handoff

## Must Preserve

- Store owns physical database behavior.
- S.4 owns recovery physics; S.4.5 consumes its closeout and reusable harness
  lessons.
- S.5 owns physical isolation; S.4.5 only proves the harness can express S.5's
  required hostile scenarios.
- `worth-store-test-support` owns mechanics and fixtures, not certification
  verdicts.
- `worth-store-certification` owns oracle meaning and closeout evidence.
- `worth-foundational` owns shared boundary vocabulary, not Store simulation
  authority.
- `worth-proof` owns progression law, not the runtime scheduler.
- JSON remains terminal projection or hostile/readmission input only.
- Profiles change execution scale, not proof meaning.
- Golden-path authoring lowers into strict proof machinery; it does not weaken
  the proof model.
- Future extension slots are promises of shape, not implemented future domain
  behavior.
- Unit tests and simple integration tests remain valid outside S.4.5 when they
  are not certifying hostile physical database claims.

## Acceptance Evidence

S.4.5 is complete only when the store satisfies the Roadmap 2 named suite:

- `Physical database simulation harness test`

Required machine-checkable outputs:

- `physical_simulation_scenario_definition`
- `physical_scenario_builder_lowering_trace`
- `physical_simulation_scenario_plan`
- `physical_interleaving_schedule`
- `physical_boundary_yieldpoint_trace`
- `physical_actor_step_trace`
- `fault_delivery_trace`
- `production_boundary_driver_trace`
- `observed_physical_trace`
- `physical_proof_oracle_verdict`
- `physical_counter_contract_trace`
- `counter_mismatch_evidence`
- `production_backed_fixture_manifest`
- `physical_simulation_transcript`
- `simulation_replay_bundle`
- `physical_certification_evidence_bundle`
- `roadmap2_physical_coverage_matrix`
- `generated_coverage_matrix_trace`
- `harness_maturity_evidence`
- `s45_harness_dogfood_report`
- `s5_readiness_shape_probe_non_claim_report`
- `synthetic_harness_shortcut_rejection_report`
- `foundational_simulation_evidence_bundle`
- `proof_progression_simulation_trace`
- `projection_authority_denial_trace`
- `S5SimulationHarnessReadiness`

Required acceptance suites:

- `s45_entry_boundary_suite`
  proves S.4.5 consumes S.4 closeout and Roadmap 2 harness requirements without
  reopening recovery physics or minting S.5 isolation authority.
- `aspect_native_scenario_definition_suite`
  proves scenarios are native aspect-backed definitions with canonical identity
  and no JSON scenario authority.
- `simulation_plan_lowering_suite`
  proves scenarios lower into plans that declare capabilities, drivers,
  observers, oracles, counters, evidence policy, and forbidden shortcuts before
  execution.
- `golden_path_authoring_suite`
  proves ordinary certification scenarios can be authored through the public
  builder and lower into the same internal scenario definition as explicit
  native construction.
- `production_driver_contract_suite`
  proves actors and drivers use production-facing boundaries and reject private
  mutation or fake authority.
- `yieldpoint_control_suite`
  proves deterministic schedules bind to named production-boundary yieldpoints
  and reject unnamed seams, sleeps, ambient timing, and unbound pauses/faults.
- `deterministic_schedule_replay_suite`
  proves schedules replay from scenario, plan, seed, actors, profile, and
  budget without timing luck.
- `fault_delivery_boundary_suite`
  proves crashes, torn writes, corruption, stale generations, delayed release,
  blocked reclaim, and I/O stalls are delivered through declared seams.
- `observer_oracle_separation_suite`
  proves observation mechanics and certification oracle verdicts are separate,
  and test support cannot own proof meaning.
- `oracle_library_suite`
  proves S.5-ready reusable oracle families exist and can be composed through
  public scenarios without bespoke oracle code for every lane.
- `counter_contract_profile_suite`
  proves exact counter expectations and resource envelopes are profile-scoped
  and enforced during execution.
- `counter_strength_suite`
  proves counters use exact, zero, positive, bounded, monotonic, or
  profile-scoped expectations according to weakest-sufficient proof strength.
- `production_backed_fixture_suite`
  proves fixtures are created through production-facing paths and cannot be
  certified by hand-filled structs or labels.
- `transcript_evidence_bundle_suite`
  proves transcripts and evidence bundles are replayable, diffable, and
  sufficient for offline pass/fail evaluation.
- `coverage_maturity_ladder_suite`
  proves coverage rows and maturity levels are typed and cannot be replaced by
  prose or smoke-only evidence.
- `generated_coverage_suite`
  proves coverage maturity is generated from registered scenarios, yieldpoints,
  actors, drivers, oracles, counters, transcripts, and mutation results.
- `forbidden_shortcut_rejection_suite`
  proves logs, JSON, terminal projections, same-run self-comparison, private
  mutation, fixture labels, skipped progression, copied fields, and
  test-support verdicts cannot close a lane.
- `harness_dogfood_vertical_slice_suite`
  proves one S.4 recovery slice, one S.5 readiness shape-probe slice, and one
  shortcut rejection slice run end-to-end through the public authoring API and
  full pipeline, while the S.5 slice emits explicit non-claim evidence for S.5
  physical isolation correctness.
- `extension_slot_containment_suite`
  proves future S.6-S.12 extension slots are typed and visible without
  implemented future behavior or false readiness claims.
- `foundational_proof_simulation_evidence_suite`
  proves executed Store simulation findings materialize into Foundational and
  Proof-compatible evidence without replacing Store authority.
- `s5_simulation_harness_readiness_suite`
  proves S.5 can express its required hostile isolation scenarios through the
  shared harness and cannot start from a generic runner.

Every suite must map to native scenario definition, lowered plan, schedule,
actors, drivers, observers, oracle families, transcript outputs, evidence
bundle fields, exact counter expectations, positive control lane, hostile lane,
forbidden-shortcut lane, replay lane, and mutation-validation lane.

## Allowed Debt

S.4.5 may reserve full release-scale hardware qualification, S.7 blob-specific
chunk lifecycle actors, S.10 operator-repair workflow UX, S.11 key/tenant
security campaigns, and S.12 full certification bundle expansion for their
consuming milestones when the core scenario-to-evidence harness substrate
exists and the missing rows are typed coverage gaps.

S.4.5 may not mark these as debt:

- aspect-native scenario definitions
- public golden-path authoring API
- scenario canonical identity
- scenario-to-plan Proof progression
- deterministic schedule and replay identity
- named production-boundary yieldpoints
- production-facing driver contracts
- fresh-runtime crash isolation support
- fault/corruption delivery through declared seams
- observer/oracle separation
- certification-owned oracle verdicts
- reusable S.5-ready oracle families
- exact counter contract vocabulary
- counter-strength posture
- production-backed fixture manifests
- replayable transcripts and evidence bundles
- coverage matrix and maturity ladder
- generated coverage from registered suites
- forbidden-shortcut rejection
- harness dogfood vertical slices for already-implemented lanes
- S.5 readiness shape-probe vertical slice with explicit S.5 correctness
  non-claim posture
- future extension slot containment
- Foundational evidence materialization from executed Store findings
- Proof-bearing progression/readmission surfaces
- concrete `S5SimulationHarnessReadiness`

## Sequencing Notes

S.4.5 belongs immediately after S.4 because S.4 is the first Roadmap 2 sequence
with substantial crash/fault harness mechanics worth generalizing, and because
S.5 is the first sequence whose correctness depends on deterministic hostile
interleavings rather than isolated crash/recovery paths.

Later sequences consume S.4.5 as follows:

- S.5 consumes deterministic schedules, actors, byte-stability scenario
  vocabulary, counter contracts, oracles, transcripts, and shortcut denials.
- S.6 extends drivers and profiles for I/O pressure, queue depth, foreground
  latency, and hardware capability qualification.
- S.7 extends fixtures, actors, corruption events, and counters for native blob
  chunk lifecycle and large-object streaming.
- S.8 consumes workload, corruption, verifier, counter, and coverage surfaces
  for access-path discipline.
- S.9 consumes scenario/transcript/evidence and mutation validation surfaces
  to connect formal models to implementation lanes.
- S.10 extends offline verifier, repair, backup, PITR, and operator workflow
  actors.
- S.11 extends security, tenant, key, audit, and authenticity campaigns.
- S.12 consumes the full harness as the certification program substrate.

## Required Self-Check

- Does S.4.5 solve a real structural problem? Yes: it makes Roadmap 2 hostile
  physical testing a reusable Store substrate before S.5 starts.
- Is the adversarial constraint precise and load-bearing? Yes: it names the
  exact shortcut classes and requires typed scenarios, deterministic schedules,
  production drivers, oracles, counters, transcripts, and evidence.
- Does the roadmap justify this milestone now? Yes: it belongs between S.4
  recovery and S.5 physical isolation because S.5 needs deterministic
  interleaving simulation immediately.
- Does the spec preserve crate authority boundaries? Yes: Store owns physical
  behavior, test support owns mechanics, certification owns verdicts,
  Foundational owns boundary vocabulary, and Proof owns progression law.
- Are the phases carrying most of the real design information? Yes.
- Is each phase centered on one conceptual detail or boundary? Yes.
- Does each phase contain at least two adversarial tests? Yes.
- Could a competent engineer map this spec into honest types, modules, and
  tests? Yes: the spec names crate skeleton, APIs, phase outputs, denials,
  suites, and handoff surfaces.
- Does the milestone belong in this roadmap sequence? Yes: it is the harness
  bridge from deterministic recovery physics to hostile physical isolation and
  the later Roadmap 2 certification program.
