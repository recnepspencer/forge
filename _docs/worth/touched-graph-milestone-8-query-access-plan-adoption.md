# Milestone 8: Query Access Plan Adoption And Receipt Hard Break

> **Status:** Draft
>
> **Purpose:** adopt Query graph-read access plans as the only execution proof
> for covered Worth graph reads, produce receipt-bearing closeout evidence, and
> delete or cap the remaining local graph-read execution folklore before
> validator and invariant routing begins.

## Goal

Milestone 8 consumes the Milestone 7 declaration closeout and turns covered
graph-read declarations into admitted Query access plans, typed access postures,
execution counters, and receipts. It builds a parallel access-plan adoption lane
beside any remaining Worth-local execution helpers, migrates covered reads into
that lane, and hard-deletes or caps the displaced local loops, caches, broad read
helpers, fabricated receipts, and compatibility wrappers.

## Why This Milestone Exists

Milestone 7 made covered graph-read intent declarative. Milestone 8 makes that
intent executable without letting operators, validators, or spatial stages
recover local graph traversal authority. The next milestones need graph-read
receipts as proof inputs; they must not inherit caller-owned loops, broad scans,
or fake receipt summaries.

## Governing Summaries

- `MENTALITY.md`: protects adversarial-constraint-first engineering. This
  milestone must solve execution authority before validators, invalidation, and
  replay depend on graph reads.
- `arch_laws.md`: protects proof-bearing phase chains. Milestone 8 must consume
  the Milestone 7 seed and produce a stronger receipt-bearing seed; no weaker
  local execution artifact may promote itself.
- `composition_laws.md`: protects named responsibilities. Access-plan
  admission, posture routing, execution receipts, counters, deletion firewalls,
  and closeout proof need separate homes rather than one broad read helper.
- `domain_structure_laws.md`: protects visible ownership. Query owns plan
  admission, access posture, receipt vocabulary, and execution counters; Worth
  owns touched-authority inputs, reference consumer migration, and closeout
  pressure.
- `perf_laws.md`: protects semantic-delta-bounded execution. Access-plan
  execution breadth must be explained by touched authority, selected read
  family, and Query counters rather than hidden traversal or broad scan cost.
- `touched-graph-roadmap.md`: places this milestone after declaration closeout
  and before validator/invariant routing because later registered families need
  real graph-read receipts, not declarations alone.

## Adversarial Constraint

Given a large Worth topology, long boolean or future curved-operation chains,
and small touched graph regions, every covered graph read must either execute
through an admitted Query access plan with receipt counters bounded by the
touched authority and selected read family, or fail before expensive traversal
with a typed Query denial or required posture.

If any covered path can still satisfy a later validator, invariant,
invalidation, evidence, replay, conflict, cache, public proof, or diagnostic
consumer through a caller-owned loop, local adjacency map, broad helper scan,
unbounded automatic index, fabricated receipt, manual plan list, operator
strategy hint, local execution-mode switch, or compatibility wrapper, the
milestone has failed.

## Product Decision Lock

- Use the parallel migration plus hard deletion format.
- Build a new responsibility-named graph-read access-plan adoption lane beside
  remaining Worth-local graph-read execution helpers; do not refactor old helper
  paths in place.
- Consume `WorthGraphReadAccessDeclarationMilestoneEightSeed` as the only
  production start point for covered access-plan adoption.
- Query owns graph-read access plans, access posture vocabulary, plan
  consumption, receipt fields, and execution counters.
- Worth owns migration of reference consumers, old-path deletion pressure,
  capped residue reporting, public closeout, and the Milestone 9 seed.
- The governing Query rule applies literally: declare graph-read intent once,
  lower it once, then execute or inspect it only through canonical
  runtime-owned artifacts.
- Registered read families must route through the adoption lane automatically.
  Operators and stages may provide touched authority, but they may not carry
  manual access-plan lists, plan hints, local execution modes, or fallback
  strategy switches.
- The execution receipt is the canonical graph-read boundary artifact. A raw
  plan, local summary, diagnostic row, support row, or unexecuted posture cannot
  satisfy later validators, invalidation, replay, conflict, cache, public proof,
  or diagnostics.
- The plan solver must decide support posture before traversal begins. The
  executor may consume a lowered plan or typed denial; it may not rediscover
  strategy, build hidden indexes, or reinterpret caller hints during execution.
- Deletion is preferred over residue. Residue is allowed only with owner, cap,
  blocker, removal trigger, and certification preventing growth.
- Milestone 8 may execute covered graph reads through Query access plans and
  may produce graph-read receipts. It must not derive validators, invariants,
  invalidation plans, replay scopes, conflict proof, cache proof, or public
  diagnostics beyond the closeout evidence needed by Milestone 9.

## Implicit Requirements Made Explicit

- Access-plan adoption is not a one-time migration table. It is the first
  executable slice of the declare-once touched-graph runtime: one registered
  read-family declaration must apply to every matching touched authority without
  editing those operators or stages.
- Query support posture is a control-plane decision. Inline indexed access,
  bounded ephemeral indexing, paged streaming, persistent-index-required,
  async-materialization-required, store-backed-required,
  access-capability-registration-required, and denial must be selected before
  any edge traversal, allocation-heavy index construction, or broad scan starts.
- Receipt identity must be deterministic and authority-rich. It must include
  touched authority identity, selected read family, requirement row, support
  posture, plan identity, execution basis or snapshot, policy or tenant
  narrowing where present, and structural counter digest.
- Ephemeral indexes are allowed only as lifecycle-scoped execution products with
  explicit resident-byte, row-count, and disposal counters. Persistent or
  restart-stable indexing requires an admitted Query capability; Worth may not
  create hidden background index authority to make execution look complete.
- Bulk or grouped adoption must preserve domain cardinality. The lane may batch
  compatible plan admissions and executions when Query can prove the structural
  basis, but it may not force every graph read through scalar caller loops that
  hide breadth and destroy amortization.
- Operational receipts and rich diagnostics have separate lifecycles. Milestone
  8 must ship the receipt proof needed by later milestones, but diagnostic
  materialization remains policy-gated future work unless the phase explicitly
  names the closeout evidence.

## Phase Plan

### Phase 1: Milestone 8 Seed Admission And Execution Folklore Inventory

This phase freezes the start boundary. The Milestone 7 closeout seed is the only
accepted input, and every current Worth graph-read execution surface is
classified before the new adoption lane can execute anything.

**Relevant subsystems**
- `worth-kernel` graph-read declaration closeout
- existing topology, spatial, kernel, and test graph-read execution helpers
- public facade contract fixtures
- source firewall and deletion ledger support

**Relevant APIs**
- `WorthGraphReadAccessDeclarationMilestoneEightSeed`
- Milestone 7 closeout counters, declaration catalog identity, read-family
  identities, requirement-row evidence, gap rows, and deletion reports
- Query receipt field names from roadmap Milestone 8:
  `graph_read_access_plan_consumption`, `ephemeral_graph_index_receipt`,
  `graph_read_streaming_receipt`, and `live_graph_read_access`

**Warnings**
- Do not accept a declaration closeout digest, read-family digest, requirement
  row digest, or raw receipt as the Milestone 8 start point.
- Do not start by editing old graph-read helpers. Inventory first, build the
  new lane second, then migrate and delete.
- Do not mark an old helper as keep. It must be migrate, delete, capped
  residue, or Query-gap.

**Test requirements**
- `milestone_eight_seed_is_the_only_access_plan_adoption_start`: raw digests,
  raw declaration rows, raw receipts, and Milestone 6/7 pre-closeout products
  cannot enter the adoption lane.
- `execution_folklore_inventory_has_no_keep_rows`: every local loop, adjacency
  map, broad helper scan, fabricated receipt, local cache, and compatibility
  wrapper is classified as migrate, delete, cap, or Query-gap.
- `inventory_rows_preserve_source_identity`: same behavior under different
  source paths yields different inventory identity so deletion cannot collapse
  unrelated old paths.

**Engineering decisions**
- The adoption lane starts from the Milestone 8 seed, not from current operator
  call sites.
- Inventory rows must carry old source path, owner, current caller,
  displacement target, migration target, deletion trigger, and blocker.
- The old-path inventory is closeout pressure, not execution authority.

**Open questions**
- None.

### Phase 2: Parallel Access-Plan Adoption Lane

This phase creates the new product lane that owns plan adoption. It lowers
Milestone 7 read-family and requirement evidence into Query-owned access-plan
admission attempts without executing old Worth traversal.

**Relevant subsystems**
- new `graph_read_access_plan_adoption` lane under `worth-kernel`
- Query graph-read access-plan admission
- Milestone 7 declaration closeout seed
- access-plan adoption errors and counters

**Relevant APIs**
- `WorthGraphReadAccessDeclarationMilestoneEightSeed`
- `ForgeQueryReadFamily`
- `ForgeQueryGraphReadAccessRequirementRow`
- `ForgeQueryGraphReadAccessAdmission`
- Query access-plan admission or current Query equivalent

**Warnings**
- Do not create a Worth-owned access-plan mirror.
- Do not let the executor infer plan strategy from local caller hints.
- Do not fabricate Query admission when Query currently reports a required
  posture or denial.

**Test requirements**
- `adoption_lane_consumes_structured_milestone_eight_seed_rows`: plan admission
  consumes structured read-family identities and requirement-row evidence, not
  digest strings or local copies.
- `registered_read_family_routes_matching_touched_authorities_without_operator_edits`:
  one registered read-family adoption rule applies to multiple matching touched
  authorities without changing the operators or stages that produced them.
- `unsupported_access_plan_shapes_become_typed_postures`: missing persistent
  index, streaming, async materialization, store-backed support, and capability
  registration produce typed rows instead of local fallback traversal.
- `raw_query_receipt_cannot_seed_plan_adoption`: public and crate-local
  compile-fail fixtures reject receipt-first adoption.

**Engineering decisions**
- The new lane should expose a closeout object, an adoption attempt ledger, an
  access posture report, and a Milestone 9 seed.
- Admission attempts are proof products. They may be executed only after the
  Query plan is admitted or a bounded ephemeral posture is explicitly selected.
- Query denial and required-posture rows are first-class outputs, not TODOs.
- The lane should be modeled as declaration catalog plus touched-authority
  routing plus Query admission, not as per-caller migration glue.

**Open questions**
- Name the exact Query access-plan admission function once implementation reads
  current `forge-query` APIs.

### Phase 3: Access Posture Matrix And Gap Cap Ledger

This phase freezes the posture vocabulary for every covered graph-read shape.
Every declaration becomes an admitted plan, a required posture, a denial, or a
typed gap with cap and removal trigger.

**Relevant subsystems**
- access-plan adoption posture matrix
- Query capability gap ledger
- Milestone 7 admission and carried requirement gaps
- closeout counters

**Relevant APIs**
- Query access posture vocabulary
- Query denial vocabulary
- Milestone 7 admission capability gap rows
- Milestone 8 access posture report

**Warnings**
- Do not soften `persistent_index_required`, `async_materialization_required`,
  `store_backed_required`, or `access_capability_registration_required` into
  "retry with a local helper."
- Do not treat denial as authorization to traverse locally.
- Do not count a required posture as an execution receipt.
- Do not allow execution to select posture after traversal has already started.

**Test requirements**
- `every_requirement_row_has_exactly_one_access_posture`: each requirement row
  from the Milestone 8 seed maps to exactly one admitted, required, denied, or
  gap posture.
- `posture_cap_growth_requires_ledger_update`: adding a new missing Query
  support family fails unless the cap ledger names owner, cap, blocker, expected
  denial, and removal trigger.
- `denied_posture_does_not_emit_execution_receipt`: denied or required-posture
  rows cannot populate plan-consumption or graph-read receipt fields.
- `posture_selection_precedes_expensive_work`: required, denied, and unsupported
  rows are emitted before edge traversal, dense frontier allocation, streaming
  page creation, or index construction begins.

**Engineering decisions**
- Posture rows must carry source declaration identity, requirement row identity,
  Query family identity, expected receipt posture, and support owner.
- Gap caps are fixed by a ledger, not by the observed gap rows themselves.
- The posture matrix is consumed by execution phases and closeout; it is not a
  display-only diagnostic artifact.
- The posture matrix is the control-plane output consumed by execution. The
  executor may not downgrade or reinterpret it.

**Open questions**
- None.

### Phase 4: First Vertical Slice Migration Through Query Plans

This phase migrates the first covered graph-read execution slice through the new
adoption lane end to end. The slice should be narrow enough to verify deeply but
real enough to displace an actual local loop or helper.

**Relevant subsystems**
- one topology or kernel reference graph-read consumer selected from the
  inventory
- Query access-plan execution
- receipt projection and counter report
- old-path deletion ledger

**Relevant APIs**
- admitted Query graph-read access plan
- graph-read plan consumption receipt
- touched authority digest and selected read-family identity
- migration inventory row for the selected old helper

**Warnings**
- Do not pick a toy fixture as the first migration if a production reference
  consumer exists.
- Do not keep the old helper as a parity witness after the Query path proves
  execution and receipt parity.
- Do not let the migrated slice call Query after local traversal already solved
  the read.

**Test requirements**
- `first_migrated_slice_receipt_matches_query_plan_identity`: the receipt
  references the admitted plan, touched authority, read-family identity, and
  requirement row consumed for the migrated slice.
- `first_migrated_slice_receipt_names_execution_basis`: the receipt records the
  snapshot, graph basis, operating world, authority epoch, or equivalent Query
  basis needed to prevent stale read proof from feeding later milestones.
- `old_slice_helper_is_deleted_or_capped_after_cutover`: once parity is proven,
  the old helper path is absent or appears as capped residue with owner,
  blocker, cap, and removal trigger.
- `local_loop_after_plan_admission_fails_source_firewall`: reintroducing the
  displaced local loop or helper fails certification.

**Engineering decisions**
- The first vertical slice is the proof template for later slices.
- Receipt parity must prove the Query path did the graph work, not merely that
  the final rows match.
- The old helper may be used only before cutover and only as a bounded parity
  witness inside tests or migration support.
- Raw admitted plans are not later proof. Only executed receipt rows or visible
  required/denied postures may flow into the closeout seed.

**Open questions**
- Select the first production consumer during implementation after reading the
  current graph-read inventory rows.

### Phase 5: Spatial, Dense Frontier, And Required-Posture Slices

This phase migrates or explicitly postures the nontrivial covered read families:
spatial evidence graph reads, dense frontier reads, broad boolean predicates,
and any kernel-level reads that cannot honestly execute inline yet.

**Relevant subsystems**
- spatial touch authority graph-read consumers
- dense frontier and broad predicate access families
- Query streaming, persistent index, async materialization, and store-backed
  posture support
- migration inventory rows for non-topology reference consumers

**Relevant APIs**
- spatial read-family identities from Milestone 7
- Query access posture rows for streaming, persistent index, async
  materialization, store-backed support, and capability registration
- Query graph-read receipts when a plan is admitted

**Warnings**
- Do not turn broad boolean or dense frontier reads into unbounded ephemeral
  indexes just to make execution green.
- Do not let spatial evidence reads fall back to raw evidence vectors or broad
  stage scans.
- Do not conflate "required posture is visible" with "execution is complete."

**Test requirements**
- `spatial_graph_reads_use_query_plan_or_required_posture`: every covered
  spatial read either executes through Query receipt proof or remains visible as
  a typed required posture.
- `spatial_and_dense_reads_do_not_scalarize_batch_admission`: grouped covered
  reads preserve batch or grouped admission evidence instead of forcing
  caller-owned loops over one read at a time.
- `dense_frontier_read_cannot_use_unbounded_ephemeral_index`: dense reads must
  deny, stream, require persistence, or execute with explicit bounded counters;
  hidden automatic indexes fail.
- `broad_boolean_predicate_does_not_degrade_to_whole_graph_scan`: broad
  predicate reads must report a Query posture or bounded plan counters, not a
  local scan.

**Engineering decisions**
- Required posture rows are successful Milestone 8 outputs when Query cannot
  honestly execute the read yet.
- Spatial evidence lookup products remain Milestone 11 work; this phase only
  handles graph-read access plan adoption for covered spatial graph reads.
- Dense and broad reads should prefer explicit denial or required posture over
  expensive execution without real Query support.
- Bounded ephemeral execution must report lifecycle scope, resident bytes,
  row-count breadth, disposal proof, and the reason persistence was not required.

**Open questions**
- None.

### Phase 6: Execution Counters And Receipt Accounting

This phase freezes the measurement boundary for every admitted plan execution.
The receipt must explain the work performed and prove no caller-owned graph work
occurred outside Query.

**Relevant subsystems**
- Query graph-read access counters
- Worth adoption closeout counters
- receipt projection
- no-local-work certification

**Relevant APIs**
- graph-read access plan consumption receipt
- ephemeral graph index receipt
- graph read streaming receipt
- live graph read access receipt
- Query counters for candidate roots, touched nodes, touched edges, frontier
  width, visited breadth, dedup breadth, resident bytes, page count, fallback
  count, and local-work count

**Warnings**
- Do not report elapsed time as a substitute for structural counters.
- Do not hide local fallback under a zero Query fallback count.
- Do not aggregate counters so broadly that Milestone 9 cannot associate
  validators with the read receipt that fed them.
- Do not merge operational receipt identity with rich diagnostics policy.

**Test requirements**
- `receipt_counters_match_touched_breadth_for_bounded_slice`: candidate roots,
  touched nodes, touched edges, frontier width, visited breadth, and dedup
  breadth match the selected touched authority and read-family shape.
- `no_caller_owned_graph_work_counter_stays_zero`: migrated slices fail if any
  caller-owned loop, adjacency lookup, broad scan, or local cache is counted.
- `receipt_identity_changes_when_plan_or_touch_changes`: changing admitted plan
  identity or touched authority changes the receipt identity.
- `same_canonical_inputs_produce_same_receipt_identity`: repeated execution
  with the same touched authority, read family, requirement row, posture, plan,
  basis, policy narrowing, and counter contract produces deterministic receipt
  identity.
- `batched_execution_reports_per_read_and_aggregate_counters`: batch execution
  records per-read receipt counters and aggregate batch counters without losing
  the ability to associate a later validator with the exact read proof it used.

**Engineering decisions**
- Counters must be carried in the Milestone 8 closeout and Milestone 9 seed.
- Receipt identity must include selected read family, requirement row, access
  posture, touched authority, execution basis, policy or tenant narrowing where
  present, plan identity, and execution counter digest.
- Caller-owned graph work is a named counter family even when the expected count
  is always zero.
- Diagnostic projection is derived from receipts and policy. It is not the
  receipt authority itself.

**Open questions**
- None.

### Phase 7: Hard Deletion And Source Firewalls

This phase performs the hard break. Any local execution path displaced by Query
plan adoption is deleted, and any remaining old path is capped residue or a
typed Query-gap with explicit removal trigger.

**Relevant subsystems**
- old topology, spatial, and kernel graph-read helpers
- local adjacency maps and local graph caches
- fabricated receipt support
- source firewall and compile-fail fixture catalog

**Relevant APIs**
- deletion ledger report
- capped residue report
- source firewall report
- public facade compile-fail contracts

**Warnings**
- Do not keep "temporary" adapters that translate old helper output into Query
  receipts.
- Do not allow old helper names to survive in production outside deletion
  ledgers, tests, or capped residue fixtures.
- Do not preserve compatibility wrappers because tests still import them.
- Do not preserve manual read-plan lists, operator plan hints, local execution
  mode switches, or hidden plan-strategy branches as "routing convenience."

**Test requirements**
- `migrated_local_execution_paths_are_deleted_or_capped`: every migrated old
  path is absent or appears in capped residue with owner, blocker, cap, and
  removal trigger.
- `fabricated_receipt_helpers_fail_public_facade_contracts`: public callers
  cannot construct access-plan receipts, graph-read receipts, posture rows, or
  Milestone 9 seeds through struct literals or raw rows.
- `source_firewall_rejects_local_loop_cache_and_receipt_residue`: reintroduced
  local loops, adjacency maps, broad helpers, fabricated receipts, or
  compatibility wrappers fail source firewall tests.
- `source_firewall_rejects_manual_plan_hint_residue`: reintroduced operator
  read-plan lists, local access-mode switches, or strategy hint enums fail
  source firewall tests unless they are certified capped residue.

**Engineering decisions**
- Deletion proof is part of the Milestone 8 closeout, not an afterthought.
- Compile-fail fixtures must guard both raw receipt construction and
  lower-authority promotion into receipt-bearing seeds.
- Residue caps must default to zero unless the spec names an owner and blocker.
- Deleting plan-hint residue is as important as deleting old execution loops;
  otherwise the executor will drift back into strategy rediscovery.

**Open questions**
- None.

### Phase 8: Public Closeout And Milestone 9 Seed

This phase publishes the Milestone 8 proof product. The closeout is read-only,
receipt-bearing, and strong enough for validator and invariant routing to begin
without reopening old graph-read execution paths.

**Relevant subsystems**
- graph-read access-plan adoption closeout
- public Worth facade contracts
- Milestone 9 validator/invariant seed
- closeout counters and residue reports

**Relevant APIs**
- Milestone 8 access-plan adoption closeout
- Milestone 9 seed for validator and invariant catalog routing
- Query graph-read access receipts
- deletion, residue, source-firewall, posture, and counter reports

**Warnings**
- Do not claim validator or invariant selection. That is Milestone 9.
- Do not hide denied or required-posture reads behind a single "complete"
  boolean.
- Do not let public callers construct the closeout, receipts, or Milestone 9
  seed.

**Test requirements**
- `milestone_eight_closeout_exports_milestone_nine_seed`: the seed carries
  admitted receipt rows, required/denied posture rows, counters, deletion proof,
  capped residue proof, source firewall proof, and no validator-selection claim.
- `milestone_nine_seed_rejects_raw_plan_without_receipt_or_posture`: admitted
  plans that were not executed, raw Query receipts, and local summaries cannot
  satisfy the Milestone 9 seed without canonical receipt or required/denied
  posture evidence from the closeout.
- `closeout_preserves_required_postures_for_milestone_nine`: required or denied
  graph-read support remains visible so validator routing cannot assume reads
  are executable.
- `public_api_cannot_fabricate_receipt_bearing_closeout`: compile-fail fixtures
  reject public struct literal construction and raw receipt promotion.

**Engineering decisions**
- The closeout should expose exact counts for admitted plans, executed plans,
  denied plans, required postures, receipt rows, deleted paths, capped residue
  rows, source firewall regions, and caller-owned graph work.
- Milestone 9 seed consumes receipts and postures; it must not accept raw Query
  receipts or local helper summaries.
- The final closeout is the only ordinary bridge from graph-read execution work
  into validator and invariant routing.

**Open questions**
- None.

## Must Ship

- A new responsibility-named graph-read access-plan adoption lane built in
  parallel from the Milestone 7 closeout seed.
- A typed inventory of remaining local graph-read execution folklore with
  migrate, delete, cap, or Query-gap disposition.
- Query access-plan admission attempts for every covered read-family and
  requirement row from Milestone 7.
- Typed access posture rows for admitted, required, denied, and gap cases.
- Registered read-family routing proof showing matching touched authorities
  reach the adoption lane without operator-local plan lists or stage-local
  strategy hints.
- At least one real migrated vertical slice executing through Query plan
  receipt proof before old helper deletion.
- Receipt-bearing closeout rows with deterministic identity, execution basis,
  structural counters, lifecycle-scoped ephemeral index evidence where used, and
  no caller-owned graph work.
- Batch or grouped admission evidence where multiple covered graph reads can be
  admitted together without scalar caller loops.
- Deletion or capped residue for displaced local loops, adjacency maps, caches,
  broad helpers, fabricated receipts, compatibility wrappers, manual read-plan
  lists, operator plan hints, and local strategy switches.
- A Milestone 9 seed carrying receipt and posture evidence without claiming
  validator or invariant selection.

## Must Preserve

- Milestone 7 declaration catalog identity, read-family identity rows,
  requirement-row evidence, capability gaps, deletion proof, and source firewall
  proof as historical inputs.
- Query ownership of access plans, access posture, denial, receipt vocabulary,
  and execution counters.
- Worth ownership of reference consumer migration, closeout pressure, deletion
  enforcement, and public facade fences.
- Required and denied postures as visible proof products when Query cannot
  honestly execute a covered read yet.
- The roadmap rule that adding a registered read family once should route
  matching operators without per-operator read-plan code.
- The Query rule that declaration, lowering, execution, inspection, support
  posture, and receipts are runtime-owned artifacts rather than Worth-owned
  pseudo-Query surfaces.
- The distinction between operational receipt proof and policy-gated diagnostic
  materialization.

## Acceptance Evidence

Milestone 8 is done when a reviewer can start from the Milestone 7 closeout,
construct the Milestone 8 access-plan adoption closeout, inspect every admitted
Query access plan, required posture, denied posture, receipt row, execution
counter, and old-path deletion result, then hand a Milestone 9 seed to validator
and invariant routing without preserving any Worth-local graph-read execution
authority.

Acceptance requires:

- declare-once routing tests proving one registered read-family adoption rule
  applies to multiple matching touched authorities without operator edits
- focused closeout tests for seed admission, posture mapping, receipt identity,
  execution basis, counter accounting, deletion, residue visibility, and
  Milestone 9 seed export
- compile-fail fixtures denying public fabrication of adoption closeouts,
  posture rows, receipt rows, and Milestone 9 seeds
- source firewalls rejecting local loops, adjacency maps, broad helpers,
  fabricated receipts, compatibility wrappers, manual read-plan lists, operator
  hints, strategy switches, and local cache revival
- exact counter tests proving Query graph-read work is bounded by touched
  authority and selected read-family shape
- lifecycle counter tests for bounded ephemeral indexes and grouped/batched
  admission where those postures are used
- explicit blocked-surface rows for Query capabilities that are required but
  not yet executable

## Sequencing Notes

- This milestone follows Milestone 7 because declarations and requirement rows
  must exist before access plans can be admitted.
- This milestone precedes Milestone 9 because validators and invariants need
  receipt/posture evidence, not declaration-only proof.
- Spatial evidence lookup remains Milestone 11 unless a spatial graph read is
  already covered by the Milestone 7 declaration catalog.
- Replay, undo, conflict, cache, public diagnostics, and cross-crate closeout
  remain Milestones 12 through 16.
- The milestone should stop before validator or invariant selection begins.
