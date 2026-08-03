# C.6 Phase 6 Closure Ledger

## Scope

This ledger audits C.6 Phase 6, **Lower Speculative Work Through The Canonical
Runtime**, against the governing C.6 specification, the WORTH engineering
constitution, the actual buffer-pool, Store, Signal, scheduler, executor,
backend, physical-format, security, and Foundational boundaries, and defects
that could satisfy the phase wording while defeating its intent.

Phase 7 and later remain blocked. This ledger does not claim final ordinary
consumer cutover, deletion of `c6_handoff`, successor adapter specimens, final
S.2 deletion, or public feature closeout. Phase 6 cleanup is in scope exactly
where canonical speculative lowering makes an isolated queue/background model
obsolete. The product is unreleased: obsolete code is deleted, not retained
behind compatibility, legacy, or migration machinery.

## Current Authority

This is a living closure audit. A row is `PROVED` only after:

1. its exact implementation and test evidence are inspected against the final
   source freeze;
2. every causally affected row is reopened after a correction;
3. the independent ledger attack finds no meaningful defect that can pass the
   row;
4. every named final gate succeeds against that same source.

The final base commit is
`95afd3a9ac80967d9a31ce75a80cad98af8c0604`. The final exact-byte source
freeze contains 285 entries after excluding only this ledger: 197 modified,
14 deleted, and 74 untracked. Closed C.6 Phase 1-5 and inherited C.5.1 work
remain part of the authority graph; this ledger does not reclassify them as
Phase 6.

## Boundary Brief

### Adversarial constraint

Phase 6 must remain correct under this hostile condition:

> Prefetch and read-ahead contend for the same foreground-read envelope while
> write-behind holds the foreground-write envelope and dirty authority. Every
> kind is driven to its exact ceiling and one request past it. A duplicate cold
> read races the source owner; another read hits; a later read faults; the
> second member of a read-ahead batch fails after the first completes. A
> write-behind is denied while dirty state is saturated, then retried. Close
> begins only after each kind has passed the point where an effect is possible
> and must demonstrably stall before safe cancellation completes. An attacker
> substitutes Prefetch for ReadAhead, changes a read grant into WriteBehind,
> steals another operation scope, erases queue kind or durability, invents a
> speculative Signal family, bypasses secure-I/O or the scheduler, routes
> directly to backend media, relocates a pool-local worker or pending registry,
> suppresses release after close, or presents a foreign/stale allocation. No
> allocation or work may precede exact admission; no hit or denial may invent
> Signal, scheduler, or media work; no accepted miss may evade the inherited
> Signal → scheduler → executor → backend → Store settlement chain; denied
> write-behind must retain dirty authority; and every terminal path must
> reconcile exact kind, scope, total-envelope, media, work, and shutdown truth.

Cooperative success is insufficient. The proof must distinguish hit, coalesced
wait, cold fault, partial batch, pre-effect pressure denial, accepted
write-behind, denied write-behind with retained dirty authority, post-effect
possible settlement, and terminal residue.

### Truth and authority

- `worth-store-buffer-pool` owns physical allocation, resident identity,
  fault ownership, pins, dirty truth, writeback claims, eviction eligibility,
  exact speculative grants, and exact counters. It imports no Signal,
  Foundational, `worth-proof`, scheduler, backend receipt, or Store-semantic
  authority.
- `worth-store` owns current Store/runtime generation, record/frame meaning,
  security, Foundational semantic basis selection, physical work identity,
  Signal profile admission, and terminal Store effect classification.
- Worth Signal owns generic readiness, cancellation, timeout, retry, and
  derived completion. Phase 6 creates no Signal family.
- `worth-store-io-scheduler` owns secure-I/O preservation, resource admission,
  queue kind/durability lowering, and dispatch order. Scheduler completion is
  not dirty/clean settlement authority.
- `worth-store-physical-backend` owns exact filesystem effects and receipts.
- physical format owns artifact and frame coordinate meaning.
- C.10 still owns QoS/fairness/stable-read policy. Phase 6 may consume current
  scheduler contracts but may not invent successor policy.

### Weaker representations that must open no door

A kind enum, scope enum, Store identity, coordinate, allocation byte count,
counter, scheduler shape, generic queue completion, backend receipt, Signal
request, Foundational fact, or `worth-proof` witness cannot construct or
substitute a Prefetch, ReadAhead, or WriteBehind grant. Governed entry points
consume the concrete pool-issued grant or claim. Queue kind is selected by the
grant type, not by a caller enum or debug assertion.

### Destination topology

```text
worth-store-buffer-pool/src/physical_residency/
├── speculation.rs                         # narrow speculation facade
├── speculation/
│   ├── admission.rs                       # kind-specific pool admission
│   ├── admission_attempt.rs               # exact attempt/denial RAII
│   ├── read_grant.rs                      # Prefetch and ReadAhead grants
│   ├── writebehind_grant.rs               # WriteBehind grant
│   └── queue_declaration/
│       ├── context.rs                     # grouping + resource context
│       ├── read.rs                        # grant-selected Prefetch/ReadAhead declaration
│       ├── writeback.rs                   # claim-selected writeback declaration
│       └── mod.rs                         # semantic exports only
└── tests/
    ├── speculation.rs                     # malformed, exact, scope pressure
    ├── speculation_limits.rs              # per-kind and total envelope
    ├── shutdown.rs                        # every live grant classified
    └── writeback_claim_exclusion.rs       # exact WriteBehind claim exclusion

worth-store/src/physical_runtime/
├── work/
│   ├── declaration/contract.rs            # one operation → Signal-family policy
│   ├── authority.rs                       # admitted family carried as authority
│   ├── observation/causal.rs              # admitted family in causal evidence
│   └── scheduler_demand/residency.rs       # typed speculative demand lowering
└── record_serving/
    ├── read_work_port/
    │   └── scheduler_preparation.rs        # secure-I/O + canonical scheduler path
    └── residency/
        ├── speculation.rs                  # Store-private speculation facade
        ├── speculation/
        │   ├── intent.rs                   # distinct Store intents
        │   ├── work_submission.rs          # canonical work submission
        │   ├── outcome.rs                  # exact public outcomes
        │   └── failure.rs                  # retained/typed failure meaning
        ├── frame_loading/speculative.rs    # grants consume canonical source
        └── scheduled_writeback.rs          # possible-effect dispatch/settlement

store-test-runner/src/physical_residency_boundary_gate/
├── runtime_ownership.rs                   # local-runtime and relocation mutants
├── removal_inventory.rs                   # phase-owned deletion truth
└── shutdown_cleanup.rs                    # post-close exact release mutants
```

A one-file directory remains valid where it preserves a committed semantic
growth axis. No Phase 7 adapter directory or empty successor shell is created.

### Intended private DX

```rust
match residency.prefetch(PhysicalPrefetchIntent::new(coordinate)) {
    PhysicalPrefetchOutcome::Hit { .. } => {}
    PhysicalPrefetchOutcome::Coalesced { .. } => {}
    PhysicalPrefetchOutcome::Loaded { work, .. } => observe(work),
    PhysicalPrefetchOutcome::Dropped(pressure) => defer(pressure),
    PhysicalPrefetchOutcome::Failed(failure) => classify(failure),
}

let batch = residency.read_ahead(PhysicalReadAheadIntent::new(&coordinates)?)?;

let prepared = residency.prepare_writeback(dirty, durability)?;
let ready = residency.request_writeback(prepared)?;
let admitted = residency.admit_writeback(ready)?;
let terminal = residency.execute_writeback(admitted)?;
```

Ordinary product callers do not receive these certification/private controls.
Invalid grant substitution, copying, reconstruction, raw frame-port access,
and direct scheduler/backend dispatch must fail at compile time or visibility.

## Closure Guarantees

| ID | Exact closure claim | Required evidence | Current result |
| --- | --- | --- | --- |
| `C6-P6-L01` | The final source authority is complete and reproducible across tracked, deleted, renamed, and untracked files while excluding only this ledger. | Independently reproduced path/status/blob manifest, counts, bytes, and SHA-256 bound to the final evidence run. | `PROVED` — `E16`: 285 exact-byte entries, 42,241 manifest bytes, SHA-256 `0b01e23387a030948792f4fd75e396f028ebb1cd3d162d128f60e776dfa16981`; Git no-filter blob IDs and independent raw blob framing agree. |
| `C6-P6-L02` | The ledger covers every Phase 6 must-ship, preserve, proof, cleanup, API, documentation-when-relevant, semantic, lifecycle, performance, and causally necessary intent guarantee. | Clause-to-row map, risk map, finding history, evidence index, QA-tests audit, composition dispositions, and final surviving-defect attack. | `PROVED` — requirement coverage, risk map, F001–F021 history, evidence index, QA-tests audit, 102-candidate structural disposition, and all twelve surviving-defect questions close without a credible surviving implementation. |
| `C6-P6-P01` | Prefetch and ReadAhead have distinct typed Store intents; Prefetch, ReadAhead, and WriteBehind have distinct move-owned concrete pool-issued grants/claims. A kind enum, scalar, copied grant, wrong grant kind, or foreign/stale allocation opens no governed path. | Constructor/visibility review, grant signatures, compile-fail docs, kind-substitution specimens, foreign-incarnation tests. | `PROVED` — `E03`, `E04`, `E06`, `E09`, `E12`: sealed constructors and compile-fail cross-kind/foreign/stale specimens require the concrete move-owned authority. |
| `C6-P6-P02` | Every authenticated, structurally kind-bearing request records exact attempts, admissions, completions, denials, active frames, and peak frames; Drop and every post-attempt error path settle exactly once. Foreign allocation authority and a writeback set that cannot yet identify a valid kind request are rejected before kind counters and mutate no speculative observation. | Exact lower tests for malformed, success, one-past, drop, failure, foreign authority, and shutdown; RAII/manual writeback source trace. | `PROVED` — `E03`, `E06`, `E10`: RAII attempt settlement and exact all-kind/drop/failure/shutdown counters reconcile with zero residual activity. |
| `C6-P6-P03` | Exact scope, kind ceiling, operation bytes, and total envelope admit before allocation or work. Prefetch/ReadAhead require typed ForegroundRead grants; WriteBehind requires typed ForegroundWrite authority. Combined kinds cannot escape the global envelope. | Typed API inspection; exact pressure tuples; per-kind limits; combined-envelope and shared-read-scope tests; allocation-event reconciliation. | `PROVED` — `E03`, `E06`, `E10`: exact one-past tuples, simultaneous three-kind envelope, scope theft specimens, and allocation-event reconciliation are green. |
| `C6-P6-P04` | Every effectful Prefetch or ReadAhead miss uses the canonical record-read path: inherited Foundational read basis → admitted `ReadFault` → secure-I/O preservation → scheduler demand → executor → backend read receipt → Store settlement. | Source trace, direct causal family/operation/backend assertions, scheduler lowering tests, real-file cold/mixed/partial journeys. | `PROVED` — `E04`, `E06`, `E08`, `E12`: real-file cold, mixed, coalesced, and partial journeys bind `ReadFault`, scheduler plan, backend receipt, and Store settlement on the same causal work identity. |
| `C6-P6-P05` | Every effectful WriteBehind uses dirty claim → dedicated frame-writeback basis → admitted `ExactWriteback` → scheduler → executor → backend write receipt → Store settlement → exact pool clean/retry/inspection transition. | Direct causal operation/family/backend/fate/derived-completion assertions, writeback journeys, exact settlement and receipt tests. | `PROVED` — `E03`, `E05`, `E06`, `E08`: accepted, retryable, and indeterminate writeback evidence binds operation, `ExactWriteback`, backend receipt, fate, derived completion, and pool transition. |
| `C6-P6-P06` | A speculative hit, coalesced waiter, and every pre-effect pressure/malformed denial create no fake Signal request, scheduler admission, physical work, or media effect. Duplicate cold faults have one source owner and one canonical work effect. | Before/after causal/work/scheduler/media oracles, coalescing gate, source-load counters, hot/mixed journeys. | `PROVED` — `E03`, `E06`, `E08`: exact before/after oracles and whole-episode coalescing prove zero invented work and one owner/media effect. |
| `C6-P6-P07` | Denied read speculation drops/defers with exact pressure and retry posture. Denied WriteBehind returns the same dirty authority, performs no work/media, and can retry cleanly after release. | Store pressure outcomes, dirty-frame identity and counters, zero-work/media comparisons, accepted retry and final zero dirty state. | `PROVED` — `E06`, `E08`: Prefetch/ReadAhead drops expose exact pressure with zero effects; WriteBehind denial retains exact dirty identity and later settles clean. |
| `C6-P6-P08` | Close classifies every live grant and every possible effect. It demonstrably enters shutdown and cannot complete safe cancellation until post-read or post-write settlement is joined; terminal counters and residue are exact. | Lower live-grant shutdown matrix; Store execution checkpoints; exact close-phase observation; repeated schedule stress; shutdown-cleanup mutants. | `PROVED` — `E03`, `E06`, `E10`: exact `AdmissionStopped`/not-`SafeCancellationComplete` observation, lower grant matrix, cleanup mutants, and 20 repeated runs for each possible-effect checkpoint are green. |
| `C6-P6-P09` | One closed operation→Signal-family classifier selects `ReadFault` for metadata/range reads, `ExactWriteback` for range writes, and `Publication` for publication. The selected family is sealed into admitted authority and carried into causal evidence. No speculative Signal family or duplicate classifier exists. | Unique-source search, classifier matrix, authority flow trace, direct Prefetch/ReadAhead/WriteBehind causal assertions, profile tests. | `PROVED` — `E06`, `E08`, `E12`: one classifier definition exists, admitted authority carries its closed result, and direct causal evidence agrees for every Phase 6 kind. |
| `C6-P6-P10` | Queue declaration kind is selected by concrete grant type; WriteBehind durability is typed; scheduler declarations derive kind/durability/grouping from typed lower declarations. Runtime kind enums, Boolean selectors, seven-axis constructors, and debug-only exclusion do not govern correctness. | API/source review, compile-fail grant substitution, Prefetch/ReadAhead/WriteBehind scheduler lowering, durability tests, obsolete API absence searches. | `PROVED` — `E03`, `E04`, `E08`, `E12`: typed read/writeback declarations, private evidence sum, specialized lowerers, compile-fail substitutions, and obsolete-setter/API absence close the guarantee. |
| `C6-P6-P11` | Store exposes composed speculation operations, not raw frame ports, canonical media sources, pool controls, or backend dispatch. Ordinary exports remain record-intent oriented; certification authority remains feature-gated and runtime-bound. | Export/visibility review, serving-capability gate, authority UI suite, direct-media/dependency gates. | `PROVED` — `E06`, `E09`, `E10`, `E12`: all 32 authority specimens and source/export gates reject raw or unbound authority escape. |
| `C6-P6-P12` | Semantic source audit finds no pool-local worker, work queue, retry loop, callback registry, timer, pending-work registry, second runtime owner, or isolated S.2 speculative model. Concrete worker, channel, and pending-runtime families are mechanically rejected across the lower owner trees; current speculation directories additionally reject named scheduling-queue, timer, retry, callback, and worker-coordination fragments. | Full lower-owner source audit; two-tier source gate; controlled thread, pending-registry, and channel relocation mutants; constructor uniqueness; Phase 6 removal rows; replacement-owner existence; metadata/source absence. | `PROVED` — `E10`, `E12`, `E15`: semantic full-tree audit is clean, 70 boundary predicates include relocation mutants, both Phase 6 removal rows resolve to deleted paths and present replacement owners. |
| `C6-P6-P13` | Tests are honest production evidence: real initialized files, real pool grants, real Signal/scheduler/executor/backend boundaries, independent media/work/counter/dirty/close oracles, exact fault selectors, and bounded teardown. Fixture-owned grants or self-certified counters do not substitute for the production world. | QA-tests setup/action/observation/teardown audit, real-file journeys, exact selector calibration, compile-fail harness review, cost accounting. | `PROVED` — `E03`–`E10`: fixture and test bodies were traced through production authority; independent media/file/causal/dirty/lifecycle oracles and intended-cause fault calibration are present. |
| `C6-P6-P14` | Phase 6 evidence is mutation-sensitive at its causal boundaries: kind bypass, scope theft, total-envelope bypass, fake hit work, duplicate source load, Signal-family drift, scheduler/backend bypass, concrete worker/channel/pending-runtime relocation, early close, dirty loss, and cleanup suppression each fail a localized predicate. | Controlled source mutants, compile-fail specimens, exact runtime predicates, strengthened relocation and shutdown gates, surviving-defect attack. | `PROVED` — `E03`, `E06`, `E08`–`E10`: each named mutant class is bound to a compile failure, controlled source mutant, or exact consequential runtime predicate. |
| `C6-P6-P15` | Phase 6 preserves C.10 ownership and lower-owner purity: no QoS/fairness/stable-read policy, pool timer, Signal/Foundational/`worth-proof` import, Query/replay authority, branch/MVCC meaning, or Phase 7 adapter authority is introduced. | Dependency/source/feature scans, boundary checker, agent context, successor seam review. | `PROVED` — `E12`–`E15`: lower-owner purity and successor-scope searches are clean; boundary-check and agent-context both pass. |
| `C6-P6-P16` | Final Phase 6 source is formatted, warning-clean, within the 400-line cap, semantically composed, API-sharp, dependency-honest, and accepted by focused and broad owner suites plus mandatory constitutional gates. Relevant private documentation, removal truth, and this closure contract are current. | Formatting, checks/tests, function scrutiny, line-cap audit, cleanup inventory, full owner/certification suites, boundary-check, agent-context. | `PROVED` — `E01`–`E16`: all Phase 6 owners and evidence gates pass; 263 dirty Rust files have zero cap violations and all 102 advisories were inspected. Scoped repository caveat: the global line-cap script remains red on 114 clean pre-existing files, none dirty or Phase 6-caused. |

## Requirement Coverage

| Governing Phase 6 obligation | Ledger rows |
| --- | --- |
| Distinct typed intent and grant for every kind | `P01`, `P10`, `P11` |
| Exact attempts/admissions/completions/denials/live/peak | `P02`, `P03` |
| Scope and total-envelope admission before work/allocation | `P03`, `P06`, `P14` |
| Signal → scheduler → executor → Store settlement | `P04`, `P05`, `P09`, `P10` |
| Drop/defer denied reads; retain dirty on denied write-behind | `P06`, `P07` |
| Shutdown classification of every grant and possible effect | `P02`, `P08` |
| Reuse `ReadFault` and `ExactWriteback`; no new family | `P04`, `P05`, `P09` |
| Preserve C.10 and Signal lifecycle ownership | `P08`, `P12`, `P15` |
| Exact limit, one-past, combined envelope | `P02`, `P03` |
| Hit zero media; miss canonical | `P04`, `P06`, `P13` |
| Local-worker, kind-bypass, scope-theft mutants fail | `P01`, `P03`, `P10`, `P12`, `P14` |
| Delete isolated S.2 model and pool worker/retry/queue/callback/timer | `P12`, `P15` |
| APIs and semantic sharpness | `P01`, `P09`, `P10`, `P11`, `P16` |
| Compile-time enforcement over conscientiousness | `P01`, `P03`, `P09`, `P10`, `P11` |
| Documentation when relevant | `L02`, `P12`, `P16` — private contract and cleanup truth live in the spec, removal ledger, source topology, and this closure ledger; Phase 9 still owns the public guide |

## Risk Map

| Risk | Earliest honest boundary | Required detector |
| --- | --- | --- |
| Allocation/work begins before kind/scope admission | pool admission | typed grants, exact allocation counters, zero-work denial oracle |
| Kind limit indexed incorrectly or bypassed | speculative attempt | all-kind exact/one-past/peak matrix |
| Combined envelope ignores one kind | operation accounting | simultaneous three-kind envelope test |
| Prefetch grant enters ReadAhead or WriteBehind | queue declaration | sealed grant trait and compile-fail substitution |
| Read miss bypasses Signal/scheduler | Store read port | causal family/operation/backend record |
| WriteBehind cleans from queue/backend receipt alone | Store settlement | dirty-retention and exact settlement tests |
| Hit invents work | frame access | before/after media/work/causal equality |
| Duplicate cold fault performs two reads | fault ownership | paused owner, coalesced waiter, source-load oracle |
| Close test false-passes because close never started | shutdown protocol | exact `AdmissionStopped` observation and absence of `SafeCancellationComplete` |
| Operation→Signal mapping drifts between admission and routing | work contract | one classifier sealed into authority |
| Queue kind/durability drift independently | lower declaration | grant-selected kind and typed durability |
| Local worker is relocated outside scanned path | lower owner tree | consequential-fragment full-tree gate and relocation mutants |
| Cleanup pulls Phase 7/8 forward or keeps legacy | removal ledger | exact phase/status grouping and absence gate |
| Tests certify their own counters | journey boundary | independent media, causal work, file, dirty, and lifecycle observations |
| Broad refactor creates god file | composition boundary | 400-line gate and semantic function scrutiny |

## QA-Tests Evidence Audit

| Axis | Phase 6 evidence posture |
| --- | --- |
| World honesty | Store journeys initialize and reopen real qualified filesystem media. Lower tests issue real pool-owned allocation grants and grants; no fixture mints authority fields. |
| Boundary honesty | Effectful journeys traverse production Signal, secure-I/O, scheduler, executor, backend, and Store settlement. Compile-fail tests use external consumer crates. |
| Oracle independence | Media counters, causal work records, residency counters, dirty authority, close phases, and fresh file truth are compared; no single counter proves itself. |
| Adversarial pressure | Each kind reaches its exact ceiling and one-past denial; combined envelope, malformed reads, duplicate frames, partial batch, coalescing, dirty retention, and close races are exercised. |
| Fault honesty | The read-ahead failure calibrates the real bootstrap read ordinal, injects at the second production read, and checks exact partial shape and media attempts. |
| Lifecycle honesty | Possible-effect close tests wait for exact shutdown phase rather than relying on elapsed time, then verify terminal zero residue. |
| Harness integrity | Trybuild specimens assert intended pass/fail direction; controlled boundary mutants invoke the same scanners used on real sources. |
| Proof economy | Long tests retain one cumulative causal narrative where splitting would destroy exact counter sequencing; repeated setup is held in responsibility-named fixtures. |
| Cost honesty | Fast owner suites are separated from the known long authority/certification suites; long runtime is reported rather than treated as a hang or hidden by narrower claims. |

## Composition Dispositions

The function scanner is advisory. Each Phase 6-facing candidate is reviewed by
semantic responsibility:

- `PoolInner::validate_transition` is one closed dirty-replacement admission
  policy over exact state, scope, identity, and bytes; its six inputs are the
  full visible contract.
- scheduler `budget_from_shape` is one exhaustive typed field-lowering step.
  Extracting generic field machinery would erase resource-unit types.
- scheduler queue work declaration, producer normalization, background-lease
  policy, and resource-budget conversion are separate semantic
  responsibilities. Their temporary coexistence in `policy/work.rs` exceeded
  the hard file cap and hid an invalid dual-evidence state; final composition
  requires responsibility-named modules and one mutually exclusive
  buffer-pool evidence sum.
- scheduled backend range-write entry points are responsibility-named by
  foreground/write-behind and existing-range/append posture. Their explicit
  inputs expose all effect context; the shared private context does not erase
  the public lane distinction.
- Store artifact-tree methods are pure physical-layout facades, not hidden
  workflow owners.
- `write_frame_via_writeback` is an explicit typestate orchestrator. Its
  prepare → ready → admit → execute → classify sequence is the table of
  contents; error branches preserve exact retained authority.
- `FrameWritebackPort::new` constructs one capability aggregate from seven
  independently typed authorities. Hiding them in an anonymous parameter bag
  would reduce honesty.
- residency failure projections are exhaustive, distinct semantic
  classifications; their length comes from the closed denial vocabulary.
- bounded/exact loader functions own one fault-ownership classification each;
  hit, coalesced, and fault branches must remain visibly adjacent.
- long limit, pressure, coalescing, partial, write-behind, and shutdown tests
  are single cumulative adversarial narratives with exact before/after
  counters. Responsibility-named assertion helpers are used where they do not
  destroy sequence.

Corrections made during review:

- split the over-cap `scheduler_demand.rs` by the semantic residency axis;
- decomposed backend exact-write validation, target preparation, effect, and
  settlement while keeping mutation guards live across the effect;
- replaced Boolean queue-fixture selection with named typed entry points and
  separated shared world construction from kind-specific grant progression.

## Finding History

| ID | Finding | Correction | Reopened rows |
| --- | --- | --- | --- |
| `C6-P6-F001` | Speculative attempt/denial accounting could be skipped or doubled across early returns and Drop. | Added `SpeculativeAdmissionAttempt` RAII: exactly one attempt, unresolved Drop denial, consuming successful permit. | `P02`, `P03`, `P08` |
| `C6-P6-F002` | Per-kind one-past and simultaneous total-envelope evidence was incomplete. | Added exact Prefetch/ReadAhead/WriteBehind and combined-envelope tests with live/peak reconciliation. | `P02`, `P03`, `P13`, `P14` |
| `C6-P6-F003` | Possible-effect close lacked a post-write/pre-scheduler checkpoint and could not prove every effect fate joined. | Split scheduled writeback dispatch/effect settlement and added exact read/write possible-effect checkpoints. | `P05`, `P08`, `P13` |
| `C6-P6-F004` | Buffer-pool queue declaration leaked backend durability vocabulary. | Added pool-owned write durability and lowered it at the Store/scheduler boundary. | `P10`, `P15` |
| `C6-P6-F005` | Store residency exposed raw frame ports/canonical source access. | Replaced raw escape with composed admit/load Prefetch and ReadAhead operations. | `P04`, `P11`, `P15` |
| `C6-P6-F006` | A read queue declaration accepted a runtime kind enum, including WriteBack, and relied on `debug_assert!`. | Added a sealed `BufferPoolReadQueueGrant`; concrete Prefetch/ReadAhead grant types select kind. Scheduler constructors consume typed WAL or pool declarations. | `P01`, `P10`, `P14` |
| `C6-P6-F007` | `scheduler_demand.rs` exceeded 400 lines and mixed ordinary and residency lowering. | Moved residency demand into `scheduler_demand/residency.rs`. | `P16` |
| `C6-P6-F008` | Backend exact range write collapsed validation, authority, target preparation, serialization, effect, durability, and receipt construction in one 133-line function. | Added typed request/posture/context and named preparation/effect/settlement steps; mutation guards remain live across the effect. | `P05`, `P16` |
| `C6-P6-F009` | Shutdown tests could false-pass a 50 ms negative receive if the close thread had not started. | Observe exact close progress: require `AdmissionStopped` and forbid `SafeCancellationComplete` before gate release; stress-run all kinds. | `P08`, `P13`, `P14` |
| `C6-P6-F010` | Operation→Signal-family policy was duplicated between admission and routing. | Established one closed classifier, sealed the selected family into admitted authority, and carried it into causal evidence. | `P04`, `P05`, `P09`, `P14` |
| `C6-P6-F011` | Scheduler test support used a Boolean to choose ReadAhead versus WriteBack and mixed shared world construction with three grant progressions. | Added named Prefetch/ReadAhead/WriteBehind fixtures and a shared typed fixture world; deleted the Boolean API without alias. | `P10`, `P13`, `P16` |
| `C6-P6-F012` | Local-worker gate scanned only current speculation directories and could be evaded by relocation. | Added consequential-fragment scans across complete lower owner trees and relocation mutants. | `P12`, `P14`, `P15` |
| `C6-P6-F013` | Successful WriteBehind asserted Signal family but not operation, backend receipt, fate, and derived completion on the same causal record. | Bound all five facts to each accepted write-behind settlement. | `P05`, `P13`, `P14` |
| `C6-P6-F014` | Coalescing proved no waiter work only while the source owner was paused; a post-wake fake waiter submission could survive. | Bound the entire owner/waiter episode to exactly one terminal causal record, exactly one media read, and the owner-reported work identity; stress-ran the schedule. | `P06`, `P13`, `P14` |
| `C6-P6-F015` | Lower tests proved ReadAhead one-past admission, but no Store journey proved the observable pressure drop, zero-work boundary, and terminal reconciliation. | Added a held-grant Store ReadAhead saturation journey with exact pressure, zero work/media/causal deltas, canonical owner completion, and terminal counter reconciliation; stress-ran it 20 times and reran all nine speculative journeys. | `P07`, `P13`, `P14` |
| `C6-P6-F016` | `PhysicalSchedulerDemand::residency_writeback` accepted the generic buffer-pool queue declaration, so a valid read declaration could be paired with range-write work and lowered with read kind/durability; the API did not make the mismatch impossible. | Replaced the generic declaration with separate read and writeback types in a semantic directory, split scheduler lowerers and evidence slots, made writeback posture concrete, removed the runtime `NotWriteback` branch, and added compile-fail cross-kind substitution plus exact lowering assertions. | `P05`, `P10`, `P13`, `P14`, `P16` |
| `C6-P6-F017` | The Phase 6 removal rows and closure topology still named the pre-split `queue_declaration.rs`, while the replacement owner had become the `queue_declaration/` semantic directory; the gate required only a nonempty owner string and could not detect the drift. | Updated both inventories and added a boundary predicate that resolves every completed Phase 6 replacement owner under its originating crate's `src/` tree and requires the path to exist. | `P12`, `P16` |
| `C6-P6-F018` | P12 claimed that relocating any queue/retry/callback/timer machinery anywhere would fail, but the broad gate intentionally permits synchronous `VecDeque` and `Condvar` uses and mechanically rejects only concrete consequential runtime families. | Separated semantic source-audited absence from mechanically rejected worker/channel/pending-runtime families, named the stricter current-speculation fragments exactly, and required a final semantic lower-owner audit. | `P12`, `P14`, `P16` |
| `C6-P6-F019` | The ReadAhead saturation journey accepted `PhysicalReadAheadOutcome::Complete(_)` for the held owner, but aggregate completion could contain a failed frame and did not prove the ledger's canonical-load claim. | Required exactly one `Loaded` frame for the held coordinate with zero hits, coalescing, or failures; stress-ran the stronger oracle 20 times and reran all nine speculative journeys. | `P07`, `P13`, `P14` |
| `C6-P6-F020` | `queue_execution/policy/work.rs` reached 420 lines and mixed queue declaration state, producer normalization, background-lease policy, and resource-budget conversion, violating the hard file cap and obscuring future insertion boundaries. | Replaced the mixed file with `work_declaration.rs`, `producer_lowering.rs`, `background_lease.rs`, and `resource_budget.rs`; preserved the public facade; reran all-dirty scrutiny over 263 Rust files with 102 inspected advisories, zero scan errors, and no file over 400 lines. | `L02`, `P10`, `P16` |
| `C6-P6-F021` | `QueueWorkDeclaration` stored buffer-pool read and writeback evidence in independent `Option` fields, so internal construction could represent both simultaneously and correctness depended on setters being called conscientiously. | Replaced the two slots and post-construction setters with one private `BufferPoolQueueExecutionEvidence` sum under a single `Option`; typed producer specializations construct the exact alternative directly, obsolete setters are absent, and scheduler/compile-fail/certification evidence is green. | `L02`, `P10`, `P14`, `P16` |

No finding is closed merely because a test became green. Every correction
reopens the rows shown and requires final-source evidence.

## Evidence Index

- `E01` — `cargo fmt --manifest-path workspaces/worth-store/Cargo.toml --all -- --check`
- `E02` — warnings-denied all-target/all-feature checks for buffer pool,
  scheduler, physical backend, Store, test support, certification, and runner
- `E03` — full `worth-store-buffer-pool` tests and 52 compile-fail docs
- `E04` — full `worth-store-io-scheduler` 89-test suite and seven docs
- `E05` — full `worth-store-physical-backend` 184-test suite and 28 docs
- `E06` — Store unit, speculative, writeback, physical-record journey, process,
  and doc suites
- `E07` — `worth-store-test-support` full suite
- `E08` — certification library/scenario/compile-fail/queue/bin/doc suites,
  split where the legitimate operational-security runtime exceeds one minute
- `E09` — `physical_runtime_authority_ui`: all 32 intended pass/fail specimens
- `E10` — `store-test-runner physical_residency_boundary_gate`: 70 predicates
- `E11` — dirty Rust function scrutiny: 263 files, 102/102 advisory
  candidates inspected, zero scanner errors, JSON SHA-256
  `566f85647dad9f2bb6a1a8d5c3b9819133de69d87512f75195ae3d375cb7879f`;
  warning-free exact dirty-file audit: zero files over 400 lines, largest 367;
  forbidden catch-all filename scan clean. The repository-wide line-cap script
  is explicitly red on 114 clean pre-existing files and has stdout SHA-256
  `e9f366dcc88293a91ca7fbb312cedd95190b1128481caa9f09d9e6a4e2983daa`.
- `E12` — exact source/API/dependency/Signal-family/cleanup/legacy absence
  searches and call-path inspection
- `E13` — `cargo run --manifest-path tools/boundary-check/Cargo.toml -- --root .`
- `E14` — `cargo run --manifest-path tools/agent-context/Cargo.toml -- check`
- `E15` — Phase 6 removal rows, deleted-path verification, replacement-owner
  existence, and explicit Phase 7/8 open-row separation
- `E16` — exact working-tree-byte source manifest: base
  `95afd3a9ac80967d9a31ce75a80cad98af8c0604`, 285 entries excluding only
  this ledger, 197 modified, 14 deleted, 74 untracked, 42,241 bytes, SHA-256
  `0b01e23387a030948792f4fd75e396f028ebb1cd3d162d128f60e776dfa16981`;
  `git hash-object --no-filters` and independent raw Git blob framing agree
  entry-for-entry.

## Final Evidence Rebound To The Frozen Source

The following evidence passed after the final F020/F021 production corrections
and before the exact-byte source freeze. No source file changed between these
evidence runs and the freeze; subsequent edits affect only this excluded
ledger.

- Store all-target/all-feature warnings-denied check;
- Store 75 unit tests;
- nine speculative journeys, including 20 repeated runs of the Store ReadAhead
  saturation journey;
- four residency-writeback journeys;
- typed buffer-pool declaration cutover: scheduler 89 tests plus seven docs,
  buffer-pool 52 compile-fail docs, Store four docs, and scheduler-queue
  certification seven tests;
- scheduler policy composition and evidence-sum correction: warnings-denied
  all-target/all-feature check, 89 tests, seven docs, 52 buffer-pool
  compile-fail docs, four Store compile-fail docs, seven scheduler-queue
  certification tests, and an all-dirty 400-line audit with no violation;
- 20 repeated runs of all three possible-effect shutdown tests;
- physical backend 184 tests and 28 docs;
- scheduler 89 tests and seven docs;
- buffer-pool 52 compile-fail docs;
- authority UI suite with 32 pass/fail specimens;
- physical-residency boundary gate with 70 predicates;
- unique operation→Signal-family source search;
- deleted Phase 6 removal paths and present replacement owner;
- function scrutiny with no scan errors before final freeze.

The controlled full-certification rerun exited zero with 24 named Cargo targets
plus docs, no failure marker, stdout SHA-256
`044243a3fcc7c9720315d1c8879c689aa69da2453bce67e6a9893a1eab95cea0`,
and stderr SHA-256
`91ba5b619ce75b5df0ae9a029b4c5cf3282a6d597353e0a08076e3beca11a266`.
The largest suites reported 403 passed with one ignored and 284 passed with
five ignored; the scheduler queue target reported seven passed and docs
reported 59 passed.

## Final Surviving-Defect Attack

Before closure, independently answer:

1. Can a read hit or pressure denial create any Signal, scheduler, work, or
   media record while all rows still pass?
2. Can a miss use the right bytes but the wrong Signal family, Foundational
   basis, secure-I/O posture, queue kind, scheduler binding, or backend receipt?
3. Can a WriteBehind denial lose, replace, or prematurely clean dirty
   authority while counters still reconcile?
4. Can one kind consume another kind's ceiling or another operation scope?
5. Can a grant be copied, forged, reconstructed, or substituted through a
   public enum or generic constructor?
6. Can close finish without classifying a live grant or possible effect?
7. Can a worker, queue, retry loop, callback, timer, or pending registry be
   moved outside the currently scanned directory?
8. Can obsolete Phase 6 code remain under a compatibility or legacy name?
9. Can a fixture bypass production Signal/scheduler/backend boundaries or
   certify itself with the same counter it mutates?
10. Can Phase 6 import C.10 policy, Query/replay, branch/MVCC, or Phase 7
    adapter authority?
11. Can any dirty Rust file exceed 400 lines or any advisory hide mixed
    semantic responsibility?
12. Can any named guarantee remain supported only by an indirect search,
    narrow test, stale artifact, or pre-correction run?

Any plausible “yes” is a new finding, reopens its affected rows, and blocks
Phase 7.

Final attack result: all twelve answers are **no** against the frozen source.
Questions 1–2 close through `P04`, `P06`, `P09`, and `P10`; 3 through `P05`
and `P07`; 4 through `P03`; 5 through `P01` and the compile-fail evidence; 6
through `P08`; 7–8 through `P12`, `P14`, and `P15`; 9 through `P13`; 10
through `P15`; 11 through `P16` and `E11`; and 12 through `L01`, `L02`, and
`E16`. No guarantee depends only on absence search or a pre-correction run.

## Final Source Freeze

`PROVED`.

- Base: `95afd3a9ac80967d9a31ce75a80cad98af8c0604`
- Exclusion: only
  `_docs/worth-store/physical-reconstruction-c6-phase-6-closure-ledger.md`
- Entries: 285 total; 197 modified, 14 deleted, 74 untracked
- Schema: `<two-character porcelain status>\t<exact-byte blob ID or
  DELETED>\t<repository path>\n`, paths ordered ordinally
- Blob semantics: exact working-tree bytes, without clean/smudge or line-ending
  filters
- Manifest bytes: 42,241
- SHA-256:
  `0b01e23387a030948792f4fd75e396f028ebb1cd3d162d128f60e776dfa16981`
- Reproduction A: `git hash-object --no-filters`
- Reproduction B: raw SHA-1 over `blob <byte-length>\0<file-bytes>`
- Result: both methods agree entry-for-entry and produce the same manifest
  SHA-256

## Closure Decision

`PROVED` — Phase 6 is closed. Phase 7 may begin only from a fresh
`plan-implementation` boundary review.
