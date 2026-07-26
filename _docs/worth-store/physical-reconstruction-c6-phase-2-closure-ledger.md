# C.6 Phase 2 Closure Ledger

This ledger is the authority for closing
`physical-reconstruction-c6-buffer-pool-runtime-join.md` Phase 2. It does not
close Phase 3 or later C.6 work.

## Audit source freeze

- base commit: `f617cdd8ee34e3dc5d8ff6ec65bf93aaedb60b73`
- tracked diff SHA-256:
  `6d9af0214d5d3274dfac3a8c84bef0befd0a484e867c844a87fe7977d38ed795`
- tracked path count: `159`
- untracked path/blob manifest SHA-256:
  `463b21519274abf401fe13f592a03e0c6fd8bed80c285c5f3c519255ae8f1809`
- untracked path count: `53`
- tracked methodology: SHA-256 over the UTF-8, LF-normalized output of
  `git diff HEAD --no-ext-diff --binary --full-index`, excluding this ledger.
- untracked methodology: SHA-256 over the UTF-8, LF-terminated, path-sorted
  manifest whose rows are `<git hash-object blob id><TAB><path>`, excluding
  this ledger.
- scope includes the complete dirty WORTH Store graph; excluding this ledger
  prevents a self-referential hash, and unrelated user changes remain
  preserved.

## Guarantees

| ID | Closure claim | Required evidence | Current result |
| --- | --- | --- | --- |
| C6P2-01 | Store admits every named global, category, kind, count, and scope limit through the exact declaration/builder/admitted-policy API, with page-shape preflight and no scalar/default bypass. | Policy unit tests, Store constructor UI tests, source and consumer absence checks. | `PROVED` — all four policy tests, Store constructor UI coverage, all-feature compilation, and obsolete scalar/default absence checks pass. |
| C6P2-02 | Every governed pool allocation requires authority from the exact live pool incarnation; a raw scope, forged marker, foreign pool, or consumed grant opens no admission path. | Compile-fail proofs, same-Store foreign-incarnation attacks, constructor and call-path inspection. | `PROVED` — 34 lower compile-fail doctests, same-Store foreign-incarnation attacks, six candidate-residency tests, and constructor/call-path gates pass. |
| C6P2-03 | One private accounting authority owns all hard-dimension current/peak mutation and fixed allocation-event publication; counters cannot grant authority. | Direct-mutation absence gate, independent event/counter reconciliation, limit and one-past tests. | `PROVED` — direct-mutation gates, all six allocation-event tests, all-scope/all-kind reconciliation, exact limits, and one-past denials pass. |
| C6P2-04 | Clean-to-dirty replacement reserves dirty and total replacement bytes before internal allocation and releases them on allocator failure, fill failure, ordinary error, drop, and unwind; no external owning replacement `Vec` compiles. | Runtime failure/release tests, concurrency ceiling test, compile-fail proofs, raw-API absence check. | `PROVED` — allocator rejection, fill failure, ordinary error/drop, unwind, exact competing-ceiling, compile-fail, and raw-API absence proofs pass. |
| C6P2-05 | Public read and append pressure is Store-generation evidence naming basis, scope, dimension, requested/admitted/limit, retry posture, and pre-effect status; the old scalar observation API is absent. | Projection tests, retry lattice tests, production pressure journey, public-source absence checks. | `PROVED` — the public Store read/append journey proves every named field and unchanged media state; retry-lattice and public-source absence gates pass. |
| C6P2-06 | One non-cloneable Store-instance residency owner covers initialize, open, construction failure, abort, close, and abandoned construction; facade clones cannot suppress close. | Exhaustive destructuring/compiler evidence, owner Drop/close tests, lifecycle integration tests. | `PROVED` — exhaustive ownership/compiler gates, both owner close/drop tests, lifecycle failure paths, and the full Store suite pass. |
| C6P2-07 | Superseded scalar/default, bare budget-denial, raw replacement, and loose lifecycle-owner surfaces no longer compile and all consumers use their canonical replacement. | Repository-wide symbol searches, removal ledger, compile-fail evidence. | `PROVED` — repository-wide production/test-crate absence scans, adversarial compile-fail evidence, and the validated 106-row future-removal inventory pass. |
| C6P2-08 | The lower buffer-pool graph remains free of Signal, `worth-proof`, Foundational, aspect-native, replay, and semantic-policy authority. | Cargo metadata/source gates and mandatory boundary checker. | `PROVED` — lower Cargo/source dependency scans are empty, audience ownership remains at Store, and the mandatory boundary checker passes. |
| C6P2-09 | Tests are independent production evidence: real allocation boundaries, exact oracles, adversarial authority attacks, failure release, and no redundant compile theater. | `qa-tests` review, focused suites, full lower/Store suites, mutation/source gates. | `PROVED` — `qa-tests` review closed F05/F06/F07/F09/F10/F11/F12/F15; 109 lower tests, the full Store suite, 143 runner tests plus two CLI tests, doctests, and controlled mutants pass. |
| C6P2-10 | Destination topology, public API documentation, cleanup ledger, roadmap handoff, line caps, formatting, context generation, and boundary checks match the final source state. | `code-quality-qa`, documentation inspection, final hashes and mandatory gates. | `PROVED` — public docs and roadmap are current; cleanup inventory validates; 196 dirty Rust files have zero cap/catch-all violations; 79 scrutinized functions have zero scan errors; formatting, diff hygiene, context generation, boundary checks, and this source freeze pass. |

## Findings

### F01 — Obsolete bare budget denials still compile

- Severity: high; invalidates `C6P2-07` and weakens `C6P2-05`.
- Evidence: repository search found
  `ResidentBudgetExhausted`, `FrameEntryBudgetExhausted`,
  `PinnedFrameBudgetExceeded`, `PinLeaseBudgetExceeded`,
  `DirtyFrameBudgetExceeded`, and `SpeculativeFrameBudgetExceeded` in the
  lower public denial enum. The C.6 pressure courtroom and inheritance journey
  still assert `PinLeaseBudgetExceeded`.
- Governing requirement: Phase 2 cleanup removes budget vocabulary not
  consumed by execution; the milestone cleanup law says a replacement is not
  complete while its predecessor compiles.
- Root correction: delete all six variants and update the courtroom protocol
  and journey to assert the exact pressure dimension/scope/current/limit.
- Closure proof: production source search has no occurrence outside deliberate
  compile-fail prose; the pressure courtroom and inheritance journey pass with
  exact evidence.

### F02 — Production trace named the pre-grant pool load

- Severity: medium; invalidated the checked-in production-boundary trace.
- Evidence: `checked_in_trace_resolves_to_the_real_production_sources` rejected
  the stale `.load(key` anchor after pool loads became grant-bearing.
- Root correction: the trace now anchors `.load(allocation`, preserving the
  governed load boundary rather than weakening the source gate.
- Closure proof: the exact production-trace test passes.

### F03 — Pressure projection test bypassed the sole Store constructor

- Severity: high; invalidated `C6P2-06`.
- Evidence: the runtime-ownership gate found a second
  `PhysicalResidencyPool::open` in `pressure_evidence.rs`.
- Root correction: the test now enters through `RecordFramePorts::bounded`,
  which owns the single production pool constructor, and still elicits a real
  lower-layer pressure denial.
- Closure proof: the exact constructor-authority test passes.

### F04 — Candidate boundary gate rejected proof-carrying residency

- Severity: high; weakened `C6P2-02` by failing to recognize the lifetime on
  the grant-borrowing candidate session.
- Evidence: the candidate/publication gate required the obsolete literal
  `fn begin(` and rejected `fn begin<'allocation>(`.
- Root correction: the gate now requires an exact
  `OperationAllocationGrant` borrow and the same `'allocation` lifetime on the
  returned residency session while continuing to reject submission authority.
- Closure proof: the exact writer-boundary test and the 140-test runner suite
  pass.

### F05 — Dirty replacement failure proof is incomplete

- Severity: high; keeps `C6P2-04` and `C6P2-09` open.
- Evidence: source tracing confirms release-on-drop and release-on-fill-error,
  but no deterministic test proves allocator failure after grant, panic unwind,
  or a competing replacement at the exact replacement-byte ceiling.
- Root correction: allocation is injected through the always-compiled
  crate-private `DirtyReplacementAllocator` port. The rejecting allocator,
  `catch_unwind` case, and held-reservation ceiling case all traverse the real
  grant, internal allocation, release, and observation progression.
- Closure proof: the three adversarial tests prove allocator failure precedes
  fill, unwind returns every counter and event cell to zero, the clean source
  remains unchanged, and a competing replacement is denied at the exact
  eight-byte ceiling.

### F06 — Allocator failure proof used test-only production semantics

- Severity: critical test-integrity defect; invalidates the allocator portion
  of `C6P2-04` and keeps `C6P2-09` open.
- Evidence: the first correction for F05 inserted a thread-local
  `#[cfg(test)]` rejection branch directly into `lease.rs`.
- Governing requirement: Testing Law 25 forbids test-only production branches
  and privileged mutation backdoors.
- Root correction: the thread-local `#[cfg(test)]` branch was deleted.
  Production uses `ProcessDirtyReplacementAllocator`; the rejecting test
  substitute implements the same always-compiled private port and cannot alter
  reservation, classification, release, or observation semantics.
- Closure proof: source inspection finds no test-conditional allocator branch
  or hidden allocator constructor, and the complete lower library suite passes.

### F07 — Public pressure evidence lacks a production journey

- Severity: high; keeps `C6P2-05` and `C6P2-09` open.
- Evidence: lower pressure projection is locally tested, and the C.6
  inheritance journey observes a lower handoff denial, but no test reaches
  `RecordReadError::pressure()` or `RecordAppendError::pressure()` through the
  public Store facade.
- Root correction: one scenario in the existing
  `physical_record_journeys` integration target creates a causally valid
  store, holds a real read allocation to force a second public read denial, and
  forces an oversized public append allocation. It asserts basis,
  generation, scope, dimension, requested/admitted/limit, retry posture,
  pre-effect status, and unchanged media counters.
- Closure proof:
  `physical_work::residency_pressure_projection::public_read_and_append_pressure_retains_exact_pre_effect_basis`
  executes through the production Store and passes with every named oracle.

### F08 — Record-open pressure discards an available record basis

- Severity: high production API defect; invalidates semantic sharpness in
  `C6P2-05`.
- Evidence: `PhysicalRecordReader::open(record, ...)` calls
  `begin_read_allocation`, whose denial path uses Store-only `read_error`
  despite already possessing the exact `PhysicalRecordId`.
- Root correction: allocation denial for a known-record open now uses the
  record-bound projection path. The public integration journey asserts that
  exact record in `PhysicalRecordPressureBasis`.
- Closure proof: the public journey observes the requested second record in
  the read basis and Store-only basis for append, with the admitted generation
  and no media effect.

### F09 — Policy completeness test uses a production-only omission backdoor

- Severity: critical test-integrity defect; keeps `C6P2-01` and `C6P2-09`
  open.
- Evidence: `PhysicalResidencyLimitsBuilder::omit_for_test` exists only under
  `#[cfg(test)]` and mutates the production builder into an incomplete state.
- Governing requirement: Testing Law 25 forbids test-only production branches
  and privileged mutation backdoors.
- Root correction: the method was deleted. The test fixture constructs each
  incomplete declaration with the ordinary public builder while deliberately
  skipping exactly one setter.
- Closure proof: the four policy tests pass and repository source search finds
  no `omit_for_test`.

### F10 — Pressure unit test fabricates lifecycle generation

- Severity: high test-integrity defect; keeps `C6P2-05` and `C6P2-09` open.
- Evidence: `LifecycleGeneration::for_test` is a hidden constructor used by a
  local projection test.
- Root correction: both were deleted. The production integration journey now
  obtains generation through real Store admission and proves both public error
  projections, making the fabricated local world redundant.
- Closure proof: source search finds no `LifecycleGeneration::for_test`; the
  exact public pressure journey passes.

### F11 — Lower tests publish dirty frames without backend receipts

- Severity: critical authority and test-integrity defect; weakens `C6P2-02`,
  `C6P2-04`, and `C6P2-09`.
- Evidence: `DirtyPhysicalFrame::publish_clean_for_pool_test` bypasses the
  sealed `CompletedArtifactRangeWrite` required by production.
- Root correction: tests not proving publication use the real
  `discard_candidate` transition. Clean identity tests begin from a real
  clean load. Receipt-governed publication remains proved at the Store/backend
  integration boundary; the receipt-free method is deleted.
- Closure proof: source search finds no
  `publish_clean_for_pool_test`; the lower 107-test library suite and the real
  foreign-receipt rejection test pass.

### F12 — Allocation-event reconciliation does not activate every fixed cell

- Severity: high evidence defect; keeps `C6P2-03` and `C6P2-09` open.
- Evidence: the existing reconciliation test activates only
  `ForegroundWrite` and no speculative grant, so incorrect indexing for the
  other six scopes or any of the three speculative kinds can remain green.
- Root correction: one dense local scenario holds every scope and kind
  active at once, establishes real dirty posture before write-behind admission,
  reconcile each event cell to counters, then prove exact release and
  completion.
- Closure proof: the six allocation-event tests pass, including exact
  reconciliation of all seven operation scopes and all three speculative
  kinds while active and after release.

### F13 — Append pressure classification carries the evidence payload

- Severity: high public-contract defect; keeps `C6P2-05` and `C6P2-09` open.
- Evidence: the normative API declares the unit classification
  `RecordAppendDenial::PhysicalPressure` and exposes evidence through
  `RecordAppendError::pressure()`, but the implementation stores
  `PhysicalRecordPressureEvidence` inside the denial variant.
- Governing requirement: pressure evidence is Store-owned observation, not
  denial authority, and the read and append APIs must expose the same sharp
  classification/evidence split.
- Root correction: the append denial variant is unit-like, the evidence is
  stored on the append error boundary, and the public production
  journey proves both the unit classification and exact evidence.
- Closure proof: Store all-target/all-feature compilation passes; the exact
  public journey executes one test and passes; source inspection finds no
  evidence-bearing `PhysicalPressure` denial variant.

### F14 — Public record denials expose lower buffer-pool vocabulary

- Severity: critical public-boundary defect; invalidates `C6P2-05` and
  `C6P2-07`.
- Evidence: `RecordReadDenial::ResidencyUnavailable` and
  `RecordAppendDenial::ResidencyUnavailable` publicly carry
  `worth_store_buffer_pool::PhysicalResidencyDenial`. The milestone cleanup
  contract explicitly requires replacing those payloads with Store-owned
  classification and `PhysicalRecordPressureEvidence`.
- Governing requirement: the Store facade owns record-facing meaning; a
  lower physical mechanism cannot become public API authority.
- Root correction: `PhysicalRecordResidencyFailure` is an opaque Store-owned
  wrapper with an exhaustive conversion to
  `PhysicalRecordResidencyFailureKind`. Read, append, and bootstrap denials
  carry only that Store type; pressure is projected to the unit classification
  plus Store basis/generation evidence.
- Closure proof: warnings-denied Store all-target/all-feature compilation
  passes, affected runtime and integration tests pass, and the record-facing
  absence gate finds no public lower pool payload.

### F15 — Candidate physical-write authority admits impossible optional states

- Severity: critical authority and test-integrity defect; invalidates
  `C6P2-02` and keeps `C6P2-09` open.
- Evidence: `CandidateFramePhysicalWrite` stores its backend receipt and Store
  settlement as independent `Option`s, then checks both at publication time.
  `for_contract_test` constructs the impossible neither-present state so local
  tests can simulate a successful physical effect without crossing a backend.
- Governing requirement: proof-bearing progression must make illegal states
  unrepresentable; Testing Law 25 forbids hidden constructors and tests that
  open production doors unavailable to real callers.
- Root correction: `CandidateFramePhysicalWrite` is constructed only from a
  concrete `CompletedArtifactRangeWrite` and matching
  `CanonicalRecordMutationSettlement`; optional proof fields, runtime
  missing-proof branches, and both `for_contract_test` constructors were
  removed. Fake-success tests were deleted as redundant; independent
  pre-effect and failure-fate tests remain.
- Closure proof: six candidate-residency tests, the real Store append journey,
  warnings-denied Store compilation, the source absence gate, and both
  candidate-write-evidence runner tests pass. Controlled optional-proof and
  hidden-constructor mutants are rejected.

### F16 — Allocation-event tests collapse distinct invariant families

- Severity: blocking structural defect; invalidates `C6P2-10`.
- Evidence: `physical_residency/tests/allocation_events.rs` is 453 lines and
  mixes global dimension reconciliation, dirty-replacement failure release,
  and fixed scope/speculation cell indexing in one file.
- Governing requirement: the 400-line cap is hard, and Composition Laws 1, 8,
  and 14 require tests to be organized by one invariant family rather than by
  the generic fact that they observe allocation events.
- Root correction: `tests/allocation_events/` now has sibling
  modules for hard-dimension reconciliation, dirty-replacement release, and
  fixed-cell reconciliation; keep only genuinely shared event oracles at the
  parent radius.
- Closure proof: the files are 106, 189, and 65 lines with a 96-line parent
  facade; the complete 109-test lower suite passes and the dirty-tree line-cap
  inventory reports zero files over 400 lines.

### F17 — Residency tests retain a milestone bucket name

- Severity: high structural and cleanup defect; invalidates `C6P2-10`.
- Evidence: `physical_residency/tests/c6_readiness.rs` contains one
  candidate-window progression test and one pin/writeback exclusion test.
  Neither responsibility is “C6 readiness,” and the milestone cleanup contract
  explicitly requires renaming or splitting this file by the law proved.
- Governing requirement: Domain Structure Laws 4 and 5 forbid milestone names
  and mixed activity buckets where authority/failure responsibilities differ.
- Root correction: the tests moved to responsibility-named
  `candidate_window.rs`, `pin_lease_pressure.rs`, and
  `writeback_claim_exclusion.rs` modules, and `c6_readiness.rs` was deleted.
- Closure proof: `c6_readiness.rs` is absent, the three independently named
  tests pass, each failure now localizes to its actual residency law, and the
  completed cleanup paths are absent from the future-removal ledger.

### F18 — Frame-space admission hides policy, eviction, and denial phases

- Severity: high structural defect; weakens `C6P2-03` and invalidates
  `C6P2-10`.
- Evidence: the 83-line `reserve_frame_space` body validates independent pin
  ceilings, decides whether space is required, mutates eviction state, and
  constructs three different terminal pressure demands inside one loop.
- Governing requirement: Composition Laws 2, 3, 5, and 6 require named policy
  facts and visible validation/effect/failure phases.
- Root correction: frame-space reservation is an orchestrator over named
  pin-capacity admission, space-requirement classification, one-frame eviction,
  and terminal pressure classification steps.
- Closure proof: function scrutiny no longer reports `reserve_frame_space`;
  all 30 physical-residency tests pass with warnings denied.

### F19 — Writeback claim mixes request, capacity, residency, and mutation

- Severity: high structural and authority defect; weakens `C6P2-03` and
  invalidates `C6P2-10`.
- Evidence: the 105-line `claim_writeback` body allocates and validates a
  request, admits write-behind capacity, validates every resident frame,
  collects bytes, and marks claims without named phase boundaries.
- Governing requirement: Composition Laws 2, 5, and 6 require preparation and
  validation to complete before claim mutation, with each denial phase
  structurally locatable.
- Root correction: `claim_writeback` now orchestrates named request validation,
  speculative-capacity admission, resident-byte collection, and
  claim-application steps; only the final step sets `writeback_claimed` or
  accounting state.
- Closure proof: function scrutiny no longer reports `claim_writeback`; the
  exact writeback exclusion and complete physical-residency suites pass.

### F20 — Retained-candidate validation passes an unnamed fact bundle

- Severity: medium composition defect; invalidates `C6P2-10`.
- Evidence: `verify_retained_frame` accepts six arguments even though five are
  one immutable expectation captured before candidate residency consumes the
  frame.
- Governing requirement: Composition Laws 3 and 4 require related validation
  facts to have a semantic name instead of a dishonest raw-argument signature.
- Root correction: `RetainedFrameExpectation` contains
  declaration, role, coordinate, byte length, and checksum; validation consumes
  that fact plus the resident frame.
- Closure proof: function scrutiny no longer reports the validation signature,
  warnings-denied Store compilation and six candidate-residency tests pass.

### F21 — Store residency observation leaks lower snapshots and omits its basis

- Severity: critical public-API defect; invalidates `C6P2-05`, `C6P2-08`, and
  `C6P2-10`.
- Evidence: `PhysicalResidencyObservation` contains only lifecycle generation,
  lower `PhysicalResidencyCounters`, and lower allocation-event snapshot. It
  omits stable Store identity and admitted limits, while public getters expose
  `worth-store-buffer-pool` types directly.
- Governing requirement: the normative API requires a Store-owned, read-only,
  identity-bound observation carrying admitted limits and exact counters; the
  lower pool remains a mechanism, not the public API authority.
- Root correction: `residency/residency_observation/` now separates the
  Store-owned facade, counter snapshot, allocation snapshot, and per-dimension
  event snapshot by responsibility. `PhysicalResidencyOwner` retains the exact
  stable Store identity and admitted Store policy used to construct the pool,
  so every observation carries that basis by construction. Public Store
  signatures return only Store-owned snapshot types; lower snapshots remain
  private wrapper state.
- Closure proof: both owner close/drop tests pass through honest Store policy
  admission; the exact public read/append pressure journey proves Store
  identity, admitted policy, counter reconciliation, and allocation-event
  identity; and the runner source gate accepts the real facade while rejecting
  controlled lower-snapshot return and public-field mutants. All focused
  proofs pass with all features and warnings denied.

### F22 — Operation accounting exceeds the hard line cap and mixes writeback lifecycle

- Severity: high composition defect; invalidates `C6P2-10`.
- Evidence: `physical_residency/pool/operation_accounting.rs` reached 410
  lines and combined operation-byte admission, speculative-frame accounting,
  and writeback claim validation/completion/release.
- Governing requirement: the workspace hard cap is 400 lines, and files must
  own one named semantic responsibility rather than absorb adjacent lifecycle
  work.
- Root correction: writeback claim validation, resident collection, capacity
  admission, application, completion, and release moved to the
  expected-growth `pool/writeback_claim.rs` module. Operation and speculative
  accounting remain in `operation_accounting.rs`.
- Closure proof: the files are 184 and 219 lines respectively; all 109 lower
  library tests pass with all features and warnings denied; the complete dirty
  graph reports zero Rust files over 400 lines and zero catch-all filenames;
  function scrutiny reports 79 advisories and zero scan errors.

## Closure rule

Phase 2 closes only when every row is `PROVED` or justified `N/A`, all findings
remain recorded with their final correction evidence, and final checks run
against a new source freeze. No green subset test can override an open ledger
row.
