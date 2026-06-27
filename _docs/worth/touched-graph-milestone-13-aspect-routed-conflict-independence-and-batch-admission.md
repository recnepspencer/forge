# Milestone 13: Aspect-Routed Conflict, Independence, And Batch Admission

## Goal

Make conflict detection, independence proof, serialization posture, and
batch admission first-class consumers of the touched-graph architecture by
lowering them from aspect-aware semantic-graph contracts instead of
entity-only overlap folklore, speculative lock-first admission, or
executor-local rollback heuristics.

## Why This Milestone Exists

Milestone 12 made replay scope, undo scope, and transaction scope real typed
products. That means the roadmap now has enough authority to stop treating
multi-operation coordination as "run both and see what breaks."

Milestone 13 belongs here because it is the first remaining family that must
prove the semantic graph is not merely a routing language for one operation at
a time. If conflict and batch admission fall back to entity-only overlap,
speculative execution, broad receipt rescans, or stage-local compatibility
lists, the touched-graph architecture stops scaling exactly where concurrent
operation pressure begins.

## Governing Summaries

- `MENTALITY.md`: protects hard-problem-first design. This milestone must solve
  structural concurrency authority before feature-local batching and overlap
  shortcuts spread.
- `arch_laws.md`: protects lowered plan authority. Conflict, independence, and
  batch admission must be decided before execution from proof-bearing admitted
  inputs; executors may not speculate, rediscover, or lock-first their way to
  the answer.
- `composition_laws.md`: protects responsibility-named decomposition.
  Inventory, aspect vocabulary, family catalog, admission, selected conflict
  plan, independence proof, batch admission plan, cutover, and closeout may
  not collapse into one overlap helper.
- `domain_structure_laws.md`: protects visible ownership and authority
  boundaries. Shared semantic-graph aspect vocabulary must stay distinct from
  topology conflict families, spatial conflict families, kernel batch-admission
  pressure, and derived diagnostics.
- `perf_laws.md`: protects semantic-delta-bounded coordination cost. Conflict
  breadth must scale with touched closure, locality footprint, aspect
  applicability, and declared overlap contracts rather than global lock sets,
  broad graph scans, or speculative execution fallout.
- `touched-graph-roadmap.md`: places this milestone after canonical replay/undo
  scope and before cache/equivalence because conflict and batch admission must
  consume typed scope products and transaction packets before reuse posture can
  be trusted.

## Adversarial Constraint

Worth must survive long boolean and future curved-operation chains where many
small local operations may coexist, partially overlap, or conflict by entity,
relation, aspect, locality, evidence requirements, validator pressure, replay
scope, undo scope, or transaction boundary.

If a covered batch or concurrency path can determine compatibility by broad
topology rescans, broad evidence rescans, operation names, family-local allow
lists, speculative execution plus rollback, lock-first probing, or entity-only
touch overlap while ignoring aspect-local semantics already captured by touched
proof, replay scope, undo scope, and transaction packets, the milestone has
failed.

## Product Decision Lock

- Milestone 13 is a parallel-cutover milestone. Build new conflict,
  independence, and batch-admission lanes beside old overlap helpers,
  serialization helpers, executor-local admission shortcuts, and speculative
  rollback paths before cutting callers.
- Use parallel migration plus hard deletion. In-place refactoring is allowed
  only inside the new lanes after their authority shape exists.
- The milestone must follow the roadmap lifecycle shape:
  `family_catalog -> admitted_input -> selected_plan -> scope_product or
  compiled_product -> execution -> cutover/public_closeout/source_firewall`.
- `worth-schema` owns any new shared semantic-graph aspect vocabulary,
  locality-footprint identity, and overlap identity distinctions required for
  conflict routing.
- `worth-schema` also owns the one shared conflict-routing contract consumed by
  every later lane in this milestone. Topology conflict families, spatial
  conflict families, kernel batch-admission families, execution receipts, and
  Milestone 14 seed products must all compile against that one contract rather
  than inventing crate-local routing vocabularies.
- `worth-topo` owns topology conflict-family declarations and topology-side
  independence proof derived from touched closure, validator/invariant
  receipts, invalidation receipts, replay scope, undo scope, and transaction
  packets where applicable.
- `worth-spatial` owns spatial conflict-family declarations and spatial-side
  independence proof derived from spatial touch authority, evidence lookup
  receipts, replay scope, undo scope, and transaction packets where applicable.
- `worth-kernel` owns batch-admission orchestration, public closeout pressure,
  residue classification, and proof that no caller-owned shortcut can bypass
  selected conflict and independence plans.
- Query-owned downstream proof, support posture, projection consumption, and
  lower-runtime boundary traceability must use real `forge-query` surfaces,
  not local folklore. This milestone must name and use actual public entry
  points such as `ForgeQueryWorkspace`,
  `workspace.public_support_matrix()`,
  `workspace.public_api_contract()`,
  `workspace.public_handle_contract()`,
  `workspace.admit_public_api_family(...)`,
  `project_workspace_support_snapshot(...)`,
  `support_pinning_contract(...)`,
  `hard_prohibition_boundary_audit()`,
  `query_consumer_residue_audit()`,
  `consume_projection_facts(...)`,
  `declare_projection_fact_consumption(...)`,
  `forge_query_domain(...).for_lower_runtime_boundary_envelope(...)`,
  `forge_query_domain(...).for_lower_runtime_boundary_source(...)`,
  `ForgeQueryDeclarationEnvelopeInput`, and
  `ForgeQueryDeclarationEnvelope`.
- Replay scope products, undo scope products, transaction packets, validator
  receipts, invalidation receipts, evidence lookup receipts, touched closures,
  spatial touch authority, conflict plans, independence proofs, and
  batch-admission plans are distinct proof products. A later product may
  consume an earlier one; it may not reconstruct it from strings, display
  labels, broad scans, or local heuristics.
- The unification center is operational, not aspirational:
  `worth-topo` and `worth-spatial` may add family-local proof and
  denial/explanation details, but they may not invent alternate overlap
  categories, routing digests, or batch-admission semantics outside the shared
  `worth-schema` conflict-routing contract. `worth-kernel` may orchestrate
  grouped execution only from that shared contract plus prior proof products.
- Deletion is part of the milestone. Lock-first admission, broad overlap scans,
  executor-side speculative rollback for ordinary conflict resolution,
  caller-owned compatibility lists, and public raw conflict constructors must
  be deleted, capped, or denied before closeout.

## Implicit Requirements Made Explicit

- Covered conflict and batch admission means every ordinary production surface
  that currently decides whether two or more operations may co-exist, serialize,
  batch, retry, or deny based on touched closures, spatial touch authority,
  validator receipts, invalidation receipts, evidence lookup receipts, replay
  scope, undo scope, transaction packets, stage identity, or local helper
  folklore.
- Non-covered coordination paths must be explicitly named as certification-only,
  report/document codec support, test fixture support, or non-ordinary residue.
  They cannot be omitted from inventory because they are "rare."
- Aspect is an operational routing axis, not descriptive metadata. Conflict and
  independence must distinguish at least entity overlap, relation overlap,
  aspect overlap, locality overlap, evidence overlap, validator/invariant
  overlap, replay/undo scope overlap, and transaction-boundary overlap when
  those distinctions change concurrency posture.
- Compatible overlap must be declared, not inferred from success in prior
  speculative runs. A family must say whether an overlap kind is
  independent, serializable, denied, advisory, or requires stronger proof.
- Batch admission is not a late executor choice. It is a selected plan derived
  from admitted proof products before execution begins.
- Workload composition, retained replay consumers, public closeout, and
  diagnostics are in-scope consumers. The milestone is not done if the new
  conflict products exist but those consumers still teach older overlap
  semantics.
- Query support ownership is explicit. Declaration-scoped support belongs on
  Query declaration support lanes, lower-runtime route support belongs on Query
  lower-runtime boundary-envelope lanes, and downstream crate proof belongs on
  Consumer Kit. This milestone may not invent kernel-local substitutes for
  those categories.

## Named Existing Surfaces We Must Design Against

The milestone must plan from current real surfaces, not from generic crate
references.

**Query-owned support, proof, and boundary surfaces**
- `forge-query/docs/AI_README.md` core rule:
  `declare intent once -> lower it once -> execute or inspect it through canonical runtime-owned artifacts`
- `ForgeQueryWorkspace`
- `workspace.public_support_matrix()`
- `workspace.public_api_contract()`
- `workspace.public_handle_contract()`
- `workspace.admit_public_api_family(...)`
- Consumer Kit entry points:
  `EvidenceReportDeclaration`,
  `hard_prohibition_boundary_audit()`,
  `query_boundary_source_inventory()`,
  `project_workspace_support_snapshot(...)`,
  `support_pinning_contract(...)`,
  `in_memory_test_runtime()`,
  `evidence_report_adoption_audit()`,
  `query_consumer_residue_audit()`,
  `forge_query_consumer_residue_certification_evidence()`
- projection-consumption entry points:
  `consume_projection_facts(...)`,
  `declare_projection_fact_consumption(...)`,
  `bind_contract()`
- lower-runtime boundary entry points:
  `forge_query_domain(...).for_lower_runtime_boundary_envelope(...)`,
  `forge_query_domain(...).for_lower_runtime_boundary_source(...)`
- declaration-envelope entry points:
  `ForgeQueryDeclarationEnvelopeInput`,
  `ForgeQueryDeclarationEnvelope`,
  `ForgeQueryDeclarationEnvelopeChecked`

**Current kernel and spatial seed surfaces**
- `WorthWorkload`
- `WorthWorkloadParts`
- `require_admitted_stage_postures(...)`
- `require_matching_evidence_ledger(...)`
- `LookupConsumedWorkloadComposition`
- `EvidenceLookupConsumedWorkloadHandoff`
- `admit_boolean_split_replay_undo_boundary(...)`
- `AdmittedBooleanSplitReplayUndoBoundary`
- `BooleanSplitReplayUndoBoundaryRequest`
- `CoplanarOverlapWorkloadOperator`
- `CoplanarOverlapOperatorReceipt`
- `EvidenceLookupFamilyDeclaration`
- `EvidenceLookupDiagnosticWitnessShape`
- `EvidenceLookupEvidenceClassSet`
- `EvidenceLookupFamilyIndexPosture`
- `EvidenceLookupFamilyQueryPosture`
- `EvidenceLookupTopologyInputPosture`

The spec must name these surfaces wherever they are the current nearest honest
seed. "Use something from this crate" is not acceptable design language.

## Directory Skeleton

The milestone should converge toward this explicit parallel lane shape before
cutover:

- `crates/worth-schema/src/touched_graph/conflict_vocabulary/`
  shared aspect, locality, overlap-category, and overlap-identity vocabulary
- `crates/worth-topo/src/touched_graph_conflict/`
  topology conflict family declarations, admitted conflict input, selected
  conflict plan, independence proof, execution receipt, and source firewalls
- `crates/worth-spatial/src/touched_graph_conflict/`
  spatial conflict family declarations, admitted conflict input, selected
  conflict plan, independence proof, execution receipt, and source firewalls
- `crates/worth-kernel/src/workload_composition/batch_admission/`
  batch-admission family declarations, admitted grouped input, selected
  batch-admission plan, execution receipt, residue ledger, and closeout
  pressure
- `crates/worth-kernel/src/workload_composition/worth_workload/`
  call-site composition consumers and cutover adapters that admit only the new
  typed products
- `crates/worth-*/tests/touched_graph_conflict/`
  family-local hostile tests, compile-fail fences, and cutover proof

If exact module names shift during implementation, the ownership boundaries may
not shift with them.

## Shared Conflict-Routing Contract

Milestone 13 must converge around one shared cross-crate contract owned by
`worth-schema`. This is the unification center for the whole milestone.

That contract must be the only ordinary vocabulary for:
- overlap participant identity
- overlap category identity
- aspect-local compatibility identity
- locality-footprint identity
- prior-proof participation identity
- conflict posture
- independence-proof posture
- batch-admission posture
- execution-receipt semantic breadth counters

Operationally, the contract must behave like this:

```text
touched/spatial authority + prior proof
-> shared conflict-routing contract
-> topo/spatial family selection and proof
-> kernel batch-admission lowering
-> typed execution receipt
-> Milestone 14 seed identity
```

This means:
- `worth-topo` and `worth-spatial` must publish family declarations and proof
  products in terms of the shared contract
- `worth-kernel` must admit grouped work only through the shared contract and
  the proof products lowered from it
- public closeout, diagnostics, and future cache/equivalence work must bind to
  the shared contract identities rather than re-deriving overlap meaning
- no crate may invent a second ordinary overlap ontology even if the local
  implementation is tempting

## Phase Plan

### Phase 1: Conflict And Batch-Admission Folklore Inventory And Cut Line

Freeze every current conflict, overlap, independence, serialization, and
multi-operation admission surface before replacement code is written. Every
entity-only overlap helper, lock-first admission path, speculative rollback
admission shortcut, broad graph scan, broad evidence scan, local compatibility
list, and caller-owned serialization hint must be classified as migrate,
delete, cap, certification-only, or Query-gap.

This phase is one closeout boundary, but it contains three required inventory
cuts that must all land before Phase 2 starts:
- current authority inventory across kernel, topo, and spatial execution paths
- Query/support/proof inventory across Consumer Kit, support posture, and
  lower-runtime boundary traces touched by those paths
- cut-line and residue classification that names exactly what migrates into the
  new lane versus what is capped or deleted

**Relevant subsystems**
- `worth-kernel` workload composition and multi-operation orchestration
- `worth-topo` validator/invariant and invalidation consumption sites
- `worth-spatial` evidence lookup and replay-scope consumption sites
- public closeout and diagnostic surfaces that currently explain overlap or
  batch posture

**Relevant APIs**
- `WorthWorkload::compose(...)`
- `WorthWorkloadParts`
- `LookupConsumedWorkloadComposition`
- `EvidenceLookupConsumedWorkloadHandoff`
- `CoplanarOverlapWorkloadOperator`
- `CoplanarOverlapOperatorReceipt`
- Milestone 10 invalidation selected plans and execution receipts
- Milestone 11 evidence lookup receipts and consumed-workload handoff counters
- Milestone 12 replay scope, undo scope, transaction boundary packets, and
  `AdmittedBooleanSplitReplayUndoBoundary`
- touched closure and spatial touch authority products
- Query Consumer Kit boundary-audit and residue-audit surfaces for closeout
  pressure against downstream folklore

**Warnings**
- This phase is not a grep-only audit. It must produce typed inventory rows
  with dispositions and removal triggers.
- Do not classify a lock-first or speculative admission path as harmless
  because current tests are small. That is the exact scaling failure this
  milestone exists to eliminate.
- Do not let "internal helper" hide ordinary authority. If it can decide batch
  posture for production work, it is in scope.

**Test requirements**
- `conflict_inventory_has_no_keep_rows`: every ordinary overlap, serialization,
  speculative-admission, and compatibility helper has exactly one migrate,
  delete, cap, certification-only, or Query-gap disposition.
- `unclassified_conflict_surface_fails_closeout`: adding a new overlap helper,
  lock-first path, speculative rollback admission path, or compatibility list
  without an inventory row fails closeout.
- `inventory_rows_preserve_source_identity`: semantically similar old paths in
  distinct source locations produce distinct inventory rows so deletion cannot
  collapse unrelated authority.

**Engineering decisions**
- Inventory rows must carry source path, old authority kind, current caller,
  disposition, replacement phase, blocker, removal trigger, and whether the row
  is certification-only.
- Inventory rows must also carry exact existing surface identity when one
  already exists, such as `LookupConsumedWorkloadComposition`,
  `EvidenceLookupConsumedWorkloadHandoff`,
  `CoplanarOverlapWorkloadOperator`,
  `CoplanarOverlapOperatorReceipt`, or `WorthWorkload::compose(...)`.
- Inventory is closeout pressure only. It may not seed selected conflict plans.
- The three inventory cuts above are implementation slices inside one spec
  phase, not optional follow-up chores. If one inventory cut is missing, the
  phase is incomplete.

**Open questions**
- None.

### Phase 2: Shared Aspect And Locality Overlap Vocabulary

Freeze the shared semantic-graph vocabulary that conflict and independence use
so later phases do not smuggle entity-only or operation-name semantics into the
new lane.

This phase does not merely publish terms. It freezes the one shared
conflict-routing contract that every later phase consumes.

**Relevant subsystems**
- `worth-schema` shared semantic-graph vocabulary
- `worth-topo` touched closure and validator/invariant identity surfaces
- `worth-spatial` spatial touch authority and evidence lookup identity surfaces
- Milestone 12 replay/undo scope and transaction packet identity surfaces

**Relevant APIs**
- touched entities, relations, locality scopes, and digests
- replay scope identities
- undo scope identities
- transaction boundary packets
- `EvidenceLookupTopologyInputPosture`
- `EvidenceLookupEvidenceClassSet`
- `EvidenceLookupFamilyQueryPosture`
- Query declaration-envelope aspect publication surfaces where public crossing
  stories must preserve aspect-local masking rather than widen back to generic
  entity overlap

**Warnings**
- Aspect is not a display tag. It must be represented as an authority-bearing
  distinction that can change conflict posture.
- Do not reuse raw digests or strings where authority class differs.
- Do not let locality be reconstructed later from ad hoc closures or scans.

**Test requirements**
- `aspect_overlap_identity_is_stable_under_rerun`: identical semantic overlap
  inputs produce stable overlap identity across reruns and benign ordering
  noise.
- `wrong_authority_cannot_mint_overlap_identity`: raw strings, copied digests,
  or foreign authority values cannot construct aspect or locality overlap
  identity.
- `aspect_and_entity_overlap_remain_distinct`: identical entity sets with
  different aspect classes produce distinct overlap identities.

**Engineering decisions**
- Put shared aspect and locality overlap distinctions in `worth-schema`, not in
  crate-local helpers.
- Distinguish entity overlap, relation overlap, aspect overlap, locality
  overlap, evidence overlap, validator overlap, replay/undo overlap, and
  transaction overlap as separately nameable categories when they change routing
  or diagnostics.
- The vocabulary should follow the same rigor already present in
  `EvidenceLookupFamilyDeclaration`: explicit posture-bearing fields, explicit
  digest basis, and denial on missing required dimensions.
- Phase 2 must end with one shared contract type family in `worth-schema`
  rather than a loose bundle of enums or digests spread across crates.

**Open questions**
- None.

### Phase 3: Parallel Conflict Family Catalogs

Build the new conflict-family catalogs beside the old overlap and serialization
helpers before any execution path is migrated. A family declaration must state
which touched or receipt-backed facts it consumes, which overlap classes it can
decide, which postures it may emit, and which proof products it requires.

This phase is where topo and spatial lanes prove they are parallel declarers of
one architecture, not competing local systems.

**Relevant subsystems**
- new `worth-topo` conflict family lane
- new `worth-spatial` conflict family lane
- `worth-kernel` batch-admission orchestration lane

**Relevant APIs**
- touched closure products
- spatial touch authority products
- validator/invariant receipts
- invalidation receipts
- evidence lookup receipts
- replay/undo scope identities
- transaction packets
- `EvidenceLookupFamilyDeclaration`
- `EvidenceLookupDiagnosticWitnessShape`
- `EvidenceLookupFamilyIndexPosture`
- `EvidenceLookupFamilyQueryPosture`
- `EvidenceLookupTopologyInputPosture`

**Warnings**
- A conflict family catalog is not a callback list and not a static array of
  predicates.
- Do not let family identity come from operation names, command names, or
  display labels.
- Do not declare one giant "global conflict family" that hides real overlap
  categories.
- Do not let topology and spatial lanes drift into separately named overlap
  ontologies that only happen to be similar. Similarity is not unification.

**Test requirements**
- `declared_once_conflict_family_routes_multiple_consumers`: one conflict family
  declaration applies to at least two matching consumers without consumer-local
  wiring.
- `family_declaration_requires_overlap_classes_and_posture`: a family missing
  overlap classes, required prior-proof posture, or emitted conflict posture
  cannot enter the catalog.
- `raw_strings_cannot_mint_conflict_family_identity`: raw strings and copied
  receipt labels cannot construct family identity.

**Engineering decisions**
- Replay-facing and undo-facing overlap families may share vocabulary, but they
  must remain distinct declarations where required proof or failure posture
  differs.
- Family declarations must say whether a matched overlap can yield
  `Independent`, `Serializable`, `Denied`, or `Advisory`.
- Conflict family declarations must be as operationally explicit as
  `EvidenceLookupFamilyDeclaration`. Each declaration must name identity,
  touched/spatial authority requirements, topology input posture, stage
  applicability, proof classes consumed, overlap classes decided,
  index/query posture, diagnostic witness shape, source-inventory pressure, and
  declaration digest basis.
- Topology and spatial family declarations must import the shared
  conflict-routing contract from `worth-schema`; they may extend it with
  family-local proof fields but may not redefine its core identities or
  postures.

**Open questions**
- None.

### Phase 4: Admitted Conflict Input Boundary

Freeze admitted conflict input as the only legal start for conflict routing.
Admission must accept only sealed touched closures or spatial touch authority,
typed prior receipts, typed replay/undo scope, typed transaction packets, and
declared locality/aspect overlap inputs. Raw rows, broad scans, and
caller-built overlap guesses cannot enter.

**Relevant subsystems**
- `worth-topo` conflict family lane
- `worth-spatial` conflict family lane
- `worth-kernel` batch-admission entry lane

**Relevant APIs**
- touched closure products
- spatial touch authority products
- validator/invariant receipts
- invalidation receipts
- evidence lookup receipts
- replay scope and undo scope products
- transaction boundary packets
- `LookupConsumedWorkloadComposition::admit(...)`
- `EvidenceLookupConsumedWorkloadHandoff`

**Warnings**
- Do not allow entity-only touched overlap to stand in for aspect-aware
  admitted input.
- Do not allow transaction packets to replace touched closure or spatial touch
  authority. They are supporting proof, not primary touch proof.
- Do not parse strings to recover overlap class or locality.

**Test requirements**
- `admitted_conflict_input_requires_typed_authority`: raw rows, copied digests,
  broad scan results, and caller-authored overlap guesses cannot seed conflict
  planning.
- `wrong_receipt_family_denies_before_selection`: mismatched validator,
  invalidation, evidence, replay, undo, or transaction proof denies before
  family selection.
- `entity_only_overlap_cannot_satisfy_aspect_input`: admitted input that lacks
  required aspect overlap proof is rejected even when entity sets match.

**Engineering decisions**
- Admission produces a phase-typed `admitted conflict input` product consumed by
  selected conflict planning.
- Admission products must carry exact identity and denial posture for each
  required prior-proof class.
- Admission must preserve the same zero-folklore constraints already enforced
  by `LookupConsumedWorkloadComposition::admit(...)`: no raw row scans, no
  broad receipt scans, no caller-owned scans, and exact stage-index identity
  agreement between the admitted handoff and the consuming workload.

**Open questions**
- None.

### Phase 5: Selected Conflict Plan Lowering

Lower admitted conflict inputs plus the family catalogs into selected conflict
plans before any batch or serialization decision executes. The selected plan
must say which overlap families matched, which overlap classes were observed,
which proofs were missing, which postures are possible, and which later
independence checks are required.

**Relevant subsystems**
- conflict family catalogs
- admitted conflict input products
- `worth-kernel` batch-admission planning lane

**Relevant APIs**
- conflict family catalog digests
- admitted conflict input digests
- replay/undo scope identities
- transaction packet identities
- Query declaration-envelope and projection-consumption surfaces when a public
  crossing story or typed consumed fact must be carried forward without
  rebuilding meaning locally

**Warnings**
- Do not let execution rediscover family applicability, overlap class, or
  serialization posture.
- Do not hide planning behind "admit and then inspect runtime state."
- Do not scalarize compatible grouped work into caller-owned loops when one
  selected plan could carry the batch.

**Test requirements**
- `same_input_and_catalog_produce_same_conflict_plan_digest`: identical admitted
  inputs and family catalogs produce stable selected conflict plan identity.
- `unrelated_families_remain_unselected`: non-intersecting conflict families do
  not enter selected plans.
- `missing_prior_proof_denies_before_execution`: a missing validator,
  invalidation, evidence, replay, undo, or transaction proof yields denial or
  required-proof posture before batch execution.

**Engineering decisions**
- Selected conflict plans are derived products and must remain distinct from
  family declarations and admitted inputs.
- Plan identity must reflect overlap classes and prior-proof posture, not
  command labels or execution order.
- If a selected conflict plan needs public explanation or downstream proof, it
  must bind to Query-owned explanation, projection-consumption, or envelope
  categories instead of exporting local payload archaeology.

**Open questions**
- None.

### Phase 6: Independence Proof Product

Freeze independence proof as its own typed product instead of treating it as
"the absence of conflict." Independence must positively prove that overlap is
either disjoint or compatible under declared aspect-local rules.

**Relevant subsystems**
- `worth-topo` topology conflict lane
- `worth-spatial` spatial conflict lane
- `worth-kernel` batch-admission planning lane

**Relevant APIs**
- selected conflict plans
- touched closure and spatial touch authority products
- replay/undo scope identities
- transaction packets

**Warnings**
- Independence is not "we did not notice a collision."
- Do not compute independence by re-running execution or by checking lock
  acquisition success.
- Do not collapse disjointness and compatible overlap into one unnamed success
  state.

**Test requirements**
- `independence_requires_positive_proof`: independence proof exists only when
  selected overlap classes are either disjoint or explicitly compatible under a
  declared family contract.
- `compatible_aspect_overlap_stays_distinct_from_disjointness`: compatible
  aspect-local overlap and fully disjoint locality produce distinct proof
  variants.
- `executor_cannot_fabricate_independence`: execution-time success without a
  selected independence proof cannot satisfy batch admission.

**Engineering decisions**
- Independence proof is a separate product consumed by batch-admission planning.
- Use explicit variants such as `Disjoint`, `CompatibleAspectOverlap`,
  `SerializableOnly`, and `Denied` rather than binary booleans.

**Open questions**
- None.

### Phase 7: Batch-Admission Family Catalog

Build the batch-admission catalog beside old batching and serialization
shortcuts. A batch-admission family declares which selected conflict plans and
independence proofs it consumes, what serialization posture it emits, and what
diagnostic witness it owes on denial or advisory outcomes.

This phase is the kernel-side proof that orchestration consumes shared routing
products instead of becoming a second semantic authority lane.

**Relevant subsystems**
- new `worth-kernel` batch-admission lane
- topology and spatial conflict lanes
- workload composition orchestration

**Relevant APIs**
- selected conflict plans
- independence proofs
- replay/undo scope products
- transaction boundary packets

**Warnings**
- Batch admission is not a convenience wrapper around conflict checks. It is a
  separate declared family because it decides grouped execution posture.
- Do not let callers hand-author serialization order or compatibility lists as
  if they were batch plans.
- Do not let `worth-kernel` invent a kernel-native overlap language while
  lowering batch posture. Kernel orchestration is a consumer here, not the
  source of conflict meaning.

**Test requirements**
- `declared_once_batch_family_applies_to_multiple_batches`: one batch-admission
  family declaration applies to multiple matching grouped operations without
  caller-local wiring.
- `batch_family_requires_serialization_posture`: a family missing emitted
  posture or denial witness shape cannot enter the catalog.
- `caller_ordering_cannot_mint_batch_plan_identity`: caller-provided order or
  labels cannot act as batch-admission family identity.

**Engineering decisions**
- Batch-admission families are kernel-owned because they orchestrate grouped
  execution across lower-authority proofs.
- Family declarations must distinguish `ParallelAdmit`, `SerialAdmit`,
  `AdvisorySerialAdmit`, and `Denied`.
- Batch-admission family declarations must consume only the shared
  conflict-routing contract plus prior proof products. If kernel needs a new
  overlap distinction, that distinction belongs back in `worth-schema`, not in
  a kernel-local family field.

**Open questions**
- None.

### Phase 8: Selected Batch-Admission Plan

Lower selected conflict plans plus independence proofs into selected
batch-admission plans before executors see grouped work. The selected plan must
state exactly which operations may run in parallel, which must serialize, which
are denied, and which are advisory-only.

**Relevant subsystems**
- batch-admission family catalog
- selected conflict plans
- independence proof products
- workload composition orchestration

**Relevant APIs**
- batch-admission family catalog digests
- selected conflict plan digests
- independence proof digests
- transaction boundary packet identities

**Warnings**
- Do not let executors choose parallel versus serial admission after planning.
- Do not hide serialization fallback inside "best effort" runtime policy.
- Do not let batch plans depend on speculative lock acquisition.

**Test requirements**
- `same_conflict_inputs_produce_same_batch_plan_digest`: identical conflict
  plans and independence proofs produce stable selected batch-admission plan
  identity.
- `parallel_admission_requires_independence_proof`: no plan may emit
  `ParallelAdmit` without an explicit independence proof.
- `serializable_overlap_does_not_upgrade_to_parallel`: compatible but
  serial-only overlap remains serialized even if prior runs succeeded in
  parallel.

**Engineering decisions**
- Selected batch-admission plans are the only acceptable grouped input to the
  execution engine.
- Batch plans must include exact participant identities, selected posture,
  supporting conflict-family rows, and denial or advisory witness shapes.

**Open questions**
- None.

### Phase 9: Batch-Admission Execution Receipt

Freeze execution as a typed receipt-bearing product rather than letting batch
admission end at plan selection. The execution lane must preserve the selected
conflict plan identity, independence-proof identity, selected batch-plan
identity, participant identities, and semantic breadth counters needed for
closeout and Milestone 14 seed reuse.

**Relevant subsystems**
- new `worth-kernel` batch-admission execution lane
- topology and spatial conflict execution witnesses
- workload composition consumers

**Relevant APIs**
- selected conflict plan digests
- independence proof digests
- selected batch-admission plan digests
- `WorthWorkload`
- `CoplanarOverlapOperatorReceipt` as the current precision model for real
  semantic breadth counters

**Warnings**
- Do not let execution collapse into a boolean success/failure result.
- Do not let executors emit generic counters like "operations checked" when the
  overlap family already knows richer semantic breadth categories.
- Do not emit receipts that can be fabricated without the selected plan and
  participating proof products.

**Test requirements**
- `batch_execution_receipt_binds_selected_plan_chain`: execution receipt binds
  selected conflict plan identity, independence-proof identity, selected
  batch-plan identity, and participant identities.
- `execution_receipt_counters_are_semantic_not_generic`: receipt exposes family-
  meaningful breadth counters instead of one undifferentiated overlap count.
- `execution_cannot_forge_success_without_selected_plan`: execution success
  without a selected batch-admission plan is rejected.

**Engineering decisions**
- Batch-admission execution receipt is its own product, not an annotation on
  the selected batch plan.
- Receipt counters should follow the rigor of
  `CoplanarOverlapOperatorReceipt`: operator input count, operator receipt
  count, extracted proof count, breadth count, required-exit count, ambiguity
  count, and family-specific certified-contact or certified-overlap counts
  where applicable.
- `WorthWorkload` and adjacent composition consumers must accept the execution
  receipt as a first-class evidence stage rather than reconstructing grouped
  admission posture later.

**Open questions**
- None.

### Phase 10: First Vertical Migration Slice

Migrate one real ordinary grouped-operation slice through the new lanes before
the broad sweep. The first slice must prove the full path from admitted
authority to selected conflict plan, independence proof, selected batch plan,
execution receipt, and execution posture.

**Relevant subsystems**
- one covered topology or spatial grouped-operation path
- `worth-kernel` workload composition
- conflict and batch-admission lanes

**Relevant APIs**
- admitted conflict inputs
- selected conflict plans
- independence proofs
- selected batch-admission plans
- batch-admission execution receipts

**Warnings**
- Do not pick a certification-only path because it is easy.
- Do not skip directly to the sweep without one real parity slice.
- Do not keep the old helper active behind a silent adapter once the slice is
  cut over.

**Test requirements**
- `vertical_slice_matches_or_strengthens_old_posture`: the migrated slice
  produces the same or stronger admit/serialize/deny outcome as the displaced
  path.
- `old_overlap_helper_cannot_satisfy_migrated_slice`: once cut over, the slice
  cannot route through the displaced overlap helper.
- `slice_preserves_aspect_local_distinctions`: the migrated slice proves that
  aspect-local compatibility and entity-only overlap are not collapsed.

**Engineering decisions**
- The first migrated slice must consume real Milestone 12 scope products and
  transaction packets.
- The first migrated slice should prefer an already-real seed surface such as
  `LookupConsumedWorkloadComposition` or `CoplanarOverlapWorkloadOperator`
  rather than a synthetic greenfield path, so the milestone proves cutover over
  authority that currently matters.
- The slice should maximize architectural coverage, not minimize implementation
  effort.

**Open questions**
- None.

### Phase 11: Ordinary Consumer Sweep And Parallel Cutover

Cut every covered ordinary conflict and batch-admission consumer to the new
lanes. This is the broad migration phase: every covered consumer must become
selected-plan-driven, be deleted, or be mechanically capped as non-ordinary
residue.

This phase is broad by consumer count, not by authority shape. The authority
is already frozen by earlier phases. Implementation should therefore execute
this phase as a series of explicit consumer-cluster cutovers rather than one
opaque sweep.

**Relevant subsystems**
- `worth-kernel` workload composition
- topology conflict consumers
- spatial conflict consumers
- replay/undo and transaction consumers that expose grouped admission posture

**Relevant APIs**
- selected conflict plans
- independence proofs
- selected batch-admission plans
- batch-admission execution receipts
- transaction boundary packets
- residue and closeout ledgers

**Warnings**
- This phase is not done when the new lanes are available but optional.
- Do not leave "temporary" caller-owned compatibility lists in ordinary paths.
- Do not let grouped consumers reroute through speculative rollback when the
  selected batch plan says `Denied` or `SerialAdmit`.

**Test requirements**
- `all_covered_consumers_route_through_selected_batch_plans`: every covered
  ordinary consumer uses the new selected batch-admission plan or has explicit
  non-ordinary residue denial.
- `parallel_cutover_preserves_zero_broad_scan_fallback`: ordinary cutover does
  not revive broad topology scans, broad evidence scans, or executor-side
  overlap rediscovery.
- `residue_rows_are_exact_and_non_authoritative`: every remaining non-ordinary
  consumer is counted, owned, blocked, and denied as ordinary admission proof.

**Engineering decisions**
- Sweep completion is an acceptance boundary, not cleanup.
- Any generic bridge from old overlap helpers to new plans counts as residue and
  must be capped or deleted in this milestone.
- Consumer-cluster slicing is expected here. For example: workload composition
  consumers, retained-replay consumers, evidence-ledger consumers, and public
  closeout/status consumers may cut over in separate implementation batches,
  but each batch must terminate at the same selected-plan and execution-receipt
  authority chain.

**Open questions**
- None.

### Phase 12: Source Firewalls, Constructor Denials, And Hard Deletion

Install firewalls before closeout and delete or cap the displaced old paths.
This phase prevents reintroduction of entity-only overlap helpers, broad scans,
lock-first admission, speculative rollback admission, caller-authored
serialization, and public constructor forgery.

**Relevant subsystems**
- old overlap and serialization helpers
- public proof and closeout facades
- conflict and batch-admission source-firewall lanes

**Relevant APIs**
- source-firewall reports
- compile-fail fixtures
- residue and deletion ledgers

**Warnings**
- Firewall success is not a substitute for deletion.
- Do not allow one generic overlap utility to survive as an unofficial second
  authority lane.
- Public callers must not be able to forge conflict-family rows, admitted
  inputs, selected plans, independence proofs, or batch-admission plans.

**Test requirements**
- `source_firewall_rejects_conflict_folklore_revival`: forbidden semantic
  surfaces for entity-only overlap helpers, broad scans, lock-first admission,
  speculative rollback admission, and caller-owned serialization cannot
  reappear on covered paths.
- `public_api_cannot_forge_conflict_or_batch_products`: compile-fail fixtures
  reject public construction of family records, admitted inputs, selected
  conflict plans, independence proofs, selected batch plans, and closeout
  products.
- `deletion_ledger_binds_firewall_report`: deletion closeout consumes the
  firewall report digest while still naming concrete deleted or capped surfaces.

**Engineering decisions**
- Source firewall must ban old authority by semantic surface, not only exact
  symbol names.
- Residue rows must carry owner, exact count, blocker, and removal trigger.

**Open questions**
- None.

### Phase 13: Public Closeout And Milestone 14 Seed

Publish Milestone 13 only after covered ordinary consumers route through real
selected conflict and batch-admission plans, old authority is deleted or capped,
and the new products expose enough identity for Milestone 14 cache and
equivalence work to start without overlap rediscovery.

**Relevant subsystems**
- `worth-kernel` public closeout pressure
- topology conflict closeout
- spatial conflict closeout
- batch-admission closeout

**Relevant APIs**
- selected conflict plan digests
- independence proof digests
- selected batch-admission plan digests
- batch-admission execution receipt digests
- residue and deletion ledgers
- source-firewall reports
- Milestone 14 seed surfaces

**Warnings**
- Do not claim cache/equivalence completion here.
- Do not let closeout be satisfied by diagnostics strings or local reports.
- Do not let the seed omit the overlap identity and locality proof Milestone 14
  will need for reuse denial.

**Test requirements**
- `milestone_thirteen_closeout_requires_real_cutover`: final closeout fails if
  any covered ordinary consumer still depends on old overlap or serialization
  authority.
- `closeout_binds_full_conflict_authority_chain`: closeout digests bind touched
  or spatial authority, prior-proof inputs, selected conflict plans,
  independence proofs, selected batch plans, residue rows, and firewall proof.
- `milestone_fourteen_seed_carries_overlap_identity_without_rediscovery`: the
  seed carries enough conflict, independence, and batch-admission identity to
  start cache/equivalence work without rescanning topology or evidence.

**Engineering decisions**
- Emit a Milestone 14 seed with admitted overlap identity, selected conflict
  plan identity, independence proof identity, selected batch-admission plan
  identity, batch-admission execution receipt identity, residue digest, and
  firewall digest.
- Final closeout consumes proof products only, never raw collections or local
  compatibility lists.

**Open questions**
- None.

## Must Ship

- A typed inventory of current conflict, independence, serialization,
  speculative-admission, and batch-admission authority surfaces with migrate,
  delete, cap, certification-only, or Query-gap disposition.
- Shared semantic-graph aspect and locality overlap vocabulary in
  `worth-schema`, including authority-safe overlap identities that distinguish
  entity, relation, aspect, locality, evidence, validator, replay/undo, and
  transaction overlap where those differences affect concurrency posture.
- Parallel topology conflict-family and spatial conflict-family lanes with
  family catalogs, admitted conflict inputs, selected conflict plans, and
  independence proof products.
- A parallel kernel-owned batch-admission lane with its own family catalog,
  selected batch-admission plans, typed execution receipts, and grouped
  execution posture proof.
- At least one real ordinary migrated vertical slice proving end-to-end
  cutover from admitted proof to selected conflict plan, independence proof,
  selected batch plan, and execution posture.
- A complete ordinary consumer sweep so every covered conflict and
  batch-admission consumer is migrated, deleted, or capped as non-ordinary
  residue.
- Source firewalls, compile-fail fences, deletion ledger, residue ledger, and
  public constructor denials that prevent entity-only overlap helpers, broad
  scans, lock-first admission, speculative rollback admission, caller-owned
  compatibility lists, and raw proof construction from returning.
- A Milestone 14 seed carrying overlap identity, selected conflict-plan
  identity, independence-proof identity, selected batch-plan identity,
  batch-execution-receipt identity, residue digest, and firewall digest.

## Must Preserve

- Touched closure and spatial touch authority remain the primary locality and
  changed-meaning authority.
- Milestone 10 invalidation receipts, Milestone 11 evidence lookup receipts,
  and Milestone 12 replay scope, undo scope, and transaction packets remain
  prior-proof inputs. They must not be recertified, widened, or substituted.
- Execution may consume lowered batch-admission plans only. Executors may not
  rediscover overlap, admit by lock contention, or treat speculative success as
  proof.
- Execution receipts remain derived from selected plans and admitted proof.
  They may not become a back door for reclassifying conflict posture after the
  fact.
- Diagnostics and public closeout remain derived observability products, not
  conflict or batch-admission authority.
- Deletion and residue honesty remain first-class closeout requirements.

## Acceptance Evidence

- Tests prove inventory completeness and reject unclassified overlap,
  serialization, speculative-admission, or compatibility surfaces.
- Tests prove raw strings, copied digests, broad scan results, and
  caller-authored overlap guesses cannot enter admitted conflict input.
- Tests prove selected conflict-plan identity is deterministic from admitted
  proof and family declarations, and that unrelated families remain unselected.
- Tests prove independence requires positive proof and cannot be fabricated from
  execution success or missing collision reports.
- Tests prove `ParallelAdmit` requires explicit independence proof and that
  serial-only compatible overlap does not silently upgrade to parallel
  execution.
- Tests prove execution receipts bind the selected conflict-plan,
  independence-proof, and selected batch-plan chain while exposing semantic
  breadth counters instead of generic overlap totals.
- Tests prove the first migrated slice matches or strengthens prior
  admit/serialize/deny posture while preserving aspect-local distinctions.
- Tests prove every covered ordinary consumer routes through selected
  batch-admission plans or explicit non-ordinary residue.
- Tests prove source firewalls and compile-fail fixtures reject reintroduction
  of entity-only overlap helpers, broad scans, lock-first admission,
  speculative rollback admission, caller-owned serialization, and raw proof
  construction.
- Tests prove Milestone 14 can start from the emitted seed without rescanning
  topology or evidence to rediscover overlap authority.

## Sequencing Notes

- Milestone 13 belongs immediately after Milestone 12 because conflict and
  batch admission must consume typed replay/undo scope and transaction packets,
  not pre-scope local overlap folklore.
- It belongs before Milestone 14 because cache and equivalence contracts need
  authoritative overlap identity and batch posture to determine whether reuse
  is semantically safe.
- It should not attempt Milestone 14 cache/equivalence closure or Milestone 15
  public explainer unification beyond emitting the typed seed those later
  milestones consume.

## Required Self-Check

- Does the milestone solve a real structural problem or just package work
  cosmetically? Yes. It replaces speculative and entity-only coordination
  folklore with declared conflict, independence, and batch-admission proof.
- Is the adversarial constraint precise and load-bearing? Yes. It rejects broad
  scans, lock-first probing, speculative rollback admission, caller-owned
  compatibility lists, and entity-only overlap under concurrent local work.
- Does the roadmap justify this milestone now? Yes. Milestone 12 already made
  replay/undo and transaction scope real typed inputs, and Milestone 14 needs
  stable overlap identity before cache and equivalence work can be honest.
- Does the spec preserve crate authority boundaries? Yes. `worth-schema` owns
  shared overlap vocabulary, `worth-topo` and `worth-spatial` own conflict
  family authority, and `worth-kernel` owns grouped admission orchestration and
  closeout pressure.
- Are the phases carrying most of the real design information? Yes. The design
  payload lives in the thirteen ordered phases.
- Is each phase centered on one conceptual detail or boundary? Yes: inventory,
  vocabulary, conflict catalog, admission, selected plan, independence proof,
  batch catalog, execution receipt, batch plan, first slice, sweep,
  firewall/deletion, and closeout/seed.
- Does each phase contain at least 2 adversarial tests by default? Yes.
- Could a competent engineer map this spec into honest types, modules, and
  tests? Yes. The spec names the proof products, old authority being displaced,
  cutover rules, firewalls, residue posture, and the next milestone seed.
- Does the milestone belong in this roadmap sequence, or is it out of order?
  It belongs here because conflict must become aspect-routed before
  cache/equivalence, public explanation, and final parity proof can unify.
