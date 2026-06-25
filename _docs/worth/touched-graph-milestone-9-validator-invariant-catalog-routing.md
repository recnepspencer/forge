# Milestone 9: Validator And Invariant Catalog Routing

## Goal

Freeze topology validator and relational invariant selection as declare-once
registered catalogs routed by expanded touched graph closure.

Milestone 9 must make ordinary topology legality enforcement consume the proof
products from Milestones 7 and 8. Operator code may produce touched authority
and execute admitted Query graph-read plans; it may not carry validator arrays,
static invariant packs, expectation lists, or "remember to run this check"
hooks on covered paths.

## Why This Milestone Exists

Milestone 8 gives Worth covered graph-read access plans, receipts, postures, and
counters. That is the last prerequisite before topology validators and
relational invariants can become catalog-routed products instead of global
packs or operator-local ceremonies.

This milestone belongs immediately after Milestone 8 because validator
selection must know which graph-read access is admitted before enforcement can
be honest. Running validators directly from static lists would preserve the old
global-validation architecture and leave no durable path toward the declare-once
operator model needed before broad booleans, NURBS, extrusions, and fillets.

## Governing Summaries

- `MENTALITY.md`: protects adversarial-constraint-first design. The spec must
  solve the failure mode where future local operations keep rediscovering or
  manually naming validator work instead of consuming touched authority.
- `arch_laws.md`: protects proof-bearing phase transitions and declare-once
  resource definition. Validator applicability must be declared once, lowered
  into Query graph obligation authority, and carried forward as typed
  selected-obligation proof.
- `composition_laws.md`: protects semantic file ownership. Catalog records,
  applicability, routing, enforcement, diagnostics, deletion proof, and public
  closeout must not collapse into one validation helper.
- `domain_structure_laws.md`: protects visible ownership and authority
  separation. The tree must distinguish validator family source truth, selected
  validator products, enforcement receipts, diagnostics, and old static-pack
  residue.
- `perf_laws.md`: protects semantic-delta-bounded execution. Validator and
  invariant breadth must scale with touched closure and Query-selected
  obligation requirements, not whole topology size or static pack length.
- `crates/forge-query/docs/AI_README.md`: protects Query as the ordinary
  domain-facing runtime layer. Worth may declare domain semantics, but Query
  must own graph touch descriptors, operating worlds, obligation selection,
  dispatch, support posture, budget denial, executor verdict evidence, and
  Consumer Kit adoption proof.
- `touched-graph-roadmap.md`: protects the Milestone 9 slot as the transition
  from adopted Query graph-read access plans to selected topology validators
  and relational invariants.

## Adversarial Constraint

Worth must survive a long local-operation chain on a large topology where each
operation touches a small closure but many validators, invariants, derived
views, and diagnostics exist in the system.

If an operator can satisfy topology legality by passing a static validator pack,
expectation array, global validation report, old `milestone_one` invariant
registration, blueprint-local validator row, local legality graph, local
validator map, private graph walk, or broad whole-view scan instead of a
Query-selected graph obligation product derived from touched closure, operating
world, admitted Query access receipts, support posture, and executor verdict
evidence, this milestone has failed.

## Product Decision Lock

- Build a parallel touched validator/invariant routing lane beside current
  `validation`, `runtime_support`, blueprint validator rows, and topology
  operator closeout expectation surfaces.
- Use parallel migration plus hard deletion. In-place refactoring is allowed
  only inside the new lane after the new authority shape exists.
- `worth-topo` owns topology validator and invariant family semantics,
  topology-specific applicability declarations, violation/advisory witness
  interpretation, and topology legality diagnostics.
- `forge-query` owns graph touch descriptors, operating world descriptors,
  graph obligation registration, obligation index selection, dispatch
  envelopes, support rows, support pinning, budget denial, executor verdict
  evidence, and Consumer Kit adoption proof.
- Milestone 8 owns graph-read access plan authority. Milestone 9 may consume
  access receipts and postures; it may not fabricate or reinterpret them.
- Public method visibility is not support. Query support posture, support pins,
  and Consumer Kit adoption proof decide whether a visible surface is admitted
  for an ordinary covered lane.
- `worth-kernel` closeout pressure may certify public proof and deletion
  posture, but it must not own topology legality semantics.
- Whole-view validation is allowed only as certification comparison or capped
  residue with owner, count, blocker, and removal trigger. It cannot satisfy
  ordinary operator closeout.

## DX Target

Milestone 9 is successful when a future topology operation gets legality
coverage from declared graph meaning, not from operator-local lists:

```rust
let touch = topology_operator.declared_touched_graph_basis();
let operating_world = QueryOperatingWorld::authoritative_workspace();

let selected = query
    .graph_obligations()
    .select_for_touch(touch.query_graph_touch_descriptor(), operating_world)?;

let enforcement = selected.execute_with_admitted_receipts(
    milestone_eight_seed.graph_read_access_receipts(),
)?;

operator_closeout.attach_legality_receipt(enforcement);
```

The exact API names may differ. The shape may not: touched authority plus
operating world selects registered obligations once; operator code does not name
validators, invariant packs, dirty work, or diagnostic choreography.

## Phase Plan

### Phase 1: Old Validation Authority Inventory And Cut Line

Freeze the old validation authority surfaces before building the new lane.
Every static pack, expectation array, public registration, whole-view validator
entry, and blueprint-local validator row must be classified as migrate, delete,
cap, or out-of-scope Query/access gap.

**Relevant subsystems**
- `crates/worth-topo/src/validation`
- `crates/worth-topo/src/runtime_support.rs`
- `crates/worth-topo/src/topology_operators/*_blueprint`
- `crates/worth-topo/src/certification/topology_operator_closeout`
- `crates/worth-kernel/src/graph_read_access_plan_adoption/phase_eight_public_closeout`

**Relevant APIs**
- `WorthGraphReadAccessPlanAdoptionMilestoneNineSeed`
- `TopologyValidator::derived_validation_report`
- `TopologyValidator::materialized_validation_report`
- `validation::rule_registry::DERIVED_TOPOLOGY_RULE_SPECS`
- `runtime_support::milestone_one_invariant_registrations`
- topology operator closeout validator expectation rows

**Warnings**
- Do not treat `TopologyValidator::derived_validation_report` as the target
  architecture. It is a comparison oracle and migration source until displaced.
- Do not let old blueprint validator rows promote into selected obligation
  proof.
  They may help inventory the old system but cannot be the new authority.
- Do not mark old whole-view validation as `keep`. It must be delete, cap, or
  certification-only comparison.

**Test requirements**
- Inventory completeness test: every current validator/invariant entry point
  discovered under validation, runtime support, blueprint rows, and closeout
  expectations has exactly one disposition row.
- Rejection test: an unclassified validator surface or `keep` disposition fails
  closeout.
- Source-firewall test: adding a new operator-local validator array or static
  invariant pack without a migration disposition fails certification.
- Public-boundary test: raw inventory rows, deletion dispositions, and old
  validator expectations cannot be constructed as selected validator products.

**Engineering decisions**
- Create an inventory product lane with names that describe old authority
  posture, not implementation provenance.
- The inventory product must carry source path, authority kind, owner,
  disposition, removal trigger, Query/access dependency if any, and whether the
  surface is allowed only for certification comparison.
- The phase closes only when the old lane has a mechanically auditable cut line.

**Open questions**
- None.

### Phase 2: Parallel Domain Catalog And Query Obligation Vocabulary

Build the new domain catalog vocabulary beside the old validation registry and
lower each covered family into Query graph obligation vocabulary. This phase
defines what Worth may declare once and what Query must own afterward.

**Relevant subsystems**
- `worth-topo` validation and topology operator authority surfaces
- `worth-schema` touched topology vocabulary, if the existing shared vocabulary
  is required for entity, relation, aspect, scope, lifecycle, or operating-world
  classes
- `forge-query` graph touch obligation authority
- Milestone 8 Query access-plan seed products consumed by validator families

**Relevant APIs**
- topology touched graph basis vocabulary and closure products
- `WorthGraphReadAccessPlanAdoptionMilestoneNineSeed`
- `TopologyValidationRuleIdentity`
- Query graph obligation registration declarations
- Query obligation kinds: `BlockingInvariant`, `SchemaContractValidator`,
  `AdvisoryObligation`, `PreflightSequencingObligation`,
  `CapabilityGapScreen`, `OperatingContextGate`
- Query support statuses: `Supported`, `Unsupported`, `NotApplicable`,
  `DiagnosticOnly`, `DeferredToBackstop`
- existing validator modules: ownership, loop wiring, radial rings, shell
  closure, vertex disks, naming

**Warnings**
- A catalog family is not a function pointer registry. It is an applicability
  and proof contract. Execution handlers are subordinate to family authority.
- Worth catalog declarations are not the final selector authority. They must
  lower into Query graph obligation registrations and support rows.
- Do not encode applicability as string matching over operator names.
- Do not let family identity be reconstructed from validator display names.
  Identity must be stable and sealed.
- Do not collapse validators and invariants if their enforcement phase,
  witness shape, or graph-read requirement differs.
- Do not invent Worth-local obligation kind labels, support status labels, or
  covered lane names. Use the Query vocabulary so support pinning and Consumer
  Kit proof stay comparable.

**Test requirements**
- Declare-once test: one registered validator family applies to at least two
  matching touched closures without editing either operator path.
- Rejection test: a family without touched-closure applicability, required
  graph-read posture, enforcement phase, or witness projection cannot enter the
  catalog.
- Query-vocabulary parity test: every covered Worth validator/invariant family
  lowers into a Query graph obligation kind, support status, covered lane, and
  operating-world posture without local label translation.
- Identity test: raw strings, copied rule names, static row names, and blueprint
  validator labels cannot mint family identity.
- Composition test: adding a new family does not require editing routing code
  outside the catalog registration surface.

**Engineering decisions**
- Introduce responsibility-named catalog products such as validator family
  identity, invariant family identity, applicability predicate, required
  touched class set, required access posture, enforcement phase, witness
  posture, and diagnostic projection.
- Worth catalog records are authoritative for topology legality meaning. Query
  graph obligation registrations are authoritative for runtime selection,
  support posture, dispatch, and execution proof.
- Catalog records must preserve operating world posture. Authoritative,
  branch, preview, construction, runtime-backed test workspace, and future
  admitted worlds are not interchangeable.
- Family records must expose read-only proof surfaces and keep constructors
  sealed to the registration path.

**Open questions**
- None.

### Phase 3: Query Obligation Selection From Touched Closure

Admit the expanded topology touched closure needed for validator and invariant
selection, lower it into Query graph touch descriptors and operating world
descriptors, then let Query select obligations from the obligation index before
any validator executes. If the incoming Milestone 8 handoff does not already
carry the exact expanded closure product needed by this milestone, this phase
must derive a validator-routing closure from sealed touched basis plus admitted
access receipts and expose it as a typed product. It may not fall back to
whole-view discovery.

**Relevant subsystems**
- `topology_operators/touched_graph_basis`
- expanded topology closure product or validator-routing closure product
- Milestone 8 access-plan receipts and postures
- new validator/invariant catalog lane
- Query graph obligation selector coverage and support rows

**Relevant APIs**
- touched graph basis digests, entity/relation/aspect classes, scopes,
  lifecycle posture, and operating world
- Milestone 8 receipt, posture, counter-accounting, and batch-accounting
  exports
- selected Query graph-read plan receipts consumed by validator families
- Query graph touch descriptors
- Query operating world descriptors
- Query obligation index and selector coverage reports
- Query dispatch envelopes and executor verdict evidence

**Warnings**
- Worth must not own the selector. Worth lowers topology meaning into Query
  descriptors; Query selects and explains the selected obligations.
- Selection must produce Query obligation products, not Worth-local selected
  validator arrays with Query-looking names.
- Closure intake must not be a broad topology scan with a touched-looking
  digest. It must be bounded by sealed touched basis plus admitted access
  receipts.
- The router must not rediscover graph reads. It consumes admitted access plans
  and typed required-posture rows from Milestone 8.
- The selected plan must carry exact counters: touched entities, touched
  relations, touched aspects, candidate obligations, selected obligations,
  denied obligations, required access receipts, missing receipts, budget
  denials, support-posture denials, and whole-view residue.

**Test requirements**
- Closure authority test: a validator-routing closure cannot be constructed
  from raw topology rows, operator names, old validation reports, or whole-view
  scans.
- Parity test: a local closure that touches successor/radial/ownership-relevant
  facts selects the same semantic obligation families that the old global
  report would have required, but through Query obligation authority rather
  than whole-view inspection.
- Rejection test: a selected obligation requiring a missing Query access
  receipt is denied before validator execution starts.
- Operating-world test: the same touched closure in authoritative, branch,
  preview, and runtime-backed test worlds preserves distinct support posture
  and cannot reuse the wrong world's selected obligation proof.
- Budget test: a touch whose selected obligation would exceed admitted budget
  denies as `BudgetExceeded` with state-load counters and cost class evidence
  instead of completing a private local graph walk.
- Breadth test: unrelated validator families remain unselected when their
  applicability predicate does not intersect the touched closure.
- Counter test: selected-obligation breadth scales with touched closure breadth
  and not with total topology row count.

**Engineering decisions**
- Treat expanded closure as an authority product with its own digest, counters,
  and source proof. Do not hide closure expansion inside validator selection.
- The selected Query obligation plan is the output authority of routing and the
  input authority for enforcement.
- Denial rows must distinguish missing access posture, missing touched class,
  wrong lifecycle posture, wrong operating world, and certification-only
  whole-view residue.
- Denial rows must preserve Query support posture labels and canonical denial
  labels, including `budget-exceeded`, instead of flattening to Worth-local
  validation errors.
- Router tests must include a larger topology with a small closure to prevent
  accidental global scan dependence.

**Open questions**
- None.

### Phase 4: First Validator Family Migration Slice

Migrate the first representative validator family from old direct execution
into the new catalog-routed lane. The slice should be narrow enough to finish
honestly but rich enough to prove the product ladder: catalog record, touched
applicability, Query-selected obligation plan, access receipt requirement,
enforcement receipt, Worth witness, and diagnostic projection.

**Relevant subsystems**
- `validation::ownership`
- `validation::loop_wiring`
- `validation::radial_rings`
- `validation::shell_closure`
- `validation::vertex_disks`
- `certification/topology_operator_closeout/validation_breadth_row.rs`

**Relevant APIs**
- `TopologyValidator::materialized_validation_report`
- `TopologyValidator::derived_validation_report`
- existing validation rule identities
- Query-selected obligation plan from Phase 3
- Query projection-consumption and typed consumed-fact receipts when the
  migrated family depends on retained materialized facts

**Warnings**
- Do not migrate by wrapping the old global validator call in a new selected
  plan. That preserves global work behind a local-looking API.
- Do not start with the easiest display-only rule if it does not prove graph
  read, witness, and denial semantics.
- Do not allow a migrated family to read outside the selected closure unless it
  consumes an explicit access receipt or emits typed residue.
- Do not read materialization rows, bridge internals, or cached report rows
  directly. If the family needs retained facts, it must consume Query projection
  facts with typed consumed-fact proof.

**Test requirements**
- Equivalence test: the migrated family produces the same pass/violation result
  and rule identity as the old validator on matching local hostile fixtures.
- Boundary-localization test: the migrated family reports the touched facts and
  witness rows that caused selection or violation without requiring a whole-view
  validation report.
- Leakage test: adversarial topology rows outside the touched closure do not
  affect a local family unless closure expansion or an admitted access receipt
  includes them.
- Denial test: attempting to execute the migrated family without the selected
  plan fails at compile time or through a typed pre-execution denial.
- Projection-consumption test: a migrated family that depends on materialized
  facts carries a Query consumed-fact receipt, while direct materialization-row
  or bridge-row access is rejected.

**Engineering decisions**
- Prefer migrating a family that exercises topology relationship structure,
  not just naming metadata, so the lane proves real graph authority.
- Preserve old validator modules as implementation mechanics only after the new
  Query-selected obligation product owns applicability and enforcement
  authority.
- The migrated family must expose an enforcement receipt that later milestones
  can consume for invalidation, replay, conflict, cache, public proof, and
  diagnostics.
- Projection consumption is the only approved bridge from retained Query facts
  into topology witness interpretation. Report-row reuse is comparison-only.

**Open questions**
- The implementation plan should choose the exact first family after measuring
  which current validator has the smallest honest slice with real graph-read
  and witness pressure.

### Phase 5: Relational Invariant Family Catalog

Split relational invariants from validator execution and register them as their
own declare-once family catalog. This phase prevents `milestone_one` invariant
registrations from surviving as a static pack under a new name, while still
respecting Query's boundary: Query owns the public registration facade and
relational owns invariant execution authority.

**Relevant subsystems**
- `validation::reference_integrity`
- `runtime_support::milestone_one_invariant_registrations`
- Query-owned runtime support entry points
- topology operator declaration legality contracts
- Query invariant registration artifacts and graph obligation authority

**Relevant APIs**
- `milestone_one_invariant_registrations`
- Query runtime invariant registration surfaces
- `forge_query_domain(...).for_intent(...).register_invariant_catalog(...)`
- `ForgeQueryRuntime::builder().invariant_catalog(...)`
- `ForgeQueryRuntime::builder().invariant_registration_artifact(...)`
- `ForgeQueryDeclarationLegalityContract`
- touched graph basis and Query-selected obligation plan products

**Warnings**
- Do not preserve the old invariant registration pack as a public ordinary
  entry point. It is old authority until re-expressed as catalog families.
- Relational invariants must not be selected only by mutation family name.
  They must route from touched closure and declared applicability.
- Query registration artifacts are not executable invariant engines. They are
  registration proof that lowers into the ordinary runtime builder.
- Do not import relational builder plumbing as the ordinary Worth path.
  Relational authority stays under Query's public invariant registration lane.
- Manual invariant packs and graph-composition invariant-pack hooks are
  compatibility or custom extension surfaces only. They cannot satisfy covered
  ordinary operator legality unless represented as Query-registered obligations
  with support posture and execution evidence.
- Validator families and invariant families may share touched vocabulary, but
  they must not share constructors or selected products if their enforcement
  phase differs.

**Test requirements**
- Applicability test: an invariant family declared once is selected for
  multiple matching topology operations from touched closure facts.
- Rejection test: a static invariant registration pack cannot satisfy ordinary
  operator closeout after the catalog path exists.
- Query-registration test: the invariant catalog materializes a Query
  invariant-registration artifact or ordinary Query builder registration, not a
  Worth-local legality graph.
- Authority test: raw Query invariant rows, copied registration names, or
  declaration legality strings cannot mint invariant-family identity.
- Mixed-authority rejection test: queued Query-owned invariants plus an
  explicitly supplied relational runtime authority fail rather than silently
  merging authority paths.
- Manual-pack rejection test: a graph-composition invariant pack or static
  invariant list cannot satisfy a covered topology operator closeout except as
  capped compatibility residue or a declared custom extension outside the
  ordinary lane.
- Residue test: any remaining `milestone_one` registration path is capped as
  certification-only or deleted, with owner and removal trigger.

**Engineering decisions**
- Invariant families must declare touched closure applicability, required
  entity/relation/aspect classes, enforcement phase, Query access posture,
  violation/advisory witness shape, and diagnostic projection.
- Invariant enforcement may reuse Query runtime mechanics only after a
  catalog-selected invariant product exists.
- A selected invariant family must produce an enforcement receipt, not just a
  passed/failed boolean.
- Structural authoring legality belongs here: containment and ownership rules,
  move eligibility, reference legality, splice boundaries, and other topology
  authoring constraints must become registered invariant families instead of a
  host-local legality graph.
- Custom invariant hooks may survive only behind explicit support posture and
  adoption proof. They are not the architectural path for Worth's built-in
  topology legality.

**Open questions**
- None.

### Phase 6: Enforcement Receipts, Violation Witnesses, And Advisory Posture

Execute selected validator and invariant obligations only from Query-selected
obligation plans and produce enforcement receipts with structured outcomes. For
covered lanes, closeout proof must be execution-backed through Query's graph
obligation authority, not selection-only.

**Relevant subsystems**
- new selected validator/invariant routing lane
- Query graph obligation dispatch envelopes and executor verdicts
- Query graph obligation Consumer Kit
- existing validator modules as execution mechanics
- topology operator application and certification closeout
- later diagnostic and public-proof milestones

**Relevant APIs**
- selected validator family plan
- selected invariant family plan
- Query graph obligation dispatch envelope evidence
- Query graph obligation executor verdict evidence
- `ForgeQueryGraphObligationExecutionBackedAdoptionProof`
- Consumer Kit support pinning, bypass audit, adoption manifest, and residue
  manifest surfaces
- Query projection consumption receipts and typed consumed-fact artifacts
- Milestone 8 access receipts and bounded execution contract
- `TopologyValidationError` and existing validation report rows as migration
  comparison surfaces

**Warnings**
- Binary pass/fail is not enough. Enforcement must distinguish passed,
  advisory, violation, denied-before-execution, and certification-only residue.
- Enforcement must not reselect families. It consumes the selected plan.
- Enforcement must not perform broad graph reads unless the selected obligation
  carries an admitted access receipt that makes that breadth explicit.
- Selection-only proof is not final closeout proof for covered execution lanes.
  It may support inspection, but adoption must connect selected obligations to
  real executor rows.
- Budget denial is a valid enforcement outcome. It must not be converted into
  success by shrinking, sampling, or locally completing graph work.
- Materialized fact consumption must stay on Query's projection-consumption
  path. Enforcement cannot bypass Query by reading retained materialization
  rows, live rows, bridge rows, cached validator reports, or derived report
  internals.
- Artifact-policy-gated diagnostics may explain a denial, but they cannot add
  hidden graph work or change an unsupported obligation into a supported one.

**Test requirements**
- Outcome topology test: selected families produce structured passed,
  advisory, violation, and denied-before-execution rows where applicable.
- Replay-honesty test: executing the same selected plan against the same
  authority produces stable enforcement receipt digests.
- Execution-backed adoption test: Consumer Kit proof contains real Query
  executor rows and an adoption manifest execution proof digest.
- Leakage test: enforcement fails if it attempts caller-owned graph work not
  covered by the selected plan or Milestone 8 receipt exports.
- Bypass-audit test: local validator maps, local graph walks, private legality
  graphs, fabricated receipts, and string-list support pins appear as audit
  failures or capped residue.
- Budget-denial test: `BudgetExceeded` preserves state-load counters, cost
  class, support posture, and artifact-policy-gated diagnostics.
- Typed-fact-consumption test: enforcement that needs retained materialized
  facts carries Query consumed-fact proof, and direct retained-row/report-row
  access is counted as bypass audit or fails the source firewall.
- Diagnostic-witness test: each violation or advisory row contains exact
  touched facts, selected obligation identity, Worth family identity, access
  receipt digest, and witness projection.

**Engineering decisions**
- The enforcement receipt is the canonical Milestone 9 proof product consumed
  by invalidation, replay, conflict, cache, public proof, and diagnostics.
- Query executor verdict evidence is the enforcement proof for covered graph
  obligation lanes. Worth witness rows are domain interpretation of that proof,
  not a replacement authority.
- Consumed projection facts are part of the receipt boundary when a validator
  or invariant needs retained material. Witness interpretation may project
  those facts, but it may not reopen materialization storage.
- Existing `TopologyValidationError` can inform witness shape, but the new
  receipt must be structured enough for downstream routing.
- Receipt counters must name candidate families, executed families, advisory
  rows, violation rows, denied rows, skipped certification-only rows, graph-read
  receipt count, caller-owned graph work count, budget-denial count, support
  pin count, executor-row count, adoption-manifest count, and residue-manifest
  count.

**Open questions**
- None.

### Phase 7: Operator And Certification Cutover

Cut ordinary operator closeout and certification paths to the new selected
validator/invariant products, then remove old expectation-array authority.
Covered lanes must keep Query's lane vocabulary visible instead of collapsing
everything into generic "operator validation."

**Relevant subsystems**
- `topology_operators/application`
- `topology_operators/declaration_entry`
- `topology_operators/*_blueprint`
- `certification/topology_operator_closeout`
- `certification/tests/topology_operator_closeout/expectations.rs`

**Relevant APIs**
- selected validator and invariant plans
- enforcement receipts
- topology operator closeout reports and validation breadth rows
- public facade contract compile-fail tests
- Query covered lane labels: `graph-composition`, `declaration-entry`,
  `read-family`, `primitive-construction-birth`, `worth-topo-operator-catalog`,
  and `worth-kernel-phase-chain`

**Warnings**
- Do not cut over by translating old expectation arrays into new selected
  plans. Selection must flow from touched closure and catalog applicability.
- Do not keep old validator breadth rows as an equal proof. They become
  comparison evidence, deletion proof, or capped residue.
- Do not let operator families hand-name validators after cutover.
- Do not hide unsupported or diagnostic-only lanes. `Unsupported`,
  `NotApplicable`, `DiagnosticOnly`, and `DeferredToBackstop` are real support
  postures that must remain visible in closeout.

**Test requirements**
- Cutover test: ordinary operator closeout consumes Query-selected obligation
  products and enforcement receipts, not validator expectation arrays.
- Source-firewall test: adding a validator array, static expectation row, or
  operator-local invariant hook to a covered path fails certification.
- Multi-operator declare-once test: adding or modifying one catalog family
  changes selection for multiple matching operators without touching operator
  code.
- Support-posture test: every covered lane reports a Query support status and
  cannot be silently omitted because the lane is unsupported or diagnostic-only.
- Public compile-fail test: public callers cannot forge selected obligation
  plans, enforcement receipts, violation witnesses, or closeout rows.

**Engineering decisions**
- Old certification rows may remain only as migration comparison fixtures until
  this phase closes.
- The public closeout should expose selected-obligation counts, Worth family
  counts, and residue posture, not internal catalog constructors.
- Closeout rows must preserve covered lane, obligation kind, support status,
  operating world, support pin digest, budget digest where relevant, and
  executor proof digest.
- Cutover must prioritize hard breaks over slow conversion. Compatibility
  bridges need cap, owner, blocker, and removal trigger.

**Open questions**
- None.

### Phase 8: Hard Deletion, Public Closeout, And Milestone 10 Seed

Delete or mechanically cap old static/global validation authority and produce
the Milestone 10 seed for derived invalidation.

**Relevant subsystems**
- new validator/invariant catalog routing closeout
- old validation registry and runtime support exports
- public facade contract compile-fail suite
- touched graph roadmap closeout counters

**Relevant APIs**
- selected validator/invariant catalog closeout
- enforcement receipt exports
- Query graph obligation Consumer Kit adoption proof
- Query support pins and selector coverage reports
- Query bypass audit and residue manifests
- deletion ledger and capped residue report
- source firewall report
- Milestone 10 seed for derived invalidation and dirty propagation

**Warnings**
- This milestone does not close derived invalidation. It prepares the selected
  validator/invariant proof that Milestone 10 consumes.
- The closeout must not claim all validators are migrated if any old whole-view
  path remains outside certification-only residue.
- Do not expose constructors for catalog records, selected plans, receipts, or
  witness rows through public facades.
- Do not accept selection-only Query proof as final adoption proof for covered
  execution lanes. The closeout must be execution-backed or explicitly
  diagnostic-only/deferred with residue.

**Test requirements**
- Hard-deletion test: old ordinary static validator packs, operator-local
  expectation arrays, and public invariant registration authority are deleted
  or represented by capped residue rows.
- Source-firewall test: reintroducing old validation authority names, public
  registration packs, or hand-named validator arrays fails.
- Consumer Kit closeout test: registration, selector coverage, support pinning,
  in-memory execution proof, bypass audit, adoption manifest, and residue
  manifest all exist for covered graph obligation work.
- Closeout counter test: selected-obligation counts, Worth family counts,
  enforcement receipt counts, graph-read receipt counts, denied-obligation
  counts, budget-denial counts,
  support-pin counts, executor-row counts, adoption-manifest counts, residue
  manifest counts, source-firewall violation counts, and whole-view
  certification-only counts are exact.
- Milestone 10 seed test: the seed contains touched closure digest, selected
  Query obligation digests, validator and invariant family digests, enforcement
  receipt digests, executor proof digest, support posture digest, witness digest
  summary, deletion proof digest, residue digest, source firewall digest, and
  does not claim invalidation planning.

**Engineering decisions**
- The Milestone 10 seed is a proof handoff, not a convenience summary.
- Public proof surfaces must be read-only and narrower than the internal
  catalog topology.
- The public proof must distinguish Query-selected obligation proof, Worth
  domain witness interpretation, and certification-only comparison residue.
- Remaining whole-view validation must be named `certification-only` or capped
  residue. No ordinary operator path may depend on it.

**Open questions**
- None.

## Must Ship

- A parallel touched validator/invariant routing lane in `worth-topo` that
  owns catalog source truth, topology applicability declarations,
  violation/advisory witness interpretation, and topology closeout proof.
- Query graph obligation registration and execution-backed Consumer Kit
  adoption proof for covered validator/invariant work.
- Projection-consumption proof for any validator or invariant that consumes
  retained materialized facts.
- Inventory and deletion/cap classification for old validation authority:
  global validator reports, `DERIVED_TOPOLOGY_RULE_SPECS`, old
  `milestone_one` invariant registrations, blueprint validator rows,
  certification expectation arrays, operator-local validator hooks, and public
  facade exposure.
- Registered validator family and invariant family records with sealed
  identity, touched-closure applicability, required graph-read posture,
  enforcement phase, witness posture, and diagnostic projection.
- Touched-closure to Query graph touch descriptor and operating-world lowering
  that lets Query select obligations from the obligation index before
  enforcement.
- At least one honest migrated validator family and one honest invariant family
  proving the full product ladder from catalog declaration to enforcement
  receipt.
- Enforcement receipts with exact counters and structured passed/advisory/
  violation/denied/certification-only outcomes.
- Query support posture rows, support pins, budget-denial evidence, bypass
  audit rows, adoption manifests, and residue manifests.
- Source firewalls that reject manual invariant packs, direct materialization
  row reads, bridge-row peeking, cached report-row authority, and local support
  matrices as ordinary covered-lane proof.
- Public closeout and compile-fail fences proving callers cannot forge catalog
  records, selected plans, enforcement receipts, or closeout proof.
- A Milestone 10 seed that carries selected validator/invariant proof without
  claiming derived invalidation work.

## Must Preserve

- `worth-topo` remains the owner of topology legality semantics.
- Query owns graph obligation selection, dispatch, support posture, and
  executor verdict evidence. Milestone 9 consumes Milestone 8 graph-read access
  receipts and produces Query-selected legality proof.
- Query owns projection consumption for retained facts. Worth may consume typed
  fact receipts for witness interpretation; it may not reopen Query
  materialization or bridge storage.
- Validator and invariant families remain declare-once catalog records; ordinary
  operators do not name validators, invariant packs, or dirty follow-on work.
- Operating world identity remains part of selection. Branch, preview,
  authoritative, construction, and runtime-backed test workspaces may not share
  selected obligation proof by string coincidence.
- Whole-view validation remains available only for certification comparison or
  capped residue until fully deleted.
- Existing semantic validator behavior must be preserved for migrated families,
  while authority moves from global pack execution to touched-routed selected
  family proof.
- Public facades expose read-only proof/status, not constructors or mutable
  internal catalog topology.

## Acceptance Evidence

- Tests prove inventory completeness, no `keep` disposition for old authority,
  source-firewall denial for new local validator arrays, and compile-fail
  denial for forged proof products.
- Tests prove one catalog declaration applies to multiple matching touched
  closures without editing the operators or stages.
- Tests prove Query selected-obligation breadth is bounded by touched closure
  and does not scale with unrelated topology rows.
- Tests prove operating world, obligation kind, support status, covered lane,
  support pin digest, and budget digest remain visible in closeout proof.
- Tests prove missing Query access receipt/posture denies enforcement before
  graph traversal begins.
- Tests prove Query execution-backed adoption contains real executor rows and
  adoption-manifest execution proof digests.
- Tests prove projection-consuming validators and invariants carry typed Query
  consumed-fact proof and fail when implemented with direct materialization row,
  bridge-row, or cached report-row access.
- Tests prove manual invariant packs and compatibility hooks cannot satisfy
  ordinary covered topology legality without Query registration, support
  posture, and execution evidence.
- Tests prove local validator maps, private legality graphs, local graph walks,
  fabricated receipts, and string-list support pins are either deleted or
  visible as bypass audit/residue.
- Tests prove migrated validator/invariant families preserve old semantic
  results while producing new Query-selected obligation receipts and Worth
  witness rows.
- Tests prove operator closeout no longer consumes validator expectation arrays
  or static invariant packs.
- Closeout counters exactly report selected families, executed families,
  denied families, advisory rows, violation rows, access receipt count,
  caller-owned graph work count, budget denials, support pins, executor rows,
  adoption manifests, residue manifests, whole-view certification-only rows,
  deletion rows, capped residue rows, and source-firewall violations.

## Sequencing Notes

- Milestone 9 starts only after Milestone 8 has produced a public
  `WorthGraphReadAccessPlanAdoptionMilestoneNineSeed` with receipt, posture,
  counter, deletion, residue, source-firewall, bounded-execution, and cutover
  proof.
- Milestone 9 must not implement derived invalidation. It should produce the
  selected validator/invariant receipts that Milestone 10 consumes.
- The first implementation plan should choose the first migrated validator
  family by measuring the smallest real graph-structured slice that can prove
  touched applicability, access receipt use, enforcement witness, and whole-view
  leakage denial.
- Parallel folder plus hard deletion remains mandatory. If an old path remains,
  it must be capped with owner, count, blocker, and removal trigger.

## Required Self-Check

- Does the milestone solve a real structural problem or just package work
  cosmetically? It solves the authority transition from static/global
  validation to touched-routed declare-once validator and invariant catalogs.
- Is the adversarial constraint precise and load-bearing? Yes: local operations
  on large topology must not reintroduce static packs, expectation arrays, or
  broad whole-view scans.
- Does the roadmap justify this milestone now? Yes: Milestone 8 gives the
  access-plan receipts that validator/invariant selection must consume.
- Does the spec preserve crate authority boundaries? Yes: `worth-topo` owns
  topology legality and witness interpretation, Query owns access-plan proof,
  obligation selection, support posture, projection consumption, dispatch, and
  executor evidence, and `worth-kernel` owns closeout pressure only.
- Are the phases carrying most of the real design information? Yes: the design
  lives in the eight ordered phases.
- Is each phase centered on one conceptual detail or boundary? Yes: inventory,
  catalog vocabulary, routing, validator migration, invariant migration,
  enforcement receipts, cutover, and closeout.
- Does each phase contain at least two adversarial tests by default? Yes.
- Could a competent engineer map this spec into honest types, modules, and
  tests? Yes: the spec names source surfaces, product boundaries, proof
  products, counters, and denial tests.
- Does the milestone belong in this roadmap sequence, or is it out of order?
  It belongs after Milestone 8 and before Milestone 10 because invalidation must
  consume Query-selected validator/invariant obligation proof rather than rerun
  validation.
