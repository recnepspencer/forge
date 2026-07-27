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
`2de25589602670ebbb6d5974aa43a67f48d82d4b` with a clean worktree on branch
`worth-store`.

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
| `C6-P8-P08` | Recovery, Scrub, Maintenance, Verification, and Blob allocation authority is Store-minted, generation-bound, move-owned, exact-scope typed, and incapable of exposing or spending the lower grant. | Positive compile specimens, cross-scope/forgery/grant-extraction negatives, real Store admission/pressure/release journey. | `OPEN` |
| `C6-P8-P09` | Certification and test fixtures prove the real Store composition or narrow canonical pool laws; no fixture constructs deleted physical truth or self-certifies from copied counters. | Complete fixture/evidence trace, real Store roots and observations, independent oracles, deletion of redundant model evidence. | `OPEN` |
| `C6-P8-P10` | Mathematical or policy tests retained from S.2 have independent value, make no production-authority claim, and add unique evidence not already owned by canonical pool or Store tests. | QA-tests proof-obligation audit, mutation sensitivity, duplicate-test review, owner placement review. | `OPEN` |
| `C6-P8-P11` | Store-to-certification dependency direction is one-way: Store owns runtime truth and certification depends on Store; no dependency cycle or public compatibility re-export remains. | Cargo metadata/tree, facade inspection, compile tests, dependency-cycle mutant. | `OPEN` |
| `C6-P8-P12` | Every dependency, feature branch, module export, registry row, test selector, and fixture capability made dead by the cutover is removed. | Warnings-denied builds, metadata diff, dead-reference searches, suite/catalog execution. | `OPEN` |
| `C6-P8-P13` | The workspace builds and its affected owner, Store, successor, certification, and test-support suites pass with no deleted feature available. | Focused tests, all-target/all-feature checks, workspace test lane, boundary-check, agent-context. | `OPEN` |
| `C6-P8-P14` | Controlled reintroduction of a legacy feature, direct pool edge, deleted authority identifier, snapshot fixture, or legacy view fails the nearest mechanical gate. | Individually localized mutants for every substitution class. | `OPEN` |
| `C6-P8-P15` | The resulting directory structure, facade placement, names, file sizes, and function composition preserve current and committed successor responsibilities. | Full dirty inventory, Rust function scrutiny, 400-line gate, composition and domain-topology review. | `OPEN` |

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
| Ledger completeness and final source truth | `L01`, `L02` |
| Composition, topology, and test quality | `P09`, `P10`, `P15` |

## Risk Map

| Risk | Earliest honest boundary | Required detector |
| --- | --- | --- |
| Disabled legacy feature survives metadata | Cargo feature declaration | metadata feature-key gate and mutant |
| Certification exemption hides a second physical world | dependency/source classifier | exact canonical-owner allowlist |
| Legacy graph is renamed instead of deleted | semantic source inventory | module-family and authority-shape gate |
| Generic allocation permits cross-scope use | public type boundary | exact concrete scope types and compile failure |
| Store scope evidence leaks a lower grant | visibility boundary | grant-extraction negative |
| View migration copies or owns bytes | adapter API and counters | borrow-check UI plus copy counters |
| Fixture opens no real Store boundary | test world construction | fixture trace and real-root observation |
| Legacy test is copied into canonical topology | proof-obligation ownership | QA-tests uniqueness and mutation review |
| Store depends on its certification consumer | Cargo graph | one-way dependency gate |
| Deletion leaves registry or selector sediment | build/catalog boundary | warnings-denied compile and exact selector runs |
| Search excludes the very consumer it must find | inventory generator | hostile consumer in every former exception class |
| Cleanup claim rests on an earlier source state | evidence freeze | final exact source fingerprint |

## Finding History

### `C6-P8-F001` - Phase 1 inventory exempts certification consumers

- status: `OPEN`
- affected guarantees: `L02`, `P01`, `P07`, `P09`, `P14`
- evidence: the Phase 7 classifier names certification crates as unrestricted
  pool owners, while live inspection finds 37 `worth-store-certification`
  files and 7 `worth-store-physical-certification` files importing the pool,
  including parallel-model evidence.
- required correction: replace broad certification disposition with an exact
  canonical physical-owner allowlist, add every discovered consumer to the
  generated removal truth, and migrate or delete it.
- closing proof: controlled direct-pool consumers in both certification classes
  fail the final gate, while real Store-bound certification passes.

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

Any credible surviving defect reopens the affected guarantees and this ledger's
completeness claim.
