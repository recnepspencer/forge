# Storage Foundation S.9 Engineering Spec: Formal Protocol Models For Physical Truth

## Goal

Turn the Store physical protocol into checked, executable state-machine law for
the crash, recovery, compaction, lease/reclaim, repair/quarantine, import,
replication, and shared-frontier composition paths that hostile tests cannot
exhaustively cover, with explicit backend assumptions, authoritative runtime
transition receipts, and machine-checkable counterexample localization.

## Why This Milestone Exists

Roadmap 2 already gives Store physical pages, integrity, recovery physics,
security scope, I/O scheduling, native blobs, and layout/access discipline.
That is enough power to build a real database and enough complexity to quietly
rebuild infrastructure folklore if the critical transitions remain "understood"
instead of modeled.

`S.8` is the last milestone before this one because it names the admitted
artifact families, access lowering phases, stale/rebind/readmission rules,
maintenance publication rules, corruption/quarantine posture, and layout hazard
inventory that `S.9` must now turn into explicit protocol models. If `S.9`
arrives too early, it models abstractions that code does not own yet. If it
arrives too late, the codebase starts preserving database truth by convention.

This milestone therefore does not "add formal methods." It freezes the
smallest operational state machines whose failure would invalidate aerospace-
grade database claims even if the rest of the codebase looked polished:

- WAL/checkpoint/page flush ordering
- recovery source precedence under contradictory evidence
- compaction cutover and visibility
- physical reader leases and reclaim barriers
- repair/quarantine transitions
- import or replication admission when physical evidence is the authority

## Governing Summaries

- `MENTALITY.md` protects adversarial-first infrastructure design. `S.9` must
  model the damaged, concurrent, operator-pressured system first rather than
  the clean protocol we hope is true.
- `arch_laws.md` protects proof-bearing phases, explicit orchestration, and
  authority separation. `S.9` must expose checked protocol phases that map to
  code states and orchestrators rather than producing detached proof bags.
- `composition_laws.md` protects named semantic steps. `S.9` must split
  protocol inventory, runtime trace extraction, model execution,
  counterexample interpretation, and certification handoff into distinct
  modules and phases instead of hiding them in one "formal verification"
  bucket.
- `domain_structure_laws.md` protects responsibility-shaped topology. `S.9`
  must give formal-model authority a dedicated crate and directory skeleton so
  runtime protocol code, checked models, certification materialization, and
  test support do not collapse into the same structural space.
- `perf_laws.md` protects cost honesty and bounded execution. `S.9` must model
  not only safety but also bounded degraded behavior where the roadmap claims
  bounded scans, bounded replay, bounded reclaim, or bounded repair posture.
- `worth_store_roadmap_2.md` protects the physical database foundation gate.
  `S.9` exists because tests and certification lanes alone cannot exhaustively
  cover the crash-plus-concurrency transitions most likely to falsify Store's
  database claim.

## Adversarial Constraint

Under crash, restart, compaction, reclaim, corruption, quarantine, repair, and
import pressure, Store must never allow a lower-authority physical artifact,
derived projection, stale visibility frontier, or ambiguous degraded path to
win over declared authority, and it must make any illegal transition detectable
through checked protocol models, typed runtime mapping, and certification lanes
that fail when the implementation drifts.

## Product Decision Lock

- `S.9` is about physical protocol law, not general semantic correctness,
  planner semantics, or user-facing API behavior.
- Models are authoritative only for the named state machines. They do not
  redefine page format, WAL bytes, or layout families; they constrain how those
  artifacts transition.
- A checked model without runtime mapping is not milestone credit.
- A runtime mapping without a checked model is not milestone credit.
- A proof artifact without orchestration, trace extraction, failure
  interpretation, and certification lanes is not milestone credit.
- `S.9` models production protocol states, not certification-only toy states.
- Recovery, compaction, repair, and import models must admit degraded and
  quarantined outcomes explicitly; they may not collapse uncertainty into
  "empty," "ignored," or "best effort."
- The milestone may create a dedicated formal-model crate, runner surfaces,
  typed trace adapters, counterexample reports, and certification closeout
  modules. It may not move runtime recovery authority into the certification
  harness.

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

## Runtime Conformance Contract

The conformance problem for `S.9` is not "can a trace be replayed through a
model." It is "can the runtime perform an in-scope protocol transition without
producing an authoritative, typed, model-mappable receipt."

Every modeled protocol family must therefore define:

- sealed runtime transition types owned by the runtime protocol crate
- typed transition receipts emitted at the authoritative transition boundary
- a public protocol-observation facade exposing those receipts without exposing
  lower private topology
- an independent abstraction function from runtime receipt -> modeled action
  owned by `worth-store-formal-models`
- completeness rules proving that every in-scope authoritative transition emits
  exactly one observable receipt or one typed denial
- omission classification rules distinguishing:
  - impossible because no transition occurred
  - instrumentation defect
  - crash-loss of a non-authoritative diagnostic
  - illegal protocol hole

Every phase-owned protocol family must classify its observation artifacts into
exactly one of these receipt classes:

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

The model runner may consume typed receipts and typed traces, but those traces
are support artifacts around the authoritative receipts. Runtime-to-model
conformance must not depend on the model layer defining what the runtime was
allowed to do after the fact.

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

## S.9 Counter Contract

`S.9` must publish exact counters for the model and the runtime conformance
boundary. At minimum:

- authoritative transition receipts emitted
- typed transition denials emitted
- runtime events rejected from mapping
- runtime receipt omissions detected
- normalization rejections
- model states explored
- model transitions explored
- invariant checks executed
- deadlocks found
- truncated searches or bound exhaustion outcomes
- counterexamples produced
- counterexamples localized back to runtime receipts
- unsupported backend/hardware assumption mismatches

## Phase Plan

### Phase 1: Runtime Owners Publish The Protocol Inventory

This phase freezes the runtime-owned inventory that every later `S.9` phase
consumes. `S.9` may not infer protocol scope from docs, crate names, or
certification tests. Each runtime protocol owner must publish its own typed
inventory rows, while `S.8` contributes only the layout/access hazard slice it
actually owns.

**Relevant subsystems**
- `worth-store-layout-indexes`
- `worth-store-wal`
- `worth-store-recovery-physics`
- `worth-store-buffer-pool`
- `worth-store-operations`
- `worth-store-offline-verifier`
- `worth-store-physical-integrity`

**Relevant APIs**
- new runtime-owned `*ProtocolInventoryRow` surfaces in each owner crate
- new `worth_store_formal_models::protocol_inventory::StoreProtocolInventory`
- `worth_store_layout_indexes::S8LayoutHazardInventory`
- `worth_store_layout_indexes::layout_closeout`
- `worth_store_recovery_physics::layout_access`
- `worth_store_operations::OperationalRecoveryPosture`

**Warnings**
- A layout-owned handoff is not enough. WAL, recovery, leases, repair, and
  replication/import owners must publish their own rows.
- Inventory rows must be operational, not literary. Each row must name the
  owner crate, authoritative inputs, derived inputs, forbidden inputs,
  transition receipts, illegal transitions, detection surface, containment
  action, recovery action, certification lane, and residual risk.
- Inventory rows must distinguish runtime authority, advisory observation,
  offline observation, certification evidence, and forbidden convenience
  projections.

**Test requirements**
- Inventory completeness test: every roadmap `S.9` protocol target has at least
  one runtime-owned inventory row, and every inventory row names its owner
  crate, receipt surface, and certification lane.
- Inventory ownership denial test: `worth-store-layout-indexes` cannot publish
  recovery, WAL, lease, repair, or replication/import authority rows that are
  not admitted from the owning runtime crate.

**Engineering decisions**
- `S8LayoutHazardInventory` becomes the layout/access hazard contribution, not
  the whole milestone authority summary.
- `worth-store-formal-models` assembles runtime-owned inventory rows into the
  milestone protocol inventory, but does not mint them.
- Each row names one protocol family only; mixed rows create proof fog.

**Open questions**
- Which protocol owners can share one inventory vocabulary file without
  collapsing their authority boundaries?

### Phase 2: Formal Semantics, Toolchain, And Directory Ownership Freeze

This phase freezes the formal toolchain, checked artifact shape, dependency
direction, and directory skeleton before any protocol family is modeled. The
topology and authority direction are part of the milestone law and may not be
retrofit after family-specific work begins.

**Relevant subsystems**
- `worth-store-formal-models` (new)
- `worth-store-certification`
- all runtime owner crates that must expose protocol-observation facades

**Relevant APIs**
- new `worth_store_formal_models::protocol_inventory`
- new `worth_store_formal_models::checked_models`
- new `worth_store_formal_models::runtime_mapping`
- new `worth_store_formal_models::backend_profile`
- new runtime-owned `protocol_observation` facades
- new `worth_store_certification::s9_formal_models_closeout`

**Warnings**
- Do not start with category-shaped `states.rs/actions.rs` folders and decide
  the real tool later.
- Do not let runtime crates depend on the model runner or certification crate.
- Do not let the formal-model crate deep-import private runtime topology.

**Test requirements**
- Topology freeze test: required `worth-store-formal-models` directories,
  checked-model artifact locations, runtime protocol-observation facades, and
  certification closeout modules exist in the declared ownership locations.
- Dependency-direction test: runtime crates cannot depend on model runner or
  certification modules, and formal-model modules cannot bypass runtime public
  protocol-observation facades.

**Engineering decisions**
- Introduce a dedicated `worth-store-formal-models` crate as production-owned
  formal-semantics, mapping, runner, and counterexample authority.
- Checked model artifacts live beside the protocol family modules that own
  their semantics.
- Certification consumes formal verdicts and localization reports; it does not
  own the model semantics.

**Open questions**
- Should the checked model artifacts live under `checked_models/` or under each
  protocol family directory as sibling `.tla/.cfg` files?

### Phase 3: Runtime Refinement And Receipt Conformance Freeze

This phase freezes the conformance mechanism between runtime and model. The
milestone does not proceed to family-specific checked models until each family
has an authoritative runtime transition receipt surface, an independent
abstraction mapping, and omission/loss classification rules.

**Relevant subsystems**
- `worth-store-recovery-physics`
- `worth-store-wal`
- `worth-store-buffer-pool`
- `worth-store-operations`
- `worth-store-formal-models`

**Relevant APIs**
- new runtime-owned sealed `*TransitionReceipt` surfaces
- new runtime-owned `protocol_observation::*`
- new `worth_store_formal_models::runtime_mapping::AbstractionFunction`
- new `worth_store_formal_models::runtime_mapping::MappingCompletenessReport`

**Warnings**
- Typed traces are not enough on their own. Authoritative transition receipts
  must exist at the runtime boundary.
- Mapping logic must be independent of receipt production or it becomes a
  shared bug surface.
- Crash-loss semantics for diagnostics and authoritative receipts must be named
  before any protocol family claims conformance.

**Test requirements**
- Receipt completeness test: every in-scope authoritative runtime transition
  either emits one typed receipt or one typed denial; silent transition paths
  fail.
- Mapping independence test: a controlled defect in abstraction mapping is
  caught by a lane that compares authoritative receipt structure to model
  consumption assumptions rather than only end verdict.

**Engineering decisions**
- Runtime owners publish protocol-observation facades.
- Formal-model code owns the abstraction functions from receipt to modeled
  action.
- Certification lanes consume receipts, mapped actions, and localization
  reports, not only replayed traces.

**Open questions**
- Which receipt families must be durable media artifacts versus in-memory
  post-reopen observations?

### Phase 4: End-To-End Durability And Recovery Frontier Model

This phase models the shared durability frontier that ties together WAL append,
page flush, checkpoint cutover, recovery source precedence, redo, root
publication, and repeated reopen. It is the first checked end-to-end reference
protocol because the later families depend on the same durability truth.

**Relevant subsystems**
- `worth-store-recovery-physics`
- `worth-store-wal`
- `worth-store-buffer-pool`
- `worth-store-offline-verifier`
- `worth-store-formal-models`

**Relevant APIs**
- `worth_store_recovery_physics::WalAppendPlan`
- `worth_store_recovery_physics::CheckpointPublicationPlan`
- `worth_store_recovery_physics::RecoverySourceDecisionTrace`
- `worth_store_recovery_physics::RecoveryDeterminismReport`
- new `worth_store_formal_models::protocols::durability_recovery_frontier`

**Warnings**
- This model must include ugly states, not only durable steady states.
- Recovery source precedence is one phase inside the frontier model, not the
  whole frontier.
- Offline observations may advise comparison lanes, but runtime authority still
  comes from admitted artifacts and receipts.
- "flush requested" and "flush durably completed" are different states.
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
- new `worth_store_formal_models::protocols::recovery_source_precedence`

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
- `worth-store-io-scheduler`
- `worth-store-formal-models`

**Relevant APIs**
- `worth_store_recovery_physics::AdmittedCompactionCutoverRecord`
- `worth_store_recovery_physics::CompactionGenerationVisibility`
- `worth_store_recovery_physics::CompactionVisibleProductEvidence`
- `worth_store_layout_indexes::S8IndexPublicationProtocol`
- `worth_store_layout_indexes::S8IndexMaintenanceTransitionOutcome`
- new `worth_store_formal_models::protocols::compaction_visibility`

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
  generation-retention receipts.
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
- `worth-store-physical-format`
- `worth-store-recovery-physics`
- `worth-store-formal-models`

**Relevant APIs**
- `worth_store_buffer_pool::PageLease`
- `worth_store_buffer_pool::PinnedPageLease`
- `worth_store_buffer_pool::LeaseEpoch`
- `worth_store_physical_format::PhysicalReclaimRegion`
- new `worth_store_formal_models::protocols::lease_reclaim`

**Warnings**
- A lease local to the buffer pool is not enough. `S.9` cares about reclaim and
  reuse meaning across compaction, recovery, and repair.
- Revocation, expiry, crash-loss, and leaked-holder states must be explicit.
- Explicit degraded exact scan is legal only where admitted; hidden broad
  fallback remains illegal.

**Test requirements**
- Lease-vs-reclaim race test: a surviving lease blocks reclaim or reuse until a
  legal barrier release occurs.
- ABA-reuse mutant test: generation reuse before durable barrier advance must
  fail.
- Hidden-fallback denial test: exact-only reads may not degrade to whole-store
  scans or lower-authority reads by convenience.

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
  - degraded exact scan widened into hidden broad fallback
  - generation reuse admitted before barrier completion
  - crashed holder treated as safely revoked without modeled revocation law

**Open questions**
- Which lease classes require durable receipts versus reconstructed post-reopen
  observations?

### Phase 8: Repair And Quarantine Submodel

This phase isolates the damaged-truth operational path. The goal is to model
how quarantine is entered, preserved, verified, rolled back, or released
without accidentally letting operator workflow substitute for protocol law.

**Relevant subsystems**
- `worth-store-operations`
- `worth-store-offline-verifier`
- `worth-store-recovery-physics`
- `worth-store-formal-models`

**Relevant APIs**
- `worth_store_operations::RepairBlastRadiusReadiness`
- `worth_store_operations::RepairQuarantineScopePreservation`
- `worth_store_offline_verifier::OfflineRepairBlastRadiusObservation`
- `worth_store_recovery_physics::RecoveryCorruptionReadmissionHandoff`
- new `worth_store_formal_models::protocols::repair_quarantine`

**Warnings**
- Offline observation may advise admission or verification, but it may not
  self-promote into authority.
- Blast radius, operator authorization, quarantine, repair execution, and post-
  repair verification are distinct states.
- Partial repair crash and quarantine release are first-class protocol
  transitions, not operator folklore.

**Test requirements**
- Quarantine-preservation test: scope, region identity, and custody posture
  survive runtime-to-offline-to-runtime handoff without widening.
- Crash-during-repair test: interrupted repair reopens into verification,
  rollback, retained quarantine, or unrecoverable posture only.
- Quarantine-release mutant test: releasing quarantine without verification
  frontier advance must fail.

**Engineering decisions**
- States must include:
  - blast radius planned
  - operator authorized
  - quarantine required
  - repair executing
  - repair interrupted
  - verification required
  - quarantine retained
  - quarantine released
  - rollback executed
- This submodel emits quarantine-frontier receipts distinct from durability or
  visibility receipts.
- Legal transition guards must include:
  - `quarantine required -> repair executing` requires admitted blast-radius and
    operator authority witnesses
  - `repair interrupted -> rollback executed` requires rollback legality owned
    by the repair protocol rather than generic operator choice
  - `verification required -> quarantine released` requires explicit successful
    verification frontier advance
- Required denials must include:
  - offline observation accepted as authority
  - quarantine released without verification
  - repair applied outside admitted blast radius
  - rollback chosen from stale or cross-scope repair inputs

**Open questions**
- Which repair actions require direct operator-proof receipts versus runtime
  protocol receipts?

### Phase 9: Import Admission Submodel

This phase isolates one-shot external physical evidence admission when the
store is deciding whether imported bytes may become local authority.

**Relevant subsystems**
- `worth-store-operations`
- `worth-store-offline-verifier`
- `worth-store-formal-models`

**Relevant APIs**
- `worth_store_operations::BackupImportCustodyReadmission`
- `worth_store_operations::ImportPlacementPlan`
- new import-owned admission receipts
- new `worth_store_formal_models::protocols::import_admission`

**Warnings**
- Authenticity unavailable and authenticity failed must remain distinct.
- Custody unavailable and custody mismatch must remain distinct.
- Partial receipt, resume, duplicate bytes, and crash before publication are
  protocol states, not transport noise.

**Test requirements**
- Plausible-import rejection test: lineage, scope, or evidence mismatches are
  rejected instead of normalized into authority.
- Crash-before-publication test: imported bytes admitted for evaluation but not
  yet published never become ambient authority after reopen.
- Duplicate-fragment test: repeated receipt of the same fragment remains inside
  the modeled import state machine.

**Engineering decisions**
- States must include:
  - source declared
  - bytes partially received
  - lineage checked
  - custody checked
  - authenticity unavailable
  - authenticity failed
  - authority publication pending
  - authority publication completed
  - authority publication denied
- Legal transition guards must include:
  - `bytes partially received -> lineage checked` requires the declared import
    identity and transfer continuity witness
  - `custody checked/authenticity checked -> authority publication pending`
    requires admitted physical evidence class, not terminal convenience output
  - `authority publication pending -> authority publication completed` requires
    durability-frontier admission
- Required denials must include:
  - authenticity unavailable collapsed into authenticity success
  - custody mismatch collapsed into import retry
  - duplicate fragment treated as fresh progress
  - crash-before-publication converted into ambient admitted authority

**Open questions**
- Which import states can share transfer primitives with replication while
  preserving distinct semantics?

### Phase 10: Replication Admission Submodel

This phase isolates peer-driven external evidence admission. Replication is not
just import with a different name because it preserves peer progress, replay,
resume, duplicate receipt, and divergence semantics over time.

**Relevant subsystems**
- `worth-store-replication`
- `worth-store-recovery-physics`
- `worth-store-formal-models`

**Relevant APIs**
- new replication-owned admission receipts
- new peer-progress observation surfaces
- new `worth_store_formal_models::protocols::replication_admission`

**Warnings**
- Replication may share evidence vocabulary with import, but it must keep
  distinct states and denials.
- Source epoch drift, peer divergence, duplicate replay, and partial transfer
  are protocol truths, not incidental transport cases.

**Test requirements**
- Replication-resume test: partial transfer, resume, and duplicate receipt stay
  within modeled replication states.
- Divergence-denial test: mismatched source epoch or lineage fork never becomes
  fresh import truth.
- Crash-before-replica-publication test: peer-provided bytes do not become
  authority through a durability shortcut.

**Engineering decisions**
- States must include:
  - peer progress observed
  - transfer resumed
  - duplicate receipt detected
  - divergence detected
  - source epoch accepted
  - source epoch denied
  - publication pending
  - publication denied
  - publication completed
- Legal transition guards must include:
  - `peer progress observed -> transfer resumed` requires continuity with the
    admitted source epoch and lineage
  - `duplicate receipt detected` may not advance publication progress unless
    the protocol explicitly marks it idempotent
  - `publication pending -> publication completed` requires the same durability
    frontier admission as import, plus replication-specific source legality
- Required denials must include:
  - source epoch drift normalized into import success
  - divergence ignored because bytes are locally decodable
  - duplicate replay counted as new admitted progress
  - replication publication completed without peer legality witness

**Open questions**
- Do replication and import share one admission frontier model or two sibling
  frontier adapters?

### Phase 11: Shared Frontier Composition Model

This phase is where `S.9` earns the roadmap claim about crash plus concurrency.
It composes the local submodels around named shared frontiers so the combined
failure surfaces are checked instead of narrated.

**Relevant subsystems**
- `worth-store-formal-models`
- all runtime owner crates
- `worth-store-certification`

**Relevant APIs**
- new `worth_store_formal_models::protocols::shared_frontiers`
- all family-local receipt and mapping surfaces from Phases 4-10
- new `worth_store_formal_models::counterexample::CrossProtocolLocalizationReport`

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
- Repair-vs-reclaim race test: repair planning or execution blocks reclaim and
  reuse until a legal frontier release.
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
  - no repair completion without verification-frontier advance
  - no compaction cutover that strands recovery without a legal precedence path
- Composition must be expressed as:
  - one named frontier model for shared state ownership
  - plus at least one race-focused composition model if one model would become
    unreadable
- Required composed races include:
  - compaction cutover vs surviving lease
  - checkpoint publication vs quarantine admission
  - repair execution vs reclaim/reuse
  - import publication vs crash durability window
  - replication publication vs divergence detection

**Open questions**
- Do we need one shared-frontier model and one race-specific model?

### Phase 12: Model Runner, Receipt Orchestration, And Counterexample Workflow

This phase turns checked models into an operational system rather than a folder
of proofs. The milestone must own how traces are gathered, normalized, replayed
through models, compared against expected legality, and surfaced back to
engineers when a counterexample appears.

**Relevant subsystems**
- `worth-store-formal-models`
- `worth-store-certification`
- `worth-store-physical-certification`

**Relevant APIs**
- new `worth_store_formal_models::model_runner`
- new `worth_store_formal_models::scenario_trace`
- new `worth_store_formal_models::counterexample`
- new `worth_store_formal_models::checked_models`
- `worth_store_certification::courtroom`

**Warnings**
- A checked TLA+ file committed beside code is not the deliverable.
- Raw counterexample output is not an operational artifact until Store can map
  it back to runtime state, protocol edge, lane, and expected repair surface.
- Receipt and trace normalization must not hide protocol ambiguity to make the
  model easier to satisfy.

**Test requirements**
- Runner parity test: a certification scenario with legal runtime receipts and
  traces replays through the model runner and lands in a legal terminal state
  with the same named protocol frontier.
- Counterexample localization test: a deliberately weakened transition rule
  produces a counterexample report naming the protocol family, illegal edge,
  runtime mapping row, and failing certification lane.
- Receipt-loss classification test: omitted diagnostics, omitted authoritative
  receipts, and crash-lost non-authoritative traces produce distinct runner
  outcomes instead of generic mismatch failure.

**Engineering decisions**
- The runner accepts typed Store receipts and typed Store traces, not ad hoc
  text logs.
- Counterexample reports are self-describing boundary artifacts with protocol
  family, state edge, trace excerpt, runtime mapping rows, and lane identity.
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

### Phase 13: Backend Assumption Matrix And Counter Program

This phase binds the checked models to exact counters and declared backend/
hardware assumptions so no protocol claim floats above the media reality it
depends on.

**Relevant subsystems**
- `worth-store-formal-models`
- `worth-store-certification`
- backend capability owner crates

**Relevant APIs**
- `worth_store_formal_models::backend_profile::*`
- `worth_store_certification::s9_formal_models_closeout::backend_matrix`
- `worth_store_certification::s9_formal_models_closeout::counter_assertions`

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
- Which backend assumptions deserve shared vocabulary with `S.6` and `S.12`
  versus `S.9`-local vocabulary?

### Phase 14: Certification Lanes, Directory Skeleton, And Closeout

This phase fixes the topology that future engineers must live inside and closes
the milestone with machine-checkable lanes. The directory shape is part of the
spec because a 293k-line database cannot leave formal protocol ownership to
grep archaeology.

**Relevant subsystems**
- `worth-store-formal-models`
- `worth-store-certification`
- `worth-store-physical-certification`

**Relevant APIs**
- new `worth_store_formal_models::*`
- new `worth_store_certification::s9_formal_models_closeout::*`
- existing `worth_store_certification::courtroom::*`

**Warnings**
- Do not dump checked models into certification or test-support folders.
- Do not create a `formal`, `proofs`, or `models` junk drawer with mixed
  runtime mapping, checked specs, and lane fixtures.
- Phases are lifecycle phases. Directories must preserve that lifecycle.
- Generic category files are not enough. The tree must reveal checked-model
  artifacts, backend profiles, abstraction functions, runtime receipt mapping,
  and family-local versus shared-frontier ownership.

**Test requirements**
- Directory topology test: required `S.9` crate and closeout modules exist,
  and forbidden cross-layer placement of checked models or runtime mapping
  files fails a topology test.
- Lane-failure test: each named `S.9` protocol family has at least one mutant
  or controlled-defect lane that passes only when the runner catches the
  illegal transition.

**Engineering decisions**
- Required directory skeleton:

```text
workspaces/worth-store/crates/worth-store-formal-models/
  src/
    lib.rs
    protocol_inventory/
      mod.rs
      family_row.rs
      runtime_owner_row.rs
      layout_hazard_contribution.rs
    runtime_mapping/
      mod.rs
      abstraction_function.rs
      receipt_mapping.rs
      omission_classification.rs
      vocabulary.rs
      denial.rs
    backend_profile/
      mod.rs
      durability_class.rs
      atomicity_posture.rs
      io_completion_posture.rs
    scenario_trace/
      mod.rs
      publication_trace.rs
      recovery_trace.rs
      compaction_trace.rs
      lease_trace.rs
      repair_trace.rs
      import_trace.rs
      replication_trace.rs
    model_runner/
      mod.rs
      checked_model.rs
      replay.rs
      verdict.rs
      bound_exhaustion.rs
    checked_models/
      mod.rs
      toolchain.rs
      commands.rs
    counterexample/
      mod.rs
      report.rs
      runtime_localization.rs
    protocols/
      durability_recovery_frontier/
        mod.rs
        durability_transition_receipts.rs
        durability_abstraction.rs
        checked_model.tla
        checked_model.cfg
        durability_frontier_states.rs
        durability_frontier_actions.rs
        durability_frontier_invariants.rs
        durability_frontier_fairness.rs
      recovery_source_precedence/
        mod.rs
        precedence_transition_receipts.rs
        precedence_abstraction.rs
        checked_model.tla
        checked_model.cfg
        precedence_states.rs
        precedence_actions.rs
        precedence_invariants.rs
      compaction_visibility/
        mod.rs
        compaction_transition_receipts.rs
        compaction_abstraction.rs
        checked_model.tla
        checked_model.cfg
        compaction_visibility_states.rs
        compaction_visibility_actions.rs
        compaction_visibility_invariants.rs
      lease_reclaim/
        mod.rs
        lease_transition_receipts.rs
        lease_abstraction.rs
        checked_model.tla
        checked_model.cfg
        lease_reclaim_states.rs
        lease_reclaim_actions.rs
        lease_reclaim_invariants.rs
      repair_quarantine/
        mod.rs
        repair_transition_receipts.rs
        repair_abstraction.rs
        checked_model.tla
        checked_model.cfg
        repair_quarantine_states.rs
        repair_quarantine_actions.rs
        repair_quarantine_invariants.rs
      import_admission/
        mod.rs
        import_transition_receipts.rs
        import_abstraction.rs
        checked_model.tla
        checked_model.cfg
        import_admission_states.rs
        import_admission_actions.rs
        import_admission_invariants.rs
      replication_admission/
        mod.rs
        replication_transition_receipts.rs
        replication_abstraction.rs
        checked_model.tla
        checked_model.cfg
        replication_admission_states.rs
        replication_admission_actions.rs
        replication_admission_invariants.rs
      shared_frontiers/
        mod.rs
        frontier_transition_receipts.rs
        frontier_abstraction.rs
        checked_model.tla
        checked_model.cfg
        shared_frontier_states.rs
        shared_frontier_actions.rs
        shared_frontier_invariants.rs
        shared_frontier_fairness.rs
      race_composition/
        mod.rs
        race_transition_receipts.rs
        race_abstraction.rs
        checked_model.tla
        checked_model.cfg
        race_states.rs
        race_actions.rs
        race_invariants.rs
  tests/
    protocol_inventory_tests.rs
    runtime_mapping_tests.rs
    model_runner_tests.rs
    backend_profile_tests.rs
    shared_frontier_tests.rs
    recovery_source_precedence_tests.rs

workspaces/worth-store/crates/worth-store-certification/
  src/
    s9_formal_models_closeout/
      mod.rs
      evidence.rs
      runner.rs
      mutant_lanes.rs
      reports.rs
      backend_matrix.rs
      counter_assertions.rs
```

- `worth-store-formal-models` owns protocol inventory, runtime mapping, model
  replay, and counterexample interpretation.
- `worth-store-certification` owns lane assembly and milestone closeout
  evidence.

**Open questions**
- Should shared-frontier checked models split into one durability/visibility
  model and one quarantine/admission race model?

## Must Ship

- runtime-owned protocol inventory rows for every `S.9` owner crate
- a real `S8LayoutHazardInventory` as the layout/access contribution
- a dedicated `worth-store-formal-models` crate
- checked `TLA+`/configuration artifacts for:
  - durability/recovery frontier
  - recovery source precedence
  - compaction visibility
  - lease/reclaim
  - repair/quarantine
  - import admission
  - replication admission
  - shared cross-protocol frontiers
- runtime protocol-observation facades with sealed transition receipts
- independent runtime-to-model abstraction functions
- backend/hardware assumption profiles
- exact `S.9` counters
- model runner and counterexample report surfaces
- certification closeout lanes that consume model runner output rather than
  only static checked artifacts
- topology tests enforcing the required directory skeleton

## Must Preserve

- runtime authority remains in runtime crates, not in certification
- offline observations remain observations unless admitted through explicit
  runtime readmission
- derived artifacts never promote themselves to authority through model
  convenience
- traces and diagnostics remain subordinate to authoritative transition receipts
- degraded, quarantined, rebuildable, and unrecoverable outcomes stay distinct
- protocol models remain aligned with runtime vocabulary and phase structure
- runtime mutation paths do not depend on formal-model or certification crates
- backend/hardware assumptions stay explicit and parameterized
- boundedness claims stay explicit where the roadmap promises bounded access,
  replay, reclaim, or degraded exact behavior

## Acceptance Evidence

- checked model artifacts committed for every required protocol family
- reproducible model-check commands committed and exercised in CI/release lanes
- protocol inventory tests proving runtime-owner completeness
- runtime mapping tests proving every modeled action is derived from sealed
  runtime receipts through an independent abstraction function
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
- counterexample reports that localize illegal transitions back to runtime
  protocol rows and certification lanes
- backend/hardware matrix evidence proving each checked family names the media
  assumptions under which the claim is valid
- topology tests proving the required crate and module ownership shape
- milestone closeout report that names residual risks explicitly rather than
  hiding them in generic proof success

## Sequencing Notes

- `S.9` belongs after `S.8` because it depends on layout families, access
  lowering phases, maintenance publication law, corruption/quarantine posture,
  and the typed layout hazard contribution.
- `S.9` belongs before `S.10` because operator repair, PITR, disaster recovery,
  and forensics need protocol law for repair/quarantine/import/replication
  transitions before they can become trustworthy operational workflows.
- `S.9` belongs before `S.12` because certification cannot honestly claim
  aerospace-grade physical truth if the highest-risk transitions are only
  tested, never modeled.
