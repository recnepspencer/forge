# Touched Graph Milestone 6: Graph-Read Access Inventory And Hard Break

> **Status:** Draft
>
> **Purpose:** replace Worth-local graph-read access folklore with a parallel
> graph-read access inventory and deletion ledger, so Milestone 7 can lower
> covered reads into Query declarations without preserving caller-owned graph
> traversal as a future dependency.

## Goal

Milestone 6 freezes the boundary between selected Query graph obligations and
covered Worth graph-read access work.

The milestone does not implement Query graph-read declarations or access-plan
adoption. It builds a new responsibility-owned graph-read access inventory and
hard-break lane beside the current Worth read surfaces, classifies every covered
local traversal, proves all unclassified read folklore is blocked, migrates
closeout/certification consumers onto the new lane, and then deletes or caps
the old graph-read adoption/bypass scaffolding.

By the end of this milestone:

- Milestone 5 selected-obligation closeout is the only accepted start point for
  covered graph-read inventory
- topology, spatial, kernel, and certification graph-read surfaces are
  classified as Query declaration candidate, deletion target, capped residue,
  certification-only support, or Query access capability gap
- old graph-read bypass/adoption scaffolding is migrated into the new
  responsibility-named lane and then deleted or mechanically capped
- local N+1 loops, ad hoc adjacency maps, hidden broad scans, local graph
  caches, local support rows, and fabricated receipts cannot masquerade as
  Query access authority
- Milestone 7 receives a concrete inventory seed for touched-authority-backed
  Query graph-read declarations

Milestone 6 does **not** close Query graph-read declarations, admitted access
plans, access receipts, validator derivation, invalidation, replay, conflict,
cache, or public diagnostics.

## Why This Milestone Exists

Milestone 5 closed selected Query graph obligations from touched authority, but
Worth still has graph-read surfaces that can teach the wrong lesson: a local
helper can be "safe" because it reads a small neighborhood today.

That is the same failure shape Milestone 5 avoided for selectors. The dangerous
version of Milestone 6 would patch existing graph-read helpers in place and let
old bypass audits, no-N+1 contracts, broad scans, local adjacency buffers, and
test receipts survive as permanent substrate.

This milestone instead performs an architectural rollover. Build the new
graph-read access inventory lane in parallel, migrate every closeout consumer
onto that lane, then delete or cap the old graph-read folklore with owner, cap,
blocker, and removal trigger.

## Governing Summaries

- `MENTALITY.md`: protects adversarial-constraint-first foundation work. This
  milestone must block caller-owned graph-read folklore before boolean, NURBS,
  extrusion, and fillet paths depend on local neighborhood reads.
- `arch_laws.md`: protects proof-bearing phase transitions. Selected
  obligations must lower into graph-read inventory rows, deletion actions, and
  Milestone 7 declaration seeds without weaker read helpers promoting
  themselves.
- `composition_laws.md`: protects responsibility-owned files. Graph-read
  inventory, bypass audit, residue, capability gaps, closeout, and seed
  construction need named homes, not one broad certification file.
- `domain_structure_laws.md`: protects visible ownership. `worth-kernel` owns
  closeout pressure, `worth-topo` owns topology read truth, `worth-spatial`
  owns spatial read truth, and `forge-query` owns graph-read access vocabulary
  and bypass/adoption proof.
- `perf_laws.md`: protects semantic-delta-bounded reads. Inventory rows must
  expose N+1 loops, broad scans, adjacency/frontier/visited/result buffers,
  hidden caches, and Query access capability gaps as cost-bearing facts.
- `touched-graph-roadmap.md`: places this milestone after selected Query
  obligations and before touched-authority-backed Query read declarations,
  because Milestone 7 needs a classified read inventory rather than another
  source-grep expedition.
- `AI_README.md`: distinguishes graph touch obligation authority from graph
  read access planning. Milestone 6 prepares the hard break; Milestones 7 and 8
  own declarations, admitted access plans, and receipts.

## Adversarial Constraint

Given a selected Query graph obligation closeout, a large topology/spatial
workspace, and a small touched graph region, every covered Worth graph-read
surface must either appear in the graph-read access inventory exactly once with
its owner, cost posture, deletion action, and Milestone 7 input shape, or fail
certification as unclassified read folklore.

Caller-owned N+1 loops, ad hoc adjacency maps, hidden broad scans, local graph
caches, local no-N+1 contracts, local support rows, fabricated read receipts,
test-only graph traversals that leak production shape, and old bypass/adoption
scaffolding must fail closed or appear as explicit capped residue with owner,
cap, blocker, and removal trigger.

No later milestone may need to preserve a Worth-local graph-read access helper
as authority after the helper has a Query declaration candidate, Query access
capability gap, or deletion row in the Milestone 6 inventory.

## Product Decision Lock

- Use the strangler migration format from Milestone 5.
- Create a new responsibility-named graph-read access inventory lane beside the
  existing `query_adoption/graph_read_access` scaffolding.
- Do not patch the old graph-read adoption path in place.
- Query owns graph-read access vocabulary, bypass audit proof, access
  requirement vocabulary, admission posture vocabulary, and receipt vocabulary.
- Worth owns inventory classification, public closeout pressure, and the seed
  that Milestone 7 consumes.
- Deletion is preferred over residue. Residue is allowed only with owner, cap,
  blocker, removal trigger, and certification preventing growth.
- Milestone 6 may name Query access capability gaps, but it must not implement
  Milestone 7 declarations or Milestone 8 access-plan adoption.

## Phase Plan

### Phase 1: Query 9.10 Graph-Read Capability Refresh

This phase freezes the Query vocabulary Milestone 6 is allowed to reference.
The implementation must start by reading the current Query graph-read access
facade and deriving a small Worth-facing capability map. This phase prevents
Worth from inventing local labels for access requirements, admission postures,
denial kinds, receipt fields, or cost counters that Query already owns.

**Relevant subsystems**
- `crates/forge-query/docs/AI_README.md`
- `crates/forge-query/docs/authoring/graph-read-access-planning.md`
- `crates/forge-query/docs/authoring/read-composition.md`
- `crates/forge-query/src/facade/exports_runtime.rs`
- `crates/forge-query/src/runtime/graph_read_access`
- `crates/forge-query/src/runtime/surface/read_receipt_accessors.rs`

**Relevant APIs**
- `derive_graph_read_access_requirements(...)`
- `try_derive_graph_read_access_requirements(...)`
- `admit_graph_read_access_for_family(...)`
- `plan_admitted_graph_read_access_for_family(...)`
- `ForgeQueryGraphReadAccessRequirementRow`
- `ForgeQueryGraphReadAccessAdmissionPosture`
- `ForgeQueryGraphReadAccessDenialKind`
- `ForgeQueryAdmittedGraphReadAccessPlan`
- `ForgeQueryReadReceipt::graph_read_access_plan_consumption()`
- `ForgeQueryReadReceipt::graph_read_access_complexity_counters()`

**Warnings**
- Do not translate Query access requirement names into Worth-local enums unless
  the enum is a read-only facade projection over Query-owned values.
- Do not treat Graph Touch Obligation Authority as graph-read access planning.
- Do not make this phase implement declarations or access plans. It only
  refreshes the vocabulary Milestone 6 needs for classification.

**Test requirements**
- `graph_read_capability_refresh_tracks_query_runtime_facade`: every Query
  graph-read access posture, denial kind, requirement kind, and receipt field
  referenced by the Worth inventory appears in a typed capability map derived
  from the current Query facade or documented Query access-planning surface.
- `worth_graph_read_inventory_rejects_local_access_vocabulary`: local Worth
  access labels such as `safe-neighborhood`, `manual-no-n-plus-one`,
  `local-adjacency-cache`, or `helper-proof` fail unless mapped to Query-owned
  vocabulary or an explicit capped residue row.
- `graph_touch_obligation_outputs_cannot_satisfy_read_access_capability`:
  selected obligations, support pins, and obligation adoption proof cannot be
  classified as graph-read access plans or receipts.

**Engineering decisions**
- The capability map lives under the new graph-read hard-break lane, not under
  old `query_adoption/graph_read_access`.
- Capability rows may store Query terminal labels for reporting, but authority
  is the Query surface they cite, not local strings.

**Open questions**
- None.

### Phase 2: Parallel Graph-Read Access Inventory Lane

This phase creates the new responsibility-owned home before touching old
callers. It should be named for the enduring responsibility, such as
`crates/worth-kernel/src/graph_read_access_inventory/`, not `v2`, `new`,
`migration`, or `phase_six`.

The new lane owns typed inventory rows, classifications, deletion actions,
capability-gap rows, capped residue rows, validation, and closeout counters.
It is allowed to inspect old read surfaces; it is not allowed to execute
ordinary graph reads.

**Relevant subsystems**
- `crates/worth-kernel/src/graph_read_access_inventory/`
- `crates/worth-kernel/src/query_obligation_selection/closeout`
- `crates/worth-kernel/src/query_adoption/graph_read_access`
- `crates/worth-kernel/src/certification/public_facade_contracts`
- `crates/forge-query/src/consumer_kit`

**Relevant APIs**
- `WorthQueryObligationSelectionMilestoneFiveCloseout`
- `WorthQueryObligationSelectionMilestoneSixSeed`
- `ForgeQueryGraphReadBypassReport`
- `ForgeQueryGraphReadBypassAdoptionProof`
- `ForgeQueryGraphReadBypassResidueManifest`
- `ForgeQueryBoundaryAuditSourceInventory`

**Warnings**
- Do not edit old graph-read adoption files into the desired final shape.
  Build the new lane beside them and migrate consumers after parity.
- Do not let inventory rows become execution authority. They classify what must
  migrate, delete, or gap; they do not read topology.
- Do not use source paths as the only proof. Rows need owner, classification,
  cost posture, deletion action, cap/blocker/removal trigger when applicable,
  and Milestone 7 input shape.

**Test requirements**
- `parallel_graph_read_inventory_lane_accepts_milestone_five_seed_only`: the
  new inventory closeout can be constructed from the Milestone 5 selected
  obligations seed and rejects raw selected counts, raw digests, local support
  rows, and fabricated receipt strings.
- `inventory_row_requires_owner_cost_posture_and_deletion_action`: every
  graph-read inventory row fails validation unless it has a source path,
  owner, classification, cost posture, deletion action, current caller, and
  Milestone 7 disposition.
- `capped_graph_read_residue_requires_cap_blocker_and_removal_trigger`: capped
  residue rows fail validation if any cap, blocker, owner, or removal trigger
  is missing or empty.

**Engineering decisions**
- Inventory types should be sealed enough that certification can read them but
  external callers cannot forge closeout authority.
- Classifications should include at least: `QueryDeclarationCandidate`,
  `DeletionTarget`, `CappedResidue`, `CertificationOnlySupport`,
  `QueryAccessCapabilityGap`, and `OutOfScopeNonGraphRead`.

**Open questions**
- None.

### Phase 3: Covered Worth Graph-Read Surface Inventory

This phase populates the parallel lane with every current Worth graph-read
surface that could influence topology, spatial evidence, boolean preparation,
kernel workload composition, or certification. The inventory must include
production and test-support roots when test support preserves production-shaped
read folklore.

**Relevant subsystems**
- `crates/worth-topo/src/projection/read_views/domain`
- `crates/worth-topo/src/projection/runtime_boundary/read_execution`
- `crates/worth-kernel/src/query_adoption/graph_read_access`
- `crates/worth-kernel/src/workload_composition`
- `crates/worth-kernel/src/binding`
- `crates/worth-spatial/src/workload_platform/planar_boolean_loop_reconstruction`
- `crates/worth-spatial/src/workload_platform/planar_boolean_events`
- `crates/worth-spatial/src/workload_platform/evidence_ledger`

**Relevant APIs**
- `TopologyReadLedger`
- `TopologyReadGraphAccessProof`
- `TopologyNoNPlusOneContract`
- `TopologyReadRequestFamily`
- `TopologyReadExecutionTarget`
- `execute_shared_neighborhood_read(...)`
- `execute_local_rewire_read(...)`
- `PlanarBooleanFragmentContinuationIndex`
- `SpatialEvidenceLookupProduct`
- `current_worth_kernel_construction_graph_read_access_adoption()`

**Warnings**
- Do not classify only the obvious production files. Test fixtures that encode
  local graph traversal, local neighborhood lookup, or fabricated receipt
  semantics must be inventoried or explicitly marked certification-only.
- Do not count a local no-N+1 row as Query access proof. It is either
  certification-only support, deletion target, or a Query declaration/access
  candidate.
- Do not classify broad boolean scans as harmless because they are currently
  small. They are exactly the workload Milestone 6 is supposed to expose.

**Test requirements**
- `graph_read_inventory_covers_topology_spatial_kernel_and_test_surfaces`:
  every covered source root exporting graph-read execution, local neighborhood
  view construction, no-N+1 contract rows, graph-read bypass adoption, spatial
  evidence lookup, or planar boolean continuation neighborhood lookup appears
  in the typed inventory exactly once.
- `unclassified_graph_read_surface_fails_inventory`: adding a production or
  production-shaped test source with relation loops, adjacency maps, frontier
  sets, broad scans, local caches, or fabricated read receipts fails
  certification until a row classifies it.
- `test_support_cannot_hide_production_read_folklore`: test-only graph-read
  helpers may be certification-only fixtures, but they cannot preserve a helper
  shape that production callers could copy as authority.

**Engineering decisions**
- Inventory rows should distinguish source truth owner from closeout owner:
  topology read truth remains `worth-topo`, spatial evidence read truth remains
  `worth-spatial`, and cross-crate enforcement remains `worth-kernel`.
- The inventory should record whether the row feeds Milestone 7 declaration
  work, Milestone 8 access-plan adoption work, or deletion-only cleanup.

**Open questions**
- Whether any binding test support roots should be marked out of scope because
  they are not graph reads, or included because they encode local replacement
  neighborhood semantics used by later topology work.

### Phase 4: Selected-Obligation Seed Binding And Access-Scope Classification

This phase binds the Milestone 5 selected-obligation closeout into graph-read
inventory scope. A graph-read row is not valid merely because source code was
found. It must state which selected obligation, touched authority product,
touch descriptor digest, read family, or certification proof makes the read
relevant to touched-graph migration.

The output is a typed scope layer that separates touched authority inputs,
selected Query graph obligations, graph-read declaration candidates,
graph-read access requirement candidates, and execution receipt expectations.

**Relevant subsystems**
- `crates/worth-kernel/src/query_obligation_selection/closeout`
- `crates/worth-kernel/src/graph_read_access_inventory`
- `crates/worth-topo/src/projection/read_views/domain/read_proof`
- `crates/worth-spatial/src/workload_platform/planar_boolean_loop_reconstruction`
- `crates/forge-query/docs/AI_README.md`

**Relevant APIs**
- `WorthQueryObligationSelectionMilestoneFiveCloseout`
- `WorthQueryObligationSelectionMilestoneSixSeed`
- `TopologyReadGraphAccessProof`
- `TopologyNoNPlusOneContract`
- `ForgeQueryGraphTouchObligationAdoptionProof`
- `ForgeQueryReadReceipt`

**Warnings**
- Do not let an inventory row exist only because a source file matched a
  string search. Source discovery is input; selected-obligation binding is
  scope authority.
- Do not treat selected obligations as graph-read declarations. They are the
  reason a read must be inventoried, not the executable read contract.
- Do not let topology or spatial fixtures bypass scope binding by claiming
  that they are "only tests" while preserving production-shaped graph reads.

**Test requirements**
- `graph_read_inventory_rows_require_selected_obligation_or_certification_scope`:
  every covered row requires a selected obligation relation, touched authority
  digest, graph-read proof family, or explicit certification-only scope.
- `selected_obligation_seed_cannot_be_relabelled_as_read_access_plan`: selected
  obligation closeout values fail if they are used as declarations, admitted
  access plans, access receipts, or no-N+1 execution proof.
- `out_of_scope_graph_read_rows_must_explain_non_graph_read_boundary`: rows
  classified out of scope require a reason tied to non-graph-read behavior, not
  convenience, uncertainty, or missing implementation time.

**Engineering decisions**
- Scope classification should be a separate typed step before deletion and
  capability-gap decisions, because false scope is the easiest way for old
  graph folklore to survive.
- Certification-only rows may remain only when they prove the boundary; they
  cannot be the boundary.

**Open questions**
- Whether Milestone 5's seed type should be extended directly, or whether
  Milestone 6 should wrap it with a graph-read-specific seed that preserves
  Milestone 5 as immutable input.

### Phase 5: Consumer Kit Bypass Audit And Residue Ledger Rollover

This phase moves graph-read bypass proof out of the old
`query_adoption/graph_read_access` scaffolding and into the new inventory lane.
The new lane must use Query Consumer Kit-backed bypass audit and adoption proof,
not Worth-local source greps, local support folklore, or fixture receipts.

The old graph-read adoption module may remain only long enough to prove parity
and call-site migration. It cannot be expanded, renamed into the new system, or
used as the future home.

**Relevant subsystems**
- `crates/worth-kernel/src/query_adoption/graph_read_access`
- `crates/worth-kernel/src/graph_read_access_inventory/bypass_audit`
- `crates/worth-kernel/src/graph_read_access_inventory/residue`
- `crates/forge-query/src/consumer_kit`
- `scripts/ci/check_workspace_rust_line_caps.sh`

**Relevant APIs**
- `graph_read_bypass_audit(...)`
- `graph_read_bypass_adoption(...)`
- `ForgeQueryGraphReadBypassReport`
- `ForgeQueryGraphReadBypassAdoptionProof`
- `ForgeQueryGraphReadBypassResidueManifest`
- `ForgeQueryBoundaryAuditSourceInventory`

**Warnings**
- Do not keep the old construction-only adoption root as the real audit. The
  new inventory must cover topology, spatial, kernel, and production-shaped
  test support roots.
- Do not allow residue to be described as future work without mechanical caps.
  Every residue row needs owner, cap, blocker, removal trigger, and growth
  prevention.
- Do not preserve broad source-grep bypass reports as proof when Query Consumer
  Kit can express the audit boundary.

**Test requirements**
- `graph_read_bypass_rollover_matches_old_adoption_until_cutover`: during the
  migration window, the new lane reports every old graph-read bypass/adoption
  row plus the newly covered roots; after cutover, old-lane exports are not
  required by public closeout.
- `graph_read_residue_manifest_cannot_grow_without_cap_update`: adding a new
  capped residue row requires an explicit cap change, owner, blocker, and
  removal trigger.
- `consumer_kit_bypass_audit_covers_all_inventory_roots`: every source root
  listed in the graph-read inventory contributes to a Consumer Kit-backed
  source inventory or explicit non-source proof row.

**Engineering decisions**
- The new lane should own its own source inventory module so Worth can audit
  graph-read access residue without depending on old adoption module names.
- The bypass audit result should be input to closeout validation, not an
  optional report printed after the milestone claims success.

**Open questions**
- Whether old construction-specific graph-read adoption test names should be
  deleted immediately in this phase or preserved until Phase 7 as parity
  witnesses.

### Phase 6: Query Access Gap And Declaration Candidate Ledger

This phase converts inventory rows into Milestone 7 starting material. Every
covered row must become one of three shapes: a Query graph-read declaration
candidate, a Query access capability gap, or a deletion-only cleanup item.

The phase does not implement declarations. It records the read family, touched
authority input, required Query requirement rows, expected posture pressure,
and the reason Query can or cannot express the read today.

**Relevant subsystems**
- `crates/worth-kernel/src/graph_read_access_inventory/candidates`
- `crates/worth-kernel/src/graph_read_access_inventory/capability_gaps`
- `crates/worth-topo/src/projection/read_views/domain`
- `crates/worth-spatial/src/workload_platform/planar_boolean_loop_reconstruction`
- `crates/forge-query/src/runtime/graph_read_access`

**Relevant APIs**
- `ForgeQueryGraphReadAccessRequirementRow`
- `ForgeQueryGraphReadAccessAdmissionPosture`
- `ForgeQueryGraphReadAccessDenialKind`
- `ForgeQueryAdmittedGraphReadAccessPlan`
- `ForgeQueryReadReceipt::graph_read_access_complexity_counters()`
- `TopologyReadRequestFamily`
- `PlanarBooleanFragmentContinuationIndex`

**Warnings**
- Do not mark a row as "keep local" because Query lacks a capability. That is
  a Query access capability gap with owner, cap, blocker, and removal trigger.
- Do not make broad boolean frontier scans a permanent local exception. They
  must become declaration candidates, capability gaps, or deletion targets.
- Do not store handwritten Query requirement rows as authority. Requirement
  rows are target vocabulary for Milestone 7 and 8, not Milestone 6 execution
  proof.

**Test requirements**
- `graph_read_declaration_candidate_requires_touched_authority_and_read_family`:
  declaration-candidate rows require touched authority input, read family,
  expected requirement vocabulary, owner, and Milestone 7 lowering target.
- `query_access_gap_requires_owner_cap_blocker_and_query_trigger`: capability
  gaps cannot be recorded without owner, cap, missing Query capability,
  blocker, removal trigger, and expected denial/posture vocabulary.
- `broad_boolean_and_dense_frontier_reads_cannot_be_keep_local_rows`: broad
  boolean scans, dense frontier reads, local adjacency caches, and materialized
  local continuation indexes fail if classified as unconstrained local keepers.

**Engineering decisions**
- Candidate and gap rows should be separate from inventory discovery rows so
  review can verify that every source row has exactly one disposition.
- The Milestone 7 seed should carry only candidate/gap/deletion closeout data,
  not old source-scan implementation details.

**Open questions**
- Whether `PlanarBooleanFragmentContinuationIndex` needs a single broad Query
  capability gap row or several rows split by adjacency, ordering, frontier,
  and materialization requirements.

### Phase 7: Hard Delete Old Graph-Read Adoption Scaffolding And Public Firewall

This phase performs the architectural rollover. After the parallel inventory
lane proves coverage and closeout parity, public Worth closeout and
certification must stop depending on `query_adoption/graph_read_access`.

The old module is deleted if all behavior has moved. If deletion is blocked,
the remaining path is capped as residue with a public firewall that prevents
new callers, new rows, new exports, and new fixture receipts from treating it
as authority.

**Relevant subsystems**
- `crates/worth-kernel/src/query_adoption/graph_read_access`
- `crates/worth-kernel/src/graph_read_access_inventory`
- `crates/worth-kernel/src/certification/public_facade_contracts`
- `crates/worth-kernel/src/lib.rs`
- `crates/worth-kernel/tests`

**Relevant APIs**
- new Milestone 6 graph-read inventory closeout facade
- new Milestone 6 graph-read inventory seed facade
- old `current_worth_kernel_construction_graph_read_access_adoption()`
- old graph-read bypass/adoption/residue exports
- Worth public certification facade contracts

**Warnings**
- Do not keep compatibility aliases for old graph-read adoption authority.
  Compatibility is how the old architecture survives.
- Do not leave provenance names such as `phase_17`, `construction`, `old`,
  `new`, `v2`, or `migration` in production module paths.
- Do not delete old files before the new lane owns closeout, source inventory,
  residue, candidate/gap rows, and public certification proof.

**Test requirements**
- `old_graph_read_adoption_scaffolding_is_not_public_authority`: public Worth
  callers cannot import the old graph-read adoption closeout, bypass report,
  residue manifest, or support rows after rollover.
- `fabricated_graph_read_receipts_and_local_support_rows_fail_firewall`: local
  support rows, copied counters, raw strings, and fabricated `ForgeQueryReadReceipt`
  stand-ins cannot satisfy Milestone 6 graph-read inventory closeout.
- `old_graph_read_path_cannot_gain_new_rows`: if any old path remains capped,
  attempts to add new sources, residues, or adoption rows under the old module
  fail certification.

**Engineering decisions**
- Prefer deletion. Mechanical capping is acceptable only when deletion would
  break a public consumer that the same phase explicitly names and schedules
  for removal.
- Public facades should expose the new responsibility name, not migration
  vocabulary.

**Open questions**
- None.

### Phase 8: Cross-Crate Closeout And Milestone 7 Readiness

This phase closes Milestone 6 as a cross-crate proof product. Closeout must
state exact counts for classified read surfaces, declaration candidates,
capability gaps, deletion targets, certification-only rows, and capped residue.

It also emits the Milestone 7 seed: every declaration candidate or capability
gap must carry selected-obligation provenance, touched-authority provenance,
source owner, Query vocabulary pressure, cost posture, and removal/declaration
trigger.

**Relevant subsystems**
- `crates/worth-kernel/src/graph_read_access_inventory/closeout`
- `crates/worth-kernel/src/graph_read_access_inventory/seed`
- `crates/worth-topo/src/projection/read_views/domain`
- `crates/worth-spatial/src/workload_platform`
- `_docs/worth/touched-graph-roadmap.md`

**Relevant APIs**
- Milestone 6 graph-read inventory closeout facade
- Milestone 7 graph-read declaration seed facade
- `WorthQueryObligationSelectionMilestoneFiveCloseout`
- `ForgeQueryGraphReadBypassAdoptionProof`
- `ForgeQueryGraphReadBypassResidueManifest`
- `ForgeQueryGraphReadAccessRequirementRow`

**Warnings**
- Do not claim Query declarations, admitted access plans, execution receipts,
  validator derivation, invalidation, replay, conflict, cache, or diagnostics.
  Those belong to later milestones.
- Do not close with "all known surfaces" language. The closeout must be tied
  to enumerated roots, typed rows, and bypass audit coverage.
- Do not allow residue counts to be implicit. Every remaining residue must be
  countable and capped.

**Test requirements**
- `milestone_six_closeout_requires_exact_inventory_and_disposition_counts`:
  closeout fails unless every covered row has exactly one disposition and the
  reported counts match the typed inventory.
- `milestone_seven_seed_contains_no_old_graph_read_folklore`: the Milestone 7
  seed contains declaration candidates and capability gaps, but no old adoption
  rows, local support rows, fabricated receipts, or caller-owned graph cache
  authority.
- `milestone_six_closeout_refuses_later_milestone_claims`: closeout fails if
  it claims Query declarations, admitted graph-read access plans, validator
  derivation, invalidation, replay, conflict, cache, or public diagnostics are
  complete.

**Engineering decisions**
- The closeout type should be the only sanctioned input to Milestone 7.
- Roadmap status should be updated only after closeout has exact counts and
  the old lane is deleted or capped.

**Open questions**
- None.

## Must Ship

- a responsibility-named parallel graph-read access inventory lane
- typed inventory rows with owner, source, classification, scope authority,
  cost posture, deletion action, and Milestone 7 disposition
- Consumer Kit-backed graph-read bypass audit and residue manifest under the
  new lane
- declaration-candidate and Query access capability-gap ledgers for Milestone 7
- deletion or mechanical cap of old `query_adoption/graph_read_access`
  scaffolding
- closeout proof with exact row counts and a Milestone 7 seed

## Must Preserve

- Query owns graph-read access vocabulary, access requirement vocabulary,
  admission posture vocabulary, denial vocabulary, and receipt vocabulary.
- Worth owns inventory classification and deletion pressure only.
- Selected obligations remain touched-authority evidence; they do not become
  graph-read declarations or access receipts.
- Local topology and spatial read proof may certify boundaries, but they may
  not become Query access authority.
- Broad boolean, dense frontier, local cache, and materialization pressure must
  remain visible as cost-bearing inventory facts.

## Acceptance Evidence

- `cargo fmt` for touched Worth crates
- focused Worth tests for Milestone 6 graph-read inventory row validation,
  scope binding, bypass rollover, candidate/gap ledgers, public firewall, and
  closeout
- focused Query integration or compile-fail tests proving Worth cannot forge
  Query graph-read access vocabulary, receipts, or access-plan authority
- source scans proving old graph-read adoption authority is deleted or capped
  and no migration/v2/old/new production module names survive
- roadmap update linking this spec and preserving the Milestone 6 -> Milestone
  7 handoff

## Sequencing Notes

Milestone 6 is intentionally more aggressive than an in-place refactor. The
implementation should build the new lane, migrate proof consumers, validate
coverage, and then delete or cap old scaffolding. Any phase that starts by
reshaping the old `query_adoption/graph_read_access` folder is off-plan.

The safest implementation rhythm is:

1. build the new lane with hostile validation
2. populate inventory and prove coverage
3. roll over bypass/residue proof
4. emit candidate/gap seed for Milestone 7
5. delete or cap the old lane
6. close with exact counts

That order is the point. It prevents Worth from slowly converting old helper
folklore into permanent architecture.
