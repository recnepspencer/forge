# Milestone 10: Derived Invalidation And Dirty Propagation

## Goal

Freeze derived topology invalidation as declare-once derived-product contracts
routed by expanded touched graph closure, Query-native runtime receipts, and
Milestone 9 validator/invariant enforcement proof.

Ordinary operators may declare touched graph authority. They may not hand-write
dirty lists, trigger whole-view derived rebuilds by default, or hide projection
expansion behind materialization helpers.

## Why This Milestone Exists

Milestones 7 and 8 moved covered graph reads onto Query declarations, access
plans, postures, counters, and receipts. Milestone 9 moved topology validators
and relational invariants onto touched-closure-routed catalogs. Milestone 9.1
removed the stale terminal Query boundary so later work can consume native
Query carriers instead of compatibility folklore.

Milestone 10 is the next authority transition: derived topology products must
stop behaving like views that every operator remembers to dirty or rebuild.
They become registered products with declared consumed graph facts, consumed
spatial evidence, invalidation predicates, update/rebuild posture, counters,
diagnostics, and denial semantics.

## Governing Summaries

- `MENTALITY.md`: protects adversarial-constraint-first engineering. This spec
  treats hidden dirty propagation as the enemy, not as acceptable local
  bookkeeping.
- `arch_laws.md`: protects declare-once, proof-bearing phase transitions.
  Touched closure, Query receipts, legality receipts, invalidation plans, and
  execution receipts must be distinct typed products.
- `composition_laws.md`: protects responsibility-named modules. Product family
  declarations, invalidation selection, execution receipts, diagnostics,
  inventory, and deletion proof must not collapse into one helper.
- `domain_structure_laws.md`: protects visible source-truth boundaries. The
  tree must separate derived product source truth from selected invalidation
  plans, execution receipts, projection consumption, and old rebuild residue.
- `perf_laws.md`: protects semantic-delta-bounded execution. Derived work must
  scale with touched closure breadth and registered product consumption, not
  whole topology breadth or projection count.
- `AI_README.md`: protects Query's core rule: declare intent once, lower it
  once, and execute or inspect it through canonical runtime-owned artifacts.
  Derived products should consume projection receipts, retained artifacts,
  live/materialization support posture, and native runtime receipts rather than
  local side maps, observers, or rebuilt rows.
- `touched-graph-roadmap.md`: protects declare-once routing across reads,
  validators, invariants, invalidation, evidence lookup, replay, conflict,
  cache, diagnostics, and public proof. Milestone 10 belongs after Milestone
  9.1 because derived invalidation must consume validator/invariant proof and
  Query-native carriers before evidence lookup, replay, conflict, and cache
  start depending on derived products.

## Adversarial Constraint

Worth must survive long boolean, NURBS, extrusion, and fillet operation chains
where each operation touches a small local closure inside a large topology with
many derived products and retained projections.

If an ordinary operator can satisfy derived topology maintenance by rebuilding
the whole materialized topology, hand-authoring dirty product lists, expanding
dirty scope inside projection code, using operator-family convention, reading
global topology to rediscover affected products, or consuming a derived product
without a registered invalidation contract, the milestone has failed.

## Product Decision Lock

- Build a parallel `derived_topology::invalidation_plan` lane beside current
  materialization, traversal, loop, radial, shell, vertex-disk, and wire view
  code before cutover.
- Use parallel migration plus hard deletion. In-place refactoring is allowed
  only inside the new lane after the new authority shape exists.
- The new lane owns derived product family declarations, selection from touched
  closure, invalidation/update plans, execution receipts, counters,
  diagnostics, deletion ledger, and source firewalls.
- The old lane being displaced is ordinary maintenance through
  `derived_topology::materialized_graph`, direct traversal/view interpreters,
  `projection::runtime_boundary::read_stage`, operator closeout derived
  fallback rows, and any caller-authored dirty/product expectation lists.
- The first migrated vertical slice proves the new lane. It does not close the
  milestone. After that slice proves the pattern, every covered derived
  topology product must migrate through the new lane before closeout.
- Covered derived products are `materialized_graph`, `traversal_views`,
  `loop_cycles`, `radial_rings`, `shell_views`, `vertex_disks`, `wire_views`,
  and any other production topology-derived product consumed by ordinary
  operators, projection staging, certification closeout, or public proof.
- Existing `derived_topology::materialized_graph` whole-view materialization is
  permitted only as a certification backstop, bootstrap support path, or capped
  residue row with owner, count, blocker, and removal trigger.
- Capped residue is not a deferral mechanism for covered ordinary products.
  Residue may remain only for certification/bootstrap surfaces or a true Query
  capability gap that cannot be closed inside Worth. Ordinary covered derived
  products must cut over.
- Query owns runtime-backed retained artifact posture, projection consumption,
  live/materialization support, read receipts, write receipts, and native
  carrier authority. Worth consumes those artifacts; it does not fabricate
  Query support, receipts, or materialization identity.
- Derived invalidation must consume Milestone 9 selected legality products and
  enforcement receipts when a derived product depends on legality results.
- No compatibility shim may accept old dirty data, local materialization rows,
  or projection-stage expansion and emit Milestone 10 receipts. Receipts are
  minted only by the selected invalidation plan execution lane.
- If an old path cannot be deleted in this milestone, it must be capped as
  residue with owner, exact count, blocker, removal trigger, and a test proving
  it cannot satisfy ordinary invalidation proof.
- No "later migration pass" is allowed for covered derived topology products.
  The milestone closes only after covered product families are cut over,
  deleted, or proven to be non-ordinary certification/bootstrap residue.

## DX Target

Milestone 10 is successful when a future topology operation gets derived
maintenance from declared touched graph meaning, not from operator-authored
dirty lists:

```rust
let touch = topology_operator.declared_touched_graph_basis();
let legality = operator_closeout.selected_legality_enforcement_receipt();
let query_receipts = operator_closeout.query_native_runtime_receipts();

let selected_invalidation = derived_products
    .select_for_touched_closure(touch.expanded_closure(), query_receipts, legality)?;

let invalidation_receipt = selected_invalidation.execute()?;

operator_closeout.attach_derived_invalidation_receipt(invalidation_receipt);
```

The exact API names may differ. The shape may not: touched closure plus
registered derived-product contracts plus Query-native receipts produce one
lowered invalidation plan; execution consumes that plan; operator code does not
name dirty products, rebuild all topology, or choreograph diagnostics.

## Phase Plan

### Phase 1: Derived Rebuild And Dirty Authority Inventory

Freeze every existing derived-topology maintenance path before replacement
code is written. The inventory must classify whole-view rebuilds, dirty-list
producers, projection-expansion helpers, diagnostic materialization, test-only
helpers, and operator closeout rows as migrate, delete, cap, certification-only,
or Query-gap.

**Relevant subsystems**
- `worth-topo::derived_topology::materialized_graph`
- `worth-topo::derived_topology::traversal_views`
- `worth-topo::derived_topology::loop_cycles`
- `worth-topo::derived_topology::radial_rings`
- `worth-topo::derived_topology::shell_views`
- `worth-topo::derived_topology::vertex_disks`
- `worth-topo::derived_topology::wire_views`
- `worth-topo::projection::runtime_boundary::read_stage`
- topology operator closeout derived fallback and derived breadth rows

**Relevant APIs**
- `TopologyMaterializer::materialize_from_truth`
- `TopologyMaterializer::materialize_query_input`
- `MaterializedTopologyView`
- `MaterializationReport`
- `MaterializationFallbackClass::WholeViewRebuild`
- `StagedTopologyRead`
- `stage_topology_read_from_view`
- derived topology interpretation entry points such as
  `interpret_topology_view`, `interpret_wires`, and radial/loop/shell view
  interpreters

**Warnings**
- Do not start by optimizing `TopologyMaterializer` in place. Whole-view
  materialization is the old product lane and must first become classified
  authority.
- Do not classify a broad rebuild as harmless because it is currently fast on a
  fixture. The relevant cost boundary is ordinary operator execution on a large
  topology with a small semantic delta.
- Do not let "test-only" hide ordinary design. Test helpers may remain only if
  the inventory proves they cannot satisfy production invalidation closeout.
- Do not rename dirty-list behavior into touched-graph language. A local list
  authored by an operator is still old authority unless it is derived from the
  sealed touched closure through a registered product contract.

**Test requirements**
- Inventory completeness test: every production path that materializes,
  interprets, invalidates, dirties, rebuilds, or consumes derived topology has a
  row with owner, disposition, blocker, and removal trigger.
- Rejection test: any unclassified whole-view rebuild, operator-authored dirty
  list, projection-expansion helper, or local derived-maintenance hook fails
  closeout.
- Boundary-localization test: certification-only whole-view materialization is
  accepted only through a named residue row and cannot satisfy ordinary operator
  invalidation proof.
- Drift test: adding a new `WholeViewRebuild` fallback or dirty producer
  without an inventory disposition fails the source firewall.

**Engineering decisions**
- The inventory is a production closeout product, not a grep-only test.
- Classification rows must distinguish source truth reads, derived product
  source declarations, selected invalidation work, execution receipts,
  diagnostics, and certification fallback.
- The first migration target should be a derived family that is small enough to
  close honestly but connected enough to prove closure selection and deletion.
- Every old path row must name the exact replacement phase in the new lane:
  catalog declaration, plan selection, execution receipt, diagnostic projection,
  deletion, cap, or Query-gap. "Covered later" is not a disposition.

**Open questions**
- None.

### Phase 2: Parallel Derived Product Family Catalog

Build the new declare-once family catalog beside the old derived topology
modules. A derived product family declares what graph facts and spatial
evidence it consumes, which touched closure facts invalidate it, whether it can
update incrementally, whether it must rebuild from a bounded product source,
which Query projection or retained artifact receipts it requires, and what
diagnostic witness it emits.

**Relevant subsystems**
- new `worth-topo::derived_topology::invalidation_plan`
- `worth-topo::topology_operators::touched_graph_basis`
- `worth-topo::validator_invariant_catalog`
- `worth-topo::query_native_runtime_boundary`
- `forge-query` projection consumption, retained artifact, live/materialized
  view, support posture, read receipt, and write receipt surfaces

**Relevant APIs**
- `TopologyTouchedGraphBasis`
- `TopologyDeclaredTouchedGraphBasis`
- `WorthTopologyValidatorRoutingClosure`
- `WorthTopologySelectedLegalityObligationPlan`
- `WorthTopologySelectedValidatorEnforcementReceipt`
- `WorthTopologySelectedGraphObligationEnforcementReceipt`
- `WorthTopologyMilestoneTenSeed`
- Query projection-consumption receipts and native retained field/path carriers
- Query read/write receipts from the current native runtime boundary

**Warnings**
- A derived product family is not a callback registry. It is a source-truth
  declaration with identity, applicability, consumed facts, required receipts,
  update posture, denial posture, and diagnostics.
- Do not let a family declaration contain execution strategy branches that the
  planner could have lowered. The catalog declares requirements and posture;
  the selected plan carries the monomorphic strategy chosen for this touch.
- Do not allow product family identity to come from display names, file names,
  operator names, or materialized view labels.
- Do not combine spatial evidence products with topology derived products.
  Spatial evidence lookup remains Milestone 11 unless a topology derived product
  only records that it consumes a spatial receipt.

**Test requirements**
- Declare-once test: one derived product family declaration applies to at least
  two matching touched closures or operators without editing those operators.
- Rejection test: a derived product without consumed fact declarations,
  invalidation predicate, required receipt posture, or diagnostic posture cannot
  be consumed.
- Authority test: raw `MaterializedTopologyView`, display labels, string paths,
  operator family names, and copied Query rows cannot mint derived product
  family identity.
- Query posture test: a family that requires projection consumption, retained
  artifact state, live maintenance, or materialization support fails with typed
  Query-required posture when the Query support row is absent.

**Engineering decisions**
- Family records must separate source authority from selected invalidation
  products. Source records are stable declarations; selected plans are derived
  per touched closure.
- Family records must use proof-bearing identity types for product family,
  consumed graph facts, required Query receipt posture, update posture,
  diagnostic posture, and support posture. Raw strings are permitted only as
  rendered labels.
- Product families should model at least materialized graph, traversal view,
  loop cycle, radial ring, shell view, vertex disk, and wire view categories.
  All covered ordinary categories must migrate before closeout.
- Required receipts must be explicit enough for later replay, conflict, cache,
  public proof, and diagnostics to consume without rediscovering meaning.

**Open questions**
- None.

### Phase 3: Touched Closure To Invalidation Plan Lowering

Lower an expanded touched graph closure plus native Query receipts into a
selected derived invalidation plan before any product updates or rebuilds
execute. The plan must say which derived products are unaffected, which are
invalidated, which can update incrementally, which require bounded rebuild,
which are denied by missing Query support, and which remain capped residue.

**Relevant subsystems**
- `derived_topology::invalidation_plan`
- `topology_operators::touched_graph_basis`
- `validator_invariant_catalog::selection_from_touched_closure`
- `validator_invariant_catalog::selected_validator_enforcement`
- `projection::runtime_boundary::read_execution`
- Query support/admission and projection consumption

**Relevant APIs**
- `TopologyTouchedGraphBasis::digest`
- `TopologyDeclaredTouchedGraphBasis::touch_descriptor`
- `WorthTopologyValidatorRoutingClosure`
- `WorthTopologySelectedLegalityObligationPlan`
- `WorthTopologySelectedValidatorEnforcementReceipt`
- `ForgeQueryReadReceipt`
- Query graph-read access summary and no-caller-owned graph work counters

**Warnings**
- Do not perform plan selection by scanning all topology entities or all
  materialized products. Selection breadth must derive from touched closure
  facts and registered family predicates.
- Do not hide closure expansion inside derived product execution. The selected
  plan is the proof product; execution consumes it.
- Do not let execution re-decide strategy, artifact policy, support posture,
  density mode, or fallback posture. If the executor branches on those, the
  selected plan is mechanically incomplete.
- Do not let missing Query support silently degrade to whole-view rebuild.
  Missing support is a typed denial or required-capability posture.

**Test requirements**
- Equivalence test: identical touched closure, family catalog, Query receipts,
  and legality receipts produce the same selected invalidation plan digest
  regardless of operator family name.
- Breadth test: unrelated derived product families remain unaffected and report
  zero execution work when their consumed facts do not intersect the touched
  closure.
- Denial test: missing required Query projection consumption, live/materialized
  support, read receipt, or legality receipt denies plan execution before any
  rebuild or traversal starts.
- Leakage test: a plan that selects more roots, entities, relations, aspects,
  or products than the touched closure plus declared family expansion permits
  fails counters.

**Engineering decisions**
- The selected plan must carry touched digest, catalog digest, Query support
  digest, required receipt digests, selected product rows, unaffected product
  rows, denied product rows, and exact breadth counters.
- Counters must at least name candidate products, matched products,
  invalidated products, unaffected products, incremental update count,
  bounded rebuild count, whole-view fallback count, touched entity count,
  touched relation count, touched aspect count, and caller-owned graph work
  count.
- Rejection must precede expensive construction. Missing support posture,
  missing receipt authority, invalid family declaration, or illegal whole-view
  fallback must deny before building materialized topology or derived
  interpreters.
- Selection may be sparse or dense by explicit policy, but the density switch
  must be named and measured.

**Open questions**
- None.

### Phase 4: Invalidation Execution Receipts And Diagnostics

Execute only from the selected invalidation plan and produce receipt-grade
proof. Execution receipts must distinguish invalidated, updated, rebuilt,
unaffected, denied, and residue products, then expose diagnostics that localize
the exact touched facts and registered product family that caused each outcome.

**Relevant subsystems**
- `derived_topology::invalidation_plan`
- `derived_topology::materialized_graph`
- `derived_topology::traversal_views`
- `derived_topology::loop_cycles`
- `derived_topology::radial_rings`
- `derived_topology::shell_views`
- `derived_topology::vertex_disks`
- `derived_topology::wire_views`
- `projection::diagnostic_surfaces`
- `validator_invariant_catalog` enforcement receipts

**Relevant APIs**
- `MaterializationReport`
- `MaterializationBreadthReport`
- `MaterializationFallbackClass`
- `MaterializedTopologyView::report`
- selected validator and graph-obligation enforcement receipts
- Query projection-consumption and retained artifact receipts

**Warnings**
- Execution may consume old materialization mechanics for migrated products,
  but the receipt must expose whether that was bounded, incremental,
  certification-only, or capped whole-view residue.
- Execution may not reinterpret touched closure, catalog applicability, Query
  support, legality receipts, artifact policy, or density policy. Those are
  planning facts.
- Do not let diagnostics become the execution plan. Diagnostics explain the
  plan and receipt after routing has already happened.
- Do not let advisory invalidation outcomes promote into successful product
  maintenance. Advisory posture must be visible and non-authoritative.

**Test requirements**
- Receipt determinism test: the same selected plan and source receipts produce
  stable invalidation execution receipt digests and stable diagnostic rows.
- Whole-view fallback denial test: ordinary operator execution fails if a
  whole-view rebuild occurs outside an explicitly selected bounded rebuild or
  capped certification residue.
- Diagnostic localization test: every invalidated, updated, rebuilt, denied, or
  residue product row identifies the touched facts, family declaration, required
  receipts, outcome, and reason.
- No hidden graph work test: execution receipts prove zero caller-owned graph
  work beyond the admitted plan and named product update/rebuild posture.

**Engineering decisions**
- The invalidation execution receipt is the canonical Milestone 10 proof
  product consumed by evidence lookup, replay, conflict, cache, public proof,
  and diagnostics.
- Diagnostics must be derived from the selected plan and execution receipt,
  not recomputed by scanning derived products after execution.
- Receipt counters must be rich enough to prove that semantic delta, not view
  count, controlled execution breadth.
- Execution artifacts must separate operational receipts from rich diagnostic
  projections. The hot path produces proof and counters; diagnostics materialize
  only from policy and may not change the operational result.
- Receipt construction must be sealed so tests and public callers cannot
  fabricate "no caller-owned graph work" or "bounded rebuild" claims.

**Open questions**
- None.

### Phase 5: First Derived Product Migration Slice

Migrate the first representative derived product family from old whole-view
maintenance into the new catalog-routed invalidation lane. The slice should
prove the full product ladder: family declaration, touched applicability,
required Query/legality receipts, selected invalidation plan, execution receipt,
diagnostic projection, public closeout row, and deletion or cap of the old
entry point.

**Relevant subsystems**
- `derived_topology::invalidation_plan`
- one representative product category from
  `materialized_graph`, `traversal_views`, `loop_cycles`, `radial_rings`,
  `shell_views`, `vertex_disks`, or `wire_views`
- topology operator closeout rows for derived work breadth
- Query read receipt and projection-consumption support

**Relevant APIs**
- selected invalidation plan and execution receipt products introduced in
  Phases 2 through 4
- existing product interpreter/materializer APIs for the chosen vertical slice
- `MaterializedTopologyView::from_complete_topology_view`
- `TopologyMaterializer::materialize_query_input`
- operator closeout derived fallback policy rows

**Warnings**
- The first slice must be vertical, not a broad half-migration. A product that
  still relies on operator dirty lists or hidden whole-view fallback has not
  migrated.
- Do not keep old and new product maintenance as equal authorities after the
  migrated slice proves parity or stronger behavior.
- Do not choose a slice that can pass only because it uses a tiny complete
  fixture. The slice must include a scale-pressure case where global rebuild
  breadth would be visible in counters.
- Do not choose a toy product with no meaningful touched-closure selectivity.
  The slice must prove unaffected products stay untouched.

**Test requirements**
- Parity test: the migrated product preserves old semantic output for a covered
  scenario while producing new plan and execution receipts.
- Multi-operator test: the same product family declaration routes from at least
  two matching touched closures or topology operators without operator edits.
- Rejection test: the old dirty hook or whole-view rebuild entry point cannot
  satisfy closeout for the migrated product after cutover.
- Unaffected-product test: a touched closure unrelated to the migrated product
  leaves that product unaffected and reports zero update/rebuild work.

**Engineering decisions**
- Prefer a slice connected to already-certified local rewrites so the touched
  closure, Query read receipts, and validator receipts are real.
- The migrated slice should preserve existing interpretation mechanics only
  behind the new execution receipt. Mechanics are not authority.
- If the chosen product cannot avoid whole-view rebuild yet, choose a different
  first slice and keep the failing product in the later migration sweep. The
  milestone may not close with that ordinary product still on whole-view
  rebuild authority.
- The migrated slice must delete or cap its old ordinary public entry point in
  the same phase. Leaving old and new call sites alive for later cleanup is a
  milestone failure unless the residue row names the exact blocker.

**Open questions**
- The implementation plan should select the exact first product family after
  reading current line counts and nearby tests.

### Phase 6: Covered Derived Product Migration Sweep

After the first slice proves the lane, migrate every covered ordinary derived
topology product into the catalog-routed invalidation architecture. This phase
is the no-deferral sweep: materialized graph, traversal views, loop cycles,
radial rings, shell views, vertex disks, wire views, and any other production
derived product consumed by ordinary operators or projection staging must exit
old dirty/rebuild authority.

**Relevant subsystems**
- `derived_topology::materialized_graph`
- `derived_topology::traversal_views`
- `derived_topology::loop_cycles`
- `derived_topology::radial_rings`
- `derived_topology::shell_views`
- `derived_topology::vertex_disks`
- `derived_topology::wire_views`
- `derived_topology::invalidation_plan`
- `projection::runtime_boundary::read_stage`
- `certification::topology_operator_closeout`

**Relevant APIs**
- every derived product family declaration introduced in Phase 2
- selected invalidation plan rows introduced in Phase 3
- invalidation execution receipts introduced in Phase 4
- existing materializer and interpreter entry points for each covered product
- Query-native read/write/projection receipts required by each product family

**Warnings**
- Do not leave a covered ordinary product as "later-product gap." A product is
  migrated, deleted because it is no longer a product, or classified as
  certification/bootstrap-only residue. No fourth category exists.
- Do not migrate by wrapping old rebuild APIs in new names. Each product needs
  an explicit family declaration, touched applicability, required receipt
  posture, selected plan row, execution receipt, and deletion/cutover proof.
- Do not allow a product family to externalize loops to operators or projection
  staging. Product fanout belongs to the catalog and selected plan.
- Do not allow whole-view materialization to remain as the ordinary maintenance
  strategy for any covered product.

**Test requirements**
- Complete coverage test: every covered derived product category has a family
  declaration, selected plan row, execution receipt path, and old-authority
  deletion/cap row.
- Rejection test: any covered product still maintained by operator dirty lists,
  hidden projection expansion, or ordinary whole-view rebuild fails closeout.
- Cross-product declare-once test: adding or editing one product family
  declaration changes routing for every matching touched closure without
  operator or projection-stage edits.
- Breadth regression test: scale-pressure cases prove each migrated product's
  invalidation breadth follows touched closure plus declared family expansion,
  not global topology size or total product count.
- Residue denial test: certification/bootstrap residue cannot satisfy ordinary
  operator or projection invalidation proof for any covered product.

**Engineering decisions**
- The migration sweep should proceed product family by product family, but each
  family must close vertically before the next one starts.
- Products that share lifecycle may share phase abstractions, but their
  consumed facts, update posture, diagnostics, and counters remain separate
  declarations.
- The sweep must leave the tree with the new lane as the ordinary maintenance
  authority. Old product code can remain only as mechanics beneath execution or
  as certification/bootstrap residue that cannot be consumed by ordinary paths.

**Open questions**
- None.

### Phase 7: Operator And Projection Read-Stage Cutover

Cut ordinary operator closeout and projection read-stage consumption over to
the new invalidation proof products. Operators may attach touched basis,
selected legality proof, Query read/write receipts, and invalidation execution
receipts. They may not attach dirty lists, derived expectation arrays, or
whole-view rebuild proofs as ordinary success evidence.

**Relevant subsystems**
- `topology_operators::application`
- `topology_operators::local_rewrites`
- `projection::runtime_boundary::read_stage`
- `certification::topology_operator_closeout`
- `derived_topology::invalidation_plan`
- `validator_invariant_catalog`

**Relevant APIs**
- `TopologyDeclaredTouchedGraphBasis`
- `TopologyDeclaredMutationSequence`
- selected legality and enforcement receipt products from Milestone 9
- Query-native write/read receipts from Milestone 9.1
- invalidation selected plan and execution receipt products from this milestone
- `StagedTopologyRead` and read-stage certification helpers

**Warnings**
- Do not let projection read-stage code expand dirty scope because it has
  access to the materialized graph. The invalidation plan is the expansion
  authority.
- Do not permit operator closeout to treat "derived fallback policy accepted"
  as equivalent to Milestone 10 proof. Fallback must be denied, capped, or
  certification-only.
- Do not create a compatibility adapter that accepts old dirty data and emits
  new receipts. Receipts must be created only by the new plan/execution lane.
- Do not let operator closeout externalize loops over product families. The
  operator supplies touched authority and receipts; the registered catalog and
  plan own product fanout.

**Test requirements**
- Cutover test: representative operator closeout succeeds only when it carries
  touched basis, legality proof, Query receipts, and invalidation execution
  receipt.
- Source-firewall test: adding an operator-local dirty list, derived product
  expectation array, or whole-view rebuild proof to a covered path fails
  certification.
- Projection-boundary test: read-stage projection consumption cannot expand
  dirty scope or satisfy invalidation proof without a selected plan receipt.
- Replay-honesty seed test: the invalidation execution receipt carries enough
  digest and counter information for Milestone 12 replay/undo scope without
  rerunning global topology.

**Engineering decisions**
- Operator closeout should expose Milestone 10 receipt identity and counters,
  not internal constructors or selected plan internals.
- Projection read-stage code may consume derived products but cannot own
  invalidation selection.
- Cutover must leave a Milestone 11 evidence lookup seed that names derived
  product receipts without claiming spatial lookup completion.
- Cutover must make invalidation proof a required operator closeout input for
  covered paths. Optional attachment would preserve the old "remember to dirty"
  architecture.
- Cutover must cover every migrated product from Phase 6. A product that remains
  unreachable from ordinary operator/projection flow has not actually migrated.

**Open questions**
- None.

### Phase 8: Hard Deletion, Residue Caps, And Source Firewalls

Delete or mechanically cap every old derived maintenance path touched by the
milestone. Old paths may survive only as named certification/bootstrap residue
with owner, count, blocker, removal trigger, and a test proving they cannot
satisfy ordinary operator invalidation.

**Relevant subsystems**
- `derived_topology::materialized_graph`
- `projection::runtime_boundary::read_stage`
- `certification::topology_operator_closeout`
- `certification::projection_closeout`
- `derived_topology::invalidation_plan`
- public facade and compile-fail contract surfaces

**Relevant APIs**
- `MaterializationFallbackClass::WholeViewRebuild`
- `MaterializationReport::whole_view_materialization`
- old derived fallback policy closeout rows
- new invalidation deletion ledger and source-firewall report
- compile-fail targets for private constructors and forbidden old paths

**Warnings**
- Deletion is part of the milestone, not optional cleanup. A migrated product
  with its old ordinary dirty path still alive is not closed.
- Capped residue must not be vague. Each cap needs exact count, owner, reason,
  blocker, and removal trigger.
- Do not allow a capped residue row to become a second authority lane. Residue
  is a denial/certification posture, not an alternate execution path.
- Do not let public facades expose constructors for invalidation family
  records, selected plans, execution receipts, deletion rows, or closeout proof.

**Test requirements**
- Hard-deletion test: migrated old dirty hooks, old operator closeout dirty
  rows, and ordinary whole-view derived rebuild paths are gone or denied.
- Residue-cap test: each remaining whole-view fallback is certification-only,
  counted, owned, and rejected as ordinary invalidation proof.
- Full-sweep deletion test: every covered product from Phase 6 has no remaining
  old ordinary dirty/rebuild/projection-expansion entry point.
- Compile-fail test: public callers cannot forge derived family records,
  selected invalidation plans, execution receipts, or closeout proof.
- Source-firewall test: forbidden phrases and modules for operator-authored
  dirty lists, hidden projection expansion, and broad rebuild helpers cannot
  reappear on covered paths without inventory disposition.

**Engineering decisions**
- The source firewall should scan production sources, not only tests, while
  allowing explicitly named documentation/report codecs.
- Deletion rows should be attached to the Milestone 10 closeout so later
  milestones can consume counts without rereading source.
- If a broad rebuild must remain, it is a named failure mode that later
  milestones must either eliminate or continue to count as residue.
- The firewall must ban old authority by semantic surface, not only exact
  symbol spellings: dirty list authorship, operator-local product fanout,
  hidden projection expansion, and ordinary whole-view rebuild proof are all
  forbidden even if renamed.

**Open questions**
- None.

### Phase 9: Closeout Scaffold And Milestone 11 Seed Contract

Build the public closeout scaffold and Milestone 11 seed contract, but do not
claim final milestone closeout until the per-family migration phases below have
closed. This phase freezes the proof shape that the family phases must satisfy:
declare-once derived invalidation, semantic-delta-bounded execution,
Query-native receipt consumption, hard deletion or capped residue, and no
operator-authored dirty choreography on covered paths.

**Relevant subsystems**
- `derived_topology::invalidation_plan`
- `validator_invariant_catalog`
- `query_native_runtime_boundary`
- `certification`
- `facade`
- Milestone 11 evidence lookup planning surfaces

**Relevant APIs**
- Milestone 10 closeout product
- invalidation family catalog digest
- selected invalidation plan digest
- invalidation execution receipt digest
- deletion ledger digest
- residue audit digest
- source-firewall report digest
- Milestone 11 seed carrying touched closure, spatial receipt references where
  available, derived product receipt identity, and lookup-readiness posture

**Warnings**
- The scaffold must not claim evidence lookup, replay, conflict, cache, public
  diagnostics, final touched-graph closeout, or completed Milestone 10 product
  migration. It seeds those milestones and defines the proof shape for the
  remaining family migrations.
- Do not publish mutable constructors for closeout products. Public surfaces
  expose read-only proof/status.
- Do not report performance as "faster" without counters naming the measured
  boundary and breadth.
- Do not treat one migrated product as proof that all derived topology is
  migrated. Closeout must name every covered product and prove it is migrated,
  deleted, or certification/bootstrap-only residue.

**Test requirements**
- Closeout integrity test: closeout digests bind touched closure, catalog,
  selected plan, execution receipts, Query support posture, legality receipts,
  deletion ledger, residue audit, and source firewall.
- Full-product closeout scaffold test: closeout fails unless every covered
  derived product has migrated execution receipts or a non-ordinary residue
  denial; before Phases 10 through 16 close, this test must expose the missing
  family as incomplete rather than fabricating success.
- Declare-once proof test: adding or modifying one derived product family once
  changes routing for multiple matching touched closures without operator
  edits.
- Performance proof test: closeout counters prove ordinary invalidation breadth
  equals touched closure plus declared family expansion, not global topology
  size or number of derived products.
- Milestone 11 seed test: the seed carries enough derived-product identity and
  receipt posture for evidence lookup to start without rebuilding derived
  topology or scanning raw evidence.

**Engineering decisions**
- Public proof should expose identities, counters, denials, residue posture, and
  selected family summaries, not constructors.
- The Milestone 11 seed must distinguish topology-derived product receipts from
  spatial evidence lookup products so the next milestone cannot substitute one
  for the other.
- Closeout must be able to answer: which products were selected, why, by which
  touched facts, with what Query receipts, and at what execution breadth.
- Closeout counters must include slope-sensitive cases so later work can tell
  whether breadth follows touched closure size, product family count, or global
  topology size.

**Open questions**
- None.

### Phase 10: Materialized Graph Product Migration

Migrate `materialized_graph` as its own vertical derived-product family. This
phase must replace ordinary whole-view materialization as operator maintenance
authority with a declared product family, real read-stage execution, real Query
and legality receipts, real product output, counters, diagnostics, and deletion
or certification/bootstrap-only capping of the old folder surfaces.

**Relevant subsystems**
- `derived_topology::materialized_graph`
- `derived_topology::invalidation_plan::migrated_products`
- `projection::runtime_boundary::read_stage`
- `query_native_runtime_boundary`
- `certification`

**Relevant APIs**
- `TopologyMaterializer::materialize_from_truth`
- `TopologyMaterializer::materialize_query_input`
- `MaterializedTopologyView`
- `MaterializationReport`
- `StagedTopologyRead`
- materialized-graph family catalog record
- materialized-graph migration closeout and receipt row

**Warnings**
- Do not wrap whole-view materialization and call it migrated. Ordinary
  invalidation must be selected from touched closure and registered consumed
  facts before read-stage execution.
- The read stage must be real: it must consume Query-native receipts and
  legality proof, emit a materialized-graph execution receipt, and expose
  counters that name touched breadth versus global topology breadth.
- Delete the old ordinary materialized-graph maintenance folder/path after
  cutover. If a materializer survives, it must be capped as
  certification/bootstrap residue with owner, exact count, blocker, and removal
  trigger.

**Test requirements**
- Semantic parity test: the migrated materialized-graph product matches the old
  materialized output for the same authoritative topology while consuming the
  selected invalidation plan, Query receipts, and legality receipts.
- Scale-pressure test: a small touched closure inside a larger topology does
  not produce ordinary work proportional to global topology size.
- Hard-deletion test: old ordinary materialized-graph rebuild entry points
  cannot satisfy Milestone 10 closeout after cutover.
- Receipt integrity test: forged or fixture-only materialized-graph receipts
  fail closeout.

**Engineering decisions**
- The migrated folder must live under the new invalidation product lane, not as
  an adapter over the old materialized graph folder.
- Product output, read-stage receipt, diagnostic projection, deletion ledger
  row, and counters are one vertical product ladder; none may be optional for
  ordinary closeout.

**Open questions**
- None.

### Phase 11: Traversal Views Product Migration

Migrate `traversal_views` as its own vertical derived-product family. This
phase must move traversal-view consumption to touched-closure-selected
invalidation receipts with real read-stage products, real Query receipts, and
hard deletion of old traversal-view ordinary maintenance paths.

**Relevant subsystems**
- `derived_topology::traversal_views`
- `derived_topology::invalidation_plan::migrated_products`
- `projection::runtime_boundary::read_stage`
- `query_native_runtime_boundary`
- `certification`

**Relevant APIs**
- traversal view interpreters and source rows
- traversal-view family catalog record
- selected invalidation plan rows
- invalidation execution receipt rows
- traversal-view migration closeout

**Warnings**
- Do not let traversal helpers rediscover affected walks from global topology.
  Touched closure plus registered consumed graph facts must select the work.
- The migrated traversal read stage must produce real receipt identity and
  product output, not copied rows from old traversal helpers.
- Delete old traversal-view ordinary folder paths after cutover or cap them as
  certification/bootstrap-only residue that cannot satisfy ordinary proof.

**Test requirements**
- Parity test: migrated traversal-view output matches the old traversal view for
  equivalent authoritative topology and selected touched closure.
- Rejection test: traversal-view execution without required Query receipts or
  legality receipt fails before building traversal output.
- Scale-pressure test: unrelated global traversal structures do not increase
  ordinary work for a local touched closure.
- Source-firewall test: old traversal-view ordinary maintenance paths cannot
  re-enter covered execution after deletion.

**Engineering decisions**
- Traversal view migration owns its own input rows, output rows, counters,
  residue scan, and phase seed.
- Traversal view receipts must be consumed by Phase 17 closeout and Milestone
  11 seed identity without rebuilding traversal state.

**Open questions**
- None.

### Phase 12: Loop Cycles Product Migration

Close `loop_cycles` as a fully real migrated product family. Existing loop
cycle migration work may be reused only if it satisfies the same read-stage,
receipt, deletion, and source-firewall requirements as the newer product
phases. This phase is not a rubber stamp.

**Relevant subsystems**
- `derived_topology::loop_cycles`
- `derived_topology::invalidation_plan::migrated_products::loop_cycles`
- `projection::runtime_boundary::read_stage`
- `query_native_runtime_boundary`
- `certification`

**Relevant APIs**
- `close_loop_cycle_migration_slice`
- `LoopCycleExecutionInput`
- `LoopCycleDerivedProductOutput`
- `LoopCycleMigrationCloseout`
- loop-cycle old-authority residue scan
- loop-cycle family catalog record

**Warnings**
- Do not accept the current loop-cycle lane unless its read-stage receipts are
  real Query-native receipts and legality receipts, not fixture-admitted proof.
- Delete the old `derived_topology::loop_cycles` ordinary execution folder/path
  after the new lane owns ordinary maintenance, or cap only
  certification/bootstrap residue.
- Loop cycle migration must prove unaffected products stay untouched and that
  selected loop work is bounded by touched closure, not whole-shell scans.

**Test requirements**
- Receipt-backed parity test: migrated loop-cycle output matches old loop-cycle
  semantics while preserving selected plan, Query receipt, legality receipt,
  and product output identity.
- Hard-break test: old loop-cycle direct interpreter paths cannot satisfy
  ordinary invalidation closeout after cutover.
- Scale-pressure test: local loop touch does not cause work proportional to all
  loops or all half-edges in the topology.
- Source-firewall test: any remaining loop-cycle old authority is named
  certification/bootstrap residue with exact count and removal trigger.

**Engineering decisions**
- Existing loop-cycle migration modules may stay only if they are the new lane,
  not a bridge to the old ordinary folder.
- The Phase 17 closeout must consume loop-cycle migrated-family proof produced
  by this phase, not a generic required-sweep bridge.

**Open questions**
- None.

### Phase 13: Radial Rings Product Migration

Migrate `radial_rings` as its own vertical derived-product family. Radial ring
maintenance is locality-sensitive and must not fall back to broad rediscovery
around every vertex or edge unless that path is explicitly
certification/bootstrap residue.

**Relevant subsystems**
- `derived_topology::radial_rings`
- `derived_topology::invalidation_plan::migrated_products`
- `projection::runtime_boundary::read_stage`
- `query_native_runtime_boundary`
- `certification`

**Relevant APIs**
- radial ring interpreters and source rows
- radial-ring family catalog record
- selected invalidation plan rows
- invalidation execution receipt rows
- radial-ring migration closeout

**Warnings**
- Do not compute radial rings by scanning all adjacency around unrelated
  topology. Touched closure must narrow the candidate radial neighborhoods.
- The read stage must consume real Query-native topology reads and legality
  proof before producing radial-ring product output.
- Delete old radial-ring ordinary folders/entry points after cutover or cap
  only certification/bootstrap residue.

**Test requirements**
- Parity test: migrated radial-ring product matches the old radial interpreter
  for equivalent topology and touched neighborhoods.
- Locality test: touching one radial neighborhood does not recompute unrelated
  vertex or edge rings.
- Denial test: missing Query read receipt or selected legality receipt rejects
  before radial product construction.
- Source-firewall test: old radial broad interpreter paths cannot satisfy
  ordinary Milestone 10 proof.

**Engineering decisions**
- Radial-ring product output must expose selected roots, radial work count, and
  touched-neighborhood breadth counters.
- The migrated product must feed Phase 17 closeout and Milestone 11 seed
  identity directly.

**Open questions**
- None.

### Phase 14: Shell Views Product Migration

Migrate `shell_views` as its own vertical derived-product family. Shell view
maintenance must be selected from touched closure and registered consumed graph
facts, not from whole-body or whole-shell rediscovery hidden in projection
helpers.

**Relevant subsystems**
- `derived_topology::shell_views`
- `derived_topology::invalidation_plan::migrated_products`
- `projection::runtime_boundary::read_stage`
- `query_native_runtime_boundary`
- `certification`

**Relevant APIs**
- shell view interpreters and source rows
- shell-view family catalog record
- selected invalidation plan rows
- invalidation execution receipt rows
- shell-view migration closeout

**Warnings**
- Do not treat shell view recomputation as harmless because shell counts are
  small in fixtures. The cost boundary is long boolean/NURBS/extrusion/fillet
  chains inside large bodies.
- The migrated shell read stage must bind real Query receipts, legality
  receipts, product output, diagnostics, and counters.
- Delete old ordinary shell-view folder paths after cutover or cap only
  certification/bootstrap residue.

**Test requirements**
- Parity test: migrated shell-view output matches old shell interpretation for
  equivalent authoritative topology.
- Scope test: local face/wire/shell touches do not recompute unrelated shell
  views as ordinary invalidation work.
- Denial test: missing Query or legality support fails before shell output
  construction.
- Hard-deletion test: old shell view ordinary interpreter paths cannot satisfy
  Phase 17 closeout.

**Engineering decisions**
- Shell view output must carry touched shell/source identity, execution breadth,
  and product output digest.
- Shell view diagnostics must be derived from the selected plan and execution
  receipt, not from a second shell scan.

**Open questions**
- None.

### Phase 15: Vertex Disks Product Migration

Migrate `vertex_disks` as its own vertical derived-product family. Vertex disk
maintenance must be rooted in touched closure locality and Query-native
neighborhood receipts rather than broad adjacency reconstruction.

**Relevant subsystems**
- `derived_topology::vertex_disks`
- `derived_topology::invalidation_plan::migrated_products`
- `projection::runtime_boundary::read_stage`
- `query_native_runtime_boundary`
- `certification`

**Relevant APIs**
- vertex disk interpreters and source rows
- vertex-disk family catalog record
- selected invalidation plan rows
- invalidation execution receipt rows
- vertex-disk migration closeout

**Warnings**
- Do not scan every vertex disk as an ordinary path when one local closure was
  touched.
- The migrated vertex-disk read stage must produce real Query receipt identity,
  legality receipt identity, product output, diagnostics, and counters.
- Delete old vertex-disk ordinary folder paths after cutover or cap only
  certification/bootstrap residue.

**Test requirements**
- Parity test: migrated vertex-disk output matches old vertex-disk semantics
  for equivalent topology and local touched closure.
- Locality test: touching one vertex neighborhood does not recompute unrelated
  vertex disks.
- Denial test: missing Query neighborhood receipt or legality proof fails
  before product construction.
- Source-firewall test: old vertex-disk broad reconstruction cannot satisfy
  ordinary invalidation proof after cutover.

**Engineering decisions**
- Vertex-disk counters must name touched vertices, touched incident edges,
  selected roots, and execution work.
- Phase 17 closeout must consume vertex-disk migrated-family proof produced by
  this phase, not generic receipt-bound placeholder proof.

**Open questions**
- None.

### Phase 16: Wire Views Product Migration

Close `wire_views` as a fully real migrated product family. Existing wire-view
migration work may be reused only if it satisfies the same read-stage, receipt,
deletion, and source-firewall requirements as the other product phases.

**Relevant subsystems**
- `derived_topology::wire_views`
- `derived_topology::invalidation_plan::migrated_products::wire_views`
- `projection::runtime_boundary::read_stage`
- `query_native_runtime_boundary`
- `certification`

**Relevant APIs**
- `close_wire_view_migration_slice`
- `WireViewExecutionInput`
- `WireViewDerivedProductOutput`
- `WireViewMigrationCloseout`
- wire-view old-authority residue scan
- wire-view family catalog record

**Warnings**
- Do not accept the current wire-view lane unless its read-stage receipts are
  real Query-native receipts and legality receipts, not fixture-admitted proof.
- Delete the old `derived_topology::wire_views` ordinary execution folder/path
  after the new lane owns ordinary maintenance, or cap only
  certification/bootstrap residue.
- Wire-view migration must prove open/closed/branching wire semantics without
  broad graph rediscovery as ordinary work.

**Test requirements**
- Receipt-backed parity test: migrated wire-view output matches old wire-view
  semantics while preserving selected plan, Query receipt, legality receipt,
  and product output identity.
- Hard-break test: old wire-view direct interpreter paths cannot satisfy
  ordinary invalidation closeout after cutover.
- Scale-pressure test: local wire touch does not cause work proportional to all
  wires or all half-edges in the topology.
- Source-firewall test: any remaining wire-view old authority is named
  certification/bootstrap residue with exact count and removal trigger.

**Engineering decisions**
- Existing wire-view migration modules may stay only if they are the new lane,
  not a bridge to the old ordinary folder.
- The Phase 17 closeout must consume wire-view migrated-family proof produced
  by this phase, not a generic required-sweep bridge.

**Open questions**
- None.

### Phase 17: Final Public Closeout And Milestone 11 Seed

Publish the real Milestone 10 closeout only after Phases 10 through 16 prove
all covered families have real migration receipts, real read-stage products,
hard deletion or certification/bootstrap residue, and source-firewall proof.

**Relevant subsystems**
- `derived_topology::invalidation_plan`
- all migrated product-family lanes from Phases 10 through 16
- `validator_invariant_catalog`
- `query_native_runtime_boundary`
- `certification`
- `facade`
- Milestone 11 evidence lookup planning surfaces

**Relevant APIs**
- Milestone 10 closeout product
- per-family migrated closeout proofs
- selected invalidation plan digest
- invalidation execution receipt digest
- deletion ledger digest
- residue audit digest
- source-firewall report digest
- Milestone 11 seed carrying derived product receipt identity and
  lookup-readiness posture

**Warnings**
- No generic required-family bridge may stand in for a family-specific migrated
  product proof at final closeout.
- No covered ordinary product may remain as "later migration" work. It must be
  migrated, hard-deleted, or proven non-ordinary certification/bootstrap
  residue.
- Public proof remains read-only. Constructors and mutable rows stay sealed.
- Milestone 11 evidence lookup is still not implemented here; this phase only
  seeds it with topology-derived product receipt identity.

**Test requirements**
- Full-family closeout test: final closeout fails if any of
  `materialized_graph`, `traversal_views`, `loop_cycles`, `radial_rings`,
  `shell_views`, `vertex_disks`, or `wire_views` lacks a family-specific
  migrated receipt or valid non-ordinary residue denial.
- Closeout integrity test: closeout digests bind touched closure, catalog,
  selected plan, per-family execution receipts, Query support posture, legality
  receipts, deletion ledger, residue audit, and source firewall.
- Declare-once proof test: adding or modifying one derived product family once
  changes routing for multiple matching touched closures without operator
  edits.
- Performance proof test: counters prove ordinary invalidation breadth follows
  touched closure plus declared family expansion, not global topology size or
  the total number of unrelated products.
- Milestone 11 seed test: the seed carries enough derived-product identity and
  receipt posture for evidence lookup to start without rebuilding derived
  topology or scanning raw evidence.

**Engineering decisions**
- Final closeout consumes per-family proof products, not raw collections,
  copied digests, generic placeholders, or test fixtures.
- The Milestone 11 seed distinguishes topology-derived product receipts from
  spatial evidence lookup products so the next milestone cannot substitute one
  for the other.
- Final counters must include slope-sensitive cases for touched closure size,
  selected product family count, unrelated product count, and global topology
  size.

**Open questions**
- None.

## Must Ship

- A parallel `worth-topo::derived_topology::invalidation_plan` lane with
  family catalog records, touched-closure applicability, required Query and
  legality receipt posture, selected invalidation plans, execution receipts,
  counters, diagnostics, deletion ledger, residue audit, source firewall, and
  closeout proof.
- A complete inventory of old derived topology rebuild, dirty propagation,
  projection expansion, and whole-view fallback authority.
- At least one honest migrated derived product family proving the full product
  ladder from declaration to selected plan to execution receipt to deletion.
- A full covered-product migration sweep with family-specific migrated
  receipts, read-stage products, deletion proof, and source-firewall evidence
  for `materialized_graph`, `traversal_views`, `loop_cycles`, `radial_rings`,
  `shell_views`, `vertex_disks`, `wire_views`, and every other ordinary
  production topology-derived product.
- Operator and projection read-stage cutover for every covered migrated product
  so old dirty lists or whole-view rebuilds cannot satisfy ordinary
  invalidation closeout.
- Exact counters for candidate products, matched products, unaffected products,
  invalidated products, incremental updates, bounded rebuilds, whole-view
  fallback residue, touched entities, touched relations, touched aspects,
  selected roots, Query-required denials, and caller-owned graph work.
- Phase-typed products for family source declarations, selected invalidation
  plans, execution receipts, diagnostic projections, deletion ledgers, residue
  audit rows, and Milestone 11 seeds. Later phases must consume the exact prior
  proof type, not raw collections.
- Hard-break tests that make operator-authored dirty lists, hidden projection
  expansion, ordinary whole-view rebuild proof, public constructor forgery, and
  local compatibility adapters fail.
- Public closeout proof and a Milestone 11 evidence lookup seed.
- Final closeout denial for generic required-family bridges, copied digests,
  fixture-admitted receipts, or any product family that has not produced its
  own migrated-family proof or valid non-ordinary residue denial.

## Must Preserve

- Milestone 2 and 3 touched graph basis authority and digest stability.
- Milestone 7 and 8 Query graph-read declaration, access plan, posture, and
  receipt authority.
- Milestone 9 validator/invariant catalog selection and enforcement receipt
  semantics.
- Milestone 9.1 Query-native aspect, row, field-path, write, read, live target,
  and probe boundaries.
- Existing semantic output for migrated derived products, except where the new
  contract intentionally denies hidden broad rebuild or missing Query support.
- Certification/bootstrap whole-view materialization only as named residue, not
  ordinary execution authority.
- The ability to destroy derived products and rebuild them from authority for
  certification or recovery. Incremental invalidation must improve ordinary
  breadth without confusing derived state with source truth.

## Acceptance Evidence

- Tests prove inventory completeness, unclassified old-path rejection, and
  source-firewall denial for new dirty lists, hidden projection expansion, and
  broad rebuild helpers.
- Tests prove a derived product family declared once applies to multiple
  matching touched closures without operator edits.
- Tests prove selected invalidation plans are deterministic from touched
  closure, family catalog, Query receipts, legality receipts, and support
  posture.
- Tests prove unrelated products stay unaffected with zero update/rebuild work.
- Tests prove missing Query support, projection consumption, read receipt, write
  receipt, or legality receipt denies before rebuild or traversal execution.
- Tests prove migrated derived product semantics preserve old output while
  producing new selected plan and execution receipts.
- Tests prove every covered ordinary derived product is migrated, deleted, or
  certification/bootstrap-only residue before closeout.
- Tests prove whole-view materialization cannot satisfy ordinary invalidation
  proof after cutover except as explicitly capped certification residue.
- Tests prove public callers cannot forge family records, selected plans,
  execution receipts, closeout proof, or Milestone 11 seed products.
- Tests prove plan execution cannot branch into unsupported strategy, support,
  density, artifact-policy, or fallback decisions that were not already lowered
  into the selected invalidation plan.
- Tests prove scale-pressure counters grow with touched closure and selected
  product breadth, not global topology entity count, relation count, or total
  derived product count.

## Sequencing Notes

Milestone 10 must start from Milestone 9 selected validator/invariant proof and
Milestone 9.1 Query-native runtime boundary proof. It should not implement
Milestone 11 evidence lookup, Milestone 12 replay/undo, Milestone 13 conflict,
Milestone 14 cache/equivalence, or Milestone 15 public diagnostics.

The first implementation plan may use one derived product family as a vertical
slice to prove the lane, but final closeout must follow the explicit
per-family phases in order: materialized graph, traversal views, loop cycles,
radial rings, shell views, vertex disks, and wire views. The runner must not
skip from the closeout scaffold to final closeout while any family still relies
on a generic required-sweep bridge, fixture-admitted receipt, copied digest, or
old ordinary folder. It should treat in-place refactors, dirty-list adapters,
hidden projection expansion, uncapped whole-view fallback, and "we will migrate
the rest later" as QA findings.

## Required Self-Check

- Does the milestone solve a real structural problem or just package work
  cosmetically? Yes. It replaces local dirty/rebuild authority with declared
  derived-product invalidation.
- Is the adversarial constraint precise and load-bearing? Yes. It rejects
  operator-authored dirty lists, whole-view rebuild defaults, hidden projection
  expansion, and consumption without contracts.
- Does the roadmap justify this milestone now? Yes. Derived invalidation is the
  next product ladder step after validator/invariant proof and Query-native
  runtime rollover.
- Does the spec preserve crate authority boundaries? Yes. Query owns runtime
  artifacts and support posture; Worth owns topology derived product semantics.
- Are the phases carrying most of the real design information? Yes.
- Is each phase centered on one conceptual detail or boundary? Yes.
- Does each phase contain at least 2 adversarial tests by default? Yes.
- Could a competent engineer map this spec into honest types, modules, and
  tests? Yes. The spec names the new lane, old surfaces, product artifacts, and
  proof requirements.
- Does the milestone belong in this roadmap sequence, or is it out of order?
  Yes. It belongs before evidence lookup because lookup should consume derived
  receipts rather than rebuild or scan derived topology.
