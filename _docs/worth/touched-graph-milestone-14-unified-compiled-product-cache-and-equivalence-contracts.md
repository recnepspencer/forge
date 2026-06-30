# Milestone 14: Unified Compiled Product, Cache, And Equivalence Contracts

## Goal

Make compiled-product identity, cache eligibility, reuse posture, and
equivalence comparison first-class consumers of the touched-graph architecture
by lowering them from one typed compiled-product contract instead of
product-local cache keys, pointer identity, row-count heuristics, provenance
folklore, or retained helper conventions.

## Semantic Definitions

Milestone 14 is only honest if the core terms below have one fixed meaning.

- **Authoritative truth**: the owning runtime's canonical graph or evidence
  state. Authoritative truth may change compiled-product validity, but a
  compiled product never becomes authority.
- **Compiled product**: a derived artifact retained or compared for future
  read, replay-support, evidence-support, or execution-support work. A compiled
  product is reusable only if its semantic validity can be expressed from:
  source authority basis, touched/locality basis, prior-proof basis, and
  family-declared equivalence policy.
- **Not a compiled product**:
  - authority packets and authority digests
  - touched-basis, replay-scope, undo-scope, conflict, independence, and
    batch-admission proof products
  - diagnostics rows, closeout rows, and report projections
  - public explainability projections
  These may participate in compiled-product identity, but they are not
  themselves compiled products unless a family explicitly declares them as
  derived reusable artifacts.
- **Compiled-product identity**: the semantic identity of one derived product
  instance. It answers "what product is this?" and "what semantic basis shaped
  it?"
- **Equivalence**: a family-declared claim that two compiled-product identities
  represent the same semantic product under an explicit comparison basis.
- **Compatibility**: a family-declared claim that two products may still
  participate in a downstream workflow together without claiming they are the
  same product. Compatibility never implies reuse.
- **Reuse**: the ordinary-path decision to consume a previously built compiled
  product instead of rebuilding it now. Reuse requires equivalence plus any
  family-declared freshness and posture requirements. Equivalence alone is not
  enough.
- **Fresh rebuild required**: the products remain comparable or even partially
  compatible, but ordinary-path reuse is not allowed. This is not a soft cache
  miss; it is a typed semantic result.
- **Reuse denial**: the claimed prior product is semantically ineligible for
  ordinary reuse. Denial must localize the mismatching basis dimension rather
  than collapse into generic inequality.
- **Rendered output equality**: equality of rows, materialized payloads, or
  display shape. This is never sufficient for equivalence unless a family
  explicitly declares rendered output as part of the equivalence basis.

The implementation must treat these distinctions as type-level and
family-declared, not comment-level guidance.

## Why This Milestone Exists

Milestone 12 made replay scope, undo scope, and transaction scope real typed
products. Milestone 13 made conflict, independence, and batch admission real
typed products and emitted the seed identity that later milestones must consume
instead of rediscovering overlap from topology or evidence scans.

That means the roadmap now has enough authority to stop treating reuse as a
local performance trick.

If Milestone 14 only adds a new cache key helper for one lane, the architecture
will split at the exact point where it needs to unify most strongly:

- topology-derived compiled products will teach one sameness language
- evidence lookup products will teach a second
- replay and retained-workload products will teach a third
- public read or closeout helpers will continue implying reuse through local
  stability assumptions

Milestone 14 therefore belongs here because it is the first remaining family
that must prove derived-product identity is itself part of the semantic-graph
architecture rather than a crate-local optimization detail.

Milestone 14 is also where the roadmap's unified architecture can still either
converge or quietly fracture.

The roadmap does not describe touched graph, aspects, compiled products,
conflict, replay, cache, diagnostics, and public proof as adjacent systems. It
describes them as different lowered forms of one semantic-graph contract:

```text
touched graph authority declares what changed
aspect vocabulary declares which dimensions of meaning exist
registered families declare applicability once
planner-owned routing intersects those proofs once
read products, validators, invalidation, evidence lookup, replay/undo,
conflict, cache/equivalence, diagnostics, and public proof all lower from
that same contract family
```

Milestone 14 therefore cannot merely make cache behavior cleaner. It must make
compiled products and reuse posture behave like registered semantic-graph
families that later planner-owned routing, diagnostics, and public proof can
consume without inventing a second ontology for "same enough to reuse."

## Governing Summaries

- `MENTALITY.md`: protects hard-problem-first design. This milestone must solve
  semantic reuse authority before local caches and retained helpers spread.
- `arch_laws.md`: protects explicit equivalence contracts, proof-bearing
  lowering, authority/derivation separation, and execution-from-lowered-plan
  only. Reuse must be decided from typed identity and declared equivalence, not
  executor-local guesswork.
- `composition_laws.md`: protects responsibility-named decomposition.
  Inventory, shared identity vocabulary, family catalog, admission, comparator
  selection, equivalence proof, cutover, and closeout may not collapse into one
  cache helper.
- `domain_structure_laws.md`: protects visible ownership. The tree must show
  shared compiled-product identity, topology compiled-product lanes, spatial
  compiled-product lanes, kernel cutover pressure, and public proof surfaces as
  distinct responsibilities.
- `perf_laws.md`: protects semantic-delta-bounded execution and explicit
  equivalence contracts. Reuse breadth must scale with touched closure, source
  authority digest, locality footprint, and declared comparator posture rather
  than broad re-materialization, broad row comparison, or path-local fallback.
- `touched-graph-roadmap.md`: places this milestone after replay/undo scope and
  aspect-routed conflict because compiled-product reuse must consume typed scope
  and overlap identity before public proof and diagnostics can be unified.
- `touched-graph-roadmap.md`: also defines the end-state declare-once routing
  target and target directory skeleton. Milestone 14 must move compiled-product
  identity and reuse into that same shared lifecycle shape rather than adding a
  one-off cache subsystem.
- `crates/forge-query/docs/AI_README.md`: protects the core Query rule:
  `declare intent once -> lower it once -> execute or inspect it through canonical runtime-owned artifacts`.
  This milestone must apply that rule to compiled-product identity and reuse.

## Adversarial Constraint

Worth must survive long boolean and future curved-operation chains where many
small local edits rebuild, retain, compare, suppress, or reuse derived products
across topology, evidence lookup, replay, retained workload, and public
read-model surfaces while authoritative graph truth changes only locally.

If a covered reuse surface can justify reuse from pointer identity, row count,
operator family, rendered shape, filename provenance, retained helper
convention, broad row comparison, or local "same as before" folklore instead
of a typed compiled-product identity grounded in touched authority, source
authority digest, locality footprint, prior proof, and explicit equivalence
policy, this milestone has failed.

## Product Decision Lock

- Milestone 14 is a parallel-cutover milestone. Build new compiled-product and
  equivalence lanes beside old cache helpers, replay-equivalence helpers,
  retained-product stability helpers, pseudo-reuse conventions, and public
  closeout identity folklore before cutting callers.
- Use parallel migration plus hard deletion. The required execution shape is:
  1. identify the existing folders, modules, and public surfaces being
     displaced
  2. create a new responsibility-named parallel folder lane
  3. migrate one vertical slice through the new lane
  4. cut callers to the new lane through typed proof products
  5. delete the displaced lane or cap exact residue with owner, count, blocker,
     and removal trigger
  6. install source firewalls and compile-fail fences so the old lane cannot
     silently revive
- In-place refactoring is not an acceptable implementation strategy for this
  milestone's authority transition. It is allowed only for small mechanical
  edits inside an already-created new lane after the new lane's ownership and
  product boundaries exist.
- "Keep the old folder and gradually reshape it" is not parallel cutover.
- "Wrap the old helper with a typed facade" is not parallel cutover.
- "Temporarily leave both semantics inside one module" is not parallel cutover.
- If the work does not create a clearly named new lane beside the displaced
  lane, the work has not followed the roadmap's migration execution law.
- The milestone must follow the roadmap lifecycle shape:
  `family_catalog -> admitted_input -> selected_plan -> compiled_product ->
  equivalence_or_reuse_result -> cutover/public_closeout/source_firewall`.
- The milestone must also preserve the roadmap's declare-once routing rule:
  operators, replay lanes, evidence lanes, grouped workload lanes, and public
  closeout lanes may consume compiled-product routing products, but they may
  not name local cache keys, local comparator rules, local reuse postures, or
  local product-sameness languages on covered paths.
- `worth-schema` owns the one shared compiled-product identity contract:
  source authority digest, touched digest, locality footprint identity,
  required prior-proof identity, validator/evidence set identity, stage
  identity, batch-admission identity where applicable, and equivalence-policy
  identity.
- `worth-schema` also owns the distinction between:
  - authoritative truth identity
  - compiled-product identity
  - equivalence-policy identity
  - reuse decision identity
  These may relate to one another, but none may substitute for another.
- `worth-topo` owns topology compiled-product family declarations, topology
  admitted compiled-product inputs, selected equivalence families for
  topology-derived products, and topology reuse-denial or reuse-admission proof.
- `worth-spatial` owns spatial compiled-product family declarations, spatial
  admitted compiled-product inputs, selected equivalence families for evidence
  lookup products and retained spatial products, and spatial reuse-denial or
  reuse-admission proof.
- `worth-kernel` owns workload-composition cutover, public closeout pressure,
  residue classification, and proof that no caller-owned pseudo-reuse shortcut
  can bypass the selected compiled-product/equivalence lane.
- This milestone is only successful if it strengthens the unified architecture
  for the remaining roadmap families:
  - Milestone 15 must be able to explain public proof and diagnostics from the
    compiled-product lane without reopening local reuse logic
  - Milestone 16 must be able to prove cross-family parity using the same
    semantic-graph vocabulary already used here
  Any design that solves cache locally but leaves later families unable to
  consume the same contract is architecturally incorrect even if tests pass.
- `forge-query` remains the owner of Query support posture, boundary envelopes,
  projection consumption identity, Consumer Kit proof, and lower-runtime
  boundary truth. This milestone must use real Query surfaces instead of local
  support folklore wherever reuse claims cross Query-backed read-product
  boundaries.
- A compiled product is derived state, never authority. Reuse may skip rebuild
  of a derived product only when the equivalence contract says the prior
  compiled product remains semantically valid for the current admitted input.
  Reuse never promotes derived representation back into authoritative truth.
- Source authority digest must mean the authoritative-basis digest declared by
  the family, not an arbitrary hash of convenient retained rows. The family
  must declare whether that digest covers:
  - authoritative graph truth only
  - authoritative graph truth plus required stage/evidence authority
  - authoritative graph truth plus prior-proof basis that shapes the product
  Two families with different declared authority basis must not share the same
  semantic "source authority digest" lane.
- Locality footprint identity must mean the exact locality basis that bounds
  semantic validity for the compiled product family. A family must declare
  whether its footprint is shaped by touched closure, invalidation closure,
  evidence neighborhood, grouped batch footprint, materialization target
  footprint, or another named semantic locality basis. "Region-ish digest" is
  not an acceptable category.
- Prior-proof identity must declare its semantic role per family:
  - validity precondition only
  - product-shaping basis
  - equivalence dimension
  - reuse-denial witness only
  A proof that only authorizes product construction must not silently become an
  equivalence dimension, and a proof that changes product meaning must not be
  treated as advisory-only context.
- Reuse denial is a first-class product. A lane that proves "must rebuild" has
  succeeded. Silent fallback from failed reuse to local helper comparison is
  forbidden.
- Deletion is part of the milestone. Product-local cache keys, pointer-identity
  shortcuts, row-count heuristics, broad replay-equivalence scans, retained
  helper stability assumptions, and public raw compiled-product constructors
  must be deleted, capped, or denied before closeout.

## Implicit Requirements Made Explicit

- Covered reuse means every ordinary production path that currently decides
  whether a derived product, retained product, replay product, evidence lookup
  product, or closeout/read-model product may be reused, suppressed, compared,
  refreshed lazily, or carried forward.
- Covered pseudo-reuse also includes any path that does not say "cache" but
  still claims a semantic shortcut such as "same basis", "same derived rows",
  "same retained replay", "same shape", "same surface", "same receipt family",
  or "same read basis" without a typed compiled-product identity contract.
- Non-covered reuse must be explicitly named as certification-only, historical
  support residue, report/document codec support, test fixture support, or
  Query-gap. It cannot disappear from inventory because it is "rare" or "just
  diagnostics."
- Compiled-product identity and equivalence policy are not the same thing. The
  identity says what product we are talking about. The equivalence policy says
  under which declared differences reuse remains semantically honest.
- Equivalence, compatibility, and reuse must stay distinct:
  - equivalence answers whether two product identities mean the same product
  - compatibility answers whether two non-identical products may still
    participate in a downstream workflow together
  - reuse answers whether the current ordinary lane may consume the prior
    product instead of rebuilding
  No implementation may collapse these into one enum, one digest comparison, or
  one boolean decision.
- Benign ordering noise is allowed only when the selected equivalence family
  declares comparator behavior that tolerates it. Stable-looking rows do not
  grant implied ordering forgiveness.
- "Benign ordering noise" must be declared per family and per compared product
  dimension. It may only mean one of the following:
  - product semantics are set-like and ordering is non-semantic
  - ordering is presentation-local and declared outside the product basis
  - ordering is canonicalized after family-declared normalization
  It may not mean "the rows happened to look stable enough."
- Batch-admission identity and conflict identity from Milestone 13 are in-scope
  prior proof where grouped execution or retained grouped products claim reuse.
- Grouped identity from Milestone 13 must declare one of three semantic roles
  per compiled-product family:
  - grouped execution changes the product meaning and therefore participates in
    compiled-product identity
  - grouped execution does not change product meaning but constrains reuse
    legality
  - grouped execution is irrelevant to the family and must not participate
  "Maybe grouped identity matters here" is not an acceptable family posture.
- Workload composition, retained replay consumers, evidence lookup consumers,
  public closeout, and public read-model helpers are in-scope consumers. The
  milestone is not done if the new equivalence products exist but those
  consumers still teach older pseudo-reuse semantics.
- Compiled-product routing must look like the other registered family lanes in
  the roadmap, not like a helper library:
  - family catalog declares applicability once
  - admitted input binds semantic basis once
  - selected family and basis product lower once
  - reuse decision executes once
  - cutover and closeout consume the typed result
  If the implementation lets each caller decide how to compare, tolerate
  ordering, or interpret rebuild-vs-reuse, the milestone has failed the
  roadmap's unified architecture target.
- The spec must keep the remaining family coverage visible during execution.
  This milestone does not close diagnostics or public proof, but every product
  and seed it emits must be shaped so those later families can consume it
  without local reinterpretation.

## Named Existing Surfaces We Must Design Against

The milestone must design from current real surfaces, not generic crate
references.

**Query-owned orientation and proof surfaces**
- `crates/forge-query/docs/AI_README.md` core rule:
  `declare intent once -> lower it once -> execute or inspect it through canonical runtime-owned artifacts`
- `ForgeQueryWorkspace`
- `workspace.public_support_matrix()`
- `workspace.public_api_contract()`
- `workspace.public_handle_contract()`
- `workspace.admit_public_api_family(...)`
- `project_workspace_support_snapshot(...)`
- `support_pinning_contract(...)`
- `hard_prohibition_boundary_audit()`
- `query_consumer_residue_audit()`
- `consume_projection_facts(...)`
- `declare_projection_fact_consumption(...)`
- `forge_query_domain(...).for_lower_runtime_boundary_envelope(...)`
- `forge_query_domain(...).for_lower_runtime_boundary_source(...)`
- `ForgeQueryDeclarationEnvelopeInput`
- `ForgeQueryDeclarationEnvelope`

**Current Worth seed and adjacent reuse-bearing surfaces**
- `LookupConsumedWorkloadComposition`
- `EvidenceLookupConsumedWorkloadHandoff`
- `CoplanarOverlapWorkloadOperator`
- `CoplanarOverlapOperatorReceipt`
- `RetainedReplayWorkload`
- `RetainedReplayWorkloadReceipt`
- `current_evidence_lookup_public_closeout()`
- Milestone 10 invalidation selected-plan and execution-receipt seeds
- Milestone 11 evidence lookup selected-plan, index-product, and execution
  receipt seeds
- Milestone 12 replay scope, undo scope, transaction packet, and
  `AdmittedBooleanSplitReplayUndoBoundary`
- Milestone 13 selected conflict-plan identity, independence-proof identity,
  selected batch-admission plan identity, batch-admission execution receipt
  identity, residue digest, and firewall digest

**Required compiled-product family classes**
- topology-derived materialization products
- evidence lookup index products
- retained replay products
- replay-support compiled products that survive beyond one immediate execution
- workload-retained products
- public closeout or read-model products that currently claim stable semantic
  sameness

Every in-scope implementation path must classify each of these family classes
as migrated ordinary lane, capped residue, certification-only, or Query-gap.
The milestone may add more classes, but it may not silently narrow below this
set.

**Required displaced-lane inventory**
- For every in-scope product family class, the spec and implementation plan
  must name:
  - the old folder or module lane being displaced
  - the new parallel folder or module lane being introduced
  - the first vertical slice that cuts from old to new
  - the cutover proof
  - the deletion or residue-capping closeout

No family class may be satisfied by "refactor the current module in place until
it looks better."

**Remaining-family consumption obligations**
- replay/undo and transaction consumers from Milestone 12 must be able to name
  compiled products through this lane rather than through local replay
  equivalence folklore
- grouped conflict and batch identity from Milestone 13 must participate here
  through explicit shared contract roles rather than grouped-local cache rules
- Milestone 15 public proof and diagnostics must be able to consume the seed
  emitted here without reopening topology, evidence, or report-row comparison
- Milestone 16 parity proof must be able to treat compiled-product reuse as one
  more registered semantic-graph family rather than a special-case subsystem

**Current equivalence and derived-product seed surfaces already in the tree**
- `build_derived_equivalence_contract(...)`
- `build_derived_equivalence_contract_report(...)`
- `compare_derived_equivalence_contracts(...)`
- `snapshot.equivalence_contract()`
- `historical_equivalence_read_basis_facts()`
- `MilestoneThreeDerivedReuseLegalityRow`

The spec must use these current surfaces as the nearest honest seeds where
applicable. "Use something from this crate" is not acceptable design language.

## Directory Skeleton

The milestone should converge toward this explicit parallel lane shape before
cutover:

- `crates/worth-schema/src/semantic_graph/`
  - `compiled_product_vocabulary/`
    shared compiled-product identity, equivalence-policy identity,
    compatibility basis, reuse basis, mismatch-locus vocabulary, and
    authority/derivation distinction
  - `route_identity/`
    product identity, equivalence identity, reuse identity, and future
    planner/public-proof seed identity surfaces
- `crates/worth-topo/src/semantic_graph_routing/`
  - `compiled_product/`
    - `family_catalog/`
    - `admitted_input/`
    - `selected_plan/`
    - `compiled_product_identity/`
    - `reuse_decision/`
    - `operator_cutover/`
    - `public_closeout/`
    - `source_firewall/`
- `crates/worth-spatial/src/semantic_graph_routing/`
  - `compiled_product/`
    - `family_catalog/`
    - `admitted_input/`
    - `selected_plan/`
    - `compiled_product_identity/`
    - `reuse_decision/`
    - `stage_cutover/`
    - `public_closeout/`
    - `source_firewall/`
- `crates/worth-kernel/src/workload_composition/`
  - `compiled_product_cutover/`
    workload-composition consumers, retained-workload cutover, residue ledger,
    public closeout pressure, and source firewalls
  - `public_closeout/`
    seed assembly for Milestone 15 without local reuse reinterpretation
- `crates/worth-*/tests/touched_graph_compiled_product/`
  family-local hostile tests, compile-fail fences, and cutover proof

This skeleton is intentionally aligned with the roadmap's target lifecycle
shape:

```text
family_catalog
-> admitted_input
-> selected_plan
-> compiled_product
-> reuse_decision
-> cutover / public_closeout / source_firewall
```

Milestone 14 may choose narrower folder names during implementation, but it may
not invent an alternate topology where compiled-product identity lives as a
sidecar helper outside the shared semantic-graph routing shape.

Milestone 14 may also not satisfy this skeleton by editing old reuse helpers in
place until they happen to resemble the target shape. The cutover must create
new parallel ownership lanes first, then move callers, then delete the old
lanes.

If exact module names shift during implementation, the ownership boundaries may
not shift with them.

## Shared Compiled-Product Contract

Milestone 14 must converge around one shared cross-crate contract owned by
`worth-schema`. This is the unification center for the whole milestone.

That contract must be the only ordinary vocabulary for:
- compiled-product family identity
- source authority digest identity
- touched digest identity
- locality-footprint identity
- prior-proof participation identity
- validator/evidence-set identity
- stage identity
- batch/conflict seed identity when grouped reuse depends on Milestone 13 proof
- equivalence-policy identity
- reuse decision posture
- reuse-denial witness
- semantic breadth counters for reuse evaluation

Every compiled-product family record must declare, at minimum:
- product family identity
- authoritative source basis kind
- source authority digest recipe
- touched/locality basis kind
- locality-footprint digest recipe
- prior-proof participants and semantic role of each participant
- validator/evidence-set participation posture
- stage participation posture
- grouped-identity participation posture
- comparator family identity
- canonical ordering contract
- benign-ordering-noise posture, if any
- reuse-result posture space
- rebuild-required witness shape
- reuse-denial witness shape
- semantic breadth counter surface

If any one of these fields is missing, the family is mechanically incomplete
and may not enter the ordinary catalog.

Operationally, the contract must behave like this:

```text
touched/spatial authority + prior proof + source authority digest
-> shared compiled-product identity contract
-> topo/spatial family selection and product identity
-> selected equivalence family and comparator posture
-> typed reuse-admit / rebuild / deny result
-> kernel cutover and Milestone 15 seed
```

This shared contract is not only a cache lane. It is the compiled-product
instance of the roadmap's larger unification rule:

```text
registered semantic-graph family declares applicability once
planner or selector lowers it once
execution consumes only the lowered product
later diagnostics/public proof explain only the lowered product
```

Milestone 14 must therefore leave behind one ordinary compiled-product lane
that Milestones 15 and 16 can treat exactly the way earlier milestones treat
read families, validators, invalidation families, evidence lookup families,
replay families, and conflict families.

This means:
- `worth-topo` and `worth-spatial` must publish family declarations and
  product/reuse proof in terms of the shared contract
- `worth-kernel` must admit reuse or rebuild only through the shared contract
  and the proof products lowered from it
- public closeout, diagnostics, and later planner-owned public explanation must
  bind to the shared contract identities rather than re-deriving product
  sameness from topology, evidence rows, or local reports
- no crate may invent a second ordinary equivalence ontology even if the local
  implementation is tempting
- rendered output equality may participate in comparison only when the family
  explicitly declares it as one comparison dimension. Matching rows or payloads
  without that declaration can at most act as diagnostics evidence, never as
  equivalence proof.
- future planners, diagnostics, and public proof surfaces must be able to name:
  - which compiled-product family matched
  - which basis dimensions participated
  - which mismatch locus forced rebuild or denial
  - which consumer cutover lane consumed the result
  without reopening local family helpers

## Phase Plan

### Phase 1: Reuse And Pseudo-Reuse Folklore Inventory And Cut Line

Freeze every current reuse, suppression, parity, retained-stability, and
"same-enough" surface before replacement code is written. Every cache helper,
derived reuse helper, replay-equivalence helper, retained-product stability
shortcut, pointer-identity shortcut, row-count heuristic, provenance check, and
public closeout identity shortcut must be classified as migrate, delete, cap,
certification-only, or Query-gap.

This phase is one closeout boundary, but it contains three required inventory
cuts that must all land before Phase 2 starts:
- current authority inventory across topo, spatial, kernel, and public closeout
  execution paths
- Query/support/proof inventory across Query-backed read-product and projection
  identity surfaces touched by those paths
- cut-line and residue classification that names exactly what migrates into the
  new lane versus what is capped or deleted

**Relevant subsystems**
- `worth-topo` derived topology and historical equivalence surfaces
- `worth-spatial` evidence lookup, retained replay, and workload-product
  surfaces
- `worth-kernel` workload composition, lookup-consumed workload handoff, and
  public closeout surfaces
- Query-backed consumer and projection-consumption surfaces where reuse claims
  already cross runtime boundaries

**Relevant APIs**
- `build_derived_equivalence_contract(...)`
- `build_derived_equivalence_contract_report(...)`
- `compare_derived_equivalence_contracts(...)`
- `snapshot.equivalence_contract()`
- `historical_equivalence_read_basis_facts()`
- `LookupConsumedWorkloadComposition`
- `EvidenceLookupConsumedWorkloadHandoff`
- `RetainedReplayWorkloadReceipt`
- `current_evidence_lookup_public_closeout()`
- Milestone 13 public closeout seed identity
- Query Consumer Kit boundary-audit and residue-audit surfaces

**Required deliverables**
- one typed inventory ledger row for every in-scope product-class/caller pair
- one cut-line table grouping inventory rows by:
  - migrated ordinary family lane
  - capped non-ordinary residue
  - certification-only support
  - Query-gap
- one Query boundary table naming, for each Query-backed row:
  - projection-consumption lane
  - lower-runtime boundary-envelope lane
  - Consumer Kit proof lane
  - support-pinning lane
  - not-applicable
- one architecture-alignment table mapping each in-scope row to the roadmap's
  unified lifecycle shape:
  - current displaced surface
  - new family catalog lane
  - new admitted-input lane
  - new selected-plan lane
  - new reuse-decision lane
  - cutover/public-closeout/source-firewall destination
- one displaced-folder ledger naming, for each migrated family class:
  - old folder/module lane
  - new parallel folder/module lane
  - first migrated vertical slice
  - caller cutover boundary
  - deletion boundary

**Warnings**
- This phase is not a grep-only audit. It must produce typed inventory rows
  with dispositions and removal triggers.
- Do not classify a pointer-identity or row-count path as harmless because
  current datasets are small. That is the exact scaling failure this milestone
  exists to eliminate.
- Do not let "closeout helper" hide ordinary authority. If it can justify
  semantic sameness for production work, it is in scope.

**Test requirements**
- `reuse_inventory_has_no_keep_rows`: every ordinary reuse, suppression,
  pseudo-equivalence, and retained-stability helper has exactly one migrate,
  delete, cap, certification-only, or Query-gap disposition.
- `unclassified_reuse_surface_fails_closeout`: adding a new cache helper,
  replay-equivalence shortcut, retained-product stability helper, or public
  closeout sameness shortcut without an inventory row fails closeout.
- `inventory_rows_preserve_source_identity`: semantically similar old paths in
  distinct source locations produce distinct inventory rows so deletion cannot
  collapse unrelated authority.

**Engineering decisions**
- Inventory rows must carry source path, old authority kind, current caller,
  disposition, replacement phase, blocker, removal trigger, and whether the row
  is certification-only.
- Inventory rows must also carry exact existing surface identity when one
  already exists, such as `EvidenceLookupConsumedWorkloadHandoff`,
  `RetainedReplayWorkloadReceipt`, or `MilestoneThreeDerivedReuseLegalityRow`.
- Inventory is closeout pressure only. It may not seed admitted compiled-
  product inputs.
- Inventory rows must also carry compiled-product family class and whether the
  row currently claims:
  - equivalence
  - compatibility only
  - ordinary reuse
  - rebuild suppression
  - diagnostic-only sameness
- Inventory rows must also carry:
  - displaced folder/module lane
  - target parallel folder/module lane
  - whether the current implementation still shares a file with old semantics

Rows that still share a module with displaced semantics after cutover are not
closed; they remain open until the old lane is deleted or capped as residue.
- Inventory rows must also carry the future-family dependency they feed:
  - replay/undo
  - grouped conflict/batch
  - public proof/diagnostics
  - parity closeout
  so later milestones cannot accidentally miss a consumer family this
  milestone should have shaped.

**Open questions**
- None.

### Phase 2: Shared Compiled-Product Identity And Equivalence Vocabulary

Freeze the shared semantic-graph vocabulary that compiled-product identity and
equivalence use so later phases do not smuggle product-local cache semantics
into the new lane.

This phase does not merely publish terms. It freezes the one shared
compiled-product contract that every later phase consumes.

**Relevant subsystems**
- `worth-schema` shared semantic-graph vocabulary
- `worth-topo` touched closure, validator/invariant, and invalidation identity
  surfaces
- `worth-spatial` spatial touch authority, evidence lookup, and retained replay
  identity surfaces
- Milestone 12 replay/undo scope and transaction packet identity surfaces
- Milestone 13 conflict, independence, and batch-admission identity surfaces

**Relevant APIs**
- touched entities, relations, aspects, locality scopes, and digests
- replay scope identities
- undo scope identities
- transaction packet identities
- selected conflict-plan identity
- independence-proof identity
- selected batch-admission plan identity
- batch-admission execution receipt identity
- Query projection-consumption identity surfaces where read-product identity
  must remain typed across Query-backed boundaries

**Warnings**
- Compiled-product identity is not a string digest bucket. It must preserve
  semantic distinctions that can change reuse posture.
- Do not reuse raw digests or strings where authority class differs.
- Do not let equivalence policy collapse into product identity or vice versa.

**Test requirements**
- `compiled_product_identity_is_stable_under_rerun`: identical semantic product
  inputs produce stable compiled-product identity across reruns and benign
  ordering noise.
- `wrong_authority_cannot_mint_compiled_product_identity`: raw strings, copied
  digests, or foreign authority values cannot construct compiled-product or
  equivalence-policy identity.
- `authority_identity_and_product_identity_remain_distinct`: identical bytes
  with different authority class or lifecycle produce distinct identities.

**Engineering decisions**
- Put shared compiled-product identity and equivalence-policy distinctions in
  `worth-schema` semantic-graph routing vocabulary, not in crate-local helper
  modules.
- The vocabulary must name source authority digest, touched digest, locality
  footprint, prior-proof participation, validator/evidence set, stage identity,
  equivalence-policy identity, and reuse-decision posture separately.
- Distinguish compiled-product identity from reuse-result identity. They may
  share some fields, but they are not the same semantic claim.
- Add explicit shared vocabulary for:
  - equivalence basis
  - compatibility basis
  - reuse basis
  - freshness requirement
  - mismatch locus
  - rendered-output comparison posture

These are separate semantic categories and may not be encoded as comments on a
generic comparator type.

**Open questions**
- None.

### Phase 3: Topology Compiled-Product Family Catalog

Build the topology compiled-product family catalog beside displaced derived
topology reuse helpers before any ordinary topology consumer is migrated.

**Relevant subsystems**
- new `worth-topo` compiled-product family lane
- `worth-topo` touched closure, invalidation, and replay-support seed surfaces

**Relevant APIs**
- touched closure proof products
- invalidation execution receipt identity
- replay scope identity
- transaction packet identity
- current derived-topology equivalence surfaces

**Warnings**
- A topology compiled-product family catalog is source truth for topology-side
  applicability and required identity. It is not a callback list.
- Do not let topology family identity come from operator names, report names,
  or old cache enum labels.

**Test requirements**
- `topology_family_declaration_applies_to_multiple_matching_products`: one
  topology family declaration applies to at least two matching topology-derived
  products without editing those consumers.
- `topology_family_missing_identity_fields_cannot_enter_catalog`: a topology
  family declaration missing touched-locality applicability, prior-proof
  posture, or equivalence-policy posture cannot enter the catalog.
- `topology_raw_strings_cannot_mint_family_identity`: raw strings and copied
  receipt digests cannot mint topology compiled-product family identity.

**Engineering decisions**
- Topology product families must lower through the shared compiled-product
  contract, not a topo-local equivalence ontology.
- The topology family-catalog implementation must land in the new parallel
  folder lane, not by extending displaced helper modules.

**Open questions**
- None.

### Phase 4: Spatial And Evidence Compiled-Product Family Catalog

Build the spatial/evidence compiled-product family catalog beside displaced
evidence-index, retained-replay, and pseudo-reuse helpers before any ordinary
spatial consumer is migrated.

**Relevant subsystems**
- new `worth-spatial` compiled-product family lane
- evidence lookup, retained replay, and grouped-stage seed surfaces

**Relevant APIs**
- spatial touch authority
- evidence lookup execution receipt identity
- retained replay workload receipt
- selected conflict-plan identity and batch-admission execution receipt

**Warnings**
- Do not let evidence lookup products, retained replay products, and grouped
  spatial support products share one vague “spatial cache family” category.
- Do not let spatial family identity come from stage labels or old workload
  helper names.

**Test requirements**
- `spatial_family_declaration_applies_to_multiple_matching_products`: one
  spatial family declaration applies to at least two matching spatial/evidence
  products without editing those consumers.
- `spatial_family_missing_identity_fields_cannot_enter_catalog`: a spatial
  family declaration missing touch, prior-proof, or equivalence posture cannot
  enter the catalog.
- `spatial_raw_strings_cannot_mint_family_identity`: raw strings and copied
  evidence or replay receipts cannot mint spatial compiled-product identity.

**Engineering decisions**
- Spatial/evidence families must classify evidence index products, retained
  replay products, and grouped support products explicitly rather than hiding
  them under one local umbrella.
- The spatial family-catalog implementation must land in the new parallel
  folder lane, not by extending displaced helper modules.

**Open questions**
- None.

### Phase 5: Kernel Consumer Dependency Matrix And Cutover Catalog

Freeze the kernel-side consumer dependency matrix that says which ordinary
consumers will later cut over to compiled-product routing, and which product
families and proof bases they depend on.

**Relevant subsystems**
- `worth-kernel` workload composition
- public closeout and read-model consumers
- Query-backed projection or lower-runtime boundary consumers

**Relevant APIs**
- `LookupConsumedWorkloadComposition`
- `EvidenceLookupConsumedWorkloadHandoff`
- `current_evidence_lookup_public_closeout()`
- Query projection-consumption and boundary-envelope surfaces

**Warnings**
- Do not hide consumer dependency classification inside the later sweep phase.
- Do not treat kernel cutover as a passive downstream detail of topo/spatial
  family work.

**Test requirements**
- `kernel_consumer_matrix_classifies_every_ordinary_consumer`: every ordinary
  kernel or public consumer is mapped to one compiled-product family class or
  explicit residue posture.
- `consumer_matrix_rejects_unbound_product_dependencies`: a consumer that uses
  product sameness without bound family-class and proof-basis metadata fails.
- `query_backed_consumers_name_real_query_boundary_lane`: every Query-backed
  consumer is classified by real Query boundary lane rather than prose.

**Engineering decisions**
- This phase owns the kernel-facing cutover catalog and must land before any
  admission or execution sweep starts.
- Consumer dependency classification must live in a new kernel cutover lane,
  not as comments or helper tables in old modules.

**Open questions**
- None.

### Phase 6: Topology Admitted Compiled-Product Input Lane

Freeze topology admitted compiled-product input as a proof-bearing lane that
accepts only topology touched authority, declared source authority basis, and
declared prior-proof classes.

**Relevant subsystems**
- `worth-topo` touched closure and invalidation/replay identity surfaces
- topology compiled-product family catalog

**Relevant APIs**
- topology touched closure proof products
- invalidation execution receipts
- replay scope identity
- transaction packet identity

**Warnings**
- Do not let topology admission quietly accept public closeout rows or
  diagnostics rows as compiled-product substitutes.
- Do not reopen broad topology scans during admission.

**Test requirements**
- `topology_equivalent_inputs_admit_to_same_identity`: topology admitted input
  identity is stable across reruns when semantic inputs match.
- `topology_wrong_receipt_or_foreign_authority_is_rejected`: mismatched receipt
  family, stage identity, or foreign authority is rejected before family
  selection.
- `topology_diagnostic_rows_cannot_act_as_input`: observability artifacts
  cannot enter topology admitted input.

**Engineering decisions**
- Topology admission is its own proof product and must live in the new admitted-
  input lane.
- Topology admission must consume current Milestone 12 and 13 proof surfaces
  rather than rebuilding them locally.

**Open questions**
- None.

### Phase 7: Spatial Admitted Compiled-Product Input Lane

Freeze spatial admitted compiled-product input as a proof-bearing lane that
accepts only spatial touch authority, evidence/replay-support receipts, and
declared grouped or retained-workload proof classes.

**Relevant subsystems**
- `worth-spatial` spatial touch authority, evidence lookup, retained replay,
  and grouped-stage identity surfaces
- spatial compiled-product family catalog

**Relevant APIs**
- evidence lookup execution receipt identity
- retained replay workload receipt
- selected conflict-plan identity
- batch-admission execution receipt

**Warnings**
- Do not let spatial admission quietly accept broad evidence scans, report
  rows, or manual retained workload claims.
- Do not collapse evidence index and retained replay proof inputs into one vague
  “spatial support” lane.

**Test requirements**
- `spatial_equivalent_inputs_admit_to_same_identity`: spatial admitted input
  identity is stable across reruns when semantic inputs match.
- `spatial_wrong_receipt_or_manual_support_is_rejected`: wrong receipt family,
  foreign authority, or manual support substitute is rejected before family
  selection.
- `spatial_diagnostic_rows_cannot_act_as_input`: observability artifacts cannot
  enter spatial admitted input.

**Engineering decisions**
- Spatial admission is its own proof product and must live in the new admitted-
  input lane.
- Spatial admission must consume current Milestone 12 and 13 proof surfaces
  rather than rebuilding them locally.

**Open questions**
- None.

### Phase 8: Selected Equivalence Family And Comparator Contract

Freeze equivalence selection as a family-catalog-driven lane that lowers from
admitted compiled-product input to a selected equivalence family with explicit
comparator, canonical ordering, and acceptable-ordering-noise posture.

**Relevant subsystems**
- topology and spatial compiled-product family catalogs
- shared compiled-product identity contract
- kernel cutover pressure over grouped and retained products

**Relevant APIs**
- admitted compiled-product input identity
- compiled-product family catalog digests
- Milestone 13 grouped identity seed
- `compare_derived_equivalence_contracts(...)` as a current closest seed for
  explicit comparison posture

**Warnings**
- Do not let equality-of-digest masquerade as semantic equivalence unless the
  selected family declares that basis explicitly.
- Do not let comparator logic hide broad row reconstruction or broad
  materialization fallback.
- Do not let selected equivalence family depend on pointer stability.

**Test requirements**
- `same_admitted_input_selects_same_equivalence_family`: identical admitted
  input and family catalog yield stable selected equivalence family identity.
- `ordering_noise_is_allowed_only_when_family_declares_it`: changing row order
  preserves equivalence only for families that explicitly admit ordering noise.
- `different_touched_or_locality_identity_denies_equivalence_even_when_rows_match`:
  superficially similar rows cannot override changed locality or touched proof.

**Engineering decisions**
- Selected equivalence family is a first-class product, not a helper choice.
- Family declarations must distinguish exact equivalence, declared benign
  ordering tolerance, fresh rebuild required, advisory-only match, and denied
  reuse.
- Selected equivalence output must bind:
  - equivalence basis identity
  - compatibility basis identity, if distinct
  - reuse basis identity
  - freshness requirement posture
  - rendered-output comparison posture

This phase is incomplete if it selects only a comparator function without these
semantic basis products.
- This phase must also bind the selected family to one future public-proof seed
  identity so Milestone 15 can explain reuse from the same lowered family
  result instead of a second diagnostic-only route.
- Selection logic must land in the new selected-plan lane. Leaving selection in
  an old helper while only moving the types is not cutover.

**Open questions**
- None.

### Phase 9: Reuse Decision Product And Rebuild Denial Product

Freeze execution of reuse posture as a typed result-bearing product rather than
letting equivalence selection end at comparator choice. The execution lane must
preserve the admitted compiled-product identity, selected equivalence family
identity, selected comparator posture, and semantic breadth counters needed for
closeout and Milestone 15 seed proof.

**Relevant subsystems**
- new topology and spatial reuse-decision lanes
- kernel cutover consumers
- current derived-equivalence certification surfaces

**Relevant APIs**
- admitted compiled-product input identity
- selected equivalence family identity
- `build_derived_equivalence_contract_report(...)`
- `compare_derived_equivalence_contracts(...)`
- current derived-topology equivalence inspection counters

**Warnings**
- Do not let reuse collapse into a boolean hit/miss result.
- Do not let rebuild-required become an implicit fallback path without typed
  witness.
- Do not emit reuse decisions that can be fabricated without the admitted input
  and selected family chain.
- Do not let the executor compare only final rows unless the selected family
  explicitly declares rendered output equality as part of the reuse basis.

**Test requirements**
- `reuse_decision_binds_identity_and_family_chain`: reuse result binds admitted
  compiled-product identity, selected equivalence family identity, and decision
  posture.
- `rebuild_required_is_first_class_not_fallback`: rebuild-required decisions
  expose typed witness and counters instead of silently disappearing behind a
  cache miss.
- `reuse_counters_are_semantic_not_generic`: result exposes family-meaningful
  breadth counters instead of one undifferentiated comparison count.
- `reuse_denial_localizes_mismatch_locus`: denial witness localizes whether the
  failure came from family mismatch, authority-basis mismatch, locality-basis
  mismatch, prior-proof mismatch, ordering-contract mismatch, freshness
  requirement, or declared non-reusability.

**Engineering decisions**
- Reuse-decision execution result is its own product, not an annotation on the
  selected equivalence family.
- Result postures must distinguish at least `ReuseAdmitted`, `FreshRebuildRequired`,
  `AdvisoryMatchRequiresRebuild`, and `Denied`.
- The reuse executor must take exactly:
  - admitted current compiled-product input
  - one prior compiled-product identity candidate
  - the selected equivalence family/basis product
  - any family-declared comparison material

The executor must emit:
  - reuse-decision posture
  - mismatch loci or freshness loci
  - the compared basis identities
  - semantic breadth counters

It may not consult undeclared caches, broad row scans, or report projections.
- The executor's output must be shaped so later public closeout and diagnostics
  can consume it directly. It may not require a second translation layer that
  maps local mismatch classes into public proof terms.
- Execution logic must land in the new reuse-decision lane. Old cache helpers
  may not remain the real executor underneath a thin adapter.

**Open questions**
- None.

### Phase 10: First Vertical Migration Slice

Migrate one real ordinary compiled-product slice through the new lanes before
the broad sweep. The first slice must prove the full path from admitted
authority to selected compiled-product family, admitted input, selected
equivalence family, reuse decision, and ordinary execution posture.

**Relevant subsystems**
- one covered topology-derived or spatial/evidence-derived product path
- `worth-kernel` workload composition
- compiled-product and equivalence lanes

**Relevant APIs**
- admitted compiled-product inputs
- selected equivalence families
- reuse-decision results
- Milestone 13 seed identity where grouped products participate

**Warnings**
- Do not pick a certification-only path because it is easy.
- Do not skip directly to the sweep without one real parity slice.
- Do not keep the old helper active behind a silent adapter once the slice is
  cut over.

**Test requirements**
- `vertical_slice_matches_or_strengthens_old_reuse_posture`: the migrated slice
  produces the same or stronger reuse/rebuild/deny outcome as the displaced
  path.
- `old_reuse_helper_cannot_satisfy_migrated_slice`: once cut over, the slice
  cannot route through the displaced pseudo-reuse helper.
- `slice_preserves_authority_vs_derived_distinction`: the migrated slice proves
  derived compiled-product identity is not treated as authoritative truth.

**Engineering decisions**
- The first migrated slice must consume real Milestone 12 and 13 proof products.
- The first migrated slice should prefer an already-real seed surface such as
  `EvidenceLookupConsumedWorkloadHandoff`, `RetainedReplayWorkloadReceipt`, or
  current derived-topology equivalence surfaces rather than a synthetic
  greenfield path.
- The first migrated slice should also maximize cross-family architectural
  leverage: prefer a slice whose output later public proof, replay support, or
  grouped workload lanes will actually consume, so the milestone validates the
  roadmap's unified architecture instead of only local parity.
- The first slice must prove real module cutover:
  - caller imports the new lane
  - old helper is no longer on the ordinary path for that slice
  - deletion or residue row is created immediately for the displaced path

**Open questions**
- None.

### Phase 11: Topology-Derived Consumer Cluster Cutover

Cut the topology-derived materialization consumers to the new lanes. This phase
is topology-only and must end with ordinary topology consumers either migrated,
deleted, or capped.

**Relevant subsystems**
- topology-derived product consumers
- topology compiled-product lane
- kernel cutover consumers that consume topology-derived products

**Relevant APIs**
- selected compiled-product family identity
- admitted topology compiled-product input identity
- selected equivalence family identity
- reuse-decision results

**Warnings**
- Do not mix spatial/evidence consumers into this phase.
- Do not let topology consumers remain on old helper imports while other
  families migrate.

**Test requirements**
- `topology_consumers_route_through_reuse_decision_products`: covered ordinary
  topology consumers use the new compiled-product chain or explicit residue.
- `topology_cutover_preserves_zero_broad_scan_fallback`: topology cutover does
  not revive broad topology comparison or legacy helper fallback.
- `topology_residue_rows_are_exact_and_non_authoritative`: remaining topology
  residue is counted, owned, blocked, and denied as ordinary proof.

**Engineering decisions**
- This phase closes the topology consumer cluster only.
- The phase is incomplete if any ordinary topology consumer still imports a
  displaced helper module.

**Open questions**
- None.

### Phase 12: Spatial, Evidence Index, And Retained-Replay Consumer Cluster Cutover

Cut the spatial/evidence/replay-support consumers to the new lanes. This phase
is spatial-side and must end with ordinary spatial consumers either migrated,
deleted, or capped.

**Relevant subsystems**
- evidence lookup index-product consumers
- retained replay product consumers
- grouped workload retained-product consumers
- spatial compiled-product lane

**Relevant APIs**
- selected spatial compiled-product family identity
- admitted spatial compiled-product input identity
- selected equivalence family identity
- reuse-decision results

**Warnings**
- Do not hide grouped workload retained-product cutover as a “later follow-up”
  inside the public sweep.
- Do not mix public closeout/read-model consumers into this phase.

**Test requirements**
- `spatial_consumers_route_through_reuse_decision_products`: covered ordinary
  spatial/evidence/replay consumers use the new compiled-product chain or
  explicit residue.
- `spatial_cutover_preserves_zero_broad_scan_fallback`: spatial cutover does
  not revive broad evidence comparison or retained helper fallback.
- `spatial_residue_rows_are_exact_and_non_authoritative`: remaining spatial
  residue is counted, owned, blocked, and denied as ordinary proof.

**Engineering decisions**
- This phase closes the spatial/evidence/replay consumer cluster only.
- The phase is incomplete if any ordinary spatial or retained-replay consumer
  still imports a displaced helper module.

**Open questions**
- None.

### Phase 13: Query-Backed, Public Closeout, And Read-Model Consumer Cluster Cutover

Cut the Query-backed consumers, public closeout consumers, and read-model
consumers to the new lanes. This phase isolates the public and boundary-crossing
consumers so they do not hide under the general sweep.

**Relevant subsystems**
- public closeout/read-model consumers
- Query-backed projection-consumption consumers
- Query lower-runtime boundary consumers
- kernel public closeout pressure

**Relevant APIs**
- `consume_projection_facts(...)`
- `declare_projection_fact_consumption(...)`
- `forge_query_domain(...).for_lower_runtime_boundary_envelope(...)`
- `forge_query_domain(...).for_lower_runtime_boundary_source(...)`
- selected equivalence family identity
- reuse-decision identity

**Warnings**
- Do not claim the ordinary cutover is done while public or Query-backed
  consumers still reinterpret sameness locally.
- Do not bury Query boundary cutover under generic public closeout wording.

**Test requirements**
- `query_backed_and_public_consumers_route_through_reuse_decision_products`:
  Query-backed and public consumers use the new chain or explicit residue.
- `public_cutover_does_not_reopen_local_equivalence_logic`: public closeout and
  read-model consumers do not compare rows or cache keys locally after cutover.
- `query_boundary_residue_rows_are_exact`: any remaining Query-backed or public
  residue is counted, owned, blocked, and denied as ordinary proof.

**Engineering decisions**
- This phase closes the boundary-crossing consumer cluster only.
- The phase is incomplete if public closeout or Query-backed consumers still
  depend on displaced helper imports or local comparison logic.

**Open questions**
- None.

### Phase 14: Ordinary Consumer Sweep Closeout

Close the broad sweep only after the topology, spatial, and public/Query-backed
consumer clusters have each closed honestly.

**Relevant subsystems**
- `worth-kernel` workload composition
- topology-derived product consumers
- evidence lookup and retained replay consumers
- public closeout or read-model consumers that currently imply stable product
  identity

**Relevant APIs**
- selected compiled-product family identity
- admitted compiled-product input identity
- selected equivalence family identity
- reuse-decision results
- residue and closeout ledgers

**Warnings**
- This phase is not the migration itself; it is the sweep closeout gate.
- Do not use this phase to hide cluster-local unfinished work.

**Test requirements**
- `all_covered_consumers_route_through_reuse_decision_products`: every covered
  ordinary consumer uses the new compiled-product/equivalence chain or has
  explicit non-ordinary residue denial.
- `parallel_cutover_preserves_zero_broad_scan_fallback`: ordinary cutover does
  not revive broad topology scans, broad evidence scans, or broad row
  comparison fallback.
- `residue_rows_are_exact_and_non_authoritative`: every remaining non-ordinary
  consumer is counted, owned, blocked, and denied as ordinary reuse proof.

**Engineering decisions**
- Sweep completion is an acceptance boundary, not cleanup.
- Any generic bridge from old pseudo-reuse helpers to new reuse-decision
  products counts as residue and must be capped or deleted in this milestone.
- Each required consumer cluster above must receive its own cutover ledger
  section naming:
  - ordinary migrated callers
  - deleted callers
  - capped residue callers
  - Query-gap callers
- Each cluster ledger must also name which later roadmap family would be
  blocked or forced into local reinterpretation if the cluster were left on old
  semantics. This keeps the sweep aligned with the unified architecture instead
  of optimizing for easiest local migrations.
- This sweep may not "share implementation temporarily" by letting ordinary
  callers continue to import displaced helper modules. Ordinary callers must
  import the new parallel lane or be explicitly marked as residue.

**Open questions**
- None.

### Phase 15: Source Firewalls, Constructor Denials, And Hard Deletion

Cut every covered ordinary reuse consumer to the new lanes. This is the broad
migration phase: every covered consumer must become compiled-product/equivalence
driven, be deleted, or be mechanically capped as non-ordinary residue.

This phase is broad by consumer count, not by authority shape. The authority is
already frozen by earlier phases. Implementation should therefore execute this
phase as a series of explicit consumer-cluster cutovers rather than one opaque
sweep.

**Relevant subsystems**
- `worth-kernel` workload composition
- topology-derived product consumers
- evidence lookup and retained replay consumers
- public closeout or read-model consumers that currently imply stable product
  identity

**Required consumer clusters**
- topology-derived materialization consumers
- evidence lookup index-product consumers
- retained replay product consumers
- grouped workload retained-product consumers
- public closeout/read-model consumers
- Query-backed projection-consumption or lower-runtime boundary consumers whose
  current behavior implies stable derived-product sameness

**Relevant APIs**
- selected compiled-product family identity
- admitted compiled-product input identity
- selected equivalence family identity
- reuse-decision results
- residue and closeout ledgers

**Warnings**
- This phase is not done when the new lanes are available but optional.
- Do not leave "temporary" product-local cache keys in ordinary paths.
- Do not let grouped consumers re-derive sameness from report rows when the
  selected reuse decision says rebuild or deny.

**Test requirements**
- `all_covered_consumers_route_through_reuse_decision_products`: every covered
  ordinary consumer uses the new compiled-product/equivalence chain or has
  explicit non-ordinary residue denial.
- `parallel_cutover_preserves_zero_broad_scan_fallback`: ordinary cutover does
  not revive broad topology scans, broad evidence scans, or broad row
  comparison fallback.
- `residue_rows_are_exact_and_non_authoritative`: every remaining non-ordinary
  consumer is counted, owned, blocked, and denied as ordinary reuse proof.

**Engineering decisions**
- Sweep completion is an acceptance boundary, not cleanup.
- Any generic bridge from old pseudo-reuse helpers to new reuse-decision
  products counts as residue and must be capped or deleted in this milestone.
- Consumer-cluster slicing is expected here, but each batch must terminate at
  the same admitted-input, selected-family, and reuse-decision authority chain.
- Each required consumer cluster above must receive its own cutover ledger
  section naming:
  - ordinary migrated callers
  - deleted callers
  - capped residue callers
  - Query-gap callers
- Each cluster ledger must also name which later roadmap family would be
  blocked or forced into local reinterpretation if the cluster were left on old
  semantics. This keeps the sweep aligned with the unified architecture instead
  of optimizing for easiest local migrations.
- This sweep may not "share implementation temporarily" by letting ordinary
  callers continue to import displaced helper modules. Ordinary callers must
  import the new parallel lane or be explicitly marked as residue.

**Open questions**
- None.

Install firewalls before closeout and delete or cap the displaced old paths.
This phase prevents reintroduction of product-local cache keys, pointer
identity shortcuts, row-count heuristics, replay-equivalence folklore, retained
stability helpers, caller-authored reuse claims, and public constructor forgery.

**Relevant subsystems**
- old cache and equivalence helpers
- public proof and closeout facades
- compiled-product source-firewall lanes

**Relevant APIs**
- source-firewall reports
- compile-fail fixtures
- residue and deletion ledgers

**Warnings**
- Firewall success is not a substitute for deletion.
- Do not allow one generic "equivalence helper" to survive as an unofficial
  second authority lane.
- Public callers must not be able to forge family rows, admitted inputs,
  selected equivalence families, reuse decisions, or closeout products.
- Do not call old modules from the new lane after cutover except through
  explicitly capped residue seams. A new folder that still executes old
  semantics underneath is not a real cutover.

**Test requirements**
- `source_firewall_rejects_reuse_folklore_revival`: forbidden semantic
  surfaces for pointer identity, row-count heuristics, report-row equality,
  retained stability folklore, and caller-authored reuse cannot reappear on
  covered paths.
- `public_api_cannot_forge_compiled_product_or_reuse_products`: compile-fail
  fixtures reject public construction of family records, admitted inputs,
  selected equivalence families, reuse decisions, and closeout products.
- `deletion_ledger_binds_firewall_report`: deletion closeout consumes the
  firewall report digest while still naming concrete deleted or capped surfaces.

**Engineering decisions**
- Source firewall must ban old authority by semantic surface, not only exact
  symbol names.
- Residue rows must carry owner, exact count, blocker, and removal trigger.
- Deletion closeout must name the displaced folder/module lanes explicitly and
  prove they are either:
  - deleted
  - reduced to capped non-ordinary residue
  - mechanically firewalled from ordinary callers

**Open questions**
- None.

### Phase 16: Public Closeout And Milestone 15 Seed

Publish Milestone 14 only after covered ordinary consumers route through real
compiled-product and equivalence products, old authority is deleted or capped,
and the new products expose enough identity for Milestone 15 planner-owned
public proof and diagnostics work to start without product-sameness
rediscovery.

**Relevant subsystems**
- `worth-kernel` public closeout pressure
- topology compiled-product closeout
- spatial compiled-product closeout
- workload-composition compiled-product cutover closeout

**Relevant APIs**
- admitted compiled-product input identity
- selected equivalence family identity
- reuse-decision identity
- residue and deletion ledgers
- source-firewall reports
- Milestone 15 seed surfaces

**Warnings**
- Do not claim planner-owned public explanation completion here.
- Do not let closeout be satisfied by diagnostics strings or local reports.
- Do not let the seed omit the product identity and reuse witness Milestone 15
  will need for public proof and routing localization.

**Test requirements**
- `milestone_fourteen_closeout_requires_real_cutover`: final closeout fails if
  any covered ordinary consumer still depends on old pseudo-reuse authority.
- `closeout_binds_full_compiled_product_authority_chain`: closeout digests bind
  touched or spatial authority, source authority digest, prior-proof inputs,
  selected equivalence families, reuse decisions, residue rows, and firewall
  proof.
- `milestone_fifteen_seed_carries_product_identity_without_rediscovery`: the
  seed carries enough compiled-product and reuse identity to start planner-owned
  public proof without rescanning topology, evidence, or report rows.

**Engineering decisions**
- Emit a Milestone 15 seed with admitted compiled-product identity, selected
  equivalence-family identity, reuse-decision identity, residue digest, and
  firewall digest.
- The Milestone 15 seed must also carry:
  - reuse basis identity
  - freshness requirement posture
  - mismatch-locus vocabulary or emitted mismatch witness identity
  - rendered-output comparison posture
  - per-family semantic reuse counters

Milestone 15 must not need to rediscover why reuse was admitted, rebuilt, or
denied.
- Final closeout consumes proof products only, never raw collections, display
  rows, or local cache-key strings.
- Final closeout must also emit an architecture-alignment report proving:
  - compiled-product families now live in the shared semantic-graph routing
    lifecycle
  - later roadmap families can consume the emitted seed without local
    reinterpretation
  - no covered ordinary lane still treats cache/equivalence as a separate local
    subsystem
- Final closeout must also prove the parallel-cutover law was honored:
  - every migrated family class has a named new lane
  - ordinary callers import that new lane
  - displaced lanes are deleted or capped
  - no ordinary caller still depends on an in-place-refactored legacy module

**Open questions**
- None.

## Must Ship

- A typed inventory of current cache, reuse, suppression, retained-stability,
  replay-equivalence, and public-closeout pseudo-reuse authority surfaces with
  migrate, delete, cap, certification-only, or Query-gap disposition.
- Shared semantic-graph compiled-product identity and equivalence vocabulary in
  `worth-schema`, including authority-safe identity distinctions for source
  truth, touched locality, prior proof, compiled-product family, equivalence
  policy, and reuse decision.
- Parallel topology and spatial compiled-product lanes with family catalogs,
  admitted inputs, selected equivalence families, and typed reuse-decision
  products.
- A parallel kernel-owned cutover lane with workload-composition consumers,
  retained-product cutover, residue ledger, and public closeout pressure.
- At least one real ordinary migrated vertical slice proving end-to-end cutover
  from admitted proof to selected equivalence family and typed reuse decision.
- A complete ordinary consumer sweep so every covered reuse consumer is
  migrated, deleted, or capped as non-ordinary residue.
- Source firewalls, compile-fail fences, deletion ledger, residue ledger, and
  public constructor denials that prevent product-local cache keys, pointer
  identity shortcuts, row-count heuristics, retained helper stability folklore,
  report-row equality shortcuts, and raw proof construction from returning.
- A Milestone 15 seed carrying admitted compiled-product identity, selected
  equivalence-family identity, reuse-decision identity, residue digest, and
  firewall digest.
- An architecture-alignment report or equivalent proof product showing that the
  compiled-product lane now fits the roadmap's unified semantic-graph routing
  model and target directory skeleton.
- A displaced-folder closeout report proving old lanes were not merely
  refactored in place, but were actually replaced by new parallel lanes and
  then deleted or capped.

## Must Preserve

- Touched closure and spatial touch authority remain the primary locality and
  changed-meaning authority.
- Milestone 10 invalidation receipts, Milestone 11 evidence lookup receipts,
  Milestone 12 replay/undo and transaction packets, and Milestone 13 conflict,
  independence, and batch-admission seeds remain prior-proof inputs. They must
  not be recertified, widened, or substituted.
- Compiled products remain derived state. Reuse never upgrades a compiled
  product into authoritative truth.
- Execution may consume lowered compiled-product and reuse-decision products
  only. Executors may not rediscover sameness from rows, labels, or broad
  scans.
- Reuse decisions remain derived from admitted inputs and selected equivalence
  families. They may not become a back door for reclassifying source authority
  after the fact.
- Diagnostics and public closeout remain derived observability products, not
  compiled-product or reuse authority.
- Deletion and residue honesty remain first-class closeout requirements.
- The roadmap's declare-once routing target remains the governing shape:
  callers may consume compiled-product routing products, but may not author
  local cache keys, local equivalence rules, local reuse postures, or local
  mismatch ontologies on covered ordinary paths.
- The roadmap's migration execution law remains the governing implementation
  shape: new parallel folders first, caller cutover second, deletion or residue
  closeout third. In-place authority migration is not an acceptable substitute.

## Acceptance Evidence

- Tests prove inventory completeness and reject unclassified cache, reuse,
  retained-stability, or pseudo-equivalence surfaces.
- Tests prove raw strings, copied digests, diagnostics rows, and caller-authored
  guesses cannot enter admitted compiled-product input.
- Tests prove selected equivalence-family identity is deterministic from
  admitted proof and family declarations, and that unrelated families remain
  unselected.
- Tests prove ordering tolerance exists only when the family declares it, and
  that changed touched closure, locality footprint, evidence set, validator
  set, or batch/conflict seed identity deny equivalence even when rendered rows
  look similar.
- Tests prove reuse decisions bind the admitted-input and selected-family chain
  while exposing semantic breadth counters instead of generic hit/miss totals.
- Tests prove the first migrated slice matches or strengthens prior reuse or
  rebuild posture while preserving authority-versus-derived distinction.
- Tests prove every covered ordinary consumer routes through compiled-product
  and reuse-decision products or explicit non-ordinary residue.
- Tests prove source firewalls and compile-fail fixtures reject reintroduction
  of product-local cache keys, pointer identity, row-count heuristics,
  retained helper stability folklore, report-row equality shortcuts, and raw
  proof construction.
- Tests prove Milestone 15 can start from the emitted seed without rescanning
  topology, evidence, or report rows to rediscover compiled-product authority.
- Tests prove the directory and module topology reflects the shared semantic-
  graph routing lifecycle rather than a one-off cache helper subsystem.
- Tests or topology audits prove later roadmap families can consume the seed
  from this milestone without inventing local reinterpretation layers.
- Tests or topology audits prove ordinary callers now import new parallel lanes
  and that displaced legacy modules were not simply rewritten in place and left
  as the real authority path.

## Sequencing Notes

- Milestone 14 belongs immediately after Milestone 13 because compiled-product
  identity and reuse posture must consume typed replay/undo scope, transaction
  packets, and grouped conflict/batch seeds rather than pre-scope local
  conventions.
- It belongs before Milestone 15 because planner-owned public proof,
  diagnostics, and explainers need authoritative compiled-product and reuse
  identity before they can localize why a route was reused, rebuilt, or denied.
- It should not attempt Milestone 15 planner-owned public explanation closure
  beyond emitting the typed seed that later milestone consumes.

## Required Self-Check

- Does the milestone solve a real structural problem or just package work
  cosmetically? Yes. It replaces product-local reuse folklore with declared
  compiled-product identity and equivalence proof.
- Is the adversarial constraint precise and load-bearing? Yes. It rejects
  pointer identity, row-count heuristics, rendered-shape similarity, retained
  helper stability, broad row comparison, and provenance folklore under local
  edit pressure.
- Does the roadmap justify this milestone now? Yes. Milestone 13 already made
  grouped conflict and batch identity real typed inputs, and Milestone 15 needs
  stable compiled-product/reuse identity before public proof can be honest.
- Does the spec preserve crate authority boundaries? Yes. `worth-schema` owns
  shared compiled-product identity and equivalence vocabulary, `worth-topo` and
  `worth-spatial` own product-family authority, and `worth-kernel` owns cutover
  pressure and public closeout pressure.
- Are the phases carrying most of the real design information? Yes. The design
  payload lives in the sixteen ordered phases.
- Is each phase centered on one conceptual detail or boundary? Yes: inventory,
  shared vocabulary, topology family catalog, spatial family catalog, kernel
  consumer dependency matrix, topology admission, spatial admission,
  equivalence selection, reuse-decision product, first slice, topology
  consumer cutover, spatial consumer cutover, Query/public consumer cutover,
  ordinary sweep closeout, firewall/deletion, and closeout/seed.
- Does each phase contain at least 2 adversarial tests by default? Yes.
- Could a competent engineer map this spec into honest types, modules, and
  tests? Yes. The spec names the proof products, old authority being displaced,
  cutover rules, firewalls, residue posture, and the next milestone seed.
- Does the milestone belong in this roadmap sequence, or is it out of order?
  It belongs here because reuse must become compiled-product- and
  equivalence-routed before planner-owned public proof and cross-family parity
  can unify.
