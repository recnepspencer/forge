# C.6 Phase 8 Closure Ledger

## Scope

This ledger audits C.6 Phase 8, **Delete The Parallel S.2 World**, against the
full C.6 contract, the WORTH engineering constitution, the live dependency
graph, the canonical Store runtime, the buffer-pool owner, successor physical
consumers, certification, and test support.

Phase 9 documentation publication and the Phase 10 hostile courtroom remain
closed until every Phase 8 guarantee is proved. Phase 8 itself permits no
quarantine, compatibility alias, disabled legacy feature, copied fixture, or
deferred cleanup.

## Audit Source State

The implementation batch begins from commit
`761268304100001eb23680155f620c4c037ae1fd` with a clean worktree on branch
`worth-store`.

The scheduler-authority cutover batch begins from commit
`0dfc0ac0c5271f36c4339194defb0a62cd9045a3` after the Phase 8 inventory
classifier was hardened and its controlled mutants passed.

The move-owned scheduler execution batch resumes from checkpoint
`6f8a5e6c837ee4ff604bde40add364cf3469d014`. Its first bounded owner compile
exposed seven mixed-cutover errors and proved that background authority still
becomes clonable after lease lowering.

The legacy-module closure audit resumes from clean commit
`4473a9d58313ec01f1e1d7c94612424a0f3f2a6e`. The strengthened inventory
generator exposed 46 identifier-free Rust files beneath legacy-gated module
roots that the prior leaf-identifier classifier did not require the removal
ledger to classify.

The final dirty-state manifest contains 452 entries after excluding only this
ledger: 280 modified, 128 deleted, and 44 untracked paths. Of those entries,
324 are present exact-byte inputs and 128 are explicit deletion records. Each
manifest row is `status,sha256-or-dash,repository-relative-path`, sorted by
UTF-8 ordinal path and then status and terminated with LF. Independent
PowerShell and Python implementations produce the same manifest SHA-256:
`2a17d612d5d8fbbd608757bf3036ecfc53ab964fc6ac291e4de964217d90713e`.

The final removal ledger contains 275 unique, field-complete rows: 182
`delete`, 93 `narrow`, and zero `preserve`. Zero preserve is intentional:
every inventoried legacy surface either disappeared or shed the legacy
responsibility. All 182 deleted paths are absent, all 93 narrowed paths are
present and no longer rediscovered, every replacement owner resolves, and all
33 removal-inventory tests plus the complete 117-test physical-residency
boundary family pass.

The complete workspace outside `worth-store-certification` passes with every
feature and warnings denied. Bounded certification execution accounts for all
341 library tests: 336 pass, one is intentionally ignored, and four failures
are byte-unchanged from the Phase 8 baseline and share the pre-existing missing
Store `certification` facade proof. Full all-target Clippy is likewise green
for every package except two unchanged Store test lints; Store library Clippy
is green. The four certification failures and two Store test lints are explicit
scoped baseline caveats, not Phase 8 defects and not authority to repair
unrelated work. They remain visible without preventing Phase 8 closure.

The scheduler migration slice has a bounded final-state inventory of 140
changed paths: 138 present files and two intentional deletions. A
line-terminator-independent scan of every present file reports zero trailing
space or tab, space-before-tab indentation, or conflict-marker violations.
This is slice evidence only; it does not close the final Phase 8 fingerprint
while the recovery migration remains open.

## Adversarial Constraint

> A certification crate, successor physical owner, or test-support crate tries
> to preserve the S.2 physical world behind an optional feature, renamed
> snapshot, copied fixture, direct pool edge, legacy owning view, or broad
> certification exception. It then claims production residency from that
> parallel model while the real Store uses the canonical pool. The compiler,
> Cargo metadata, generated removal inventory, source gates, real Store tests,
> and controlled reintroduction mutants must reject or expose the substitution
> at its causal boundary.

## Authority Boundary

- `worth-store-buffer-pool` owns canonical resident-frame state and narrow
  owner-local pool proofs.
- `worth-store` owns Store lifecycle generation, successor allocation
  admission, borrowed record chunks, pressure projection, and composition.
- Integrity and isolation consume Store-borrowed physical bytes and basis, not
  pool leases, frame keys, or legacy views.
- Recovery, scrub, maintenance, verification, and blob consumers receive
  exact Store-minted scope types, not generic lower grants.
- Exact successor allocation authority borrows the issuing serving runtime;
  closing or moving that runtime while authority remains live must not compile.
- The scheduler owns scheduler policy and execution admission, not copied
  physical-isolation readiness or Store composition authority.
- Certification drives the real Store composition or leaves a genuinely local
  pool law with the pool owner.
- Test support may construct real Store worlds but may not manufacture
  residency authority.

## Closure Guarantees

Every uncertain claim begins `OPEN`. Existing tests and gates are candidate
evidence until their boundary, world, oracle, and mutation sensitivity are
inspected.

| ID | Exact closure claim | Required evidence | Current result |
| --- | --- | --- | --- |
| `C6-P8-L01` | The final source state is complete and reproducible across tracked, deleted, renamed, and untracked files, excluding only this ledger from its own fingerprint. | Exact-byte manifest, independent hashes, status counts, final evidence rebound. | `CLOSED` |
| `C6-P8-L02` | The ledger covers every Phase 8 must-ship, preserve, proof, cleanup, API, dependency, lifecycle, test, and causally necessary intent guarantee. | Requirement map, risk map, finding history, evidence index, and surviving-defect attack. | `CLOSED` |
| `C6-P8-P01` | Every Phase 1 Phase-8 consumer and every newly discoverable parallel-world consumer has an exact migration or deletion disposition; no broad certification exemption hides a consumer. | Generated source/manifest inventory, Cargo metadata, exact removal-ledger reconciliation, hostile unclassified/stale-row mutants. | `CLOSED` |
| `C6-P8-P02` | `legacy-s2-models` and `legacy-certification-models` do not exist as feature declarations, optional branches, dependency features, or activated metadata edges anywhere in the live Store workspace. | Manifest-key inspection, Cargo metadata and tree inspection, repository source search, declaration/edge mutants. | `CLOSED` |
| `C6-P8-P03` | Snapshot-derived residency admission, `S2PhysicalResidencyEntry`, `S2PhysicalEntryFacts`, and every equivalent count-snapshot authority graph are deleted without a renamed replacement. | Source/module absence, public API inspection, compile failure on controlled reintroduction, canonical Store admission tests. | `CLOSED` |
| `C6-P8-P04` | `ResidentFrameTable` and its request, lease, dirty, eviction, report, and capacity graph are deleted; the canonical `PhysicalResidencyPool` is the only resident-frame truth. | Complete module and symbol absence, canonical pool owner tests, Store-bound journeys, direct-owner graph. | `CLOSED` |
| `C6-P8-P05` | Legacy zero-copy, bounded-copy, materialization-profile, pinned-view, and owning-read-buffer graphs are deleted; integrity and isolation consume the Store borrowed chunk contract. | Public API/source absence, positive Store-view consumers, lifetime and construction compile failures, bounded-copy runtime evidence. | `CLOSED` |
| `C6-P8-P06` | Isolated S.2 background, speculative, queue, allocation-envelope, and evidence-source models are deleted; effectful speculation remains only on the canonical Store runtime. | Module/source absence, canonical speculation tests and counters, no local worker/queue gate, controlled legacy-model mutant. | `CLOSED` |
| `C6-P8-P07` | Direct buffer-pool dependency and source access is limited to exact canonical physical owners; certification, successor domains, and test support cannot import it. | Cargo metadata allowlist, source-import inventory, dependency/source mutants, boundary checker. | `CLOSED` |
| `C6-P8-P08` | Recovery, Scrub, Maintenance, Verification, and Blob allocation authority is Store-minted, runtime-borrow-bound, generation-bound, move-owned, exact-scope typed, and incapable of exposing or spending the lower grant. Verification authority covers every protected byte before an integrity witness can mint. The issuing serving runtime cannot close or move while any successor authority remains live. | Positive compile specimens; cross-scope, forgery, grant-extraction, clone, move-after-use, runtime-escape, close-while-live, and undersized-verification negatives; exact protected-width success; real Store admission/pressure/release/close journey. | `CLOSED` |
| `C6-P8-P09` | Certification and test fixtures prove the real Store composition or narrow canonical pool laws; no fixture constructs deleted physical truth, self-certifies from copied counters, or uses one compromised envelope for semantically distinct publication and successor-pressure journeys. | Complete fixture/evidence trace, real Store roots and observations, exact publication-demand counters, profile-specific pressure evidence, independent oracles, deletion of redundant model evidence. | `CLOSED` |
| `C6-P8-P10` | Mathematical or policy tests retained from S.2 have independent value, make no production-authority claim, and add unique evidence not already owned by canonical pool or Store tests. | QA-tests proof-obligation audit, mutation sensitivity, duplicate-test review, owner placement review. | `CLOSED` |
| `C6-P8-P11` | Store-to-successor and Store-to-certification dependency direction is one-way: Store owns runtime truth; successors and certification consume its facade; no normal dependency cycle, peer-owned composition adapter, or public compatibility re-export remains. | Cargo metadata/tree, strongly connected component inspection, facade review, compile tests, dependency-cycle mutant. | `CLOSED` |
| `C6-P8-P12` | Every dependency, feature branch, module export, registry row, test selector, and fixture capability made dead by the cutover is removed. | Warnings-denied builds, metadata diff, dead-reference searches, suite/catalog execution. | `CLOSED` |
| `C6-P8-P13` | The workspace builds and its affected owner, Store, successor, certification, and test-support suites pass with no deleted feature available. | Focused tests, all-target/all-feature checks, workspace test lane, boundary-check, agent-context. | `CLOSED` — all Phase 8 evidence passes; four certification tests and two Store test Clippy lints remain explicit unchanged baseline caveats |
| `C6-P8-P14` | Controlled reintroduction of a legacy feature, direct pool edge, deleted authority identifier, snapshot fixture, or legacy view fails the nearest mechanical gate. | Individually localized mutants for every substitution class. | `CLOSED` |
| `C6-P8-P15` | The resulting directory structure, facade placement, names, file sizes, and function composition preserve current and committed successor responsibilities. | Full dirty inventory, Rust function scrutiny, 400-line gate, composition and domain-topology review. | `CLOSED` |
| `C6-P8-P16` | Scheduler-native policy cannot become physical-isolation or Store authority: the scheduler has no physical-isolation/recovery dependency, copied readiness admission, generic `AuthorityMarker`, or counter-derived execution capability. Cross-domain physical composition occurs only at Store. | Dependency/source absence, public API inspection, policy-versus-authority type review, compile mutants, Store scheduler journeys. | `CLOSED` |
| `C6-P8-P17` | Scheduler execution capacity is concrete, move-owned, and single-consumption through lease, queue declaration, policy admission, ready plan, and consuming-domain handoff: none can be cloned to admit or execute the same capacity twice; non-admitted outcomes and observations mint no authority; a consuming domain cannot self-admit. | Public API inspection, clone/copy/move-after-use and duplicate-lowering/admission compile failures at every authority-bearing stage, positive queue and compaction progressions, consumer-construction mutant. | `CLOSED` |
| `C6-P8-P18` | Tiering and blob placement consume only class-relevant physical authority: inline and external placement require no cold-tier or scheduler readiness, cold placement consumes the exact cold posture, and layout projection cannot promote scheduler counters into placement truth. | Public API/source inspection, class-specific positive specimens, cold-scope negative, copied-readiness absence and reintroduction mutant. | `CLOSED` |
| `C6-P8-P19` | Every background-consuming blob path carries an exact scheduler class lease into the effectful operation and retains it through completion. Yielded, deferred, denied, violated, or zero-capacity throttled outcomes fail before source polling, verification, mutation, or publication. | Ingest, verification-read, and compaction authority traces; exact-class negatives; fail-before-effect runtime oracles; non-clone and move-after-use compile failures. | `CLOSED` |
| `C6-P8-P20` | The removal inventory covers the complete filesystem closure of every legacy-gated Rust module and every file in a predecessor owner tree, including ungated roots and descendants with no legacy identifier of their own. An identifier-free file cannot survive merely because only its ancestor carries the feature gate or because it sits beside rather than beneath a gated module. | Module-resolution and predecessor-tree inventory, exact ledger reconciliation, directory-style, file-style, ungated-root, and stale-consumer hostile mutants, plus stale and rediscovered-row mutants. | `CLOSED` |

## Requirement Coverage

| Governing obligation | Ledger rows |
| --- | --- |
| Migrate or delete every Phase 1 legacy consumer | `P01`, `P03`-`P07`, `P12` |
| Remove all legacy feature declarations and edges | `P02`, `P11`, `P12`, `P14` |
| Delete snapshot, frame-table, view, background, speculative, and materialization authority | `P03`-`P06` |
| Replace useful tests at an honest owner boundary | `P09`, `P10`, `P13` |
| Remove dead dependencies and branches | `P11`, `P12` |
| Source, manifest, and metadata absence gates | `P01`-`P07`, `P14` |
| Workspace builds and tests without deleted features | `P13` |
| Controlled reintroduction fails CI evidence | `P14` |
| No quarantine, alias, disabled feature, copied fixture, or deferred cleanup | `P01`-`P07`, `P09`, `P12` |
| Compile-time enforcement and successor handoff honesty | `P05`, `P07`, `P08`, `P11` |
| Verification allocation covers the complete protected byte view | `P08`, `P09`, `P13` |
| No orphaned synthetic closeout authority survives public exports or registries | `P01`, `P09`, `P10`, `P12`, `P14` |
| No cross-domain dependency cycle or copied scheduler authority | `P11`, `P16` |
| Move-owned scheduler execution capacity with no consumer self-admission | `P16`, `P17` |
| Placement consumes only class-relevant physical authority | `P06`, `P11`, `P16`, `P18` |
| Fail-closed exact scheduler capacity through every blob consumer | `P17`, `P19` |
| Complete legacy-gated module closure, including identifier-free descendants | `P01`, `P12`, `P14`, `P20` |
| Ledger completeness and final source truth | `L01`, `L02` |
| Composition, topology, and test quality | `P09`, `P10`, `P15` |

## Risk Map

| Risk | Earliest honest boundary | Required detector |
| --- | --- | --- |
| Disabled legacy feature survives metadata | Cargo feature declaration | metadata feature-key gate and mutant |
| Certification exemption hides a second physical world | dependency/source classifier | exact canonical-owner allowlist |
| Legacy graph is renamed instead of deleted | semantic source inventory | module-family and authority-shape gate |
| Generic allocation permits cross-scope use | public type boundary | exact concrete scope types and compile failure |
| Allocation outlives or survives closure of its issuing runtime | lifecycle/type boundary | runtime lifetime on exact capability and close-while-live compile failure |
| Store scope evidence leaks a lower grant | visibility boundary | grant-extraction negative |
| Verification reserves fewer bytes than the integrity witness protects | integrity entry boundary | exact protected-width comparison, typed denial, release oracle, and underallocation attack |
| View migration copies or owns bytes | adapter API and counters | borrow-check UI plus copy counters |
| Fixture opens no real Store boundary | test world construction | fixture trace and real-root observation |
| A fixture admits successor scopes but cannot publish the record needed to reach them | fixture residency policy | real append/read journey with exact foreground peak and release counters |
| Legacy test is copied into canonical topology | proof-obligation ownership | QA-tests uniqueness and mutation review |
| A compile-fail specimen passes for stale imports, arity, or setup instead of the claimed authority boundary | negative-test oracle | current production types with every non-subject argument well typed |
| A new runtime denial is enforced but omitted from exhaustive certification closeout | denial-to-evidence handoff | compiler-exhaustive denial mapping, closeout `ALL` membership, and real executed-denial evidence |
| Cleanup leaves a composition or line-cap fixture pointing at a deleted owner | certification fixture path | owner-existence execution plus current semantic-path review |
| A consumer invokes legacy constructors without naming the legacy type roots in the inventory | public legacy API family | constructor/scope-fragment classification and real consumer discovery |
| Isolation accepts a pool lease or fabricated lower physical-format view instead of a Store chunk | isolation byte-guard boundary | one Store-chunk constructor, exact chunk-basis denial, and real-Store certification |
| Isolation pairs a caller-selected protected reference with a different Store chunk | Store-to-isolation provenance boundary | Store-minted physical owner in chunk basis, chunk-derived reference construction, and compile failure for a second reference argument |
| A new physical owner domain encodes but cannot validate, reconstruct, or retain distinct range meaning | owner serialization and range boundary | binary manifest round-trip, lease-persistence round-trip, family compatibility, and range-family separation |
| A fixture proves its envelope by comparing constants derived from the same configuration | test oracle boundary | executed Store counters and independently expected exact peaks |
| Certification rebuilds one immutable readiness/replay world per test until the suite becomes unusable | certification fixture reuse boundary | immutable canonical cache plus independent per-test clones and full-suite timing |
| A timed-out Windows test binary survives and locks the next linker output | test process teardown | exact orphan-process inspection and cleanup before evidence rerun |
| Store depends on its certification consumer | Cargo graph | one-way dependency gate |
| Scheduler copies physical-isolation counters into self-minted authority | cross-domain composition boundary | dependency absence, concrete public types, and authority-source mutant |
| A renamed background-pacing wrapper restores generic proof authority while exact deleted identifiers stay absent | scheduler background-pacing source boundary | path-scoped proof-vocabulary gate and renamed-wrapper mutant |
| A compiler proof attacks an adjacent scheduler policy artifact rather than the historical background-pacing basis | public type boundary | exact historical type in every negative specimen and intended-cause diagnostic inspection |
| Physical isolation imports scheduler policy directly and hosts the forbidden conversion in the target-owning crate | successor dependency boundary | exact manifest denial, path-bound reverse-edge mutant, and depth-1 tree inspection |
| Scheduler capacity is copied or lowered twice | scheduler-to-queue type boundary | move-only lease and move-after-use compile failure |
| A move-only lease becomes duplicable after queue lowering | queue policy and execution progression | move-only declaration/admission/ready plan and duplicate-admission compile failure |
| Blob compaction self-admits or discards scheduler admission into booleans | consumer handoff | sealed scheduler-derived pacing admission and construction mutant |
| Inline/external blob placement requires irrelevant cold readiness | placement intent type boundary | class-specific intent variants and cold-only scope validation |
| Deletion leaves registry or selector sediment | build/catalog boundary | warnings-denied compile and exact selector runs |
| Synthetic S.2 closeout reports survive after their legacy feature and consumers disappear | public API, module graph, and evidence registry | identifier-family and deleted-path inventory plus source/registry absence |
| Search excludes the very consumer it must find | inventory generator | hostile consumer in every former exception class |
| Identifier-free files survive under a legacy-gated module root | Rust module-resolution boundary | complete gated-module filesystem closure and directory/file-style mutants |
| A leaf identifier family displaces its ancestor's legacy module-closure family | inventory family aggregation | union control plus complete multi-row mismatch denial |
| Executable compile-fail documentation is omitted from inventory or mistaken for a live forbidden call | code-adjacent documentation boundary | `src`/`tests` Markdown discovery, stale-type classification, and Rust-only live-call classification |
| Obsolete aggregate evidence and a current shared performance contract share one file or denial type | semantic cleanup boundary | consumer map, responsibility extraction, dead-helper warnings, and exact direct-edge inventory |
| Cleanup claim rests on an earlier source state | evidence freeze | final exact source fingerprint |
| Physical foreground lowering substitutes a different locality while constructor tests accept mere presence | scheduler declaration boundary | exact caller-locality equality oracle plus Store structural-locality journeys |
| Secure-I/O certification reaches a violation for the wrong causal reason | scheduler execution outcome | exact `BackendContradictedWitness` oracle for a cross-key backend completion |
| Queue execution hides self-minted generic proof authority beneath concrete public typestates | scheduler execution progression | exact generic-proof source absence, dead-dependency removal, concrete consuming typestates, and renamed-authority mutant |
| A scheduler authority source gate rejects concrete platform witnesses because generic proof names are matched as substrings | source-gate classifier | Rust-identifier boundary matching plus concrete-witness positive control |
| A completed removal row names a conceptual replacement but no inspectable live owner | removal-ledger owner binding | exact path validation for every completed phase row and family-wide invalid-owner inventory |
| A fixed-posture public declaration delegates to an internal constructor that can recombine durability and writeback incoherently | physical foreground declaration | closed operation posture plus one semantic common-input packet |
| Queue admission proves a transition and then flattens its facts into a multi-argument plan constructor | scheduler admission-to-plan boundary | concrete validated admission packet consumed by plan construction |

## Finding History

### `C6-P8-F001` - Phase 1 inventory exempts certification consumers

- status: `CORRECTED`
- affected guarantees: `L02`, `P01`, `P07`, `P09`, `P14`
- evidence: the Phase 7 classifier names certification crates as unrestricted
  pool owners, while live inspection finds 37 `worth-store-certification`
  files and 7 `worth-store-physical-certification` files importing the pool,
  including parallel-model evidence.
- correction: direct-pool discovery now applies to every crate rather than six
  fixed legacy roots; package-name values and workspace policy declarations
  are distinguished from real Rust imports and package dependencies;
  certification direct edges require an exact Phase 8 row; 23 previously
  hidden certification paths and 7 physical-certification paths were added to
  the removal ledger.
- closing proof: the live inventory reconciles, the certification no-row
  mutant fails at the direct-edge classifier, and the corresponding exact-row
  control passes. Consumer migration and final direct-edge absence remain open
  under `P01`, `P07`, and `P09`.

### `C6-P8-F002` - Store's scheduler dependency closes a successor cycle

- status: `CORRECTED`
- affected guarantees: `L02`, `P08`, `P11`, `P12`, `P13`, `P16`
- evidence: the normal graph contains
  `worth-store -> worth-store-io-scheduler ->
  worth-store-physical-isolation -> worth-store-recovery-physics`; therefore
  Recovery cannot consume Store-minted allocation authority without closing a
  Cargo dependency cycle.
- correction: scheduler-owned physical-isolation composition was removed;
  `worth-store-io-scheduler` retains only scheduler-native policy and execution
  dependencies, so physical-isolation and recovery-physics may consume the
  Store facade one-way.
- closing proof: the scheduler's normal direct dependency tree contains neither
  `worth-store-physical-isolation` nor `worth-store-recovery-physics`; the
  workspace source and manifest gate finds no surviving scheduler authority
  derived from physical isolation; exact adversarial mutants that reintroduce
  either forbidden dependency are rejected; and
  `worth-store-physical-isolation` plus `worth-store-recovery-physics` compile
  together against the resulting graph.

### `C6-P8-F003` - Exact successor allocation authority can escape its runtime

- status: `CORRECTED`
- affected guarantees: `L02`, `P08`, `P13`
- evidence: `PhysicalScopedAllocationAdmission<'runtime>` returns exact
  allocation values with no lifetime parameter. A caller may retain one,
  consume or close the issuing `ServingPhysicalRuntime`, and continue holding
  copied runtime/generation identity plus the lower grant.
- required correction: bind every exact allocation capability to the lifetime
  of the issuing serving runtime and propagate that lifetime through successor
  wrappers.
- closing proof: positive successor integrations compile while allocation
  escape and close-while-live specimens fail for the intended borrow cause.
- correction: `StoreScopedAllocation<'runtime>` now privately owns the lower
  `OperationAllocationGrant` together with `&'runtime RecordFramePorts`,
  runtime identity, and lifecycle generation. Five non-`Clone`, non-`Copy`
  concrete capabilities expose only observation: `Recovery`, `Scrub`,
  `Maintenance`, `Verification`, and `Blob`. Their only constructors are the
  correspondingly exact methods on
  `PhysicalScopedAllocationAdmission<'runtime>`. Recovery memory, scrub plans,
  maintenance envelopes and queue reports, integrity entry requests and
  inspection leases, and Blob ingest/read execution own and propagate the same
  `'runtime` lifetime; none exposes the lower grant.
- closing evidence: the Store adapter UI suite accepts all five positive exact
  allocations and rejects cross-scope substitution, field forgery, lower-grant
  extraction, runtime escape, and close-while-live at the intended type or
  borrow boundary. Recovery, Maintenance, and Blob owner compile-fail suites
  reject wrong-scope, successor-wrapper escape, and close-while-live. The
  physical-integrity owner suite adds the same direct proofs for both Scrub and
  Verification; all 52 physical-integrity compile-fail doctests pass with
  warnings denied. Four real Scrub courtroom tests and three integrity-entry
  tests pass with warnings denied, including retained allocation through
  pause/resume, exact Verification coverage, foreign-Store rejection, and
  undersized denial with immediate release. The feature-enabled real Store
  world test runs once and passes: all five allocations carry the issuing
  runtime identity, exhaust one global envelope together, release to zero
  active bytes when dropped, and permit a clean runtime close. The mandatory
  workspace boundary checker reports valid Road 1 Cargo topology, and the
  generated agent-context checker passes at the final corrected source state.
  Seventeen migrated `direct-pool-consumer` rows now carry exact path-bound
  replacement owners and `deleted-phase-8` status; the complete 24-test
  removal-inventory family passes live discovery equality and replacement-owner
  validation.

### `C6-P8-F004` - Scheduler projects copied isolation counters into authority

- status: `CORRECTED`
- affected guarantees: `L02`, `P06`, `P11`, `P16`
- evidence: `IoSchedulerIsolationAdmission` copies physical-isolation
  assumptions and counters; background pacing then mints a generic
  `AuthorityMarker` witness from copied freshness state. Store consumes neither
  this admission nor the generic foreground path that requires it.
- required correction: delete copied readiness/capability surfaces and generic
  marker authority; retain independently valuable scheduler resource policy
  only as explicitly non-authoritative policy data.
- correction: the scheduler-owned physical-isolation readiness graph,
  counter-derived capability, and generic background-pacing authority are
  deleted. `BackgroundPacingAdmissionBasis` remains copyable observation and
  scheduler policy data but implements no generic proof marker and converts
  into no isolation admission. The repository gate rejects the deleted
  identifier family, forbidden scheduler dependency edges, and generic
  authority or capability proof vocabulary anywhere beneath the scheduler's
  `background_pacing/` responsibility.
- closing proof: one neutral trybuild session rejects both
  `BackgroundPacingAdmissionBasis: AuthorityMarker` and
  `BackgroundPacingAdmissionBasis -> PhysicalIsolationEntryAdmission` at
  E0277. The three-test repository gate passes and its renamed-wrapper mutant
  fails at the path-scoped background-pacing predicate. The scheduler passes
  87 unit tests and 8 compile-fail doctests; physical isolation passes 26 unit
  tests, one layout integration test, and 4 compile-fail doctests; the exact
  Store `physical_work::scheduler::` family executes all 8 journeys. The
  scheduler's 151-line normal dependency tree contains zero
  `worth-store-physical-isolation` or `worth-store-recovery-physics` entries.
  Physical isolation's depth-1 normal tree contains exactly one
  `worth-store` facade entry and zero direct `worth-store-io-scheduler`
  entries.

### `C6-P8-F005` - Tier placement promotes irrelevant scheduler readiness

- status: `CORRECTED`
- affected guarantees: `L02`, `P06`, `P11`, `P16`, `P18`
- evidence: `TierPlacementIoAdmission` joins a copied
  `IoSchedulerIsolationAdmission` to `ColdTierIoPosture`; every blob placement
  class must carry that joined value even though admission reads only the cold
  posture's security scope. Tier layout projection then publishes copied
  scheduler counters as placement interference truth.
- required correction: delete tier placement scheduler readiness, represent
  blob placement classes with only their relevant authority, validate cold
  posture only for cold placement, and project tier layout from exact cold-tier
  posture alone.
- closing proof: class-specific positive placement tests, wrong-scope cold
  denial, copied-readiness source/API/dependency absence, and a controlled
  scheduler-readiness reintroduction rejected by the boundary gate.
- correction: commit `9e590f7b` deleted tiering `io_readiness`, its scheduler
  dependency, the joined placement admission, and scheduler-counter layout
  projection. The final API represents inline, external, unowned-sidecar, and
  cold placement as exhaustive `BlobPlacementIntent<'evidence>` variants.
  External recoverability, sidecar observation, and `ColdTierIoPosture` are
  borrowed only for synchronous admission; inline carries neither. Cold scope
  is checked only through the cold variant. `TierPlacementLayoutReport` is
  constructed only from `ColdTierIoPosture` and exposes
  `ReclaimPolicyCounterSnapshot` plus `ColdTierMovementPosture`, never scheduler
  counters.
- closing evidence: the complete warnings-denied blob owner executes 177 unit
  tests and 162 compile-fail doctests; its 9 focused placement tests separately
  prove inline, external, and cold admission, exact borrowed-evidence identity,
  a four-machine-word maximum intent envelope, counter separation, wrong-scope
  cold denial, and hostile external denials. The complete warnings-denied
  tiering owner test proves the reclaim-policy counter type and cold movement
  posture. Six scheduler-authority boundary tests reject exact deleted names,
  a renamed tiering `io_readiness` module, a renamed scheduler-backed blob
  placement permit, generic background authority, and forbidden dependency
  edges. All 24 removal-inventory tests pass. Production source contains zero
  deleted placement-readiness identifiers, and tiering's depth-1 normal
  dependency tree contains neither scheduler nor physical isolation.

### `C6-P8-F006` - Background capacity is duplicable and blob compaction self-admits

- status: `CORRECTED`
- affected guarantees: `L02`, `P06`, `P11`, `P16`, `P17`
- evidence: `BackgroundIdleCapacityLease` is `Copy`,
  `BackgroundPacingCapability` is `Clone`, and
  `BlobCompactionPacingAdmission::admitted_compaction` constructs admitted
  pacing directly from a declaration. Blob compaction constructors install
  that bypass by default, while the scheduler-capability conversion discards
  the capability into booleans.
- required correction: make scheduler execution admission move-owned and
  single-consumption, let only genuinely admitted scheduler outcomes yield it,
  carry it into blob compaction without boolean laundering, and make an
  unpaced compaction basis unable to enter planning.
- closing proof: positive scheduler-to-queue and scheduler-to-compaction
  journeys; copy, clone, duplicate-lowering, unpaced-planning, and consumer
  self-admission compile failures; source absence for the bypass constructors.
- correction: commit `6f8a5e6c` replaced duplicable scheduler capability with
  move-owned `BackgroundIdleCapacityLease`, and commit `4473a9d5` carried that
  exact lease into blob compaction. Only throttled work with nonzero admitted
  budget and admitted-with-debt outcomes can yield a lease. Yield, deferred,
  denied, violation, and zero-admitted throttle outcomes cannot. Blob
  compaction now exposes an unpaced `BlobCompactionIntentBasis`; consuming an
  exact compaction-rewrite lease is the only public transition to
  `BlobCompactionIntent`. The private pacing admission retains the lease rather
  than projecting it into booleans.
- closing evidence: the positive
  `secure_scope_background_leases_lower_into_queue_admission` and
  `compaction_plan_admits_blob_owned_rewrite_basis` journeys pass, while a real
  ingest-class lease reaches typed compaction denial. Fourteen scheduler
  compile-fail doctests reject raw-label construction, copy, clone, duplicate
  queue lowering, and lease extraction from yield, deferred, denied, or
  violation outcomes. The blob owner's 165 compile-fail doctests reject
  unpaced planning, duplicate compaction pacing from one lease, and consumer
  self-admission; all 177 unit tests pass with every feature enabled. The
  three-test capacity boundary rejects exact deleted identifiers, renamed
  boolean pacing, declaration-based self-admission, and `Clone` or `Copy`
  authority. All 25 removal-inventory tests pass with four exact deleted
  capacity-era paths assigned to `scheduler-capacity-publication`; production
  Rust contains zero deleted capacity or bypass identifiers.

### `C6-P8-F007` - Queue lowering restores duplicability after a move-owned lease

- status: `CORRECTED`
- affected guarantees: `L02`, `P13`, `P16`, `P17`
- evidence: `BackgroundIdleCapacityLease` is now move-owned, but
  `QueueWorkDeclaration`, `QueuePolicyAdmissionReceipt`,
  `QueueExecutionAdmissionRequest`, and `AdmittedQueueExecutionPlan` remain
  `Clone`. `QueuePolicyAdmissionReceipt` also stores a cloned work declaration
  while queue admission accepts a second declaration, so one lowered
  background capacity can be copied into multiple executable progressions.
- required correction: make every authority-bearing queue stage move-owned;
  consume the work declaration into policy admission; consume that joined
  stage into execution admission; expose borrowed observation rather than
  cloned authority.
- closing proof: one positive lease-to-executed-plan progression plus
  declaration clone, policy clone, duplicate admission, and ready-plan reuse
  compile failures.
- correction: commit `e060df70` made every authority-bearing queue stage
  move-owned. `QueueWorkDeclaration` consumes the scheduler lease;
  `QueuePolicyAdmissionReceipt` owns that exact declaration;
  `QueueExecutionAdmissionRequest` owns the receipt while borrowing only backend
  capability; `QueueExecutionReadyPlan` owns the admitted plan; and execution
  consumes the ready plan. Public observation methods borrow these stages and
  return copyable policy or identity observations, never cloned authority.
- closing evidence: `one_background_lease_progresses_into_one_executed_queue_plan`
  consumes one scheduler-issued repair lease through policy admission, execution
  admission, backend completion, and `Executed` progression. Six queue-owned
  compile-fail doctests reject declaration, policy receipt, request, and
  admitted-plan cloning, a request admitted twice, and a ready plan executed
  twice. Four repository-gate tests verify the exact ownership topology and
  reject cloneable stages, borrowed or parallel work, borrowed policy, and
  borrowed ready-plan execution with line-ending-independent mutants. Controlled
  mutant 16 removes the live durability-grouping check, binds exactly once, and
  dies at its designated `scheduler-admission` predicate. The warnings-denied
  all-feature scheduler passes 88 unit tests and 20 doctests; the all-feature
  runner passes 242 unit tests plus its integration and scheduler UI lanes.

### `C6-P8-F008` - Blob streaming continued after non-admitted scheduler outcomes

- status: `CORRECTED`
- affected guarantees: `L02`, `P13`, `P16`, `P17`, `P19`
- evidence: blob ingest and verification-read classification previously
  projected scheduler outcomes into counters and allowed yielded or
  zero-capacity throttled work to continue. Admitted leases were not retained
  through the effectful session, so copied observations could outlive the
  authority they purported to describe.
- correction: ingest and verification read now require exact class leases,
  retain them in their execution sessions, and convert every non-admitted
  outcome into a typed denial before source polling or verification.
- closing proof: focused fail-before-source tests pass. Six owner-level
  compile-fail specimens reject clone and second consumption of ingest
  pressure admission, verification-read admission, and paced compaction
  intent. The complete blob owner passes 178 runtime tests and 171
  documentation tests with every feature enabled and warnings denied.

### `C6-P8-F009` - Identifier-only inventory omitted legacy module descendants

- status: `CORRECTED`
- affected guarantees: `L02`, `P01`, `P12`, `P14`, `P20`
- evidence: the former inventory classified each source independently. A Rust
  file beneath a `legacy-s2-models` module root escaped when it contained no
  legacy identifier itself. The first strengthened run rejected 46 such
  unclassified buffer-pool files.
- correction: `legacy_module_closure` resolves both directory-style and
  file-style cfg-gated module roots, inventories every Rust descendant, and
  merges that closure into discovery without weakening stronger leaf
  classifications. F020 later exposed and corrected an implementation drift
  in that merge: leaf and ancestor families are now set-unioned rather than
  first-writer-wins. All 46 descendants have exact Phase 8 rows and path-bound
  replacement owners.
- closing proof: all 19 removal-inventory tests pass with warnings denied,
  including identifier-free directory/file modules, multiline cfg, path
  override, workspace escape, cfg_attr precision, indirect manifest alias, and
  stale or rediscovered-row mutants. Final deletion and
  rediscovered-deleted rejection remain open under `P20`.

### `C6-P8-F010` - Multiline cfg attributes escaped module-closure discovery

- status: `CORRECTED`
- affected guarantees: `L02`, `P01`, `P14`, `P20`
- evidence: the first module-closure parser inspected one source line at a
  time. A valid multiline `#[cfg(... feature = "legacy-s2-models" ...)]`
  attribute separated the feature name from the opening attribute line, so an
  identifier-free gated module could survive while every initial mutant
  passed.
- correction: module discovery now accumulates complete Rust attributes before
  classifying legacy features or path overrides.
- closing proof: the dedicated multiline hostile mutant discovers both the
  gated root and its identifier-free descendant in the warnings-denied
  19-test inventory suite. Final physical deletion remains open under `P20`.

### `C6-P8-F011` - cfg_attr overclassified canonical module descendants

- status: `CORRECTED`
- affected guarantees: `L02`, `P01`, `P10`, `P20`
- evidence: the corrected multiline parser initially treated any complete Rust
  attribute containing a legacy feature name as a module gate. A
  `cfg_attr(feature = "legacy-s2-models", ...)` conditionally changes metadata
  but does not remove the module, so its canonical descendants would have been
  falsely assigned deletion dispositions.
- correction: module-closure feature extraction now accepts only actual
  `#[cfg(...)]` predicates; path attributes remain independently accumulated.
- closing proof: a dedicated cfg_attr control retains an ordinary module
  outside the legacy closure while the leaf feature reference remains visible
  to normal source-family discovery.

### `C6-P8-F012` - Orphaned S.2 closeout authority survived the cutover

- status: `CORRECTED`
- affected guarantees: `L02`, `P01`, `P09`, `P10`, `P12`, `P14`
- evidence: the legacy feature and its effectful consumers were removed while
  `S2AcceptanceSuiteKind`, `BoundedMemoryResidencySuite`, synthetic closeout
  reports and transcripts, their public re-exports, three module roots, and
  canonical-basis registry rows remained live. Because the inventory knew only
  earlier S.2 identifiers, the entire exported closeout cluster was invisible
  to cleanup reconciliation.
- correction: the three synthetic authority files, their module declarations,
  public re-exports, and registry rows are deleted. The inventory now classifies
  all seven source fragments as `legacy-certification-closeout`; the removal
  ledger records all eight affected paths with exact Phase 8 dispositions and
  path-bound real-Store evidence owners.
- closing proof: the warnings-clean certification test builds exercised by the
  focused authority and recovery suites accept the deleted API surface; all 19
  removal-inventory tests pass, including the closeout-family controls,
  current/open/deleted equality, and present replacement-owner enforcement.

### `C6-P8-F013` - Verification bytes did not cover the protected integrity view

- status: `CORRECTED`
- affected guarantees: `L02`, `P08`, `P09`, `P13`
- evidence: integrity entry checked Store identity and generation, then minted
  `IntegrityEntryWitness` while merely recording the Verification allocation's
  byte count. A one-byte Store-minted Verification allocation could therefore
  authorize inspection of a larger Store-borrowed chunk.
- correction: integrity entry now compares the concrete allocation width with
  the complete protected view before witness minting and returns
  `VerificationAllocationTooSmall { protected_bytes, allocation_bytes }`.
  Store and generation mismatch retain precedence. Every real WAL, recovery,
  pre-decode, and certification caller derives its allocation demand from the
  protected chunk rather than a fixture constant.
- closing proof: all three entry-authority tests pass: exact-width admission,
  foreign-Store denial, and undersized denial with immediate Verification-byte
  release. Six pre-decode tests, the real WAL integrity journey, and two
  recovery-backed physical-digest tests pass through the migrated callers.

### `C6-P8-F014` - The real Store fixture could not publish its own record

- status: `CORRECTED`
- affected guarantees: `L02`, `P09`, `P13`, `P15`
- evidence: the former fixture allowed only 16,384 ForegroundWrite bytes, while
  one inline record publication reserves two record frames plus 196 routing
  frames: `198 * 16,384 = 3,244,032` bytes. Integrity tests therefore failed
  before reaching their claimed authority boundary.
- correction: physical-residency configuration is split by semantic journey.
  `record_publication` admits the real Store append/read path under a 4 MiB
  operation envelope; test-only `successor_scope_pressure` preserves the exact
  five-scope exhaustion world. A directory is retained for the expected growth
  of independently meaningful fixture configurations.
- closing proof: the complete physical-residency fixture module passes both
  journeys. The publication test observes the exact 3,244,032-byte
  ForegroundWrite peak, one-frame ForegroundRead peak, borrowed payload and
  Store basis, zero active operation bytes after return, and clean close. The
  pressure test proves all five exact scope capabilities, global exhaustion,
  release, and clean close.

### `C6-P8-F015` - Renamed contents could resurrect a deleted closeout file

- status: `CORRECTED`
- affected guarantees: `L02`, `P01`, `P12`, `P14`
- evidence: `legacy-certification-closeout` initially classified only the
  deleted identifiers and module names. Recreating any of the three deleted
  source paths with renamed synthetic authority contained no recognized
  fragment, so discovery omitted the path and its `deleted-phase-8` row did not
  trigger the rediscovered-deleted denial.
- correction: the inventory now classifies each deleted closeout authority
  path independently of its contents while retaining fragment classification
  for surviving public facades, module roots, and registry sources.
- closing proof: the hostile renamed-content control classifies all three
  deleted paths as `legacy-certification-closeout`; the full inventory suite
  must also prove that a discovered deleted row is rejected.

### `C6-P8-F016` - Integrity compile failures targeted stale or incomplete setup

- status: `CORRECTED`
- affected guarantees: `L02`, `P05`, `P08`, `P13`, `P14`
- evidence: the raw-path and unlowered-view specimens called
  `IntegrityEntryRequest::new` without its required Verification allocation, so
  arity alone satisfied `compile_fail`. The lifetime and unlowered-view
  specimens also imported deleted `PinnedFrameView` authority and could pass
  before checking the current Store-borrowed chunk boundary.
- correction: every non-subject input is now a correctly typed
  `VerificationPhysicalAllocation`. Lifetime widening and explicit lowering are
  attacked through the current `PhysicalRecordChunkView`; the raw-path
  specimen now fails solely because a path is not a protected Store view.
- closing proof: the physical-integrity doctest lane must pass all corrected
  compile-fail examples against the current public API.

### `C6-P8-F017` - Verification coverage was absent from certification closeout

- status: `CORRECTED`
- affected guarantees: `L02`, `P08`, `P09`, `P13`
- evidence: the integrity closeout suite requires every member of
  `IntegrityCloseoutDenialBoundary::ALL`, but adding
  `VerificationAllocationTooSmall` did not add a corresponding boundary.
  Certification could therefore close while never executing the new authority
  denial.
- correction: `VerificationAllocationCoverage` is part of the exhaustive
  closeout lattice. A compiler-exhaustive mapping converts every
  `IntegrityEntryDenialKind` into its exact executed boundary; Store/generation
  mismatch remains distinct from allocation coverage.
- closing proof: the foreign-Store and underallocation tests derive closeout
  evidence from their real denials and assert the exact boundary. The authority
  tests and physical-integrity closeout suite must pass with the seven-member
  lattice.

### `C6-P8-F018` - Closeout composition still named a deleted scrub owner

- status: `CORRECTED`
- affected guarantees: `L02`, `P12`, `P13`, `P15`
- evidence: the physical-integrity closeout line-cap fixture attempted to read
  deleted `scrub/scrub_execution.rs`. The closeout filter failed before
  evaluating its denial lattice, so cleanup had left certification unable to
  execute.
- correction: `IntegrityCloseoutModuleKind::Scrub` now binds to the current
  semantic execution owner at `scrub/execution/run.rs`; every other path in the
  fixture was independently confirmed present.
- closing proof: the same physical-integrity closeout filter must execute both
  tests successfully from the Store workspace.

### `C6-P8-F019` - Legacy byte-guard consumers escaped type-name inventory

- status: `CORRECTED`
- affected guarantees: `L02`, `P01`, `P05`, `P12`, `P14`
- evidence: three certification scenarios invoked
  `for_owned_read_buffer`, `from_bounded_copy`,
  `for_legacy_resident_frame`, or `from_pinned_frame` through fixture helpers
  without naming the legacy record-view or frame-table types. Identifier-only
  classification omitted all three from the Phase 8 ledger.
- correction: the inventory classifies the complete public legacy byte-guard
  family, including frame tokens, pinned leases/views, legacy scopes, owned
  read buffers, and their constructors. The three discovered scenarios now
  have exact path-bound Phase 8 dispositions.
- closing proof: inventory equality passed before migration, then rejected
  exactly the three scenario rows plus their rewritten support and four
  physical-isolation owners as stale-open. After those eight rows were
  explicitly completed with path-bound owners, all 24 inventory tests passed.

### `C6-P8-F020` - Leaf families displaced module-closure membership

- status: `CORRECTED`
- affected guarantees: `L02`, `P01`, `P12`, `P14`, `P20`
- evidence: module-closure discovery used `or_insert(families)`. When a source
  already contained a legacy identifier, its leaf family won and the
  `legacy-s2-module-closure` family was silently discarded. Strengthening the
  leaf classifier exposed the contradiction between F009's claimed union and
  the generator's first-writer-wins implementation.
- correction: discovery now set-unions ancestor closure families into every
  existing leaf classification. Inventory comparison also aggregates every
  family mismatch into one denial instead of serially revealing one row per
  run.
- closing proof: hostile controls prove leaf-plus-ancestor union and prove that
  two independent family mismatches are both named by one denial. The full
  24-test removal-inventory suite passes after one complete 30-row
  reconciliation denial and the Phase 8 ledger records those exact unions.

### `C6-P8-F021` - Isolation byte guards crossed lower physical owners

- status: `CORRECTED`
- affected guarantees: `L02`, `P01`, `P02`, `P05`, `P07`, `P09`, `P11`,
  `P12`, `P14`
- evidence: physical-isolation accepted canonical and legacy pool leases,
  pinned views, owning bounded copies, and independently fabricated
  `PhysicalPayloadViewAdmission` values. Its manifest depended directly on
  buffer-pool and its certification feature reactivated `legacy-s2-models`.
  Certification constructed a snapshot-admitted `ResidentFrameTable`, so even
  non-legacy guard tests could prove a byte boundary the production Store did
  not mint.
- correction: `PhysicalByteGuardScope` is now one semantic value carrying a
  Store-derived protected reference and exact `PhysicalRecordChunkBasis`.
  `PhysicalByteGuard::from_record_chunk` consumes the lifetime-bound
  `PhysicalRecordChunkView` and rejects any different Store chunk basis.
  Physical-isolation depends one-way on the Store facade; the direct pool edge,
  legacy feature, lease/key variants, owning-copy variants, lower-format view
  constructors, and kind bag are deleted. Shared certification support now
  publishes and reads a real Store record chunk.
- closing proof: physical-isolation compiles; the affected certification
  binaries compile; eight stable-read execution tests and three security-scope
  tests pass against real Store worlds; the foreign Store chunk test reaches
  `StoreChunkBasisMismatch`; and the 24-test removal inventory passes after
  proving the eight migrated rows stale-open.

### `C6-P8-F022` - Immutable physical-isolation evidence was rebuilt per test

- status: `CORRECTED`
- affected guarantees: `L02`, `P09`, `P10`, `P13`, `P15`
- evidence: the full 109-test physical-isolation target exceeded 180 seconds.
  One evidence-materialization test spent 24.7 seconds rebuilding the same CI
  readiness replay, and the five-test closeout module exceeded 150 seconds
  while repeatedly rebuilding the same readiness receipt and six lane replay
  bundles.
- correction: shared test support caches one immutable
  `SimulationPlanningContext` and one immutable
  `PhysicalIsolationHarnessReadinessReceipt`. The closeout fixture caches its
  six immutable lane evidence rows; every test receives clones before removing,
  duplicating, or mismatching rows, so mutable fate is not shared.
- closing proof: the evidence-materialization module completes in 32.8
  seconds, closeout completes in 74.4 seconds, modules 6-10 complete in 115.0
  seconds, modules 11-20 complete in 46.7 seconds, and the complete 109-test
  target passes in 140.9 seconds.

### `C6-P8-F023` - Security certification replayed identical physical schedules

- status: `CORRECTED`
- affected guarantees: `L02`, `P09`, `P10`, `P13`
- evidence: operational-security exceeded 120 seconds and its closeout alone
  exceeded 90 seconds. Each of 12 security replay transcripts lowered and
  executed baseline and mutant physical replays even though the suite has only
  four physical schedule bindings. Timed-out Windows test binaries survived
  and held the next linker output open, producing `LNK1104`.
- correction: security replay support caches one immutable
  `SimulationReplayBundle` for each of the four concrete schedule bindings and
  clones it into freshly constructed scenario-specific security evidence.
  The two exact orphan test processes were identified by command line and
  terminated before recompilation; no source change was made for the linker
  lock.
- closing proof: security closeout passes in 42.3 seconds, the simulation
  harness passes in 45.4 seconds, the other eight tests pass in 1.3 seconds,
  and the complete 17-test operational-security target passes in 49.7 seconds.

### `C6-P8-F024` - Removal inventory owner exceeded the Rust file cap

- status: `CORRECTED`
- affected guarantees: `L02`, `P14`, `P15`, `P20`
- evidence: dirty-file structural QA measured
  `removal_inventory.rs` at 460 lines. Its 89-line classifier mutant test mixed
  unclassified-path denial, Cargo-edge classification, deleted-closeout
  resurrection, and legacy byte-guard API classification.
- correction: the classifier mutation family now lives at the semantic test
  insertion point `removal_inventory/classifier_tests.rs` and is split into
  four independently named proof obligations. The inventory owner is 371
  lines; every one of the 28 dirty Rust files is at or below 400 lines.
- closing proof: function scrutiny reports only the cohesive 70-line
  identifier-to-family table and other responsibility-cohesive dispatch tables
  as advisories, with no scan errors; all 51 dirty Rust files remain at or
  below 400 lines, with a maximum of 371; all 24 removal inventory tests,
  boundary-check, and agent-context enforcement pass.

### `C6-P8-F025` - Store chunk scope could pair unrelated physical owners

- status: `CORRECTED`
- affected guarantees: `L02`, `P05`, `P09`, `P11`, `P13`, `P14`, `P15`
- evidence: the first F021 correction made
  `PhysicalByteGuardScope::for_record_chunk(reference, chunk)` accept an
  independently selected `CurrentGenerationPhysicalReference`.
  `PhysicalRecordChunkBasis` carried Store, lifecycle, record, and frame
  coordinates but no durable physical owner. Scope construction therefore
  allowed reference A to be paired with Store chunk B; later chunk-basis
  equality only proved that the guard consumed B, not that A owned B.
- correction: `PhysicalRecordChunkBasis` now carries the exact Store-minted
  `PhysicalGenerationOwner`. Inline sessions carry their slot owner; extent
  sessions carry a distinct top-level `RecordExtentAllocation` owner with no
  fabricated segment. Store construction is compiler-separated:
  `RecordReadIdentity::for_inline` requires `SlotGenerationCell`, while
  `RecordReadIdentity::for_extent` requires `RecordExtentGenerationCell`;
  there is no generic owner constructor.
  `CurrentGenerationPhysicalReference::for_record_chunk` derives from that owner, and
  `PhysicalByteGuardScope::for_record_chunk` accepts only the chunk. Private
  fields leave no mismatched pairing route.
- closing proof: the two-argument scope call fails to compile; real Store
  journeys prove inline `SlotAllocation` and top-level
  `RecordExtentAllocation`; the scope owner equals the chunk-basis owner; 131
  format unit tests, 5 manifest-access tests, 4 physical-record-access tests,
  20 format compile-fail tests, 26 isolation unit tests, one isolation layout
  test, four isolation compile-fail tests, eight stable-read tests, and three
  security-scope tests pass. Affected targets compile with warnings denied.

### `C6-P8-F026` - Record-extent owner tag encoded without complete meaning

- status: `CORRECTED`
- affected guarantees: `L02`, `P05`, `P09`, `P13`, `P15`
- evidence: semantic QA found the initial `RecordExtentAllocation` patch
  assigned wire/fingerprint tag `7`, but `BackupBundlePhysicalOwner` did not
  accept tag `7` as valid, reconstruct it, or match it exclusively to the
  record-extent artifact family. Compilation could not expose the numeric
  dispatch omission.
- correction: tag `7` now requires an extent coordinate and forbids segment,
  page, slot, root, and allocation coordinates. It reconstructs through
  `record_extent_cell`, matches `Extent` but not segment-owned `BlobChunk`, and
  receives a distinct protected-range family from segment-owned extents.
- closing proof: one exact binary manifest round-trip, one exact backup-lease
  persistence round-trip, and one exact range-family separation test pass.
  The complete physical-format and physical-isolation suites pass, including
  the real Store extent journey.

### `C6-P8-F027` - Fixture envelope assertion shared its implementation source

- status: `CORRECTED`
- affected guarantees: `L02`, `P09`, `P10`, `P15`
- evidence: `record_chunk_world_tests.rs` re-exported the configured
  publication envelope only to assert that a locally derived expected peak
  was less than that same configured envelope. The comparison could stay green
  when configuration and expectation drifted together.
- correction: the self-derived inequality and test-only re-export are deleted;
  the envelope constant is private to configuration. The fixture retains exact
  executed Store counter assertions for foreground write/read peaks, released
  operation bytes, payload bytes, and clean close.
- closing proof: the feature-enabled exact real Store publication-and-borrowed
  read test executes once and passes after the cleanup. The new record-extent
  tests live in semantic sibling modules, keeping touched parent test owners at
  354 and 356 lines rather than consuming their remaining growth budget.

### `C6-P8-F028` - Scrub and Verification wrapper lifetimes lacked direct negative proof

- status: `CORRECTED`
- affected guarantees: `P08`, `P13`
- evidence: Store UI specimens proved the raw allocation lifetime with Blob
  escape and Recovery close-while-live cases, while successor-owner specimens
  directly proved propagation through Recovery, Maintenance, and Blob
  wrappers. `ScrubPlan<'runtime, 'lease>` and
  `IntegrityEntryRequest<'runtime, 'lease>` were correctly typed, but their
  owner suite had no direct wrong-scope, runtime-escape, or close-while-live
  specimens. Every current test could therefore pass without independently
  proving that those two owning wrappers preserved the Store runtime borrow.
- correction: the physical-integrity compile-fail proof owner now contains
  exact Scrub and Verification wrong-scope, wrapper-escape, and
  close-while-live specimens. Every non-subject import, constructor, input, and
  policy is current and well typed, leaving the exact allocation type or
  runtime borrow as the intended failing cause.
- closing proof: all 52 physical-integrity compile-fail doctests pass with
  warnings denied. The Store UI suite supplies captured E0308, lifetime, and
  E0505 diagnostics for the same raw capability boundaries. The four Scrub and
  three Verification courtroom tests pass through real
  `PhysicalResidencyStoreWorld` admission and independently observe retained
  and released scope bytes; the feature-enabled five-scope Store-world test
  executes once and closes cleanly after release.

### `C6-P8-F029` - F004 compiler proof targeted adjacent foreground policy

- status: `CORRECTED`
- affected guarantees: `L02`, `P13`, `P16`
- evidence: the first neutral compiler cases attacked
  `ForegroundReservationReceipt`. That scheduler-native receipt was adjacent
  to the boundary, but F004's exact historical defect promoted
  `BackgroundPacingAdmissionBasis`; the initial cases could therefore pass
  while the named defect returned.
- correction: both negative specimens now use
  `BackgroundPacingAdmissionBasis` directly. One attempts the exact generic
  `AuthorityMarker` promotion and the other attempts conversion into
  `PhysicalIsolationEntryAdmission`.
- closing proof: the single trybuild compiler session executes both current
  specimens and captures E0277 at the intended trait or conversion boundary.
  The scheduler's positive all-feature suite continues to exercise the same
  basis as non-authoritative pacing policy.

### `C6-P8-F030` - Stale Store selector executed zero scheduler journeys

- status: `CORRECTED`
- affected guarantees: `L02`, `P13`, `P16`
- evidence: `cargo test -p worth-store --all-features
  physical_work_scheduler` compiled every Store test target but executed zero
  tests. Treating that command as green would have supplied no Store
  composition evidence.
- correction: the live Cargo test catalog was inspected and identified the
  exact current family as `physical_work::scheduler::`.
- closing proof: the corrected selector executes all 8 named Store scheduler
  journeys, including cross-owner denial, incompatible-lane denial, policy
  receipt non-authority, exact budget lowering, disjoint admission, and
  effect-free planning; all 8 pass with warnings denied.

### `C6-P8-F031` - Exact-name gates allowed renamed background pacing authority

- status: `CORRECTED`
- affected guarantees: `L02`, `P14`, `P16`
- evidence: the initial source gate rejected the deleted authority identifiers,
  while the compiler cases rejected promotion of the basis itself. A new
  wrapper beneath scheduler `background_pacing/` could still carry that policy,
  implement generic `AuthorityMarker`, and mint an `AuthorityWitness` under a
  different name while every existing F004 proof remained green.
- correction: the scheduler-authority gate now treats `background_pacing/` as
  a non-authoritative policy responsibility and rejects generic authority or
  capability marker/witness vocabulary there. Legitimate scheduler-native
  queue-execution proof progression remains outside that subtree.
- closing proof: a controlled renamed-wrapper mutant imports
  `AuthorityMarker` and `AuthorityWitness`, implements the marker under a new
  name, and attempts to mint the witness. The focused gate rejects it at the
  background-pacing policy predicate; all three gate tests and the complete
  scheduler suite pass on the corrected source.

### `C6-P8-F032` - Reverse isolation-to-scheduler edge was not enforced

- status: `CORRECTED`
- affected guarantees: `L02`, `P11`, `P14`, `P16`
- evidence: the focused gate rejected scheduler dependencies on physical
  isolation and recovery, but did not reject a direct
  `worth-store-physical-isolation -> worth-store-io-scheduler` edge. Because
  physical isolation owns the conversion target, that edge could host
  `From<BackgroundPacingAdmissionBasis> for
  PhysicalIsolationEntryAdmission`, bypass Store composition, and turn the
  second compiler negative green.
- correction: the manifest boundary now forbids a direct scheduler dependency
  from physical isolation. All scheduler, physical-isolation, and tiering
  dependency mutants are path-bound to the exact manifest where their illegal
  edge would be introduced.
- closing proof: the reverse-edge mutant fails at the manifest predicate; all
  three focused gate tests pass. An initial full-tree probe was discarded
  because it incorrectly treated the legitimate transitive
  `physical-isolation -> Store -> scheduler` path as a direct defect. The
  corrected depth-1 proof finds exactly one direct Store entry and zero direct
  scheduler entries, while the scheduler's complete normal tree still
  contains zero physical-isolation or recovery entries.

### `C6-P8-F033` - F003 migrations left removal rows open

- status: `CORRECTED`
- affected guarantees: `L02`, `P01`, `P07`, `P08`, `P12`, `P14`
- evidence: F003 migrated Recovery, Maintenance, Blob, certification, and test
  support from direct pool grants to Store-minted lifetime-bound allocations,
  but 17 corresponding `direct-pool-consumer` rows remained
  `inventory-open`. Live discovery therefore rejected them as stale even
  though their old authority signatures were absent.
- correction: each of the 17 migrated rows now has the exact retained
  `workspace:` source owner, the existing
  `source-and-metadata-absence` gate, and `deleted-phase-8` status. No
  later-finding row was advanced.
- closing proof: all 24 removal-inventory tests pass, including exact current,
  open, and deleted equality; present path-bound replacement owners; stale-open
  and rediscovered-deleted rejection; direct-pool classification; complete
  legacy module closure; and hostile classifier mutants.

### `C6-P8-F034` - Runner Clippy gate exposed current lint blockers

- status: `CORRECTED`
- affected guarantees: `P13`, `P15`
- evidence: the exact runner all-target/all-feature Clippy lane rejected one
  obsolete `Option::map_or` predicate and two redundant borrows in the dirty
  removal-inventory owner.
- correction: the identifier-boundary predicate uses the mechanically
  equivalent `Option::is_none_or`; the inventory loop passes its existing
  path references directly. No classification, path, or status semantics
  changed.
- closing proof: `store-test-runner` passes all-target, all-feature,
  warnings-denied Clippy with `--no-deps`; all 24 removal-inventory tests pass
  after the correction. The broader transitive Clippy lane separately exposes
  an unchanged buffer-pool `never_loop` warning and is not reported as green.

### `C6-P8-F035` - Inherited Phase 8 scrub denial source failed formatting

- status: `CORRECTED`
- affected guarantees: `P13`, `P15`
- evidence: the full Store workspace format check rejected clean committed
  `worth-store-physical-integrity/src/scrub/scrub_denial.rs`; its enum variants
  retained pre-rustfmt single-line field layout.
- correction: only that inherited Phase 8 integrity file was formatted. No
  denial variant, field, visibility, or behavior changed.
- closing proof: physical integrity passes all 52 compile-fail doctests with
  warnings denied, and `cargo fmt --all -- --check` passes for the complete
  Store workspace.

### `C6-P8-F036` - F005's exact-name gate admitted renamed placement readiness

- status: `CORRECTED`
- affected guarantees: `L02`, `P14`, `P16`, `P18`
- evidence: the scheduler boundary rejected `TierPlacementIoAdmission` and
  `IoSchedulerIsolationAdmission` by name, but a new tiering
  `io_readiness/placement_permit.rs` or a blob placement intent importing
  scheduler admission under a renamed type could pass.
- correction: any tiering `io_readiness/` source is forbidden, and tiering plus
  blob placement-admission sources reject scheduler/readiness imports and type
  fragments. Separate hostile mutants use `PlacementExecutionPermit` rather
  than any deleted public type name.
- closing proof: the six-test scheduler-authority family executes with warnings
  denied and both renamed mutants fail at the path-scoped placement predicate.

### `C6-P8-F037` - Class-specific placement evidence inflated one transient enum

- status: `CORRECTED`
- affected guarantees: `L02`, `P13`, `P15`, `P18`
- evidence: warnings-denied Clippy measured `BlobPlacementIntent` at at least
  752 bytes because external recoverability and cold posture were stored inline.
  Boxing would have hidden a heap allocation on per-placement admission.
- correction: `BlobPlacementIntent<'evidence>` borrows the exact external,
  sidecar, or cold evidence for one synchronous admission. The admitted
  placement retains only validated blob facts, class, cold state, counters, and
  explicit non-claims. No heap allocation or proof reconstruction was added.
- closing proof: owner tests assert pointer identity for external and cold
  evidence and cap the intent at four machine words; the full 177-unit and
  162-doctest blob suite passes with warnings denied. Blob all-target,
  all-feature Clippy passes with only the pre-existing, out-of-scope
  `large_enum_variant` class excluded for
  `streaming/read/denial.rs`; all other warnings remain denied.

### `C6-P8-F038` - Movement documentation retained deleted readiness authority

- status: `CORRECTED`
- affected guarantees: `L02`, `P12`, `P18`
- evidence: `placement/movement/mod.rs` still claimed
  `S7PlacementIoReadinessSeed` entered at placement admission and survived in
  every `AdmittedBlobPlacement`, contradicting the production type graph.
- correction: the state graph now says placement carries no scheduler
  readiness, distinguishes inline/external/cold evidence, and assigns movement
  scheduling to the movement execution lane. The stale identifier is now part
  of both the scheduler boundary and removal-inventory classifier.
- closing proof: production source contains zero
  `S7PlacementIoReadinessSeed` references; the exact-name mutant fails; all six
  scheduler boundary and 24 removal-inventory tests pass.

### `C6-P8-F039` - Placement capability verification hid three class paths

- status: `CORRECTED`
- affected guarantees: `P15`, `P18`
- evidence: `verify_class_backend_capability` combined inline, external, and
  cold capability, authority, and denial behavior in one 75-line function.
- correction: exhaustive dispatch now delegates to named inline, external, and
  cold verifiers with one shared typed backend-capability check.
- closing proof: targeted function scrutiny reports zero advisory functions
  across the placement verifier, owner tests, and scheduler gate; the focused
  9-test and complete 177-test blob lanes pass with warnings denied.

### `C6-P8-F040` - Capacity-era deletions were absent from the canonical inventory

- status: `CORRECTED`
- affected guarantees: `L02`, `P12`, `P14`, `P20`
- evidence: the removal ledger and its semantic classifier contained no rows or
  family for the deleted scheduler capability, scheduler proof, progression
  tests, or blob pacing-admission owner.
- correction: the four exact deleted paths now carry
  `scheduler-capacity-publication` dispositions and path-bound replacement
  owners. The classifier recognizes both those paths and the deleted
  capability, authority, progression, conversion, boolean-laundering, and
  self-admission fragments.
- closing proof: the dedicated classifier test rejects every listed fragment
  and deleted path; all 25 removal-inventory tests pass live discovery equality,
  completed-row replacement ownership, and hostile inventory mutants.

### `C6-P8-F041` - Non-admitted scheduler outcomes lacked compiler-negative lease proof

- status: `CORRECTED`
- affected guarantees: `P11`, `P16`, `P17`
- evidence: runtime matching showed that yield, deferred, denied, and violation
  outcomes carried no lease, but no compiler evidence prevented a later public
  lease extractor from appearing on those outcome types.
- correction: independent compile-fail specimens attempt `into_lease` on each
  non-admitted outcome type. Runtime tests additionally prove that a throttled
  outcome with zero admitted budget yields no lease while partial and
  policy-limited nonzero admission yield the exact admitted budget.
- closing proof: all 14 scheduler doctests and all 87 scheduler unit tests pass
  with every feature enabled and warnings denied.

### `C6-P8-F042` - Blob compaction's type boundary lacked hostile compiler proof

- status: `CORRECTED`
- affected guarantees: `L02`, `P11`, `P16`, `P17`
- evidence: production types required scheduler pacing, but the blob crate had
  no compile-fail evidence for unpaced planning, reusing one lease across two
  compactions, or direct consumer self-admission.
- correction: the compaction owner now publishes three separate compile-fail
  specimens at its module boundary for those exact attacks.
- closing proof: all 165 blob doctests pass with every feature enabled and
  warnings denied; the positive compaction journey consumes a real
  scheduler-issued lease, and an ingest-class lease is rejected before
  planning.

### `C6-P8-F043` - Exact capacity names did not exclude renamed authority laundering

- status: `CORRECTED`
- affected guarantees: `L02`, `P14`, `P16`, `P17`
- evidence: source absence for the historical capability and constructor names
  could remain green while a renamed boolean pacing struct or
  declaration-derived compaction permit recreated the same authority bypass.
- correction: a repository boundary inspects scheduler and blob compaction
  responsibilities semantically: authority-bearing stages must be move-owned,
  blob pacing may retain only the exact scheduler lease, its constructor must
  consume that lease, and declaration or boolean fields are forbidden.
- closing proof: all three capacity-boundary tests pass; separate renamed
  boolean and declaration-based mutants fail without using any deleted public
  identifier.

### `C6-P8-F044` - Capacity classification accumulated mixed semantic branches

- status: `CORRECTED`
- affected guarantees: `P15`, `P20`
- evidence: adding the capacity family expanded
  `classify_identifier_families` to 98 lines and mixed legacy, scheduler, and
  closeout classifications in one function.
- correction: the classifier is a five-line dispatcher over separately named
  legacy, scheduler, and closeout fragment tables plus one shared insertion
  operation. The capacity family remains grouped with scheduler publication.
- closing proof: the classifier owner is 199 lines, every new or changed
  classifier/gate function remains below the 60-line advisory, the focused
  capacity-classifier test passes, and all 25 inventory tests remain green.

### `C6-P8-F045` - Queue hostile mutants could silently no-op on CRLF source

- status: `CORRECTED`
- affected guarantees: `P14`, `P17`
- evidence: the first ready-plan Clone mutant replaced one LF-specific
  derive-and-declaration sequence. On the Windows checkout it changed zero
  bytes, so the test expected denial without having installed its attack.
- correction: every queue-gate mutation now targets a line-ending-independent
  declaration or field token through `mutate_once`, which asserts that the
  source changed before the gate may evaluate it.
- closing proof: all four queue authority gate tests pass on the CRLF checkout;
  the ready-plan, borrowed-policy, parallel-work, and borrowed-execution mutants
  each prove a changed source and a semantic denial.

### `C6-P8-F046` - Queue ownership gate conflated policy and execution attacks

- status: `CORRECTED`
- affected guarantees: `P15`, `P17`
- evidence: the initial hostile gate test grew to 64 lines by mixing
  declaration/policy ownership attacks with request/ready/execution attacks.
- correction: policy ownership and execution progression are separate hostile
  tests with shared mutation and denial helpers.
- closing proof: the gate owner is 230 lines; its four semantic tests are 15,
  20, 29, and 38 lines, and no function exceeds the 60-line advisory.

### `C6-P8-F047` - Queue move-ownership invalidated a controlled mutation seam

- status: `CORRECTED`
- affected guarantees: `P14`, `P17`
- evidence: controlled mutant 16 still searched
  `request.work.durability_class()` after the policy receipt became the sole
  owner of work, so the full runner found zero source bindings.
- correction: the mutant retains its scheduler-admission predicate and exact
  durability-check deletion, but binds the current borrowed
  `policy_receipt.work()` local.
- closing proof: the catalog binding audit finds exactly one seam; the baseline
  detector passes; the installed mutant fails only
  `grouping_mismatch_is_a_typed_admission_denial` with matching expected and
  actual `scheduler-admission` predicates and source, mutant, and binary hashes.

### `C6-P8-F048` - Move-ownership derive inspection could bind a predecessor type

- status: `CORRECTED`
- affected guarantees: `P14`, `P17`
- evidence: the first queue ownership inspector searched backward for any
  preceding derive list, so removing a target type's derive could associate the
  previous type's list and make the proof dishonest.
- correction: inspection anchors on the full `pub struct Type` declaration,
  requires a complete immediately adjacent derive list, and then rejects
  `Clone` or `Copy`.
- closing proof: production checks pass for all five authority-bearing queue
  stages, while an inserted ready-plan Clone derive is rejected by the same
  parser.

### `C6-P8-F049` - Courtroom source manifest retained a stale workflows hash

- status: `CORRECTED`
- affected guarantees: `L01`, `L02`, `P09`, `P10`, `P15`
- evidence: the Phase 16 lifecycle courtroom rejected the current
  `workflows.rs` bytes because its checked-in source manifest still carried
  the pre-correction hash. The courtroom therefore failed before exercising
  its lifecycle claims.
- correction: the manifest row now carries the SHA-256 of the exact
  line-ending-normalized source bytes:
  `ec629e6d134c199a5e45cb3d48502625cfcf1312cee6ae7e1f81f7e8611c25c2`.
  Independent .NET and Python implementations agreed on the 8,749-byte input
  and digest.
- closing proof: the exact source-manifest selector and the complete Phase 16
  lifecycle-maelstrom courtroom selector both execute and pass.

### `C6-P8-F050` - Sealed record-chunk UI proof expected obsolete diagnostics

- status: `CORRECTED`
- affected guarantees: `L02`, `P05`, `P09`, `P13`, `P14`
- evidence: the record-chunk construction specimen correctly attacked the
  current sealed boundary, but its trybuild snapshot still expected diagnostics
  from the former constructor topology.
- correction: the snapshot now records the current causal failures:
  `PhysicalRecordChunkBasis::new` does not exist and
  `PhysicalRecordChunkView::new` is private.
- closing proof: the complete 30-specimen physical-runtime authority UI lane
  executes and passes; the specimen reaches both intended constructor
  boundaries.

### `C6-P8-F051` - Certification preserved four synthetic S.2 memory evidence families

- status: `CORRECTED`
- affected guarantees: `L02`, `P01`, `P07`, `P09`, `P10`, `P12`, `P14`
- evidence: allocation-envelope, eviction-protection, pin-lifecycle, and
  resident-frame-authority evidence in certification directly constructed the
  obsolete S.2 allocation admission and `ResidentFrameTable`. Public substrate
  exports and canonical-basis rows then advertised that parallel model as
  Store evidence.
- correction: all four evidence modules and all four synthetic test modules
  are deleted. Their memory-module declarations, substrate and public exports,
  and eight canonical-basis registry rows are removed. The eight exact removal
  rows are complete with path-bound owners under canonical
  `worth-store-buffer-pool::physical_residency`; no producer API is deleted
  before its remaining consumers migrate.
- closing proof: certification compiles with all features; the source boundary
  finds all eight paths absent and all twelve exported evidence symbols
  unreferenced; all 25 removal-inventory tests pass; and 22 focused canonical
  owner tests plus 10 identity/hot-access tests execute allocation pressure,
  exact operation accounting, eviction siege, pin pressure, shutdown
  reconciliation, frame-access lifecycle, exact-and-bounded identity
  transition, foreign-key rejection, hot hits, and fault-owner convergence.
  The affected certification library is Clippy-clean with warnings denied.

### `C6-P8-F052` - Scrub receipt registry row pointed at a deleted source owner

- status: `CORRECTED`
- affected guarantees: `L02`, `P09`, `P12`, `P14`
- evidence: the live canonical-basis scanner found
  `ScrubExecutionReceipt` at `scrub/execution/receipt.rs`, while the registry
  still named deleted `scrub/scrub_execution.rs`. Five neighboring registry
  tests passed and the source-coverage test rejected the stale owner.
- correction: the existing row now binds the receipt to
  `worth-store-physical-integrity::scrub::execution::receipt`; no duplicate row
  or scanner exception was added.
- closing proof: all six canonical-basis source tests execute and pass,
  including live scanned-source coverage.

### `C6-P8-F053` - Consumer deletion left an empty memory-evidence module shell

- status: `CORRECTED`
- affected guarantees: `L02`, `P01`, `P12`, `P14`, `P15`
- evidence: after the four synthetic evidence families were deleted,
  certification still declared `evidence::memory` and retained a two-byte
  `memory/mod.rs` with no semantic responsibility, owner, or expected growth.
- correction: the empty module file and its parent declaration are deleted.
  The certification evidence topology now names only live semantic families.
- closing proof: source inspection finds no `evidence::memory` reference,
  parent declaration, or surviving file beneath `evidence/memory`;
  certification compiles with all features; all six canonical-basis tests and
  all 25 removal-inventory tests pass. Formatting, dirty-file line cap,
  Road 1 boundary-check, and generated agent-context checks pass.

### `C6-P8-F054` - Record-view wrappers escaped identifier-only inventory

- status: `CORRECTED`
- affected guarantees: `L02`, `P01`, `P05`, `P09`, `P12`, `P14`, `P20`
- evidence: the record-view conflict courtroom and its test support consumed
  `RecordViewEvidenceReport` and helper constructors without spelling the
  lower `ZeroCopyRecordView` or `ResidentFrameTable` identifiers used by the
  original classifier. Renaming the file contents could also hide the family.
- correction: record-view wrapper vocabulary is classified directly, and the
  four exact evidence, admission, conflict, and courtroom-support paths retain
  path-scoped classification even when their contents are renamed.
- closing proof: the wrapper-fragment controls and all four renamed-path hostile
  controls execute and pass; exact live inventory now includes the conflict
  courtroom and support path rather than relying on lower-type coincidence.

### `C6-P8-F055` - Executable Markdown was outside removal inventory

- status: `CORRECTED`
- affected guarantees: `L02`, `P01`, `P12`, `P14`
- evidence: inventory source discovery admitted Rust and Cargo manifests only.
  Code-adjacent compile-fail documents could therefore preserve deleted API
  names indefinitely. Adding Markdown discovery immediately exposed legacy
  buffer-pool proofs, the obsolete certification receipt document, and stale
  physical-integrity specimens. It also exposed one intentional blob attack
  on the prohibited `admitted_compaction` call.
- correction: Markdown under `src` or `tests` is now inventory source. Legacy
  type names remain classifiable there, while the removed self-admission call
  fragment is classified as live bypass use only in Rust; the compile-fail
  specimen remains free to attempt the forbidden call.
- closing proof: the source-selection regression and the live-Rust-versus-
  negative-Markdown classifier regression execute and pass. Exact inventory
  passes with the legacy buffer-pool proof document still open, the obsolete
  certification document deleted, and the current blob attack unclassified as
  a live consumer.

### `C6-P8-F056` - Certification retained a parallel record-view and aggregate-receipt world

- status: `CORRECTED`
- affected guarantees: `L02`, `P01`, `P05`, `P07`, `P09`, `P10`, `P12`, `P14`
- evidence: certification wrapped the legacy record-view graph in synthetic
  evidence reports, rebuilt a completed residency receipt from copied pool
  counters, joined both into protected-integrity evidence, exported the model
  publicly, and registered seven synthetic evidence owners as canonical.
- correction: the record-view evidence module, two synthetic test modules,
  courtroom support, aggregate foundational evidence and tests, protected-view
  evidence, and obsolete receipt compile-fail document are deleted. All module
  declarations, public/substrate exports, and seven canonical-basis rows are
  removed. Exact removal rows bind replacement evidence to the real Store
  record-chunk authority suite, real Store test world, canonical pool conflict
  tests, and current integrity-entry courtroom.
- closing proof: all eight obsolete paths and all exported synthetic symbols are
  absent; certification checks with all features; 32 certification doctests,
  30 Store authority UI specimens, the real Store record-chunk world test,
  three integrity-entry authority tests, and six canonical source-owner tests
  execute and pass. Certification is Clippy-clean with warnings denied.

### `C6-P8-F057` - Obsolete aggregate receipts owned a live performance denial contract

- status: `CORRECTED`
- affected guarantees: `L02`, `P07`, `P12`, `P15`
- evidence: deleting the aggregate receipt module wholesale would also delete
  `FoundationalBoundaryEvidenceDenial`, which still carried the two real
  failure variants used by S5, S5.1, and S6 counter-backed performance
  receipts. The shared performance builder also retained five now-unused pool
  counter translation helpers after aggregate deletion.
- correction: the current contract is extracted and semantically narrowed to
  `FoundationalPerformanceEvidenceDenial` in
  `evidence/foundational/performance_evidence_denial.rs`. Current S5, S5.1,
  and S6 consumers use that type. The unused resident, allocation, and copy
  translators plus their private scope helper are deleted, removing the
  performance builder's direct pool edge without deleting its live
  `counter_receipt` responsibility.
- closing proof: all-features certification check and warnings-denied Clippy
  pass with no dead helpers; exact inventory closes the performance-builder
  direct-edge row; Store-wide source composition remains at or below 400 lines.

### `C6-P8-F058` - Physical-integrity compile-fail docs proved obsolete pool names

- status: `CORRECTED`
- affected guarantees: `L02`, `P05`, `P07`, `P12`, `P14`
- evidence: two physical-integrity negative specimens used the legacy
  `ResidentFrameToken`. Their green result could arise from an obsolete import
  instead of reaching a current authority mismatch. Replacing the token with a
  current pool lease preserved an equally dishonest direct pool dependency.
- correction: both redundant cross-substrate specimens are deleted. Existing
  current specimens already attack the actual `IntegrityEntryRequest`
  boundary: raw paths and `PhysicalRecordChunkView` cannot substitute for
  `ProtectedPhysicalByteView` even when a real
  `VerificationPhysicalAllocation` is supplied.
- closing proof: the integrity proof document contains no pool import or legacy
  residency identifier; all 50 physical-integrity compile-fail specimens
  execute and pass; exact inventory and the Phase 8 replacement-owner gate
  execute and pass; physical-integrity is Clippy-clean with warnings denied.

### `C6-P8-F059` - Certification retained synthetic speculation, dirty-publication, and S.2 entry evidence

- status: `CORRECTED`
- affected guarantees: `L02`, `P01`, `P03`, `P04`, `P06`, `P07`, `P09`,
  `P10`, `P12`, `P14`
- evidence: three certification-local evidence families consumed legacy pool
  plans, admissions, frame-table counters, dirty states, publication receipts,
  shutdown reports, and S.2 entry denials. Their only consumers were synthetic
  tests, public/substrate exports, and six canonical-basis registry rows.
  Canonical pool and Store owners already execute each useful speculation,
  dirty-transition, writeback, shutdown, and entry-authority obligation.
- correction: speculative-work evidence and tests, dirty-publication evidence
  and tests, dirty-publication courtroom support, and S.2 entry-boundary
  evidence are deleted. Their declarations and public/substrate exports are
  removed, as are all six synthetic canonical-basis rows. Exact removal rows
  point to Store speculation and dirty topology, canonical pool speculation,
  clean-to-dirty and shutdown tests, and Store authority UI evidence.
- closing proof: certification checks with all features and is Clippy-clean
  with warnings denied; all 27 removal-inventory tests and all six canonical
  source-owner tests pass. The complete 165-test buffer-pool owner suite
  executes the canonical speculation, limits, clean-to-dirty, writeback,
  shutdown, allocation, eviction, and frame-access laws with no failure.

### `C6-P8-F060` - Synthetic evidence deletion left three empty topology shells

- status: `CORRECTED`
- affected guarantees: `L02`, `P01`, `P12`, `P15`
- evidence: deleting the final synthetic evidence children left
  `evidence/durability/mod.rs`,
  `evidence/physical_substrate/mod.rs`, and the
  `evidence/by_substrate/buffer_pool.rs` facade with no semantic
  responsibility. Rustfmt exposed the facade as a doc-comment-only file.
- correction: all three shells and their parent declarations are deleted.
  Legitimate courtroom durability and physical-substrate modules remain
  untouched because they own distinct live responsibilities.
- closing proof: all nine synthetic sources and topology shells are absent;
  deleted evidence type/support names are absent globally; the scoped evidence
  module declarations are absent while courtroom modules remain. Formatting,
  diff hygiene, and the Store-wide 400-line source gate pass.

### `C6-P8-F061` - Warnings-denied Store analysis exposes unresolved composition and layout debt

- status: `CORRECTED`
- affected guarantees: `P12`, `P13`, `P15`
- failed evidence: three-crate all-target/all-feature clippy with warnings
  denied reaches the canonical `worth-store` dependency and fails on four
  needless queue-policy borrows, two eight-argument functions, and two
  materially imbalanced runtime enums.
- correction: the queue-policy calls now pass the existing declaration borrow
  directly. Capacity rebuild and prepared root publication use semantic request
  packets; the manifest planner selects named capacity-rebuild or same-capacity
  paths, and recursive rebuild state lives in one traversal owner. The
  cloneable residency port shares one stable access bundle allocated at runtime
  construction, keeping ordinary readers small without per-read allocation.
  Writeback effect dispatch is an exact `Result`: the hot completed-effect
  continuation stays inline and only uncommon terminal failures are boxed.
  Compile-time size assertions bound the residency port and writeback result.
- closing proof: Store all-target/all-feature check, all 75 Store owner tests,
  five manifest-focused journeys, and the explicit manifest-capacity
  reconstruction journey pass. Dirty function scrutiny reports no collapsed
  manifest request or recursive traversal signature. All 4,992 tracked
  worth-store Rust files satisfy the 400-line gate with a 400-line maximum.
  The original three-crate all-target/all-feature Clippy command passes with
  warnings denied and no lint allowances.

### `C6-P8-F062` - Scheduler certification rebuilt a pool to declare non-pool work

- status: `CORRECTED`
- affected guarantees: `P01`, `P07`, `P09`, `P12`, `P16`, `P17`
- evidence: six certification consumers reached through
  `scheduler_declaration_fixture`, which constructed a buffer pool and lowered
  a pool read declaration solely to obtain scheduler work. The fixture gave
  scheduler tests an irrelevant residency dependency and left certification
  with a direct `worth-store-buffer-pool` edge.
- correction: `PhysicalForegroundWorkDeclaration` now consumes one admitted
  foreground reservation through closed `read`, `buffered_write`, and
  `durable_write` constructors. Lowering derives security from the reservation,
  fixes durability/writeback/recovery posture, and consumes the declaration.
  Test support owns a scheduler-native foreground-read fixture under
  `harness/scheduling/`; all six certification consumers use it. The obsolete
  top-level fixture and certification pool dependency are deleted.
- closing proof: the scheduler passes 89 owner tests and 21 compile-fail
  doctests, including fixed-posture and duplicate-lowering proofs. Thirteen Store
  scheduler journeys, seven scheduler-queue certification tests, 34 I/O
  scheduling tests, and the 23-unit/one-integration/two-doctest test-support
  suite pass. Scheduler, certification, and test support are Clippy-clean with
  warnings denied. Certification has zero direct pool imports or dependency;
  the only three remaining pool references across physical certification,
  certification, and test support are the exact recovery fixture feature,
  optional dependency, and implementation scheduled for the next migration
  slice. The bounded scheduler migration inventory contains 140 changed paths,
  including two intentional deletions, and its 138 present files pass the
  line-terminator-independent whitespace and conflict-marker scan.

### `C6-P8-F063` - Blob streaming denial layout carries an oversized stable-read failure inline

- status: `CORRECTED`
- affected guarantees: `P12`, `P13`, `P15`
- failed evidence: after the Store F061 corrections, the original all-target,
  all-feature, warnings-denied Clippy gate reaches
  `BlobStreamingReadDenial` and reports a 552-byte
  `StablePhysicalReadDenied` variant against a 312-byte next-largest variant.
- correction: the complete `PhysicalReadExecutionDenial` is boxed only at the
  sole stable-read rejection translation. Successful streaming reads allocate
  nothing for this path, while the failure retains its exact physical-isolation
  denial and proof context.
- closing proof: the complete six-test blob streaming-read module passes,
  including an exact oracle that unboxes and compares the retained
  `PhysicalReadExecutionDenial`; the original warnings-denied Clippy command
  passes without a lint allowance.

### `C6-P8-F064` - Backup artifact materialization retains a needless digest borrow

- status: `CORRECTED`
- affected guarantees: `P12`, `P13`, `P15`
- failed evidence: after F063 is corrected, the original warnings-denied
  Clippy gate reaches canonical backup artifact materialization and rejects
  `Sha256::digest(&bytes)` because the generic input already satisfies the
  required borrow contract.
- correction: canonical index bytes are borrowed for media materialization and
  then moved into final digest construction; no clone or second owner is added.
- closing proof: both focused backup certification scenarios pass, and the
  original warnings-denied Clippy command passes without a lint allowance.

### `C6-P8-F065` - Scheduler declaration proof accepted any locality identity

- status: `CORRECTED`
- affected guarantees: `P09`, `P10`, `P13`, `P16`, `P17`
- failed evidence: the focused physical-foreground declaration test asserted
  only that lowered grouping carried some locality. A constant, substituted,
  or otherwise incorrect locality identity would have passed while violating
  the declaration's exact caller-scope contract.
- correction: every read, buffered-write, and durable-write case now carries
  its expected locality beside the declaration and asserts exact equality
  after lowering.
- closing proof: the focused constructor test passes, all 21 scheduler
  compile-fail documentation tests pass, and 13 scheduler-filtered real Store
  composition-root journeys pass under their explicit certification authority
  feature.

### `C6-P8-F066` - Secure-I/O execution proof accepted an unrelated violation cause

- status: `CORRECTED`
- affected guarantees: `P09`, `P10`, `P13`, `P16`
- failed evidence: the secure-frame certification scenario asserted only that
  a cross-key backend completion produced some `QueueExecutionOutcome::Violation`.
  `ExecutionReclassifiedWork` would have satisfied the test even though it
  would not prove that the backend contradicted its secure-I/O witness.
- correction: the scenario now asserts the exact
  `QueueExecutionViolationCause::BackendContradictedWitness`.
- closing proof: the exact secure-frame scenario passes through the real
  `io_scheduling` integration target, and the complete seven-test scheduler
  queue-execution certification target passes.

### `C6-P8-F067` - Queue execution self-minted generic proof authority beneath concrete typestates

- status: `CORRECTED`
- affected guarantees: `P12`, `P13`, `P16`, `P17`
- evidence: final-source inventory found `AuthorityMarker`,
  `AuthorityWitness`, `CapabilityMarker`, and `CapabilityWitness` in queue
  execution progression. Private recipe transitions minted resolution and
  readiness authority locally even though the public
  `AdmittedQueueExecutionPlan`, `QueueExecutionReadyPlan`, and
  `QueueExecutedPlan` already formed the concrete move-owned progression. An
  unused public `queue_execution_lowering_authority()` token opened no path,
  and `worth-proof` remained as a dependency solely for this redundant layer.
- correction: the concrete public plan typestates are now the only queue
  progression proof. Admission constructs the ready plan; execution consumes
  it into the executed plan. The generic recipe layer, ceremonial public
  authority token, exports, and dead `worth-proof` dependency are removed.
  The scheduler authority gate now rejects exact generic proof identifiers
  across the scheduler's complete Rust source tree, including a renamed
  queue-execution mutant.
- closing proof: exact source inventory finds no generic proof identifier or
  deleted lowering-authority export in the scheduler. All 89 scheduler owner
  tests, 21 compile-fail documentation tests, 13 Store scheduler journeys,
  seven scheduler certification tests, 34 I/O scheduling scenarios, and the
  full test-support suite pass. The original broad warnings-denied Clippy gate
  also passes.

### `C6-P8-F068` - Widened scheduler authority gate overclassified concrete witnesses

- status: `CORRECTED`
- affected guarantees: `P14`, `P16`
- evidence: widening the existing background-only proof-vocabulary gate to
  the full scheduler source initially matched `AuthorityWitness` and
  `CapabilityWitness` as raw substrings inside legitimate concrete platform
  types such as `AdmittedBackendCapabilityWitness`.
- correction: the gate now recognizes exact Rust identifier boundaries rather
  than arbitrary substrings. A positive control admits a concrete backend
  witness while the renamed generic background and queue-execution authority
  mutants remain rejected.
- closing proof: all eight focused scheduler-authority gate tests pass, and the
  gate accepts the complete current scheduler source tree.

### `C6-P8-F069` - Completed removal rows named prose instead of live replacement owners

- status: `CORRECTED`
- affected guarantees: `P01`, `P12`, `P14`, `P20`
- evidence: the complete removal-inventory family failed its Phase 8
  replacement-owner gate. The three latest scheduler migration rows and 13
  earlier migrated-in-place certification, physical-certification, and
  test-support rows used conceptual prose such as “canonical Store-bound
  residency and evidence” rather than an inspectable source path. The
  sequential validator exposed only the first invalid row per run.
- correction: the full completed Phase 8 ledger was audited in one pass. The
  three scheduler rows now identify their exact current manifest, secure-I/O
  scenario, and scheduler-native test-support fixture. Every migrated-in-place
  row binds to its exact surviving workspace path.
- closing proof: all 105 completed Phase 8 rows resolve to present path-bound
  owners with zero invalid rows, and all 27 removal-inventory tests pass.

### `C6-P8-F070` - Scheduler construction flattened posture and validated admission facts

- status: `CORRECTED`
- affected guarantees: `P12`, `P13`, `P15`, `P16`, `P17`
- evidence: dirty function scrutiny found a six-argument physical foreground
  declaration helper that accepted durability and writeback as independent
  values, plus a six-argument admitted-plan constructor that unpacked an
  already validated queue transition into unrelated parts. The public
  declaration constructors fixed posture correctly, but the internal boundary
  could still recreate an incoherent pair; the plan boundary relied on call
  placement rather than carrying one validated fact.
- correction: `PhysicalForegroundOperationPosture` is a closed internal choice
  that derives durability and writeback together, while
  `PhysicalForegroundWorkInputs` carries the common reservation, locality,
  resource, and epoch basis. Queue validation now emits one
  `ValidatedQueueExecutionAdmission`, and plan construction consumes that
  packet as a single compiler-visible phase result.
- closing proof: dirty function scrutiny reports 48 advisory candidates with
  zero scan errors; neither scheduler constructor remains. The only scheduler
  advisory is the cohesive 64-line parameterized three-posture test. All 89
  scheduler owner tests, 21 compile-fail documentation tests, 13 Store
  scheduler journeys, seven scheduler certification tests, 34 I/O scheduling
  scenarios, eight scheduler-authority gates, and warnings-denied Clippy pass.

### `C6-P8-F071` - Removal inventory missed ungated predecessor files and orphaned authority consumers

- status: `CORRECTED`
- affected guarantees: `L02`, `P01`, `P03`, `P04`, `P06`, `P09`, `P10`,
  `P12`, `P14`, `P20`
- evidence: the Phase 8 ledger classified every previously discovered path,
  yet three unconditional buffer-pool predecessor files were absent and the
  buffer-pool manifest plus crate facade were incorrectly marked for deletion.
  After deleted page-publication authority definitions disappeared, 20 live
  production, certification, formal-model, and test-support consumers were
  also invisible to the identifier inventory. Extending path closure exposed
  the complete 27-path mixed cutover and reopened a falsely completed
  certification registry row.
- correction: the inventory now classifies every identifier-free Rust file
  outside the canonical `physical_residency` destination as a buffer-pool
  predecessor and recognizes the obsolete `page_lsn_publication` module plus
  its deleted authority identifiers wherever consumers retain them. The CSV
  now gives all 271 rows an explicit `narrow` or `delete` disposition, adds the
  missing predecessor and consumer paths, narrows the surviving pool manifest
  and facade, reopens the stale registry, unions module-closure families, and
  binds every open row to an exact replacement path.
- closing proof: the hostile predecessor, canonical-destination control,
  orphaned-authority consumer, identifier-free module descendant, and suite
  selector tests pass. All 30 removal-inventory tests pass, including exact
  family reconciliation, typed disposition parsing, replacement-owner
  validation, stale-open denial, and rediscovered-completed-row denial.

### `C6-P8-F072` - Disposition-complete rows retained stale status and unproved replacement owners

- status: `CORRECTED`
- affected guarantees: `L02`, `P01`, `P12`, `P14`, `P20`
- evidence: a complete disposition audit found that all 271 rows named
  `preserve`, `narrow`, or `delete`, but 111 finished Phase 8 rows still said
  `inventory-open`. Five earlier-phase rows retained prose rather than
  path-bound replacement owners, and one completed recovery row pointed back
  into the deleted `page_lsn_publication` tree. The validator checked selected
  completed phases separately and therefore did not enforce replacement-owner
  integrity across the entire ledger.
- correction: every row was reviewed against path fate, semantic family, and
  replacement ownership. All 180 whole-path predecessors are classified
  `delete` and are absent; all 91 files with surviving current responsibility
  are classified `narrow` and remain present. No row is `preserve`: retaining
  an unchanged legacy-matched surface would violate the Phase 8 cutover, while
  independently valuable behavior survives through a canonical replacement
  owner or a narrowed current file. The six invalid owners now resolve to
  exact current paths, all 111 stale statuses are closed as
  `deleted-phase-8`, and the gate validates every row in one aggregated pass.
- closing proof: all 271 rows have a nonempty typed disposition and basis,
  zero duplicate paths, zero open statuses, and 271 present path-bound
  replacement owners. All 31 removal-inventory tests pass, including the new
  multi-defect owner-integrity mutant, exact live-family reconciliation,
  stale-open denial, and rediscovered-deleted denial.

### `C6-P8-F073` - Queue authority gate required a deleted generic proof recipe

- status: `CORRECTED`
- affected guarantees: `L02`, `P13`, `P14`, `P16`, `P17`
- evidence: the complete physical-residency boundary family failed while
  inspecting `QueueExecutionReadyPlan`. Production still move-owned the exact
  `AdmittedQueueExecutionPlan`, but the source oracle additionally required a
  `QueueReadyRecipe` field deleted by F067 because that redundant generic
  recipe self-minted authority beneath the concrete plan typestates.
- correction: the queue progression oracle now requires the current concrete
  ready-plan shape: one owned admitted plan plus observational progression.
  It does not restore the forbidden recipe. A new controlled mutant replaces
  the admitted plan with a flattened work declaration and must fail the same
  boundary gate.
- closing proof: all four focused queue-authority tests pass, including the
  new flattened-ready-plan mutant. All 89 scheduler owner tests, 21 scheduler
  compile-fail documentation tests, and the complete 115-test physical
  residency boundary family pass with warnings denied.

### `C6-P8-F074` - A renamed forgeable count snapshot survived outside the removal inventory

- status: `CORRECTED`
- affected guarantees: `L02`, `P01`, `P03`, `P12`, `P14`, `P20`
- evidence: an independent deleted-authority scan found the public
  `PhysicalSubstrateReadinessSnapshot`. It exposed `from_exact_counts` over
  seven caller-supplied readiness fields and was reconstructed by a public
  `PhysicalSubstrateReadiness::physical_substrate_snapshot` accessor. No live
  consumer used the packet, and none of its contracts or readiness paths
  appeared in the removal ledger because the classifier recognized only the
  original `S2*` authority names.
- correction: the snapshot-only contracts module and forgeable packet are
  deleted. The contracts facade and proof-derived readiness type remain, but
  no longer export or reconstruct count-snapshot readiness. The removal ledger
  adds exact `delete` or `narrow` dispositions for all four paths, and the
  classifier recognizes the renamed snapshot identifier as
  `snapshot-residency-authority`.
- closing proof: the renamed-definition, re-export, and return-type mutants
  are rejected. All 32 removal-inventory tests pass with exact live-family
  reconciliation. Contracts and readiness pass six owner tests, one contracts
  compile-fail test, and eight readiness compile-fail tests with warnings
  denied. Independent production scans find zero legacy feature, deleted
  authority, page-publication, C6 Rust/manifest, or executable-document
  matches outside the exact enforcement sources that must spell the forbidden
  vocabulary.

### `C6-P8-F075` - Blob consumer ownership lacked the compiler attacks required by `P19`

- status: `CORRECTED`
- affected guarantees: `L02`, `P17`, `P19`
- evidence: the runtime traces retain exact scheduler-class leases through
  ingest, verification read, and compaction, but the blob owner had no direct
  compile-fail specimens proving its public ingest and read admissions are
  non-cloneable or unusable after move. Compaction proved lease
  single-consumption and unpaced-planning rejection, but did not attack the
  paced intent itself. `P19` explicitly requires consumer-level non-clone and
  move-after-use compiler failures, and `F008` recorded that evidence as still
  open.
- correction: add owner-level compiler attacks for clone and second
  consumption of `BlobStreamingPressureAdmission`,
  `BlobStreamingReadAdmission`, and `BlobCompactionIntent`. Keep the attacks
  on public consumer types so the proof cannot pass merely because a private
  scheduler field happens to be inaccessible.
- closing proof: all six new compiler attacks fail at the intended public
  ownership boundary. The complete blob owner passes 171 documentation tests
  and 178 runtime tests with every feature enabled and warnings denied.

### `C6-P8-F076` - Foreground-reservation tests retained a forbidden `common.rs` bag

- status: `CORRECTED`
- affected guarantees: `L02`, `P12`, `P20`
- evidence: the whole-dirty-set code-quality audit found
  `worth-store-io-scheduler/src/foreground_reservation/tests/common.rs`.
  Despite being test support, it combined five independent responsibilities:
  reservation-case construction, resource budgets, policy-capacity admission,
  backend capability admission, and security-scope admission. The generic
  filename and wildcard consumers violated the composition law and hid future
  insertion points.
- correction: delete `common.rs`; move those responsibilities into
  `backend_capability.rs`, `capacity_policy.rs`, `foreground_case.rs`,
  `resource_budget.rs`, and `security_scope.rs`; and make every consumer import
  only the semantic fixture owners it uses.
- closing proof: all 23 focused foreground-reservation tests, 21 scheduler
  documentation tests, and 89 scheduler owner tests pass with every feature
  enabled and warnings denied. The whole-dirty-set scan finds zero forbidden
  catch-all filenames or empty survivors; function scrutiny reports zero scan
  errors with every advisory inspected; and no dirty or untracked Worth Store
  Rust file exceeds 400 lines.

### `C6-P8-F077` - Deletion left obsolete predecessor directory topology on disk

- status: `CORRECTED`
- affected guarantees: `L02`, `P12`, `P15`, `P20`
- evidence: a diff-bound filesystem audit found 15 directory subtrees that
  contained Phase 8 predecessor files at the implementation checkpoint and now
  contain no files: nine retired buffer-pool owner families, three retired
  certification evidence families, the deleted page-LSN certification
  scenario, and the retired contracts and recovery-physics predecessor
  families. These are deletion residue, not intentional semantic growth seams.
- correction: remove only those exact empty predecessor subtrees,
  preserving unrelated empty directories that express deliberate future
  insertion points.
- closing proof: an independent scan derives directory candidates from all 128
  deleted Worth Store paths, walks their still-present crate-local ancestors,
  and finds zero ancestor subtrees without files.

### `C6-P8-F078` - Delete dispositions were typed but not bound to path absence

- status: `CORRECTED`
- affected guarantees: `L02`, `P01`, `P12`, `P14`, `P20`
- evidence: all 275 removal rows carry a parsed `preserve`, `narrow`, or
  `delete` disposition, but the disposition gate checks path existence only
  for `preserve` and `narrow`. A row marked `delete` can therefore survive as a
  present file after its recognized legacy vocabulary is removed and still
  satisfy both the inventory reconciliation and disposition checks.
- correction: bind every disposition to exact path fate:
  `preserve` and `narrow` require a present path, while `delete` requires an
  absent path. Add controlled positive and hostile tests for all three
  variants.
- closing proof: all 33 removal-inventory tests pass with every feature enabled
  and warnings denied. The controlled path-fate test rejects a live `delete`,
  an absent `narrow`, and an absent `preserve`, while accepting the inverse
  valid states; the parser independently accepts exactly `preserve`, `narrow`,
  and `delete`. The real 275-row ledger passes the same gate.

### `C6-P8-F079` - Blob migration replaced a planned-window oracle with raw bytes

- status: `CORRECTED`
- affected guarantees: `P09`, `P13`, `P19`
- failed evidence: the full certification owner suite reached
  `protected_bytes_cannot_exceed_admitted_streaming_window` and observed 1,024
  planned windows for a 4,096-byte object inspected through 4-byte windows,
  while the migrated test expected `4`. Production derives the counter
  independently as `ceil(object_bytes / window_bytes)`; the test edit had
  substituted raw window width for planned-window count.
- correction: restore the independent 1,024-window oracle while
  retaining the Store-issued exact Blob allocation and protected-width denial.
- closing proof: the exact protected-width test passes, the complete eight-test
  blob courtroom partition passes with its one intentional ignore, and the
  production counter remains independently derived from object and window
  widths rather than the test expectation.

### `C6-P8-F080` - Durability closeout conflated policy-only page states with production ownership

- status: `CORRECTED`
- affected guarantees: `P09`, `P10`, `P12`, `P13`, `P20`
- failed evidence: Phase 8 correctly deleted the synthetic page-flush receipt
  and its production mapping, but certification still required ordinary owner
  execution to emit every formal action, including the three retained
  page-flush policy states. The formal frontier also refused a typed
  `CheckpointCutoverReceipt` progression unless the deleted receipt had first
  manufactured a durable page state. This made ordinary durability and five
  closeout tests fail.
- correction: keep page-flush states as an explicit independent
  policy-only partition, define the exact current production-owned action
  partition, require production coverage against that partition, and allow
  checkpoint durability mapped from the typed cutover receipt to advance
  without reconstructing page-flush authority.
- closing proof: seven durability tests prove the disjoint, exhaustive
  production-owned and policy-only action partitions and exercise both the
  real owner mapping and independent page policy. All four closeout tests pass.
  Bounded certification execution accounts for all 341 library tests: 336
  pass, one remains intentionally ignored, and four unchanged
  pre-Phase-8 facade-dependent failures are isolated as baseline evidence.

### `C6-P8-F081` - Speculation test used an artificial drop to end a borrow

- status: `CORRECTED`
- affected guarantees: `P13`, `P15`
- failed evidence: warnings-denied workspace Clippy rejects
  `drop(first_grant)` because `ReadAheadFrameGrant` is a borrowed view with no
  `Drop` implementation. The call exists only to force its borrow to end
  before the owning read-ahead grant is released, hiding the intended lifetime
  boundary in a procedural cleanup statement.
- correction: express the admitted grant and its borrowed frame in a
  lexical scope, then assert terminal counter reconciliation after the scope
  ends naturally.
- closing proof: the exact read-ahead authority and terminal-reconciliation
  test passes with warnings denied, and all-target/all-feature buffer-pool
  Clippy passes.

### `C6-P8-F082` - Uncommon retry authority inflated the ordinary writeback result

- status: `CORRECTED`
- affected guarantees: `P13`, `P15`
- failed evidence: workspace Clippy reports a 6,384-byte
  `PhysicalWritebackExecution` because the uncommon
  `RetryablePhysicalWriteback` retains settled scheduler evidence and dirty
  ownership inline beside a 96-byte clean settlement. No compile-time layout
  guard prevents the hot result from regressing again.
- correction: box only the uncommon retry payload at the result
  boundary, preserve its move-owned authority and consuming accessors, and add
  a compile-time size bound for the public execution result.
- closing proof: all-target/all-feature Store check passes; exact clean,
  retryable, and inspection-required journeys pass; Store library Clippy
  passes with every feature; and a compile-time assertion keeps
  `PhysicalWritebackExecution` at or below 128 bytes. All-target Clippy passes
  for every other workspace package. The only Store all-target Clippy failures
  are two test lints unchanged from the Phase 8 baseline.

### `C6-P8-F083` - The final source fingerprint was not reproducible

- status: `CORRECTED`
- affected guarantees: `L01`, `L02`
- failed evidence: the recorded manifest retained the correct 452-path scope
  and the correct 280 modified, 128 deleted, and 44 untracked counts, but its
  SHA-256 could not be reproduced from the documented row format. Eighty
  plausible status-token, hash-case, header, and final-newline encodings all
  rejected the recorded digest.
- correction: bind the final source state again using the fully explicit
  documented encoding: lowercase `modified`, `deleted`, or `untracked`;
  lowercase exact-byte SHA-256 or `-`; repository-relative slash path;
  comma-separated fields; UTF-8 ordinal path then status order; and one LF
  after every row.
- closing proof: independent PowerShell and Python implementations each
  produce 452 rows with 280 modified, 128 deleted, and 44 untracked entries,
  and both produce
  `2a17d612d5d8fbbd608757bf3036ecfc53ab964fc6ac291e4de964217d90713e`.
  The 90 additional paths reported only by porcelain status have no content
  diff and are excluded from source-state evidence.

## Surviving-Defect Attack

Before closure, answer with evidence:

1. Can any feature remain declared but disabled?
2. Can a certification feature indirectly reactivate deleted code?
3. Can a renamed count snapshot admit residency?
4. Can any fixture construct a frame table or equivalent second identity map?
5. Can a view own bytes or outlive its Store session?
6. Can a successor receive a generic grant and spend the wrong scope?
7. Can a Store observation or pressure report be promoted into authority?
8. Can certification import the pool directly and still pass the inventory?
9. Can test support counterfeit a real Store world?
10. Can an old evidence registry retain a deleted source path?
11. Can a dead dependency or feature branch survive all focused builds?
12. Can a copied mathematical test still claim production authority?
13. Can a new direct pool consumer enter outside the checked crate list?
14. Can source searches pass because policy exclusions are too broad?
15. Can every row pass while the final source differs from the audited source?
16. Can an exact successor allocation remain usable after its Store runtime
    closes or moves?
17. Can scheduler policy copy isolation counters and mint authority without
    Store composition?
18. Can a peer dependency recreate the Store-to-successor Cargo cycle?
19. Can one background admission lower two queue declarations?
20. Can yield, deferred, denied, or observational counters mint executable
    background capacity?
21. Can blob compaction enter planning without consuming scheduler-issued
    pacing admission?
22. Can inline or external placement be forced to carry cold-tier or scheduler
    readiness that has no authority over that placement class?
23. Can a move-owned background lease become duplicable after it is lowered
    into a queue declaration or policy receipt?
24. Can one policy-admitted queue work value be used to construct two ready
    execution plans?
25. Can an identifier-free Rust file survive beneath a legacy-gated module
    because only its ancestor contains the feature name?
26. Can blob ingest, verification read, or compaction begin effects after a
    yielded or zero-capacity scheduler outcome?
27. Can a Store-minted Verification allocation cover fewer bytes than the
    protected view while still minting an integrity witness?
28. Can a real-Store fixture pass successor-scope tests while failing before
    record publication reaches the claimed boundary?
29. Can synthetic S.2 closeout reports, public exports, registry rows, or
    renamed contents at a deleted source path survive because the inventory
    recognizes only the legacy authorities they describe?
30. Can a compile-fail proof remain green because setup is stale or incomplete
    before the compiler reaches the authority substitution it claims to test?
31. Can a runtime denial protect the implementation while the exhaustive
    certification suite silently omits that same boundary?
32. Can cleanup delete or split a semantic owner while a composition fixture
    stays green only because its failing path is never executed?
33. Can a consumer retain legacy authority by calling a constructor through a
    helper while never spelling the legacy type name the inventory searches?
34. Can a source belong to both a leaf legacy family and a legacy-gated module
    closure while the inventory records only one of those truths?
35. Can isolation guard bytes obtained from a pool lease or independently
    fabricated physical-format admission instead of a Store-minted chunk?
36. Can certification rebuild deterministic immutable replay evidence per test
    until the suite times out or leaves orphan processes behind?
37. Can a caller pair an unrelated protected reference with a Store chunk
    because guard scope construction accepts both independently?
38. Can a newly added physical owner tag encode successfully while validation,
    reconstruction, family matching, or range identity silently rejects or
    collapses it?
39. Can a fixture prove its configured envelope using an expectation derived
    from that same configuration rather than executed Store observations?
40. Can F004 compiler evidence attack an adjacent foreground receipt while the
    exact background-pacing basis regains generic or isolation authority?
41. Can a stale focused selector compile all Store targets but execute zero
    scheduler journeys and still be reported as proof?
42. Can renamed generic proof authority reappear beneath scheduler
    `background_pacing/` while every exact deleted identifier stays absent?
43. Can physical isolation import the scheduler directly and implement the
    forbidden policy conversion in the crate that owns the isolation target?
44. Can production consumers migrate successfully while their exact removal
    rows remain open, leaving live inventory and audit history contradictory?
45. Can clean committed Phase 8 source escape formatting because structural QA
    inventories only initially dirty files?
46. Can renamed tiering `io_readiness` or a scheduler import alias reattach
    readiness to blob placement while every deleted public name stays absent?
47. Can class-specific placement evidence be cloned, reconstructed, boxed, or
    carried inline so the API hides authority identity or per-placement cost?
48. Can surviving module documentation teach callers that an admitted
    placement carries scheduler readiness after the type and dependency graph
    delete it?
49. Can one placement-class verifier grow until inline, external, and cold
    authority or denial rules become coupled and mutation-insensitive?
50. Can deleted capacity capability, proof, progression, or blob-admission
    files disappear without receiving a path-bound inventory disposition?
51. Can a non-admitted scheduler outcome acquire an `into_lease` method while
    runtime tests continue matching only its current fields?
52. Can an unpaced compaction basis, one reused lease, or a consumer-local
    constructor enter planning despite the positive scheduler journey passing?
53. Can renamed booleans or a pressure declaration recreate executable
    compaction pacing while every historical identifier remains absent?
54. Can the inventory classifier grow into one mixed branch table where a new
    semantic family silently inherits incomplete matching or review?
55. Can a hostile source mutant change zero bytes because its replacement is
    bound to one platform's line endings and still be reported as evidence?
56. Can policy and execution mutants accumulate in one test until failures no
    longer identify which authority transition regressed?
57. Can a production refactor leave a controlled mutation bound to zero source
    seams while the designated baseline test remains green?
58. Can a move-ownership gate associate one type with a predecessor's derive
    list after the target's own derive is deleted?
59. Can a courtroom source manifest retain a stale hash and prevent execution
    of the lifecycle claims it is cited to prove?
60. Can a compile-fail snapshot reject current sealed constructors while still
    expecting obsolete diagnostics from an earlier API topology?
61. Can certification rename synthetic S.2 counters and receipts as evidence,
    export them publicly, and register them as canonical while never driving
    the real Store or canonical pool?
62. Can a registry preserve the right evidence-family name but point at a
    deleted source path, leaving the real semantic owner unregistered?
63. Can cleanup delete every semantic child while retaining an empty module
    declaration or directory that falsely implies a live future owner?
64. Can a certification wrapper or helper retain legacy record-view authority
    without spelling any lower legacy type name?
65. Can executable Markdown hide a deleted type, or can an intentional
    compile-fail call be misclassified as a live production bypass?
66. Can cleanup delete a current shared performance contract merely because it
    shares a module with obsolete aggregate receipts?
67. Can dead pool-counter translation survive after its only aggregate
    consumer disappears and preserve a direct certification edge?
68. Can a compile-fail document remain green on an obsolete import without
    reaching the current authority boundary named by its prose?
69. Can certification repackage legacy speculation, dirty-state, or entry
    counters as canonical evidence after every real consumer has disappeared?
70. Can deleting a final evidence family leave a doc-comment-only facade or an
    empty semantic directory that still advertises a dead substrate owner?
71. Can certification reconstruct a buffer pool solely to mint scheduler work
    after the scheduler already owns an exact typed producer declaration?
72. Can physical foreground lowering substitute an unrelated locality while
    every constructor proof checks only that some locality survived?
73. Can a cross-scope secure-I/O scenario pass because any violation is
    accepted, even when the backend witness was not the causal contradiction?
74. Can queue execution hide locally minted generic proof authority beneath
    concrete public plan typestates while only background pacing is scanned?
75. Can widening a generic-proof source gate reject legitimate concrete
    platform witnesses because it matches substrings instead of identifiers?
76. Can a completed removal row satisfy human review with a conceptual owner
    phrase while no mechanical link reaches the live replacement source?
77. Can a fixed-posture foreground API hide an internal constructor that
    recombines durability and writeback into a forbidden pair?
78. Can queue admission validate one request and then lose that phase proof by
    flattening its facts into a conventional multi-argument constructor?
79. Can an ungated file beside a legacy module or a consumer of an already
    deleted authority definition survive because neither carries the original
    feature gate?
80. Can a source-bound authority gate require a proof type that was correctly
    deleted, making the real typestate fail while a redundant authority layer
    appears necessary to satisfy the oracle?
81. Can a forgeable exact-count residency snapshot survive by dropping its
    `S2` prefix after every current consumer disappears?
82. Can a blob consumer retain the exact scheduler lease internally while its
    public execution admission or paced intent remains cloneable or reusable
    after move?
83. Can test support preserve unrelated authority, policy, budget, and security
    fixtures behind a catch-all filename or wildcard import even when
    production topology is semantically split?
84. Can deletion remove every file in a predecessor family while leaving its
    empty directory topology on disk, falsely advertising a live owner or
    future insertion point?
85. Can a row say `delete` while its path survives after shedding the exact
    vocabulary that would otherwise rediscover it?
86. Can a migrated allocation test compare a counter to the admitted byte
    width when the counter semantically reports the number of bounded windows?
87. Can a formal policy state survive honestly while closeout still forces
    production to counterfeit the deleted receipt that once emitted it?
88. Can a test force a borrow to end with `drop` instead of making the
    authority lifetime visible in its lexical structure?
89. Can a rare retry or inspection payload silently inflate every ordinary
    writeback result because no compile-time layout bound protects the hot
    variant?
90. Can a final manifest retain the right path and status counts while its
    digest no longer reproduces from the documented exact-byte encoding?

Any credible surviving defect reopens the affected guarantees and this ledger's
completeness claim.
