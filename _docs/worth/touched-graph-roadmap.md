# Worth Touched Graph Roadmap

> **Status:** Draft
>
> **Purpose:** split the Worth touched graph authority program into
> milestone-sized implementation gates before `Milestone 7.5` resumes broad
> planar boolean work.

## Goal

Make touched graph authority the mandatory Worth spine for graph-affecting
topology and spatial work.

The original touched graph authority gate proved the right product ladder, but
Phase 3 showed that each authority transition is milestone-sized. This roadmap
keeps the shared architecture in one place while making each transition small
enough to plan, implement, QA, commit, and close honestly.

## Governing Summaries

- `MENTALITY.md`: protects adversarial-constraint-first design. This roadmap
  exists because local boolean, NURBS, extrusion, and fillet work will touch
  small graph regions inside large topology, and global rediscovery will fail.
- `arch_laws.md`: protects proof-bearing phase transitions. Each milestone must
  produce the exact proof product the next milestone consumes; weaker products
  may not promote themselves into authority.
- `composition_laws.md`: protects responsibility boundaries. Touched graph
  request, admission, closure, validation, invalidation, evidence, replay,
  conflict, cache, API, and closeout must not collapse into one broad helper.
- `domain_structure_laws.md`: protects visible ownership. The tree must show
  where topology truth, spatial evidence, Query obligation proof, and kernel
  composition each live.
- `perf_laws.md`: protects semantic-delta-bounded execution. Work breadth must
  scale with touched graph breadth, not global topology size, broad evidence
  scans, or repeated selector rediscovery.
- `worth_roadmap.md`: protects Worth from duplicated authority across topology,
  geometry, validation, naming, orchestration, interaction, and diagnostics.

## Adversarial Constraint

Worth must survive long boolean and future curved-operation chains where every
operation touches a small local graph region while the model contains a large
body of unrelated topology, geometry evidence, derived state, replay history,
cache entries, and diagnostics.

If an operator can commit, replay, invalidate, validate, find evidence, prove
independence, reuse cache, or explain itself from static/global packs, raw rows,
operator-family convention, broad scans, or compatibility adapters instead of a
typed touched graph product, the roadmap has failed.

## Product Decision Lock

- This roadmap replaces the old "one gate with many runner phases" execution
  shape. Each former phase is now a milestone-sized gate.
- The parent architecture remains shared: product ladders, authority matrix,
  cross-crate boundary invariants, and delete-first pressure apply to every
  milestone.
- `forge-query` owns graph touch descriptors, selector matching, obligation
  selection, support posture, execution proof, and Query-gap posture.
- `worth-topo` owns topology truth, topology touched basis construction,
  topology closure expansion, validator derivation, and topology legality
  diagnostics.
- `worth-spatial` owns spatial evidence, boolean stage receipts, spatial touch
  authority, evidence lookup products, and geometry evidence diagnostics.
- `worth-kernel` owns workload composition, public closeout pressure, and
  cross-crate proof that no lower-authority substitute can pass.
- Compatibility adapters, feature-gated bridges, public raw constructors,
  copied-row admission, and type-name guards are hostile until proven
  non-authoritative. The default action is deletion and caller cleanup.

## Why This Needs Its Own Roadmap

Touched graph authority is not one refactor. It is a sequence of authority
transitions:

- inventory old authority
- define topology touched basis
- force operators to produce it
- define spatial touch authority
- feed Query selection
- inventory local Worth graph-read access folklore after Query `9.10`
- express covered Worth graph reads as touched-authority-backed Query
  declarations
- adopt Query access plans and receipts as the only covered graph-read proof
- derive validators, invalidation, evidence lookup, replay, conflict, cache,
  public proof, and diagnostics

Trying to close all of that inside one runner state makes the automation slow,
history-heavy, and prone to discovering milestone-sized gaps late. This roadmap
turns each authority transition into its own coherent closeout unit.

## Dependency Chain

The intended chain is:

`Worth Query Graph Authority Hardening Gate` ->
`Touched Graph Milestone 1` ->
`Touched Graph Milestone 2` ->
`Touched Graph Milestone 3` ->
`Touched Graph Milestone 4` ->
`Touched Graph Milestone 5` ->
`Touched Graph Milestone 6` ->
`Touched Graph Milestone 7` ->
`Touched Graph Milestone 8` ->
`Touched Graph Milestone 9` ->
`Touched Graph Milestone 10` ->
`Touched Graph Milestone 11` ->
`Touched Graph Milestone 12` ->
`Touched Graph Milestone 13` ->
`Touched Graph Milestone 14` ->
`Touched Graph Milestone 15` ->
`Touched Graph Milestone 16` ->
`Milestone 7.5`

Each milestone should have its own implementation spec or runner state, its own
QA closeout, and its own commit boundary.

## Shared Product Ladders

Topology branch:

1. raw topology operator intent
2. admitted topology operator intent
3. declared topology touched graph basis
4. expanded topology touched graph closure
5. selected Query graph obligations
6. covered graph-read access inventory and deletion ledger
7. touched-authority-backed Query graph-read declarations
8. adopted Query graph-read access plans and receipts
9. selected topology validators and relational invariants
10. derived invalidation plan
11. replay scope
12. conflict and independence proof
13. cache and equivalence invalidation proof
14. undo and transaction scope
15. executed topology touched graph receipt
16. public topology authority proof and diagnostics

Spatial evidence branch:

1. sealed boolean or spatial evidence receipt
2. spatial touch authority product
3. Query touch descriptor or Query adoption proof
4. covered spatial graph-read access inventory
5. touched-authority-backed Query graph-read declaration
6. Query graph read access plan where evidence reads are covered graph reads
7. spatial evidence lookup key and lookup product
8. spatial replay scope
9. spatial cache or equivalence proof
10. public spatial authority proof and diagnostics

The branches may share schema vocabulary, digest recipes, counters, and Query
descriptor language. They may not share constructors, admission authority, or
proof ownership.

## Authority Product Matrix

| Milestone | Input Authority | Output Authority | Owner | Forbidden Substitute | Deletion Target |
| --- | --- | --- | --- | --- | --- |
| 1 | old static/global authority surfaces | typed inventory and deletion ledger | `worth-kernel` closeout pressure | prose, grep-only coverage, keep rows | unclassified static/global paths |
| 2 | topology vocabulary and admitted topology meaning | sealed topology touched graph basis vocabulary, digest, counters | `worth-topo` with shared `worth-schema` vocabulary | raw ids, strings, copied Query rows, mutation records, spatial receipts | schema projection/admission traits, type-name guards, topology geometry-only bridge |
| 3 | admitted topology operator intent | declared topology touched graph basis before Query/local execution | `worth-topo` | operator validator arrays, mutation records as proof, local touch languages | bypass fronts and operator-local validator declarations |
| 4 | sealed boolean or spatial evidence receipt | spatial touch authority, Query descriptor/adoption proof, evidence lookup key | `worth-spatial` with `worth-schema` and `forge-query` | raw evidence rows, broad stage scans, copied receipt fields, public schema constructors | public projection/admission experiments, topology geometry-only lowering |
| 5 | topology basis or spatial Query descriptor | selected Query graph obligations | `forge-query` | Worth-local selector copies, in-memory adoption as execution proof | broad selector adapters and local selector forks |
| 6 | selected Query graph obligations plus local Worth graph-read surfaces | graph-read access inventory and deletion ledger | `worth-kernel` closeout pressure with `forge-query` Consumer Kit | caller-owned N+1 loops, ad hoc adjacency maps, hidden broad scans, local support rows | unclassified graph-read folklore and broad read adapters |
| 7 | touched authority products plus graph-read inventory | covered Query graph-read declarations and required access rows | `worth-topo`, `worth-spatial`, `worth-kernel` through `forge-query` | declaration wrappers around local traversal, copied access rows, strings as support | local declaration shims and access requirement mirrors |
| 8 | covered graph-read declarations and Query support rows | admitted Query graph-read access plans, postures, counters, and receipts | `forge-query` with Worth reference consumers | caller-owned execution proof, unbounded automatic indexes, fabricated receipts, helper loops | migrated local loops, caches, fabricated receipts, broad read helpers |
| 9 | expanded topology closure | selected topology validators and relational invariants | `worth-topo` | global validator packs, expectation arrays | static invariant packs and operator-local global validation |
| 10 | expanded topology closure | derived invalidation and dirty propagation plan | `worth-topo` | whole-view rebuild by default, hidden projection expansion | broad derived rebuild paths |
| 11 | topology basis or spatial touch authority | evidence lookup plan and lookup product | `worth-spatial` | raw evidence vectors, broad receipt scans | raw public evidence scans |
| 12 | topology/spatial proof products | replay, undo, and transaction scope | owning crate plus `worth-kernel` | global replay, re-query rollback authority | replay/undo paths without proof scope |
| 13 | touched closures and spatial authority products | conflict, independence, batch-admission proof | `worth-topo`, `worth-spatial`, `forge-query` | speculative lock-first conflict discovery | batch shortcuts without structural proof |
| 14 | touched proof plus source authority digest | cache/equivalence proof | owning product crate | pointer identity, row count, operator family | cache keys without touched authority |
| 15 | executed proof products | public read-only authority proof and diagnostics | public facades | raw constructors, support pins, local ceremony | public escape hatches |
| 16 | certified milestone products | cross-crate closeout matrix | `worth-kernel` closeout pressure | prose claims, untested residue | uncapped adapters and stale doc claims |

## Milestone 1: Inventory And Hard Break Plan

Freeze every static/global validator, invariant, invalidation, replay, evidence,
conflict, cache, undo, and diagnostic path that acts without touched graph
authority.

Closes:
- typed inventory rows by category and owner
- deletion action and removal trigger for old authority
- facade/export audit for delete and collapse rows
- line-cap and composition pressure for touched code

Done when:
- no old authority surface is unclassified
- no old static/global surface can be marked `keep`
- delete/collapse rows fail certification if still exported

## Milestone 2: Topology Touched Graph Basis Types

Freeze the sealed topology touched graph basis and shared schema vocabulary that
topology operators use to state what graph meaning they touched.

Closes:
- topology entities, relations, relation kinds, aspects, scopes, lifecycle
  posture, operating world, digest, and counters
- shared `worth-schema` vocabulary without public admission authority
- compile-fail denial for raw ids, strings, copied descriptors, mutation
  records, raw rows, and copied spatial receipts

Done when:
- topology touched basis is sealed and digest-stable
- `worth-topo` does not consume spatial geometry receipts as topology authority
- public projection/admission traits and type-name guards are deleted or capped

## Milestone 3: Topology Operator Intent To Touched Basis

Freeze the rule that every topology operator produces a declared touched graph
basis before Query lowering or local execution.

Closes:
- admitted topology intent to declared touched-basis proof
- Query workflow and graph-compose helper fronts that bypass basis proof
- omission denial before Query write execution
- public facade proof/status without raw constructors

Done when:
- no production topology operator or helper writes Query graph mutations without
  declared touched-basis proof
- mutation records may feed the basis but cannot stand in as proof
- helper fronts that bypass basis construction are deleted or certification-only

## Milestone 4: Spatial Geometry Evidence Touch Authority

Spec: [`touched-graph-milestone-4-spatial-geometry-evidence-touch-authority.md`](./touched-graph-milestone-4-spatial-geometry-evidence-touch-authority.md)

Freeze spatial and boolean geometry evidence as its own sealed touch authority,
without laundering geometry evidence through topology basis construction.

Closes:
- sealed BooleanEvidenceReceipt-backed spatial touch authority
- Query descriptor/adoption proof from spatial evidence when needed
- evidence lookup identity keyed by spatial touch authority
- product separation between spatial lookup products and Query descriptors

Done when:
- external callers cannot fake spatial receipts or implement admission traits
- production spatial evidence does not route through `worth-topo`
- old projection/admission/type-name bridges are deleted or capped
- public closeout surfaces are
  `worth_spatial::facade::workload_vocabulary` for spatial touch authority and
  `worth_spatial::facade::query_adoption` for Query adoption status
- Query adoption counters are read through
  `current_spatial_query_consumer_kit_adoption_status`
- Consumer Kit residue count: 1 capped row for the older spatial support
  projection facade
- Milestone 4 does not close Milestone 5 Query obligation selection
- Milestone 4 does not close Milestones 6 through 8 graph-read access work

## Milestone 5: Query Obligation Selection From Touched Basis

Spec: [`touched-graph-milestone-5-query-obligation-selection-migration.md`](./touched-graph-milestone-5-query-obligation-selection-migration.md)

Freeze Query obligation selection as a consumer of topology basis translations
and spatial Query descriptors.

Closes:
- strangler migration inventory for every Worth graph-obligation selection,
  adoption, support, selected-count, and local ceremony surface
- parallel Query-owned selection substrate built beside current Worth-local
  paths before public cutover
- primitive construction as the first migrated vertical lane, with parity and
  stronger denial proof
- spatial touch authority to Query obligation selection through
  `SpatialEvidenceQueryTouchDescriptor`
- Query-owned selector semantics for touched descriptors
- counters for attempted buckets, matches, deduplication, rejection, selection
- Query-gap rows for missing selector expressiveness

Done when:
- Worth does not fork Query selector matching
- the old selector path is no longer a future dependency after each migrated
  vertical lane reaches parity
- broad collection-only or lifecycle-only selector use is capped residue
- selection breadth scales with touched descriptor breadth
- local selector tables, local graph walks, copied counts, local support rows,
  and in-memory adoption-as-execution-proof are deleted or mechanically denied
- Phase 8 closeout exposes a selected-obligation Milestone 6 seed carrying
  nonempty authority digests, touch descriptor digests, selected registration
  digests, execution proof digests, adoption manifest digests, residue manifest
  digests, and selector precision report digests
- Phase 8 closeout records exactly 1 capped broad selector residue row and
  exactly 1 Query selector expressiveness gap row for the spatial descriptor
  lane, while topology touched basis selection remains touched-descriptor
  bounded with 0 broad selector residue and 0 Query selector gap rows
- Phase 8 closeout explicitly does not claim graph-read access planning,
  validator derivation, invalidation, replay, conflict, cache, or diagnostics

## Milestone 6: Worth Graph-Read Access Inventory And Hard Break

Freeze every Worth graph-read access surface that still teaches local graph
folklore after Query `9.10` exists.

Closes:
- typed inventory rows for covered topology, spatial, kernel, and test graph
  reads that use relation loops, per-result neighbor lookup, ad hoc adjacency
  maps, local graph caches, broad boolean scans, local support rows, or
  fabricated receipts
- deletion action, owner, cap, and removal trigger for every local graph-read
  path that Query can express or should be taught to express
- distinction between real touched authority inputs, Query graph touch
  obligations, graph-read declarations, access requirement rows, and execution
  receipts
- Consumer Kit-backed bypass audit rows rather than Worth-local source greps or
  support folklore

Done when:
- no covered Worth graph-read access surface is unclassified
- local graph-read residue is capped by owner, count, and concrete Query access
  capability trigger
- broad boolean and dense frontier reads are named as Query access-plan work,
  required-capability work, or deleted local folklore
- no local support row, helper wrapper, or fixture receipt can be mistaken for
  Query access authority

## Milestone 7: Touched Authority To Query Access Declarations

Freeze the translation from touched graph authority products into real Query
graph-read declarations, read families, access requirements, and required
capability rows.

Closes:
- topology closure and spatial touch authority lower into covered Query
  graph-read declarations before any covered read executes
- domain graph-read operations needed by Worth declare operation resolution,
  access shape, selectivity posture, requirement rows, basis, policy, tenant,
  and relationship-proof posture through Query-owned vocabulary
- missing Query expressiveness becomes typed Query-gap or required-capability
  posture, not a local Worth traversal adapter
- declaration construction is sealed so raw ids, strings, mutation rows, local
  support labels, and copied access rows cannot promote into executable read
  authority

Done when:
- every covered touched graph read has either a Query declaration path or a
  typed gap with owner, cap, and deletion trigger
- access requirements are derived by Query from admitted declarations, not
  hand-written in Worth
- broad boolean graph predicates and dense frontier reads reach Query posture
  admission instead of hidden local traversal
- declaration tests prove touched authority is the input and admitted Query
  access artifacts are the next proof product

## Milestone 8: Worth Graph-Read Access Plan Adoption

Freeze execution of covered Worth graph reads through admitted Query access
plans, typed postures, counters, and receipts.

Closes:
- covered topology, spatial, and kernel graph reads execute only through
  admitted Query graph-read access plans or typed Query postures
- access posture rows distinguish inline indexed, bounded ephemeral, admitted
  paged streaming, persistent-index-required, async-materialization-required,
  store-backed-required, access-capability-registration-required, and denial
  cases
- exact counters for candidate roots, touched nodes, touched edges, frontier
  width, visited/dedup breadth, resident bytes, page count, fallback count, and
  no-caller-owned graph work
- deletion of local loops, adjacency maps, broad read helpers, fabricated
  receipts, and compatibility wrappers whose Query replacements exist

Done when:
- covered Worth graph reads consume admitted Query access plans or fail with a
  typed Query denial/required posture before expensive edge traversal starts
- access-plan receipts are available to later validator, invalidation, evidence,
  replay, conflict, cache, public proof, and diagnostic milestones
- no covered lane performs caller-owned N+1 work, hidden broad scans, unbounded
  background indexing, or fabricated execution proof
- deletion manifests and closeout tests prove migrated local folklore is gone

## Milestone 9: Validator And Invariant Derivation

Freeze topology validator and invariant selection as a derived consequence of
touched graph closure.

Closes:
- validator family predicates over touched closure
- deletion or collapse of static milestone-one invariant packs
- whole-view validation restricted to certification or named residue

Done when:
- operator-local legality consumes touched graph closure
- global validator packs cannot satisfy ordinary operator closeout
- adding a validator family without a touched predicate fails certification

## Milestone 10: Derived Invalidation And Dirty Propagation

Freeze derived topology invalidation as a plan derived from touched graph
closure.

Closes:
- direct versus closure-expanded touch propagation
- invalidation contracts for derived products
- counters proving invalidation breadth equals closure breadth

Done when:
- ordinary local operators do not rebuild all derived topology
- projection rebuild code does not hide dirty expansion
- derived products without invalidation contracts cannot be consumed

## Milestone 11: Evidence Lookup And Boolean Stage Indexing

Freeze spatial and boolean evidence lookup around spatial touch authority and
related topology touched graph identity.

Closes:
- spatial touch authority keyed lookup products
- stage and receipt digest lookup identity
- deletion of raw evidence vectors and broad stage scans

Done when:
- wrong spatial touch digest or mismatched topology digest cannot satisfy lookup
- raw evidence vectors cannot act as public lookup products
- Query descriptors and evidence lookup products cannot satisfy each other

## Milestone 12: Replay, Undo, And Transaction Scope

Freeze replay and undo boundaries as consumers of touched graph proof.

Closes:
- replay scope from touched graph closure
- minimal reversible graph patch or named undo residue
- transaction boundaries exposing touched graph digest, validators,
  invalidation, and evidence receipts

Done when:
- replay does not re-run global topology to prove local edits
- rollback does not re-query authority already captured by proof products
- hidden mutation outside undo scope fails closeout

## Milestone 13: Conflict, Independence, And Batch Admission

Freeze independence proof and conflict denial as touched graph products.

Closes:
- disjointness and compatible aspect-level overlap proof
- conflict classes for entity, relation, aspect, closure, evidence, validators
- batch admission before execution

Done when:
- conflict detection is structural, not speculative lock-first execution
- disjoint operations batch-admit with separate proof products
- closure conflicts deny or serialize with named reasons

## Milestone 14: Cache, Equivalence, And Reuse Contracts

Freeze reuse as a touched graph equivalence claim.

Closes:
- cache keys over touched digest, source authority digest, stage, equivalence
  policy, validator set, and evidence set
- separation of geometry-only and topology-touch equivalence
- public denial of cache proof forgery

Done when:
- operator family, pointer identity, row count, and filename provenance cannot
  justify reuse
- benign ordering noise preserves equivalence identity
- different touched closures deny reuse

## Milestone 15: Public API, Diagnostics, And Explainers

Freeze public surfaces that expose touched graph proof without leaking
constructors or internals.

Closes:
- read-only public proof/status APIs
- selected obligation, validator, invalidation, evidence, conflict, and denial
  diagnostics
- compile-fail fences against raw constructors and local ceremony

Done when:
- public callers can inspect proof/status but cannot construct authority
- diagnostics localize rejection to exact touched graph facts or Query gaps
- public APIs do not expose support pins, raw rows, or proof helpers

## Milestone 16: Cross-Crate Closeout And 7.5 Readiness

Close the touched graph program only when every category consumes touched graph
authority or is explicitly deleted, capped residue, or Query-gap.

Closes:
- cross-category closeout matrix
- hard-break reintroduction tests
- line-cap and composition proof
- documentation consistency with certified counts

Done when:
- `Milestone 7.5` can consume touched graph proof for overlap extraction
- no slow-conversion adapter remains in production without cap and removal
  trigger
- certified closeout counts match the roadmap and implementation state

## Runner Policy

Each touched graph milestone should have its own runner state file and closeout
boundary. A runner may finish the current `Milestone 3` in the old state file,
but it should stop before starting `Milestone 4` so the remaining work can be
split into standalone milestone specs.

Runner escalation policy:
- pause and report after 10 attempts in one milestone
- pause and report on `blocked`
- pause and report when the same finding id reopens twice
- do not continue past a milestone boundary without human confirmation

## Final Acceptance

The touched graph roadmap is complete only when:

- every graph-affecting topology operator and boolean/spatial stage produces
  the correct touched authority product before execution
- Query obligation selection consumes touched authority products
- covered Worth graph reads have local folklore inventoried, lower from
  touched authority into Query graph-read declarations, and execute only
  through admitted Query access plans or typed Query postures
- validators, invariants, invalidation, evidence lookup, replay, undo,
  conflict, cache, public proof, and diagnostics derive from those products
- old static/global paths are deleted or mechanically sealed as residue
- `Milestone 7.5` starts from touched graph authority rather than local
  validator, evidence, dirty-region, replay, or diagnostic rules
