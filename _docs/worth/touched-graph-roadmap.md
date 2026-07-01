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

## Migration Execution Law

Every broad touched-graph cleanup must use a parallel migration plus hard
deletion shape unless the milestone spec names a narrower exception.

The required pattern is:

1. inventory the old authority surface and classify every row as migrate,
   delete, cap, or Query-gap
2. build a responsibility-named parallel folder or product lane beside the old
   code
3. migrate one vertical product slice until the new lane proves parity or
   stronger authority
4. cut public callers to the new lane through sealed proof products
5. delete the old lane, or cap residue with owner, count, blocker, and removal
   trigger
6. add hard-break compile-fail, source-firewall, and closeout tests so the old
   lane cannot be silently revived

In-place refactoring is not the default strategy. It is allowed only for small
mechanical edits inside the new lane or when a spec explicitly proves that
parallel ownership would create a false authority boundary. Slow conversion
adapters, compatibility shims, and "temporarily keep both paths" bridges are
hostile until the spec gives them a cap, owner, blocker, and deletion trigger.

Each milestone spec must therefore plan:

- the old folders and public surfaces being displaced
- the new parallel folder/product lane
- the first migrated vertical slice
- the cutover proof
- the hard deletion or capped residue closeout
- the firewall that prevents reintroduction

If a milestone does not delete, cap, or Query-gap the old path it touches, it
has not closed.

## Declare-Once Routing Target

The final architecture is not "run smaller local checks." It is one canonical
semantic-graph routing model:

```text
domain/operator declares touched graph authority once
semantic graph vocabulary names entities, relations, aspects, and locality once
registered catalogs and compiled products declare applicability once
planner-owned routing intersects touched graph facts with catalogs/products once
matching read plans, validators, invariants, invalidation, evidence lookup,
replay, undo, conflict, cache, diagnostics, and public proof apply automatically
```

Operator code may produce touched authority. It may not carry manual read-plan
lists, validator arrays, invariant packs, dirty lists, evidence lookup lists,
replay scopes, conflict predicates, cache keys, or diagnostic choreographies.

The remaining roadmap therefore does not treat touched graph, aspects,
compiled products, replay, conflict, cache, and diagnostics as adjacent local
systems. They are different lowered forms of the same semantic graph contract:

- touched graph authority declares what changed
- aspect vocabulary declares which dimensions of meaning exist on that graph
- compiled products/indexes declare which read structures are maintained over
  that graph
- planner-owned routing lowers all remaining families from those same proofs

Later milestones must treat each category as a registered catalog consumed by
touched graph routing:

- graph-read families declare access shape, selectivity posture, required
  capabilities, and receipt requirements
- validator and invariant families declare touched-closure applicability,
  required entity/relation/aspect classes, required graph-read posture,
  enforcement phase, violation/advisory witness, and diagnostic projection
- derived products declare consumed graph facts, consumed spatial evidence,
  invalidation predicate, and rebuild/update posture
- evidence lookup families declare spatial touch authority, topology touch
  identity, stage/receipt identity, and lookup product posture
- replay and undo families declare scope derivation from touched proof and
  effect receipts
- conflict and batch families declare overlap/independence predicates over
  touched closures before execution
- cache and equivalence families declare source authority digest, touched
  digest, invariant set, evidence set, stage, and equivalence policy
- diagnostics declare exact touched facts, registered family, selected
  obligation, witness, and denial or advisory posture

Adding a new registered family once must make it apply to every matching
operator or stage without editing those operators. If that is not true by
Milestone 16, the roadmap has produced scoped refactoring rather than the
declarative touched-graph architecture.

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
`Touched Graph Milestone 9.1` ->
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
| 9.1 | validator/invariant catalog plus stale terminal Query boundary | Query-native runtime boundary for rows, writes, probes, live targets, and certification | `worth-topo` consuming `forge-query` native carriers | compatibility shims, terminal JSON runtime rows, raw string aspect paths, caller-built write commands | stale terminal Query runtime boundary and certification helpers |
| 10 | expanded topology closure plus Query-native runtime boundary | derived invalidation and dirty propagation plan | `worth-topo` | whole-view rebuild by default, hidden projection expansion | broad derived rebuild paths |
| 11 | topology basis or spatial touch authority | evidence lookup plan and lookup product | `worth-spatial` | raw evidence vectors, broad receipt scans | raw public evidence scans |
| 12 | topology/spatial proof products | replay, undo, and transaction scope | owning crate plus `worth-kernel` | global replay, re-query rollback authority | replay/undo paths without proof scope |
| 13 | touched closures and spatial authority products | conflict, independence, batch-admission proof | `worth-topo`, `worth-spatial`, `forge-query` | speculative lock-first conflict discovery | batch shortcuts without structural proof |
| 14 | touched proof plus source authority digest | cache/equivalence proof | owning product crate | pointer identity, row count, operator family | cache keys without touched authority |
| 15 | executed proof products | public read-only authority proof and diagnostics | public facades | raw constructors, support pins, local ceremony | public escape hatches |
| 16 | certified milestone products | cross-crate closeout matrix | `worth-kernel` closeout pressure | prose claims, untested residue | uncapped adapters and stale doc claims |

## Remaining Family Coverage Ledger

Milestones 12 through 16 are the architecture-convergence stretch of this
roadmap. Each milestone must close both:

- one explicit unification boundary
- one explicit set of remaining family surfaces

The remaining family coverage is:

| Milestone | Unification boundary | Remaining family coverage |
| --- | --- | --- |
| 12 | canonical semantic graph contract for post-lookup work | replay scope, undo scope, transaction scope, transaction receipts |
| 13 | aspects as a first-class routing axis | conflict classes, independence proof, batch admission, aspect-local overlap denial |
| 14 | unified compiled-product and equivalence model | cache keys, reuse posture, equivalence comparators, compiled read-product identity |
| 15 | planner-owned routing and public explainability | public proof/status APIs, diagnostics, explainers, routing-localization surfaces |
| 16 | cross-family parity proof and residue collapse | cross-category closeout, representative family declare-once proof, hard-break reintroduction denial |

No remaining milestone may be architecture-only theory, and no remaining
family may be closed without being brought under the milestone's unification
boundary.

## Post-M11 Operational Seed Surfaces

Milestone 11 did not leave abstract "future replay work." It left real
operational seed surfaces that later milestones must consume, inventory, or
delete honestly.

The important current surfaces are:

- `worth-spatial` current evidence lookup cutover path:
  admitted input -> selected plan -> index product -> execution receipt
- `worth-spatial` current public closeout assembly:
  family catalog -> query surface matrix -> query consumer kit -> source
  firewall -> family-stage proof rows
- `worth-kernel` workload composition:
  topology + geometry binding + surface support + projection + transform +
  retained replay + diagnostics + response + evidence ledger
- `worth-kernel` lookup-consumed workload handoff:
  workload stage-index identity must match and raw-row, broad-receipt, and
  caller-owned scan fallback must stay at zero

Operationally, this means the remaining milestones cannot start from a blank
speculation surface. When a milestone touches replay, conflict, cache,
diagnostics, public proof, or closeout:

- it must inventory the in-scope current consumers and producers of these seed
  surfaces
- it must either migrate those surfaces through the new lane, cap them as
  residue with owner/blocker/removal trigger, or prove they are out of scope
- it may not declare success by building only a fresh local subsystem while the
  existing seed surfaces continue teaching older semantics beside it

## Target Directory Skeleton

The remaining milestones should not invent one-off module piles for replay,
conflict, cache, diagnostics, or public proof. They should converge toward one
shared lifecycle shape that is already visible in Milestones 10 and 11:

```text
family_catalog
-> admitted_input
-> selected_plan
-> compiled_product or scope_product
-> execution
-> cutover / public_closeout / source_firewall
```

This is a target architecture skeleton, not a demand for one giant rename pass
before work continues. New work should land in this shape; existing lanes
should migrate toward it as milestones cut over.

Proposed target layout:

```text
crates/
  worth-schema/
    src/
      semantic_graph/
        vocabulary/
          entity.rs
          relation.rs
          aspect.rs
          locality.rs
          scope.rs
        touch_authority/
          topology_basis.rs
          spatial_basis.rs
          closure.rs
        route_identity/
          family_identity.rs
          receipt_identity.rs
          product_identity.rs
          equivalence_identity.rs

  worth-topo/
    src/
      semantic_graph_routing/
        invalidation/
          family_catalog/
          admitted_input/
          selected_plan/
          compiled_product/
          execution/
          operator_cutover/
          public_closeout/
          source_firewall/
        validator_invariant/
          family_catalog/
          admitted_input/
          selected_plan/
          execution/
          public_closeout/
        replay_scope/
          family_catalog/
          admitted_input/
          selected_plan/
          scope_product/
          execution/
        conflict_scope/
          family_catalog/
          admitted_input/
          selected_plan/
          execution/

  worth-spatial/
    src/
      semantic_graph_routing/
        evidence_lookup/
          family_catalog/
          admitted_input/
          selected_plan/
          compiled_product/
          execution/
          stage_cutover/
          workload_cutover/
          public_closeout/
          source_firewall/
          diagnostics/
        replay_scope/
          family_catalog/
          admitted_input/
          selected_plan/
          scope_product/
          execution/
        conflict_scope/
          family_catalog/
          admitted_input/
          selected_plan/
          execution/
        compiled_products/
          equivalence/
          reuse_policy/
          product_identity/
        diagnostics/
          routing_explainers/
          receipt_projections/

  worth-kernel/
    src/
      semantic_graph_runtime/
        workload/
          admitted_workload/
          receipt_set/
          stage_index/
          handoff_composition/
        planner/
          route_request/
          admitted_request/
          selected_route/
          lowered_plan/
        public_proof/
          proof_surfaces/
          diagnostics_surfaces/
          closeout/
        cross_family_parity/
          representative_paths/
          residue_matrix/
          reintroduction_firewall/
```

This skeleton is meant to preserve crate authority:

- `worth-schema` owns shared semantic-graph vocabulary and identity kinds, not
  execution
- `worth-topo` owns topology-native routing families and truth-adjacent
  planning/execution
- `worth-spatial` owns spatial evidence routing families and spatial compiled
  products
- `worth-kernel` owns cross-family workload composition, planner-facing public
  proof, and parity/closeout pressure

This skeleton also names what should not happen:

- no remaining milestone should close by adding a single broad `replay.rs`,
  `conflict.rs`, `diagnostics.rs`, or `cache.rs` file that collapses family
  catalog, admission, planning, execution, and public proof into one place
- no remaining milestone should hide a new family under generic buckets such as
  `helpers`, `support`, `misc`, or `utils`
- no remaining milestone should let diagnostics or public proof become an
  alternate execution path

Operational planning rule:

- if a remaining milestone introduces a new family, that family should be
  planned against this lifecycle skeleton
- if a remaining milestone extends an existing family, it should identify which
  step of this lifecycle it is adding, replacing, or deleting
- if a milestone cannot map its work onto this skeleton honestly, the spec must
  explain why instead of silently creating a parallel architecture

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

Spec:
[touched-graph-milestone-6-graph-read-access-inventory-hard-break.md](./touched-graph-milestone-6-graph-read-access-inventory-hard-break.md)

Freeze every Worth graph-read access surface that still teaches local graph
folklore after Query `9.10` exists.

Closes:
- a parallel responsibility-named graph-read access inventory lane built beside
  old Worth graph-read adoption scaffolding before public closeout cuts over
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
- old `query_adoption/graph_read_access` authority is deleted or mechanically
  capped so future milestones cannot keep building on the old lane

## Milestone 7: Touched Authority To Query Access Declarations

Spec:
[touched-graph-milestone-7-query-access-declarations.md](./touched-graph-milestone-7-query-access-declarations.md)

Freeze the translation from touched graph authority products into real Query
graph-read declarations, read families, access requirements, and required
capability rows.

Closes:
- topology closure and spatial touch authority lower into covered Query
  graph-read declarations before any covered read executes
- graph-read families become a registered declaration catalog keyed by touched
  authority, not per-operator read-plan code
- Milestone 6 graph-read access inventory seed is the only production start
  point for covered declaration work
- domain graph-read operations needed by Worth declare operation resolution,
  access shape, selectivity posture, requirement rows, basis, policy, tenant,
  and relationship-proof posture through Query-owned vocabulary
- registered read families declare access shape, selectivity posture, required
  capability rows, required receipt posture, and Query-gap posture once
- missing Query expressiveness becomes typed Query-gap or required-capability
  posture, not a local Worth traversal adapter
- old local declaration shims, access requirement mirrors, fallback traversal
  helpers, and fabricated support rows are deleted or mechanically capped
- declaration construction is sealed so raw ids, strings, mutation rows, local
  support labels, and copied access rows cannot promote into executable read
  authority

Done when:
- every covered touched graph read has either a Query declaration path or a
  typed gap with owner, cap, and deletion trigger
- one registered read family applies to multiple matching touched authorities
  without editing those operators or stages
- no operator-local read plan, helper-local neighbor traversal, declaration
  shim, or Worth-owned access requirement mirror remains on a covered path
- access requirements are derived by Query from admitted declarations, not
  hand-written in Worth
- broad boolean graph predicates and dense frontier reads reach Query posture
  admission instead of hidden local traversal
- declaration tests prove touched authority is the input and admitted Query
  access artifacts are the next proof product
- closeout produces a Milestone 8 seed with declaration catalog identity,
  Query-derived requirement rows, capability-gap evidence, deletion proof, and
  no access-plan execution or receipt-consumption claim

## Milestone 8: Worth Graph-Read Access Plan Adoption

Spec:
[touched-graph-milestone-8-query-access-plan-adoption.md](./touched-graph-milestone-8-query-access-plan-adoption.md)

Freeze execution of covered Worth graph reads through admitted Query access
plans, typed postures, counters, and receipts.

Closes:
- covered topology, spatial, and kernel graph reads execute only through
  admitted Query graph-read access plans or typed Query postures
- access-plan routing consumes the registered read-family catalog from
  Milestone 7 instead of operator-local execution hints
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
- operators cannot opt into access plans manually; touched authority plus
  registered read-family applicability selects the plan or denial
- access-plan receipts are available to later validator, invalidation, evidence,
  replay, conflict, cache, public proof, and diagnostic milestones
- no covered lane performs caller-owned N+1 work, hidden broad scans, unbounded
  background indexing, or fabricated execution proof
- deletion manifests and closeout tests prove migrated local folklore is gone

## Milestone 9: Validator And Invariant Catalog Routing

Spec:
[touched-graph-milestone-9-validator-invariant-catalog-routing.md](./touched-graph-milestone-9-validator-invariant-catalog-routing.md)

Freeze topology validator and invariant selection as registered catalogs routed
by touched graph closure.

Closes:
- validator and invariant family catalogs with touched-closure applicability
  predicates
- required entity, relation, aspect, scope, lifecycle, and authority classes
  for each family
- required graph-read/access posture for each family before enforcement
- enforcement phase, violation/advisory witness, and diagnostic projection for
  each family
- deletion or collapse of static milestone-one invariant packs
- whole-view validation restricted to certification or named residue

Done when:
- operator-local legality consumes touched graph closure and registered
  families, not manual validator lists
- adding one validator or invariant family once applies to multiple matching
  operators without editing those operators
- global validator packs cannot satisfy ordinary operator closeout
- adding a validator family without a touched predicate fails certification
- operator-local validator arrays, expectation arrays, and "remember to run"
  invariant hooks fail certification or source-firewall tests

## Milestone 9.1: Query-Native Runtime Boundary Rollover

Spec:
[touched-graph-milestone-9.1-query-native-runtime-boundary-rollover.md](./touched-graph-milestone-9.1-query-native-runtime-boundary-rollover.md)

Freeze `worth-topo` against the current `forge-query` aspect-native runtime
boundary before later touched-graph milestones consume runtime rows, writes,
truth probes, live targets, receipts, validators, invalidation, replay,
conflict, cache, or diagnostics.

Closes:
- inventory and deletion ledger for every stale terminal Query API occurrence
  in production `worth-topo`
- Worth topology vocabulary lowering into native Query/Foundation carriers
  instead of strings or terminal JSON
- native entity row production and read decode through `AspectValue`,
  `CanonicalFieldPath`, and Query entity accessors
- backend-admissible write authority consumption instead of caller-built Query
  write commands
- existing-truth probes and retained scalar facts through Query-native probe
  and field-path carriers
- live runtime source routing through `ForgeQueryLiveArtifactTarget`
- certification and operator closeout cut over to the same native production
  boundary
- hard deletion or exact capped residue for old terminal runtime helpers

Done when:
- full `cargo check -p worth-topo --lib` passes
- full `cargo check -p worth-topo --tests` passes or reaches only unrelated
  explicitly recorded external blockers
- ordinary production and certification paths no longer use `external_row`,
  `from_external_projection`, `ForgeQueryAspectValue`, raw aspect-path mutation
  helpers, caller-constructed Query write commands, local probe tuples, or raw
  live-view names as authority
- terminal JSON exists only in named report/document codecs
- no compatibility shim can satisfy the Query-native runtime boundary
- Milestone 10 can consume Milestone 9 validator/invariant catalog products
  through native Query boundary proofs without reviving stale terminal APIs

## Milestone 10: Derived Invalidation And Dirty Propagation

Spec:
[touched-graph-milestone-10-derived-invalidation-dirty-propagation.md](./touched-graph-milestone-10-derived-invalidation-dirty-propagation.md)

Freeze derived topology invalidation as registered derived-product contracts
routed by touched graph closure and the Query-native runtime boundary.

Closes:
- direct versus closure-expanded touch propagation
- derived product catalogs declaring consumed graph facts, consumed spatial
  evidence, invalidation predicate, rebuild/update posture, and diagnostics
- full covered-product migration for materialized graph, traversal views, loop
  cycles, radial rings, shell views, vertex disks, wire views, and every other
  ordinary topology-derived product
- selected invalidation plans derived from touched closure, Query-native
  receipts, and Milestone 9 validator/invariant enforcement proof
- execution receipts proving invalidated, updated, rebuilt, unaffected, denied,
  and capped-residue products
- counters proving invalidation breadth equals closure breadth
- deletion of operator-authored dirty lists and hidden projection expansion

Done when:
- ordinary local operators do not rebuild all derived topology
- ordinary local operators do not author dirty lists except as touched facts
- projection rebuild code does not hide dirty expansion
- derived products without invalidation contracts cannot be consumed
- adding a derived product invalidation contract once makes every matching
  touched closure route to it without editing operators
- every covered ordinary derived topology product is migrated through the new
  invalidation lane, deleted, or proven certification/bootstrap-only residue
- whole-view materialization survives only as certification/bootstrap residue
  with owner, cap, blocker, and removal trigger
- Milestone 11 can consume derived product receipt identity without rebuilding
  derived topology or scanning raw evidence

## Milestone 11: Evidence Lookup And Boolean Stage Indexing

Spec:
[touched-graph-milestone-11-evidence-lookup-and-boolean-stage-indexing.md](./touched-graph-milestone-11-evidence-lookup-and-boolean-stage-indexing.md)

Freeze spatial and boolean evidence lookup as registered lookup families over
spatial touch authority and related topology touched graph identity.

Closes:
- spatial touch authority keyed lookup products
- lookup family catalog entries for spatial touch authority, topology touch
  identity, stage/receipt identity, and lookup product posture
- stage and receipt digest lookup identity
- deletion of raw evidence vectors and broad stage scans

Done when:
- wrong spatial touch digest or mismatched topology digest cannot satisfy lookup
- boolean and spatial stages cannot call "find nearby evidence" loops outside
  registered lookup products
- raw evidence vectors cannot act as public lookup products
- Query descriptors and evidence lookup products cannot satisfy each other
- adding one lookup family once applies to every matching stage/touch authority
  without stage-local lookup wiring

## Milestone 12: Canonical Semantic Graph Contract For Replay And Undo

Spec:
[touched-graph-milestone-12-canonical-semantic-graph-contract-for-replay-and-undo.md](./touched-graph-milestone-12-canonical-semantic-graph-contract-for-replay-and-undo.md)

Freeze the post-lookup routing language so replay, undo, and transaction scope
become consumers of one explicit semantic-graph contract rather than
family-local proof folklore.

Closes:
- one canonical post-lookup vocabulary for touched entities, relations,
  aspects, locality scopes, receipt identity, and transaction-scope claims
- replay-scope families lowered from touched closure plus prior milestone
  receipts instead of operator-family rediscovery
- undo-scope families lowered from the same contract with explicit effect
  receipt requirements and rollback denial posture
- transaction receipts that expose touched digest, validator/invariant outcome,
  invalidation product receipts, evidence lookup receipts, and replay/undo
  scope identity as one boundary packet
- deletion of replay/undo helpers that rediscover scope from command names,
  operation classes, broad topology reads, or post-hoc readback conventions

Done when:
- replay scope, undo scope, and transaction scope all describe their inputs in
  the same semantic-graph terms used by earlier read, validator, invalidation,
  and lookup milestones
- replay does not re-run global topology or broad evidence lookup to prove a
  local edit boundary
- rollback does not re-query authority already captured by touched proof,
  invalidation receipts, evidence lookup receipts, or transaction receipts
- hidden mutation outside admitted undo scope fails closeout with localized
  proof instead of broad transaction failure
- future replay/undo families can be added once and route from semantic-graph
  proof without inventing a parallel scope language

Operationally, this milestone must:
- inventory every current replay-, retained-replay-, rollback-, and
  transaction-scope consumer that already depends on workload receipts,
  evidence-ledger stage identities, lookup receipts, invalidation receipts, or
  public closeout seeds
- define which of those consumers migrate in this milestone, which are capped
  residue, and which remain explicit Query-gap or later-scope work
- cut replay and undo entry only through typed scope products derived from the
  current seed surfaces rather than local reconstruction helpers

This milestone is too narrow if:
- it only builds a replay packet or undo receipt type without classifying the
  current replay/retained-replay/transaction consumers
- it proves topology replay but leaves boolean ledger replay, retained replay,
  or transaction rollback semantics on pre-existing local folklore
- it treats "undo" as a UI-level command reversal instead of an authority- and
  receipt-backed scope product

## Milestone 13: Aspect-Routed Conflict, Independence, And Batch Admission

Spec:
[touched-graph-milestone-13-aspect-routed-conflict-independence-and-batch-admission.md](./touched-graph-milestone-13-aspect-routed-conflict-independence-and-batch-admission.md)

Freeze aspects as a first-class routing axis for post-lookup concurrency so
conflict, independence, and batch admission reason over the same graph meaning
language rather than entity-only overlap heuristics.

Closes:
- aspect-aware overlap contracts over touched closures, replay/undo scope, and
  evidence/validator receipts
- conflict classes for entity, relation, aspect, locality scope, evidence,
  validator pressure, and transaction-scope overlap
- independence proof for disjoint closures and compatible aspect-local overlap
- batch-admission families declaring compatible overlap, denied overlap,
  serialization posture, and diagnostic witness
- deletion of speculative lock-first conflict discovery and caller-owned
  "run both and see" admission folklore

Done when:
- conflict detection is structural and aspect-routed, not speculative
  execution plus rollback
- disjoint operations batch-admit with separate proof products and no hidden
  shared-scope broad scans
- compatible aspect-local overlap is admitted or serialized from declared
  overlap contracts rather than ad hoc stage knowledge
- closure conflicts deny or serialize with named entity/relation/aspect/locality
  reasons
- future conflict or batch families can be declared once against the shared
  aspect-aware routing model and apply to multiple operators or stages without
  operator-local overlap logic

Operationally, this milestone must:
- inventory every current conflict, independence, serialization, and
  multi-operation admission surface that already consumes touched closures,
  validator receipts, evidence lookup receipts, retained replay receipts, or
  workload stage identities
- classify each in-scope path as migrated structural conflict proof, capped
  residue, or later Query-gap rather than leaving "temporary" lock-first or
  executor-first behavior in place
- make aspect-local overlap a named operational distinction so later teams do
  not collapse entity conflict and aspect conflict during implementation
- publish one public closeout product that binds selected conflict plans,
  independence proof, selected batch-admission plan, execution receipt,
  ordinary-consumer residue posture, source-firewall proof, and the Milestone
  14 seed surface without reopening local diagnostics or report strings

This milestone is too narrow if:
- it only adds a conflict enum or overlap helper without inventorying current
  batch-admission and serialization surfaces
- it proves direct-touch disjointness but ignores closure-, validator-, replay-,
  or evidence-derived overlap
- it closes with entity-only conflict classes and leaves aspect-local overlap to
  "future tuning"

## Milestone 14: Unified Compiled Product, Cache, And Equivalence Contracts

Freeze compiled read products, cache identity, and reuse posture as one
contract family so indexes, projections, evidence products, replay products,
and later read acceleration all live under the same equivalence model.

Closes:
- a compiled-product identity contract covering basis, touched digest, source
  authority digest, stage, locality footprint, validator/evidence set, and
  equivalence policy
- cache/equivalence families declaring comparator, canonical ordering,
  acceptable ordering noise, and reuse posture over that same product identity
- separation of authoritative graph truth from compiled read-product truth so
  reuse never promotes derived representation into authority
- reuse denial for geometry-only, topology-touch, replay, and evidence products
  whose semantic graph basis differs even if their rendered output looks
  similar
- deletion of operator-local cache-key code, pointer-identity shortcuts,
  row-count heuristics, and provenance-based reuse folklore

Done when:
- every covered reuse surface is expressed as a compiled-product equivalence
  claim rather than a family-specific helper convention
- operator family, pointer identity, row count, filename provenance, or display
  shape cannot justify reuse
- benign ordering noise preserves equivalence only when the comparator and
  canonical ordering contract say so
- different touched closures, locality footprints, evidence sets, or validator
  sets deny reuse even when product rows look superficially similar
- future compiled products can opt into reuse by declaring one equivalence
  family instead of inventing product-local cache identity

Operationally, this milestone must:
- inventory every current reuse or pseudo-reuse surface for topology-derived
  products, evidence lookup index products, replay products, retained-workload
  products, and any public closeout/read-model helper that currently depends on
  stable identity claims
- consume the phase-13 seed as the starting authority for overlap identity,
  locality footprint identity, selected conflict plan identity, independence
  proof identity, batch-admission plan identity, execution receipt identity,
  residue digest, and firewall digest rather than rediscovering those facts
  from topology, evidence, or local reports
- define the compiled-product identity fields each family must expose before it
  may reuse prior work
- cut all ordinary reuse through those identity contracts or classify the old
  path as residue with blocker and removal trigger

This milestone is too narrow if:
- it only introduces cache keys for one family while other existing compiled
  products still teach ad hoc identity
- it treats index reuse as a local performance optimization instead of a proof
  contract shared across product families
- it closes without distinguishing authoritative truth identity from derived
  product identity

## Milestone 15: Planner-Owned Routing, Public Proof, And Diagnostics

Freeze the ordinary public and diagnostic path so the planner is the single
authority that explains why replay, conflict, cache, invalidation, evidence,
and read-routing decisions happened.

Closes:
- planner-owned lowering for remaining replay, undo, conflict, cache, and
  diagnostic families so execution consumes lowered plans and receipts only
- public read-only proof/status APIs over the same planner-owned products
  instead of local explainers or support-ceremony wrappers
- diagnostics that identify exact touched facts, aspects, locality scope,
  selected family, selected product, receipt chain, witness, and
  denial/advisory posture
- explainers that localize why routing selected a read family, validator,
  invalidation product, evidence lookup, replay scope, conflict class, or
  cache/equivalence result without reopening lower-authority internals
- compile-fail and source-firewall fences against raw constructors, support
  pins, proof helpers, and local routing ceremony

Done when:
- no covered executor re-decides replay scope, conflict class, cache posture,
  or diagnostic route during execution
- public callers can inspect proof/status and routing explanation but cannot
  construct authority, plan identity, or proof products
- diagnostics localize rejection to exact touched graph facts, aspects, or
  Query/posture gaps instead of broad category labels
- public APIs do not expose support pins, raw rows, proof fabrication helpers,
  or family-local explainer shortcuts
- future public proof or diagnostic families plug into planner-owned routing
  once and become visible across matching operators or stages without custom
  explain wiring

Operationally, this milestone must:
- inventory every current public proof, closeout, diagnostic, explainer, and
  workload-composition surface that already exposes receipt-backed status or
  routing-localization claims
- start from the typed `WorthTouchedGraphConflictMilestoneFifteenSeed` and
  `WorthTouchedGraphConflictArchitectureAlignmentReport` emitted by Milestone
  14 phase 16 instead of reopening local reuse, cache, or support logic
- classify which surfaces become planner-owned public products, which remain
  internal diagnostics, and which are deleted as local ceremony or duplicate
  explanation lanes
- preserve the current anti-theatre guards: operational receipts stay the
  authority, while rich diagnostics remain derived projections selected by
  artifact policy

This milestone is too narrow if:
- it only adds nicer diagnostics without migrating the current receipt-backed
  public proof surfaces
- it centralizes diagnostics but leaves execution-time route rediscovery inside
  family executors
- it exposes public "why" APIs that can be satisfied by strings, local report
  rows, or non-authoritative helpers rather than planner-owned proof products

## Milestone 16: Cross-Family Parity Proof, Residue Collapse, And 7.5 Readiness

Close the touched graph program only when the remaining families prove they are
instances of one semantic-graph architecture and all ordinary residue that
disagrees with that claim is deleted, capped, or Query-gap.

Closes:
- cross-category closeout matrix for read families, validators, invariants,
  invalidation, evidence lookup, replay/undo, conflict, cache/equivalence,
  diagnostics, and public proof
- representative declare-once parity proof showing the same routing language
  governs multiple families end to end
- hard-break reintroduction tests for old family-local routing seams, broad
  scans, proof fabrication, and support-ceremony escape hatches
- residue collapse for any remaining slow-conversion adapter, compatibility
  bridge, public raw constructor, or family-local routing helper
- line-cap, composition, and documentation proof that the final architecture is
  legible and matches certified counts

Done when:
- `Milestone 7.5` can consume touched graph proof for overlap extraction
  without reviving local replay, conflict, cache, or diagnostic rules
- the Milestone 15 seed and architecture-alignment report remain sufficient
  typed entry points for later public proof and diagnostics without adding
  local route-rediscovery helpers
- one new representative registered read family, validator/invariant family,
  invalidation family, evidence lookup family, replay/undo family, conflict
  family, cache/equivalence family, and diagnostic family can each be declared
  once and apply to multiple matching operators or stages without editing those
  operators or stages
- no operator names validator lists, invariant lists, dirty lists, evidence
  lookup lists, replay scopes, conflict predicates, cache keys, or diagnostic
  choreography on covered paths
- no remaining ordinary path disagrees with the claim that touched graph,
  aspects, compiled products, and planner-owned routing form one architecture
- no slow-conversion adapter remains in production without cap and removal
  trigger
- certified closeout counts match the roadmap, implementation state, and
  declared family coverage ledger

Operationally, this milestone must:
- inventory every remaining ordinary adapter, helper, compatibility bridge,
  residue row, and source firewall exception still attached to replay, undo,
  conflict, cache, public proof, diagnostics, or workload composition
- prove at least one representative end-to-end path where one shared semantic
  graph language drives read routing, validator selection, invalidation,
  evidence lookup, replay/undo scope, conflict posture, reuse posture, and
  diagnostic/public proof output
- refuse closeout if any remaining ordinary path still requires a family-local
  routing language, even if the path is small

This milestone is too narrow if:
- it is treated as documentation cleanup plus a few residue deletions
- it proves parity only inside one family instead of across multiple family
  kinds
- it allows "mostly unified" ordinary paths to remain uncapped because they
  feel operationally harmless

## Runner Policy

Each touched graph milestone should have its own runner state file and closeout
boundary. A runner may finish the current `Milestone 3` in the old state file,
but it should stop before starting `Milestone 4` so the remaining work can be
split into standalone milestone specs.

Each runner plan must follow the Migration Execution Law. It must name the
parallel folder/product lane, the old lane being displaced, the cutover proof,
and the hard deletion or capped-residue closeout before implementation starts.
The runner should treat in-place refactoring, compatibility adapters, and
slow-conversion bridges as QA findings unless the milestone spec already
approved them with an owner, cap, blocker, and deletion trigger.

Runner escalation policy:
- pause and report after 10 attempts in one milestone
- pause and report on `blocked`
- pause and report when the same finding id reopens twice
- do not continue past a milestone boundary without human confirmation

## Final Acceptance

The touched graph roadmap is complete only when:

- every graph-affecting topology operator and boolean/spatial stage produces
  the correct touched authority product before execution
- touched graph authority, aspect vocabulary, compiled read products, and
  planner-owned routing behave as one semantic-graph architecture rather than
  adjacent local systems
- Query obligation selection consumes touched authority products
- covered Worth graph reads have local folklore inventoried, lower from
  touched authority into Query graph-read declarations, and execute only
  through admitted Query access plans or typed Query postures
- validators, invariants, invalidation, evidence lookup, replay, undo,
  conflict, cache, public proof, and diagnostics are registered once and routed
  automatically from touched graph products
- adding a representative read family, validator/invariant family,
  invalidation family, evidence lookup family, replay/undo family, conflict
  family, cache/equivalence family, or diagnostic family once applies to
  multiple matching operators or stages without editing them
- the remaining family coverage ledger is honest: replay/undo/transaction
  scope, aspect-routed conflict, compiled-product reuse, planner-owned public
  proof, and cross-family parity are all closed under their named milestones
- covered operators do not carry manual read plans, validator arrays, invariant
  packs, dirty lists, evidence lookup lists, replay scopes, conflict
  predicates, cache keys, or diagnostic choreography
- old static/global paths are deleted or mechanically sealed as residue
- `Milestone 7.5` starts from touched graph authority rather than local
  validator, evidence, dirty-region, replay, or diagnostic rules
