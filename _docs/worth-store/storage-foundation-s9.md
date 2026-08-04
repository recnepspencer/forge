# Storage Foundation S.9 Engineering Spec: Executable Protocol Law For Physical Truth

## Goal

Turn the Store's highest-risk physical protocols into checked, executable
state-machine law. The checked models must refine ordinary production outcomes
for durability, recovery, compaction, lease/reclaim, quarantine/readmission,
import, replication admission, and their shared frontiers. The result must bind
counterexamples to real owner-issued outcomes without moving runtime authority
into the model runner or certification courtroom.

## Why This Milestone Exists

Roadmap 2 already gives Store physical pages, integrity, recovery physics,
security scope, I/O scheduling, native blobs, and layout/access discipline.
That is enough power to build a real database and enough complexity to quietly
rebuild infrastructure folklore if the critical transitions remain "understood"
instead of modeled.

`S.8` is the last milestone before this one because Store now has real owner
outcomes for layout admission, access execution, LSM membership, physical
compaction, migration, rollback, corruption, and readmission. Those outcomes
replaced the old milestone-shaped `S8*` catalogs. `S.9` must model the current
owner boundaries, not resurrect the deleted catalogs under formal-method names.

This milestone therefore does not "add formal methods." It freezes the
smallest operational state machines whose failure would invalidate aerospace-
grade database claims even if the rest of the codebase looked polished:

- WAL/checkpoint/page flush ordering
- recovery source precedence under contradictory evidence
- compaction cutover and visibility
- physical reader leases and reclaim barriers
- quarantine and physical-evidence readmission transitions
- import or replication admission when physical evidence is the authority

The current baseline is uneven and the spec must say so:

- `worth-store-formal-models` already exists, but it is only a placeholder with
  `ModeledStateMachine` and a milestone-shaped README. This milestone replaces
  that placeholder through a hard cutover.
- Layout, LSM membership, and physical compaction already expose private-field
  owner outcomes and owner-issued observations. Their model bindings must reuse
  those surfaces.
- WAL, recovery physics, physical publication, integrity, and security already
  expose operation-specific receipts, outcomes, and denials. They must not be
  wrapped in one generic `TransitionReceipt` family.
- Certification already has layout owner coverage and a
  `LayoutFormalObservation`. Those are courtroom evidence and a descriptive
  projection. They are not the checked model, runtime law, or an authority to
  define owner cases.
- Import has custody readmission and restored-layout materialization, but not a
  transport state machine for partial byte receipt.
- Replication currently exposes only `ReplicationCapsuleId`. The production
  replication admission capability required by the roadmap must be built before
  its model can claim runtime conformance.
- Repair execution and operator authorization belong to `S.10`. This milestone
  models current quarantine, blast-radius preservation, verification, and
  readmission law; it must not invent operator authorization as Store authority.

## Governing Summaries

- `MENTALITY.md` protects adversarial-first infrastructure design. `S.9` must
  model the damaged, concurrent, operator-pressured system first rather than
  the clean protocol we hope is true.
- `arch_laws.md` protects proof-bearing phases, explicit orchestration, owner
  authority, and identity continuity. `S.9` must consume owner-issued outcomes
  and concrete Store authority witnesses. A model action, copied case id,
  declaration row, trace, or certification receipt must never open a production
  transition.
- `composition_laws.md` protects named semantic steps. `S.9` must split
  owner binding, observation mapping, model execution,
  counterexample interpretation, and certification handoff into distinct
  modules and phases instead of hiding them in one "formal verification"
  bucket.
- `domain_structure_laws.md` protects responsibility-shaped topology. Runtime
  transition ownership stays in each owner crate; checked semantics and
  abstraction mappings live in `worth-store-formal-models`; scenario assembly,
  mutants, and verdict adjudication live in certification.
- `perf_laws.md` protects cost honesty and bounded execution. Model checking is
  cold-path work. Runtime mutation and read paths may emit already-required
  owner outcomes and counters, but may not pay model-runner, trace-normalization,
  heap-materialization, or process-launch cost.
- `dx_laws.md` protects phase-legible APIs. A caller must be able to see the
  sequence `owner outcome -> non-authoritative observation -> modeled action ->
  checked verdict -> courtroom evidence` in code without guessing which object
  carries authority.
- `physical-database-roadmap.md` protects the physical database foundation gate.
  `S.9` exists because tests and certification lanes alone cannot exhaustively
  cover the crash-plus-concurrency transitions most likely to falsify Store's
  database claim.

## Adversarial Constraint

Under crash, restart, compaction, reclaim, corruption, quarantine, readmission,
import, and replication pressure, Store must never allow a lower-authority
artifact, copied observation, stale frontier, contradictory candidate, or
partially published external source to become current truth. Every in-scope
ordinary owner outcome must map to exactly one modeled action or an explicit
out-of-model denial, and a changed owner case set must fail conformance until the
model binding is updated.

## Product Decision Lock

- `S.9` is about physical protocol law, not general semantic correctness,
  planner semantics, or user-facing API behavior.
- Production owners define what can execute. Models define the legal abstract
  behavior of the named protocols. Certification evaluates evidence. None may
  impersonate another.
- A checked model without runtime mapping is not milestone credit.
- A runtime mapping without a checked model is not milestone credit.
- A proof artifact without orchestration, trace extraction, failure
  interpretation, and certification lanes is not milestone credit.
- `S.9` models production protocol states reached through ordinary facades, not
  certification-only toy states or catalog-only cases.
- Recovery, compaction, quarantine/readmission, and import models must admit
  degraded and quarantined outcomes explicitly; they may not collapse
  uncertainty into "empty," "ignored," or "best effort."
- Every production surface introduced or modified here must use domain
  vocabulary. `S<number>`, `s<number>_`, `phase_`, `milestone_`, and roadmap
  provenance are forbidden in production names. Milestone vocabulary is allowed
  in docs, test names, runner configuration, and release reports only.
- The public `worth-store` facade must not re-export the cold model runner as an
  ordinary database capability. The formal crate is an implementation and
  certification dependency, not a runtime authority facade.

## Current API Contract

Implementation must begin from this current surface inventory. Names marked
"required new" are capability work; all others exist at spec revision time.

- Durability and WAL:
  `AdmittedWalAppendReceipt`, `AdmittedCheckpointPublicationReceipt`,
  `PublicationDeclaration`, `WalReplayTailRecordReport`,
  `WalAppendPlan`, `WalAppendProgress`, `WalAppendReceipt`,
  `DurableAckReceipt`, `PageFlushRecoveryReceipt`, and
  `StoreDurabilityExecutionProof`.
- Recovery and precedence:
  `RecoveryCandidateDiscoveryTrace`, `RecoverySourceDecisionTrace`,
  `AdmittedRecoverySource`, `CheckpointBaseAdmission`, `WalTailRedoSource`,
  `RecoveryRedoPlan`, `RedoExecutionReceipt`, `RecoveryCompletion`,
  `RecoveryDeterminismReport`, and `ReopenedRecoveryArtifactAdmission`.
- Layout and LSM:
  `ObserveOwnerCase`, `OwnerCaseObservation`, family-specific layout outcomes,
  `LsmMembership*Outcome`, `LsmMembershipOwnerCaseObservation`,
  `LsmExecutionOwnerCaseObservation`, and
  `LsmMaintenanceOwnerCaseObservation`.
- Physical compaction and publication:
  `CompactionOwnerCaseObservation`, `CompactionMutationLaneReceipt`,
  `CompactionCutoverStabilityProof`, `CompactionRewritePublication`,
  `PhysicalPublicationReceipt`, `PublicationCrashRecoveryOutcome`, and
  `DrainedCompactionReclaim`.
- Lease, reclaim, and reuse:
  `PageLease` and `PinnedPageLease` describe local buffer residency;
  `HazardLeaseTable`, `ActiveHazardLease`, `ProtectedReferenceLease`,
  `DeferredReclaimReceipt`, `ReclaimEligibilityProof`,
  `CrashStableReclaimReuseFence`, and `GenerationAdvanceReceipt` own physical
  reclaim/reuse law.
- Integrity and quarantine:
  `ExecutedQuarantineFinding`, `QuarantineRecord`, `QuarantineReceipt`,
  `QuarantineHandoffPosture`, `RecoveryCorruptionReadmissionHandoff`, and
  `RecoveryLayoutReadmissionOutcomeView`.
- Import and trust-boundary readmission:
  `BackupImportCustodyReadmission`, `BackupExportCustodyAdmission`,
  `RestoredLayoutMaterializationOutcome`,
  `RestoredLayoutMaterializationObservation`,
  `StoreTrustBoundaryReadmissionTrigger`, and
  `StoreReadmittedSecurityScope`.
- Backend assumptions:
  `AdmittedBackendCapabilityWitness`, `BackendCapabilityClaimWitness`,
  `BackendDurabilityProfile`, `StoreDurabilityRequirement`,
  `StoreDurabilityExecutionProof`, `AccessPolicyExecutionReceipt`, and
  `BackendQueueExecutionCompletion`.
- Existing courtroom evidence:
  `LayoutOwnerExecutionEvidence`, `LayoutOwnerCoverageReceipt`, and
  `LayoutFormalObservation`. These may support certification comparisons but
  may not define model actions or production authority.
- Replication admission, required new:
  owner-issued source-epoch admission, lineage continuity, duplicate/resume,
  divergence, publication-readiness, and publication outcomes in
  `worth-store-replication`.

Any listed current API that proves insufficient must be changed at its owning
boundary. The implementer may not preserve a bad boundary and compensate with a
formal-model wrapper.

## Formal Modeling Contract

`S.9` must choose one primary checked-model toolchain and treat every other
artifact as support around that toolchain. The default posture is:

- primary protocol models: `TLA+` with checked `.tla` and `.cfg` artifacts
- executable algorithm sketches when useful: `PlusCal` lowered into checked
  `TLA+`
- bounded model execution in CI and local certification lanes through a pinned
  model-checking runner and checked command set
- optional secondary analysis tools are allowed only when they consume the same
  protocol vocabulary and do not replace the primary checked model

Every modeled protocol family must ship:

- a named model file
- a named configuration file
- declared constants and bound parameters
- declared initial states
- declared actions
- declared safety invariants
- declared liveness/fairness assumptions when any claim depends on them
- declared finite-state abstraction rules explaining what runtime detail has
  been collapsed and why the collapse is honest
- a reproducible check command used by CI and release certification

`S.9` does not get credit for prose about models, Rust files named
`states.rs/actions.rs/invariants.rs`, or manually inspected counterexample
output. The checked artifact itself is part of the milestone surface.

## Runtime Refinement Contract

The conformance problem is not "can a trace be replayed through a model." It is
"can an in-scope owner operation return an outcome that the model binding does
not classify, or can the model advertise an action that no ordinary owner
operation can reach."

Every modeled protocol family must therefore define:

- an opaque owner-issued outcome or durable family-specific receipt
- an owner-issued, read-only observation when cold consumers need one
- an independent exhaustive abstraction function from the specific owner
  observation or receipt to one modeled action
- exact-set coverage between owner-declared cases, ordinary executed cases, and
  mapped model actions
- an explicit classification for durable evidence, reopened observation,
  ephemeral diagnostics, and forbidden authority substitutes
- omission classification rules distinguishing:
  - impossible because no transition occurred
  - instrumentation defect
  - crash-loss of a non-authoritative diagnostic
  - illegal protocol hole

Every protocol family must classify each input into exactly one evidence class:

- `DurableAuthoritativeReceipt`
  The receipt is a physical artifact or directly bound physical witness whose
  survival across crash is part of the protocol claim.
- `ReopenedObservedReceipt`
  The receipt is reconstructed after reopen from authoritative physical state
  and is valid only because the phase defines the reconstruction law.
- `EphemeralDiagnosticTrace`
  The trace is useful for runner support, debugging, or certification
  localization, but its loss may not change protocol truth.
- `ForbiddenAuthoritySubstitute`
  The artifact may appear useful for diagnostics or convenience, but may never
  satisfy authority, legality, or conformance claims.

An owner observation proves only which owner case was reached. It cannot satisfy
admission, planning, execution, readmission, publication, reclaim, or repair
APIs. Certification must bind observations to ordinary owner execution evidence
before using them as conformance evidence.

## WORTH Proof And Foundational Contract

- Use `worth_proof::TransitionOutcome` for new multi-posture production
  transitions, including the replication admission capability. Preserve
  success, denial, deferred, stale, rebind-required, and failed postures when
  they are semantically real.
- `worth_proof::AuthorityWitness` may back owner issuance internally, but public
  governed APIs require concrete Store witness or outcome types. Public generic
  `AuthorityMarker` bounds are forbidden.
- Use `worth_store_aspect_native::StoreExecutedBoundaryReceiptEvidence` for
  executed boundary evidence that is actually backed by a
  `FoundationalBoundaryEvidenceExecutedReceiptArtifact` and a physical boundary
  witness.
- Use `StoreCompletedBoundaryReceiptEvidence` only for completed outcomes; a
  completed receipt must not substitute for executed authority.
- Use `StoreDiagnosticSupportReportEvidence` or
  `StoreDiagnosticExplanationBundleEvidence` for model verdicts,
  counterexamples, and localization reports. These remain diagnostic evidence.
- Use `FoundationalCounterBackedPerformanceReceipt` through existing Store
  wrappers for runtime cost claims. Model-state exploration counters are cold
  runner counters and must not be presented as runtime performance authority.
- Use Foundational authority, projection, receipt, and diagnostic roles rather
  than JSON or serde-shaped protocol values. Deserialization creates raw
  declarations only and requires explicit readmission before any witness can be
  formed.
- Do not create a second proof grammar inside `worth-store-formal-models`.
  Model states and actions are domain-specific finite abstractions; proof
  progression and evidence roles remain WORTH Proof and Foundational concepts.

## Backend And Hardware Assumption Contract

Every checked protocol family must declare the backend and hardware assumptions
required for its safety claim. At minimum this includes:

- write completion semantics
- fsync or equivalent durability fence semantics
- rename/publication semantics
- torn-write posture
- sector/page atomicity posture
- buffered versus direct-I/O posture
- checksum coverage assumptions
- clock or ordering assumptions when any timeout/lease claim depends on them

The model must parameterize these assumptions where the roadmap allows backend
variation. `S.9` may not quietly encode a perfectly atomic storage device and
claim the result as general Store protocol law.

## Counter Contract

`S.9` must publish exact counters for the model and the runtime conformance
boundary. At minimum:

- owner cases declared, executed, and mapped per family
- typed outcome postures observed per family
- runtime observations rejected from mapping
- owner cases missing from or duplicated in model mapping
- normalization rejections
- model states explored
- model transitions explored
- invariant checks executed
- deadlocks found
- truncated searches or bound exhaustion outcomes
- counterexamples produced
- counterexamples localized back to owner cases and evidence identities
- unsupported backend/hardware assumption mismatches

Runtime counters must attach to the owner transition that produced them. Cold
runner counters must attach to the exact model-check invocation, model digest,
configuration digest, backend-assumption profile, and finite bounds.

## DX Target

The target shape is a visible progression. Exact family names may differ, but
the authority direction may not:

```rust
let outcome = compaction_runtime.execute_cutover(request);
let observation = outcome.owner_case_observation();

// Cold projection: observation is not production authority.
let action = compaction_model_binding().map(observation)?;
let check = checked_protocols().check(action, backend_assumptions)?;

// Courtroom combines executed owner evidence with a checked diagnostic verdict.
let evidence = protocol_courtroom().adjudicate(owner_execution, check)?;
```

The following must not compile:

```rust,compile_fail
let action = ModeledAction::CompactionPublished;
publish_compaction(action);
```

The model layer may explain and falsify owner behavior. It may never authorize
owner behavior.

## S.4.5 Harness Integration Contract

The checked models and the physical simulation harness must falsify each other.
They remain different tools with a typed bridge:

- `physical_scenario` and `PhysicalScenarioBuilder` author executable Store
  scenarios through production facades.
- `PhysicalInterleavingSchedule`, `ReplaySeed`, `StateSpaceBudget`,
  `PartialOrderReductionPosture`, and `ScheduleShrinkTrace` own concrete
  schedule exploration, deterministic replay, and shrinking.
- `CrashEvent`, `DroppedFlushEvent`, `ReorderedPersistenceEvent`,
  `TornWriteEvent`, `ByteCorruptionEvent`, `BlockedReclaimEvent`, and
  `StaleGenerationEvent` inject faults at declared production yieldpoints.
- `PhysicalSimulationTranscript`, `ObservedPhysicalTrace`, and
  `PhysicalSimulationBoundaryObservation` carry executed observations. They do
  not become production authority or model semantics.
- `CrashRecoversOldOrNewNeverMixedOracle`, `NoMixedRootOracle`,
  `BlockedReclaimUntilReleaseOracle`, `IndependentVerifierAgreementOracle`, and
  new family-specific oracles judge concrete outcomes independently of the
  abstraction mapping.
- `GeneratedCoverageMatrix`, `PhysicalMutationCoverageEvidence`, and exact
  counter contracts record scenario, transition, mutant, oracle, and transcript
  coverage.

Required cross-amplification:

- Every checked counterexample that is executable at the current abstraction
  boundary must lower into a deterministic physical scenario and replay seed.
- Every minimized hostile harness trace for an in-scope protocol must map into
  a checked model action sequence or produce a typed mapping-gap verdict.
- Model checking owns exhaustive finite-state abstraction. The harness owns real
  Rust execution, backend behavior, crash seams, byte corruption, memory/I/O
  pressure, and independent observations. Neither substitutes for the other.
- Model-generated scenarios enter the harness as plans. They receive no fixture,
  schedule, driver, or verdict authority merely because the model produced
  them.

Harness adversarial gates:

- Counterexample round-trip test: model counterexample -> executable scenario ->
  deterministic transcript -> mapped action sequence preserves the illegal
  edge and owner identity.
- Mapping-gap test: a new owner outcome absent from the model fails the coverage
  matrix and cannot be normalized into a known action.
- Oracle-independence mutant: a shared defect in the abstraction mapping and
  model still fails an independent concrete oracle.
- Shrink-preservation test: schedule shrinking retains the failing owner edge,
  crash seam, backend assumptions, and evidence identity.

## Phase Plan

### Phase 1: Owner Boundary Ledger And Capability Gap Freeze

This phase records the exact production boundary for every modeled family. It
does not add a generic protocol registry to runtime crates. The ledger is a cold
formal-model binding manifest that points at concrete owner outcomes,
observations, durable receipts, denials, and current capability gaps.

**Relevant subsystems**
- all current API owners listed in the Current API Contract
- `worth-store-formal-models`
- `worth-store-certification::courtroom::layout::owner_coverage`

**Relevant APIs**
- existing owner inventories such as `lsm_membership_owner_case_inventory`,
  `lsm_execution_owner_case_inventory`, `lsm_maintenance_owner_case_inventory`,
  and `compaction_owner_case_inventory`
- existing `ObserveOwnerCase` implementations and operation-specific receipts
- required new cold `protocol_bindings::ProtocolBindingManifest`
- required new cold `protocol_bindings::OwnerBoundaryGap`

**Warnings**
- `LayoutOwnerFamily`, `LayoutOwnerCaseDeclarations`, and
  `LayoutFormalObservation` are certification-owned coverage projections. They
  may be compared against this ledger, but cannot define production owner cases.
- Do not create `*ProtocolInventoryRow` in every runtime crate. Owner outcomes
  and owner case declarations are already the runtime inventory where they
  exist.
- A family with no honest production outcome is a capability gap, not a reason
  to model a fictional transition.

**Test requirements**
- Exact owner binding test: every bound owner case is declared by its production
  owner and every declared in-scope case appears exactly once in the binding
  manifest.
- Courtroom forgery test: adding a case only to `LayoutOwnerFamily` or a
  certification transcript does not make it a valid protocol binding.
- Capability-gap honesty test: replication remains unmodelable until the
  required production outcomes exist; a placeholder id cannot satisfy the
  binding manifest.

**Engineering decisions**
- Each binding names one production owner, one operation family, the concrete
  outcome/receipt source, the observation class, the model action family, and
  its crash-survival posture.
- The manifest records missing runtime capabilities explicitly and blocks the
  dependent model phase.
- The manifest is not production authority and is not exported from the public
  Store facade.

**Open questions**
- Which WAL and recovery operations need a small owner-issued observation
  adapter because their existing durable receipt is intentionally too detailed
  for the finite model?

### Phase 2: Formal Model Crate Cutover, Toolchain, And Topology

This phase replaces the existing placeholder crate, pins the checked toolchain,
and freezes a responsibility-shaped topology before protocol implementation.
The checked artifact lives with its mapping so the code-to-model relationship
is visible without duplicating TLA+ states as decorative Rust enums.

**Relevant subsystems**
- `worth-store-formal-models`
- workspace dependency configuration
- `worth-store-certification`
- boundary-check configuration

**Relevant APIs**
- remove placeholder `ModeledStateMachine`
- required new `protocol_bindings`, `assumptions`, `runner`, and `protocols`
  facades
- `worth_proof::TransitionOutcome`
- Store aspect-native evidence wrappers named in the Foundational contract

**Warnings**
- The formal-model crate already exists. Do not describe it as new and do not
  preserve its milestone-shaped placeholder API for compatibility.
- No runtime owner crate may depend on `worth-store-formal-models` or
  `worth-store-certification`.
- Remove the currently unused `worth-store-formal-models` dependency from the
  thin public `worth-store` facade. Certification consumes the formal crate;
  ordinary Store users do not.
- `mod.rs` files route ownership only. Checked-model invocation, mapping, and
  counterexample logic belong in named responsibility files.
- The formal crate may depend on public owner facades, never private source-tree
  topology.

Dependency direction is fixed:

```text
worth-store-certification -> worth-store-formal-models -> runtime owner facades
worth-store-certification -> worth-store-physical-certification -> runtime owner facades
```

There is no dependency from a runtime owner, the thin `worth-store` facade, or
`worth-store-physical-certification` back into the formal-model crate.

**Test requirements**
- Dependency direction test: runtime owners cannot import the formal-model or
  certification crates; the formal crate cannot deep-import private owner
  modules.
- Public-facade dependency test: `worth-store` does not depend on or re-export
  the model runner, checked artifacts, or counterexample machinery.
- Placeholder residue test: `ModeledStateMachine`, Roadmap/S.9 production
  names, and milestone-shaped README claims are absent after cutover.
- Toolchain reproducibility test: a clean checkout can run the pinned checker
  with the committed command and verify the checker digest/version.

**Engineering decisions**
- Required target skeleton:

```text
worth-store-formal-models/
  src/
    lib.rs
    protocol_bindings/
      mod.rs
      manifest.rs
      evidence_class.rs
      completeness.rs
      capability_gap.rs
    assumptions/
      mod.rs
      backend.rs
      atomicity.rs
      clock.rs
    runner/
      mod.rs
      invocation.rs
      verdict.rs
      bounds.rs
      counterexample.rs
      localization.rs
    protocols/
      durability_recovery/
      source_precedence/
      compaction_visibility/
      lease_reclaim/
      quarantine_readmission/
      import_publication/
      replication_admission/
      shared_frontiers/
  tests/
    binding_completeness.rs
    runner_contract.rs
    backend_assumptions.rs
```

- Each protocol directory owns `mod.rs`, `mapping.rs`, `model.tla`, and
  `model.cfg`, plus only the additional named files its responsibility needs.
- Do not create parallel Rust `states.rs`, `actions.rs`, and `invariants.rs`
  merely to mirror the checked model.
- Certification adds a domain-named `courtroom/protocol_models/` tree with
  `scenarios`, `mutants`, `adjudication`, and `evidence`; no production path is
  named after S.9 or a phase number.

**Open questions**
- The primary checker remains TLA+/TLC unless a documented toolchain spike
  proves another checked engine integrates more honestly with Rust owner
  bindings and CI.

### Phase 3: Owner Observation And Refinement Completeness

This phase builds only the observation adapters genuinely missing from current
owners, then proves exact correspondence between production cases and modeled
actions. It is the reusable conformance substrate for all later families.

**Relevant subsystems**
- runtime owner crates with in-scope outcomes
- `worth-store-formal-models::protocol_bindings`
- `worth-store-certification::courtroom::protocol_models`

**Relevant APIs**
- `ObserveOwnerCase`, `OwnerCaseObservation`
- `LsmMembershipOwnerCaseObservation`
- `CompactionOwnerCaseObservation`
- `RestoredLayoutMaterializationObservation`
- operation-specific durable receipts from WAL, recovery, publication,
  quarantine, and reclaim
- required new family-specific mapping traits/functions inside the formal crate

**Warnings**
- Do not add a generic crate-wide `TransitionReceipt` or a constructor that can
  pair arbitrary payloads with case ids.
- An observation is a projection. It is accepted for model mapping, but it
  cannot satisfy production authority or certification conformance without
  bound executed-owner evidence.
- Mapping must be exhaustive over concrete owner case types. String matching on
  case names is not conformance.

**Test requirements**
- Three-set equality test: owner-declared cases, cases reached through ordinary
  production scenarios, and mapped model actions are exactly equal per family.
- Mapping independence mutant: changing a mapping edge while leaving owner
  execution intact fails the checked invariant or correspondence oracle.
- Compile-fail authority test: model actions, observations, copied case ids, and
  binding manifests cannot satisfy production admission or execution APIs.
- Crash-loss classification test: durable receipts, reopened observations, and
  ephemeral diagnostics produce distinct omission verdicts.

**Engineering decisions**
- Runtime owners issue outcomes and, only when necessary, minimal read-only
  observations.
- Formal-model code owns abstraction functions and model action vocabulary.
- Certification owns executed-case coverage and the comparison between ordinary
  execution, mappings, and checked verdicts.

**Open questions**
- Which existing outcome families need borrowed mapping to preserve opaque
  payloads, and which durable receipt families require canonical identity-only
  projection before model checking?

### Phase 4: End-To-End Durability And Recovery Frontier Model

This phase models the shared durability frontier that ties together WAL append,
page flush, checkpoint cutover, recovery source precedence, redo, root
publication, and repeated reopen. It is the first checked end-to-end reference
protocol because the later families depend on the same durability truth.

**Relevant subsystems**
- `worth-store-recovery-physics`
- `worth-store-wal`
- `worth-store-buffer-pool`
- `worth-store-physical-backend`
- `worth-store-offline-verifier`
- `worth-store-formal-models`

**Relevant APIs**
- `worth_store_recovery_physics::WalAppendPlan`
- `worth_store_recovery_physics::WalAppendProgress`
- `worth_store_recovery_physics::WalAppendReceipt`
- `worth_store_recovery_physics::DurableAckReceipt`
- `worth_store_recovery_physics::PageFlushRecoveryReceipt`
- `worth_store_recovery_physics::CheckpointPublicationPlan`
- `worth_store_recovery_physics::CheckpointCutoverReceipt`
- `worth_store_recovery_physics::RecoverySourceDecisionTrace`
- `worth_store_recovery_physics::RecoveryDeterminismReport`
- `worth_store_physical_backend::StoreDurabilityExecutionProof`
- `worth_store_physical_backend::StoreDurabilityCounterSnapshot`
- required new `worth_store_formal_models::protocols::durability_recovery`

**Warnings**
- This model must include ugly states, not only durable steady states.
- Recovery source precedence is one phase inside the frontier model, not the
  whole frontier.
- Offline observations may advise comparison lanes, but runtime authority still
  comes from admitted artifacts and receipts.
- "flush requested" and "flush durably completed" are different states.
- Backend acceptance, file sync, directory sync, rename, and ordering-barrier
  completion are different states. A boolean "durable" abstraction is too weak.
- "checkpoint begun", "checkpoint durable", "checkpoint published", and
  "checkpoint selected for recovery" are different states.
- "redo considered", "redo applied", and "redo skipped as stale/idempotent" are
  different states.

**Test requirements**
- Crash seam frontier test: crashing at every named append/flush/checkpoint/
  recovery/root-publication seam yields only modeled frontier states and
  deterministic reopen classification.
- Contradictory evidence test: checkpoint, WAL, pageLSN, compaction residue,
  and quarantine observations preserve contradiction until an admitted
  precedence rule resolves them or rejects them.
- Lost/reordered barrier mutant test: a backend profile that loses or reorders a
  required durability barrier produces a counterexample before legal
  acknowledgment or root publication.

**Engineering decisions**
- This phase owns:
  - append submitted/completed/acknowledged
  - dirty page publication submitted/completed
  - directory sync submitted/completed/failed
  - checkpoint begin/cutover/root publication
  - recovery candidate discovery
  - precedence selection
  - redo considered/applied/skipped/denied
  - reopened recovered-root publication
- Durability frontier states must include at minimum:
  - WAL append proposed
  - WAL append completed in process memory
  - WAL durability fence requested
  - WAL durability fence completed
  - WAL acknowledgment legal
  - dirty page flush requested
  - dirty page flush completed
  - dirty page flush durability uncertain
  - checkpoint begun
  - checkpoint durable
  - checkpoint published
  - checkpoint selected
  - recovery replay required
  - recovery replay applied
  - recovery replay skipped as idempotent
  - recovered root publication pending
  - recovered root publication completed
- Legal transition guards must include:
  - page publication may not advance to durable visibility without the required
    WAL durability witness
  - checkpoint publication may not advance without the required root and
    frontier durability witnesses
  - redo may not apply without admitted target generation and precedence
    selection
  - recovered-root publication may not advance without replay completion or
    explicit replay-skip legality
- Required denials must include:
  - ambiguous WAL durability
  - page flush ahead of WAL legality
  - checkpoint publication without durable frontier
  - root publication without selected recovery basis
  - redo target generation mismatch
  - failed directory sync treated as durable publication
- Determinism is part of the protocol contract, not only a certification note.

**Open questions**
- Which frontier states must be parameterized by backend durability class?

### Phase 5: Recovery Source Precedence Submodel

This phase takes the recovery-source slice out of the durability frontier and
forces it into its own fully enumerated submodel so the code cannot hide
precedence folklore behind a clean "recovery succeeded" story.

**Relevant subsystems**
- `worth-store-recovery-physics`
- `worth-store-offline-verifier`
- `worth-store-formal-models`

**Relevant APIs**
- `worth_store_recovery_physics::AdmittedRecoverySource`
- `worth_store_recovery_physics::RecoverySourceDecisionTrace`
- `worth_store_recovery_physics::RecoveryCandidateDiscoveryTrace`
- `worth_store_recovery_physics::CheckpointBaseAdmission`
- `worth_store_recovery_physics::WalTailRedoSource`
- `worth_store_recovery_physics::PhysicalRecoverySource`
- `worth_store_recovery_physics::RecoveryRedoPlan`
- `worth_store_recovery_physics::RedoExecutionReceipt`
- `worth_store_recovery_physics::ReopenedRecoveryArtifactAdmission`
- required new `worth_store_formal_models::protocols::source_precedence`

**Warnings**
- The small `PhysicalRecoverySource` summary enum must not become the real
  model vocabulary.
- Losing candidates must stay visible in the model and in runtime receipts.
- The model must distinguish "advisory only," "authority candidate,"
  "quarantine-only," and "forbidden convenience" sources explicitly.

**Test requirements**
- Contradictory precedence test: checkpoint, WAL tail, page image, compaction
  residue, quarantine, and offline observations preserve contradiction until a
  legal precedence rule resolves or rejects them.
- False-authority mutant test: a derived replay/index/locator source accepted
  as authoritative must fail.
- Deterministic reopen test: repeated reopen from the same bytes and backend
  profile produces the same precedence outcome.

**Engineering decisions**
- Precedence states must include:
  - candidate discovered
  - candidate admitted
  - candidate advisory only
  - candidate rejected
  - contradiction preserved
  - source selected
  - source quarantined
  - source denied
- Precedence transitions must name why one source loses, not only why another
  wins.
- Legal transition guards must include:
  - a candidate may not enter `admitted` without an authority-class-specific
    admission witness
  - a losing candidate may not disappear from the decision trace unless the
    phase declares it irrelevant by rule
  - a quarantined candidate may not transition to selected authority without an
    explicit repair/readmission witness owned by a later protocol
- Required denials must include:
  - derived locator accepted as authority
  - replay helper accepted as authority
  - contradictory candidates collapsed into one synthetic source
  - quarantined source silently ignored as empty absence

**Open questions**
- Which precedence decisions need first-class replay receipts versus derivable
  decision rows?

### Phase 6: Compaction Cutover And Visibility Submodel

This phase isolates compaction cutover, visibility, rollback, and orphaned
product handling as its own explicit submodel. The purpose is to stop "the new
run became visible somehow" from remaining an implementation story.

**Relevant subsystems**
- `worth-store-recovery-physics`
- `worth-store-layout-indexes`
- `worth-store-lsm-authority`
- `worth-store-physical-isolation`
- `worth-store-io-scheduler`
- `worth-store-formal-models`

**Relevant APIs**
- `worth_store_recovery_physics::AdmittedCompactionCutoverRecord`
- `worth_store_recovery_physics::CompactionGenerationVisibility`
- `worth_store_recovery_physics::CompactionVisibleProductEvidence`
- `worth_store_lsm_authority::LsmMembershipReplacementOutcome`
- `worth_store_lsm_authority::LsmMembershipPersistOutcome`
- `worth_store_lsm_authority::LsmPublishedMembershipLookupOutcome`
- `worth_store_layout_indexes::LsmCompactionPreparationOutcome`
- `worth_store_layout_indexes::LsmPhysicalCompactionBindingOutcome`
- `worth_store_layout_indexes::LsmCompactionPublicationOutcome`
- `worth_store_physical_isolation::CompactionMutationLaneReceipt`
- `worth_store_physical_isolation::CompactionCutoverStabilityProof`
- `worth_store_physical_isolation::CompactionRewritePublication`
- `worth_store_physical_isolation::DeferredReclaimReceipt`
- required new `worth_store_formal_models::protocols::compaction_visibility`

**Warnings**
- The model must include orphaned products, rollback, cancellation, retry, and
  partial publication, not just clean cutover.
- Visibility and reclaimability are different states.
- Tombstone/version retention is a protocol rule here, not an implementation
  detail inside compaction code.

**Test requirements**
- Crash-during-cutover test: every seam from durable new product through old
  generation reclaim eligibility reopens into a modeled visibility state.
- Tombstone-loss mutant test: a visible product missing required tombstones or
  version retention must fail.
- Orphan-product cleanup test: a durable unpublished product after crash
  becomes either a modeled retry candidate, rollback candidate, or quarantine
  candidate rather than ambient residue.

**Engineering decisions**
- States must include:
  - compaction planned
  - product writing
  - product durable
  - publish attempted
  - visible alongside old generation
  - old generation retained
  - old generation reclaim-eligible
  - product orphaned
  - cutover rolled back
  - retry required
- The submodel must emit explicit visibility-frontier receipts and old-
  generation-retention observations derived from the existing owner-issued
  publication, stability, and reclaim outcomes. It must not invent a parallel
  receipt lane.
- Legal transition guards must include:
  - `product durable -> publish attempted` requires compaction product integrity
    and publication witness
  - `publish attempted -> visible alongside old generation` requires an explicit
    visibility frontier receipt
  - `old generation retained -> old generation reclaim-eligible` requires both
    visibility and lease/reclaim barrier satisfaction
- Required denials must include:
  - visible product missing tombstone or version-retention obligations
  - orphaned product treated as ambient retry success
  - rollback source chosen from derived compaction outputs
  - reclaimability inferred from scheduler idleness

**Open questions**
- Which compaction states need a direct shared-frontier representation versus a
  local-only representation?

### Phase 7: Lease, Reclaim, And Reuse Barrier Submodel

This phase isolates the physical lease, reclaim, and generation-reuse barrier
rules as their own submodel. The main job is to make "safe to stop pinning" and
"safe to reclaim or reuse" impossible to confuse.

**Relevant subsystems**
- `worth-store-buffer-pool`
- `worth-store-physical-isolation`
- `worth-store-formal-models`

**Relevant APIs**
- `worth_store_buffer_pool::PageLease`
- `worth_store_buffer_pool::PinnedPageLease`
- `worth_store_buffer_pool::LeaseEpoch`
- `worth_store_physical_isolation::HazardLeaseTable`
- `worth_store_physical_isolation::ActiveHazardLease`
- `worth_store_physical_isolation::ProtectedReferenceLease`
- `worth_store_physical_isolation::HazardLeaseReleaseReceipt`
- `worth_store_physical_isolation::DeferredReclaimReceipt`
- `worth_store_physical_isolation::ReclaimEligibilityProof`
- `worth_store_physical_isolation::CrashStableReclaimReuseFence`
- `worth_store_physical_isolation::GenerationAdvanceReceipt`
- required new `worth_store_formal_models::protocols::lease_reclaim`

**Warnings**
- A lease local to the buffer pool is not enough. `S.9` cares about reclaim and
  reuse meaning across compaction, recovery, and quarantine.
- Buffer-pool page leases and physical hazard leases are distinct protocols.
  The model may relate them, but it may not treat a local pin as the durable
  reclaim authority.
- Revocation, expiry, crash-loss, and leaked-holder states must be explicit.

**Test requirements**
- Lease-vs-reclaim race test: a surviving lease blocks reclaim or reuse until a
  legal barrier release occurs.
- ABA-reuse mutant test: generation reuse before durable barrier advance must
  fail.
- Leaked-holder/reopen test: crash or process loss does not silently convert an
  active hazard into reuse permission; the configured reopen law must resolve
  or retain the barrier explicitly.

**Engineering decisions**
- States must include:
  - lease requested
  - lease granted
  - lease pinned
  - lease revocation requested
  - lease expired
  - holder crashed
  - reclaim blocked
  - reclaim admitted
  - generation reuse admitted
- Reclaimability and reusability remain distinct until the model proves they
  can converge under the current backend profile.
- Legal transition guards must include:
  - `lease granted -> lease pinned` requires the correct generation and scope
    witness
  - `reclaim blocked -> reclaim admitted` requires all relevant lease classes
    to have exited or been legally revoked
  - `reclaim admitted -> generation reuse admitted` requires the durable
    frontier advance that prevents ABA reuse
- Required denials must include:
  - lease forgotten because no pages are currently pinned
  - generation reuse admitted before barrier completion
  - crashed holder treated as safely revoked without modeled revocation law

**Open questions**
- Which lease classes require durable receipts versus reconstructed post-reopen
  observations?

### Phase 8: Quarantine, Blast-Radius Preservation, And Readmission Model

This phase models the damaged-truth boundary that exists today. It covers
quarantine entry, scope preservation across offline observation, verification,
and explicit readmission. It does not model an operator authorization or full
repair-execution system that belongs to `S.10`.

**Relevant subsystems**
- `worth-store-physical-integrity`
- `worth-store-operations`
- `worth-store-offline-verifier`
- `worth-store-recovery-physics`
- `worth-store-security`
- `worth-store-formal-models`

**Relevant APIs**
- `ExecutedQuarantineFinding`, `QuarantineRecord`, `QuarantineReceipt`
- `QuarantineHandoffPosture`, `RepairBlastRadiusReadiness`
- `RepairQuarantineScopePreservation`
- `OfflineRepairBlastRadiusObservation`
- `RecoveryCorruptionReadmissionHandoff`
- `RecoveryLayoutReadmissionOutcomeView`
- `StoreRepairPhysicalRegionWitness`
- required new `protocols::quarantine_readmission`

**Warnings**
- Repair blast-radius readiness proves where repair may physically observe or
  read. It does not prove who may initiate repair.
- Bind the model to domain-named readiness and quarantine APIs, not the existing
  milestone-shaped `S10RepairBlastRadiusHandoff` compatibility surface. If that
  surface must be modified, rename it at its owner boundary rather than
  propagating the name.
- JWT subjects, application org ids, KMS key ids, IAM roles, operator identity,
  and repair audit records are declarations or diagnostics, never Store tenant,
  key, custody, or repair authority.
- Offline observations remain observations until current runtime authority
  readmits them.

**Test requirements**
- Scope-preservation test: region, tenant scope, key generation, authenticity,
  and custody posture survive runtime-to-offline-to-runtime comparison without
  widening.
- Observation-forgery test: copied offline findings and operator claims cannot
  release quarantine or construct readmission authority.
- Verification-frontier mutant: a transition that releases quarantine before
  successful current-scope verification fails the model.

**Engineering decisions**
- States include quarantine required, quarantine sealed, offline observation,
  readmission requested, readmission denied, verification required,
  quarantine retained, quarantine released, and unrecoverable.
- Legal release requires a current-scope readmission witness and successful
  verification over the same physical region and evidence identity.
- Interrupted repair execution is not modeled until `S.10` provides an honest
  production owner outcome. This model exposes the quarantine/readmission
  contract that future repair execution must satisfy.

**Open questions**
- Does `S.10` need a future refinement relation from repair execution outcomes
  into this model, or should it compose a sibling repair workflow model?

### Phase 9: Import Readmission, Publication Capability, And Model

This phase models the authority boundary that current import code actually
owns: trust-boundary custody/security readmission, recovered-layout
materialization, and publication eligibility. Byte transfer, resume, and
fragment transport are outside this model until a production owner exists.

**Relevant subsystems**
- `worth-store-operations`
- `worth-store-offline-verifier`
- `worth-store-recovery-physics`
- `worth-store-security`
- `worth-store-layout-indexes`
- `worth-store-formal-models`

**Relevant APIs**
- `BackupImportCustodyReadmission`
- `BackupExportCustodyAdmission`
- `RestoredLayoutMaterializationOutcome`
- `RestoredLayoutMaterializationObservation`
- `ReopenedRecoveryArtifactAdmission`
- `StoreTrustBoundaryReadmissionTrigger`
- `StoreReadmittedSecurityScope`
- `worth_store_physical_isolation::PhysicalRootPublicationRuntime`
- `worth_store_physical_isolation::PhysicalPublicationReceipt`
- required new domain-named import publication readiness and outcome in
  `worth-store-operations`, bound to the physical publication owner
- required new `protocols::import_publication`

**Warnings**
- A trust boundary includes a different deployment, Store instance, key-scope
  generation, tenant-scope authority, custody domain, offline export/import, or
  backup restoration after key rotation.
- Deserialization and terminal JSON can create raw declarations only. They are
  never admission authority.
- Authenticity unavailable, authenticity failed, custody unavailable, custody
  mismatch, and stale key generation remain distinct denials.

**Test requirements**
- Cross-boundary substitution test: a valid capsule from another deployment,
  instance, tenant authority, or key generation cannot become current through
  copied metadata.
- Crash-before-publication test: readmitted and materialized evidence that has
  not crossed the durable publication frontier does not become ambient current
  authority after reopen.
- Serde authority compile-fail test: deserialized security values cannot satisfy
  `StoreReadmittedSecurityScope` or restored materialization admission.
- Publication-binding test: restored materialization plus copied security or
  root fields cannot construct import publication readiness; the ordinary path
  must retain the actual physical publication receipt.

**Engineering decisions**
- States include raw declaration, offline observation, current-scope
  readmission requested, readmission denied, recovered artifact admitted,
  layout materialization admitted/denied, publication pending, publication
  durable, and publication denied.
- `worth-store-operations` orchestrates import publication, while
  `PhysicalRootPublicationRuntime` remains the physical publication owner.
- Publication completion composes the current durability frontier and the same
  security/readmission identity; it cannot be inferred from materialization or
  minted inside the formal model.
- Transport progress is not invented here. A future import transport owner may
  refine into the raw-declaration state without changing admission authority.

**Open questions**
- What is the smallest adapter from admitted restored materialization into the
  existing physical publication intent that preserves root, generation,
  security, and recovery identities without duplicating publication law?

### Phase 10: Replication Admission Production Capability

The roadmap requires replication admission modeling, but the production crate
currently exposes only `ReplicationCapsuleId`. This phase builds the minimum
real owner state machine before any checked model is attempted.

**Relevant subsystems**
- `worth-store-replication`
- `worth-store-authority`
- `worth-store-security`
- `worth-store-recovery-physics`
- `worth-store-wal`
- `worth-store-physical-backend`

**Relevant APIs**
- existing `ReplicationCapsuleId`
- existing `StoreCurrentAuthorityWitness`
- existing Store trust-boundary and security-scope readmission APIs
- existing durable publication and replay identities
- required new domain-named source epoch, lineage continuity, peer progress,
  duplicate/resume, divergence, publication-readiness, and publication outcomes

**Warnings**
- This is production capability work, not model vocabulary. The outcome types
  live in `worth-store-replication` and are useful without S.9.
- Do not call peer identity, transport success, or decodable bytes authority.
- Do not reuse import outcomes for replication. Shared lower evidence is fine;
  persistent peer progress and divergence are different semantics.

**Test requirements**
- Hostile peer test: copied source epoch, mismatched lineage, or divergent peer
  progress produces typed denial and no publication readiness.
- Duplicate/resume test: duplicate delivery is idempotent and cannot count as
  fresh admitted progress; a legal resume preserves source and lineage identity.
- Compile-fail authority test: callers cannot construct admitted peer source,
  publication readiness, or published replication outcome from raw ids.

**Engineering decisions**
- Use `TransitionOutcome` and private-field owner-issued outcomes.
- Preserve current Store authority, security scope, source epoch, lineage,
  replay identity, and durability publication identity through the outcome
  chain.
- Expose a sealed read-only observation for cold model binding, but no model or
  certification dependency.

**Open questions**
- Does replication publication reuse an existing durable publication owner or
  require a lower shared publication capability to avoid duplicating WAL and
  root-publication law?

### Phase 11: Replication Admission Model

This phase models the ordinary replication outcomes built in Phase 10. It is
blocked until every advertised case is reachable through production facades.

**Relevant subsystems**
- `worth-store-replication`
- `worth-store-formal-models`
- `worth-store-certification`

**Relevant APIs**
- Phase 10 owner outcomes and sealed observations
- required new `protocols::replication_admission`

**Warnings**
- The model cannot add a convenient state that the production owner does not
  expose.
- Replication and import may compose at current-scope readmission and durable
  publication frontiers, but their local action vocabularies remain distinct.

**Test requirements**
- Exact case/action test: every Phase 10 owner case maps exactly once and every
  modeled local action is reached by an ordinary production scenario.
- Divergence mutant test: treating source-epoch drift or lineage fork as legal
  resume produces a checked counterexample.
- Crash-before-publication test: peer bytes never become current authority
  without the same admitted durability frontier modeled in Phase 4.

**Engineering decisions**
- States include peer source admitted/denied, progress observed, duplicate
  observed, resume admitted/denied, divergence detected, publication pending,
  publication durable, and publication denied.
- Duplicate progress is explicitly idempotent and does not advance the admitted
  frontier.

**Open questions**
- Which peer liveness/fairness claims are required for eventual progress, and
  which safety checks remain valid without fairness?

### Phase 12: Shared Frontier Composition Model

This phase is where `S.9` earns the roadmap claim about crash plus concurrency.
It composes the local submodels around named shared frontiers so the combined
failure surfaces are checked instead of narrated.

**Relevant subsystems**
- `worth-store-formal-models`
- all runtime owner crates
- `worth-store-certification`

**Relevant APIs**
- required new `worth_store_formal_models::protocols::shared_frontiers`
- all family-local outcome, receipt, observation, and mapping surfaces from
  Phases 4-11
- required new `worth_store_formal_models::runner::CrossProtocolLocalization`

**Warnings**
- This phase must not flatten all local models into one unreadable giant
  machine.
- Shared evidence vocabulary is not enough. The model must own cross-family
  invariants explicitly.
- A passed local submodel does not imply a passed composed protocol.

**Test requirements**
- Crash-during-compaction-with-live-lease test: compaction cutover, retained old
  generation, surviving lease, crash, and reopen land in a legal shared
  frontier state.
- Checkpoint-vs-quarantine race test: checkpoint publication and quarantine
  requirement cannot make quarantined truth current authority.
- Quarantine-vs-reclaim race test: quarantined or verification-pending regions
  block reclaim and reuse until a legal frontier release.
- Import-or-replication-vs-publication race test: external evidence admission
  cannot publish new authority through a durability shortcut.

**Engineering decisions**
- Shared frontiers are:
  - durability frontier
  - visibility frontier
  - reachability frontier
  - quarantine frontier
  - admission frontier
- Cross-family invariants must include:
  - no reclaim of reachable authority
  - no publication of quarantined authority as current
  - no import/replication publication without durability-frontier admission
  - no quarantine release without verification-frontier advance
  - no compaction cutover that strands recovery without a legal precedence path
- Composition must be expressed as:
  - one named frontier model for shared state ownership
  - plus at least one race-focused composition model if one model would become
    unreadable
- Required composed races include:
  - compaction cutover vs surviving lease
  - checkpoint publication vs quarantine admission
  - quarantine/readmission vs reclaim/reuse
  - import publication vs crash durability window
  - replication publication vs divergence detection

#### Checked protocol state, transition, and denial tables

These tables are the phase-owned vocabulary mirrored by the executable Rust
models and the checked TLA+ specifications. A later milestone may extend a
family, but it must not silently rename a frontier, collapse a typed denial, or
treat a blocked edge as an ordinary transition.

##### Durability and recovery

| Kind | Explicit model vocabulary |
| --- | --- |
| States/frontiers | WAL: `Absent`, `Proposed`, `CompletedInMemory`, `FenceRequested`, `FenceCompleted`, `Acknowledged`; page: `Clean`, `FlushRequested`, `Durable`, `DurabilityUncertain`; checkpoint: `Absent`, `Begun`, `Durable`, `Published`, `Selected`; directory sync: `Absent`, `Completed`, `Failed`; replay: `Absent`, `Required`, `Applied`, `SkippedIdempotent`; recovered root: `Absent`, `Pending`, `Completed`; crash posture: open or crashed |
| Legal transitions/actions | `WalAppendProposed`, `WalAppendCompletedInMemory`, `WalFenceRequested`, `WalFenceCompleted`, `WalAcknowledgmentLegal`, `PageFlushRequested`, `PageFlushCompleted`, `PageFlushDurabilityUncertain`, `CheckpointBegun`, `CheckpointDurable`, `DirectorySyncCompleted`, `DirectorySyncFailed`, `CheckpointPublished`, `CheckpointSelected`, `RecoveryReplayRequired`, `RecoveryReplayApplied`, `RecoveryReplaySkippedIdempotent`, `RecoveredRootPublicationPending`, `RecoveredRootPublicationCompleted`, `Crash`, `Reopen` |
| Typed denials/blocked edges | `AmbiguousWalDurability`, `PageFlushAheadOfWal`, `CheckpointFrontierNotDurable`, `DirectorySyncNotDurable`, `RecoveryBasisNotSelected`, `ReplayNotResolved`, `RedoGenerationMismatch`, `IllegalTransition` |

##### Recovery-source precedence

| Kind | Explicit model vocabulary |
| --- | --- |
| States/frontiers | Candidate sets: `discovered`, `admitted`, `advisory`, `rejected`, `quarantined`, `selected`; contradiction posture: absent or preserved; authority postures: `AdmittedAuthority`, `AdvisoryOnly`, `DerivedLocator`, `ReplayHelper`, `Quarantined` |
| Legal transitions/actions | `CandidateDiscovered`, `CandidateAdmitted`, `CandidateAdvisoryOnly`, `CandidateRejected`, `ContradictionPreserved`, `SourceSelected`, `SourceQuarantined`, `SourceDenied` |
| Typed denials/blocked edges | `DerivedSourceCannotBeAuthority`, `QuarantinedSourceCannotBeSelected`, `CandidateNotAdmitted` |

##### Compaction visibility

| Kind | Explicit model vocabulary |
| --- | --- |
| States/frontiers | `Idle`, `Planned`, `WritingNewGeneration`, `NewGenerationDurable`, `PublicationAttempted`, `NewGenerationVisible`, `OrphanedNewGeneration`, `PublicationRolledBack`, `ReclaimEligible`; carried frontiers: old generation retained/released, tombstone preserved/resurrected, live readers, visible generation |
| Legal transitions/actions | `Plan`, `Write`, `Durable`, `AttemptPublish`, `Publish`, `CrashToOrphan`, `Rollback`, `Retry`, `ReleaseReader`, `Reclaim`; owner mappings: `LsmMembership`, `LsmExecution`, `LsmMaintenance`, `LowerRewrite`, `PublishRewrite`, `AdmitRecoveryVisibility`, `DeferReclaim`, `DrainReclaimAfterReadRelease` |
| Typed denials/blocked edges | Lifecycle: `PublicationBeforeDurability`, `ReclaimBeforeReadRelease`, `TombstoneResurrection`, `GenerationMismatch`, `IllegalTransition`; explicit race denials: `DenyInPlaceOverwrite`, `DenyEarlyReclaim`, `DenyStaleEpochReuse`, `DenyBackendResidueCandidateSelection`, `DenyLatchHierarchyInversion`, `DenyMixedRootRead`; owner outcomes remain typed as `LsmMembershipDenial`, `LsmExecutionDenial`, or `LsmMaintenanceDenial` rather than being flattened into a boolean |

##### Lease and reclaim

| Kind | Explicit model vocabulary |
| --- | --- |
| States/frontiers | Lease: `Absent`, `Active`, `Released`, `Revoked`, `ExpiredNoAuthority`; lease generation and identity generation; reclaimed/not reclaimed; reused/not reused; crashed/open; leaked/not leaked |
| Legal transitions/actions | `LeaseAcquired`, `LeaseReleased`, `LeaseRevoked`, `LeaseExpiredWithoutAuthority`, `OwnedCopyStabilized`, `ReclaimAdmitted`, `ReclaimDeniedByLiveLease`, `IdentityReuseAdmitted`, `IdentityReuseDenied`; checked machine actions additionally expose `Leak` and `Crash` |
| Typed denials/blocked edges | `LiveLeaseProtectsIdentity`, `ExpiryIsNotReleaseAuthority`, `GenerationDidNotAdvance`, `StaleLeaseGeneration` |

##### Quarantine and readmission

| Kind | Explicit model vocabulary |
| --- | --- |
| States/frontiers | `Proposed`, `Sealed`, `RecoveryVerificationPending`, `Readmitted`, `RetainedForAudit`, `Denied`; carried frontiers: scope preserved, verification complete/incomplete, current authority absent/present, observation-only evidence, operator intent |
| Legal transitions/actions | `Seal`, `ObserveOffline`, `RequestOperatorRepair`, `BeginVerification`, `CompleteVerification`, `AdmitAuthority`, `Readmit`, `RetainAudit` |
| Typed denials/blocked edges | `QuarantineReceiptRequired`, `VerificationFrontierIncomplete`, `CurrentAuthorityRequired`, `ScopeMismatch`, `ObservationIsNotRepairAuthority`, `OperatorIntentIsNotRepairAuthority` |

##### Import publication

| Kind | Explicit model vocabulary |
| --- | --- |
| States/frontiers | `RawDeclaration`, `CurrentScopeReadmitted`, `RecoveredArtifactAdmitted`, `LayoutMaterialized`, `PublicationPending`, `PublicationDurable`, `PublicationDenied` |
| Legal transitions/actions | `RawDeclarationObserved`, `CurrentScopeReadmitted`, `RecoveredArtifactAdmitted`, `LayoutMaterializationAdmitted`, `PublicationPending`, `PublicationDurable`, `CrashBeforePublication`, `PublicationDenied` |
| Typed denials/blocked edges | `CurrentScopeReadmissionRequired`, `RecoveredArtifactAdmissionRequired`, `LayoutMaterializationRequired`, `PublicationReadinessRequired`, `ExactPhysicalPublicationRequired` |

##### Replication admission

| Kind | Explicit model vocabulary |
| --- | --- |
| States/frontiers | `Raw`, `SourceAdmitted`, `SourceDenied`, `ProgressObserved`, `DuplicateObserved`, `ResumeDenied`, `DivergenceDetected`, `PublicationPending`, `PublicationDurable`, `PublicationDenied`; delivery: `None`, `Fresh`, `Resumed`; current/candidate progress frontiers; epoch, lineage, durability, and current-publication postures |
| Legal transitions/actions | `SourceAdmitted`, `FreshProgressObserved`, `ResumeProgressObserved`, `DuplicateObserved`, `FreshPublicationPending`, `ResumePublicationPending`, `FreshPublicationDurable`, `ResumePublicationDurable` |
| Typed denials/blocked edges | `SourcePeerIdentityDenied`, `SourceEpochRequiredDenied`, `SourceLineageIdentityDenied`, `SourceCurrentAuthorityDenied`, `SourceReplayIdentityDenied`, `ResumeCurrentAuthorityDenied`, `SourceEpochDivergenceDetected`, `LineageDivergenceDetected`, `ReplayOverlapDivergenceDetected`, `ResumeProgressGapDenied`, `PublicationCurrentAuthorityDenied`, `PublicationPeerProgressChangedDenied` |

##### Shared-frontier composition

| Kind | Explicit model vocabulary |
| --- | --- |
| States/frontiers | Durability: `Pending`, `Admitted`; visibility: `Stable`, `CompactionCutover`, `Reopened`; reachability: `Reachable`, `LiveLease`, `ReleaseEligible`, `Reused`; quarantine: `Clear`, `Sealed`, `VerificationPending`, `Released`; external admission: `None`, `ImportPending`, `ReplicationPending`, `ExternalDurable`, `Divergence`, `Published`; carried proof: recovery precedence, verification advance, old-authority reachability, crash posture, external publication |
| Legal transitions/actions | `DurabilityAdmitted`, `RecoveryPrecedencePreserved`, `LiveLeaseAcquired`, `LeaseReleased`, `CompactionCutover`, `Crash`, `Reopen`, `QuarantineSealed`, `QuarantineVerificationStarted`, `QuarantineReadmitted`, `ReclaimDeferred`, `ReclaimReleased`, `GenerationReused`, `CheckpointPublicationRequested`, `ImportAdmissionPending`, `ReplicationAdmissionPending`, `ExternalDurabilityAdmitted`, `ExternalPublicationRequested`, `ReplicationDivergenceDetected` |
| Typed denials/blocked edges | `RecoveryPrecedenceRequired`, `CrashRequired`, `QuarantineVerificationRequired`, `LiveLeaseBlocksRelease`, `QuarantineBlocksRelease`, `QuarantineBlocksReuse`, `ReleaseRequiredBeforeReuse`, `DurabilityAdmissionRequired`, `ExternalAdmissionRequired`, `ReopenRequiredAfterCrash`, `DivergenceBlocksPublication`, `QuarantineBlocksPublication`, `IllegalTransition` |

**Open questions**
- Do we need one shared-frontier model and one race-specific model?

### Phase 13: Model Runner, Evidence Orchestration, And Counterexample Workflow

This phase turns checked models into an operational system rather than a folder
of proofs. The milestone must own how executed observations are mapped into
canonical action sequences, checked, compared against expected legality, and
surfaced back to engineers when a counterexample appears.

**Relevant subsystems**
- `worth-store-formal-models`
- `worth-store-certification`
- `worth-store-physical-certification`

**Relevant APIs**
- required new `worth_store_formal_models::runner`
- required new `ProtocolCheckInvocation`, `ProtocolCheckVerdict`,
  `ProtocolCheckBounds`, and `CounterexampleLocalization`
- `worth_store_certification::courtroom`
- `StoreDiagnosticSupportReportEvidence`
- `StoreDiagnosticExplanationBundleEvidence`

**Warnings**
- A checked TLA+ file committed beside code is not the deliverable.
- Raw counterexample output is not an operational artifact until Store can map
  it back to runtime state, owner case, protocol edge, and certification lane.
- Receipt and trace normalization must not hide protocol ambiguity to make the
  model easier to satisfy.

**Test requirements**
- Runner parity test: a certification scenario with legal owner outcomes and
  receipts maps to an action sequence that the checker accepts at the same
  named protocol frontier.
- Counterexample localization test: a deliberately weakened transition rule
  produces a counterexample report naming the protocol family, illegal edge,
  owner binding, abstraction function, and failing certification lane.
- Receipt-loss classification test: omitted diagnostics, omitted authoritative
  receipts, and crash-lost non-authoritative traces produce distinct runner
  outcomes instead of generic mismatch failure.

**Engineering decisions**
- The runner accepts canonical cold projections derived from typed Store
  outcomes and receipts, not ad hoc text logs.
- Counterexample reports are self-describing boundary artifacts with protocol
  family, state edge, trace excerpt, owner bindings, and lane identity.
- Certification lanes may cache checked model artifacts, but the formal-model
  crate owns how they are interpreted.
- Runner outcomes must distinguish:
  - legal protocol execution
  - illegal runtime transition
  - receipt omission defect
  - unsupported backend profile
  - bound exhaustion or inconclusive check
  - counterexample found and localized

**Open questions**
- Which runner outputs must be persisted as certification artifacts versus
  transient local debugging support?

### Phase 14: Backend Assumption Matrix And Counter Program

This phase binds the checked models to exact counters and declared backend/
hardware assumptions so no protocol claim floats above the media reality it
depends on.

**Relevant subsystems**
- `worth-store-formal-models`
- `worth-store-certification`
- backend capability owner crates

**Relevant APIs**
- required new `worth_store_formal_models::assumptions::*`
- `AdmittedBackendCapabilityWitness`
- `BackendCapabilityClaimWitness`
- `BackendDurabilityProfile`
- `StoreDurabilityRequirement`
- `StoreDurabilityExecutionProof`
- `AccessPolicyExecutionReceipt`
- `BackendQueueExecutionCompletion`
- required new `worth_store_certification::courtroom::protocol_models::backend_matrix`
- evidence-derived runner counter projection plus exact closeout assertions in
  `worth_store_certification::courtroom::protocol_models::closeout`

**Warnings**
- Protocol claims without backend assumption rows are not valid roadmap credit.
- Counter presence without exact counter assertions is not valid roadmap credit.
- Bound exhaustion and unsupported backend posture must be typed outcomes, not
  buried in logs.

**Test requirements**
- Backend-matrix test: every checked family declares the backend/hardware
  assumptions under which its model claim is valid.
- Counter exactness test: all required `S.9` counters are asserted exactly in
  at least one lane.
- Unsupported-backend denial test: a backend profile that cannot satisfy a
  family's assumptions produces a typed non-claim or denial.

**Engineering decisions**
- Backend profiles are first-class model parameters.
- Counter families include receipt emission, mapping rejection, state
  exploration, deadlock detection, bound exhaustion, and counterexample
  localization.

**Open questions**
- Which assumptions already belong to `worth-store-physical-backend`, and which
  finite abstraction limits belong only to the formal-model runner?

### Phase 15: Certification Lanes, Mutation Program, And Closeout

This phase assembles the courtroom, runs controlled defects against every model
family, performs workspace residue checks, and closes only when the checked
semantics, ordinary owner executions, and backend assumptions agree.

**Relevant subsystems**
- `worth-store-formal-models`
- `worth-store-certification`
- `worth-store-physical-certification`

**Relevant APIs**
- all `worth_store_formal_models::protocols`, `runner`, `assumptions`, and
  `protocol_bindings` surfaces
- required new `worth_store_certification::courtroom::protocol_models::*`
- existing `LayoutOwnerExecutionEvidence`, `LayoutOwnerCoverageReceipt`, and
  certification scenario/replay harnesses
- `StoreDiagnosticSupportReportEvidence`
- `StoreDiagnosticExplanationBundleEvidence`

**Warnings**
- Certification is the courtroom, not the law. It may author scenarios,
  schedules, faults, mutants, and diagnostic evidence; it may not construct an
  owner outcome or add a model action.
- A passing checked artifact without ordinary owner reachability is not
  closeout evidence.
- `LayoutFormalObservation` is retained only if it remains a useful courtroom
  projection. It must not duplicate the model semantics or binding manifest.
- Test support may prepare hostile inputs but cannot issue production outcomes.

**Test requirements**
- Exact closeout matrix test: each model family has a checked artifact,
  reproducible invocation, exact owner-case mapping, ordinary executed
  scenarios, backend assumptions, and at least one localized controlled defect.
- Authority inversion compile-fail test: certification, formal-model, and test-
  support crates cannot construct owner outcomes or use model verdicts as
  production authority.
- Residue test: no placeholder model enum, milestone-shaped production name,
  stale `S8*` API reference, generic transition receipt lane, or certification-
  authored owner catalog remains.
- Mutation sensitivity test: weakening each critical invariant causes the named
  lane to fail for the intended counterexample, not for setup or environment
  errors.

**Engineering decisions**
- `worth-store-formal-models` owns checked semantics, owner-to-model mappings,
  assumption declarations, checker invocation, and counterexample
  interpretation.
- Runtime crates own outcomes, receipts, denials, and ordinary execution.
- `worth-store-certification` owns lane assembly, executed scenario evidence,
  mutant selection, and final adjudication.
- The Phase 2 skeleton is the target topology. Small deviations are allowed
  when they improve responsibility shape, but ownership direction and domain
  names are not flexible.

**Open questions**
- Which model bounds are required in the fast CI lane, nightly hostile lane,
  and release certification lane?

## Must Ship

- hard cutover of the existing `worth-store-formal-models` placeholder into the
  Phase 2 responsibility-shaped crate
- a cold owner-binding manifest over current production outcomes and receipts,
  with explicit capability gaps
- the missing production replication admission outcomes in
  `worth-store-replication`
- checked `TLA+`/configuration artifacts for:
  - durability/recovery frontier
  - recovery source precedence
  - compaction visibility
  - lease/reclaim
  - quarantine/readmission
  - import readmission/publication
  - replication admission
  - shared cross-protocol frontiers
- family-specific owner observations only where current outcomes or durable
  receipts do not already provide an honest cold projection
- independent, exhaustive owner-to-model abstraction functions
- backend/hardware assumption profiles
- exact owner-binding and model-runner counters
- model runner and counterexample report surfaces
- certification courtroom lanes that bind ordinary executed-owner evidence to
  model-runner output
- topology tests enforcing the required directory skeleton

## Must Preserve

- runtime authority remains in runtime crates, not in certification
- offline observations remain observations unless admitted through explicit
  runtime readmission
- derived artifacts never promote themselves to authority through model
  convenience
- observations, model actions, traces, and diagnostics remain subordinate to
  owner-issued outcomes and durable family-specific receipts
- degraded, quarantined, rebuildable, and unrecoverable outcomes stay distinct
- protocol models remain aligned with domain vocabulary and owner boundaries,
  never roadmap or milestone vocabulary
- runtime mutation paths do not depend on formal-model or certification crates
- backend/hardware assumptions stay explicit and parameterized
- boundedness claims stay explicit where the roadmap promises bounded access,
  replay, reclaim, or degraded exact behavior

## Acceptance Evidence

- checked model artifacts committed for every required protocol family
- reproducible model-check commands committed and exercised in CI/release lanes
- binding tests proving exact equality among declared owner cases, ordinary
  executed cases, and mapped model actions
- runtime mapping tests proving every modeled action is derived from a concrete
  owner observation or durable receipt through an independent abstraction
  function
- certification lanes proving legal receipts/traces pass and illegal
  receipts/traces fail
- explicit state/transition/denial tables in the phase-owned spec content for
  each modeled family and shared-frontier composition family
- exact counter assertions for:
  - receipt emission
  - omission classification
  - mapping rejection
  - state exploration
  - transition exploration
  - deadlock detection
  - bound exhaustion
  - counterexample localization
- at least one mutant or controlled-defect lane per protocol family
- counterexample reports that localize illegal transitions back to concrete
  owner families, case identities, mapping functions, and certification lanes
- backend/hardware matrix evidence proving each checked family names the media
  assumptions under which the claim is valid
- topology tests proving the required crate and module ownership shape
- milestone closeout report that names residual risks explicitly rather than
  hiding them in generic proof success

## False-Completion Gates

The milestone is not complete if any of the following remain:

- `ModeledStateMachine` or another placeholder enum stands in for checked
  artifacts.
- A production type, function, module, or directory introduced or modified by
  this milestone is named after an `S.*` sequence, a phase, a milestone, or
  roadmap provenance.
- The formal-model or certification crate can construct a production owner
  outcome.
- A model action exists without an ordinary owner execution, or an in-scope
  owner case exists without exactly one mapped action.
- A case id, string, declaration row, diagnostic trace, or
  `LayoutFormalObservation` substitutes for executed-owner evidence.
- A generic transition receipt is added where an operation-specific outcome or
  existing durable receipt is the real boundary.
- Replication is modeled while `worth-store-replication` still lacks the real
  admission and publication outcomes named in Phase 10.
- Repair execution or operator authorization is claimed even though `S.10` has
  not built those production owners.
- Model checking, normalization, or process execution appears on an ordinary
  read, write, recovery, compaction, import, or replication hot path.
- A backend claim is checked only under idealized atomic storage assumptions.
- A compile-fail fixture fails because of a missing crate or setup error rather
  than the intended privacy or type boundary.
- A mutant passes, a bound-exhausted run is reported as proof, or a
  counterexample cannot be localized to the owner boundary.

## Sequencing Notes

- `S.9` belongs after `S.8` because it depends on layout families, access
  lowering phases, maintenance publication law, corruption/quarantine posture,
  LSM membership, physical compaction, and owner-issued observation surfaces.
- `S.9` belongs before `S.10` because operator repair, PITR, disaster recovery,
  and forensics need protocol law for quarantine, readmission, import
  publication, and replication admission before they can become trustworthy
  operational workflows. `S.10` remains responsible for operator authorization
  and repair execution.
- `S.9` belongs before `S.12` because certification cannot honestly claim
  aerospace-grade physical truth if the highest-risk transitions are only
  tested, never modeled.
