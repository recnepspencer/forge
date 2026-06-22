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
6. admitted Query graph read access plan
7. selected topology validators and relational invariants
8. derived invalidation plan
9. replay scope
10. conflict and independence proof
11. cache and equivalence invalidation proof
12. undo and transaction scope
13. executed topology touched graph receipt
14. public topology authority proof
15. diagnostics and explanation surface

Spatial evidence branch:

1. sealed boolean or spatial evidence receipt
2. spatial touch authority product
3. Query touch descriptor or Query adoption proof
4. Query graph read access plan where evidence reads are covered graph reads
5. spatial evidence lookup key and lookup product
6. spatial replay scope
7. spatial cache or equivalence proof
8. public spatial authority proof
9. diagnostics and explanation surface

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
| 6 | selected Query graph obligations plus covered Worth graph-read declarations | admitted Query graph read access plans, postures, counters, and receipts | `forge-query` with Worth reference consumers | caller-owned N+1 loops, ad hoc adjacency maps, unbounded automatic indexes, hidden broad scans | Worth-local graph-read folklore and broad read adapters |
| 7 | expanded topology closure | selected topology validators and relational invariants | `worth-topo` | global validator packs, expectation arrays | static invariant packs and operator-local global validation |
| 8 | expanded topology closure | derived invalidation and dirty propagation plan | `worth-topo` | whole-view rebuild by default, hidden projection expansion | broad derived rebuild paths |
| 9 | topology basis or spatial touch authority | evidence lookup plan and lookup product | `worth-spatial` | raw evidence vectors, broad receipt scans | raw public evidence scans |
| 10 | topology/spatial proof products | replay, undo, and transaction scope | owning crate plus `worth-kernel` | global replay, re-query rollback authority | replay/undo paths without proof scope |
| 11 | touched closures and spatial authority products | conflict, independence, batch-admission proof | `worth-topo`, `worth-spatial`, `forge-query` | speculative lock-first conflict discovery | batch shortcuts without structural proof |
| 12 | touched proof plus source authority digest | cache/equivalence proof | owning product crate | pointer identity, row count, operator family | cache keys without touched authority |
| 13 | executed proof products | public read-only authority proof and diagnostics | public facades | raw constructors, support pins, local ceremony | public escape hatches |
| 14 | certified milestone products | cross-crate closeout matrix | `worth-kernel` closeout pressure | prose claims, untested residue | uncapped adapters and stale doc claims |

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

## Milestone 5: Query Obligation Selection From Touched Basis

Freeze Query obligation selection as a consumer of topology basis translations
and spatial Query descriptors.

Closes:
- Query-owned selector semantics for touched descriptors
- counters for attempted buckets, matches, deduplication, rejection, selection
- Query-gap rows for missing selector expressiveness

Done when:
- Worth does not fork Query selector matching
- broad collection-only or lifecycle-only selector use is capped residue
- selection breadth scales with touched descriptor breadth

## Milestone 6: Query Graph Read Access Planning Hardening

Freeze Forge Query `9.10` graph read access planning as a second Query
hardening pass over Worth now that Query can derive access postures, required
indexes, budgets, and receipts from declared graph reads.

Closes:
- covered Worth topology, spatial, and kernel graph-read declarations lower into
  Query graph read access plans before execution
- access posture rows distinguish admitted inline indexed, bounded ephemeral,
  paged streaming, persistent-index-required, async-materialization-required,
  store-backed-required, access-capability-required, and typed denial cases
- exact counters for candidate roots, touched nodes, touched edges, frontier
  width, visited/dedup breadth, resident bytes, and page count
- consumer bypass audit for caller-owned N+1 relation loops, ad hoc adjacency
  maps, manual frontier scans, local graph caches, and hidden broad reads
- Query-gap or required-capability rows for graph-read access features Worth
  needs but Query still cannot admit

Done when:
- covered Worth graph reads execute only through admitted Query access plans or
  typed Query postures
- broad boolean and dense frontier reads deny, stream, require persistent index,
  or require async/materialized support before expensive edge traversal starts
- Worth deletes or caps local graph-read folklore rather than wrapping it behind
  compatibility helpers
- access-plan receipts are available to later validator, invalidation, evidence,
  replay, conflict, cache, public proof, and diagnostic milestones

## Milestone 7: Validator And Invariant Derivation

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

## Milestone 8: Derived Invalidation And Dirty Propagation

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

## Milestone 9: Evidence Lookup And Boolean Stage Indexing

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

## Milestone 10: Replay, Undo, And Transaction Scope

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

## Milestone 11: Conflict, Independence, And Batch Admission

Freeze independence proof and conflict denial as touched graph products.

Closes:
- disjointness and compatible aspect-level overlap proof
- conflict classes for entity, relation, aspect, closure, evidence, validators
- batch admission before execution

Done when:
- conflict detection is structural, not speculative lock-first execution
- disjoint operations batch-admit with separate proof products
- closure conflicts deny or serialize with named reasons

## Milestone 12: Cache, Equivalence, And Reuse Contracts

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

## Milestone 13: Public API, Diagnostics, And Explainers

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

## Milestone 14: Cross-Crate Closeout And 7.5 Readiness

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
- validators, invariants, invalidation, evidence lookup, replay, undo,
  conflict, cache, public proof, and diagnostics derive from those products
- old static/global paths are deleted or mechanically sealed as residue
- `Milestone 7.5` starts from touched graph authority rather than local
  validator, evidence, dirty-region, replay, or diagnostic rules
