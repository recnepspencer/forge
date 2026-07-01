# Milestone 15: Planner-Owned Routing, Public Proof, And Diagnostics

## Goal

Freeze the ordinary public and diagnostic path so planner-owned routing
products, not executor-local rediscovery or local explainer folklore, are the
single authority for explaining why replay, undo, conflict, cache,
invalidation, evidence, and read-routing decisions happened.

## Semantic Definitions

Milestone 15 is only honest if the public-proof and diagnostic vocabulary below
has one fixed meaning.

- **Prior-proof input**: a typed artifact already proven by earlier milestones
  and consumed here without reclassification. Prior-proof inputs may justify a
  route, but they are not themselves route explanation products.
- **Family-route product**: a declare-once family-specific explanation product
  derived from prior-proof inputs and family declarations. It states what a
  given family contributes to route meaning before planner selection.
- **Selected route identity**: the canonical machine identity of the chosen
  routing meaning. It names the winning route meaning, not a human-readable
  message, not a public projection, and not a bag of convenient fields.
- **Selected-route packet**: the sealed planner authority artifact carrying the
  selected route identity plus the exact family, product, witness, Query, and
  residue references the planner used. It is authoritative planner output for
  route meaning.
- **Public-proof input**: a sealed derived assembly input lowered directly from
  a selected-route packet for public-proof construction. It is not a second
  planner authority lane and it may not be caller-constructed.
- **Public-proof projection**: a read-only derived public artifact that explains
  authoritative route meaning without becoming the authority for that meaning.
- **Derived-diagnostic projection**: an artifact-policy-selected derived
  localization artifact that elaborates authoritative route meaning with richer
  touched, aspect, witness, and posture detail.
- **Witness identity**: the machine identity of the exact acceptance, denial,
  mismatch, or advisory fact used by planner-owned routing. A witness is not a
  rendered reason string.
- **Decision-trace identity**: the machine identity of the planner decision
  chain record associated with a selected route. It is the identity of the
  decision chain, not merely the identity of one formatted trace view.
- **Public-proof identity**: the projection identity of one public-proof
  artifact derived from one selected-route packet and one artifact policy. It
  is a derived projection identity, not proof-of-proof authority.
- **Derived-diagnostic contract identity**: the projection contract identity
  that determines which rich localization fields a diagnostic projection may
  add beyond the minimal receipt-backed truth.
- **Route explanation**: the machine-explainable answer to all covered
  ordinary-path questions:
  - why this family was selected
  - why another family was not selected or was denied
  - why reuse was admitted, denied, or downgraded to compatibility-only
  - why Query posture was sufficient or insufficient where Query participates
  - why any residue remained after cutover
  If any of those answers still requires local rediscovery outside the
  planner-owned lane, route explanation is not closed.
- **Inspection**: read-only observation of final public-proof or
  derived-diagnostic projections. Inspection may expose selected route identity,
  selected family identity, selected product identity where applicable, witness
  identity, prior-proof references the facade explicitly allows, and
  human-facing explanation fields carried by the projection. Inspection may not
  expose raw constructors, mutable planner packets, caller-authored route
  assembly, or hidden support seams.
- **Query-gap**: a residue classification meaning the remaining debt is caused
  by Query boundary capability rather than Worth-local refusal to cut over.
  Query-gap must be classified as one of:
  - required Query artifact does not yet exist
  - artifact exists but is not admitted on the public supported path
  - artifact is admitted but not exposed at the boundary this milestone needs
  - artifact exists and is exposed, but it does not yet preserve the identity
    semantics this milestone requires
  Query-gap is not a synonym for vague future work.

## Why This Milestone Exists

Milestones 12 through 14 turn replay scope, conflict independence, batch
admission, compiled-product identity, and reuse posture into typed semantic
products.

That still leaves one dangerous escape hatch: ordinary execution and public
explanation can remain locally correct while privately re-deciding the same
meaning from receipts, rows, helper conventions, or crate-local summaries.

If that survives, Worth will have built a semantic-graph kernel whose public
and diagnostic surfaces still behave like a pile of local explainers.

Milestone 15 exists to close that gap. It makes planner-owned routing the one
explanation authority for covered ordinary paths and forces public proof and
diagnostics to consume the same lowered products execution already consumed.

This milestone is not a sidecar observability program. It is the next lowered
form of the same semantic-graph routing model already established by the
roadmap:

```text
touched graph facts and aspect vocabulary define what meaning changed
registered families declare applicability once
planner-owned routing intersects those proofs once
execution consumes the selected route once
public proof and diagnostics lower from that same selected route once
```

If Milestone 15 creates a second architecture where "public explanation"
becomes its own local ontology beside touched authority, aspects, replay,
conflict, compiled products, and Query posture, then the roadmap has already
fractured before parity closeout.

## Governing Summaries

- `MENTALITY.md`: protect the hard problem first. The hard problem here is not
  "make diagnostics nicer"; it is "make execution-time and public-facing route
  meaning impossible to rediscover locally."
- `arch_laws.md`: protect type-declared contracts, lowered-plan execution,
  self-describing envelopes, decision-log authority, and authority/derivation
  separation. Public proof must consume lowered routing products and derived
  diagnostics must remain derived.
- `composition_laws.md`: protect responsibility-named decomposition. Planner
  routing products, public proof assembly, public facades, diagnostic
  projection, source firewalling, and residue closeout must not collapse into
  one mega "closeout" lane.
- `domain_structure_laws.md`: protect directory truth. The tree must visibly
  distinguish planner-owned routing contracts, public proof products,
  diagnostic projections, Query boundary evidence, and source firewalls.
- `perf_laws.md`: protect semantic-delta-bounded execution and forbid
  re-deciding policy on the hot path. Execution and public reads must consume
  narrowed routing products instead of rescanning receipts, rows, or support
  state to explain themselves.
- `touched-graph-roadmap.md`: protect the roadmap sequence and milestone role.
  This milestone belongs after compiled-product/reuse closure and before
  cross-family parity because planner-owned public proof needs stable typed
  routing products before parity can be honestly certified.
  The strongest expectation from the roadmap is that planner-owned routing is
  not a separate subsystem family. It is the one shared lowering boundary that
  public proof, diagnostics, replay, conflict, invalidation, evidence, and
  reuse all consume as instances of the same semantic-graph language.
- `crates/forge-query/docs/AI_README.md`: protect the Query rule
  `declare intent once -> lower it once -> execute or inspect it through canonical runtime-owned artifacts`.
  Milestone 15 must apply that rule to public proof, routing explanation, and
  diagnostics.

## Adversarial Constraint

Worth must survive long mixed operation chains where topology-derived reads,
spatial evidence lookup, replay/undo scope, grouped conflict, batch admission,
compiled-product reuse, Query-backed read products, and public closeout all
participate in one ordinary path while only a small locality footprint
actually changes.

If any covered executor, public closeout API, proof surface, or diagnostic
surface can still explain its route by:

- reopening topology or evidence internals after planning
- scanning receipts or raw rows to rediscover what family was selected
- deriving "why" from local strings, helper reports, or support wrappers
- rebuilding reuse meaning from local cache posture rather than planner-owned
  products
- exposing constructor or mutation seams that let callers fabricate authority,
  plan identity, or proof artifacts

then the milestone has failed.

For this milestone, "explain its route" explicitly includes:
- why this family was selected
- why a competing family was not selected
- why reuse was admitted, denied, or compatibility-only
- why Query posture was sufficient or insufficient
- why residue remained after cutover

An implementation that closes only the success path explanation but leaves any
of those denial, posture, or residue answers to local helper folklore is not
complete.

## Product Decision Lock

- Milestone 15 is a parallel-cutover milestone. Build new planner-owned
  routing, public proof, and diagnostic lanes beside displaced local explainers,
  closeout helpers, report assemblers, support-ceremony wrappers, and
  executor-side route rediscovery before cutting callers.
- The required cutover law is:
  1. inventory the exact current public-proof, diagnostic, and explainer lanes
  2. create a new responsibility-named parallel lane
  3. migrate one vertical slice through typed routing products
  4. cut ordinary callers to the new lane
  5. delete the displaced lane or cap exact residue with owner, blocker, and
     removal trigger
  6. install source firewalls and compile-fail fences so local explanation
     ceremony cannot silently revive
- In-place refactoring is not an acceptable strategy for the authority
  transition. Mechanical edits inside an already-created new lane are allowed
  only after that lane exists as a real ownership boundary.
- The following do **not** count as parallel cutover:
  - keeping the old folder and gradually reshaping it until it "becomes" the
    new lane
  - introducing new types inside the old module and calling that migration
  - wrapping a legacy helper with a new facade while the legacy helper remains
    the ordinary authority path
  - splitting one oversized legacy file into smaller files under the same
    legacy folder while keeping that folder as the real ordinary route owner
  - moving only tests or public exports first while the production authority
    path still lives in the displaced lane
- A phase is not allowed to claim cutover progress until all of the following
  are true for that phase's family:
  1. the new responsibility-named folder lane exists on disk
  2. the new lane contains the owning production types for that responsibility
  3. one ordinary caller imports the new lane directly
  4. the displaced lane is no longer the write location for new ordinary-path
     logic
- Once a new lane exists, all subsequent ordinary-path edits for that
  responsibility must land in the new lane. Editing the displaced lane for new
  ordinary behavior after the new lane exists is a QA failure unless the edit
  is a deletion, an import cutover, or an explicit residue cap.
- The milestone lifecycle shape is:
  `routing_inventory -> shared_explanation_contract -> family_route_products -> planner_selected_route -> public_proof_input -> derived_diagnostic_projection -> public_facade -> source_firewall -> closeout`.
- The milestone must preserve the roadmap's unified semantic-graph routing
  model:
  - touched graph and aspect vocabulary remain the upstream meaning authority
  - family route products remain declare-once family instances, not
    caller-authored explainers
  - planner selection remains the one place where matching family meanings are
    intersected
  - public proof and diagnostics remain downstream projections over selected
    route authority
  - no covered family may invent a second local explanation language after
    planner selection
- `worth-schema` owns the shared explanation vocabulary:
  selected family identity, selected product identity, selected route identity,
  denial or advisory posture, mismatch locus vocabulary, witness identity,
  decision-trace identity, and public-proof identity contracts.
- `worth-topo` owns topology read-routing, invalidation, and query-backed
  read-model explanation products.
- `worth-spatial` owns evidence lookup, spatial closeout, and retained spatial
  explanation products.
- `worth-kernel` owns planner-owned routing selection, workload-composition
  public proof assembly, public diagnostic projection selection, public facades,
  source firewalls, and residue ledgers.
- `forge-query` remains the owner of Query support posture, public runtime
  facade truth, lower-runtime boundary evidence, support-matrix admission, and
  boundary envelopes. Milestone 15 must consume those Query artifacts rather
  than reproducing them in Worth-local wrappers.
- Public proof and diagnostics are derived observability, never authority.
  Operational receipts and lowered route products remain authoritative for
  routing decisions; public artifacts explain them.
- The explanation artifact chain must be explicit and shared across families:

```text
prior_proof_inputs
-> admitted_explanation_input
-> family_route_product
-> selected_route_packet
-> public_proof_input
-> public_proof_projection | derived_diagnostic_projection
```

Only the planner-owned lane may construct `admitted_explanation_input`,
`family_route_product`, `selected_route_packet`, or `public_proof_input`.
Public callers and downstream surfaces may inspect only the final projection
artifacts.
- Target directory skeleton:

```text
crates/worth-schema/src/data/authority/touched_graph_planner_routing/
  mod.rs
  admitted_explanation_input.rs
  selected_route_identity.rs
  selected_family_identity.rs
  decision_trace_identity.rs
  denial_witness.rs
  public_proof_contract.rs
  derived_diagnostic_contract.rs

crates/worth-kernel/src/workload_composition/planner_owned_routing/
  mod.rs
  family_catalog/
  admitted_public_proof_input/
  selected_route/
  public_proof/
  derived_diagnostics/
  public_facade/
  source_firewall/

crates/worth-topo/src/projection/planner_owned_routing/
  mod.rs
  query_backed_read_family/
  invalidation_route/
  diagnostic_projection_input/

crates/worth-spatial/src/workload_platform/planner_owned_routing/
  mod.rs
  evidence_lookup_route/
  public_closeout_route/
  diagnostic_projection_input/
```

Mandatory directory rules:
- `worth-schema/.../touched_graph_planner_routing/` is required and owns the
  shared route and projection contract types.
- `worth-kernel/.../planner_owned_routing/selected_route/`,
  `public_proof/`, `derived_diagnostics/`, `public_facade/`, and
  `source_firewall/` are required and may not be collapsed into one file or
  folder.
- `worth-topo/.../planner_owned_routing/query_backed_read_family/` and
  `invalidation_route/` are required if topology remains an ordinary route
  contributor.
- `worth-spatial/.../planner_owned_routing/evidence_lookup_route/` and
  `public_closeout_route/` are required if spatial remains an ordinary route
  contributor.
- The new planner-owned folders must be created before the corresponding legacy
  folders are split, renamed, or partially rewritten. "We will create the new
  folder after cleaning up the old one" is out of spec.
- Legacy folders may be edited only for:
  - import cutover into the new lane
  - deletion
  - residue capping
  - compile-fail or firewall fencing
  They may not remain the place where new ordinary-path planner, proof, or
  diagnostic behavior is authored.

Displaced lanes that this milestone is expected to replace or cap include:
- `crates/worth-kernel/src/workload_composition/public_closeout/*` local
  route-local explanation ownership
- `crates/worth-topo/src/projection/query_backed_consumer_cutover/*` local
  route rediscovery and mixed closeout logic
- `crates/worth-topo/src/projection/diagnostic_surfaces/*` as ordinary route
  explanation authority
- `crates/worth-spatial/src/workload_platform/evidence_lookup_public_closeout/*`
  current-source and local closeout explanation ownership

The exact filenames may vary, but the mandatory folders, ownership axes, and
displaced-lane responsibilities may not.

## Phase Plan

### Phase 1: Public Proof, Diagnostic, And Explainer Inventory Cut Line

Freeze every current receipt-backed status, public closeout, diagnostic,
explainer, and route-localization surface before introducing replacement lanes.

**Relevant subsystems**
- `worth-kernel` workload composition and public closeout
- `worth-topo` projection diagnostics and query-backed cutover
- `worth-spatial` evidence lookup public closeout
- `forge-query` runtime-backed public/support surfaces

**Relevant APIs**
- `current_worth_touched_graph_conflict_public_closeout`
- `WorthTouchedGraphConflictProofChain`
- `DerivedReadDiagnostics`
- `current_evidence_lookup_public_closeout`
- Query support posture and boundary-envelope surfaces from `AI_README.md`

**Warnings**
- Do not treat grep counts as an honest inventory.
- Do not bucket multiple ordinary surfaces under one vague "public diagnostics"
  row.
- Do not leave current receipt-backed public proof lanes unnamed just because
  they already look polished.

**Test requirements**
- `planner_owned_routing_inventory_is_scope_complete`: fails if any covered
  public proof, status, explainer, or diagnostic surface lacks a named
  migration row with `migrate`, `delete`, `cap`, or `Query-gap` posture.
- `ordinary_public_surfaces_cannot_hide_local_route_rediscovery`: fails if a
  covered public or diagnostic surface still depends on local helper
  rediscovery that is absent from the inventory.

**Engineering decisions**
- Inventory rows must name current module path, public surface, current
  authority source, displaced lane, future lane, and residue posture.
- Inventory rows must also classify each surface into the shared semantic-graph
  lifecycle:
  - prior-proof input consumer
  - family-route product
  - planner-selected route consumer
  - public-proof projection
  - derived-diagnostic projection
  - forbidden legacy explainer
- Inventory must classify which surfaces become:
  - planner-owned public proof products
  - planner-owned derived diagnostics
  - internal-only debug or certification views
  - deleted local ceremony
  - Query-gap residue
- Inventory rows must also record the exact parallel target folder that will be
  created before cutover starts. A row without a named new folder lane is not
  ready for implementation.

**Open questions**
- None.

### Phase 2: Shared Planner-Owned Explanation Vocabulary

Land one shared contract lane for route explanation identity before any family
tries to explain itself locally.

**Relevant subsystems**
- `worth-schema`
- `worth-kernel`
- `worth-topo`
- `worth-spatial`

**Relevant APIs**
- Milestone 14 seed and proof-chain products
- compiled-product identity and reuse-decision identity contracts
- Query machine-identity and stop-class rules from `AI_README.md`

**Warnings**
- Do not collapse selected route identity, selected product identity, and
  public-proof identity into one digest.
- Do not treat human-readable reason strings as machine explanation authority.
- Do not let lower-authority report rows mint denial or witness identity.

**Test requirements**
- `planner_explanation_identity_preserves_authority_distinctions`: selected
  route identity, selected family identity, denial-witness identity,
  public-proof identity, and decision-trace identity remain distinct even when
  they share the same textual reason.
- `rendered_strings_cannot_mint_route_explanation_identity`: raw labels,
  `Display`, or debug output cannot construct planner-owned explanation
  artifacts.

**Engineering decisions**
- `worth-schema` must define typed contracts for:
  - admitted explanation input
  - selected route identity
  - selected family identity
  - selected product identity
  - denial or advisory witness identity
  - mismatch-locus vocabulary
  - decision-trace identity
  - public-proof identity
  - derived-diagnostic projection contract identity
- The shared vocabulary must encode whether an explanation product is:
  - authoritative planner output
  - prior-proof input
  - derived public projection
  - derived diagnostic projection
- The vocabulary must also freeze the semantic differences between:
  - route identity: identity of the chosen routing meaning
  - selected-route packet: sealed authority artifact carrying the chosen route
  - public-proof projection: public derived explanation over the selected route
  - derived-diagnostic projection: artifact-policy-selected rich localization
    over the selected route

**Open questions**
- None.

### Phase 3: Planner Route Trace And Public-Proof Input Packet

Create the planner-owned route packet that later execution, public proof, and
diagnostics all consume unchanged.

**Relevant subsystems**
- `worth-kernel` planner-owned routing
- `worth-kernel` public closeout
- `worth-kernel` source firewall

**Relevant APIs**
- `WorthTouchedGraphConflictMilestoneFourteenSeed`
- `WorthTouchedGraphConflictProofChain`
- source-firewall report surfaces
- residue-chain surfaces

**Warnings**
- Do not let execution reconstruct a route packet from receipts after the fact.
- Do not let public proof pull directly from raw seeds, conflict packets, or
  residue rows after this packet exists.
- Do not mix selection logic with projection formatting inside one file.

**Test requirements**
- `planner_selected_route_packet_is_sufficient_for_public_proof`: public proof
  assembly succeeds from the selected route packet without reopening topology,
  evidence, or reuse internals.
- `selected_route_packet_rejects_missing_prior_proof`: construction fails when
  required Milestone 12-14 prior-proof identities are absent or mismatched.

**Engineering decisions**
- The packet must carry:
  - prior-proof identities from Milestones 12 through 14
  - selected route identity
  - selected family and product identity
  - denial or advisory witness identity where applicable
  - Query support or posture witness identity where applicable
  - residue digest and firewall digest references
- Packet construction must be sealed to the planner-owned lane.
- The packet is authoritative planner output for explanation meaning, not a
  public projection and not a caller-constructible transport bundle.
- Every later phase must consume the packet by identity or by sealed wrapper
  over that identity; no later phase may mint a second route packet from the
  same underlying facts.

**Open questions**
- None.

### Phase 4: Query-Backed Read Routing Explanation Lane

Move query-backed read-family explanation out of local closeout and read-model
helpers into a dedicated planner-consumable route lane.

**Relevant subsystems**
- `worth-topo` query-backed consumer cutover
- `worth-topo` projection runtime boundary
- `forge-query`

**Relevant APIs**
- query-backed consumer cutover closeout surfaces
- declared Query support and boundary-traceability surfaces
- Query support-matrix and lower-runtime boundary docs named by `AI_README.md`

**Warnings**
- Do not let query-backed explanation devolve into "this row came from Query"
  strings.
- Do not re-read lower runtime support posture during public proof assembly if
  it was already admitted into the route packet.
- Do not leak host-facing Query support seams onto ordinary Worth public APIs.

**Test requirements**
- `query_backed_read_route_explanation_uses_real_query_artifacts`: route
  explanation binds admitted Query support posture and boundary evidence rather
  than Worth-local support folklore.
- `foreign_query_posture_cannot_explain_read_route`: explanation denies when
  the supplied Query posture or boundary witness does not match the selected
  route packet.

**Engineering decisions**
- `worth-topo` owns planner input products for query-backed read-family route
  explanation.
- Query-backed explanation must expose typed support, boundary, and denial
  posture to the planner without exposing raw lower-runtime internals.
- Allowed Query-derived inputs are limited to:
  - admitted Query support-posture artifacts
  - boundary-envelope or lower-runtime boundary identities
  - typed denial or stop-class witnesses
  - typed Query machine identity where the Query contract already requires it
- Forbidden Query-derived inputs include:
  - ad hoc Worth-local enums summarizing Query support
  - rendered boundary strings
  - raw lower-runtime report rows
  - caller-authored wrapper structs that restate Query admission in Worth terms
- The old `closeout.rs` style mixed route-lowering and test mutation seams must
  be displaced by a new planner route lane rather than refined in place.
- Phase 4 is not complete if the new
  `worth-topo/src/projection/planner_owned_routing/query_backed_read_family/`
  lane does not exist and ordinary callers still author route logic under
  `query_backed_consumer_cutover/`.

**Open questions**
- None.

### Phase 5: Invalidation Route Input Lane

Make invalidation explanation consumable as typed planner inputs instead of
local report reconstruction.

**Relevant subsystems**
- `worth-topo` invalidation plan and touched-closure proof
- `worth-kernel` planner-owned routing

**Relevant APIs**
- invalidation catalog and touched-closure products
- invalidation family-route products
- invalidation proof inputs from earlier milestones

**Warnings**
- Do not let broad invalidation summaries erase exact touched facts or aspect
  scope.
- Do not reopen touched closure from report rows after invalidation proof
  already exists.

**Test requirements**
- `invalidation_route_input_carries_exact_touched_fact_scope`: planner input
  preserves touched fact identity, aspect scope, and selected invalidation
  family rather than broad category labels.
- `report_rows_cannot_reconstruct_invalidation_authority`: projection rows
  alone cannot mint invalidation route input without the real prior proof.

**Engineering decisions**
- `worth-topo` must emit planner-consumable invalidation explanation inputs
  from real invalidation proof products.
- This phase owns invalidation route authority only, not diagnostic projection
  formatting.

**Open questions**
- None.

### Phase 6: Derived-Read Diagnostic Input Lane

Build the derived-read diagnostic input lane as a separate consumer of prior
planner and invalidation products instead of mixing it with invalidation-route
authority.

**Relevant subsystems**
- `worth-topo` diagnostic input products
- `worth-kernel` planner-owned derived diagnostics

**Relevant APIs**
- `DerivedInvalidationReport`
- `DerivedRebuildReport`
- `DerivedFallbackReport`
- planner-selected route packet

**Warnings**
- Do not make derived-read diagnostic inputs authoritative over invalidation
  truth.
- Do not let broad derived-read summaries erase exact touched facts or aspect
  scope.
- Do not merge route-input construction with human-facing projection
  formatting.

**Test requirements**
- `derived_read_diagnostic_inputs_localize_exact_touched_fact_scope`:
  diagnostic inputs preserve touched fact identity, aspect scope, selected
  route identity, and selected family.
- `diagnostic_inputs_require_real_invalidation_and_route_products`: a derived
  diagnostic input cannot be minted from report rows or route-local helper
  summaries alone.

**Engineering decisions**
- Derived-read diagnostic input products must separate:
  - invalidation authority
  - selected route meaning
  - projection-ready localization payload
- This phase does not yet build the final rich diagnostics surface. It only
  creates the sealed input lane consumed later.

**Open questions**
- None.

### Phase 7: Evidence Lookup Route Lane

Move evidence lookup explanation onto the planner-owned route vocabulary
instead of crate-local closeout assembly.

**Relevant subsystems**
- `worth-spatial` evidence lookup family catalog
- `worth-kernel` planner-owned routing

**Relevant APIs**
- evidence lookup family and index-product surfaces
- admitted spatial compiled-product and reuse products
- evidence lookup route explanation inputs

**Warnings**
- Do not let evidence lookup explanation re-explain reuse or rebuild from local
  retained product semantics after Milestone 14 emitted typed reuse proof.
- Do not merge evidence lookup counters with planner route authority.
- Do not let evidence rows themselves become the explanation lane.

**Test requirements**
- `evidence_lookup_route_explanation_consumes_milestone_fourteen_seed`:
  evidence lookup route explanation proves selected family, selected product,
  and reuse posture from the emitted seed without rescanning evidence rows.
- `evidence_lookup_route_denial_localizes_family_or_support_mismatch`: denial
  localizes exact route mismatch rather than broad lookup failure.

**Engineering decisions**
- `worth-spatial` owns planner input products for evidence lookup route
  explanation.
- This phase does not own public closeout projection. It only closes the
  evidence lookup route-authority lane.

**Open questions**
- None.

### Phase 8: Spatial Public-Closeout Route Lane

Move spatial public-closeout explanation onto the planner-owned route
vocabulary instead of crate-local current-source assembly.

**Relevant subsystems**
- `worth-spatial` evidence lookup public closeout
- `worth-spatial` evidence lookup family catalog
- `worth-kernel` planner-owned public proof assembly

**Relevant APIs**
- `EvidenceLookupPublicCloseout`
- `EvidenceLookupPublicCloseoutAssemblyInput`
- `current_evidence_lookup_public_closeout_assembly_input`
- evidence lookup family and index-product surfaces

**Warnings**
- Do not let spatial closeout re-explain reuse or rebuild from local retained
  product semantics after Milestone 14 emitted typed reuse proof.
- Do not merge spatial public-closeout counters with planner route authority.
- Do not leave current-source assembly as the ordinary explanation path.

**Test requirements**
- `evidence_lookup_route_explanation_consumes_milestone_fourteen_seed`: spatial
  public closeout proves selected family, selected product, and reuse posture
  from the emitted seed without rescanning evidence rows.
- `spatial_closeout_denial_localizes_family_or_support_mismatch`: denial
  localizes exact route mismatch rather than broad "lookup failed" status.

**Engineering decisions**
- `worth-spatial` owns planner input products for public-closeout route
  explanation.
- The current `assembler/current_source/closeout_artifacts` lane is displaced
  by a new planner route lane and may not remain the ordinary authority path.
- This phase is not complete if new ordinary-path closeout or explanation logic is
  still being added under
  `workload_platform/evidence_lookup_public_closeout/assembler/`,
  `current_source/`, or `closeout_artifacts/` instead of the new
  `planner_owned_routing/` lane.

**Open questions**
- None.

### Phase 9: Replay, Undo, And Transaction Explanation Route Lane

Lower replay, undo, and transaction-scope explanation into planner-consumable
route products so no caller or executor reclassifies scope semantics locally.

**Relevant subsystems**
- `worth-kernel` replay/undo workload composition
- `worth-schema` replay/undo semantic-graph authority
- planner-owned routing selection

**Relevant APIs**
- replay/undo semantic-graph basis products
- transaction and undo packet identities from Milestone 12
- workload-composition replay/undo consumers

**Warnings**
- Do not let replay explanation degrade into "same scope as before" folklore.
- Do not rebuild scope posture from current-source rows or helper summaries.
- Do not fold transaction explanation into generic diagnostic text.

**Test requirements**
- `replay_scope_explanation_is_lowered_once`: execution and public proof both
  consume the same replay or undo route product without executor-side
  reclassification.
- `mismatched_scope_packet_cannot_explain_public_route`: foreign or stale
  replay-scope packets deny before public proof assembly.

**Engineering decisions**
- Replay, undo, and transaction explanation products must remain distinct
  planner route families even when they share projection vocabulary.
- Scope explanation products must identify the exact prior-proof packet and
  route identity they were derived from.

**Open questions**
- None.

### Phase 10: Conflict And Independence Route Lane

Lower conflict and independence meaning into
planner-consumable route products without letting public proof or diagnostics
rebuild grouped-routing semantics locally.

**Relevant subsystems**
- `worth-kernel` workload composition
- `worth-schema` touched-graph conflict routing contracts

**Relevant APIs**
- conflict plan identity
- independence proof identity

**Warnings**
- Do not let public proof reinvent grouped conflict meaning from current
  operator shape or row overlap.
- Do not compress denial reasons into one generic "blocked" or "conflicted"
  posture.

**Test requirements**
- `conflict_and_independence_explanation_share_selected_route_chain`: conflict
  and independence explanation both bind the same selected route
  identity rather than parallel local ontologies.
- `denial_witness_localizes_conflict_vs_independence_failure`: diagnostics
  distinguish conflict-route denial from independence denial with
  typed witness identity.

**Engineering decisions**
- Planner route selection must treat conflict and independence as related but
  non-interchangeable route contributors.
- Public proof may describe their relationship, but it may not collapse their
  machine identities into one bucket.

**Open questions**
- None.

### Phase 11: Batch-Admission Route Lane

Lower batch-admission meaning into its own planner-consumable route products
instead of treating it as a side-effect of conflict explanation.

**Relevant subsystems**
- `worth-kernel` workload composition
- `worth-schema` touched-graph conflict routing contracts

**Relevant APIs**
- batch-admission plan identity
- grouped conflict plan identity
- batch-route witness inputs

**Warnings**
- Do not let batch-admission explanation piggyback on conflict wording while
  dropping its own admission identity.
- Do not treat batch authorization as generic grouped-routing success.
- Do not reconstruct batch route meaning from executor-local admission
  bookkeeping.

**Test requirements**
- `batch_admission_explanation_binds_selected_route_and_batch_identity`: batch
  explanation binds selected route identity and batch-admission identity
  together.
- `batch_denial_witness_remains_distinct_from_conflict_denial`: diagnostics
  distinguish batch denial from conflict or independence denial with separate
  witness identity.

**Engineering decisions**
- Batch-admission explanation is a separate family-route lane even when it
  shares prior proofs with conflict or independence.
- Public proof may show conflict and batch in one envelope, but the machine
  identities and denials must stay distinct.

**Open questions**
- None.

### Phase 12: Compiled-Product Reuse Explanation Route Lane

Lower compiled-product reuse, rebuild-required, compatibility-without-reuse,
and denial posture into planner-consumable route meaning instead of reopening
Milestone 14 cache or equivalence logic locally.

**Relevant subsystems**
- `worth-topo` compiled-product consumers
- `worth-spatial` compiled-product consumers
- `worth-kernel` planner-owned routing and public proof

**Relevant APIs**
- selected equivalence-family identity
- compiled-product reuse-decision identity
- rebuild-denial identity
- Milestone 14 seed and proof-chain products

**Warnings**
- Do not let reuse explanation bypass selected route identity and jump straight
  to cache posture.
- Do not let public proof compare rendered outputs or helper summaries to
  explain reuse after Milestone 14 closed that authority gap.
- Do not merge compatibility-without-reuse into successful reuse explanation.

**Test requirements**
- `reuse_explanation_consumes_milestone_fourteen_products_only`: public proof
  and diagnostics explain reuse, rebuild, or denial from typed Milestone 14
  products rather than local cache folklore.
- `compatibility_without_reuse_remains_distinct_from_reuse`: route explanation
  preserves compatibility posture without silently promoting it to reuse.

**Engineering decisions**
- Reuse explanation remains a planner-owned route family even though its prior
  proofs come from Milestone 14 rather than Milestone 13.
- Public proof may describe conflict and reuse together when they are both
  present, but it must preserve their separate machine identities and witness
  chains.

**Open questions**
- None.

### Phase 13: Planner-Owned Public Proof Assembly Lane

Build the public proof lane as a dedicated derived assembly boundary that
consumes selected route packets and family route products only.

**Relevant subsystems**
- `worth-kernel` public proof
- `worth-kernel` planner-owned routing
- `worth-kernel` source firewall

**Relevant APIs**
- `WorthTouchedGraphConflictProofChain`
- `WorthTouchedGraphConflictPublicCloseout`
- planner selected-route packet
- residue and firewall digests

**Warnings**
- Do not make public proof the new place where route policy is decided.
- Do not let public proof accept raw constructor input from tests or callers.
- Do not mix proof assembly, facade export, and diagnostics formatting in one
  file.

**Test requirements**
- `public_proof_chain_consumes_planner_route_products_only`: proof assembly
  fails if any covered meaning is provided as local report data instead of a
  planner-owned route product.
- `public_proof_rejects_foreign_route_or_firewall_identity`: foreign route,
  firewall, or residue identities cannot be combined into a plausible-looking
  public proof.

**Engineering decisions**
- The public proof lane is a derived artifact-policy consumer. It must not own
  planning or route admission.
- `WorthTouchedGraphConflictProofChain` and successor products must lower from
  selected-route packets and sealed planner proof inputs only.
- `public_proof/` is forbidden from importing covered meaning directly from:
  - topology diagnostic report rows
  - topology query-backed closeout-local helpers
  - spatial current-source closeout assembly
  - local support-ceremony wrappers over Query posture
  - legacy public-closeout helper modules that predate the planner-owned lane

**Open questions**
- None.

### Phase 14: Derived Diagnostic Projection Lane

Build a separate planner-owned derived diagnostics lane that localizes exact
route meaning without becoming the authority for that meaning.

**Relevant subsystems**
- `worth-kernel` derived diagnostics
- `worth-topo` diagnostic input products
- `worth-spatial` diagnostic input products

**Relevant APIs**
- planner selected-route packet
- denial-witness identity contracts
- existing derived diagnostic projections

**Warnings**
- Do not let diagnostics become required for ordinary execution correctness.
- Do not merge human-facing explanation formatting with machine explanation
  identity.
- Do not allow broad category labels where exact touched fact, aspect, or
  support mismatch is available.

**Test requirements**
- `derived_diagnostics_localize_exact_route_and_mismatch_locus`: diagnostics
  name exact selected family, selected product, touched or aspect scope,
  witness identity, and denial posture.
- `artifact_policy_can_suppress_rich_diagnostics_without_losing_operational_truth`:
  operational receipts remain valid even when rich diagnostics are disabled by
  artifact policy.

**Engineering decisions**
- Diagnostics must be selected by artifact policy and remain optional derived
  projections.
- The machine lane for denial remains typed error or stop-class identity;
  diagnostics elaborate it.
- The minimal non-diagnostic public truth that must remain available when rich
  diagnostics are disabled is:
  - selected route identity
  - selected family identity
  - selected product identity where applicable
  - denial or advisory posture identity
  - residue posture identity where applicable
- Rich diagnostics may add locality, touched-fact, aspect, witness, Query
  posture, and mismatch-locus detail, but they may not become the sole carrier
  of machine denial truth.

**Open questions**
- None.

### Phase 15: Public Read-Only Facade Boundary

Expose planner-owned proof and diagnostics through a narrow public facade that
allows inspection but not fabrication, mutation, or route-local rediscovery.

**Relevant subsystems**
- `worth-kernel` public facade
- `forge-query` public runtime facade and support posture

**Relevant APIs**
- planner-owned public proof products
- planner-owned diagnostic projection products
- Query public runtime facade and support-matrix posture surfaces

**Warnings**
- Do not export raw planner constructors.
- Do not expose support pins, report rows, or debug helper types because they
  seem useful for tests.
- Do not make the public facade mirror internal directory topology.

**Test requirements**
- `public_facade_exports_inspection_without_authority_construction`: callers
  can inspect proof and diagnostics but cannot construct planner route or proof
  products.
- `public_facade_rejects_support_wrapper_shortcuts`: public callers cannot
  satisfy the facade through support-ceremony wrappers, raw rows, or local
  explainer helpers.

**Engineering decisions**
- Public facade files may aggregate but may not implement route policy.
- Support posture and boundary evidence from Query remain visible only through
  the typed public explanation artifacts that actually need them.

**Open questions**
- None.

### Phase 16: Kernel Public-Closeout Cutover

Cut kernel public-closeout callers from mixed local closeout helpers onto the
planner-owned proof and diagnostics lanes.

**Relevant subsystems**
- `worth-kernel` public closeout
- `worth-kernel` planner-owned routing

**Relevant APIs**
- current public closeout entry points
- planner-owned public proof assembly
- planner-owned derived diagnostics
- source-firewall reports

**Warnings**
- Do not leave old public-closeout modules as the real ordinary authority path.
- Do not satisfy cutover by wrapping legacy helpers with new names.
- Do not leave current-source assembly mixed with public facade export.

**Test requirements**
- `kernel_public_closeout_routes_through_planner_owned_proof_only`: covered
  public-closeout entry points fail if planner-owned route products are absent
  and cannot silently reopen legacy helper logic.
- `public_closeout_legacy_helpers_are_deleted_or_capped`: no covered ordinary
  kernel public-closeout helper lane remains outside the planner-owned cutover
  or explicit residue ledger.

**Engineering decisions**
- Create a new planner-owned kernel cutover lane beside displaced public
  closeout helpers and move ordinary imports to that lane.
- Legacy public-closeout helpers that remain temporarily must be explicit
  residue, never silent fallback authority.
- The default displacement map is:
  - `workload_composition/public_closeout/proof_chain.rs` route-local proof
    assembly -> `planner_owned_routing/public_proof/`
  - `workload_composition/public_closeout/public_closeout.rs` ordinary public
    route explanation -> `planner_owned_routing/public_facade/`
  - `workload_composition/public_closeout/milestone_fourteen_seed.rs` as
    direct ordinary explainer input -> `planner_owned_routing/admitted_public_proof_input/`
- This phase is not complete if kernel ordinary callers still gain new behavior
  by editing `workload_composition/public_closeout/*` as the primary
  implementation location rather than cutting imports to
  `workload_composition/planner_owned_routing/*`.

**Open questions**
- None.

### Phase 17: Workload-Composition Explainer Cutover

Cut workload-composition explainers and route-local status surfaces off legacy
kernel helper lanes and onto planner-owned proof or diagnostic imports.

**Relevant subsystems**
- `worth-kernel` workload composition
- `worth-kernel` planner-owned routing
- `worth-kernel` derived diagnostics

**Relevant APIs**
- workload-composition explainer and status surfaces
- planner-owned public proof assembly
- planner-owned derived diagnostics

**Warnings**
- Do not let workload composition keep a second local explanation language
  after public-closeout cutover is complete.
- Do not treat route-local status helpers as harmless convenience surfaces if
  they still reconstruct planner meaning.
- Do not hide workload-composition route debt inside kernel-wide residue rows.

**Test requirements**
- `workload_composition_explainers_import_planner_owned_lanes`: workload
  composition explanation surfaces import planner-owned proof or diagnostic
  lanes directly.
- `workload_composition_local_explainers_are_deleted_or_capped`: no ordinary
  workload-composition explainer surface remains outside the new lane or
  explicit residue ledger.

**Engineering decisions**
- Public-closeout cutover and workload-composition explainer cutover are
  separate phases because they touch different caller families and failure
  surfaces.
- Kernel workload-composition explainer debt must be counted independently from
  public-closeout debt.

**Open questions**
- None.

### Phase 18: Topology Explainer Cutover

Cut topology callers off displaced local explainers, report builders, and
closeout-local route logic, then delete or cap the old topo lanes.

**Relevant subsystems**
- `worth-topo` query-backed cutover and diagnostics
- `worth-kernel` planner-owned route consumers

**Relevant APIs**
- current topology diagnostic surfaces
- planner-owned route input products
- residue-manifest surfaces

**Warnings**
- Do not leave the old topo files alive as ordinary helper backdoors after
  cutover.
- Do not treat test-only mutation seams as acceptable ordinary public surfaces.
- Do not cap broad mixed topo lanes as residue when they should be split and
  deleted.

**Test requirements**
- `topology_callers_import_new_parallel_route_lanes`: ordinary topology callers
  resolve through the new planner-owned route lanes, not displaced legacy
  modules.
- `deleted_topology_explainers_cannot_be_reintroduced`: compile-fail or source
  firewall proof rejects imports of displaced local topology explainer and
  closeout helper lanes.

**Engineering decisions**
- Topology cutover must happen through real parallel folders and named new
  imports, not by gradually reshaping the old modules in place.
- The default displacement map is:
  - `worth-topo/src/projection/query_backed_consumer_cutover/*` ordinary route
    explanation ownership -> `worth-topo/src/projection/planner_owned_routing/query_backed_read_family/`
  - `worth-topo/src/projection/diagnostic_surfaces/*` ordinary route authority
    -> `worth-topo/src/projection/planner_owned_routing/diagnostic_projection_input/`

**Open questions**
- None.

### Phase 19: Spatial Explainer Cutover And Hard Deletion

Cut spatial callers off displaced local explainers and closeout-local route
logic, then delete or cap the old spatial lanes.

**Relevant subsystems**
- `worth-spatial` evidence lookup public closeout
- `worth-kernel` planner-owned route consumers

**Relevant APIs**
- current spatial closeout and diagnostic surfaces
- planner-owned route input products
- residue-manifest surfaces

**Warnings**
- Do not leave the old spatial files alive as ordinary helper backdoors after
  cutover.
- Do not let current-source assembly linger as the normal authoring home for
  route meaning once the new lane exists.
- Do not cap broad mixed spatial lanes as residue when they should be split and
  deleted.

**Test requirements**
- `spatial_callers_import_new_parallel_route_lanes`: ordinary spatial callers
  resolve through the new planner-owned route lanes, not displaced legacy
  modules.
- `deleted_spatial_explainers_cannot_be_reintroduced`: compile-fail or source
  firewall proof rejects imports of displaced local spatial explainer and
  closeout helper lanes.

**Engineering decisions**
- Spatial cutover must happen through real parallel folders and named new
  imports, not by gradually reshaping the old modules in place.
- Residue is allowed only for non-ordinary, certification-only, or Query-gap
  surfaces with owner, blocker, and removal trigger.
- The default displacement map is:
  - `worth-spatial/src/workload_platform/evidence_lookup_public_closeout/*`
    ordinary closeout explanation ownership ->
    `worth-spatial/src/workload_platform/planner_owned_routing/public_closeout_route/`
- This phase is not complete if spatial ordinary callers still treat the
  displaced folders as the normal authoring home for route meaning, even if
  some helper types have been copied into the new lane.

**Open questions**
- None.

### Phase 20: Source Firewalls, Constructor Denials, And Compile-Fail Fences

Seal the planner-owned route, proof, and diagnostics boundaries so local
explanation ceremony cannot come back.

**Relevant subsystems**
- `worth-kernel` source firewall
- public facade boundaries
- certification and compile-fail suites

**Relevant APIs**
- source-firewall report surfaces
- forbidden-surface registry
- public proof constructors
- public diagnostic constructors

**Warnings**
- Do not rely on comments or QA memory to preserve the new architecture.
- Do not leave raw constructors visible "just for tests."
- Do not firewall only imports while leaving local fabrication APIs public.

**Test requirements**
- `planner_public_proof_firewall_rejects_raw_constructor_fabrication`: callers
  cannot fabricate planner route, public proof, or diagnostic authority from
  raw collections, strings, or copied digests.
- `local_explainer_shortcuts_are_compile_fail`: compile-fail fixtures reject
  imports of forbidden helper lanes, support wrappers, and route-local
  construction seams.

**Engineering decisions**
- Source firewalls must guard:
  - raw planner route construction
  - local public-proof fabrication
  - local diagnostic fabrication
  - displaced support-ceremony shortcuts
  - legacy local explainer imports
- Constructor visibility must force all covered callers through planner-owned
  admission or derived public facade lanes.
- Firewalls must be installed only after the new lane exists and callers have a
  real replacement import path. A firewall without a real replacement lane is
  just a blocker, not a cutover.

**Open questions**
- None.

### Phase 21: Residue Ledger, Query-Gap Ledger, And Honest Public Debt

Any public or diagnostic path not fully migrated must be explicit residue,
explicit Query-gap, or deleted.

**Relevant subsystems**
- `worth-kernel` residue ledgers
- `worth-topo` and `worth-spatial` displaced helper lanes
- `forge-query` support and admission boundaries

**Relevant APIs**
- residue-chain products
- public-closeout consumer residue manifests
- Query support posture and admission surfaces

**Warnings**
- Do not call something "future cleanup" without owner, blocker, count, and
  removal trigger.
- Do not hide ordinary-path debt inside certification-only language.
- Do not claim Query support where `forge-query` has not actually admitted it.

**Test requirements**
- `public_and_diagnostic_residue_is_exact`: every remaining unmigrated public
  or diagnostic surface is counted exactly with owner, blocker, and removal
  trigger.
- `query_gap_rows_are_distinct_from_local_debt`: Query-backed support gaps are
  not mixed with Worth-local architecture residue.

**Engineering decisions**
- Residue rows must distinguish:
  - deleted
  - non-ordinary capped residue
  - Query-gap
  - blocked by upstream milestone
- The ledger must remain mechanically derivable from live surfaces, not
  maintained as hand-edited prose.

**Open questions**
- None.

### Phase 22: Milestone 15 Closeout And Milestone 16 Handoff

Close the milestone only when public proof and diagnostics are visibly derived
from the same planner-owned route authority execution consumed, and emit the
handoff products Milestone 16 needs for cross-family parity.

**Relevant subsystems**
- `worth-kernel` closeout and public proof
- `worth-topo` route inputs and diagnostics
- `worth-spatial` route inputs and diagnostics
- `forge-query` support and envelope boundaries

**Relevant APIs**
- planner selected-route packet
- public proof products
- derived diagnostic products
- residue and firewall ledgers
- Milestone 16 parity handoff products

**Warnings**
- Do not close the milestone just because diagnostics read nicely.
- Do not emit a Milestone 16 handoff that requires parity work to rediscover
  route meaning from rows, strings, or helpers.
- Do not let closeout ignore structural debt that still lives on ordinary
  public paths.

**Test requirements**
- `milestone_fifteen_closeout_proves_execution_and_public_explanation_share_route_authority`:
  one representative path proves execution, public proof, and diagnostics all
  consume the same planner-owned selected route chain.
- `milestone_sixteen_handoff_is_sufficient_without_local_rediscovery`: parity
  seed products carry enough route identity, family identity, witness identity,
  residue posture, and Query posture for Milestone 16 without rescanning
  implementation internals.

**Engineering decisions**
- Closeout must emit a Milestone 16 handoff carrying:
  - selected route identity
  - selected family and product identity
  - denial or advisory witness identity
  - Query posture or support witness identity where applicable
  - residue digest
  - source-firewall digest
  - public-proof and derived-diagnostic contract identity
- Final closeout must also prove the migration execution law was honored:
  - new lanes were created beside old ones
  - ordinary callers now import the new lanes
  - displaced lanes were deleted or capped
  - no ordinary path still depends on local route rediscovery
- Milestone 16 is allowed to compare, aggregate, and certify these handoff
  products across family kinds, but it is forbidden to rediscover:
  - selected route identity
  - Query support admission meaning
  - reuse basis selection meaning
  - denial witness meaning
  - public-proof input construction from local helper rows

**Open questions**
- None.

## Must Ship

- A complete inventory of current public proof, public closeout, diagnostic,
  explainer, and route-localization surfaces with explicit migrate, delete,
  cap, or Query-gap posture.
- One shared planner-owned explanation vocabulary in `worth-schema` for route
  identity, family identity, product identity, decision-trace identity,
  denial or advisory witness identity, mismatch locus vocabulary, and
  public-proof identity.
- Planner-selected route packets that carry Milestone 12 through 14 prior-proof
  identities plus Query posture and residue/firewall digests where applicable.
- Parallel route-input lanes for:
  - query-backed read routing
  - invalidation route authority
  - derived-read diagnostic input
  - evidence lookup route authority
  - spatial public-closeout route authority
  - replay and undo route meaning
  - transaction route meaning
  - conflict and independence route meaning
  - batch-admission route meaning
  - compiled-product reuse route meaning
- A dedicated planner-owned public proof assembly lane.
- A dedicated planner-owned derived diagnostics lane selected by artifact
  policy.
- A narrow public read-only facade for proof and diagnostics.
- Hard cutover of kernel, topology, and spatial ordinary callers onto the new
  planner-owned lanes.
- Source firewalls, constructor denials, compile-fail fences, and exact residue
  ledgers.
- A Milestone 16 handoff product proving parity work can start without local
  route rediscovery.

## Must Preserve

- Planner-owned routing products remain authoritative for route meaning.
  Public proof and diagnostics remain derived projections.
- Milestones 12 through 14 prior-proof packets remain prior-proof inputs. They
  must not be reclassified or widened by Milestone 15.
- `forge-query` remains the owner of Query support, public runtime admission,
  boundary envelopes, and lower-runtime truth. Worth consumes those artifacts;
  it does not reproduce them.
- Artifact policy remains the selector for rich diagnostics. Operational truth
  must not depend on diagnostics materialization.
- Public facade boundaries remain read-only and non-constructive.
- The migration execution law remains mandatory: new lane first, caller cutover
  second, deletion or capped residue third.

## Acceptance Evidence

- Tests prove inventory completeness and reject unnamed public proof or
  diagnostic surfaces.
- Tests prove planner-owned explanation identity cannot be fabricated from
  strings, rows, or copied digests.
- Tests prove query-backed, invalidation, evidence, replay, conflict, batch,
  transaction, and reuse explanation all lower to planner-consumable route
  products.
- Tests prove public proof consumes route packets and fails on foreign or stale
  witness identity.
- Tests prove diagnostics localize exact touched facts, aspects, selected
  families, selected products, denial posture, and Query posture gaps.
- Tests prove public callers can inspect but cannot construct planner authority
  or proof products.
- Tests prove displaced local explainers are deleted, compile-fail, or capped
  as non-ordinary residue.
- Tests prove Milestone 16 can start from the emitted handoff without route
  rediscovery.
- Directory or topology audits prove the resulting structure matches the named
  planner-owned routing lifecycle instead of hiding it inside closeout files.

## Sequencing Notes

- Milestone 15 belongs immediately after Milestone 14 because public proof and
  diagnostics need stable compiled-product and reuse identity before they can
  explain why a route reused, rebuilt, denied, or escalated.
- It belongs before Milestone 16 because parity proof across families is only
  honest once public explanation and diagnostics consume the same planner-owned
  route authority execution already consumed.
- It should not absorb Milestone 16's cross-family parity matrix or final
  residue collapse beyond the handoff products and exact Milestone 15 residue
  ledger.

## Required Self-Check

- Does the milestone solve a real structural problem or just package work
  cosmetically? Yes. It eliminates the remaining split between planner-owned
  execution authority and local public or diagnostic explanation folklore.
- Is the adversarial constraint precise and load-bearing? Yes. It targets the
  exact failure mode where public proof or diagnostics reopen route meaning
  from local rows, strings, helpers, or support wrappers under mixed-family
  pressure.
- Does the roadmap justify this milestone now? Yes. Milestone 14 emits stable
  compiled-product and reuse identity; Milestone 15 must make public proof and
  diagnostics consume that authority before parity closeout.
- Does the spec preserve crate authority boundaries? Yes. `worth-schema` owns
  shared explanation contracts, `worth-topo` and `worth-spatial` own family
  route inputs, `worth-kernel` owns planner selection and public derived
  products, and `forge-query` remains the owner of Query boundary truth.
- Are the phases carrying most of the real design information? Yes. The phase
  plan holds the concrete architecture, cutover, and proof work.
- Is each phase centered on one conceptual detail or boundary? Yes.
- Does each phase contain at least 2 adversarial tests by default? Yes.
- Could a competent engineer map this spec into honest types, modules, and
  tests? Yes. The spec names the route products, displaced lanes, firewalls,
  facades, residue posture, and handoff contracts explicitly.
- Does the milestone belong in this roadmap sequence, or is it out of order?
  It belongs here because planner-owned public proof must close before
  cross-family parity and residue collapse can be honest.
