# Milestone 11: Evidence Lookup And Boolean Stage Indexing

## Goal

Freeze spatial and boolean evidence lookup as declare-once registered lookup
families over sealed spatial touch authority, related topology touched graph
identity, stage and receipt identity, and Query-native support posture.

Boolean and spatial stages may consume evidence lookup products. They may not
scan raw evidence vectors, walk broad receipt ledgers, call nearby-evidence
helpers, hand-author lookup lists, or confuse Query descriptors with spatial
lookup authority.

## Why This Milestone Exists

Milestone 4 made spatial evidence touch authority real. Milestones 8 through 10
made Query graph-read receipts, legality receipts, and topology-derived product
receipts available as proof inputs. Milestone 11 is the next scaling boundary:
evidence lookup must stop being stage-local folklore and become a registered
product family that every matching boolean or spatial stage can consume without
rebuilding local lookup machinery.

This milestone belongs before replay, undo, conflict, cache, diagnostics, and
public proof because those later milestones must consume lookup receipts. They
must not rediscover evidence scope by rescanning spatial ledgers, topology
products, boolean stage rows, or Query descriptors.

## Governing Summaries

- `MENTALITY.md`: protects adversarial-constraint-first engineering. This
  milestone solves evidence lookup authority before later replay, conflict,
  cache, and diagnostics depend on evidence scope.
- `arch_laws.md`: protects declare-once, proof-bearing phase chains. Spatial
  touch authority, lookup family declarations, selected lookup plans, execution
  receipts, and diagnostics must be distinct typed products.
- `composition_laws.md`: protects responsibility-named modules. Inventory,
  family declarations, selection, indexing, execution receipts, diagnostics,
  deletion proof, and closeout must not collapse into one evidence helper.
- `domain_structure_laws.md`: protects visible ownership and authority
  separation. `worth-spatial` owns spatial evidence lookup products; Query owns
  Query artifacts, support/admission, projection consumption, and Consumer Kit
  proof; `worth-topo` contributes topology-derived receipt identity but cannot
  become a spatial evidence adapter.
- `perf_laws.md`: protects semantic-delta-bounded execution. Lookup breadth must
  scale with sealed spatial touch authority plus declared lookup family
  expansion, not raw evidence count, receipt ledger size, stage count, or global
  topology size.
- `AI_README.md`: protects Query's core rule: declare intent once, lower it
  once, and consume canonical runtime-owned artifacts. Milestone 11 must use
  Query support/admission, projection-consumption, lower-runtime boundary
  envelopes, and Consumer Kit proof where those are the right Query-owned
  surfaces, while keeping spatial lookup products separate from Query
  descriptors.
- `touched-graph-roadmap.md`: places this milestone after derived invalidation
  and before replay/undo because evidence lookup must consume topology-derived
  receipt identity without rebuilding topology or scanning raw evidence.

## Adversarial Constraint

Worth must survive long boolean, NURBS, extrusion, fillet, and future curved
operation chains where each stage touches a small spatial/topology region while
the workspace contains large evidence ledgers, many boolean stage receipts,
many topology-derived product receipts, retained Query artifacts, replay
history, cache entries, and diagnostics.

If a covered boolean or spatial stage can satisfy evidence lookup by scanning
raw evidence vectors, walking broad receipt ledgers, parsing receipt strings,
matching display labels, using stage-local "find nearby evidence" loops,
reusing topology-derived product receipts as lookup products, using Query
descriptors as evidence authority, or keeping a compatibility adapter that
accepts old evidence rows and emits lookup receipts, the milestone has failed.

## Product Decision Lock

- Build a parallel `worth-spatial` evidence lookup family lane beside existing
  spatial evidence ledgers, boolean stage helpers, receipt lookup helpers, and
  any public evidence-scan surfaces before cutover.
- Use parallel migration plus hard deletion. In-place refactoring is allowed
  only inside the new lane after its authority shape exists.
- `worth-spatial` owns lookup family source declarations, spatial touch keyed
  lookup identity, selected lookup plans, bounded index products, lookup
  execution receipts, diagnostics, deletion ledgers, source firewalls, and
  public closeout proof.
- `forge-query` owns Query support/admission posture, projection consumption
  receipts when lookup consumes Query-materialized facts, lower-runtime boundary
  envelopes for rows classified as lower-runtime boundary-envelope touchpoints,
  Consumer Kit proof for Query consumption, and typed Query artifacts. Worth
  must not mirror those surfaces locally.
- `worth-topo` may supply Milestone 10 topology-derived product receipt identity
  as an input to lookup planning. It may not satisfy spatial lookup authority or
  become a spatial evidence lookup adapter.
- Query descriptors, graph obligations, access-plan receipts, topology-derived
  product receipts, and spatial evidence lookup products are distinct
  authorities. None may substitute for another.
- Every covered lookup must produce either a selected bounded lookup plan and
  receipt, a typed missing-support denial, a typed required-capability posture,
  or capped non-ordinary residue. Raw vectors and broad scans are not postures.
- Deletion is part of the milestone. Raw evidence vector public scans, broad
  receipt scans, stage-local nearby loops, copied digest lookup, compatibility
  wrappers, and public constructors must be deleted, capped, or denied before
  closeout.

## Implicit Requirements Made Explicit

- Covered lookup means every ordinary production boolean or spatial stage that
  consumes spatial evidence, every workload-composition handoff that relies on
  spatial evidence lookup, and every public or certification path that could
  currently satisfy evidence lookup from raw evidence rows, broad receipt scans,
  copied digests, nearby-evidence loops, or public evidence vectors.
- Non-covered lookup must be explicitly named as certification-only,
  documentation/report codec, test fixture support, or non-ordinary residue. It
  cannot be omitted from inventory because it is "not production" unless a
  closeout row proves it cannot satisfy ordinary lookup proof.
- Lookup family source declarations, admitted lookup inputs, selected lookup
  plans, index products, execution receipts, diagnostic projections, deletion
  rows, residue rows, and Milestone 12 seeds are separate proof products. A
  later product may consume an earlier product; it may not reconstruct it from
  strings, rows, display labels, or source scans.
- Index lifecycle is a first-class product boundary. Sparse index use, bounded
  dense index construction, rebuild, reuse, disposal, and required persistent
  capability posture must be selected before execution and measured by counters.
- Density policy belongs in selected lookup plans, not execution. Execution may
  consume a sparse plan, bounded dense plan, denied plan, or required-capability
  posture; it may not decide density by inspecting evidence breadth at runtime.
- Query involvement must be classified by exact surface: support/admission,
  support pinning, projection consumption, lower-runtime boundary envelope,
  typed Query artifact identity, or Consumer Kit downstream proof. A generic
  "Query proof" row is not precise enough to satisfy this milestone.
- The Milestone 12 seed must preserve lookup scope, evidence ledger basis,
  selected plan identity, execution receipt identity, denial posture, index
  lifecycle posture, and Query support posture where present. It must not claim
  replay or undo execution.

## Phase Plan

### Phase 1: Evidence Lookup Folklore Inventory And Cut Line

Freeze every current evidence lookup authority surface before replacement code
is written. Every raw evidence vector access, broad receipt scan, stage-local
nearby lookup, copied digest search, public evidence row exposure, and boolean
stage lookup helper must be classified as migrate, delete, cap,
certification-only, or Query-gap.

**Relevant subsystems**
- `worth-spatial` spatial evidence ledger and boolean stage receipt surfaces
- `worth-spatial` workload platform boolean stage helpers
- `worth-kernel` workload composition surfaces that consume spatial evidence
- public spatial evidence facades and certification closeout surfaces
- Query Consumer Kit residue and hard-prohibition proof where Query artifacts
  are consumed

**Relevant APIs**
- sealed boolean and spatial evidence receipts from Milestone 4
- spatial touch authority products and lookup key identity from Milestone 4
- existing boolean stage receipts and evidence ledgers
- Milestone 10 `DerivedInvalidationMilestoneElevenSeed`
- Query Consumer Kit support snapshots, support pinning, boundary audit, and
  consumer-residue audit surfaces

**Warnings**
- Do not start by improving an old lookup helper in place. Old lookup helpers
  are the authority being displaced.
- Do not classify a broad evidence scan as harmless because current fixtures are
  small. The cost boundary is long operation chains over large evidence ledgers.
- Do not let "test-only" hide ordinary lookup authority. Test support may remain
  only if it cannot satisfy production lookup closeout.
- Do not treat Query descriptors, topology-derived receipts, or rendered
  strings as lookup authority during inventory. They are inputs or adjacent
  products, not evidence lookup products.

**Test requirements**
- `evidence_lookup_inventory_has_no_keep_rows`: every raw vector scan, broad
  receipt scan, nearby-evidence loop, copied digest search, public evidence row,
  and compatibility wrapper has exactly one migrate, delete, cap,
  certification-only, or Query-gap disposition.
- `unclassified_lookup_surface_fails_closeout`: adding a new lookup helper,
  evidence vector reader, receipt ledger scan, or public evidence scan without a
  disposition fails closeout.
- `query_artifact_inventory_rows_do_not_mint_lookup_authority`: Query
  descriptors, support rows, graph-read receipts, and topology-derived receipts
  may be inventoried as inputs but cannot become lookup products.
- `inventory_rows_preserve_source_identity`: identical lookup behavior in two
  source locations yields distinct inventory identity so deletion cannot collapse
  unrelated old paths.

**Engineering decisions**
- The inventory is a production closeout product, not a grep-only test.
- Inventory rows must carry source path, old authority kind, owner, current
  caller, disposition, replacement phase, blocker, removal trigger, and whether
  the old surface is certification-only.
- Query-related rows must name the exact AI_README category they touch:
  Consumer Kit proof, projection consumption, lower-runtime boundary envelope,
  support/admission, or typed artifact identity.
- The old-path inventory is closeout pressure and cannot seed lookup execution.

**Open questions**
- None.

### Phase 2: Parallel Evidence Lookup Family Catalog

Build the new `worth-spatial` lookup family catalog beside the old evidence
lookup code. A lookup family declares the spatial touch authority it consumes,
the related topology touch or derived-product receipt identity it either marks
`NotRequired` or requires as typed input, the stage and receipt identity it
applies to, the evidence classes it can return, its indexing posture, its Query
support posture, and its diagnostic witness shape.

**Relevant subsystems**
- new `worth-spatial` evidence lookup family lane
- Milestone 4 spatial touch authority and lookup key products
- boolean stage receipt families and evidence ledgers
- Milestone 10 topology-derived product receipt references
- `forge-query` support/admission and projection-consumption surfaces

**Relevant APIs**
- sealed spatial touch authority product
- spatial evidence lookup key identity from Milestone 4
- boolean stage receipt identity and stage family identity
- `DerivedInvalidationMilestoneElevenSeed`
- `DerivedInvalidationMilestoneElevenProductReceiptRef`
- Query projection-consumption declarations and typed fact receipts when a
  lookup family consumes Query-materialized facts
- Query support snapshots and support pinning contracts

**Warnings**
- A lookup family is not a callback registry or a function pointer table. It is
  source truth for lookup applicability, authority requirements, index posture,
  support posture, and receipt shape.
- Do not let family identity come from stage names, file names, display labels,
  evidence row strings, or copied receipt digests.
- Do not combine topology-derived product receipts with spatial evidence lookup
  products. Topology receipts may narrow or bind lookup planning, but they do not
  prove spatial evidence existence.
- Do not invent local Query support enums. Use Query support/admission and
  Consumer Kit proof where Query posture is involved.

**Test requirements**
- `lookup_family_declared_once_routes_multiple_matching_stages`: one lookup
  family declaration applies to at least two matching boolean or spatial stages
  without editing those stages.
- `lookup_family_requires_authority_and_posture`: a family without spatial touch
  authority applicability, stage/receipt identity, evidence class, lookup
  product posture, index posture, or diagnostic posture cannot enter the
  catalog.
- `raw_labels_and_digests_cannot_mint_lookup_family_identity`: raw strings,
  copied stage labels, display names, Query descriptor labels, and receipt
  digest strings cannot construct family identity.
- `query_support_posture_is_imported_not_mirrored`: family rows that depend on
  Query support must bind Query support snapshot or support pin evidence instead
  of local support lists.

**Engineering decisions**
- Family records must separate source authority from selected lookup products.
  Source records are stable declarations; selected plans are derived per touch
  and stage receipt.
- Family records must expose read-only proof surfaces and keep constructors
  sealed to registration.
- Family records must distinguish spatial touch authority, topology-derived
  receipt identity, Query descriptor identity, stage receipt identity, and lookup
  product identity as separate typed fields.
- Required Query surfaces must be named by category and evidence: projection
  consumption receipt, support pin, boundary envelope, or Consumer Kit audit.

**Open questions**
- None.

### Phase 3: Lookup Input Admission And Product Separation

Freeze the admission boundary for lookup planning. Lookup may start only from
sealed spatial touch authority, admitted stage/receipt identity, topology input
state recorded as either `NotRequired` or typed Milestone 10 product receipt
references, and explicit Query support/projection artifacts for rows classified
as Query touchpoints. Raw evidence rows, topology rows, Query descriptors, and
copied digests cannot enter.

**Relevant subsystems**
- `worth-spatial` lookup family lane
- spatial touch authority admission products
- boolean stage receipt products
- `worth-topo` Milestone 10 seed facade
- `forge-query` projection consumption and support/admission surfaces

**Relevant APIs**
- spatial touch authority digest and lookup key
- boolean or spatial stage receipt digest and family identity
- `DerivedInvalidationMilestoneElevenSeed::topology_derived_product_receipts`
- `DerivedInvalidationMilestoneElevenLookupReadiness`
- Query projection-consumption typed fact receipts
- Query lower-runtime boundary envelope sources when lookup needs lower-runtime
  posture

**Warnings**
- Do not allow raw evidence vectors to seed lookup planning.
- Do not allow Milestone 10 topology-derived receipt refs to stand in for
  spatial touch authority.
- Do not allow Query graph touch descriptors, graph-read receipts, support rows,
  or boundary envelopes to stand in for evidence lookup products.
- Do not parse digest strings to recover stage order, evidence identity,
  topology identity, or Query support posture.

**Test requirements**
- `lookup_input_requires_sealed_spatial_touch_authority`: raw evidence rows,
  copied receipt fields, topology-derived receipt refs, and Query descriptors
  cannot seed lookup without sealed spatial touch authority.
- `topology_receipt_narrows_but_cannot_satisfy_lookup`: a Milestone 10 product
  receipt may bind related topology identity but cannot produce a lookup product
  or evidence existence proof.
- `query_descriptor_and_lookup_product_are_not_interchangeable`: compile-fail
  and runtime denial tests prove Query descriptors cannot satisfy lookup product
  APIs and lookup products cannot satisfy Query descriptor APIs.
- `wrong_stage_receipt_identity_denies_before_selection`: a valid spatial touch
  authority paired with the wrong stage or receipt identity denies before family
  selection or evidence scanning.

**Engineering decisions**
- Admission produces a phase-typed `admitted lookup input` product consumed by
  plan selection.
- Admission products must carry spatial touch digest, stage receipt digest,
  topology-derived receipt state and digest summary when present, Query support
  digest for Query-classified rows, and exact denial posture.
- Product separation is a first-class proof boundary. The spec intentionally
  rejects structural type reuse when authority differs.
- Missing Query posture is a typed required-support or Query-gap row, not
  permission to scan locally.

**Open questions**
- None.

### Phase 4: Touched Authority To Selected Lookup Plan Lowering

Lower admitted lookup input plus the lookup family catalog into a selected
lookup plan before any evidence lookup executes. The selected plan must say
which families matched, which were unaffected, which require Query support,
which deny, which can use indexed lookup, and which remain capped residue.

**Relevant subsystems**
- `worth-spatial` lookup family catalog
- admitted lookup input products
- spatial evidence ledger index planning
- boolean stage receipt families
- Query support/admission and projection-consumption posture

**Relevant APIs**
- lookup family catalog digest
- admitted lookup input digest
- spatial touch authority digest and lookup key
- stage receipt identity
- topology-derived product receipt refs from Milestone 10
- Query support snapshot and support pin digest

**Warnings**
- Do not select lookup families by scanning all evidence rows, all boolean
  stages, all receipt strings, or all topology products.
- Do not hide index construction or density decisions inside execution. The
  selected plan must carry the selected lookup strategy and support posture.
- Do not scalarize grouped lookup requests into caller-owned loops when the
  touch authority and stage family are batch-compatible.
- Do not let execution rediscover family applicability, stage posture, Query
  posture, or artifact policy.

**Test requirements**
- `same_authority_and_catalog_produce_same_lookup_plan_digest`: identical
  admitted input, family catalog, Query support posture, and topology receipt
  references produce stable selected lookup plan identity independent of stage
  display name.
- `unrelated_lookup_families_remain_unselected`: families whose spatial touch
  applicability, topology identity, stage identity, or evidence class does not
  intersect the admitted input report zero execution work.
- `missing_query_projection_consumption_denies_before_lookup`: a family that
  requires Query projection facts denies before index construction when the
  typed consumed-fact receipt is absent.
- `selected_lookup_plan_breadth_matches_touch_and_family_expansion`: selected
  candidate counts are bounded by spatial touch authority plus declared family
  expansion, not evidence ledger size.

**Engineering decisions**
- The selected plan is the only execution input for covered lookup.
- Plan rows must carry family identity, spatial touch digest, stage receipt
  digest, topology-derived receipt state, Query support posture,
  strategy posture, matched evidence class, and denial reason if any.
- Counters must at least include candidate families, matched families,
  unaffected families, denied families, required Query posture rows, selected
  spatial regions, selected stage receipts, topology receipt refs consumed, and
  caller-owned evidence work count.
- Grouped lookup selection is a plan-level responsibility. Stages supply
  authority; they do not loop over family rows.

**Open questions**
- None.

### Phase 5: Bounded Evidence Index Product Contract

Freeze index products as derived lookup products selected by the lookup plan,
not as hidden execution conveniences. This phase defines sparse index use,
bounded dense index construction, index reuse, disposal, required persistent
capability posture, and index counters before lookup execution can consume any
evidence.

**Relevant subsystems**
- `worth-spatial` selected lookup plan lane
- spatial evidence ledger storage
- new lookup index product lane
- boolean stage receipt products
- Query projection-consumption typed fact receipts for selected families that
  consume Query-materialized facts
- Query support/admission for required persistent or materialized capability

**Relevant APIs**
- selected lookup plan rows
- spatial touch authority lookup key
- stage receipt identity
- evidence class and evidence ledger basis identity
- lookup index product identity
- Query projection-consumption receipts
- Query support/admission posture evidence

**Warnings**
- Do not construct a hidden all-evidence index to make tests pass. Index scope,
  resident bytes, row count, disposal posture, and reuse basis must be explicit.
- Do not let execution decide sparse versus dense strategy by inspecting ledger
  breadth. Density posture must already be selected by the plan.
- Do not treat an index product as persistent or restart-stable unless Query or
  the owning spatial support posture admits that capability.
- Do not allow a cached index to prove evidence lookup without a selected plan
  and matching authority basis.

**Test requirements**
- `index_product_identity_depends_on_selected_plan_and_basis`: the same
  selected plan, spatial touch authority, stage receipt identity, evidence
  ledger basis, and Query support posture produce stable index product identity.
- `hidden_all_evidence_index_fails_index_contract`: constructing an all-ledger
  index for a local touched lookup fails because selected scope, row count, and
  resident-byte counters exceed the selected plan.
- `index_reuse_requires_equivalence_basis`: a reused index must bind selected
  plan digest, spatial touch digest, evidence ledger basis digest, stage receipt
  identity, and index lifecycle posture; pointer identity or cache key strings
  cannot justify reuse.
- `persistent_index_claim_requires_admitted_support`: persistent or restart-
  stable index posture denies unless the required Query/spatial support posture
  is present before execution.

**Engineering decisions**
- Index products are derived execution products. They may be destroyed and
  rebuilt from spatial evidence authority and selected plan proof.
- Index product records must carry selected plan digest, evidence ledger basis
  digest, spatial touch digest, stage receipt identity, topology-derived receipt
  state, Query support digest for Query-classified rows, lifecycle posture, row
  count, resident bytes, and disposal/reuse proof.
- Index product construction must be sealed so public callers and tests cannot
  fabricate "bounded index" or "persistent index" claims.
- Milestone 14 may later add cache/equivalence proof for lookup reuse, but
  Milestone 11 must already name the exact index equivalence basis it used.

**Open questions**
- None.

### Phase 6: Lookup Execution Receipts And Product Outputs

Execute lookup only from selected lookup plans and admitted index products, then
produce receipt-grade proof and lookup product output. The execution lane must
distinguish indexed hit, indexed miss, bounded rebuild, required Query support,
denied-before-execution, and capped residue outcomes without selecting strategy
or rebuilding index posture.

**Relevant subsystems**
- `worth-spatial` selected lookup plan execution lane
- lookup index product lane
- spatial evidence ledger storage
- boolean stage receipt products
- Query projection-consumption typed fact receipts for executions that consume
  Query-materialized facts
- lookup diagnostics and counter reports

**Relevant APIs**
- selected lookup plan rows
- lookup index product records
- spatial touch authority lookup key
- stage receipt identity
- evidence class and evidence receipt identity
- Query projection-consumption receipts
- Query support/admission posture evidence

**Warnings**
- Execution may consume old evidence ledger mechanics while migrating, but the
  receipt must expose whether work was indexed, bounded, denied, or capped
  residue. Mechanics are not authority.
- Execution may not select index lifecycle, density posture, Query support
  posture, artifact policy, or fallback posture. Those are planning/index facts.
- Do not materialize rich diagnostics on the hot path unless artifact policy
  demands it. Operational receipt proof and diagnostic projection are separate.
- Do not downgrade missing Query support, missing stage authority, or missing
  index product authority into broad evidence scans.

**Test requirements**
- `lookup_execution_receipt_is_deterministic`: the same selected plan, index
  product, spatial touch authority, stage receipt identity, Query support
  posture, and evidence ledger basis produce stable receipt and diagnostic row
  digests.
- `lookup_execution_counters_follow_selected_plan_and_index`: selected regions,
  evidence candidates, ledger rows touched, index rows consumed, resident bytes,
  hit count, miss count, and caller-owned scan count match the selected plan and
  index product.
- `broad_evidence_scan_fails_even_when_result_matches`: a broad raw-vector scan
  that returns the same evidence as the selected lookup plan fails execution
  proof because counters expose unauthorized breadth.
- `missing_query_consumed_fact_receipt_does_not_execute_lookup`: lookup families
  requiring Query typed facts deny before evidence access when
  projection-consumption proof is absent.

**Engineering decisions**
- The lookup execution receipt is the canonical Milestone 11 proof product
  consumed by replay, undo, conflict, cache, public proof, and diagnostics.
- Receipt fields must include selected plan digest, index product digest,
  spatial touch digest, stage receipt digest, evidence ledger basis digest,
  topology-derived receipt state and digest summary when present, Query support
  digest, index posture, lookup product output digest, and structural counter
  digest.
- Lookup product output is derived from execution receipt authority and cannot
  be reconstructed from raw evidence vectors by later stages.
- Receipt construction must be sealed so public callers and tests cannot
  fabricate "bounded lookup" or "zero caller-owned scan" claims.

**Open questions**
- None.

### Phase 7: First Boolean Stage Lookup Migration Slice

Migrate one real boolean or spatial stage from old local evidence lookup into
the new catalog-routed lookup lane. The slice must prove the complete ladder:
family declaration, admitted input, selected lookup plan, bounded execution
receipt, lookup product output, diagnostic witness, and deletion or cap of the
old lookup helper.

**Relevant subsystems**
- `worth-spatial` boolean stage receipt families
- one production boolean stage that currently consumes spatial evidence
- spatial evidence ledger and old nearby-evidence helper paths
- new evidence lookup family lane
- Query Consumer Kit proof if the stage consumes Query artifacts

**Relevant APIs**
- selected lookup plan products from Phase 4
- lookup execution receipt products from Phase 5
- existing boolean stage receipt identity
- sealed spatial touch authority product
- old stage-local evidence lookup helper or broad receipt scan

**Warnings**
- Do not choose a display-only or toy stage that does not prove spatial touch
  authority, stage receipt identity, evidence result identity, and denial
  semantics.
- Do not wrap the old stage helper and call the wrapper migrated. The stage must
  consume lookup products and receipts from the new lane.
- Do not leave old and new lookup authorities alive as equal success paths after
  parity is proven.
- Do not pick a tiny complete fixture as the only proof. Include a scale-pressure
  case where broad evidence scanning would be visible.

**Test requirements**
- `first_stage_lookup_slice_preserves_semantics_with_new_receipt`: the migrated
  stage receives the same evidence meaning as the old path for a covered hostile
  scenario while producing selected plan, execution receipt, and lookup product
  identity.
- `first_stage_lookup_slice_rejects_old_helper_after_cutover`: the old
  stage-local nearby-evidence loop, raw vector reader, or receipt scan cannot
  satisfy closeout after the migrated slice cuts over.
- `first_stage_lookup_slice_denies_wrong_touch_or_stage_receipt`: a valid
  evidence ledger with mismatched spatial touch authority or stage receipt
  identity denies before lookup execution.
- `first_stage_lookup_slice_scale_pressure_exposes_broad_scan`: unrelated
  evidence rows outside the touched region do not increase ordinary lookup work.

**Engineering decisions**
- The first migrated slice is the proof template for later lookup families.
- Old helper mechanics may survive only below the new execution receipt if they
  are bounded by selected plan proof and cannot be called directly.
- The migrated slice must delete or cap its old ordinary lookup path in the same
  phase.
- Query-related proof for the slice must use the correct AI_README category:
  projection consumption for materialized facts, support pinning for support
  posture, Consumer Kit for downstream proof, and lower-runtime boundary
  envelopes only when crossing lower runtime authority.

**Open questions**
- Select the first production stage during implementation after reading current
  lookup inventory rows and line counts.

### Phase 8: Boolean Stage Lookup Catalog Sweep

After the first slice proves the lane, migrate every covered boolean and
spatial stage lookup into registered lookup families. Stage-local evidence
lists, nearby-evidence loops, broad receipt scans, and raw vector access must
exit ordinary execution.

**Relevant subsystems**
- all covered `worth-spatial` boolean stage receipt families
- spatial evidence ledger and lookup helpers
- `worth-kernel` workload composition surfaces consuming stage evidence
- new evidence lookup family catalog
- public and certification closeout surfaces

**Relevant APIs**
- every lookup family declaration introduced in Phase 2
- selected lookup plan rows
- lookup execution receipts
- boolean stage receipt identity products
- spatial touch authority products
- Query projection-consumption and support pinning artifacts for lookup families
  classified as Query touchpoints

**Warnings**
- Do not leave a covered stage as "later lookup gap." A covered stage is
  migrated, deleted because it no longer performs lookup, or classified as
  certification-only/non-ordinary residue.
- Do not migrate by wrapping old lookup APIs in new names. Each stage family
  needs family declaration, admitted input, selected plan, execution receipt,
  lookup product output, and deletion/cutover proof.
- Do not allow stage code to externalize loops over lookup families or evidence
  classes. Product fanout belongs to the catalog and selected plan.
- Do not allow Query support posture to be inferred from public method
  visibility or local support strings.

**Test requirements**
- `all_covered_stage_lookups_have_registered_families`: every covered stage
  lookup has a lookup family declaration, selected plan row, execution receipt
  path, and old-authority deletion/cap row.
- `stage_local_evidence_lists_fail_after_sweep`: any covered stage still
  maintained by local evidence lists, nearby loops, broad scans, or copied
  digest lookups fails closeout.
- `cross_stage_declare_once_lookup_routes_without_stage_edits`: adding or
  editing one lookup family changes routing for every matching stage/touch
  authority without stage-local wiring.
- `sweep_counters_scale_with_touch_not_stage_count`: scale-pressure cases prove
  lookup breadth follows spatial touch authority and selected families, not the
  total number of stages or evidence ledger rows.

**Engineering decisions**
- The sweep should proceed stage family by stage family, but each family must
  close vertically before the next starts.
- Stage families that share lifecycle may share phase abstractions, but their
  authority requirements, evidence classes, diagnostics, counters, and Query
  posture remain separate declarations.
- The sweep must leave the new lookup lane as ordinary lookup authority. Old
  lookup code can remain only as mechanics beneath receipt-backed execution or
  as certification-only residue.
- Full sweep proof is required before final closeout; a generic required-family
  bridge cannot stand in for family-specific migrated lookup receipts.

**Open questions**
- None.

### Phase 9: Query Surface Contract Matrix

Freeze a Query surface contract matrix for every Query touchpoint in the lookup
lane. Each row must classify the touchpoint as support/admission, support
pinning, projection consumption, lower-runtime boundary envelope, typed artifact
identity, or not a Query surface. This phase prevents generic "Query proof"
language from becoming a local pseudo-Query lane.

**Relevant subsystems**
- Query support/admission and support pinning
- Query projection consumption
- Query lower-runtime capability routing and boundary envelopes
- `worth-spatial` lookup family and closeout products
- `worth-topo` Milestone 10 seed consumption

**Relevant APIs**
- Query support snapshots and `support_pinning_contract(...)`
- Query projection-consumption declarations and typed fact receipts
- `ForgeQueryLowerRuntimeBoundaryEnvelopeSource` values for matrix rows
  classified as lower-runtime boundary-envelope touchpoints
- `DerivedInvalidationMilestoneElevenSeed` and product receipt refs
- lookup selected plan and execution receipt products

**Warnings**
- Do not use "Query proof" as a catch-all label. The exact AI_README surface
  category must be named for every Query dependency.
- Do not use Query graph touch descriptors as spatial evidence lookup products.
- Do not read Query materialization rows, bridge helper state, retained rows, or
  live rows directly when projection consumption owns the public fact lane.
- Do not construct lower-runtime boundary envelopes from strings.

**Test requirements**
- `every_query_touchpoint_has_exact_surface_category`: every Query dependency in
  lookup family declarations, selected plans, index products, execution
  receipts, diagnostics, and closeout proof is classified by exact AI_README
  surface category.
- `projection_consuming_lookup_uses_typed_fact_receipts`: lookup families that
  need Query-materialized facts carry projection-consumption proof and fail if
  they read materialization rows or bridge helper state directly.
- `lower_runtime_boundary_envelopes_are_not_synthesized`: any lookup path that
  needs lower-runtime posture consumes real boundary-envelope sources and rejects
  string-built envelopes.
- `query_descriptor_product_swap_fails`: Query descriptors cannot satisfy
  lookup product APIs, and lookup products cannot satisfy Query descriptor or
  graph-obligation APIs.

**Engineering decisions**
- Query support posture belongs in Query-owned support artifacts, not in
  `worth-spatial` local enums.
- Query projection facts are consumed through projection consumption; inspection
  or direct retained-row access is not a substitute.
- Lower-runtime boundary envelopes may appear only as real Query boundary
  receipts or other `ForgeQueryLowerRuntimeBoundaryEnvelopeSource` values.
- Query descriptors remain descriptors; they do not become spatial lookup
  products.
- The public closeout must expose Query surface category, support posture, and
  proof digests, not Query internal constructors or local mirrors.

**Open questions**
- None.

### Phase 10: Consumer Kit Adoption And Residue Proof

Certify downstream Query consumption and Query-related residue through Consumer
Kit surfaces instead of Worth-local report structs, source greps, required-row
lists, or fabricated receipts. This phase proves the Query surface matrix from
Phase 9 is enforced by Query-owned downstream proof.

**Relevant subsystems**
- `forge-query` Consumer Kit
- Query hard-prohibition registry and boundary audit
- Query support snapshots and support pinning
- Query consumer-residue audit
- `worth-spatial` lookup family and closeout products
- public facade contract fixtures

**Relevant APIs**
- `forge_query::facade::consumer_kit`
- `EvidenceReportDeclaration`
- `hard_prohibition_registry()`
- `hard_prohibition_boundary_audit()`
- `project_workspace_support_snapshot(...)`
- `support_pinning_contract(...)`
- `query_consumer_residue_audit(...)`
- lookup closeout Query surface matrix rows

**Warnings**
- Do not hand-roll Query proof with local report structs, source greps,
  required-family row lists, copied support rows, debug strings, or fabricated
  receipts.
- Do not describe Consumer Kit proof as optional convenience. It is the
  downstream proof lane for Query consumption and residue.
- Do not let a clean local source scan stand in for Query hard-prohibition or
  consumer-residue audit.
- Do not hide Query residue as spatial lookup residue. Query residue and spatial
  lookup residue are separate closeout rows.

**Test requirements**
- `lookup_query_consumption_uses_consumer_kit_not_local_reports`: Query-related
  lookup proof uses Consumer Kit evidence, support pins, boundary audit, and
  residue audit instead of local report structs or source greps.
- `query_residue_and_spatial_lookup_residue_are_separate`: Query-proof residue
  rows cannot be counted as spatial lookup residue, and spatial lookup residue
  cannot satisfy Query Consumer Kit proof.
- `support_pinning_uses_live_query_rows`: support pinning binds typed support
  row identity and digest from Query support snapshots, not a checked-in list of
  row names.
- `consumer_residue_audit_blocks_local_query_folklore`: local Query support
  lists, fabricated Query receipts, debug-derived proof strings, and delimiter
  proof strings fail Consumer Kit residue audit.

**Engineering decisions**
- Consumer Kit proof is the ordinary downstream proof lane for Query adoption
  and residue in this milestone.
- Query residue rows must carry Query finding identity, audited source paths,
  report identity, and source-inventory digest.
- Spatial lookup closeout consumes Consumer Kit proof digests but does not own
  Query detector implementation.
- Public closeout must distinguish Query hard-prohibition, support pinning,
  boundary audit, and consumer-residue proof.

**Open questions**
- None.

### Phase 11: Stage Consumption And Workload Cutover

Cut ordinary boolean/spatial stage execution and workload composition over to
lookup products and receipts. Stages may attach spatial touch authority, stage
receipt identity, topology-derived receipt state recorded as `NotRequired` or
typed refs, selected lookup plan identity, and lookup execution receipt identity.
They may not attach evidence lists, raw vectors, receipt-scan results, or
nearby-loop products as ordinary success evidence.

**Relevant subsystems**
- `worth-spatial` boolean stage execution and closeout surfaces
- `worth-kernel` workload composition and public closeout pressure
- lookup selected plan and receipt products
- Milestone 10 seed consumption where topology-derived identity is required
- public spatial evidence facade surfaces

**Relevant APIs**
- lookup execution receipt and lookup product output
- boolean stage receipt identity
- sealed spatial touch authority
- topology-derived receipt state, recorded as `NotRequired` or typed product
  receipt refs
- workload composition handoff products
- public facade contract compile-fail fixtures

**Warnings**
- Do not let stage code expand lookup scope because it has access to the
  evidence ledger. The selected lookup plan is the expansion authority.
- Do not permit workload composition to treat "evidence fallback accepted" as
  equivalent to Milestone 11 lookup proof.
- Do not create an adapter that accepts old evidence rows or lookup lists and
  emits new lookup receipts.
- Do not make lookup receipt attachment optional for covered stages. Optional
  attachment preserves the old "remember to look up evidence" architecture.

**Test requirements**
- `covered_stage_closeout_requires_lookup_receipt`: representative covered
  stages succeed only when they carry spatial touch authority, stage receipt
  identity, selected lookup plan, and lookup execution receipt.
- `workload_composition_rejects_raw_evidence_lookup_results`: workload handoff
  cannot consume raw evidence vectors, broad scan results, or old nearby-helper
  outputs as ordinary evidence lookup proof.
- `stage_consumption_does_not_expand_lookup_scope`: stage code that touches
  evidence outside the selected lookup plan fails counters or source firewall.
- `milestone_twelve_seed_has_lookup_scope_without_rescan`: cutover products
  carry enough lookup digest and counter information for replay/undo planning
  without rescanning evidence.

**Engineering decisions**
- Covered stage closeout should expose lookup receipt identity and counters, not
  internal lookup constructors or evidence rows.
- Workload composition may consume lookup products but cannot own lookup
  selection.
- Cutover must leave a Milestone 12 replay/undo seed posture that names lookup
  receipts without claiming replay completion.
- Cutover must cover every migrated lookup family from Phase 8.

**Open questions**
- None.

### Phase 12: Lookup Diagnostics And Denial Witnesses

Define diagnostics as derived projections from selected lookup plans and lookup
execution receipts. Diagnostics must localize the exact spatial touch facts,
stage receipt identity, family declaration, topology-derived receipt refs,
Query posture, evidence class, and denial/advisory reason without adding hidden
lookup work.

**Relevant subsystems**
- `worth-spatial` lookup diagnostics lane
- lookup selected plan and execution receipt products
- public evidence lookup proof/status facade
- Query projection-consumption and support posture rows when involved
- later public diagnostics Milestone 15 handoff

**Relevant APIs**
- selected lookup plan diagnostic rows
- lookup execution receipt diagnostic rows
- Query support/admission posture evidence
- Query projection-consumption typed fact receipts
- spatial touch authority and stage receipt identity accessors

**Warnings**
- Diagnostics must not become the execution plan. They explain selected plan and
  receipt proof after routing has already happened.
- Diagnostics may not scan raw evidence ledgers to provide richer explanation.
- Diagnostic rows cannot promote advisory, denied, required-support, or residue
  lookup outcomes into successful lookup products.
- Do not flatten Query support and projection-consumption failures into generic
  spatial lookup errors.

**Test requirements**
- `lookup_diagnostics_are_derived_from_plan_and_receipt`: diagnostic rows are
  stable from selected plan and execution receipt identity and do not change
  when unrelated evidence rows are added.
- `diagnostic_projection_cannot_perform_hidden_lookup`: any diagnostic path that
  touches raw evidence vectors, broad receipt ledgers, or old helper scans fails
  source firewall or counter proof.
- `denial_witness_preserves_query_and_spatial_posture`: Query-required support,
  missing projection fact, wrong spatial touch digest, wrong stage receipt, and
  product-swap denials remain distinct diagnostic rows.
- `advisory_lookup_posture_stays_non_authoritative`: advisory or diagnostic-only
  lookup outcomes cannot satisfy ordinary stage lookup proof.

**Engineering decisions**
- Diagnostics are policy-gated projections from operational proof.
- Diagnostic rows must name spatial touch facts, lookup family identity, stage
  receipt identity, evidence class, selected plan row, execution receipt row,
  Query support posture where present, and exact denial/advisory reason.
- Public diagnostics should be rich enough for Milestone 15 but cannot expose
  constructors or mutable internal topology.
- The operational receipt remains the authority; diagnostics are not proof
  substitutes.

**Open questions**
- None.

### Phase 13: Source Firewalls And Public Constructor Denials

Install source and public-constructor firewalls before final deletion closeout.
This phase blocks old lookup authority from reappearing after migration by
denying raw evidence vector scans, broad receipt scans, copied digest lookup,
stage-local nearby loops, Query/lookup product swaps, and public construction
of lookup proof products.

**Relevant subsystems**
- spatial evidence ledgers and raw vector readers
- boolean stage nearby-evidence helpers
- broad receipt scan helpers
- public evidence row and public scan facades
- lookup family lane source firewall
- public facade compile-fail contract suite

**Relevant APIs**
- old raw evidence vector accessors
- old receipt ledger scan helpers
- old nearby-evidence or evidence-search helper names
- lookup source-firewall report
- public lookup closeout proof and compile-fail fixtures

**Warnings**
- Do not let a clean migration slice count as a firewall. The firewall must run
  against production source surfaces after the catalog sweep.
- Do not make the firewall a narrow symbol grep only. It must ban old authority
  by semantic surface and include exact allowed documentation/report-codec
  exceptions.
- Do not let public facades expose constructors for lookup family records,
  admitted lookup input, selected plans, execution receipts, lookup products,
  deletion rows, or closeout proof.
- Do not let Query/lookup product substitution hide behind generic trait bounds
  or common digest wrappers.

**Test requirements**
- `source_firewall_rejects_lookup_folklore_revival`: forbidden semantic surfaces
  for raw evidence vectors, broad receipt scans, copied digest lookup, stage
  nearby loops, and Query/lookup product swaps cannot reappear on covered paths.
- `public_api_cannot_forge_lookup_products`: compile-fail fixtures reject public
  construction of lookup family records, admitted lookup inputs, selected plans,
  execution receipts, lookup product rows, closeout proof, and Milestone 12 seed
  products.
- `firewall_exceptions_are_named_non_authoritative_codecs`: every allowed
  documentation/report/test-fixture exception is named, counted, and rejected as
  ordinary lookup proof.
- `generic_digest_wrappers_cannot_bridge_query_and_lookup_products`: common
  digest wrappers, display labels, and trait-object erasure cannot satisfy both
  Query product APIs and spatial lookup product APIs.

**Engineering decisions**
- The source firewall should scan production sources, not only tests, while
  allowing explicitly named documentation/report codecs.
- The firewall must ban old authority by semantic surface, not only exact symbol
  spellings.
- Public proof surfaces are read-only and narrower than internal lookup lane
  topology.
- Firewall output is a closeout input, not the deletion ledger itself.

**Open questions**
- None.

### Phase 14: Hard Deletion And Residue Caps

Delete or mechanically cap every old evidence lookup path touched by the
milestone. Old paths may survive only as named certification-only or
non-ordinary residue with owner, exact count, blocker, removal trigger, and a
test proving they cannot satisfy ordinary lookup proof.

**Relevant subsystems**
- spatial evidence ledgers and raw vector readers
- boolean stage nearby-evidence helpers
- broad receipt scan helpers
- public evidence row and public scan facades
- lookup family lane deletion ledger
- source-firewall report from Phase 13

**Relevant APIs**
- old raw evidence vector accessors
- old receipt ledger scan helpers
- old nearby-evidence or evidence-search helper names
- lookup deletion ledger
- public lookup closeout proof

**Warnings**
- Deletion is part of the milestone, not optional cleanup.
- Capped residue must not be vague. Each cap needs exact count, owner, reason,
  blocker, and removal trigger.
- Do not allow a capped residue row to become a second lookup authority lane.
- Do not use source-firewall success as a substitute for deletion or explicit
  residue rows.

**Test requirements**
- `old_lookup_paths_are_deleted_or_denied`: migrated nearby helpers, raw vector
  readers, broad receipt scans, and public evidence scans are gone or denied.
- `residue_caps_are_exact_owned_and_non_authoritative`: each remaining lookup
  residue row is certification-only or non-ordinary, counted, owned, and
  rejected as ordinary lookup proof.
- `deletion_ledger_binds_firewall_report`: deletion closeout consumes the Phase
  13 firewall report digest and still names the concrete deleted or capped
  surfaces.
- `residue_rows_cannot_seed_lookup_planning`: capped residue rows cannot enter
  lookup family selection, admitted lookup input, selected plans, or execution
  receipts.

**Engineering decisions**
- Deletion rows attach to Milestone 11 closeout so later milestones consume
  counts without rereading source.
- Residue rows carry owner, exact count, reason, blocker, removal trigger,
  source identity, and ordinary-proof denial test identity.
- Deletion proof consumes the source firewall report but does not depend on
  source grep as its only evidence.
- Public closeout receives deletion ledger digests and residue digests as
  separate inputs.

**Open questions**
- None.

### Phase 15: Public Closeout And Milestone 12 Seed

Publish the real Milestone 11 closeout only after covered lookup families have
real family declarations, selected plans, lookup execution receipts, Query
surface proof for every row classified as a Query touchpoint, hard deletion or
valid non-ordinary residue, and source-firewall proof. Emit the Milestone 12
replay/undo seed without claiming replay, undo, conflict, cache, public
diagnostics, or final touched-graph closeout.

**Relevant subsystems**
- `worth-spatial` evidence lookup family lane
- all migrated lookup family and stage lanes
- Query Consumer Kit and projection-consumption proof products
- `worth-topo` Milestone 10 seed references
- `worth-kernel` closeout pressure and roadmap proof
- public spatial facade and certification contract surfaces

**Relevant APIs**
- Milestone 11 evidence lookup closeout product
- per-family lookup execution receipts
- selected lookup plan digest
- lookup family catalog digest
- deletion ledger digest
- residue audit digest
- source-firewall report digest
- Query support and Consumer Kit proof digests
- Milestone 12 seed carrying lookup receipt identity and replay-readiness
  posture

**Warnings**
- No generic required-family bridge may stand in for family-specific migrated
  lookup proof at final closeout.
- No covered ordinary lookup may remain as later migration work.
- Public proof remains read-only. Constructors and mutable rows stay sealed.
- Milestone 12 replay/undo is not implemented here; this phase only seeds it
  with lookup receipt identity and bounded lookup scope.

**Test requirements**
- `milestone_eleven_closeout_requires_all_covered_lookup_families`: final
  closeout fails if any covered boolean/spatial stage lookup lacks
  family-specific migrated receipt proof or valid non-ordinary residue denial.
- `closeout_digests_bind_lookup_authority_chain`: closeout digests bind spatial
  touch authority, stage receipt identity, topology-derived refs, family catalog,
  selected plans, execution receipts, Query support posture, deletion ledger,
  residue audit, and source firewall.
- `declare_once_lookup_proof_applies_after_closeout`: adding or modifying one
  lookup family once changes routing for multiple matching stages without stage
  edits.
- `milestone_twelve_seed_carries_lookup_scope_without_evidence_rescan`: the seed
  carries enough lookup product identity, receipt posture, counters, and denial
  posture for replay/undo planning to start without scanning raw evidence or
  rebuilding derived topology.

**Engineering decisions**
- Final closeout consumes per-family proof products, not raw collections, copied
  digests, generic placeholders, or test fixtures.
- The Milestone 12 seed distinguishes spatial evidence lookup receipts,
  topology-derived product receipts, Query artifacts, and boolean stage receipts
  so replay cannot substitute one for another.
- Final counters must include slope-sensitive cases for spatial touch size,
  selected lookup family count, stage count, evidence ledger size, and unrelated
  topology-derived product count.
- Final proof must answer: which evidence lookup families were selected, why, by
  which spatial/topology/stage facts, with what Query posture, and at what
  execution breadth.

**Open questions**
- None.

## Must Ship

- A parallel `worth-spatial` evidence lookup family lane with catalog records,
  admitted lookup inputs, selected lookup plans, bounded index products, lookup
  execution receipts, diagnostics, deletion ledger, residue audit, source
  firewall, public closeout proof, and Milestone 12 seed.
- A complete inventory of old evidence lookup authority: raw evidence vectors,
  broad receipt scans, stage-local nearby loops, copied digest lookup, public
  evidence scans, compatibility wrappers, and Query-looking local proof.
- Lookup family declarations over sealed spatial touch authority, related
  topology-derived receipt state recorded as `NotRequired` or typed receipt
  identity, stage/receipt identity, evidence class, lookup product posture,
  index posture, Query support posture, and diagnostic witness.
- Admission proof that raw evidence rows, topology-derived receipts, Query
  descriptors, copied digests, and display labels cannot seed lookup.
- Selected lookup plans derived from admitted lookup input and catalog
  declarations before evidence work begins.
- Bounded lookup execution receipts with exact counters for selected regions,
  evidence candidates, ledger rows touched, index rows built, resident bytes,
  hit/miss count, required Query posture, and caller-owned scan count.
- At least one real migrated boolean/spatial stage slice proving the full
  product ladder from family declaration to lookup receipt and old-path
  deletion.
- A full covered-stage migration sweep so every covered ordinary boolean/spatial
  evidence lookup is migrated, deleted, or classified as non-ordinary residue.
- Query-surface proof using the correct `AI_README.md` surfaces: Consumer Kit
  for downstream Query proof, support snapshots and support pinning for support
  posture, projection consumption for Query-materialized facts, and real
  lower-runtime boundary-envelope sources for every lookup touchpoint classified
  as lower-runtime boundary-envelope dependent.
- Hard-break tests and source firewalls rejecting raw evidence vectors, broad
  receipt scans, stage-local nearby loops, copied digest lookup, public
  constructor forgery, compatibility adapters, local Query support mirrors, and
  Query/lookup product swaps.
- Public read-only closeout proof and a Milestone 12 replay/undo seed carrying
  lookup receipt identity without claiming replay or undo completion.

## Must Preserve

- Milestone 4 spatial touch authority and spatial evidence receipt authority.
- Milestone 8 Query graph-read access plan and receipt authority where lookup
  consumes covered graph-read proof.
- Milestone 9 validator/invariant enforcement receipt semantics where lookup
  depends on legality posture.
- Milestone 10 topology-derived product receipt identity as related topology
  input, not spatial evidence lookup authority.
- Query ownership of Query support/admission, projection consumption,
  lower-runtime boundary envelopes, typed artifacts, and Consumer Kit proof.
- `worth-spatial` ownership of spatial evidence lookup products, lookup
  execution receipts, and geometry evidence diagnostics.
- The ability to destroy lookup index products and rebuild them from spatial
  evidence authority plus selected lookup plans.
- The distinction between operational lookup receipts and policy-gated
  diagnostics.

## Acceptance Evidence

- Tests prove inventory completeness, no `keep` disposition, unclassified old
  lookup rejection, and source-firewall denial for new raw evidence scans,
  nearby loops, broad receipt scans, copied digest lookup, and compatibility
  wrappers.
- Tests prove lookup family declarations are sealed, declare-once, and apply to
  multiple matching stages without stage-local wiring.
- Tests prove admitted lookup input rejects raw evidence rows, topology receipt
  substitution, Query descriptor substitution, copied digest identity, wrong
  spatial touch digest, and wrong stage receipt identity.
- Tests prove selected lookup plan identity is deterministic from admitted
  input, family catalog, Query support posture, and topology-derived receipt
  refs, and that unrelated families remain unselected.
- Tests prove missing Query support, support pinning, projection consumption,
  lower-runtime boundary envelope, or stage authority denies before index
  construction or evidence scanning.
- Tests prove migrated lookup execution preserves old semantic evidence results
  for covered hostile scenarios while producing new lookup receipts and bounded
  counters.
- Tests prove every covered ordinary lookup is migrated, deleted, or
  non-ordinary residue before closeout.
- Tests prove source firewall output and deletion ledger output are separate:
  firewall success blocks revival, while deletion/residue proof names concrete
  removed or capped authority surfaces.
- Tests prove Query descriptors and evidence lookup products cannot satisfy each
  other.
- Tests prove public callers cannot forge lookup family records, admitted
  inputs, selected plans, execution receipts, lookup product rows, closeout
  proof, or Milestone 12 seed products.
- Tests prove scale-pressure counters grow with spatial touch breadth, selected
  family breadth, and selected evidence class breadth, not global evidence
  ledger size, total stage count, global topology size, or unrelated derived
  product count.

## Sequencing Notes

Milestone 11 starts only after Milestone 10 produces a public
`DerivedInvalidationMilestoneElevenSeed` with topology-derived product receipt
identity and lookup-readiness posture. It may consume those topology-derived
refs as narrowing and replay inputs, but it must not treat them as spatial
evidence lookup products.

Milestone 11 should not implement replay, undo, conflict, cache/equivalence,
public diagnostics, final touched-graph closeout, or resumed Milestone 7.5
boolean work. It should produce lookup receipts and a Milestone 12 seed strong
enough for replay and undo to start without rescanning evidence.

The first implementation plan may use one boolean/spatial stage as a vertical
slice to prove the lane, but final closeout must follow the sweep and delete or
cap every covered old lookup path. The runner should treat in-place refactors,
slow-conversion adapters, raw vector wrappers, broad receipt scans, copied
digest helpers, local Query support rows, and product-substitution bridges as QA
findings.

## Required Self-Check

- Does the milestone solve a real structural problem or just package work
  cosmetically? Yes. It replaces stage-local evidence lookup folklore with
  registered lookup products and bounded receipts.
- Is the adversarial constraint precise and load-bearing? Yes. It rejects raw
  evidence vectors, broad receipt scans, copied digest lookup, nearby loops,
  product substitution, and compatibility adapters under large evidence ledgers.
- Does the roadmap justify this milestone now? Yes. Milestone 10 provides
  topology-derived receipt identity, and Milestone 12 needs lookup receipts
  before replay/undo can avoid evidence rediscovery.
- Does the spec preserve crate authority boundaries? Yes. `worth-spatial` owns
  spatial lookup products, Query owns Query artifacts and proof surfaces, and
  `worth-topo` contributes topology receipt identity without becoming a spatial
  adapter.
- Are the phases carrying most of the real design information? Yes. The design
  lives in the fifteen ordered phases.
- Is each phase centered on one conceptual detail or boundary? Yes: inventory,
  catalog, admission, selection, index contract, execution receipt, first slice,
  sweep, Query matrix, Consumer Kit proof, cutover, diagnostics, source
  firewall, deletion, and closeout.
- Does each phase contain at least 2 adversarial tests by default? Yes.
- Could a competent engineer map this spec into honest types, modules, and
  tests? Yes. The spec names the new lane, old surfaces, proof products,
  counters, denial tests, and Query-surface requirements.
- Does the milestone belong in this roadmap sequence, or is it out of order?
  It belongs after Milestone 10 and before Milestone 12 because replay/undo
  must consume lookup receipts rather than rediscover evidence scope.
