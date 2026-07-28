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

The final source fingerprint is `OPEN` until implementation and all corrections
have stopped.

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
| `C6-P8-L01` | The final source state is complete and reproducible across tracked, deleted, renamed, and untracked files, excluding only this ledger from its own fingerprint. | Exact-byte manifest, independent hashes, status counts, final evidence rebound. | `OPEN` |
| `C6-P8-L02` | The ledger covers every Phase 8 must-ship, preserve, proof, cleanup, API, dependency, lifecycle, test, and causally necessary intent guarantee. | Requirement map, risk map, finding history, evidence index, and surviving-defect attack. | `OPEN` |
| `C6-P8-P01` | Every Phase 1 Phase-8 consumer and every newly discoverable parallel-world consumer has an exact migration or deletion disposition; no broad certification exemption hides a consumer. | Generated source/manifest inventory, Cargo metadata, exact removal-ledger reconciliation, hostile unclassified/stale-row mutants. | `OPEN` |
| `C6-P8-P02` | `legacy-s2-models` and `legacy-certification-models` do not exist as feature declarations, optional branches, dependency features, or activated metadata edges anywhere in the live Store workspace. | Manifest-key inspection, Cargo metadata and tree inspection, repository source search, declaration/edge mutants. | `OPEN` |
| `C6-P8-P03` | Snapshot-derived residency admission, `S2PhysicalResidencyEntry`, `S2PhysicalEntryFacts`, and every equivalent count-snapshot authority graph are deleted without a renamed replacement. | Source/module absence, public API inspection, compile failure on controlled reintroduction, canonical Store admission tests. | `OPEN` |
| `C6-P8-P04` | `ResidentFrameTable` and its request, lease, dirty, eviction, report, and capacity graph are deleted; the canonical `PhysicalResidencyPool` is the only resident-frame truth. | Complete module and symbol absence, canonical pool owner tests, Store-bound journeys, direct-owner graph. | `OPEN` |
| `C6-P8-P05` | Legacy zero-copy, bounded-copy, materialization-profile, pinned-view, and owning-read-buffer graphs are deleted; integrity and isolation consume the Store borrowed chunk contract. | Public API/source absence, positive Store-view consumers, lifetime and construction compile failures, bounded-copy runtime evidence. | `OPEN` |
| `C6-P8-P06` | Isolated S.2 background, speculative, queue, allocation-envelope, and evidence-source models are deleted; effectful speculation remains only on the canonical Store runtime. | Module/source absence, canonical speculation tests and counters, no local worker/queue gate, controlled legacy-model mutant. | `OPEN` |
| `C6-P8-P07` | Direct buffer-pool dependency and source access is limited to exact canonical physical owners; certification, successor domains, and test support cannot import it. | Cargo metadata allowlist, source-import inventory, dependency/source mutants, boundary checker. | `OPEN` |
| `C6-P8-P08` | Recovery, Scrub, Maintenance, Verification, and Blob allocation authority is Store-minted, runtime-borrow-bound, generation-bound, move-owned, exact-scope typed, and incapable of exposing or spending the lower grant. The issuing serving runtime cannot close or move while any successor authority remains live. | Positive compile specimens; cross-scope, forgery, grant-extraction, clone, move-after-use, runtime-escape, and close-while-live negatives; real Store admission/pressure/release/close journey. | `OPEN` |
| `C6-P8-P09` | Certification and test fixtures prove the real Store composition or narrow canonical pool laws; no fixture constructs deleted physical truth or self-certifies from copied counters. | Complete fixture/evidence trace, real Store roots and observations, independent oracles, deletion of redundant model evidence. | `OPEN` |
| `C6-P8-P10` | Mathematical or policy tests retained from S.2 have independent value, make no production-authority claim, and add unique evidence not already owned by canonical pool or Store tests. | QA-tests proof-obligation audit, mutation sensitivity, duplicate-test review, owner placement review. | `OPEN` |
| `C6-P8-P11` | Store-to-successor and Store-to-certification dependency direction is one-way: Store owns runtime truth; successors and certification consume its facade; no normal dependency cycle, peer-owned composition adapter, or public compatibility re-export remains. | Cargo metadata/tree, strongly connected component inspection, facade review, compile tests, dependency-cycle mutant. | `OPEN` |
| `C6-P8-P12` | Every dependency, feature branch, module export, registry row, test selector, and fixture capability made dead by the cutover is removed. | Warnings-denied builds, metadata diff, dead-reference searches, suite/catalog execution. | `OPEN` |
| `C6-P8-P13` | The workspace builds and its affected owner, Store, successor, certification, and test-support suites pass with no deleted feature available. | Focused tests, all-target/all-feature checks, workspace test lane, boundary-check, agent-context. | `OPEN` |
| `C6-P8-P14` | Controlled reintroduction of a legacy feature, direct pool edge, deleted authority identifier, snapshot fixture, or legacy view fails the nearest mechanical gate. | Individually localized mutants for every substitution class. | `OPEN` |
| `C6-P8-P15` | The resulting directory structure, facade placement, names, file sizes, and function composition preserve current and committed successor responsibilities. | Full dirty inventory, Rust function scrutiny, 400-line gate, composition and domain-topology review. | `OPEN` |
| `C6-P8-P16` | Scheduler-native policy cannot become physical-isolation or Store authority: the scheduler has no physical-isolation/recovery dependency, copied readiness admission, generic `AuthorityMarker`, or counter-derived execution capability. Cross-domain physical composition occurs only at Store. | Dependency/source absence, public API inspection, policy-versus-authority type review, compile mutants, Store scheduler journeys. | `OPEN` |
| `C6-P8-P17` | Scheduler execution capacity is concrete, move-owned, and single-consumption: a background lease cannot be cloned or copied, non-admitted outcomes and observations mint no lease, and a consuming domain cannot self-admit or lower the same admission twice. | Public API inspection, move-after-use and duplicate-lowering compile failures, positive queue-lowering specimen, consumer-construction mutant. | `OPEN` |
| `C6-P8-P18` | Tiering and blob placement consume only class-relevant physical authority: inline and external placement require no cold-tier or scheduler readiness, cold placement consumes the exact cold posture, and layout projection cannot promote scheduler counters into placement truth. | Public API/source inspection, class-specific positive specimens, cold-scope negative, copied-readiness absence and reintroduction mutant. | `OPEN` |

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
| No cross-domain dependency cycle or copied scheduler authority | `P11`, `P16` |
| Move-owned scheduler execution capacity with no consumer self-admission | `P16`, `P17` |
| Placement consumes only class-relevant physical authority | `P06`, `P11`, `P16`, `P18` |
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
| View migration copies or owns bytes | adapter API and counters | borrow-check UI plus copy counters |
| Fixture opens no real Store boundary | test world construction | fixture trace and real-root observation |
| Legacy test is copied into canonical topology | proof-obligation ownership | QA-tests uniqueness and mutation review |
| Store depends on its certification consumer | Cargo graph | one-way dependency gate |
| Scheduler copies physical-isolation counters into self-minted authority | cross-domain composition boundary | dependency absence, concrete public types, and authority-source mutant |
| Scheduler capacity is copied or lowered twice | scheduler-to-queue type boundary | move-only lease and move-after-use compile failure |
| Blob compaction self-admits or discards scheduler admission into booleans | consumer handoff | sealed scheduler-derived pacing admission and construction mutant |
| Inline/external blob placement requires irrelevant cold readiness | placement intent type boundary | class-specific intent variants and cold-only scope validation |
| Deletion leaves registry or selector sediment | build/catalog boundary | warnings-denied compile and exact selector runs |
| Search excludes the very consumer it must find | inventory generator | hostile consumer in every former exception class |
| Cleanup claim rests on an earlier source state | evidence freeze | final exact source fingerprint |

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

- status: `OPEN`
- affected guarantees: `L02`, `P08`, `P11`, `P12`, `P13`, `P16`
- evidence: the normal graph contains
  `worth-store -> worth-store-io-scheduler ->
  worth-store-physical-isolation -> worth-store-recovery-physics`; therefore
  Recovery cannot consume Store-minted allocation authority without closing a
  Cargo dependency cycle.
- required correction: remove scheduler-owned physical-isolation composition
  and retain only scheduler-native policy/execution contracts. After that
  cutover, successor crates may depend one-way on the Store facade.
- closing proof: metadata has no scheduler-to-isolation/recovery edge; a
  simulated or controlled reintroduction forms the rejected cycle; successor
  packages compile against Store exact types.

### `C6-P8-F003` - Exact successor allocation authority can escape its runtime

- status: `OPEN`
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

### `C6-P8-F004` - Scheduler projects copied isolation counters into authority

- status: `OPEN`
- affected guarantees: `L02`, `P06`, `P11`, `P16`
- evidence: `IoSchedulerIsolationAdmission` copies physical-isolation
  assumptions and counters; background pacing then mints a generic
  `AuthorityMarker` witness from copied freshness state. Store consumes neither
  this admission nor the generic foreground path that requires it.
- required correction: delete copied readiness/capability surfaces and generic
  marker authority; retain independently valuable scheduler resource policy
  only as explicitly non-authoritative policy data.
- closing proof: source/API/dependency absence, policy-to-authority compile
  failure, and unchanged real Store physical-instance scheduler journeys.

### `C6-P8-F005` - Tier placement promotes irrelevant scheduler readiness

- status: `OPEN`
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

### `C6-P8-F006` - Background capacity is duplicable and blob compaction self-admits

- status: `OPEN`
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

Any credible surviving defect reopens the affected guarantees and this ledger's
completeness claim.
