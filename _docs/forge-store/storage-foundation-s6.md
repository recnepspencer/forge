# Storage Foundation S.6: Hardware-Aware I/O, QoS, And Background Work Pacing

## Goal

Make physical I/O behavior an explicit Store contract: backend capability,
media assumptions, queue admission, foreground reservations, background pacing,
flush ordering, secure-I/O posture, and interference evidence must be typed,
counter-backed, and testable.

## Why This Milestone Exists

S.5 made physical reads stable under maintenance. S.5.1 made security scope
structural through physical paths. S.6 is the next foundation gate: once Store
can safely read bytes, it must prove that foreground reads and writes are not
silently captured by background compaction, checkpointing, scrub, replication
preparation, blob ingest, blob migration, backup preparation, or repair
pressure.

This milestone is not a backend preference enum and not a benchmark suite. It
is the first Store-owned I/O contract: hardware assumptions are admitted before
use, foreground lanes reserve capacity before execution, background work paces
or denies itself through typed policy, and certification can explain latency
interference through counters rather than folklore.

## Governing Summaries

- `MENTALITY.md` protects adversarial-first design. S.6 must begin from hostile
  foreground/background interference and make the wrong thing mechanically
  unrepresentable rather than adding optimistic I/O knobs.
- `arch_laws.md` protects proof-bearing phase progression and explicit
  boundary crossings. S.6 must separate capability admission, scheduling
  policy, queue execution, flush durability, and evidence publication.
- `composition_laws.md` protects named responsibility and file topology. S.6
  must not hide backend capabilities, queue decisions, latency accounting,
  flush semantics, and diagnostics in one I/O manager.
- `domain_structure_laws.md` protects authority topology. Backend assumptions,
  media facts, foreground reservations, background pacing, page-cache policy,
  secure-I/O posture, and certification evidence fail differently and need
  separate homes.
- `perf_laws.md` protects visible cost. S.6 must expose queue-depth,
  foreground-wait, background-yield, sync, flush, bandwidth, stall, and
  interference counters at the boundary where claims are made.
- `forge_store_roadmap_2.md` places S.6 after stable reads and security-scope
  metadata because I/O pacing can be honest only after bytes are stable and
  security scope can survive the physical path.
- `crates/forge-foundational/docs/FOUNDATIONAL_README.md` protects the rule:
  keep the strongest owning Store type while meaning is Store-owned, then
  lower into Foundational only for shared boundary, profile, canonical,
  diagnostic, provenance, support, and performance meaning.
- `crates/forge-proof/README.md` protects progression law. S.6 should use
  Proof for phase/stage readiness, witnesses, freshness, trust-boundary
  readmission, checked outcomes, and fixed-shape composition, not as a runtime
  scheduler or diagnostic crate.

## Adversarial Constraint

S.6 must survive this hostile condition:

> Foreground reads and writes run while compaction, checkpointing, scrub,
> replication preparation, blob ingest, blob migration, backup preparation, and
> repair scans create queue-depth pressure, flush pressure, page-cache pressure,
> bandwidth contention, and backend latency spikes. Store must preserve the
> declared foreground latency/interference envelope and security scope through
> typed I/O admission, scheduling, pacing, and evidence. It must not rely on OS
> scheduler luck, aggregate throughput, untyped backend folklore, hidden worker
> queues, copied S.5/S.5.1 handoffs, logs, terminal projections, or same-run
> self-comparison.

If a foreground path can freeze because background work failed to yield, if a
backend can claim durability or direct-I/O support without admitted media
assumptions, if secure/platform-grade I/O admits unsupported encrypted or
authenticated posture, if security scope is stripped at the scheduler boundary,
or if certification cannot causally explain interference with counters, S.6 is
not closed.

## Product Decision Lock

- S.6 owns physical I/O admission, queue scheduling, foreground reservation,
  background pacing, backend/media capability posture, and interference
  evidence.
- S.6 does not reopen S.5 physical stability, S.5.1 security admission, S.7
  blob lifecycle, S.10 backup/repair semantics, S.11 key lifecycle, or S.12
  final database certification.
- Backend capability changes may affect cost and availability, not durability
  meaning.
- Unsupported capability, durability, secure-I/O, or QoS claims fail typed
  rather than silently degrading.
- Certification is the courtroom. Lower Store crates own I/O law and
  constructors; certification proves the law with hostile scenarios.

## Foundational And Proof Usage Lock

S.6 must use `forge-foundational` and `forge-proof` deliberately, not as vague
quality labels.

**Store-owned authority stays in Store**

- Backend/media witnesses, foreground reservations, background pacing plans,
  queue admission, flush durability, secure-I/O admission, page-cache policy,
  and scheduler admission are Store authority.
- Foundational artifacts may describe, package, compare, profile, certify, or
  publish that authority after Store has produced it; they may not replace the
  Store-owned witness or admit scheduler work by themselves.
- Proof artifacts may enforce the legal progression into execution readiness;
  they may not become the runtime scheduler or the certification report.

**Foundational surfaces S.6 should use**

- `forge_foundational::performance_api::common_path` for descriptive
  performance claim authoring and layout intent before execution.
- `forge_foundational::performance_api::lower_lane::policy` for
  `FoundationalPolicyAdmissionReceipt` when foreground or background budget
  policy admits, defers, denies, or records debt before execution.
- `forge_foundational::performance_api::lower_lane::receipts` for
  `FoundationalCounterBackedPerformanceReceipt` and
  `FoundationalPerformanceCounterRow` after I/O execution produces exact
  queue, flush, sync, wait, yield, bandwidth, and interference counters.
- `forge_foundational::performance_api::lower_lane::reports` for deliberate
  support/report widening through `FoundationalPerformanceReportRequest`,
  `FoundationalPerformanceReportPlan`, and
  `FoundationalMaterializedPerformanceReport`.
- `forge_foundational::performance_api::stronger_lane::certified` and
  `forge_foundational::performance_api::stronger_lane::readiness` only after
  lower Store evidence and counter-backed receipts exist.
- `forge_foundational::profiles_api` for compatibility, support, diagnostic
  richness, certification posture, and target-aware materialization profiles.
- `forge_foundational::boundary_evidence_api` for provenance, lineage,
  completed/executed receipts, support truth, attachment readmission, and
  support-grade closeout evidence.
- `forge_foundational::canonicalization_api` for canonical basis, digest-ready
  comparison, export/readmission, and mismatch classification of published S.6
  evidence bundles. Digest evidence is never Store authority.
- `forge_foundational::aspects()` only where S.6 needs native aspect-authored
  boundary facts or explicit security/profile metadata; `compatibility().json()`
  remains terminal/hostile/readmission only.

**Proof surfaces S.6 should use**

- `forge_proof::prelude::*` is the default import lane for implementation.
- `recipe(...)`, `.resolve_with(...)`, `.lower_with(...)`,
  `.ready_with(...)`, and `.execute()` should model ordinary
  capability/reservation/pacing/queue/flush progression when each step consumes
  a Store-owned witness or capability.
- Checked variants such as `.try_resolve_ready(...)`,
  `.try_lower_ready(...)`, `.try_ready_now(...)`, and `.try_execute()` should
  be used where denial, deferred, stale, rebind-required, and failed outcomes
  must remain distinct.
- `AuthorityWitness<_>` and `CapabilityWitness<_>` authorize Store-owned
  transitions; they are not semantic proof by themselves.
- `.bridge_trust_boundary()`, `.rebind_with(...)`, and
  `.readmit_with(...)` are required for imported, restored, replayed, or
  certification-returned evidence that crosses a trust boundary.
- `Pair`, `NonEmpty`, `UniqueVec`, `CanonicalVec`, `join_ready(...)`, and
  `compose_ready(...)` should be used only when S.6 has a fixed, meaningful
  composition shape that must not degrade into raw tuples or unordered vectors.

## S.6 Operational Vocabulary

These terms are Store-owned S.6 vocabulary. Implementation may encode them as
types, enums, witnesses, or receipts, but the distinctions must remain
mechanically visible.

**Foreground envelope states**

- `ReservationAdmitted`: the foreground lane received concrete resource units
  under a declared backend/media/security basis.
- `ReservationHeld`: execution completed inside the admitted envelope or inside
  the declared bounded-interference posture for that profile.
- `ReservationViolatedWithCause`: execution exceeded the envelope and carries
  causal attribution, affected resource units, and remediation posture.
- `ReservationAdmissionDenied`: Store denied foreground work before execution
  because required resource units or assumptions were unavailable.
- `ReservationStaleRebindRequired`: backend/media/security/resource basis
  changed after admission, so the reservation must be rebound before trusted
  execution.

S.6 does not promise universal latency numbers. QoS law controls admission,
pacing, revocation, violation reporting, and interference attribution.
Certification proves those laws under named profiles; it does not prove
universal backend performance.

**Resource units**

- `QueueSlot`: an admitted place in the scheduler/backend queue.
- `BandwidthToken`: admitted byte-rate or byte-window capacity for a lane.
- `FlushPermit`: permission to enter a flush/sync ordering group.
- `SyncDebt`: accumulated durability work that must be paid before a stronger
  durability or readiness claim can close.
- `ReadAheadWindow`: bounded speculative read extent tied to a security basis.
- `WriteBackWindow`: bounded deferred write extent tied to dirty-page and flush
  policy.
- `DirtyPageBudget`: admitted dirty physical page/frame pressure from S.2.
- `WorkerPermit`: bounded scheduler worker capacity for foreground/background
  execution.
- `CacheResidencyHint`: admitted page-cache/mmap/buffer residency intent, not a
  proof that the OS complied.
- `ReclaimPermit`: admitted trim, punch-hole, or cold-tier movement authority
  under S.5 protected-footprint evidence.

Reservation and pacing receipts must name the concrete resource units they
admit, deny, borrow, revoke, or violate.

**Background borrowing**

Background work may consume idle foreground-adjacent capacity only through a
revocable borrow lease. Borrowed capacity must yield when foreground demand
appears. Failure to yield becomes typed interference debt and may produce
`ReservationViolatedWithCause` for the affected foreground lane.

**Capability evidence classes**

- `DeclaredByConfig`: supplied by configuration or deployment contract.
- `ObservedByProbe`: observed through Store probes under the current process,
  kernel, filesystem, mount, backend, and hardware basis.
- `CertifiedBackendProfile`: proven by the S.6 qualification matrix for the
  named backend profile.
- `ExternallyGuaranteed`: guaranteed by an external platform contract that
  Store can reference but not prove internally.
- `UnverifiableAssumption`: required assumption that cannot be proven inside
  Store and must be surfaced as residual risk or denied platform-grade posture.

Capability admission must preserve the evidence class; it may not turn
configuration, probes, external guarantees, or unverifiable assumptions into
stronger truth by naming.

**Durability progression vocabulary**

- `WriteSubmitted`: Store submitted bytes to a backend path.
- `WriteAcceptedByBackend`: the backend accepted the write request.
- `WriteReachedDurabilityBoundary`: backend-specific durability boundary was
  reached under admitted assumptions.
- `ParentNamespaceDurable`: parent namespace or equivalent name authority is
  durable for rename/link visibility.
- `RenameDurable`: rename/link replacement is durable under the backend model.
- `OrderingBarrierDurable`: ordering barrier required by WAL/checkpoint/recovery
  law is durable.
- `DurabilityUnsupported`: backend cannot support the requested claim.
- `DurabilityUnknown`: Store cannot prove the requested claim under the current
  evidence class.

POSIX `fsync`, `fdatasync`, directory sync, and durable rename are backend
implementations of these Store terms, not the ontology itself.

**Counter strength classes**

- `ExactCounter`: exact structural count required for closeout claims that
  assert exactness.
- `MonotonicCounter`: nondecreasing counter suitable for accumulated debt or
  progress.
- `SampledCounter`: sampled observation suitable only for profiled telemetry,
  not exact proof.
- `BoundedEstimate`: bounded estimate with declared error policy.
- `AttributionCounter`: causal classification counter tying waits, debt, or
  interference to a resource and lane.
- `DiagnosticOnlyCounter`: support evidence that cannot satisfy readiness or
  scheduler authority.

Closeout must say which counter strength each claim consumes. "Counter-backed"
without a strength class is not an S.6 proof.

**Post-admission violation outcomes**

- `AdmissionDenied`: denied before execution.
- `ExecutionViolated`: admitted execution broke a scheduler, grouping,
  security, access-mode, or durability invariant.
- `BackendContradictedWitness`: observed backend behavior contradicted an
  admitted capability witness.
- `EnvelopeExceeded`: foreground envelope was exceeded under an admitted lane.
- `PolicyDebtIncurred`: execution remained legal but incurred named debt that
  blocks stronger posture until paid or closed.

S.6 must handle both pre-execution denial and post-admission violation. A
system that can only deny before execution cannot honestly model real I/O.

**Fault evidence classes**

- `SimulatedFault`: harness-modeled pressure.
- `ProductionBoundaryFault`: pressure injected at a production boundary through
  declared yieldpoints.
- `BackendEmulatedFault`: backend implementation emulates a fault class.
- `RealBackendQualificationResult`: qualification evidence from the actual
  backend profile.

Fault-injection evidence proves scheduler behavior under modeled pressure. It
does not prove the backend actually exhibits or survives that behavior unless
the qualification profile supplies `RealBackendQualificationResult` evidence.

## Planned Directory Skeleton

Implementation may adjust exact names to match local topology, but these
responsibility boundaries must remain visible:

- `forge-store-physical-backend/src/io_capability/` owns backend and media
  capability declarations, admission, and assumption witnesses.
- `forge-store-io-scheduler/src/foreground_reservation/` owns foreground lane
  budgets, reservation admission, and reservation receipts.
- `forge-store-io-scheduler/src/background_pacing/` owns maintenance lane
  pacing, yield/deny decisions, and debt posture.
- `forge-store-io-scheduler/src/queue_admission/` owns queue-depth,
  bandwidth, read-ahead, write-back, and write-grouping admission.
- `forge-store-io-scheduler/src/flush_durability/` owns fsync/fdatasync,
  directory sync, durable rename, flush ordering, and sync evidence.
- `forge-store-io-scheduler/src/page_cache_access_policy/` owns buffered,
  mmap, and direct-I/O page-cache interaction policy.
- `forge-store-io-scheduler/src/space_reclaim_tier_io_policy/` owns
  trim/punch-hole policy and cold-tier movement I/O posture.
- `forge-store-io-scheduler/src/security_scope_io/` owns preservation of
  S.5.1 security scope through I/O scheduling and secure-I/O posture denial.
- `forge-store-io-scheduler/src/interference_counters/` owns causal counters
  for queueing, backpressure, flush delay, reclaim debt, compaction debt, scrub
  pressure, backup pressure, repair pressure, and blob contention.
- `forge-store-physical-certification/src/s6_io_pressure_harness/` owns S.4.5
  scenario families, production-boundary drivers, observers, transcripts, and
  evidence for S.6 I/O pressure.
- `forge-store-certification/src/s6_io_qos_closeout/` owns courtroom suites,
  compile-fail boundaries, performance receipts, and closeout evidence.

## Phase Plan

### Phase 1: Backend And Media Capability Admission

Freeze the backend/media contract before any I/O path can claim durability,
queue behavior, direct I/O, mmap safety, async execution, secure-frame support,
or hardware-aware QoS.

**Relevant subsystems**

- `forge-store-physical-backend`
- `forge-store-io-scheduler`
- `forge-store-security`
- `forge-store-readiness`
- `forge-store-certification`
- `forge-foundational`
- `forge-proof`

**Relevant APIs**

- S.5 `S6IoQosIsolationReadiness`
- S.5.1 S.6 security-scope handoff
- physical backend capability declarations
- media durability assumption declarations
- capability evidence classes: `DeclaredByConfig`, `ObservedByProbe`,
  `CertifiedBackendProfile`, `ExternallyGuaranteed`, and
  `UnverifiableAssumption`
- secure-I/O posture vocabulary from S.5.1
- `forge_foundational::profiles_api` support/compatibility/certification
  posture profiles for backend support publication
- `forge_foundational::canonicalization_api` basis and digest comparison for
  capability evidence bundles
- `forge_foundational::boundary_evidence_api` provenance and support truth for
  admitted backend/media assumptions
- `forge_proof::prelude::*` checked readiness progression for current, stale,
  denied, deferred, failed, and rebind-required capability witnesses

**Warnings**

- Do not model backend capability as a string label or enum alone.
- Do not let a backend choose durability semantics at execution time.
- Do not accept direct-I/O, mmap, async-I/O, fsync, directory-sync, or durable
  rename claims without admitted assumptions.
- Do not let unsupported secure-I/O posture degrade into ordinary platform
  admission.
- Do not upgrade `DeclaredByConfig`, `ObservedByProbe`,
  `ExternallyGuaranteed`, or `UnverifiableAssumption` into
  `CertifiedBackendProfile` by wrapping it in a stronger-looking witness.

**Test requirements**

- Capability parity: two independently admitted backends with the same media
  assumptions produce equivalent capability witnesses, boundary evidence, and
  support posture.
- Capability denial: copied backend labels, raw config, terminal projections,
  OS names, environment variables, and same-process metrics cannot satisfy
  backend capability admission.
- Unsupported posture: a backend that lacks direct I/O, durable rename,
  directory sync, mmap safety, async completion ordering, or secure-frame
  compatibility fails typed when a platform-grade lane requires it.
- Rebind honesty: changing sector size, alignment, flush semantics, or page
  cache policy marks prior capability witnesses stale or rebind-required.
- Confidence honesty: a backend admitted from configuration, probe, external
  guarantee, certified profile, and unverifiable assumption preserves the
  evidence class through support posture and cannot satisfy stronger APIs.
- Environment drift: kernel version, filesystem, mount option, firmware, cloud
  volume class, Store backend version, sector/alignment, or security posture
  changes trigger stale or rebind-required posture.

**Engineering decisions**

- Capability admission produces sealed backend/media witnesses with private
  fields and read-only accessors.
- Backend capability tiers must distinguish buffered file, mmap, direct I/O,
  and optional async I/O as separate admitted postures, not execution modes on
  one mutable backend object.
- Media assumptions must name fsync/fdatasync behavior, directory sync,
  durable rename, alignment, sector atomicity, page-cache policy, and flush
  ordering.
- Capability witnesses carry evidence class, rebind triggers, and confidence
  limitations; admission is honest only when the caller can see how the claim
  was established.
- Foundational profiles package admitted support posture and certification
  posture for publication; Store-owned backend/media witnesses remain the
  authority consumed by S.6.
- Proof freshness and rebind-required states must wrap capability witness
  progression when backend/media assumptions change across a trust or hardware
  boundary.

**Open questions**

- Exact OS/backend implementations may land incrementally, but unsupported
  posture and denied platform claims must land in this phase.

### Phase 2: Foreground Lane Contracts And Reservation Admission

Define the foreground I/O lanes that S.6 protects and make reservation
admission explicit before any scheduler can execute reads or writes.

**Relevant subsystems**

- `forge-store-io-scheduler`
- `forge-store-physical-isolation`
- `forge-store-physical-backend`
- `forge-store-security`
- `forge-store-certification`

**Relevant APIs**

- stable-read execution receipts from S.5
- S.5 wait/interference surfaces and protected-footprint counters
- backend/media capability witnesses from Phase 1
- S.5.1 admitted security-scope witnesses and S.6 handoff
- foreground lane declaration and reservation receipt constructors
- foreground envelope states: `ReservationAdmitted`, `ReservationHeld`,
  `ReservationViolatedWithCause`, `ReservationAdmissionDenied`, and
  `ReservationStaleRebindRequired`
- resource units: `QueueSlot`, `BandwidthToken`, `FlushPermit`, `SyncDebt`,
  `ReadAheadWindow`, `WriteBackWindow`, `DirtyPageBudget`, `WorkerPermit`,
  `CacheResidencyHint`, and `ReclaimPermit`
- `forge_foundational::performance_api::common_path` performance claims and
  layout intent for lane envelopes
- `forge_foundational::performance_api::lower_lane::policy`
  `FoundationalPolicyAdmissionReceipt` for reservation budget admission
- `forge_proof::prelude::*` `AuthorityWitness<_>` and checked readiness
  progression for reservation admission

**Warnings**

- Do not make foreground priority a best-effort flag.
- Do not let background work borrow foreground capacity without an explicit
  reservation or debt record.
- Do not allow a semantic transaction, snapshot, or maintenance label to
  define physical I/O priority.
- Do not reserve capacity without naming the backend capability envelope it
  depends on.
- Do not claim a latency envelope without declaring whether the profile means
  hard bound, soft SLO, bounded interference, starvation freedom, or
  certification-only target.
- Do not handle foreground-vs-background while leaving foreground-vs-foreground
  arbitration implicit.

**Test requirements**

- Reservation parity: replaying the same foreground read/write lane under the
  same backend/media capability and S.5/S.5.1 handoffs produces the same
  reservation basis, counters, and latency envelope declaration.
- Reservation denial: raw lane labels, semantic priority labels, copied S.5
  counters, copied security-scope fields, and terminal projections cannot mint
  foreground reservation receipts.
- Capacity pressure: a foreground lane that cannot reserve its declared queue,
  bandwidth, flush, or memory-adjacent I/O budget denies before executing
  physical I/O.
- Security preservation: reservation input must carry S.5.1 key scope, tenant
  scope, authenticity requirement/class, and custody posture through the
  reservation basis.
- Foreground fairness: point reads, range reads, commit-critical WAL writes,
  ordinary page writes, interactive reads, and internal foreground reads cannot
  starve or launder priority through one another without a declared arbitration
  rule.
- Violation reporting: if an admitted foreground lane exceeds its envelope,
  the result is `ReservationViolatedWithCause` or `EnvelopeExceeded`, not a
  silently successful reservation.

**Engineering decisions**

- Foreground lanes are Store physical I/O lanes, not semantic operation
  classes.
- Reservation admission consumes backend/media capability, S.5 stability
  readiness, S.5.1 security readiness, and declared lane envelope.
- Reservation receipts carry structural counters for requested capacity,
  admitted capacity, denied capacity, and assumed backend limits.
- Latency envelope declarations must be lane-local and profile-scoped; broad
  global p99 claims are not enough.
- Reservation receipts must name concrete resource units reserved, denied,
  borrowed, revoked, or violated.
- Foreground arbitration must distinguish foreground read, foreground write,
  commit-critical WAL write, point read, range read, interactive read, and
  internal foreground read classes when they can interfere.
- Foundational policy receipts can publish budget admission decisions, but
  Store reservation receipts remain the scheduler admission input.
- Proof witnesses authorize the transition into reservation readiness; copied
  witnesses or copied counters do not establish the reservation.

**Open questions**

- Exact envelope numbers can be profile-defined later; this phase must define
  the typed envelope and denial shape.

### Phase 3: Background Work Classes And Pacing Policy

Define background physical work classes and require them to pace, yield, or deny
themselves before they can interfere with admitted foreground lanes.

**Relevant subsystems**

- `forge-store-io-scheduler`
- `forge-store-physical-isolation`
- `forge-store-blob-chunks`
- `forge-store-operations`
- `forge-store-offline-verifier`
- `forge-store-certification`

**Relevant APIs**

- compaction, checkpoint, scrub, replication-preparation, blob-ingest,
  blob-migration, backup-preparation, and repair-scan declarations
- foreground reservation receipts from Phase 2
- background debt and pacing counters
- revocable borrow leases for idle capacity
- `PolicyDebtIncurred`, `ExecutionViolated`, and
  `ReservationViolatedWithCause` outcomes
- S.4.5 actor and driver extension slots
- `forge_foundational::performance_api::lower_lane::policy` receipts for
  yield, defer, deny, throttle, admit-with-debt, and budget outcomes
- `forge_foundational::performance_api::common_path` work-class and layout
  intent claims for background pressure shapes
- `forge_proof::prelude::*` checked outcomes for admitted, deferred, denied,
  stale, rebind-required, and failed pacing progression

**Warnings**

- Do not treat "background" as permission to use leftover capacity
  opportunistically without evidence.
- Do not make compaction, checkpoint, scrub, blob, backup, or repair semantics
  part of S.6; this phase admits only their I/O pressure shape.
- Do not let background work silently become foreground when it is blocked.
- Do not hide background work in worker-local queues that foreground counters
  cannot observe.
- Do not over-throttle background work by forbidding idle-capacity use; require
  revocable leases instead.
- Do not allow borrowed capacity to survive foreground demand without an
  explicit yield, revoke, or debt outcome.

**Test requirements**

- Pacing parity: equivalent background pressure declarations under equivalent
  foreground reservations produce equivalent yield, pace, deny, and debt
  decisions.
- Starvation denial: a background class that would exceed queue-depth,
  bandwidth, flush, or foreground-wait limits yields or denies before it can
  starve foreground reads or writes.
- Class separation: compaction, checkpoint, scrub, replication prep, blob
  pressure, backup prep, and repair pressure cannot substitute for one another
  in pacing APIs.
- Debt visibility: accumulated background debt remains visible through typed
  counters and cannot be hidden in logs, elapsed time, or worker-local state.
- Borrow revocation: background work using idle `QueueSlot`, `BandwidthToken`,
  `WorkerPermit`, `FlushPermit`, or `WriteBackWindow` leases yields when
  foreground demand appears.
- Late yield: a background lease that fails to yield before foreground
  envelope pressure appears records typed interference debt and causal
  attribution.

**Engineering decisions**

- Background classes are I/O pressure shapes, not product lifecycle semantics.
- Pacing policy consumes foreground reservations and backend capability rather
  than recomputing the foreground envelope.
- Yield, defer, deny, throttle, and admit-with-debt are distinct outcomes.
- Idle-capacity borrowing is legal only through revocable borrow leases tied to
  the foreground reservation basis and named resource units.
- Background debt counters must distinguish compaction debt, checkpoint flush
  debt, scrub pressure, replication prep, blob contention, backup pressure, and
  repair pressure.
- Foundational policy receipts publish budget decisions; they do not authorize
  background I/O without the Store-owned pacing decision.
- Proof checked outcomes must preserve non-success categories instead of
  flattening background pressure into one scheduler error.

**Open questions**

- Exact class list can grow in later milestones, but the initial phase must
  include the roadmap-named background pressure families.

### Phase 4: Queue Admission, Grouping, And Backpressure Execution

Lower foreground reservations and background pacing decisions into a typed I/O
queue plan that controls queue depth, read-ahead, write-back, write grouping,
bandwidth, and backpressure without re-deciding policy at execution time.

**Relevant subsystems**

- `forge-store-io-scheduler`
- `forge-store-physical-backend`
- `forge-store-buffer-pool`
- `forge-store-wal`
- `forge-store-certification`

**Relevant APIs**

- foreground reservation receipts
- background pacing decisions
- backend/media capability witnesses
- queue-depth and bandwidth counter surfaces
- WAL and page-frame I/O submission surfaces
- write grouping basis: tenant scope, key scope, authenticity requirement,
  durability class, flush epoch, foreground/background class, recovery
  ordering, and writeback policy
- scoped read-ahead basis from the triggering foreground/security admission
- `forge_foundational::performance_api::lower_lane::policy` receipts for queue
  budget admission and backpressure decisions
- `forge_foundational::performance_api::lower_lane::receipts` counter-backed
  receipts only after execution emits queue-depth and backpressure rows
- `forge_proof::prelude::*` lowered -> readiness -> executed progression for
  queue plans

**Warnings**

- Do not let queue execution reclassify foreground/background work.
- Do not hide queueing in backend-private worker pools.
- Do not batch unrelated security scopes or durability classes behind one
  write-grouping convenience path.
- Do not let read-ahead or write-back exceed the admitted memory/I/O envelope
  from S.2/S.5/S.6.
- Do not treat "execution may not rediscover strategy" as a ban on mechanical
  adaptation to short writes, partial reads, `EINTR`, `EAGAIN`, temporary
  backend saturation, delayed fsync, or device-queue-full responses.
- Do not let mechanical adaptation reclassify work, strengthen claims, merge
  incompatible scopes, or invent new scheduling policy.

**Test requirements**

- Queue replay: replaying the same admitted foreground/background plan under
  the same backend capability produces the same queue admission, grouping,
  backpressure, and counter sequence.
- Queue denial: copied reservation receipts, raw operation labels, backend
  private queue handles, and elapsed-time observations cannot admit queue work.
- Backpressure proof: queue saturation causes typed foreground-preserving
  background backpressure before foreground wait exceeds the declared envelope.
- Grouping boundary: write grouping cannot merge work with incompatible
  durability, security scope, flush ordering, or foreground/background class.
- Grouping basis: write grouping is legal only when tenant scope, key scope,
  authenticity requirement, durability class, flush epoch,
  foreground/background class, recovery ordering, and writeback policy are
  compatible.
- Speculative read denial: read-ahead cannot observe bytes outside the tenant,
  key, security, repair/export/import, and physical-region scope admitted for
  the foreground read that caused it.
- Mechanical adaptation: partial writes, short reads, retries, backend
  saturation, and delayed completions remain inside the admitted policy
  envelope and produce typed counters or violations.

**Engineering decisions**

- Queue admission produces an execution-ready queue plan consumed by the
  scheduler; execution may not rediscover strategy.
- Queue execution may adapt mechanically inside the admitted policy envelope;
  it may not reclassify work, strengthen claims, merge incompatible scopes, or
  invent scheduling policy.
- Queue counters must include submitted units, admitted units, denied units,
  queue-depth samples, grouped writes, backpressure events, and foreground wait
  attribution.
- Read-ahead and write-back are admitted speculative I/O, not invisible helper
  behavior.
- Read-ahead must inherit the same tenant/key/security admission basis as the
  foreground read; cross-scope speculative read-ahead is denied.
- Queue plans must carry security-scope grouping constraints forward from
  S.5.1.
- Foundational counter-backed receipts are emitted from executed queue plans,
  never from planned queue policy alone.
- Proof `ExecutionReadyRecipe`-style progression should make it impossible to
  execute a raw or merely lowered queue plan.

**Open questions**

- None.

### Phase 5: Flush, Sync, Rename, And Durability Ordering

Make flush and sync behavior explicit so S.6 can distinguish durable progress,
buffered progress, delayed writeback, and unsupported durability claims.

**Relevant subsystems**

- `forge-store-io-scheduler`
- `forge-store-physical-backend`
- `forge-store-wal`
- `forge-store-recovery-physics`
- `forge-store-certification`

**Relevant APIs**

- fsync/fdatasync and directory sync capability witnesses
- durable rename and flush-ordering assumption witnesses
- backend-neutral durability states: `WriteSubmitted`,
  `WriteAcceptedByBackend`, `WriteReachedDurabilityBoundary`,
  `ParentNamespaceDurable`, `RenameDurable`, `OrderingBarrierDurable`,
  `DurabilityUnsupported`, and `DurabilityUnknown`
- WAL frame and checkpoint publication surfaces
- recovery replay source-precedence evidence
- queue execution plans from Phase 4
- `forge_foundational::performance_api::lower_lane::receipts` for executed
  flush, sync, directory-sync, and durable-rename counter rows
- `forge_foundational::boundary_evidence_api::lower_lane::receipts` for
  completed durability receipts and support truth
- `forge_proof::prelude::*` checked progression for buffered, flushed,
  directory-synced, durably-renamed, denied, stale, and rebind-required states

**Warnings**

- Do not let "write returned ok" mean durable.
- Do not claim durable rename without directory sync evidence when the backend
  requires it.
- Do not let checkpoint or WAL publication rely on OS writeback folklore.
- Do not flatten flush delayed, flush failed, directory sync unsupported,
  rename unsupported, and rebind-required into one I/O error.
- Do not make POSIX `fsync`, `fdatasync`, directory sync, or durable rename the
  Store ontology; they are backend implementations of Store durability states.
- Do not let a backend without parent-namespace durability silently claim the
  same namespace durability as one that proves it.

**Test requirements**

- Durability replay: equivalent WAL/checkpoint publication under equivalent
  backend assumptions produces equivalent flush ordering, sync receipts,
  directory sync posture, and durability counters.
- Durability denial: raw filesystem names, backend labels, copied sync rows,
  logs, and terminal projections cannot satisfy durable flush or rename APIs.
- Delay attribution: delayed fsync/fdatasync and directory sync produce typed
  foreground wait/interference counters instead of disappearing into elapsed
  time.
- Unsupported claim: a backend that cannot prove required sync, rename, or
  sector-atomicity posture denies platform-grade durability before execution.
- Backend-neutral parity: POSIX file, Windows, cloud volume, network
  filesystem, object-backed, or custom block-device profiles lower into the
  same Store durability states when they claim equivalent durability.
- Unknown honesty: `DurabilityUnknown` cannot satisfy `RenameDurable`,
  `ParentNamespaceDurable`, or `OrderingBarrierDurable` APIs even if writes
  completed successfully.

**Engineering decisions**

- Flush durability is a phase-typed progression over Store durability states:
  submitted, backend-accepted, reached durability boundary, parent-namespace
  durable, rename durable, ordering-barrier durable, unsupported, unknown,
  denied, stale, or rebind-required.
- Recovery-facing durability evidence remains compatible with S.4 source
  precedence and does not reopen recovery correctness.
- Flush/sync counters are separate from queue-depth counters because they fail
  and explain latency differently.
- Foundational boundary evidence may publish durability support, completed
  receipts, and residual debt, but Store-owned sync receipts remain authority.
- Proof progression must make out-of-order durable rename, directory sync, and
  flush-readiness transitions uncallable.
- Backend support posture may affect which durability claims are available. It
  may not silently redefine the meaning of an already-admitted durability
  claim.

**Open questions**

- Backend-specific filesystem details may be introduced per backend, but this
  phase must define the shared authority topology.

### Phase 6: Page-Cache, Mmap, And Direct-I/O Access Policy

Define how Store admits buffered access, mmap visibility, page-cache
interaction, and direct-I/O-shaped execution without turning OS behavior into
an ambient backend assumption.

**Relevant subsystems**

- `forge-store-io-scheduler`
- `forge-store-physical-backend`
- `forge-store-buffer-pool`
- `forge-store-certification`

**Relevant APIs**

- backend capability witnesses from Phase 1
- queue and flush plans from Phases 4 and 5
- S.2 memory admission and page-pin evidence
- buffered, mmap, and direct-I/O access policy declarations
- admitted coherence basis for mixed buffered, mmap, and direct-I/O access to
  the same physical region
- mmap fault posture for lazy faults, `SIGBUS`/fault failure, dirty writeback,
  shared mapping visibility, truncate, and punch-hole interaction
- `forge_foundational::performance_api::common_path` layout intent for
  buffered, mmap, and direct-I/O-shaped access claims
- `forge_foundational::performance_api::lower_lane::policy` receipts for
  page-cache/direct-I/O policy admission
- `forge_proof::prelude::*` capability and readiness progression for
  alignment and buffer-lifecycle requirements

**Warnings**

- Do not let page-cache behavior become an ambient OS assumption.
- Do not allow direct-I/O paths without alignment and buffer-lifecycle proof.
- Do not let mmap visibility bypass S.2 pin/lease/buffer-lifecycle law.
- Do not let direct-I/O-shaped claims stand in for measured direct-I/O
  execution evidence.
- Do not treat mmap as just another I/O mode; it changes fault timing,
  visibility, writeback, and truncate/punch-hole failure behavior.
- Do not mix buffered, mmap, and direct-I/O access for the same physical region
  without an admitted coherence basis.

**Test requirements**

- Policy parity: equivalent page-cache/direct-I/O policy declarations under
  equivalent backend capability produce equivalent admitted policy, alignment
  requirements, and counters.
- Policy denial: unaligned direct-I/O buffers, copied page-cache labels,
  unsupported mmap safety claims, and backend-private cache assumptions deny
  before backend I/O.
- Lease interaction: mmap and direct-I/O access cannot bypass S.2 page pins,
  leases, dirty tracking, or buffer-lifecycle proof.
- Cost visibility: page-cache bypass, cache residency hints, read-ahead
  interaction, and direct-I/O alignment costs produce separate counters rather
  than generic I/O counters.
- Mixed-mode denial: buffered read after direct write, direct read while an
  mmap dirty page exists, mmap visibility before flush/admission, and dirty
  page writeback racing Store buffer state deny without admitted coherence.
- Mmap fault handling: lazy page fault failure, shared mapping visibility,
  kernel-driven read timing, and writeback outside explicit Store calls produce
  typed counters, denial, or violation outcomes.

**Engineering decisions**

- Page-cache policy is an admitted Store policy object, not a backend option
  string.
- Direct-I/O paths require alignment, buffer lifecycle, and sector assumptions
  before execution.
- Mmap and direct-I/O policy consume S.2 buffer and page-pin proof; they do not
  reopen memory-budget law.
- Mixed access modes require an admitted coherence basis. Without it, Store
  denies mixed buffered/mmap/direct access for the same physical region.
- Mmap plans must carry fault posture and writeback posture; they cannot enter
  platform-grade execution as ordinary read/write plans.
- Access policy counters must preserve distinction between cost choices and
  durability meaning.
- Foundational layout intent may describe representation and allocation
  posture; it does not prove the policy executed or that direct I/O is
  supported.
- Proof readiness must carry alignment and buffer-lifecycle proof before
  direct-I/O-shaped plans can execute.

**Open questions**

- Exact direct-I/O implementation depth may grow per backend; this phase
  freezes the admission, denial, and proof shape.

### Phase 7: Trim, Punch-Hole, And Cold-Tier I/O Posture

Define how Store admits space-reclaim and cold-tier movement I/O without
letting reclaim policy race protected reachability or pretend to own S.7/S.10
storage lifecycle semantics.

**Relevant subsystems**

- `forge-store-io-scheduler`
- `forge-store-physical-backend`
- `forge-store-tiering`
- `forge-store-physical-isolation`
- `forge-store-certification`

**Relevant APIs**

- backend capability witnesses from Phase 1
- queue and flush plans from Phases 4 and 5
- page-cache/direct-I/O access posture from Phase 6
- S.5 protected-footprint and reclaim/barrier evidence
- trim, punch-hole, and cold-tier movement policy declarations
- hole interpretation states: `HoleReadsAsZero`, `HoleIsUnavailable`,
  `HoleRequiresTierFetch`, and `HoleDeniedInPlatformGrade`
- `forge_foundational::performance_api::common_path` layout intent for
  trim, punch-hole, and cold-tier movement claims
- `forge_foundational::performance_api::lower_lane::policy` receipts for
  reclaim/tier-movement budget admission
- `forge_proof::prelude::*` readiness progression for protected-footprint and
  movement-admission requirements

**Warnings**

- Do not let trim or punch-hole race S.5 protected reachability.
- Do not let cold-tier movement claim S.10 backup, restore, or tier migration
  correctness.
- Do not implement S.7 blob lifecycle here; S.6 owns only I/O posture and
  pacing for blob pressure.
- Do not let filesystem support labels replace backend capability witnesses.
- Do not permit reclaim without declaring whether holes read as zero, are
  unavailable, require tier fetch, or are denied for platform-grade posture.
- Do not let reclaim create states that backup/PITR, checksum, cold-tier
  restore, sparse-file, copy-on-write snapshot, or mmap fault paths cannot
  interpret.

**Test requirements**

- Reclaim parity: equivalent trim/punch-hole/cold-tier policy declarations
  under equivalent backend and S.5 reachability evidence produce equivalent
  admission, pacing, and counters.
- Reclaim denial: unsupported punch-hole claims, copied filesystem labels,
  terminal projections, and cold-tier movement without S.5 reachability proof
  deny before backend I/O.
- Protected-footprint interaction: trim/punch-hole cannot observe, free, or
  move protected bytes while S.5 reachability barriers are active.
- Lifecycle boundary: cold-tier movement evidence cannot satisfy S.7 blob
  lifecycle APIs or S.10 backup/repair/tier migration APIs.
- Hole interpretation: checksum-over-logical-zero, unavailable-region,
  tier-fetch-required, and platform-denied hole policies produce distinct
  evidence and cannot substitute for one another.
- Snapshot/PITR safety: trim and punch-hole cannot invalidate copy-on-write
  snapshot, backup, PITR, or cold-tier restore assumptions without typed denial
  or rebind-required posture.

**Engineering decisions**

- Trim/punch-hole and cold-tier movement consume S.5 reachability and S.6
  pacing; they do not mint storage lifecycle authority.
- Reclaim and tier-movement counters remain separate from page-cache/direct-I/O
  access counters because the failure mode is physical reachability, not just
  access cost.
- Reclaim plans must carry hole interpretation and later-system compatibility
  posture, while still refusing to claim S.7/S.10 lifecycle correctness.
- Foundational layout intent may describe reclaim/tier access posture, but
  Store-owned reclaim/tier witnesses remain the only execution input.
- Proof readiness must make protected-footprint bypass uncallable for reclaim
  and movement plans.

**Open questions**

- Exact cold-tier backend behavior remains later work; this phase freezes the
  admission and pacing shape.

### Phase 8: Security-Scope And Secure-I/O Preservation

Make every S.6 queue, reservation, flush, and backend admission path preserve
S.5.1 security scope and deny unsupported secure-I/O posture before
platform-grade execution.

**Relevant subsystems**

- `forge-store-security`
- `forge-store-readiness`
- `forge-store-io-scheduler`
- `forge-store-physical-backend`
- `forge-store-physical-format`
- `forge-store-certification`

**Relevant APIs**

- S.5.1 admitted security-scope witnesses
- S.5.1 S.6 I/O QoS security readiness handoff
- physical security metadata carriers
- authenticity requirement/class and result separation
- backend secure-frame compatibility witnesses
- queue, reservation, and flush plans from earlier S.6 phases
- secure-I/O posture dimensions: scope-preserving queue grouping,
  scope-preserving read-ahead/writeback, cache/reclaim exposure posture,
  key-version-compatible write path, and unsupported encrypted/authenticated
  frame compatibility
- `forge_foundational::aspects()` native aspect-authored security/profile
  facts where S.5.1 exposes them as boundary facts
- `forge_foundational::boundary_evidence_api` support truth and provenance for
  secure-I/O unsupported, unavailable, denied, degraded, and admitted posture
- `forge_proof::prelude::*` authority-witness and readmission progression for
  imported or restored security-scope evidence

**Warnings**

- Do not treat security metadata as scheduler side data.
- Do not let a KMS key id, IAM role, tenant label, terminal projection, or
  serde-loaded metadata satisfy secure-I/O admission.
- Do not let physical metadata declare `AuthenticityResult`.
- Do not let unsupported encrypted/authenticated-frame posture enter
  platform-grade I/O paths.
- Do not let read-ahead observe bytes outside the security basis admitted for
  the foreground read that caused it.
- Do not let background batching merge tenant, key, authenticity, custody, or
  repair/export/import scopes through shared queue convenience.
- Do not treat S.6 secure-I/O as encryption, MAC/signature implementation, key
  rotation, operator authorization, audit correctness, or secure deletion.

**Test requirements**

- Scope preservation: foreground and background I/O plans preserve key scope,
  tenant scope, authenticity requirement/class, custody posture, key-version
  posture, and legacy/readmission posture from admission through queue, flush,
  and evidence publication.
- Scope denial: wrong tenant, stale key-version posture, missing authenticity
  requirement, unsupported secure-frame compatibility, terminal projection, and
  serde-loaded metadata deny before platform-grade queue admission.
- Grouping security boundary: write grouping, read-ahead, and background
  batching cannot merge incompatible security scopes.
- Evidence separation: Foundational evidence and Proof progression cannot
  satisfy Store secure-I/O authority without the Store-owned S.5.1 witness.
- Speculative security: read-ahead and write-back remain bounded by the same
  tenant/key/security admission basis as the foreground or background work that
  caused them; cross-scope speculation is denied.
- Secure-I/O scope: S.6 secure-I/O proves scheduler preservation of admitted
  scope through queueing, grouping, flushing, caching, and reclaim, not
  encryption or identity-provider correctness.

**Engineering decisions**

- Security scope is part of the queue admission basis, not metadata attached
  after scheduling.
- Secure-I/O posture must distinguish unsupported, unavailable, stale,
  denied, platform-admitted, and degraded legacy/readmission lanes.
- S.6 consumes S.5.1 witnesses and handoffs; it does not define identity,
  key lifecycle, operator authorization, or encryption algorithms.
- Compile-fail coverage must protect against raw identity/provider/KMS/IAM
  substitutions in I/O scheduler APIs.
- Secure-I/O posture in S.6 means physical scheduler preservation and
  enforcement of admitted security scope through queueing, grouping, flushing,
  caching, and reclaim.
- Encryption, MAC/signature implementation, key rotation, operator
  authorization, audit correctness, secure erase/discard semantics, and
  identity-provider integration remain S.11 or later owner responsibilities.
- Foundational aspect values and boundary evidence may carry native security
  facts at boundaries, but they are not scheduler authority until Store admits
  them into S.5.1/S.6 witnesses.
- Proof readmission is mandatory after backup/import/replay/certification
  boundary crossing; a bridged basis is not current scheduler authority.

**Open questions**

- Encryption and MAC/signature mechanics remain S.11; S.6 owns secure-I/O
  readiness and denial topology only.

### Phase 9: Latency Envelope And Interference Counter Model

Define the counter and attribution model that makes foreground latency,
background interference, queueing, flush delay, backpressure, and maintenance
debt explainable without relying on aggregate throughput or elapsed time alone.

**Relevant subsystems**

- `forge-store-io-scheduler`
- `forge-store-physical-backend`
- `forge-store-budgets`
- `forge-store-certification`
- `forge-foundational`

**Relevant APIs**

- foreground reservation receipts
- queue-depth and backpressure counters
- flush/sync counters
- background debt counters
- `forge_foundational::performance_api::lower_lane::basis` canonical
  performance bundles and comparison basis
- `forge_foundational::performance_api::lower_lane::receipts`
  `FoundationalCounterBackedPerformanceReceipt` and
  `FoundationalPerformanceCounterRow`
- `forge_foundational::performance_api::lower_lane::reports`
  `FoundationalPerformanceReportPlan` and materialized report widening
- `forge_foundational::performance_api::stronger_lane::certified` only for
  evidence-backed bundles after Store execution counters exist
- latency envelope reports from test requirements
- counter strength classes: `ExactCounter`, `MonotonicCounter`,
  `SampledCounter`, `BoundedEstimate`, `AttributionCounter`, and
  `DiagnosticOnlyCounter`
- post-admission outcomes: `ExecutionViolated`, `BackendContradictedWitness`,
  `EnvelopeExceeded`, and `PolicyDebtIncurred`

**Warnings**

- Do not make elapsed time the only proof.
- Do not claim p99/p999 without lane, profile, sample basis, and causal
  counters.
- Do not aggregate foreground and background waits into one "I/O time" number.
- Do not expose rich diagnostics on the hot path unless the policy admits that
  diagnostic cost.
- Do not require deterministic replay of native wall-clock timing or OS
  completion order.
- Do not let sampled, estimated, or diagnostic-only counters satisfy exact
  closeout claims.

**Test requirements**

- Attribution parity: equivalent executed schedules produce equivalent
  foreground wait, background yield, queue-depth, flush-delay, and causal
  interference counters.
- Replay scope: deterministic replay applies to policy decisions, admission
  outcomes, simulated/fault-injected schedules, counter topology, and causal
  attribution categories, not native wall-clock timing or OS completion order.
- Counter denial: a performance report lacking causal counters, lane-local
  sample basis, background debt, or queue-depth evidence cannot satisfy S.6
  closeout.
- Counter strength denial: a `SampledCounter`, `BoundedEstimate`, or
  `DiagnosticOnlyCounter` cannot satisfy a claim requiring `ExactCounter`,
  `MonotonicCounter`, or `AttributionCounter` evidence.
- Post-admission violation: foreground wait exceeded after admission, late
  background yield, backend witness contradiction, or unexpected direct-I/O
  rejection produces typed violation evidence rather than a successful receipt.
- Hidden wait exposure: latch wait, queue wait, flush wait, page-cache wait,
  and worker handoff wait are counted separately so no wait class disappears
  into foreground latency.
- Richness policy: diagnostic materialization can be enabled or disabled
  without changing I/O admission or execution outcome.

**Engineering decisions**

- Latency envelopes are profile-scoped, lane-local contracts with explicit
  sample basis and counters.
- Counters must explain cause: queueing, backpressure, flush delay, reclaim
  debt, compaction debt, scrub pressure, backup pressure, repair pressure, blob
  contention, and backend latency injection.
- Every counter row carries a strength class and claim scope. Closeout claims
  must reject insufficient strength.
- Foundational performance receipts publish executed counter evidence from
  Store snapshots; Store counter snapshots remain the local execution truth.
- Foundational materialized reports are support widening and cannot be used as
  hot-path scheduler inputs.
- Exact counters are mandatory where exactness is the claim; bounded or
  monotonic counters are acceptable for pressure envelopes where the spec names
  the policy.

**Open questions**

- The first local profiles can use deterministic miniature workloads, but they
  must preserve the same counter topology as large profiles.

### Phase 10: S.4.5 I/O Pressure Harness Extension

Extend the S.4.5 simulation harness with S.6 I/O pressure families, production
drivers, latency faults, queue-depth faults, observers, oracles, transcripts,
and generated coverage rows.

**Relevant subsystems**

- `forge-store-physical-certification`
- `forge-store-test-support`
- `forge-store-certification`
- `forge-store-io-scheduler`
- `forge-store-physical-backend`

**Relevant APIs**

- S.4.5 scenario authoring API
- `PhysicalSimulationScenarioDefinition`
- `PhysicalSimulationPlan`
- `PhysicalInterleavingSchedule`
- `PhysicalProofOracle`
- `PhysicalSimulationTranscript`
- `PhysicalCertificationEvidenceBundle`
- S.6 foreground/background I/O declarations and counters
- fault evidence classes: `SimulatedFault`, `ProductionBoundaryFault`,
  `BackendEmulatedFault`, and `RealBackendQualificationResult`
- `forge_foundational::boundary_evidence_api` transcript lineage,
  provenance, executed receipts, support truth, and attachment bundles
- `forge_foundational::performance_api::lower_lane::receipts` executed counter
  receipts emitted from harness-observed Store runs
- `forge_proof::prelude::*` fixed-shape ready joins only where harness
  scenarios have a real static composition shape

**Warnings**

- Do not build an S.6-only runner.
- Do not use single-stream benchmarks as certification evidence.
- Do not let test support decide I/O law or oracle meaning.
- Do not use logs, same-run self-comparison, private mutation, fixture labels,
  or JSON scenario authority as evidence.
- Do not treat simulated, production-boundary, or backend-emulated fault
  evidence as real backend qualification evidence.

**Test requirements**

- Harness replay: the same S.6 I/O pressure scenario replayed through S.4.5
  produces the same schedule, transcript, oracle verdict, counter evidence, and
  coverage rows.
- Harness rejection: a scenario that uses private backend mutation,
  same-process metrics, logs, JSON authority, or a local runner cannot publish
  S.6 certification evidence.
- Fault coverage: backend latency injection, queue-depth saturation, bandwidth
  throttling, delayed fsync/fdatasync, page-cache pressure, and background
  pacing faults all flow through declared production-boundary yieldpoints.
- Cross-profile scaling: deterministic local profiles and large pressure
  profiles use the same scenario family and evidence topology.
- Fault class separation: simulated faults, production-boundary faults,
  backend-emulated faults, and real backend qualification results cannot
  substitute for one another in certification evidence.

**Engineering decisions**

- S.6 adds scenario families and driver/fault/oracle extensions to S.4.5; it
  does not fork the harness lifecycle.
- The golden-path authoring API must let authors express foreground lane,
  background pressure family, backend profile, security posture, and expected
  envelope without hand-building internal Proof or Foundational artifacts.
- Certification-owned oracles decide whether foreground envelopes held and
  whether interference attribution is complete.
- Fault-injection evidence proves scheduler behavior under modeled pressure.
  It does not prove the backend actually exhibits or survives that behavior
  unless the backend qualification profile supplies real qualification
  evidence.
- Generated coverage rows must include backend tier, foreground lane,
  background pressure family, fault kind, secure-I/O posture, and evidence
  maturity.
- Harness support may materialize Foundational boundary evidence after a
  production-boundary run, but test support never mints Store I/O authority.
- Proof joins must remain fixed-shape and explicit; the harness must not hide
  dynamic scenario scheduling inside Proof composition helpers.

**Open questions**

- None.

### Phase 11: Cross-Backend Qualification Matrix

Publish the initial qualification matrix that proves which backend/media
profiles support which S.6 claims and which claims remain unsupported,
unavailable, stale, degraded, or denied.

**Relevant subsystems**

- `forge-store-physical-backend`
- `forge-store-io-scheduler`
- `forge-store-certification`
- `forge-store-compatibility`
- `forge-foundational`

**Relevant APIs**

- backend/media capability witnesses
- `forge_foundational::profiles_api` support, compatibility, certification,
  and target-aware profile attachment surfaces
- `forge_foundational::canonicalization_api` canonical comparison and mismatch
  classification for independently materialized matrix rows
- `forge_foundational::boundary_evidence_api` support truth and residual-debt
  evidence for unsupported, unavailable, stale, degraded, and denied posture
- S.6 latency envelope evidence
- S.6 durability/flush evidence
- qualification rebind triggers: kernel, filesystem, mount options, cloud
  volume class, sector/alignment, firmware, Store backend version, security
  posture, and backend configuration changes
- certification matrix publication surfaces

**Warnings**

- Do not let one backend's successful evidence certify another backend.
- Do not collapse unsupported, unavailable, stale, degraded, and denied.
- Do not certify platform-grade behavior for a backend profile that lacks the
  required secure-I/O, flush, queue, or page-cache evidence.
- Do not hide backend-specific residual debt in prose.
- Do not let stale qualification rows become runtime authority after backend,
  hardware, filesystem, kernel, mount, cloud-volume, or security-posture
  changes.

**Test requirements**

- Matrix parity: independently materialized qualification rows for the same
  backend/profile/evidence basis converge to the same support posture and
  residual-debt classification.
- Matrix denial: copied rows, logs, environment names, test-only backend labels,
  and unsupported capability claims cannot publish backend qualification.
- Cross-backend separation: buffered, mmap, direct-I/O-shaped, and async-shaped
  profiles cannot substitute evidence for one another.
- Residual debt visibility: every unsupported or degraded claim carries a
  machine-readable reason, affected lane, missing evidence, and rebind trigger.
- Rebind trigger proof: qualification rows become stale or rebind-required when
  kernel, filesystem, mount options, cloud volume class, sector/alignment,
  firmware, Store backend version, security posture, or backend configuration
  changes.

**Engineering decisions**

- Qualification matrix rows are evidence products, not runtime authority.
- Runtime S.6 execution consumes admitted backend/media witnesses, not matrix
  projections.
- Foundational profiles package support posture, compatibility posture, and
  certification posture for readers.
- Canonical comparison, not digest equality alone, decides whether two matrix
  rows describe the same support claim.
- Boundary evidence support truth carries residual-debt and degraded-operation
  posture; matrix rows remain publication evidence, not scheduler authority.
- Matrix rows include expiry/revalidation basis and rebind triggers; a stale
  row cannot be promoted into runtime capability admission.
- Backend support can grow after S.6, but every unsupported claim must remain
  explicit.

**Open questions**

- None.

### Phase 12: S.7/S.10/S.11 Readiness And Non-Claim Handoffs

Publish typed handoffs that let S.7 blobs, S.10 backup/repair, and S.11
security consume S.6 I/O pacing evidence without treating S.6 as their domain
law.

**Relevant subsystems**

- `forge-store-readiness`
- `forge-store-io-scheduler`
- `forge-store-blob-chunks`
- `forge-store-operations`
- `forge-store-security`
- `forge-store-certification`

**Relevant APIs**

- S.7 blob I/O pacing readiness
- S.10 backup/export I/O pressure readiness
- S.10 repair scan I/O pressure readiness
- S.11 secure-I/O foundation readiness
- S.6 qualification matrix rows
- S.6 counter-backed performance receipts
- required non-claim markers for S.7 blob, S.10 backup/export, S.10 repair,
  and S.11 secure-I/O readiness handoffs
- `forge_foundational::boundary_evidence_api` completed/executed receipts,
  lineage, provenance, and support truth for handoff evidence
- `forge_foundational::canonicalization_api` export/readmission surfaces for
  published handoff bundles that cross backup/import/certification boundaries
- `forge_proof::prelude::*` trust-boundary bridge, rebind, and readmission
  progression for later-readiness handoff consumption

**Warnings**

- Do not make one generic "I/O readiness" object satisfy all later milestones.
- Do not let S.6 claim blob lifecycle correctness, backup correctness, repair
  authorization, encryption, key rotation, audit, or full security.
- Do not let later handoffs be reconstructed from logs, counters alone, matrix
  rows alone, or terminal projections.
- Do not publish a later-readiness handoff without an explicit non-claim
  section or type-level marker.

**Test requirements**

- Handoff separation: S.7 blob I/O readiness cannot satisfy S.10 backup,
  S.10 repair, or S.11 security readiness APIs, and vice versa.
- Handoff denial: raw S.6 counters, copied qualification rows, logs,
  terminal projections, and certification-only evidence cannot mint later
  readiness.
- Non-claim proof: every S.6 handoff names the exact later claim it does not
  make and the later milestone that must close that claim.
- Non-claim examples: S.7 blob I/O readiness does not prove blob lifecycle
  correctness; S.10 backup I/O readiness does not prove backup restore
  correctness; S.10 repair I/O readiness does not prove operator
  authorization; S.11 secure-I/O readiness does not prove encryption or key
  rotation.
- Security propagation: secure-I/O foundation readiness preserves S.5.1 scope
  and unsupported secure-I/O posture without claiming encryption algorithms.

**Engineering decisions**

- Later-readiness handoffs are sealed Store readiness artifacts with distinct
  authority and accessors.
- Handoffs carry S.6 evidence enough for later milestones to begin without
  reopening S.6 queue, pacing, or backend capability law.
- Each handoff has an explicit non-claim section so later specs cannot
  accidentally inherit more than S.6 proved.
- Each handoff type must carry a non-claim section or type-level marker naming
  the later milestone that owns the unproved claim.
- Certification publishes handoff evidence after lower Store readiness
  construction.
- Foundational boundary evidence describes and packages handoffs; sealed Store
  readiness artifacts remain the only later-milestone admission input.
- Proof readmission must be visible when S.7/S.10/S.11 consume a handoff after
  export, backup, restore, or certification publication.

**Open questions**

- None.

### Phase 13: Certification Evidence Materialization And API Adoption Proof

Materialize the certification evidence products that prove backend capability
admission, foreground reservation, background pacing, queue execution, flush
durability, security-scope preservation, harness replay, cross-backend
qualification, and later handoffs through production-owned types and
certification-owned evidence.

**Relevant subsystems**

- `forge-store-certification`
- `forge-store-physical-certification`
- `forge-store-io-scheduler`
- `forge-store-physical-backend`
- `forge-store-readiness`
- `forge-foundational`
- `forge-proof`

**Relevant APIs**

- S.6 certification suites and compile-fail runners
- S.4.5 transcript and evidence bundle surfaces
- counter strength declarations for every closeout claim
- admission and post-admission violation evidence
- `forge_foundational::performance_api::lower_lane::receipts`
  counter-backed receipts and `stronger_lane::certified` certified bundles
- `forge_foundational::performance_api::stronger_lane::readiness`
  performance production-readiness closure
- `forge_foundational::profiles_api` certification posture and target-aware
  profile attachments
- `forge_foundational::boundary_evidence_api` provenance, lineage,
  completed/executed receipts, support truth, and readmission attachments
- `forge_foundational::canonicalization_api` canonical comparison,
  digest-ready evidence, and trust-boundary readmission for published bundles
- `forge_proof::prelude::*` staged progression, checked outcomes, witnesses,
  freshness, bridge, rebind, and readmission surfaces

**Warnings**

- Do not close on logs, summaries, benchmark throughput, or same-run
  self-comparison.
- Do not let certification define I/O scheduler law.
- Do not let closeout evidence mint runtime authority.
- Do not let performance reports, profile attachments, canonical digests,
  boundary evidence bundles, or Proof witnesses satisfy Store runtime
  authority.
- Do not close a QoS claim as "certified performance" unless the claim names
  the profile, resource units, counter strength, fault evidence class, and
  whether the envelope was held or violated with cause.

**Test requirements**

- Evidence replay: certification can replay S.6 pressure scenarios and produce
  equivalent transcripts, oracle verdicts, performance receipts, and support
  posture from the same executed evidence.
- Evidence denial: certification-only rows, performance reports, matrix
  projections, and copied counters cannot satisfy runtime scheduler admission
  or later-readiness constructors.
- Counter proof: foreground wait, background yield, queue-depth, flush, sync,
  bandwidth, backpressure, security-denial, and unsupported-claim counters are
  present with the strength required by each phase.
- Violation proof: `ExecutionViolated`, `BackendContradictedWitness`,
  `EnvelopeExceeded`, and `PolicyDebtIncurred` outcomes are materialized and
  cannot be hidden by successful completion.
- API adoption proof: ordinary S.6 evidence uses the named Foundational
  performance, profile, boundary-evidence, and canonicalization lanes plus
  Proof witnesses, checked outcomes, freshness, bridge, rebind, and
  readmission surfaces, without replacing Store authority.

**Engineering decisions**

- Certification is the courtroom and consumes lower Store execution evidence.
- Evidence materialization includes focused runtime tests, S.4.5 harness
  scenarios, compile-fail boundary tests, and counter-backed performance
  receipts.
- Materialized evidence distinguishes admitted, unsupported, unavailable,
  stale, degraded, denied, and residual-debt posture.
- Materialized evidence distinguishes admission denial from post-admission
  violation; both must carry causal attribution and counter strength.
- Evidence materialization must prove the Foundational/Proof usage lock: no
  performance report,
  profile attachment, canonical digest, boundary evidence bundle, Proof
  witness, or certification row can satisfy Store runtime authority by itself.

**Open questions**

- None.

### Phase 14: Production Readiness Closure And Later-Milestone Admission

Close S.6 by turning the materialized evidence from Phase 13 into readiness
closure artifacts that later milestones can consume without reopening S.6 I/O
law or inheriting unmade claims.

**Relevant subsystems**

- `forge-store-certification`
- `forge-store-readiness`
- `forge-store-physical-certification`
- `forge-store-io-scheduler`
- `forge-store-physical-backend`
- `forge-foundational`
- `forge-proof`

**Relevant APIs**

- materialized S.6 evidence from Phase 13
- S.7/S.10/S.11 handoffs from Phase 12
- cross-backend qualification matrix rows from Phase 11
- `forge_foundational::performance_api::stronger_lane::readiness`
  performance production-readiness closure
- `forge_foundational::profiles_api` production-readiness and certification
  posture surfaces
- `forge_foundational::boundary_evidence_api::stronger_lane::readiness`
  support/readiness closure surfaces
- `forge_proof::prelude::*` checked readmission and readiness progression for
  published closure artifacts

**Warnings**

- Do not skip unsupported and degraded backend profiles just because the
  admitted profile passes.
- Do not let readiness closure strengthen evidence that Phase 13 did not
  materialize.
- Do not let S.6 readiness claim S.7 blob lifecycle, S.10 backup/repair, or
  S.11 security correctness.
- Do not close the milestone with residual debt that is not named, scoped, and
  mechanically denied from platform-grade posture.

**Test requirements**

- Readiness replay: independently closing readiness from the same Phase 13
  evidence produces equivalent readiness artifacts, support posture, residual
  debt, and later-handoff availability.
- Readiness denial: missing Phase 13 evidence, unsupported backend posture,
  copied matrix rows, raw counters, terminal projections, and certification
  summaries cannot close S.6 readiness.
- Later-admission proof: S.7, S.10 backup/export, S.10 repair, and S.11
  secure-I/O consumers can admit only their specific S.6 handoff and cannot
  substitute another handoff.
- Residual-debt proof: unsupported, unavailable, degraded, denied, stale, and
  rebind-required claims remain visible in readiness closure and cannot be
  promoted to platform-grade posture.

**Engineering decisions**

- Readiness closure consumes Phase 13 evidence; it does not recompute scheduler
  law, backend capability, queue admission, or counter truth.
- Each readiness artifact has private fields, narrow read accessors, and
  compile-fail coverage against external minting.
- Later milestones consume sealed readiness handoffs, not certification
  reports, matrix rows, or Foundational bundles alone.
- Certification closes S.6 only after runtime authority, evidence publication,
  performance readiness, backend matrix, later handoffs, and non-claim
  boundaries agree.

**Open questions**

- None.

## Must Ship

- Backend/media capability admission for buffered, mmap, direct-I/O-shaped, and
  optional async-I/O-shaped profiles.
- Admitted assumptions for fsync/fdatasync, directory sync, durable rename,
  alignment, sector atomicity, page-cache behavior, and flush ordering.
- Foreground lane contracts, reservation receipts, and profile-scoped latency
  envelope declarations.
- Background pacing for compaction, checkpointing, scrub, replication
  preparation, blob ingest, blob migration, backup preparation, and repair
  pressure.
- Queue-depth, read-ahead, write-back, write-grouping, bandwidth, and
  backpressure execution plans.
- Flush/sync/durable-rename progression and counters.
- Page-cache, mmap, and direct-I/O access policy admission.
- Trim, punch-hole, and cold-tier I/O posture admission that consumes S.5
  protected-footprint evidence without claiming S.7/S.10 lifecycle semantics.
- S.5.1 security-scope preservation and secure-I/O unsupported posture denial
  throughout scheduler, queue, flush, and evidence paths.
- S.4.5 I/O pressure harness extension with production-boundary yieldpoints,
  observers, oracles, transcripts, generated coverage rows, and counter-backed
  evidence.
- Cross-backend qualification matrix with explicit unsupported, unavailable,
  stale, degraded, denied, and residual-debt posture.
- Separate later-readiness handoffs for S.7, S.10 backup/export, S.10 repair,
  and S.11 secure-I/O foundation.
- Certification evidence materialization that includes named Foundational
  performance counter receipts, profile attachments, boundary-evidence support
  truth, canonical comparison/readmission evidence, and Proof checked
  progression evidence for the phase contracts.
- Production readiness closure artifacts that consume materialized evidence,
  preserve non-claims, and expose only sealed later-milestone handoffs.

## Must Preserve

- S.5 owns physical read stability; S.6 consumes it.
- S.5.1 owns security-scope admission; S.6 preserves and denies based on it.
- S.6 owns I/O admission, scheduling, pacing, backend/media capability, and
  interference proof.
- S.7 owns blob lifecycle. S.10 owns backup/repair semantics. S.11 owns
  encryption, key lifecycle, identity integration, audit, and operator
  authorization.
- Backend support posture may affect which durability claims are available. It
  may not silently redefine the meaning of an already-admitted durability
  claim.
- Foundational carries shared evidence, profile, canonical, support, and
  performance meaning at publication/support boundaries; it does not replace
  Store-owned I/O authority.
- Proof encodes legal progression, witnesses, freshness, readmission, fixed
  shape, and checked outcomes; it does not become the runtime scheduler.
- JSON/serde/terminal projections remain hostile/readmission or terminal-only
  and cannot satisfy I/O authority.

## Acceptance Evidence

- Compile-fail tests proving raw backend labels, raw lane labels, terminal
  projections, logs, copied counters, copied S.5/S.5.1 handoff fields, matrix
  rows, and certification-only evidence cannot satisfy runtime I/O authority.
- Runtime tests proving foreground reservation, background pacing, queue
  admission, flush progression, page-cache/mmap/direct-I/O access policy,
  trim/punch-hole/cold-tier I/O posture, and security-scope preservation.
- Resource-unit tests proving reservations, pacing, borrowing, revocation,
  denial, and violation receipts name concrete S.6 resource units.
- Foreground fairness tests proving foreground reads, foreground writes,
  commit-critical WAL writes, point reads, range reads, interactive reads, and
  internal foreground reads cannot starve or launder priority through one
  another.
- S.4.5 simulation suites for foreground read/write under compaction,
  checkpoint, scrub, replication preparation, blob ingest, blob migration,
  backup preparation, and repair pressure.
- Fault-injection suites for delayed fsync/fdatasync, directory sync failure,
  queue saturation, bandwidth throttle, backend latency spike, page-cache
  pressure, and unsupported direct-I/O or secure-I/O posture.
- Cross-backend qualification tests for buffered, mmap, direct-I/O-shaped, and
  async-I/O-shaped profiles, including unsupported and degraded cases.
- Exact or policy-declared counters for foreground wait, background yield,
  queue depth, backpressure, flush, sync, bandwidth, page-cache interaction,
  trim/punch-hole, security denial, unsupported capability, and residual debt.
- Counter-strength evidence proving exact, monotonic, sampled, bounded,
  attribution, and diagnostic-only counters cannot substitute for one another.
- Post-admission violation evidence proving admitted plans can report
  execution violation, backend witness contradiction, envelope exceedance, and
  policy debt.
- Named Foundational `performance_api`, `profiles_api`,
  `boundary_evidence_api`, and `canonicalization_api` evidence exactly where
  the phases require them.
- Proof checked progression evidence for capability admission, reservation,
  pacing, queue execution, flush ordering, security readmission, and handoff
  consumption without allowing witnesses to substitute for Store authority.
- Separate S.7, S.10 backup/export, S.10 repair, and S.11 handoff artifacts
  proving later milestones consume S.6 without inheriting unmade claims.
- Readiness closure evidence proving materialized certification evidence,
  backend matrix rows, and later handoffs agree without strengthening any
  unsupported, degraded, denied, stale, or residual-debt posture.

## Sequencing Notes

S.6 belongs after S.5 and S.5.1 because foreground I/O protection is meaningful
only after physical reads are stable and security scope survives physical
paths. It belongs before S.7 because native blobs need chunk streaming and
dedupe under honest queue pressure. It belongs before S.10 because backup,
PITR, export, repair, and forensics need explicit I/O pressure and durability
assumptions. It belongs before S.11 because secure I/O posture and unsupported
encrypted/authenticated-frame readiness must exist before full key lifecycle
and encryption work.

Internally, S.6 must proceed in this order:

- Capability admission comes before every scheduler phase because backend/media
  assumptions define what can be claimed.
- Foreground reservations come before background pacing because background work
  can yield honestly only after the protected foreground envelope exists.
- Queue admission follows reservation and pacing because execution must consume
  lowered plans rather than rediscover policy.
- Flush durability follows queue admission because sync and durable rename are
  execution/ordering facts, not raw backend labels.
- Page-cache/mmap/direct-I/O access policy comes before trim/punch-hole/cold
  movement because reclaim/tier movement must consume admitted access and
  buffer-lifecycle posture.
- Security-scope preservation follows the physical I/O policy phases so every
  scheduler, queue, flush, access, and reclaim path can carry S.5.1 scope.
- Interference counters follow execution-policy phases because counters must
  explain the actual admitted paths.
- Harness extension follows counters so scenarios can assert causal evidence,
  not just outcomes.
- Backend qualification follows the harness because matrix rows need executed
  evidence, not backend declarations.
- Later handoffs follow qualification because they must expose exactly which
  S.6 claims are supported, denied, degraded, or unavailable.
- Certification evidence materialization follows handoffs so the courtroom can
  prove both runtime authority and publication surfaces.
- Readiness closure is last because it consumes materialized evidence and must
  not strengthen anything that the evidence did not prove.

## Known Risks

- Backend capability admission may overstate what can be proven inside the
  process unless evidence classes and rebind triggers are enforced strictly.
- Foreground latency envelopes may need profile-specific numeric policy before
  implementation can choose exact default limits.
- Foreground-vs-foreground arbitration may expose more lane classes than the
  first implementation wants to support; unsupported classes must deny rather
  than collapse into generic foreground work.
- Mmap/direct-I/O/buffered coherence may require stricter access-mode
  separation on some backends than the initial policy allows.
- Mmap fault behavior may require backend-specific safety posture before it can
  be platform-grade.
- Trim/punch-hole/cold-tier posture may need conservative denial on backends
  whose sparse-file, copy-on-write, checksum, or hole semantics are ambiguous.
- Fault injection proves scheduler behavior under modeled pressure; it does
  not prove backend reality unless the qualification matrix carries real
  backend evidence.
- Counter exactness may impose hot-path cost unless counter strength and
  diagnostic materialization are profile-gated.
- Background borrowing can become hidden interference if revocation and debt
  counters are not exact enough.
- S.6 readiness can be misread as universal performance certification unless
  every readiness artifact preserves profile, resource-unit, and non-claim
  boundaries.
