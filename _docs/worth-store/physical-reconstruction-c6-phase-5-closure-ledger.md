# C.6 Phase 5 Closure Ledger

## Scope

This ledger audits C.6 Phase 5, **Join Dirty State And Writeback To Ordinary
Publication**, against the governing C.6 specification, the WORTH engineering
laws, the actual Store, Signal, scheduler, executor, backend, physical-format,
and buffer-pool boundaries, and defects that could satisfy the phase wording
while defeating its intent.

Phase 6 and later remain blocked. This ledger does not claim canonical
prefetch/read-ahead/write-behind lowering, final ordinary-consumer cutover,
complete S.2 deletion, public feature documentation, or full C.6 closeout.
Phase 5 cleanup is in scope exactly where Phase 5 makes a dirty/writeback
predecessor obsolete. Unreleased-product code is deleted or cut over; no
compatibility or legacy lane is created.

## Current Authority

This is a living audit. A row is `PROVED` only for the exact source freeze and
evidence named here. Existing tests and reports begin as candidate evidence.
Any correction reopens every row whose source, assumptions, generated
artifacts, or downstream behavior changed.

The audit is closed against the exact `Final Source Freeze` below. Every
closure row is proved by evidence bound to that freeze, every recorded finding
is corrected, and no known in-scope defect remains. Any non-ledger source
change invalidates this closure and requires a new final freeze and affected
evidence.

## Audit-Start Source Freeze

- base commit: `95afd3a9ac80967d9a31ce75a80cad98af8c0604`
- dirty entries excluding this ledger: `145`
- tracked entries: `121`
- untracked entries: `24`
- manifest bytes: `21,340`
- path/status/blob manifest SHA-256:
  `0faee49aaee2295fdef5130e212d344bff4f6c7b88600195dce3b4521be25205`
- row shape: `<two-column Git status><TAB><Git blob id or
  DELETED><TAB><porcelain path>`
- source: `git status --porcelain=v1 --untracked-files=all`
- ordering: Git porcelain order
- encoding: UTF-8 without BOM, one LF after every row
- rename hashing: hash the destination path while retaining the complete
  porcelain path in the manifest row
- self-exclusion: only this ledger
- independent reproduction: PowerShell/.NET using `git hash-object
  --no-filters` and Python/hashlib using raw Git blob framing produced the same
  entry list, counts, manifest bytes, and digest

Unrelated dirty work is preserved. Closed C.6 Phase 1-4 work and inherited
C.5.1 work are part of the current source authority; this ledger does not
reclassify them as Phase 5 changes.

## Final Source Freeze

- base commit: `95afd3a9ac80967d9a31ce75a80cad98af8c0604`
- dirty entries excluding this ledger: `229`
- tracked entries: `186`
- untracked entries: `43`
- manifest bytes: `33,914`
- path/status/blob manifest SHA-256:
  `b593761ac3a95ff89e6869ded85af2c4a39aac7241be262c57b80cf0311beafa`
- schema, ordering, encoding, rename handling, and self-exclusion are exactly
  those declared by the audit-start freeze
- independent reproduction:
  PowerShell/.NET using `git hash-object --no-filters` and Python/hashlib using
  raw Git blob framing independently produced the same 229 ordered rows,
  tracked/untracked counts, byte count, and SHA-256 digest

The retained mutation and Courtroom reports live under ignored build evidence
storage and therefore do not alter this source freeze. Subsequent edits to this
ledger also do not alter it because this ledger is the sole declared
self-exclusion.

## Boundary Brief

### Adversarial constraint

The Phase 5 path must survive the following hostile condition:

> One current clean frame has an immutable lease while a second caller tries to
> mutate it; dirty capacity is saturated; a sole writeback is delayed after
> backend dispatch; cancellation and close race the delayed effect; a second
> claim and eviction are attempted; one backend attempt is proven no-effect
> and retried; another produces only a partial effect; a valid receipt from the
> wrong Store, coordinate, payload, length, work identity, or scheduler fate is
> presented; and a deliberately undersized or already-consumed operation grant
> is offered for replacement allocation. No allocation may precede or exceed
> exact grant use. No immutable observer may see in-place mutation. No dirty or
> claimed frame may be evicted. No receipt, counter, coordinate, generic
> completion, Signal outcome, or scheduler receipt may clean the frame without
> the exact Store settlement chain. Proven no-effect retains retryable dirty
> authority; partial or indeterminate effect retains dirty truth and requires
> inspection; post-dispatch cancellation cannot erase settlement; and a fresh
> process observes the final files without serialized pool or Signal state.

Cooperative success is not sufficient evidence. The proof must distinguish
pre-effect denial, safe no-effect retry, exact completion, partial effect,
indeterminate effect, stale or foreign completion, and shutdown residue.

### Truth and authority

- `worth-store-buffer-pool` solely owns resident identity, pins, candidate
  allocation posture, dirty truth, writeback-claim exclusion, eviction
  eligibility, and exact pool counters.
- `worth-store` solely joins current Store generation, record/frame meaning,
  security, the dedicated frame-writeback semantic basis, physical work
  identity, and terminal effect fate.
- Worth Signal derives readiness and owns generic cancellation, timeout, and
  retry lifecycle. It does not own dirty or clean truth.
- `worth-store-io-scheduler` owns resource admission and dispatch order. Its
  grants and counters do not settle effects.
- `worth-store-physical-backend` owns exact filesystem effects and receipts.
  A receipt is effect evidence, not Store settlement authority by itself.
- physical format owns coordinate and byte interpretation.
- publication/root truth remains distinct from pool cleanliness. C.7 alone
  will add WAL, checkpoint, and durable publication ordering.

### Weaker representations that must open no door

A Store identity, lifecycle generation, frame coordinate, work identity,
payload digest, dirty counter, writeback counter, scheduler grant, generic
backend completion, Signal settlement, Foundational fact, or certification
observation cannot construct or advance dirty/writeback authority. The
compiler-visible path must consume the concrete predecessor typestate.

### Destination topology

The current populated Phase 5 destination is:

```text
worth-store-buffer-pool/src/physical_residency/
├── lease.rs                         # narrow typestate facade
├── pool_ownership.rs                # pool owner and distinct clean capabilities
├── writeback_range_posture.rs       # existing range vs candidate tail
└── lease/
    ├── frame.rs                     # clean lease and dirty-replacement admission
    ├── dirty.rs                     # replacement reservation and candidate transition
    └── writeback.rs                 # exact claim and authorized clean transition

worth-store/src/physical_runtime/record_serving/
├── work_semantics/
│   ├── mod.rs                       # exact semantic installation facade
│   ├── publication_basis.rs         # Publication only
│   └── frame_writeback_basis.rs     # ExactWriteback only
└── residency/
    ├── mod.rs                       # narrow Store composition facade
    ├── capability.rs                # generation-fenced residency capability
    ├── frame_ports.rs               # pool-owned frame operations
    ├── scheduled_writeback.rs       # exact claim/queue range-posture join
    ├── scheduled_writeback/
    │   └── tests.rs                 # stale/missing posture attacks
    ├── candidate_frame_residency/
    │   └── writeback_progression.rs # ordinary candidate tail through C.5.1
    ├── dirty/
    │   ├── mod.rs                   # dirty/writeback facade
    │   ├── admitted_frame.rs        # Store-bound dirty authority
    │   ├── failure.rs               # retained-dirty transition failures
    │   ├── outcome.rs               # clean/retry/inspection outcomes
    │   └── writeback/
    │       ├── progression.rs       # dirty -> prepared -> ready
    │       ├── admission.rs         # ready -> scheduler-admitted
    │       └── execution.rs         # dispatched -> settled classification
    └── residency_observation/
        └── writeback.rs             # Store-owned writeback observation
```

A one-file responsibility directory remains valid when it preserves committed
growth. Phase 5 does not flatten `work_semantics/` or `dirty/writeback/` because
their authority, lifecycle, and successor insertion axes are already distinct.
It creates no empty C.7 directory.

### Intended private DX

Ordinary callers continue to append through `PhysicalRecordSubmission`; they
never receive dirty or writeback controls. The certification-only proof surface
mirrors the Store-private progression:

```rust
let dirty = residency.admit_dirty_frame(clean_lease, replace)?;
let prepared = residency.prepare_writeback(dirty, durability)?;
let ready = residency.request_writeback(prepared)?;
let admitted = residency.admit_writeback(ready)?;

match residency.execute_writeback(admitted)? {
    PhysicalWritebackExecution::Clean(settlement) => observe(settlement),
    PhysicalWritebackExecution::Retryable(retry) => schedule(retry),
    PhysicalWritebackExecution::InspectionRequired(inspection) => {
        revoke_and_inspect(inspection)
    }
}
```

Invalid skips, copies, reconstruction from scalars, generic completion, and
external construction must fail to compile. Every transition failure returns
the retained dirty authority when retry remains physically safe.

## Closure Guarantees

| ID | Exact closure claim | Required evidence | Current result |
| --- | --- | --- | --- |
| `C6-P5-L01` | The final source supporting Phase 5 closure is complete and independently reproducible across tracked, deleted, renamed, and untracked files without self-hashing this ledger. | Two independent final-source manifest implementations using the declared schema and identical per-entry blobs, counts, bytes, and digest. | `PROVED` — independent PowerShell/Git and Python/raw-blob implementations reproduce the exact 229-row, 33,914-byte final manifest and SHA-256. |
| `C6-P5-L02` | The ledger covers every Phase 5 must-ship, preserve, proof, cleanup, API, authority, lifecycle, performance, semantic, documentation-when-relevant, and causally necessary intent guarantee. | Clause-to-row coverage, exact row-to-evidence index, finding history, and a final attack asking what meaningful defect could pass every row. | `PROVED` — requirement coverage, exact evidence index, `F001`–`F032`, and the final surviving-defect attack cover the specification and the causally necessary authority, integration, failure, cleanup, semantic, and evidence guarantees. |
| `C6-P5-P01` | Candidate and copy-on-write replacement bytes are admitted before allocation under the exact `ForegroundWrite` operation grant, dirty-replacement limit, and total envelope; the grant cannot be undersized, foreign, stale, or double-spent. Every denial, allocator failure, fill failure, unwind, and success releases exact authority once. | Source trace through operation grant use and dirty reservation; allocation-event reconciliation; undersized, foreign, concurrent double-spend, allocator-failure, fill-failure, unwind, and success tests. | `PROVED` — the constructor-free pool-issued `ForegroundWriteAllocationGrant`, exact runtime denial/release cases, compile-fail specimens, and candidate-allocation gates all pass on final source. |
| `C6-P5-P02` | Mutation is exclusive or separately copy-on-write admitted: a clean frame can become dirty only from one consumed current lease, never mutates bytes visible through another immutable view, and racing pins cause a typed pre-mutation denial. | Pool state-machine trace; exact competing-pin and pin-race tests; source/pointer oracle showing original visible bytes never change; no raw in-place mutation path. | `PROVED` — deterministic pre-existing and post-reservation pin races return typed denial, preserve the original pointer/bytes, and reconcile replacement authority on final source. |
| `C6-P5-P03` | Dirty/writeback progression is move-owned and compiler-visible: clean lease -> replacement reservation -> dirty frame -> prepared -> ready -> scheduler-admitted -> dispatched/claimed -> exact settled clean, retryable dirty, or inspection required. A scalar, copied value, raw receipt, generic completion, or skipped phase cannot substitute. | Constructor/visibility inspection; compile-pass intended progression; compile-fail construction, extraction, duplication, skipped-phase, generic-completion, and lower-internal attacks; diagnostic-cause inspection. | `PROVED` — distinct pool-bound candidate/writeback clean capabilities, move-only Store typestates, 43 negative compiler specimens, and the positive ordinary/certification progressions enforce the exact sequence. |
| `C6-P5-P04` | Every effectful frame writeback follows the one C.5.1 path: dedicated Store intent -> Signal `ExactWriteback` readiness -> canonical scheduler demand -> executor -> backend receipt -> Store settlement -> pool transition. Hits and pre-effect denials create no fake work. No residency-local queue, pending map, callback registry, timer, retry loop, scheduler, executor, or direct backend route exists. | End-to-end work/effect identity trace; scheduler/media/Signal/counter reconciliation; source and dependency gates; local-topology mutant. | `PROVED` — ordinary segment/extent continuations and adversarial writeback journeys reconcile the canonical Signal/scheduler/executor/backend/settlement chain; source gates and mutant 43 reject a local scheduler. |
| `C6-P5-P05` | Cancellation, timeout, retry, partial effect, indeterminate effect, receipt mismatch, stale/foreign completion, and close preserve exact dirty/effect obligations: pre-dispatch denial performs no media work; proven no-effect alone is retryable; post-dispatch cancellation and close continue settlement; ambiguous effects never clean and require inspection. | One-axis adversarial journeys with independent media/file/counter oracles; exact typed outcomes; close-residue observations; generic cancellation preservation test. | `PROVED` — lifecycle, no-effect retry, partial/indeterminate effect, mismatch, post-dispatch cancellation, and close evidence require exact fate, recovery, Signal, file, residency, and residue outcomes. |
| `C6-P5-P06` | A dirty, loading, candidate, pinned, or writeback-claimed frame is never evictable; one frame admits at most one claim; claim drop releases claim posture without cleaning; exact counters reconcile dirty transitions, candidate publications, attempts, claims, receipts, retries, inspection, and shutdown residue. | Pool siege and claim-exclusion tests; delayed-dispatch Store courtroom; independent media/allocation observations; exact current/peak/release assertions. | `PROVED` — pool siege/claim tests and accepted Courtroom C prove one active delayed claim, competing-claim and eviction denial, continued settlement, exact receipt cleaning, and zero terminal residue. |
| `C6-P5-P07` | `store.physical.record.frame-writeback-basis` is a distinct admitted Foundational mutation contract/patch with `DependencyAndOutput`, bound only to `ExactWriteback`; `store.physical.record.publication-basis` is bound only to `Publication`. Neither basis owns dirty/clean truth, and no pool API contains Signal, Foundational, aspect-native, or `worth-proof` authority. | Exact source/profile inspection; semantic identity/family tests; dependency graph and API scans; controlled broadening/substitution failure. | `PROVED` — split semantic owners, focused substitution tests, dependency/API scans, and constitutional gates prove exact family binding and a Signal/Foundational/proof-agnostic pool. |
| `C6-P5-P08` | Ordinary product APIs expose record intent and typed outcomes only. Dirty/writeback controls remain Store-private; certification access is runtime-bound and feature-gated; external consumers cannot construct, extract, or duplicate dirty, prepared, ready, admitted, retry, settlement, or inspection authority. | Public export review; positive certification specimen; negative UI specimens and checked diagnostics; dependency gates preventing unrelated direct pool consumption. | `PROVED` — all 32 authority UI specimens, source gates, private Store composition, and constructor-free non-cloneable capability kinds reject external construction, extraction, substitution, and skipped progression. |
| `C6-P5-P09` | Real ordinary append/publication creates and settles admitted dirty candidate state against real files, while the delayed frame-writeback courtroom proves dirty-at-dispatch, claim exclusion, eviction exclusion, post-dispatch settlement, exact receipt cleaning, and fresh-process file truth without a fixture-owned authority substitute. | Production path trace from `record_submission`; real multi-process writer/observer/reopener; delayed media gate; independent physical artifact/file oracle; proof that each setup input is causally used. | `PROVED` — real ordinary segment and 65-frame extent journeys produce exact scheduled writebacks/receipts/backend identities and fresh-process reads; fault journeys and Courtroom C prove dirty pressure and settlement against real files. |
| `C6-P5-P10` | Phase 5 evidence is mutation-sensitive at the causal boundary: premature clean, exact-receipt bypass, settlement bypass, local scheduler/pending registry, cancellation erasure, skipped backend write, raw backend dispatch, and stale/foreign settlement fail their declared localized predicates. | Final 30-mutant report bound to the final source, exact nonidentity replacements, unique predicates, retained binaries, first-failure localization, and independent report/artifact validation. | `PROVED` — final IDs 15–44 are 30 nonidentity mutants killed by exact predicates; current source hashes, retained binary hashes, Courtroom embeddings, and report bindings independently validate with zero violations. |
| `C6-P5-P11` | Phase 5 cleanup leaves no raw replacement allocation before admission, duplicate dirty/writeback helper, Phase-5 temporary `C6*` dirty/writeback type, obsolete test/selector, compatibility alias, legacy bridge, or misleading evidence input. The removal ledger truthfully distinguishes Phase-5 deletions from Phase-7 and Phase-8 ownership. | Removal-ledger audit; source/metadata/path absence checks; deleted-path verification; stale selector and unused-input scans; no compatibility/deprecation machinery. | `PROVED` — removal inventory, deleted-path verification, source/selector/unused-input scans, and the 67-predicate boundary gate show no Phase 5 predecessor or compatibility/legacy lane remains. |
| `C6-P5-P12` | Phase 5 preserves C.5 publication/root truth independently of frame cleanliness and introduces no WAL, checkpoint, recovery, replay, serialized pool, durability-order, Query, branch, MVCC, or semantic-residency authority. C.7 can consume one admitted dirty/writeback truth without acquiring pool control. | Source/dependency/API scans; publication/writeback semantic split; fresh reopen from files; successor-boundary review; C.7 vocabulary absence in Phase-5 owners. | `PROVED` — publication/writeback semantic separation, successor scans, dependency gates, and fresh-process file truth preserve the C.7 insertion seam without importing later authority. |
| `C6-P5-P13` | Tests and generated reports are honest evidence: real causal worlds, independent file/media/allocation oracles, intended failure causes, nonzero exact selectors, unique proof obligations, bounded cost, current source identity, and lifecycle-aware artifact semantics. Mutable `mutation.lock` diagnostic content is not misrepresented as durable bytes. | `qa-tests` trace of setup/action/observation/teardown; exact test counts; mutation fault sensitivity; schema-v7 stage/content-stability validation; report/binary/source hash validation. | `PROVED` — real-file/process oracles, exact nonzero suites, schema-v7 stage semantics, 30 localized mutants, retained artifacts, and independent report/source/binary validation close every corrected evidence defect. |
| `C6-P5-P14` | The final Phase 5 source is formatted, warning-clean, within the dirty-scope 400-line cap, structurally coherent, dependency-honest, and accepted by focused and broad tests plus both mandatory constitutional gates. | `cargo fmt --check`, focused and broad Cargo checks/tests, dirty function scrutiny, exact line-cap audit, explicit topology/API searches, boundary-check, agent-context, and final-source audit. | `PROVED` — formatting, warning-denied checks, focused/broad suites, 211-file function scrutiny, zero over-cap files across 4,948 Store and 211 dirty Rust files, topology/API scans, both constitutional gates, and the final freeze pass. |

## Requirement Coverage

| Governing Phase 5 obligation | Ledger rows |
| --- | --- |
| Pre-allocation candidate/copy-on-write admission | `P01`, `P02`, `P09` |
| Exclusive or separately admitted mutation with immutable views | `P02` |
| Typed clean/dirty/claim/settlement/retry/inspection progression | `P03`, `P05`, `P08` |
| Exact backend receipt and Store settlement before clean | `P03`, `P04`, `P05`, `P10` |
| Cancellation, retry, timeout, close, partial, indeterminate | `P05`, `P06` |
| Responsibility-named private writeback capability | `P03`, `P04`, `P08`, `P11` |
| Dedicated frame-writeback basis, publication split | `P07`, `P12` |
| Preserve publication/root truth and defer C.7 ordering | `P07`, `P12` |
| Real append, dirty pressure, delayed writeback, eviction/claim denial, retry, fresh reopen | `P06`, `P09`, `P10`, `P13` |
| Delete raw allocation, duplicate helpers, temporary dirty/writeback names, obsolete evidence | `P01`, `P11` |
| APIs and semantic sharpness | `P03`, `P07`, `P08`, `P12` |
| Documentation when relevant | `L02`, `P11`, `P12` — the private Phase-5 contract is documented here and in the governing spec/removal ledger; Phase 9 still owns the public feature guide |
| Compile-time enforcement over developer conscientiousness | `P01`, `P02`, `P03`, `P08` |

## Risk Map

- **Allocation authority:** primary. A correctly sized global envelope does not
  prove the caller consumed the exact operation grant.
- **Cleanliness authority:** primary. A valid backend receipt can still be
  misused if Store settlement is not structurally joined.
- **Lifecycle and cancellation:** primary. Post-dispatch cancellation or close
  must not erase a possible effect; safe no-effect and ambiguity must diverge.
- **Visibility and concurrency:** primary. Another immutable pin must prevent
  mutation rather than observe an in-place write.
- **Semantic authority:** high. Publication and frame writeback share mutation
  shape but not meaning or authority.
- **Topology:** high. A local retry registry or scheduler can remain green
  under cooperative single-attempt tests while creating a second owner.
- **Eviction and accounting:** high. Dirty or claimed frames must remain
  resident, and counters cannot certify themselves.
- **Evidence integrity:** high. An unused synthetic request, zero-test selector,
  stale source-bound mutant, or mutable diagnostic hash can manufacture a
  plausible report without proving the production path.
- **Successor scope:** boundary check. Phase 5 must preserve the C.7 insertion
  seam without inventing durability order.

## Exact Evidence Index

Path roots:

- `B`: `workspaces/worth-store/crates/worth-store-buffer-pool/src/physical_residency`
- `S`: `workspaces/worth-store/crates/worth-store/src/physical_runtime`
- `J`: `workspaces/worth-store/crates/worth-store/tests/physical_record_journeys`
- `U`: `workspaces/worth-store/crates/worth-store/tests/physical_runtime_authority`
- `R`: `workspaces/worth-store/tools/store-test-runner/src`

`L01`

- manifest schema and audit-start reproduction under `Audit-Start Source
  Freeze`; exact final values and both independent reproductions under `Final
  Source Freeze` and `Final Evidence Results`

`L02`

- authority: governing C.6 `Inherited Truth`, `Adversarial Constraint`,
  `Authority Topology`, `Normative API Contract`, `Dependency Semantics`,
  `Authority Type Ledger`, `Type And Work Progression`, destination topology,
  dirty/writeback, cleanup, acceptance, Phase 5, non-goals, and closeout
  sections; roadmap C.6; all seven engineering-law documents
- completeness: requirement coverage, risk map, this index, findings history,
  and final surviving-defect attack

`P01`, `P02`

- production:
  `B/operation_allocation.rs`, `B/lease/frame.rs`, `B/lease/dirty.rs`,
  `B/operation_allocation/foreground_write.rs`,
  `B/lease/{candidate_allocation,dirty_replacement_allocation}.rs`,
  `B/pool/dirty_transition.rs`, `B/pool/candidate_admission.rs`,
  `B/observation/resource_accounting/frame.rs`
- pool evidence:
  `B/tests/clean_to_dirty.rs`,
  `B/tests/operation_allocation.rs`,
  `B/tests/allocation_events/dimension_reconciliation.rs`,
  `B/tests/allocation_events/candidate_materialization.rs`,
  `B/tests/allocation_events/dirty_replacement_release.rs`,
  `B/tests/candidate_concurrency.rs`
- compiler and source evidence:
  `worth-store-buffer-pool/src/api_compile_fail_proofs.md`,
  `R/physical_residency_boundary_gate/{candidate_allocation,foreground_write_authority}.rs`

`P03`, `P08`

- production:
  `S/record_serving/residency/dirty/{admitted_frame,failure,outcome,writeback}.rs`
  and `S/record_serving/residency/dirty/writeback/{progression,admission,execution}.rs`;
  `B/physical_residency/pool_ownership.rs` and
  `B/physical_residency/lease/{dirty,writeback}.rs` provide distinct
  pool-instance clean capabilities
- compiler:
  `U/residency_writeback_internals_are_sealed.rs`,
  `U/lower_clean_authority_is_required.rs`, their `.stderr` files,
  `physical_runtime_authority_ui.rs`, and
  `R/physical_residency_boundary_gate/clean_transition_authority.rs`

`P04`

- production:
  `S/record_serving/residency/dirty/writeback/{progression,admission,execution}.rs`,
  `S/record_serving/residency/candidate_frame_residency/writeback_progression.rs`,
  `S/record_serving/residency/scheduled_writeback.rs`,
  `S/work/{progression,execution,submission}/`, `S/instance/executor/`,
  `S/instance/signal_owner/`, and Store settlement
- evidence:
  `J/physical_work/residency_writeback_retry.rs`,
  `J/physical_work/post_dispatch_cancellation.rs`,
  `R/mutation_campaign/catalog/phase_16.rs` mutant 43, and the physical
  residency boundary gate

`P05`

- `J/physical_work/residency_writeback_lifecycle.rs`
- `J/physical_work/residency_writeback_retry.rs`
- `J/physical_work/post_dispatch_cancellation.rs`
- `J/ordinary_writeback_failures.rs`
- `B/tests/shutdown.rs`
- Store settlement classification and serving-health revocation

`P06`

- `B/pool/writeback_claim.rs`
- `B/tests/writeback_claim_exclusion.rs`
- `B/tests/eviction_siege.rs`
- `B/tests/shutdown.rs`
- `S/record_serving/residency/residency_observation/writeback.rs`
- `S/bin/physical_store_work_courtroom/c6_pressure/writeback_pressure.rs`

`P07`

- `S/record_serving/work_semantics/{mod,publication_basis,frame_writeback_basis}.rs`
- focused semantic tests in those modules
- Cargo metadata/dependency scans for the pool crate

`P09`

- ordinary publication:
  `S/record_serving/publication/`,
  `S/record_serving/residency/candidate_frame_publishers.rs`,
  `S/record_serving/residency/candidate_frame_residency/{write_progression,writeback_progression}.rs`
- ordinary success and failure:
  `J/segment_journeys.rs`,
  `J/extent_streaming/roundtrip.rs`, and
  `J/ordinary_writeback_failures.rs`
- delayed writeback:
  `S/bin/physical_store_work_courtroom/c6_pressure/writeback_pressure.rs`
- fresh process:
  `J/residency_writeback_fresh_reopen.rs` and its child-process dispatch

`P10`

- `R/mutation_campaign/catalog/phase_16.rs` mutants 15-44
- final mutation report, retained binaries, raw source blobs, selectors, and
  independent validation recorded under `Final Evidence Results`

`P11`

- `_docs/worth-store/physical-reconstruction-c6-removal-ledger.csv`
- deleted Phase-5 paths in Git status
- physical residency boundary-gate removal inventory
- exact source/path/selector/unused-input/compatibility scans

`P12`

- `S/record_serving/work_semantics/`
- publication and dirty/writeback owners
- ordinary Cargo dependency graph and forbidden-authority scans
- fresh-process evidence showing no persisted pool/Signal state

`P13`

- `R/courtroom_campaign/c6_inheritance_siege/`
- `R/courtroom_campaign/offline_observation.rs`
- schema-v7 evidence projection and protocol parser tests
- complete test fixture, oracle, process, mutation, artifact, and rerun traces

`P14`

- command catalog `E01` through `E15`

## Evidence Command Catalog

All Cargo commands run from `workspaces/worth-store`; repository tools run from
the repository root.

- `E01`: `cargo fmt --all -- --check`
- `E02`: focused buffer-pool dirty, allocation-event, eviction, claim, and
  shutdown tests with verified nonzero counts
- `E03`: fully module-qualified Store tests for writeback lifecycle, retry,
  cancellation, ordinary segment/extent success, ordinary no-effect/partial
  failure, fresh reopen, semantic basis, and exact receipt, with verified
  nonzero counts
- `E04`: `cargo test -p worth-store --features
  certification-test-authority --lib`
- `E05`: `cargo test -p worth-store --features
  certification-test-authority --test physical_record_journeys`
- `E06`: `cargo test -p worth-store --features
  certification-test-authority --test physical_runtime_authority_ui`
- `E07`: feature-enabled, registered `store-test-runner` proof families with
  explicit nonzero counts:
  `cargo test -p store-test-runner --features physical-work-evidence --lib
  'courtroom_campaign::c6_inheritance_siege::' -- --nocapture` must execute
  exactly 13 tests, including the terminal-settlement protocol substitution
  proof in `c6_inheritance_siege/protocol/tests.rs`, and
  `cargo test -p store-test-runner --features physical-work-evidence --lib
  'courtroom_campaign::offline_observation::tests::only_the_mutation_owner_observation_has_mutable_content'
  -- --exact --nocapture` must execute exactly 1 test; mutation-catalog and
  Courtroom execution evidence remain separately enumerated by their final
  commands and report paths
- `E08`: `cargo test -p store-test-runner physical_residency_boundary_gate`
- `E09`: `cargo check`/`cargo clippy` over the affected default and
  certification graphs, with warnings denied where supported
- `E10`: `python scripts/quality/scrutinize_rust_functions.py --dirty .`
- `E11`: dirty-scope PowerShell reproduction of the CI Rust line-cap rule:
  exact Git pathspecs, LF count, 400-line cap, and allowlist
- `E12`: exact source, path, dependency, feature, API, semantic-family,
  topology, compatibility, cleanup, and successor-boundary scans
- `E13`: `cargo run --manifest-path tools/boundary-check/Cargo.toml -- --root .`
- `E14`: `cargo run --manifest-path tools/agent-context/Cargo.toml -- check`
- `E15`: independent PowerShell/.NET and Python/hashlib final-source manifest
  reproductions using the declared schema

The full 30-mutant campaign and Courtroom C execution are final-source evidence
with distinct retained report paths recorded below. Earlier reports are
explicitly not reused.

## Final Evidence Results

The final source remained byte-identical through all final evidence:

- base commit:
  `95afd3a9ac80967d9a31ce75a80cad98af8c0604`
- 229 ordered dirty entries excluding only this ledger: 186 tracked and 43
  untracked, encoded in 33,914 manifest bytes
- independent PowerShell/Git and Python/raw-Git-blob implementations both
  produced
  `b593761ac3a95ff89e6869ded85af2c4a39aac7241be262c57b80cf0311beafa`

Final functional, compiler, structural, and constitutional evidence:

- `E01` passed.
- Warning-denied default and certification workspace checks passed.
- The buffer-pool evidence passed 85 unit tests, 3 integration tests, and 43
  compile-fail documentation specimens.
- The certification-feature Store library passed 74 tests.
- The real physical-record journey target passed 231 tests.
- The public-authority UI target passed all 32 specimens.
- The complete `store-test-runner` library passed 139 tests, both CLI targets
  passed, and the physical-residency boundary gate passed all 67 predicates.
- Dirty function scrutiny inspected 211 Rust files, reported 93 advisory
  candidates and zero scan errors; every advisory was reviewed against
  semantic-step/orchestrator responsibility rather than treated as a numeric
  defect.
- The exact LF-count line-cap audit inspected all 4,948 existing WORTH Store
  Rust files and all 211 dirty existing Rust files. Both scopes contain zero
  unallowlisted files over 400 lines.
- `boundary-check` reported `Road 1 Cargo topology is valid`.
- `agent-context check` passed.

The repository-global
`scripts/ci/check_workspace_rust_line_caps.sh` remains red on pre-existing,
unchanged files outside WORTH Store and outside this dirty implementation
scope. No compatibility path, legacy correction, or allowlist entry was added
to disguise that unrelated baseline. The complete dirty worktree and the
entire WORTH Store scope are independently clean, which is the exact scope of
`P14`.

Final mutation evidence:

- report:
  `workspaces/worth-store/target/c6-phase5-final-evidence/phase16-mutants-final-rerun.json`
- bytes: 37,444
- report SHA-256:
  `8707f47856f32eb411f4298ca6e7c16ebc6f82ff5b05667a5169d82d62fd3b45`
- schema: `worth.store.c5_1.mutation-evidence.v2`
- source-closure SHA-256:
  `a9cd70ff76ff50b2a1a05cd93bf93e9d1d3195847d0dfe462231fe52d6f39c6f`
- exact identities: 15 through 44, with no gap or duplicate
- all 30 mutants were nonidentity replacements killed by their exact expected
  predicate; all 30 current source hashes and all 30 retained executable
  hashes were independently revalidated
- retained executable directory:
  `workspaces/worth-store/target/c6-phase5-final-evidence/phase16-mutants-final-rerun.json.artifacts.current`

Final Courtroom C evidence:

- report:
  `workspaces/worth-store/target/c6-phase5-final-evidence/courtroom-c-final.json`
- bytes: 160,503
- report SHA-256:
  `bade7504b6b7d1dae82784a30ddfcfdf395ec4bdf7f485166d50e80c5f58b7b6`
- schema: `worth.store.c5_1.c6-inheritance-siege-courtroom.v7`
- source-closure SHA-256:
  `cf2120546a19021ead555c8d68761b078469e6062ac4def11d011ccd49af48e5`
- accepted oracle SHA-256:
  `5aab799bdb2229fcbefb4b4627acd283bf3883c3db5b863752e438160db99745`
- accepted verdict with zero findings at
  `after-siege-writer-close-before-fresh-reopen`
- one dirty frame and one active claim existed at the delayed-dispatch pause;
  the competing claim and eviction were denied; post-dispatch cancellation
  continued settlement; one positioned write and one exact receipt produced
  `write-completed`, `continue-settlement`, and
  `reconciled-from-physical-truth`; dirty and claim residue returned to zero
- all 30 mutation records exactly matched the retained mutation report, and
  the runner, writer, observer, source, mutant, and executable hashes were
  independently revalidated with zero binding violations

### Final ledger-completeness attack

The closing attack asked what meaningful Phase 5 defect could still pass every
row. It specifically attempted to preserve:

- a lower receipt-free cleaner despite exact Store settlement;
- a typestate progression unreachable from ordinary publication;
- an untyped candidate-tail append that could corrupt an existing range;
- a segment-only proof while the distinct extent branch bypassed writeback;
- weak failure oracles that accepted any Signal outcome or a copied format
  constant;
- a stale mutation seam, stale executable, or report detached from final
  source; and
- a source change hidden by the self-referential ledger.

Those are the defects recorded as `F024` through `F032`; the corrected
compile-time boundaries, ordinary real-file journeys, independent failure
oracles, exact 30-mutant campaign, Courtroom binding, and two-source-manifest
implementations now make each attack fail. The surrounding semantic families
were inspected for the same defects. No credible in-scope defect remains that
can satisfy every current row, so the ledger is complete for Phase 5 rather
than merely complete for the specification's wording.

## Findings And Reopening History

### `C6-P5-F001` — Temporary handoff duplicated dirty/writeback authority

- affected rows: `P03`, `P04`, `P08`, `P11`
- defect: C.5.1 `c6_handoff/residency` dirty and writeback types competed with
  the responsibility-named Phase 5 progression
- correction: move the progression into `residency/dirty/` and delete the
  Phase-5 handoff files without aliases
- current evidence: deleted paths and responsibility-named replacements are in
  the audit-start source; final absence gate remains pending
- status: `CORRECTED`, rows reopened for final evidence

### `C6-P5-F002` — Writeback admission mixed semantic levels

- affected rows: `P04`, `P14`
- defect: one admission body mixed runtime preflight, scheduler reservation,
  queue declaration, secure-I/O admission, demand lowering, and retry binding
- correction: decompose the orchestration into named semantic steps and a
  private `WritebackSchedulerBasis`
- status: `CORRECTED`, final composition review pending

### `C6-P5-F003` — Cancelled ready work could panic after request removal

- affected rows: `P04`, `P05`, `P13`
- defect: pre-dispatch cancellation removed request state before scheduler
  progression, and later access assumed the request remained present
- correction: typed active/current preflight now returns
  `ConsumerCancelled`, `AdmissionStopped`, or `CommandAbsent` before scheduler
  reservation
- current evidence: generic cancelled-ready-work test and focused writeback
  timeout path
- status: `CORRECTED`, final evidence pending

### `C6-P5-F004` — Courtroom expected the wrong Signal settlement

- affected rows: `P05`, `P09`, `P13`
- defect: after post-dispatch consumer cancellation, the courtroom expected
  `Committed`; canonical Signal authority correctly reconciles the physical
  completion as `ReconciledFromPhysicalTruth`
- correction: require the exact physical fate, recovery disposition, and
  reconciled Signal outcome
- status: `CORRECTED`, final Courtroom pending

### `C6-P5-F005` — Runner projection independently hard-coded the same wrong signal

- affected rows: `L02`, `P09`, `P13`
- defect: parent parsing/projection could reject the corrected child while
  continuing to manufacture `Committed` labels
- correction: add typed `CancelledWritebackTerminalSettlement`, construct it
  only from the exact three-field relationship, and derive evidence labels
  from that proof
- status: `CORRECTED`, schema-v7 final run pending

### `C6-P5-F006` — Mutant 43 targeted the deleted temporary handoff

- affected rows: `P04`, `P10`, `P11`, `P13`
- correction: bind the local-scheduler mutant to canonical writeback admission
  and its safe retry journey
- status: `CORRECTED`, final source-bound mutation campaign pending

### `C6-P5-F007` — Artifact manifest implied mutation-lock byte stability

- affected rows: `P13`
- defect: `namespace/mutation.lock` is a persistent, mutable owner diagnostic;
  a fresh writer legitimately rewrites it while the live OS lock remains
  authoritative
- correction: schema v7 names the manifest stage and classifies only this
  artifact as `mutable-mutation-owner-diagnostic`; durable artifacts retain
  byte-stability requirements
- status: `CORRECTED`, lifecycle-aware final validation pending

### `C6-P5-F008` — Initial focused selectors executed zero tests

- affected rows: `P05`, `P13`, `P14`
- defect: unqualified exact selectors returned green with zero executed tests
- correction: closure evidence must record fully module-qualified selectors
  and nonzero counts; aggregate green output alone is insufficient
- status: `CORRECTED` — feature-enabled Cargo registration listed 12 tests
  beneath `courtroom_campaign::c6_inheritance_siege::`; that exact family
  executed 12 passed tests, and the fully qualified offline-observation
  selector separately executed 1 passed test

### `C6-P5-F009` — Mutation source hashes conflated two contracts

- affected rows: `P10`, `P13`
- defect: the mutation report records raw source bytes while the Phase-16
  source manifest normalizes line endings; treating them as one digest falsely
  invalidated or accepted evidence
- correction: validate each artifact against its declared raw or normalized
  contract independently
- status: `CORRECTED`, final report validation pending

### `C6-P5-F010` — Removal evidence did not distinguish deleted content from later path ownership

- affected rows: `P11`
- defect: some Phase-5 rows removed only obsolete C6 content from files whose
  remaining C6 path/name is assigned to Phase 7
- correction: the ledger records Phase-5 deletion by token/source inventory
  while preserving explicit Phase-7 path cleanup ownership
- status: `CORRECTED`, final absence and removal-ledger audit pending

### `C6-P5-F011` — Dirty replacement does not consume operation-grant use

- severity: high
- affected rows: `L02`, `P01`, `P02`, `P13`, `P14`
- concrete defect:
  `PhysicalFrameLease::begin_dirty_replacement` calls
  `OperationAllocationGrant::scope_for` but never `reserve_use`. A foreign
  grant is rejected, yet an undersized grant or a grant whose full allowance
  is already active can still authorize allocation under the separate
  dirty-replacement and total limits.
- violated invariant: pre-allocation candidate authority must carry exact,
  non-double-spend operation allocation; ownership alone is not allocation
  capacity
- required correction: store one exact `OperationAllocationUse` inside
  `PhysicalDirtyReplacementReservation`, acquire it before dirty reservation,
  and release it on every denial, allocator failure, fill failure, unwind,
  success, and drop
- required proof: undersized and concurrent double-spend attacks fail on
  `OperationBytes` before allocation; foreign, dirty-limit, allocator, fill,
  unwind, and success paths reconcile grant use and allocation events exactly
- status: `CORRECTED` — the reservation now owns an exact
  `OperationAllocationUse`; the focused dirty-replacement allocation-event
  family compiled and executed 5 passed tests, including undersized,
  double-spend, allocator-failure, dirty-limit, and unwind attacks; affected
  guarantees are reopened for the complete final-source release audit

### `C6-P5-F012` — Courtroom surface carries an unused synthetic write request

- severity: medium evidence-integrity defect
- affected rows: `L02`, `P09`, `P11`, `P13`
- concrete defect:
  `c6_pressure::run_inheritance_siege` passes a synthetic
  `PhysicalMutationWorkRequest` into `writeback_pressure::prove`, whose
  `_request` argument is unused. The real writeback correctly constructs the
  dedicated frame-writeback request internally, so the argument proves
  nothing and falsely suggests causal participation.
- violated invariant: every proof input must have causal provenance and every
  claimed integration must follow the production derivation
- required correction: remove the unused request from the writeback proof
  surface and name only the profile input that actually configures the Store;
  preserve the separate exact-write courtroom where that synthetic request is
  genuinely executed
- required proof: unused-input scan, focused Courtroom tests, and final
  multi-process Courtroom C
- status: `CORRECTED` — the C6 pressure path now requests only
  `exact_write::profile()` and the writeback proof accepts no synthetic work
  request; `exact_write::bind()` remains the request-producing path used by
  the exact-write courtroom; compilation passed and affected guarantees are
  reopened pending final Courtroom C

### `C6-P5-F013` — Allocator-failure evidence could not compile

- severity: high evidence-integrity defect
- affected rows: `P01`, `P13`, `P14`
- concrete defect: the dirty-replacement allocator-failure test names the real
  `DirtyReplacementAllocator`, but its owning child module was private to
  `lease`; the sibling `physical_residency::tests` module failed with `E0603`
  before any runtime assertion executed
- violated invariant: a named test is not evidence unless its production
  boundary is reachable and the intended test count actually executes
- correction: expose the allocator abstraction only to its owning
  `physical_residency` parent boundary with `pub(super)`; do not make it
  crate-public or add a test-only production branch
- required proof: the exact allocation-event family compiles, executes a
  nonzero count, and reaches allocator failure before fill
- status: `CORRECTED` — allocator injection is visible only to the owning
  `physical_residency` parent through `pub(super)`; the exact family compiled
  and executed 5 passed tests, including the pre-fill allocator failure

### `C6-P5-F014` — Candidate admission does not bound the actual allocation

- severity: high
- affected rows: `L02`, `P01`, `P09`, `P11`, `P13`, `P14`
- concrete defect:
  `PhysicalCandidateFrameReservation::materialize` accepts
  `FnOnce() -> Vec<u8>` after reserving only the candidate coordinate length.
  The producer can allocate a wrong-length arbitrarily large vector before the
  post-allocation length check, or return the expected length with arbitrarily
  excessive capacity. Neither allocation is represented by the exact
  operation use or resident-byte envelope.
- violated invariant: Phase 5 requires pre-allocation candidate admission and
  deletion of raw `Vec` replacement paths. Admitting the expected payload
  length is not authority for an unbounded producer-selected allocation.
- required root correction: move candidate allocation behind the pool
  reservation, allocate exactly the admitted coordinate length, and expose
  only a fixed-size fill slice to the producer. Preserve exact release on
  allocator failure, fill failure/unwind, publication conflict, and success;
  remove the raw-`Vec` API and migrate every production caller.
- required proof: source/API absence scan for producer-owned candidate vectors;
  adversarial allocator-failure, wrong-size-impossibility, unwind, conflict,
  and successful publication tests with exact allocation-event and operation
  use reconciliation; real ordinary append/publication proof
- status: `CORRECTED` — `CandidateFrameAllocator` now returns only the opaque
  `CandidateFrameBuffer`, constructible inside its semantic allocator owner;
  producers receive only the exact mutable slice. The raw contract is absent,
  candidate allocation/failure/unwind executes 3/3, candidate conflict
  executes 5/5, the certification Store graph compiles, the foreign-receipt
  test executes 1/1, and ordinary append/writeback/observer/fresh-reopen
  executes 1/1. Affected guarantees remain reopened for final-source proof.

### `C6-P5-F015` — Dirty-replacement allocation does not prove the admitted extent

- severity: high
- affected rows: `L02`, `P01`, `P02`, `P11`, `P13`, `P14`
- concrete defect:
  `DirtyReplacementAllocator::allocate` returns `Vec<u8>` after the clean
  lease and exact `OperationAllocationUse` reserve the frame's declared
  length. A replacement allocator can return a shorter or larger vector;
  `finish_dirty_replacement` then installs it while the resident frame entry
  continues accounting the original coordinate length.
- violated invariant: copy-on-write mutation must allocate exactly the
  separately admitted replacement extent. The exact grant and counter state
  cannot rely on every allocator implementation voluntarily returning the
  right vector shape.
- required root correction: give dirty replacement its own opaque exact buffer
  type, construct it only inside the semantic allocator owner, expose only the
  exact source/target fill slices, and prevent allocator implementations from
  constructing an arbitrary successful buffer. Do not unify candidate and
  dirty-replacement authority merely because both use byte storage.
- required proof: source/API absence for raw allocator-vector results; exact
  candidate and dirty-replacement slice widths; allocator failure before fill;
  fill failure/unwind, conflict, success, operation-use, allocation-event, and
  resident-byte reconciliation.
- status: `CORRECTED` — `DirtyReplacementAllocator` now returns only the
  separately owned opaque `DirtyReplacementBuffer`; the fill receives exact
  source and target slices and no allocator implementation outside that owner
  can construct a successful arbitrary buffer. The exact grant,
  allocator-failure, ceiling, double-spend, success, and unwind family executes
  5/5, and the Store certification graph compiles. Affected guarantees remain
  reopened for final-source proof.

### `C6-P5-F016` — Lower pool receipt APIs bypass Store settlement authority

- severity: high
- affected rows:
  `L02`, `P03`, `P04`, `P05`, `P08`, `P09`, `P10`, `P11`, `P12`, `P13`,
  `P14`
- concrete defect:
  `DirtyPhysicalFrame::publish_clean` and
  `PhysicalWritebackClaim::publish_clean` are public lower-pool methods that
  accept `CompletedArtifactRangeWrite` directly. Ordinary candidate
  publication receives `CandidateFramePhysicalWrite`, which carries both the
  exact receipt and `CanonicalRecordMutationSettlement`, but binds the work
  identity only to `_work` and calls the pool with the borrowed receipt. The
  Store settlement proof is neither validated nor consumed.
- violated invariant:
  the normative lower-pool API forbids scheduler/backend receipt types, and a
  dirty frame may become clean only through the exact Store settlement chain.
  A receipt proves a physical effect; it is not Store cleanliness authority.
- required root correction:
  remove backend receipt types and receipt validation from physical-residency
  pool APIs. Keep exact receipt/work/fate validation in Store-owned settlement
  types; consume one non-duplicable candidate or writeback settlement
  completion to authorize one pool claim/candidate clean transition. Preserve
  pool ownership of dirty truth and claim exclusion without introducing a
  generic marker, callback, or Store dependency.
- required proof:
  source and dependency absence for backend receipts in physical residency;
  compile-time inability to clean from a receipt, generic completion, copied
  settlement, or skipped Store phase; foreign work/settlement/receipt tests;
  exact ordinary publication and writeback progression; premature-clean and
  settlement-bypass mutants localized at the clean transition.
- status: `CORRECTED` — the first correction was reopened by `F024` because
  it exposed public zero-argument clean transitions. `F024` then completed the
  root correction with distinct pool-instance-bound candidate and writeback
  clean capabilities consumed only after matching Store settlement. Final UI,
  source-gate, ordinary-path, and mutant evidence proves that neither a receipt
  nor possession of lower dirty/claim state can clean a frame.

### `C6-P5-F017` — Compile-fail clean-authority evidence contains an unrelated name failure

- severity: high evidence-integrity defect
- affected rows: `L02`, `P11`, `P13`, `P14`
- concrete defect:
  after the F016 API correction, the candidate half of
  `generic_completion_cannot_clean.rs` passed `&completion` without declaring
  that parameter. Its snapshot therefore contained `E0425` in addition to the
  intended `E0061` phase-boundary failure.
- violated invariant:
  compile-fail evidence must fail for the forbidden authority transition
  itself. An unrelated unresolved name can make the test survive even if the
  clean-authority boundary later regresses.
- required correction:
  give each probe its own concrete generic scheduler completion parameter and
  retain only the compiler error showing that the receipt-free pool
  transitions accept no such argument.
- required proof:
  regenerate the exact snapshot, inspect it for only the intended method
  signature failures, then run the authority suite without overwrite.
- status: `CORRECTED` — the first snapshot repair was reopened by `F024`
  because an argument-count error could not prove missing authority. The final
  constructor-free specimen now fails on the absent pool-bound clean
  capability itself, its checked diagnostic describes the current API, and
  the complete 32-specimen authority UI suite passes without overwrite.

### `C6-P5-F018` — Boundary gate does not preserve the lower receipt-absence invariant

- severity: high regression-prevention defect
- affected rows: `L02`, `P03`, `P04`, `P11`, `P13`, `P14`
- concrete defect:
  `physical_residency_boundary_gate::dependency_boundary` rejects direct
  Signal, Proof, Foundational, and aspect-native authority throughout the
  buffer-pool crate, but it does not reject
  `worth_store_physical_backend` or `CompletedArtifactRangeWrite` inside the
  narrower `physical_residency` subtree. F016 currently survives only because
  of a manual source scan.
- violated invariant:
  a corrected authority defect is not closed when the deterministic boundary
  gate still accepts the exact forbidden topology. Legitimate backend use by
  separate background-work responsibilities does not authorize physical
  residency to consume backend receipts.
- required correction:
  add a physical-residency-specific source prohibition for backend imports and
  receipt types while retaining the crate's separate background-work
  dependency. Add controlled mutants for both an import and a receipt-bearing
  API.
- required proof:
  focused dependency-boundary tests and the complete physical-residency
  boundary suite pass with nonzero counts; both controlled mutants fail at the
  new localized boundary predicate.
- status: `CORRECTED` — the dependency boundary now scans the exact
  `physical_residency` subtree for both backend module imports and
  `CompletedArtifactRangeWrite` exposure while leaving separate background
  work intact. Both localized mutants fail; the focused family executes 5/5
  and the complete physical-residency boundary suite executes 59/59.

### `C6-P5-F019` — Current courtroom protocol test has no removal disposition

- severity: medium cleanup/evidence-integrity defect
- affected rows: `L02`, `P11`, `P13`, `P14`
- concrete defect:
  the complete physical-residency boundary suite classifies
  `tools/store-test-runner/src/courtroom_campaign/c6_inheritance_siege/protocol/tests.rs`
  as a temporary or legacy consumer, but the C.6 removal ledger contains no
  row naming its current responsibility, future owner, and removal phase.
- violated invariant:
  every temporary consumer must be either deleted when obsolete or assigned an
  explicit successor/removal disposition. An unclassified test can preserve a
  stale protocol or disappear without transferring its proof obligation.
- required correction:
  inspect the test's causal role and the neighboring removal dispositions.
  Delete it if obsolete; otherwise add the exact current owner, successor
  owner, phase, and mechanical absence gate without labeling current evidence
  as legacy compatibility.
- required proof:
  the exact removal-inventory selector and the complete physical-residency
  boundary suite pass; no stale open or rediscovered deleted row appears.
- status: `CORRECTED` — the protocol test remains current adversarial evidence
  and is assigned to Phase 7's responsibility-named physical writeback
  courtroom protocol tests, with the standard source-and-metadata absence
  gate. The exact selector executes 1/1 and the complete boundary suite
  executes 59/59 with no unclassified, stale-open, or rediscovered-deleted
  consumer.

### `C6-P5-F020` — Exact-settlement mutation evidence is stale and incomplete

- severity: high evidence-integrity defect
- affected rows: `L02`, `P03`, `P05`, `P10`, `P13`, `P14`
- concrete defect:
  mutant 42 still targets the deleted lower-pool receipt validator in
  `worth-store-buffer-pool`, so it cannot bind to current source. The
  Store-owned candidate validator therefore has no executable current mutant,
  and the distinct Store-owned writeback validator has no mutant at all. The
  new writeback unit test calls `receipt_matches_claim` directly and carries
  no causal mutation marker, so it cannot prove that the real completion
  transition consumes exact receipt validation or localize a mutant failure.
- violated invariant:
  mutation evidence must bind exactly once to current source, attack every
  independently regressible authority boundary, and fail through the real
  production transition at one declared predicate. Candidate publication and
  dirty-frame writeback are separate typestate authorities and cannot share
  one nominal receipt mutant.
- required correction:
  retarget mutant 42 to the Store-owned candidate settlement seam; add one
  adjacent mutant for Store-owned writeback settlement; drive writeback
  through its real consuming transition with exact wrong and valid twins;
  mutation-test the pure candidate receipt predicate while mechanically
  requiring the production `store_write -> settle_residency -> publish_clean`
  progression, because constructing a foreign outer candidate completion
  would counterfeit the settlement authority under test; add unique causal
  markers; and update the catalog, report, Courtroom requirement, denominator,
  source mapping, and scenario mapping atomically.
- required proof:
  exact source binding and catalog-shape tests pass with 30 Phase-16 mutants;
  each affected baseline selector passes; both mutants fail only their
  declared runtime predicate without compilation or unrelated failure; report
  contract tests and Courtroom mutation requirements accept the expanded
  catalog.
- status: `CORRECTED` — mutant 42 now attacks the Store-owned candidate
  receipt predicate and mutant 44 independently attacks the Store-owned
  writeback predicate. Both fail only their exact causal marker; both baseline
  selectors pass. The writeback test now consumes the real
  `PhysicalResidencyWritebackCompletion`, observes mismatch-retained dirty
  truth and zero-claim exact cleaning, and performs honest grant teardown.
  The candidate proof is deliberately split across the mutated pure predicate,
  the real positive publication path, the complete authority UI suite, and a
  controlled source gate that rejects skipped, reordered, lower, or forged
  cleaning. The Phase-16 catalog/report contract contains IDs 15–44 exactly,
  the feature-enabled mutation family passes 32/32, report contracts pass 3/3,
  and Courtroom C requires the exact `(42, candidate)`, `(44, writeback)`, and
  `(43, scheduler)` identity/predicate pairs. The candidate gate now has its
  own semantic module at 212 lines; the aggregate writer gate is 248 lines.

### `C6-P5-F021` — Mutation binding gate hides stale seams after its first failure

- severity: high evidence-completeness defect
- affected rows: `L02`, `P10`, `P13`, `P14`
- concrete defect:
  the catalog binding gate fails on the first non-unique source seam. It
  exposed mutant 13's stale pre-extraction
  `extent_read_session.rs` needle, but aborted before reporting whether any
  other mutant is stale. Consequently a repair/rerun loop could discover
  catalog defects one at a time and falsely treat the first green local repair
  as a complete binding audit.
- violated invariant:
  a completeness gate must report the complete evaluated set and every
  violation in that set. Mutation evidence cannot be closed through serial
  first-failure archaeology, and a semantically current mutant must follow its
  current authority seam rather than a historical spelling.
- required correction:
  make the binding gate collect every missing or duplicate seam before
  failing; audit all catalog entries in one execution; classify phase-16 and
  earlier mismatches explicitly; and retarget only mutants whose production
  claim remains current. Delete obsolete mutants rather than preserving
  compatibility with dead source.
- required proof:
  a controlled multi-mismatch fixture proves aggregate diagnostics; the real
  catalog reports no missing or duplicate seam; mutant 13 binds the current
  bounded extent-read plan and still fails its causal allocation-slope
  selector at runtime.
- status: `CORRECTED` — the binding audit now accumulates every missing or
  duplicate seam and its controlled two-mismatch case proves non-fail-fast
  diagnostics. The one real execution reported exactly stale IDs 13 and 14.
  Mutant 13 now binds `ExtentReadState::plan_chunk_read` and fails only
  `transfer-allocation-slope`. Mutant 14 and its absent selector were retired:
  its replacement required the forbidden `for_contract_test` settlement
  forge, while the new move-only authority, candidate progression gate, and
  authority UI suite make the bypass structurally unavailable. ID 14 is
  explicitly non-selectable rather than retained through an alias. The
  complete catalog binding family passes 5/5 and the feature-enabled mutation
  family passes 32/32.

### `C6-P5-F022` — Dirty-replacement evidence misses the post-reservation pin race

- severity: high evidence defect
- affected rows: `L02`, `P02`, `P13`
- concrete defect:
  `clean_to_dirty.rs` proves denial when a competing immutable pin already
  exists before `begin_dirty_replacement`, but no test acquires a new pin after
  replacement admission and allocation begin and before
  `finish_dirty_replacement` revalidates the frame. A regression that removed
  the final pin revalidation, installed the replacement while the competing
  view remained live, or mutated the original allocation could pass every
  current P02 test.
- violated invariant:
  immutable frame views must never observe mutation, and the proof must
  exercise the actual race window rather than infer it from cooperative
  preconditions. P02 explicitly requires a pin-race and source/pointer oracle.
- required correction:
  add a deterministic two-thread test whose replacement fill pauses while a
  second lease pins the same clean allocation, then completes the fill and
  requires typed `FramePinned` denial. Observe the competing lease's pointer
  and bytes across the failed finish, and reconcile replacement authority after
  teardown.
- required proof:
  the new test fails if final revalidation is removed or if replacement writes
  touch the original allocation; the complete clean-to-dirty and allocation
  release families pass with exact nonzero counts.
- status: `CORRECTED` — the deterministic two-thread test opens the race only
  after exact replacement admission, holds a second immutable pin across final
  installation, receives typed `FramePinned`, reconciles one admission and one
  release, and proves the original allocation pointer and `[1; 8]` bytes are
  unchanged before, during, and after denial. The clean-to-dirty family passes
  3/3 and the complete current buffer-pool unit suite passes 83/83.

### `C6-P5-F023` — Scope-erased grants can authorize foreground mutation

- severity: critical authority defect
- affected rows: `L02`, `P01`, `P02`, `P08`, `P13`, `P14`
- concrete defect:
  `PhysicalResidencyPool::{materialize_dirty_candidate,
  reserve_candidate_frames,begin_candidate_batch}` and
  `PhysicalFrameLease::begin_dirty_replacement` accept the generic
  `OperationAllocationGrant`. `reserve_use` proves pool identity and available
  bytes but deliberately preserves any `PhysicalOperationAllocationScope`; no
  mutation boundary requires `ForegroundWrite`. A sufficiently large
  `ForegroundRead`, recovery, scrub, maintenance, verification, or blob grant
  can therefore spend its allowance on candidate or copy-on-write mutation.
  The stale `clean_to_dirty.rs` tests even pass a one-byte `ForegroundRead`
  grant to an eight-byte replacement, contradicting the current exact-grant
  contract before the intended transition is reached.
- violated invariant:
  Phase 5 requires candidate and copy-on-write allocation under the exact
  `ForegroundWrite` grant. Authority boundaries must be compiler-visible where
  practical; an enum value checked only by developer conscientiousness is not
  mutation authority.
- required correction:
  introduce a pool-issued, non-forgeable foreground-write allocation grant
  type, return it from an exact foreground-write admission API, and require it
  at every candidate and dirty-replacement entry point. Preserve the generic
  grant for non-mutation operation scopes and ordinary read/planning work.
  Cut Store publication, certification support, boundary gates, and tests over
  without a compatibility overload or scope-checking alias.
- required proof:
  compile-fail evidence shows a generic/read grant cannot call candidate or
  dirty-replacement APIs; positive Store publication uses the typed grant;
  foreign, undersized, and concurrent-use runtime cases remain causal; source
  gates reject a reintroduced generic mutation signature.
- status: `CORRECTED` — `ForegroundWriteAllocationGrant` is a constructor-free
  wrapper with a private generic grant field. Its sole issuance implementation
  is colocated with that private representation and binds
  `PhysicalOperationAllocationScope::ForegroundWrite`; the type dereferences
  only toward weaker generic planning authority. Candidate and dirty
  replacement APIs, Store append publication, and certification mutation now
  require the stronger type with no generic overload. Current evidence passes:
  83/83 buffer-pool unit tests, 40/40 compile-fail doctests, 6/6 candidate
  allocation gates, 3/3 foreground-write authority gates, 7/7 Store candidate
  tests, and the integrated certification-feature compile.

### `C6-P5-F024` — Receipt-free pool cleaners are public settlement bypasses

- severity: critical authority and evidence-completeness defect
- affected rows:
  `L02`, `P03`, `P04`, `P05`, `P06`, `P08`, `P09`, `P10`, `P13`, `P14`
- concrete defect:
  `DirtyPhysicalFrame::complete_candidate_publication()` and
  `PhysicalWritebackClaim::complete_writeback()` are public zero-argument
  methods. `PhysicalResidencyPool::{materialize_dirty_candidate,
  claim_writeback}` and every necessary key/allocation type are also exported.
  A temporary integration test compiled in an external-crate privacy context,
  opened a pool, created two dirty frames, cleaned the first directly through
  candidate completion, claimed the second and cleaned it directly through
  writeback completion, and observed `dirty_frames == 0` both times without a
  backend effect, exact receipt, Signal settlement, or Store completion.
- ledger-completeness defect:
  F016 treated “receipt-free” as “authority-safe,” F017 asserted only that a
  generic completion was an extra argument, and the current boundary/mutation
  gates check Store validator presence without proving that the lower
  transition is inaccessible. All could pass while the exact forbidden bypass
  compiled and executed.
- violated invariant:
  pool cleanliness is lower physical truth, but only Store-owned exact
  settlement may authorize its transition. Removing backend types from the
  pool must not turn the transition into an unguarded command. Weaker values,
  possession of a dirty frame, and possession of a claim must open no cleaning
  door.
- required root correction:
  introduce pool-instance-bound cleaning authority issued only with controlled
  pool ownership, keep it separate from ordinary cloneable pool access, and
  require the semantically exact candidate or frame-writeback authority at
  each lower clean transition. Store keeps these capabilities private and
  invokes them only after consuming its move-owned exact settlement. The pool
  remains backend-, Signal-, Foundational-, and `worth-proof`-agnostic.
- required proof:
  an external crate with a pool, dirty frame, claim, generic completion, raw
  receipt, and copied scalar values cannot compile either clean transition;
  the intended Store candidate and writeback completion paths compile and
  execute; foreign pool authority is rejected; candidate and writeback
  capabilities cannot substitute for one another; source gates reject public
  no-proof cleaners; exact-settlement and bypass mutants fail their causal
  predicates.
- status: `CORRECTED` — `PhysicalResidencyPoolOwner` now issues distinct,
  non-cloneable `CandidateFrameCleanAuthority` and
  `FrameWritebackCleanAuthority` values bound to one pool incarnation.
  Candidate publication and exact writeback completion require the matching
  capability; the kinds cannot substitute, foreign-pool authority is denied,
  and no zero-argument clean transition remains. Store retains the
  capabilities behind its private frame ports and consumes them only after
  candidate or writeback settlement. The external UI specimen,
  `clean_transition_authority` source gate, candidate/writeback settlement
  tests, and distinct mutants 42 and 44 cover the corrected boundary.

### `C6-P5-F025` — Ordinary publication did not reach the writeback progression

- severity: critical integration defect
- affected rows: `L02`, `P04`, `P09`, `P12`, `P13`, `P14`
- concrete defect:
  the typed frame-writeback progression existed only behind certification and
  direct proof fixtures. Ordinary segment and extent publication continued to
  publish every candidate frame through the new-artifact lane, so green
  typestate tests did not prove the Phase 5 telos. A default warning-denied
  build exposed the disconnected production path.
- root correction:
  first frames retain canonical new-artifact publication; every continuation
  frame is converted from resident candidate authority into admitted dirty
  authority and traverses prepare, `ExactWriteback` readiness, scheduler
  admission, executor dispatch, Store settlement, and the exact lower clean
  transition. The obsolete candidate-append retry posture was removed rather
  than retained as a parallel lane.
- status: `CORRECTED` — ordinary segment and extent publication call
  `write_frame_via_writeback` for continuation frames, while root,
  publication, and first-artifact truth remain distinct.

### `C6-P5-F026` — Candidate tails and existing ranges had no typed write posture

- severity: critical physical-effect and semantic-authority defect
- affected rows: `P03`, `P04`, `P05`, `P09`, `P12`, `P13`
- concrete defect:
  the existing exact-range backend correctly rejected writes extending past
  EOF. Treating every nonzero offset as append would make dirty replacement
  unsafe, while treating every write as replacement made ordinary candidate
  continuation fail before effect.
- root correction:
  `PhysicalWritebackRangePosture::{ExistingRange,
  CandidateArtifactTail}` is derived from frame origin, carried by the claim
  and queue declaration, compared at scheduled admission, and consumed by the
  backend. `ExistingRange` requires the complete range already to exist;
  `CandidateArtifactTail` requires the coordinate offset to equal current EOF.
  A nonzero-offset dirty replacement remains `ExistingRange`.
- status: `CORRECTED` — same-coordinate pool tests distinguish candidate from
  replacement origin; scheduled admission rejects missing, stale, and swapped
  posture; real ordinary publication proves exact EOF extension.

### `C6-P5-F027` — Ordinary writeback success and effect-failure evidence was absent

- severity: high proof-reachability defect
- affected rows: `L02`, `P04`, `P05`, `P09`, `P13`
- concrete defect:
  certification journeys proved the progression in isolation, but no ordinary
  `record_submission` journey reconciled candidate frames to scheduled
  writebacks, and no ordinary failure proved typed no-effect versus partial
  effect outcomes against real files.
- root correction and proof:
  the four-segment ordinary journey proves 14 candidate frames, 10
  first-artifact publications, 4 scheduled writebacks, 4 exact receipts, 4
  terminal `ArtifactRangeWrite` records, concrete backend identities,
  `ExactWriteback` Signal bindings, zero dirty/candidate/claim residue, four
  exact 32,768-byte files, and fresh-process reads. Separate fail-before and
  three-byte partial-effect journeys target the second candidate write by an
  independently measured media ordinal and require exact failure cause,
  effect fate, recovery, `ReconciledFromPhysicalTruth`, file length, residency
  retention/discard, and close posture.
- status: `CORRECTED`

### `C6-P5-F028` — The distinct extent continuation branch had no causal predicate

- severity: high mutation-survival defect
- affected rows: `L02`, `P09`, `P10`, `P13`
- concrete defect:
  segment evidence could remain green if the separate extent loop regressed to
  direct candidate publication. Existing extent tests proved bytes and
  streaming behavior but not the Phase 5 work path.
- status: `CORRECTED` — the real 65-frame extent journey now requires exactly
  64 scheduled writebacks, 64 exact receipts, 64 terminal range-write work
  records with backend identity and `ExactWriteback` binding, the exact
  artifact length, and a fresh-process read.

### `C6-P5-F029` — Clean-authority compile-fail diagnostics described a deleted constructor

- severity: medium evidence-currentness defect
- affected rows: `P03`, `P08`, `P13`
- concrete defect:
  the trybuild snapshot still expected privacy around an earlier constructor,
  so it did not describe the current constructor-free authority boundary.
- status: `CORRECTED` — the specimen and checked diagnostic now fail because no
  current constructor exists and the required clean capability is absent.

### `C6-P5-F030` — Mutant 39 targeted a stale backend-write seam

- severity: high mutation-binding defect
- affected rows: `P10`, `P13`
- concrete defect:
  cleanup and executor cutover changed the exact backend call while the mutant
  retained its prior source replacement. Catalog shape could remain green even
  though the skipped-write attack no longer bound current source.
- status: `CORRECTED` — mutant 39 targets
  `tree.write_scheduled_foreground_exact_at(coordinate, &payload, ...)`;
  the real mutant run bound once and was killed by the exact
  `skipped-backend-write` predicate.

### `C6-P5-F031` — Ordinary failure oracles accepted weak Signal and format claims

- severity: medium test-honesty defect
- affected rows: `P05`, `P13`
- concrete defect:
  both ordinary fault journeys accepted any present Signal outcome and
  hard-coded the dense page width in their file-length oracle. The first
  sharpened run correctly rejected an assumed `Committed` outcome and exposed
  the actual reconciliation semantics.
- status: `CORRECTED` — both require
  `ReconciledFromPhysicalTruth`, and expected artifact lengths derive from the
  admitted physical format declaration.

### `C6-P5-F032` — New proof functions mixed orchestration and terminal oracles

- severity: medium composition and maintainability defect
- affected rows: `P13`, `P14`
- concrete defect:
  adding ordinary segment/extent and fault evidence pushed individual test
  functions past the function-size advisory and mixed behavior setup,
  counter/Signal reconciliation, physical artifact truth, and fresh-process
  truth.
- status: `CORRECTED` — responsibility-named baseline and assertion helpers
  separate frame/media, work/Signal, artifact/placement, failure cleanup, and
  fresh-process proof. All dirty Rust files remain below the hard 400-line cap,
  and no forbidden catch-all module name exists.

## Current Closure Posture

Phase 5 is **CLOSED** against the exact final source and evidence recorded in
this ledger. `F001` through `F032` remain preserved as corrected audit history;
all 16 closure rows are `PROVED`; no `OPEN`, `DEFECT`, or `BLOCKED` row remains;
and no known scoped implementation or evidence defect survives the final
ledger-completeness attack. Phase 6 may begin only from a new boundary review
and implementation plan.
