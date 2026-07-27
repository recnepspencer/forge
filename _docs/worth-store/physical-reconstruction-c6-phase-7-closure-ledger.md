# C.6 Phase 7 Closure Ledger

## Scope

This ledger audits C.6 Phase 7, **Cut Over Every Ordinary Consumer And Future
Adapter**, against the governing C.6 specification, the WORTH engineering
constitution, the real Store facade and lower physical owners, and defects that
could satisfy the phase wording while defeating its intent.

Phase 8 and later remain out of scope. Phase 7 does not implement successor
integrity, isolation, QoS, stable-read, recovery, maintenance, or blob policy,
and it does not delete the separately inventoried Phase 8 physical model. The
product is unreleased: Phase 7 predecessors are deleted rather than retained
behind aliases, compatibility exports, or migration machinery.

## Audit Source State

The audit began from base commit
`95afd3a9ac80967d9a31ce75a80cad98af8c0604` with 351 porcelain entries: 199
modified, 38 deleted, and 114 untracked. This ledger is excluded from its own
final source fingerprint. The final fingerprint contains 367 entries excluding
only this ledger: 211 modified, 38 deleted, and 118 untracked. Its 53,676-byte
manifest has SHA-256
`c477353b7e8945de26bccbc2537e4b45ff0b300f05411a444f26b3f0864675c7`;
Git no-filter blob IDs and independent raw Git blob framing agree
entry-for-entry.

## Boundary Brief

### Adversarial constraint

Phase 7 must survive this hostile condition:

> An ordinary integrity, isolation, or blob adapter tries to turn a borrowed
> chunk basis, Store generation, pressure report, counter, scope enum, or
> certification observation into cache-control authority. At the same time,
> Recovery saturates its exact scope while Scrub, Maintenance, Verification,
> and Blob contend under the global operation envelope. An attacker imports the
> pool directly, activates certification or replay through a feature edge,
> constructs a generation, infers semantic residency from physical residency,
> revives a phase-named facade under an alias, relocates a direct pool edge into
> an ordinary consumer, or changes the courtroom CLI/protocol/mutant identity
> without changing all consumers. The compiler, feature graph, runtime
> pressure evidence, removal inventory, and source-bound mutants must reject or
> expose every substitution.

Positive examples alone are insufficient. The proof must show both that future
physical adapters can consume the stable basis and that every stronger
authority remains impossible.

### Truth and authority

- Physical format owns artifact, frame-coordinate, and physical record shape.
- `worth-store-buffer-pool` owns physical allocation, exact scope accounting,
  resident identity, pins, dirty state, eviction, and lower counters. It owns no
  Signal, Foundational, `worth-proof`, Query, replay, or semantic-residency
  meaning.
- Store owns current Store generation, the ordinary records facade,
  responsibility-named pressure and observation projection, physical-work
  meaning, and composition of the inherited runtime.
- `worth-proof` owns the governed raw-policy-to-admitted-policy outcome. The
  admitted policy does not authorize a frame operation.
- Foundational owns the admitted Store-native read projection and dedicated
  frame-writeback bases. It does not classify pressure or residency.
- Signal owns derived readiness for real effectful work. Real misses reuse
  `ReadFault`; exact writeback reuses `ExactWriteback`; publication remains
  `Publication`. Hits, coalesced consumers, and pre-effect denials create no
  Signal request.
- Successor milestones own integrity, stable-read/isolation, QoS, recovery,
  maintenance, verification, and blob semantics. Phase 7 exposes physical
  inputs and pressure, not those meanings or their policy.

### Weaker representations that must open no door

A Store identity, generation value, record ID, frame coordinate, scope enum,
byte count, pressure report, counter snapshot, borrowed bytes, chunk basis,
certification evidence, or generic marker cannot construct a pool, allocation
grant, frame lease, dirty transition, eviction, retry, scheduler admission,
backend effect, semantic-residency claim, or successor proof.

`CertificationScopedAllocation` seals the concrete lower grant and exposes
only stable Store identity, scope, and bytes. It has no pool-incarnation,
operation, or grant-extraction API.

### Destination topology

```text
worth-store/src/bin/physical_store_work_courtroom/
└── bounded_residency/                    # executable lane and protocol schema

worth-store/src/physical_runtime/record_serving/
├── access/record_chunk_view.rs           # borrowed adapter basis
└── residency/
    ├── pressure_evidence.rs              # Store-owned pressure projection
    ├── residency_observation/            # identity-bound read-only evidence
    └── certification/
        └── scope_admission.rs            # certification-only sealed scope proof

worth-store/tests/
├── physical_adapter_authority/           # positive and hostile compiler cases
└── physical_record_journeys/
    └── successor_scope_admission.rs      # exact five-scope runtime proof

store-test-runner/src/
├── courtroom_campaign/
│   └── bounded_residency_siege/          # hostile bounded-residency evidence
└── physical_residency_boundary_gate/
    ├── ordinary_feature_graph.rs         # feature/dependency authority
    └── removal_inventory.rs              # predecessor and replacement truth
```

The nested directories are semantic growth axes, including where one file
currently occupies the directory. No catch-all file or empty successor shell is
introduced.

### Intended adapter DX

```rust
let mut session = runtime.records().open(record_id, limits)?;
while let Some(chunk) = session.next_chunk()? {
    inspect(
        chunk.bytes(),
        chunk.logical_range(),
        chunk.basis().store_generation(),
        chunk.basis().frame_coordinate(),
    )?;
}

if let Some(pressure) = session.observation().pressure() {
    defer(
        pressure.basis(),
        pressure.store_generation(),
        pressure.retry_posture(),
    );
}
```

The adapter borrows validated physical bytes and observes identity and
pressure. It does not receive the pool, retain the chunk past the session
borrow, forge the generation, or infer semantic residency.

## Closure Guarantees

| ID | Exact closure claim | Required evidence | Current result |
| --- | --- | --- | --- |
| `C6-P7-L01` | The final source state is complete and reproducible across tracked, deleted, renamed, and untracked files while excluding only this ledger. | Exact-byte manifest, independent hashes, status counts, final evidence rebound. | `PROVED` - `E13`: 367 entries, 53,676 bytes, SHA-256 `c477353b7e8945de26bccbc2537e4b45ff0b300f05411a444f26b3f0864675c7`; two blob implementations agree with zero mismatch. |
| `C6-P7-L02` | The ledger covers every Phase 7 must-ship, preserve, proof, cleanup, API, documentation, topology, test, and causally necessary intent guarantee. | Requirement map, risk map, finding history, evidence audit, composition review, surviving-defect attack. | `PROVED` - the requirement/risk maps, QA-tests audit, `F001`-`F013`, exact evidence index, and 17-question surviving-defect attack cover both stated and causally necessary guarantees. |
| `C6-P7-P01` | Every ordinary inline, extent, scan, manifest, publication, and writeback route uses the canonical bounded Store residency and inherited physical-work path; no direct pool/media or temporary handoff route competes. | Ordered production trace, journey inventory, direct-media and serving-capability gates, ordinary Store tests. | `PROVED` - 241 physical journeys, 78 boundary predicates, exact hot/truncation selectors, and accepted Courtroom C prove one bounded inherited route with zero live temporary path. |
| `C6-P7-P02` | C.9 integrity, C.10 isolation, and C.11 blob adapters can consume borrowed chunk bytes/basis, Store generation, pressure/observation, and admitted scope vocabulary without importing the pool. | Four ordinary positive compile specimens and public-export inspection. | `PROVED` - all four ordinary adapter specimens compile in the no-feature graph and consume only the responsibility-named borrowed/observational facade. |
| `C6-P7-P03` | Ordinary adapters cannot construct the pool, evict, dirty a frame, forge a generation, infer semantic residency, or access certification authority. | Six intended-cause compile failures plus ordinary feature graph. | `PROVED` - all six hostile specimens fail at their intended boundary; the three-test metadata-derived ordinary graph gate excludes forbidden authority from every ordinary root. |
| `C6-P7-P04` | Recovery, Scrub, Maintenance, Verification, and Blob have exact isolated scope ceilings inside one aggregate operation envelope; one-past denial is exact, one scope cannot steal another, release reconciles to zero, and the proof grants no pool operation. | Exact five-scope journey, API/visibility inspection, pressure tuple and release counters. | `PROVED` - the exact nonzero five-scope journey reaches every ceiling and one-past denial simultaneously, proves disjointness/global pressure, and reconciles release to zero. |
| `C6-P7-P05` | Ordinary and Part II feature graphs cannot import pool internals, certification authority, replay, or a legacy feature edge; Phase 8 direct-pool exceptions remain exact ledger-bound inventory rather than ordinary precedent. | Cargo tree/metadata/manifests, controlled graph mutants, boundary checker. | `PROVED` - the complete 78-predicate boundary family, three ordinary graph tests, exact manifest scan, and constitutional boundary checker pass; Phase 8 rows remain separately open. |
| `C6-P7-P06` | Public exports, executable mode, protocol/schema, journey, courtroom runner, timing identities, and mutation predicate are responsibility-named. No product identifier or test path begins with `C6`/`c6_`, no `c6_handoff` path exists, and no alias preserves it. | Exact path/token search excluding only anti-regression policy literals; compile and source-bound mutant tests. | `PROVED` - zero non-policy phase tokens, zero live phase-named paths, all 29 matching tracked paths deleted, current bounded-residency owners present, and 213 library plus two CLI runner tests pass. |
| `C6-P7-P07` | Every Phase 7 predecessor is absent and every ledger row names an exact present replacement owner. A stale open row, rediscovered deleted row, missing replacement, or unclassified consumer fails the gate. | 38 Phase 7 rows, ten removal tests, controlled mutants, old-path absence. | `PROVED` - all 38 Phase 7 rows are exact and deleted, every completed Phase 5-7 replacement resolves to a present `workspace:` path, and all ten hostile removal tests pass. |
| `C6-P7-P08` | Authority placement remains exact: `worth-proof` only governs policy admission, Foundational only governs real physical-work bases, Signal only governs real work readiness, and none enters the pool or becomes pressure/residency/cache truth. | Dependency/source gates, operation-to-Signal classifier, real miss/writeback journeys, adapter negatives. | `PROVED` - dependency/source gates, 241 real journeys, authority UI negatives, and 30 causal mutants preserve the exact proof/Foundational/Signal roles without pool or semantic substitution. |
| `C6-P7-P09` | The feature guide teaches the real ordinary API, marks direct speculative/scope operation as certification-only, distinguishes physical from semantic residency, names cleanup honestly, and keeps every Rust block compiler-bound. | Guide/source comparison, four-block binding test, runtime authority UI. | `PROVED` - all four guide blocks are exact compiler specimens inside the 31-case runtime authority UI; source/export review agrees with the documented ordinary and certification-only surfaces. |
| `C6-P7-P10` | Adapter, scope, graph, cleanup, and courtroom tests are causally honest, intended-cause sensitive, and cost-proportionate. The ordinary adapter UI remains a separate no-feature compiler graph because certification absence cannot be proved in the maximal-authority target. | QA-tests trace; nonzero test counts; compile-cost inventory; independent runtime observations. | `PROVED` - 213 library plus two CLI runner tests, 241 physical journeys, 31 runtime UI cases, ten adapter UI cases, exact focused selectors, 30 mutants, and independent Courtroom observations all execute nonzero intended-cause evidence. |
| `C6-P7-P11` | Destination topology and naming preserve semantic responsibility and future insertion; every dirty code/test file meets the 400-line law and advisory functions are inspected. | Full dirty inventory, function scrutiny, line-cap audit, topology review. | `PROVED` - all 313 dirty Rust files are at or below 400 lines; 119 advisory candidates were inspected with zero scan errors; pressure, cancellation, dirty-close, and replacement-owner responsibilities live in named modules. |
| `C6-P7-P12` | Lower owners remain Signal-agnostic, Store remains branch/MVCC-agnostic, future adapters receive observation rather than cache control, and no Phase 8 policy or deletion is falsely claimed as Phase 7. | Dependency/source/feature scans, API review, scope review, ledger phase separation. | `PROVED` - dependency/source/feature gates are clean, adapter APIs expose observation rather than operation, and Phase 8 inventory remains present and explicitly outside Phase 7 closure. |
| `C6-P7-P13` | The bounded-residency courtroom remains executable and mutation-sensitive after the semantic rename: CLI, configuration schema, emitted markers, report schema, digest, timings, oracle, and hostile predicate agree exactly. | Feature-bearing compile, 13 courtroom tests, mutation parser tests, argument test, source-bound mutation catalog. | `PROVED` - current report `phase16-mutants-final.json` contains exact IDs 15-44 with current source/binary hashes; final Courtroom C accepts all 30 with zero findings and source closure `acd276137a477609a0d15935435da24f01116ee7c4de1acd641ceee552245c3b`. |
| `C6-P7-P14` | The final Phase 7 source is formatted, warning-clean, boundary-clean, documentation-current, and accepted by focused and broad ordinary/certification suites plus mandatory constitutional gates. | Formatting, strict builds/tests, boundary-check, agent-context, final diff and source freeze. | `PROVED` - rustfmt/diff checks, both `-D warnings` Store graphs, focused/broad suites, boundary-check, agent-context, dirty line cap, cleanup searches, and `E13` pass. Repository caveat: the global cap still has 114 clean pre-existing non-allowlisted files and zero dirty violations. |
| `C6-P7-P15` | Adapter consumption remains lease-scoped and bounded: a chunk or its bytes cannot outlive or advance the session, `read_next` copies only into caller storage, and Store exposes no owning whole-record convenience or compatibility conversion. | Borrow-check compile failures, positive bounded read specimen, public API/source absence, copy counters. | `PROVED` - the positive bounded-access specimen and six lifetime/construction/authority negatives pass in the 31-case UI; source/API gates and real chunk journeys expose no owning or compatibility bypass. |

## Requirement Coverage

| Governing obligation | Ledger rows |
| --- | --- |
| All ordinary routes on canonical bounded residency | `P01`, `P08` |
| Integrity, isolation, and blob adapter specimens | `P02`, `P03`, `P10` |
| Exact five successor scopes without successor policy | `P04`, `P12` |
| Ordinary/Part II feature graph exclusion | `P03`, `P05`, `P12` |
| Responsibility-named public exports | `P02`, `P06`, `P13` |
| Lower owner Signal purity and Store semantic neutrality | `P08`, `P12` |
| Positive UI through intended adapters | `P02`, `P09`, `P10` |
| Hostile pool/eviction/dirty/generation/semantic cases | `P03`, `P10` |
| Exact ordinary fault/hit/copy/writeback behavior | `P01`, `P08`, `P10` |
| Delete temporary handoff, symbols, re-exports, and shims | `P06`, `P07`, `P13` |
| Product graph fails if a predecessor returns | `P05`, `P06`, `P07` |
| API semantic sharpness and authority placement | `P02`-`P04`, `P08`, `P09` |
| Compile-time enforcement over developer conscientiousness | `P02`-`P06` |
| Documentation when relevant | `P09`, `P14` |
| Cleanup without pulling Phase 8 forward | `P07`, `P12` |
| Borrowed views, bounded copies, and no owning whole-record shortcut | `P02`, `P09`, `P15` |
| Structural and test quality | `P10`, `P11`, `P14` |

## Risk Map

| Risk | Earliest honest boundary | Required detector |
| --- | --- | --- |
| Adapter imports or reconstructs lower authority | ordinary compiler graph | positive/negative trybuild pairs |
| Certification method leaks into ordinary build | facade feature gate | no-feature compile failure and Cargo feature graph |
| Chunk basis is treated as semantic proof | adapter type surface | semantic-inference compile failure and docs |
| Scope proof exposes a usable grant | certification wrapper | visibility/API inspection and compile boundary |
| One successor scope steals another ceiling | pool operation accounting | simultaneous exact/one-past runtime journey |
| Aggregate operation bytes ignore successor scopes | global envelope | all-five-live plus one-byte denial |
| Feature-tree text passes while manifest metadata leaks | dependency graph | Cargo tree plus metadata/manifests |
| Phase rename leaves protocol or mutant drift | CLI/protocol/oracle boundary | exact parser/schema/digest/predicate tests |
| Anti-regression literals are mistaken for live legacy | source inventory | explicit policy-source exclusion and product-only search |
| Deleted status has no real replacement | removal ledger | exact `workspace:` path validation |
| New UI target silently multiplies compile cost | test catalog | explicit target inventory and no-feature justification |
| Documentation promises certification controls as ordinary | feature guide | source/export comparison and compile-bound examples |
| Physical residency becomes integrity/stable-read/blob truth | successor boundary | negative compile cases and explicit non-goals |
| Owning whole-record convenience bypasses the lease/budget boundary | public records facade | borrow-check compile failures and source absence |
| Phase 8 inventory is deleted or normalized early | phase ownership | ledger phase/status grouping |
| Rename creates a catch-all or oversized file | physical topology | composition review and hard line cap |

## QA-Tests Evidence Audit

| Axis | Required Phase 7 posture |
| --- | --- |
| World honesty | The five-scope journey constructs a real admitted Store and obtains scope authority only through its bound certification probe. |
| Boundary honesty | Positive adapter cases compile against the ordinary public crate; certification absence is tested without certification features. |
| Oracle independence | Scope denials expose exact requested/current/limit tuples and are reconciled against Store observation after move-owned release. |
| Fault pressure | Every named scope reaches exact and one-past pressure; Recovery saturation coexists with Scrub; the aggregate ceiling is attacked. |
| Compiler intent | Each negative specimen has a positive neighbor and must fail at the named visibility/type boundary, not due to unrelated syntax. |
| Harness integrity | Graph and removal mutants invoke the same production scanners used for current source; source-bound mutation seams must still resolve. |
| Proof economy | Adapter authority uses one ordinary trybuild target; its separate compiler graph is justified only by the need to prove certification absence. |
| Cost honesty | The integration-target catalog names the adapter target explicitly and asserts it has no required features. |

## Finding History

### `C6-P7-F001` - truncated rename patch left source state unknown

- A four-file patch returned truncated output, so no applied hunk was assumed.
- Exact per-file inspection proved all four intended hunks had landed.
- The recovery then found remaining timing consumers and completed the semantic
  family with small verified edits.
- Affected rows: `P06`, `P13`, `P14`.

### `C6-P7-F002` - hostile predicate retained phase vocabulary

- Three test/runner sites still expected `c6-local-scheduler` while the mutation
  catalog and courtroom oracle used `local-physical-work-scheduler`.
- All producers and consumers now use one responsibility-named predicate.
- Affected rows: `P06`, `P13`.

### `C6-P7-F003` - a zero-test filter falsely appeared green

- The first focused runner filter omitted `physical-work-evidence` and executed
  zero tests.
- Exact harness enumeration exposed the feature gate; subsequent focused runs
  executed 13 courtroom, three mutation-parser, and one argument test.
- A later guessed `frame_loading` selector again executed zero tests. It was
  discarded and replaced by exact nonzero hot-path and truncated-artifact
  journey selectors derived from the real module tree.
- Affected rows: `P10`, `P13`, `P14`.

### `C6-P7-F004` - adapter UI target lacked compile-cost registration

- The complete runner rejected the new integration target.
- The target remains separate because its ordinary no-feature graph proves
  certification absence, which the maximal-authority UI target cannot prove.
- The catalog now inventories the target and mechanically asserts no required
  features.
- Affected rows: `P03`, `P10`, `P14`.

### `C6-P7-F005` - removal rows were status-only cleanup claims

- Twenty-four removed consumers remained `inventory-open`; fourteen already
  deleted rows named only prose replacement owners.
- All 38 Phase 7 rows now name exact present `workspace:` paths and carry
  `deleted-phase-7`.
- The gate now rejects a completed Phase 7 row without a present exact
  replacement.
- Affected rows: `P06`, `P07`, `P14`.

### `C6-P7-F006` - the feature guide described completed work as future

- The guide still said dirty/writeback, speculative lowering, successor scopes,
  and temporary handoff cleanup were unfinished.
- It now distinguishes stable ordinary APIs, stable vocabulary, certification-
  only direct operations, and successor-owned semantics; its four Rust blocks
  remain compiler-bound.
- Affected rows: `P08`, `P09`, `P12`.

### `C6-P7-F007` - the first ledger omitted bounded lifetime and copy behavior

- Every original row could pass while an owning whole-record convenience or
  escapable chunk defeated the adapter memory boundary.
- `P15` now requires the session borrow, caller-owned bounded copy, absence of
  Store-owned whole-record results, and copy-accounting evidence.
- Affected rows: `L02`, `P02`, `P09`, `P15`.

### `C6-P7-F008` - the active feature-graph denominator covered only Store

- The all-manifest scanner rejected direct forbidden edges, but the only
  executed Cargo tree was `worth-store --no-default-features`; another ordinary
  root could therefore acquire a forbidden default or transitive activation
  without entering the evidence.
- The gate now derives the exact two certification exclusions from Cargo
  metadata and executes one additive default-feature workspace tree over every
  remaining root, including operations, reclaim, tiering, WAL, and the runner.
- Every derived ordinary root is hostile-mutated against every forbidden
  feature, and denominator assertions prevent the named roots or certification
  exclusions from silently changing class.
- Affected rows: `L02`, `P03`, `P05`, `P10`, `P12`, `P14`.

### `C6-P7-F009` - the mutation campaign could report diagnostics without proving consequential mutants

- The first campaign interface did not make every diagnostic or incomplete
  campaign state durable enough to distinguish a killed mutant from a runner
  failure, and the original catalog omitted direct attacks on settlement,
  backend-receipt, and derived-completion authority.
- Mutation evidence now has one owned report/artifact publication boundary,
  durable per-mutant localization, current source and binary hashes, and the
  complete IDs 15-44 campaign. Mutants 15, 17, and 18 directly attack the three
  missing causal authority boundaries.
- The final bounded-residency courtroom independently reloads the published
  report and rejects stale source, altered bindings, missing observations,
  escaped artifacts, or stale binaries.
- Affected rows: `L02`, `P01`, `P08`, `P10`, `P13`, `P14`.

### `C6-P7-F010` - read-pressure evidence could pass without proving the hot path

- The first read-pressure oracle allowed aggregate counters to stand in for the
  cold, hot, and refault intervals, so a metadata read on every hit could remain
  invisible.
- Inspection exposed a production rediscovery of structural artifact length on
  each read. Store now carries exact frame-source extent as typed structural
  proof and performs metadata validation only on a real fault.
- The courtroom now requires cold work to equal cold metadata effects, hot work
  and all hot media effects to be zero, refault work to equal refault metadata
  effects, and total metadata effects to equal metadata-work plus range-work
  terminals. The final accepted report proves all equations.
- Affected rows: `L02`, `P01`, `P08`, `P10`, `P13`, `P14`.

### `C6-P7-F011` - proof-carriage exposed stale boundary gates and journey fixtures

- After structural extent became carried proof, the fault-ownership gate still
  sliced source around the removed `file_length` helper, the direct-media gate
  asserted an obsolete constructor count, and hot/coalesced journeys still
  expected metadata work.
- The gates now parse the real bounded-loader function body and require one
  bootstrap owner for every direct constructor. Journeys require zero hot work,
  partition scan evidence from a fresh runtime, and current source hashes.
- The complete 241 physical-record journeys and 213 library plus two CLI runner
  tests pass on the corrected semantics.
- Affected rows: `P01`, `P06`, `P08`, `P10`, `P13`, `P14`.

### `C6-P7-F012` - cleanup truth and QA topology stopped at the active phase

- The removal gate required present path-bound replacement owners for completed
  Phase 6 and Phase 7 rows but allowed completed Phase 5 rows to retain prose
  owners. Several QA files also crossed the 400-line law while accumulating
  unrelated pressure, cancellation, and dirty-close responsibilities.
- Completed removal rows in every implemented phase now require a present,
  non-escaping `workspace:` replacement path. Hostile tests reject prose-only,
  empty, escaping, and absent owners.
- Pin pressure, cancellation, and dirty-close evidence moved into named modules;
  all 313 dirty Rust files are at or below 400 lines, and 119 advisory
  candidates were inspected with zero scan errors.
- Affected rows: `L02`, `P07`, `P10`, `P11`, `P14`.

### `C6-P7-F013` - the ordinary adapter graph was green but not warning-clean

- The ordinary adapter UI compiled all intended positive and hostile cases but
  emitted dead-code warnings for `ExactFrameSourceExtent::CoordinateOnly`,
  `load_admitted_exact`, and its `FrameReadSourceFailure` import.
- All three are certification-only authority. Their definitions and import are
  now gated by `certification-test-authority`; the ordinary
  `CompleteArtifact` match remains exhaustive without lint suppression.
- Both no-feature and certification-feature Store graphs pass under
  `RUSTFLAGS=-D warnings`, and both UI suites pass after the correction.
- Because production source changed, the prior mutation report and Courtroom
  were discarded and fully regenerated before closure.
- Affected rows: `L01`, `P03`, `P10`, `P13`, `P14`.

## Evidence Index

- `E01` - governing spec, authority ledger, cleanup contract, public exports,
  certification gates, and strongest Phase 5-7 journeys inspected.
- `E02` - focused runner evidence: 13 bounded-residency courtroom tests, three
  mutation-parser tests, and one exact CLI argument test.
- `E03` - feature-bearing complete runner: 213 library and two CLI tests passed,
  zero failed.
- `E04` - runtime authority UI: one harness passed with all 31 listed
  pass/fail specimens, including four compiler-bound guide blocks.
- `E05` - ordinary adapter UI: one harness passed with four positive and six
  hostile specimens.
- `E06` - removal inventory: ten passed, including all 38 Phase 7 exact
  predecessor/replacement rows, completed Phase 5 replacement ownership, and
  controlled prose/empty/escaping/absent/stale/rediscovered/unclassified
  failures.
- `E07` - exact product/test legacy search: zero non-policy `C6`/`c6_` tokens,
  zero phase-named paths, and zero displaced runner identifiers.
- `E08` - exact five-scope runtime journey: one exact selector passed with 240
  unrelated journeys filtered out.
- `E09` - complete physical-residency boundary family: 78 passed, including
  the metadata-derived ordinary workspace graph and its full-root hostile
  denominator.
- `E10` - warning-denied no-feature and certification-feature Store checks;
  ordinary adapter UI with four positive/six hostile specimens and runtime
  authority UI with 31 pass/fail specimens.
- `E11` - code-quality candidate evidence: all 313 dirty Rust files are at or
  below 400 lines; 119 advisory functions were inspected with zero scan errors.
  QA-tests rebound includes 213 library plus two CLI runner tests, 241 physical
  journeys, ten removal tests, exact nonzero focused selectors, 30 mutants, and
  the independent Courtroom oracle.
- `E12` - boundary-check and agent-context both pass; rustfmt and
  `git diff --check` pass. The canonical Bash line-cap script is unavailable in
  this Windows environment, so its exact tracked-file/allowlist semantics were
  reproduced in PowerShell: 18,934 tracked Rust files, 208 allowlisted
  over-cap files, 114 clean pre-existing violations, and zero dirty violations.
- `E13` - final exact-byte source freeze: base
  `95afd3a9ac80967d9a31ce75a80cad98af8c0604`, 367 entries excluding only this
  ledger, 211 modified, 38 deleted, 118 untracked, 53,676 manifest bytes,
  SHA-256
  `c477353b7e8945de26bccbc2537e4b45ff0b300f05411a444f26b3f0864675c7`;
  Git no-filter hashes and independent raw Git blob framing agree with zero
  mismatch.
- `E14` - current mutation/courtroom evidence: all 30 source-bound mutants
  (IDs 15-44) have exact expected predicates, current source and retained binary
  hashes; final Courtroom C accepted the independent oracle with zero findings,
  causal cold/hot/refault equations, and successful writer/observer/reopener
  process fates.

## Ledger Completeness Attack

Before closure, answer:

1. Can an ordinary adapter compile while silently importing pool or
   certification authority through a feature edge?
2. Can a positive adapter example compile while using a fixture-only or
   certification-only constructor?
3. Can a chunk basis, generation, pressure report, counter, or scope enum mint
   semantic-residency or mutation authority?
4. Can a certification scope wrapper expose the lower grant, pool incarnation,
   or an operation method?
5. Can one successor scope consume another scope's ceiling, or can all five
   escape the aggregate operation envelope?
6. Can release/drop leave active operation bytes while the journey still
   passes?
7. Can a real miss or writeback use the wrong Signal family or Foundational
   basis while adapter tests remain green?
8. Can a hit, coalesced consumer, or denial invent Signal/scheduler/media work?
9. Can a phase-named identifier survive in CLI, schema, report, timing, test,
   or mutation code while the path search remains green?
10. Can a deleted predecessor return under an unclassified path or can a
    replacement disappear while the ledger stays closed?
11. Can policy-gate detection literals be deleted to manufacture a clean
    absence search?
12. Can the docs imply that physical residency proves integrity, stable
    semantic reads, QoS, recovery correctness, or blob completeness?
13. Can the ordinary adapter UI move into a certification-enabled target and
    falsely prove certification absence?
14. Can Phase 7 close while a dirty file violates the 400-line law or hides a
    mixed responsibility?
15. Can Phase 8 work be silently pulled forward, or can its open inventory be
    misreported as a Phase 7 defect?
16. Can any guarantee depend only on a stale pre-correction run, zero-test
    filter, truncated output, or absence search?
17. Can Store reintroduce an owning whole-record result, compatibility
    conversion, or escapable chunk while every adapter authority row passes?

Any plausible yes adds or revises a ledger row, reopens downstream rows, and
blocks closure.

Final attack result: all seventeen answers are **no** against the frozen
source. Ordinary feature and compiler graphs close questions 1-4 and 13;
exact five-scope and real physical-work journeys close 5-8; responsibility
naming, removal truth, and source-bound mutation evidence close 9-11; the
compiler-bound guide and successor separation close 12 and 15; structural
scrutiny and the dirty line cap close 14; the exact final rebound and manifest
close 16; and the lease/copy compiler specimens plus public API gates close 17.
No closure claim depends only on an absence search, self-certified counter,
zero-test filter, stale report, pre-correction run, or truncated transcript.

## Closure Decision

`PROVED` - Phase 7 is closed against the exact source and evidence recorded in
`E01`-`E14`.

All seventeen closure rows are `PROVED`; the completeness attack has no
surviving in-scope defect; all evidence is rebound after `F013`; and no
Phase 8 policy or deletion is claimed as Phase 7 work.
