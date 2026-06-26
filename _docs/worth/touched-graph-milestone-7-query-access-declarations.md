# Touched Graph Milestone 7: Query Access Declarations

> **Status:** Draft
>
> **Purpose:** lower the Milestone 6 graph-read access inventory seed into
> touched-authority-backed Query read declarations, registered read families,
> access requirement rows, support posture evidence, and capability gaps, so
> Milestone 8 can adopt admitted access plans without preserving Worth-local
> graph traversal authority.

## Goal

Milestone 7 freezes the boundary where Worth touched graph authority becomes
Query-owned graph-read declaration authority.

The milestone does not execute covered graph reads. It builds a fresh
declaration lane beside the Milestone 6 inventory lane, consumes the
`WorthGraphReadAccessMilestoneSevenSeed`, lowers declaration candidates into a
registered Query read-family catalog, derives Query-owned access requirement
rows, routes missing support into typed capability gaps, deletes or caps local
declaration/access-requirement residue, and emits the Milestone 8 seed for
access-plan adoption.

By the end of this milestone:

- Milestone 6 closeout is the only accepted start point for covered graph-read
  declarations
- topology closure and spatial touch authority lower into covered Query
  read-family declarations before any covered read executes
- graph-read families are registered by touched authority and read shape, not
  by operator-local read-plan code
- access requirement rows are derived through Query vocabulary instead of
  hand-written Worth requirement mirrors
- missing Query support becomes typed capability-gap evidence with owner, cap,
  blocker, expected denial, and removal trigger
- old Worth-local declaration shims, access requirement mirrors, and traversal
  fallback residue are deleted or mechanically capped
- Milestone 8 receives a seed containing declaration catalog identity,
  requirement-row evidence, capability gaps, deletion ledger evidence, and no
  execution authority

Milestone 7 does **not** close admitted access-plan execution, plan
consumption, access receipts, validator derivation, invalidation, replay,
conflict, cache, or public diagnostics.

## Why This Milestone Exists

Milestone 6 classified graph-read folklore and produced the seed this milestone
must consume. The dangerous next step would be letting each Worth operator turn
that inventory into its own "small read" helper, "safe adjacency" helper, or
hand-written access requirement list.

That would recreate the exact local-authority problem Query `9.10` was built
to remove: the declaration would live in Worth, the access requirements would be
copied in Worth, denial handling would become caller policy, and Milestone 8
would inherit local traversal instead of a Query-admitted access plan.

Milestone 7 is therefore an architectural rollover. Build the declaration lane
fresh, migrate from the Milestone 6 seed into Query-owned declaration and
requirement surfaces, then delete or cap local declaration residue before
access-plan adoption begins.

## Governing Summaries

- `MENTALITY.md`: protects adversarial-constraint-first engineering. This
  milestone must make caller-owned graph-read declaration folklore impossible
  before boolean, NURBS, extrusion, and fillet work starts depending on local
  neighborhoods.
- `arch_laws.md`: protects proof-bearing phase chains. Milestone 6 seed data
  must lower into declaration catalog proof, requirement-row proof, capability
  gap proof, deletion proof, and a Milestone 8 seed without weaker products
  promoting themselves.
- `composition_laws.md`: protects named responsibilities. Declaration catalog,
  seed consumption, Query lowering, capability gaps, deletion ledger, and
  closeout proof need separate homes rather than a broad graph-read helper.
- `domain_structure_laws.md`: protects visible authority boundaries.
  `forge-query` owns read-family, requirement, support posture, denial, and
  access-planning vocabulary; Worth owns touched-authority inputs and closeout
  pressure.
- `perf_laws.md`: protects semantic-delta-bounded reads. Declarations must be
  keyed by touched authority and explicit access shape, not by broad scans,
  whole-graph caches, or per-operator rediscovery.
- `touched-graph-roadmap.md`: places this milestone after inventory hard break
  and before access-plan adoption because Milestone 8 needs registered
  declaration authority, not another source-grep expedition.
- `AI_README.md`: protects the declare-once architecture. A caller declares
  graph read intent once, Query lowers that intent into requirements and
  admission evidence, and unsupported shapes become typed denials or required
  postures instead of local fallback walks.

## Adversarial Constraint

Given a Milestone 6 graph-read access seed, a large topology/spatial workspace,
and a small touched graph region, every covered Worth graph-read declaration
must lower through one Query-owned read-family catalog entry with derived
Query-owned requirement rows, support posture evidence, and typed capability
gap evidence when support is missing.

Worth-local declaration shims, operator-local access requirement arrays,
copied Query labels with no Query authority, traversal fallback loops after
denial, persistent-index wishes encoded as comments, fabricated support rows,
and compatibility wrappers that preserve local read planning must fail closed
or appear as explicit capped residue with owner, cap, blocker, and removal
trigger.

No Milestone 8 access-plan adoption work may depend on a Worth-local read
declaration or access requirement helper after the same shape has a Milestone 7
Query declaration candidate, requirement row, capability gap, or deletion row.

## Product Decision Lock

- Use the parallel migration plus hard deletion format.
- Build a new responsibility-named graph-read declaration lane beside the
  Milestone 6 inventory lane; do not refactor old declaration/adoption files in
  place.
- Consume `WorthGraphReadAccessMilestoneSevenSeed` as the only production
  start point for covered declaration work.
- Do not reopen source inventories manually except in tests that prove the seed
  covers every production-reachable declaration input.
- Query owns `ForgeQueryReadFamily`, read family admission, access requirement
  kinds, access requirement rows, admission postures, denial kinds, support
  owners, and receipt vocabulary.
- Worth owns lowering touched topology/spatial authority into Query declaration
  inputs, preserving source identity, public closeout pressure, and deletion
  enforcement.
- Deletion is preferred over residue. Residue is allowed only with owner, cap,
  blocker, removal trigger, and certification preventing growth.
- Milestone 7 may derive requirements, support posture evidence, denials, and
  capability gaps. It must not execute access plans, consume
  `graph_read_access_plan_consumption`, or fabricate receipt evidence.
- "Registered catalog" means a Worth closeout catalog that references
  Query-owned read-family and requirement authority. It does not mean inventing
  a new persistent Query registry if Query currently exposes process-owned
  reusable read-family artifacts.

## Implicit Assumptions Made Explicit

- Milestone 6 seed coverage is trusted as the source of truth, but Milestone 7
  must still prove the seed has no uncapped old graph-read folklore before it
  becomes declaration input.
- A declaration is incomplete unless it names touched authority, read family
  target, access shape, selectivity posture, basis/snapshot posture, operating
  world, policy/tenant posture where relevant, requirement-row evidence,
  support posture, and the Milestone 8 adoption target.
- Query capability surfaces may be vocabulary-only at this point. When Query
  cannot provide real derivation, admission, or support evidence for a shape,
  the correct Milestone 7 output is a typed capability gap, not a local Worth
  implementation that pretends Query support exists.
- Topology and spatial lowering may have different domain source facts, but
  they must converge before declaration catalog identity. Divergence after the
  catalog boundary is a product gap.
- Tests are allowed to use small fixtures, but the proof target must be the
  production closeout/declaration path and Query-owned vocabulary. A test that
  only formats a local report is not proof.
- Deletion is not a cleanup epilogue. Every phase that creates a replacement
  must either delete the displaced local surface in that phase or carry a
  capped residue row with owner, blocker, and removal trigger into Phase 6.
- The Milestone 8 seed is a handoff artifact, not a promise that execution will
  succeed. Unsupported shapes remain visible as gaps so access-plan adoption
  cannot silently route around them.

## Artifact Chain

```text
Milestone 6 graph-read access closeout
  -> WorthGraphReadAccessMilestoneSevenSeed
  -> graph-read declaration catalog
  -> Query-derived requirement rows or typed capability gaps
  -> deletion/residue firewall proof
  -> Milestone 7 closeout
  -> Milestone 8 access-plan adoption seed
```

Every arrow is a proof boundary. A later artifact may reference an earlier
digest or row identity, but it must not rebuild earlier authority from source
files, local helper output, or copied strings.

## Developer Target

The intended final feel is declaration-first and boring:

```rust
let declaration_closeout =
    current_worth_graph_read_access_declaration_closeout(
        milestone_six_closeout.into_milestone_seven_seed(),
    )?;

let read_family = declaration_closeout
    .catalog()
    .read_family_for(touched_authority, graph_read_shape)?;

let requirement_rows = read_family.query_requirement_rows();
let milestone_eight_seed = declaration_closeout.into_milestone_eight_seed();
```

The caller asks for a read family through touched authority. It does not name
adjacency loops, frontier loops, visited sets, persistent-index fallback policy,
or local access requirements.

## Phase Plan

### Phase 1: Milestone 6 Seed Contract And Parallel Declaration Lane

This phase creates the new declaration lane and freezes its input contract. The
implementation must consume `WorthGraphReadAccessMilestoneSevenSeed` from the
Milestone 6 closeout and expose only read-only access to declaration
candidates, capability gaps, deletion items, counters, and no-execution
authority proof.

**Relevant subsystems**
- `crates/worth-kernel/src/graph_read_access_inventory/phase_six_closeout`
- `crates/worth-kernel/src/graph_read_access_inventory/candidates`
- `crates/worth-kernel/src/graph_read_access_inventory/capability_gaps`
- new responsibility-named declaration lane under `crates/worth-kernel/src`

**Relevant APIs**
- `WorthGraphReadAccessMilestoneSevenSeed`
- `WorthGraphReadDeclarationCandidate`
- `WorthGraphReadQueryAccessCapabilityGap`
- `WorthGraphReadDeletionLedgerItem`
- `WorthGraphReadAccessMilestoneSevenSeed::claims_execution_authority()`

**Warnings**
- Do not construct declarations from raw source paths, raw inventory rows, or
  direct source scans.
- Do not place the new work under a `phase_seven`, `new`, `v2`, or
  `migration` directory name.
- Do not let the seed become execution authority. `claims_execution_authority()`
  must remain false.

**Test requirements**
- `declaration_lane_accepts_only_milestone_six_seed`: the new closeout path
  accepts the Milestone 6 seed and rejects raw candidates, raw counters, raw
  deletion rows, and fabricated capability gaps.
- `milestone_seven_seed_cannot_claim_execution_authority`: declaration closeout
  fails if the input seed or output closeout claims access-plan execution,
  receipt consumption, or runtime graph-read work.
- `declaration_lane_preserves_seed_counts_exactly`: candidate, capability-gap,
  deletion, and residue counts match the Milestone 6 seed before any lowering
  occurs.
- `declaration_lane_rejects_uncapped_old_folklore_seed`: seed input fails if
  `contains_uncapped_old_graph_read_folklore_as_declaration_or_gap()` reports
  old graph-read adoption residue in declaration or capability-gap position.

**Engineering decisions**
- The lane name should describe the durable responsibility, such as
  `graph_read_access_declarations`.
- The first public artifact is a closeout/seed proof, not an operator helper.

### Phase 2: Registered Read Family Declaration Catalog

This phase turns seed candidates into a registered declaration catalog keyed by
touched authority input, read family target, requirement vocabulary, and
lowering target. The catalog is the only Worth-facing place where covered read
families are named.

**Relevant subsystems**
- `crates/worth-kernel/src/graph_read_access_inventory/candidates`
- `crates/forge-query/docs/authoring/read-composition.md`
- `crates/forge-query/docs/authoring/graph-read-access-planning.md`

**Relevant APIs**
- `WorthGraphReadDeclarationCandidate::read_family_target()`
- `WorthGraphReadDeclarationCandidate::touched_authority_input()`
- `WorthGraphReadDeclarationCandidate::requirement_vocabulary()`
- `WorthGraphReadDeclarationCandidate::milestone_seven_lowering_target()`
- `ForgeQueryReadFamily`
- `ForgeQueryReadFamilyAdmission`

**Warnings**
- Do not make one read-family wrapper per operator.
- Do not key catalog identity by source file path or test fixture label.
- Do not copy Query family vocabulary into Worth-owned enums unless those
  values are sealed projections over Query authority.

**Test requirements**
- `catalog_registers_each_seed_candidate_once`: every Milestone 6 declaration
  candidate appears in the declaration catalog exactly once.
- `catalog_rejects_conflicting_touched_authority_keys`: two declarations with
  the same touched authority and read shape but different requirement
  vocabulary fail with a typed conflict.
- `one_catalog_family_can_cover_multiple_callers`: multiple operator or
  certification callers can reference the same catalog read family without
  creating duplicate local declaration code.
- `catalog_identity_is_stable_under_source_ordering`: declaration catalog
  digests are independent of source discovery order and depend on touched
  authority, read family target, requirement basis, support posture, and
  lowering target.
- `catalog_record_requires_complete_declaration_dimensions`: catalog records
  fail unless they include touched authority, access shape, selectivity posture,
  basis/snapshot posture, operating world, policy/tenant posture when relevant,
  requirement evidence, support posture, and Milestone 8 adoption target.

**Engineering decisions**
- Catalog records must preserve source row identity for diagnostics, but source
  row identity is not declaration authority.
- Declaration identity must be digestible and stable enough for the Milestone 8
  seed to reference without rebuilding the catalog.

### Phase 3: Touched Authority Lowering From Topology And Spatial Inputs

This phase proves that topology closure and spatial touch authority lower into
the same declaration catalog shape. Worth may translate domain authority into
Query declaration inputs, but it may not keep topology-only or spatial-only
local graph-read declaration systems.

**Relevant subsystems**
- topology touched graph basis and closure products
- spatial Query touch descriptor and evidence touch authority products
- `crates/forge-query/docs/authoring/graph-touch-obligation-authority.md`
- `crates/forge-query/docs/authoring/graph-obligation-consumer-kit.md`

**Relevant APIs**
- `WorthGraphReadDeclarationCandidate::inventory_row_context()`
- `WorthGraphReadReadFamilyTarget`
- Query graph touch descriptor builders
- Query operating world descriptor builders
- Query selector/support posture vocabulary

**Warnings**
- Do not treat selected graph obligations as a substitute for graph-read access
  declarations.
- Do not let spatial descriptors use a broad collection fallback when a touched
  authority input exists.
- Do not let topology closure rebuild adjacency maps merely to prove it can
  name a read declaration.

**Test requirements**
- `topology_and_spatial_inputs_lower_through_same_catalog`: representative
  topology and spatial seed candidates lower into the shared declaration catalog
  without separate declaration shims.
- `selected_obligation_proof_cannot_stand_in_for_read_declaration`: graph
  obligation support/adoption evidence is rejected when passed as graph-read
  access declaration authority.
- `raw_spatial_or_topology_strings_cannot_construct_declarations`: callers must
  provide typed touched authority inputs from the seed path, not arbitrary
  labels.
- `branch_preview_and_authoritative_worlds_do_not_collapse`: operating world
  descriptors remain part of declaration identity so branch, preview, and
  authoritative reads cannot silently share access posture.

**Engineering decisions**
- This phase should prefer small typed lowering records over one large
  cross-domain conversion function.
- If Query lacks an expressive declaration input, record a capability gap
  rather than adding a Worth-local workaround.

### Phase 4: Query Requirement Derivation And Requirement Row Evidence

This phase routes declared read families through Query-owned requirement
derivation. Worth can preserve the requirement vocabulary from the seed as
input evidence, but the output requirement rows must come from Query-owned
types and vocabulary.

**Relevant subsystems**
- `crates/forge-query/docs/authoring/graph-read-access-planning.md`
- `crates/forge-query/docs/authoring/read-composition.md`
- `crates/forge-query/src/runtime/graph_read_access`
- `crates/forge-query/src/runtime/surface/read_receipt_accessors.rs`

**Relevant APIs**
- `derive_graph_read_access_requirements(...)`
- `try_derive_graph_read_access_requirements(...)`
- `ForgeQueryGraphReadAccessRequirementRow`
- `ForgeQueryGraphReadAccessRequirementKind`
- `ForgeQueryGraphReadAccessRequirementCounters`
- `ForgeQueryReadReceipt::graph_read_access_summary()`

**Warnings**
- Do not hand-write Worth requirement rows for directional adjacency,
  traversal worksets, visited sets, predicate support, result buffers, or
  dedup sets.
- Do not treat requirement derivation as execution.
- Do not add local "no N+1" proof in place of Query requirement counters.

**Test requirements**
- `query_derives_requirement_rows_for_registered_families`: every registered
  declaration can derive Query requirement rows or produce a typed Query
  denial/capability gap.
- `worth_local_requirement_rows_are_rejected`: hand-written Worth requirement
  arrays fail even when their labels match Query labels.
- `requirement_rows_preserve_seed_vocabulary_basis`: derived rows can trace
  back to the Milestone 6 candidate vocabulary without using the candidate as
  final authority.
- `vocabulary_only_query_capability_routes_to_gap_when_derivation_is_missing`:
  if the current Query capability report can only cite vocabulary for a
  requirement family, the declaration records a capability gap instead of
  fabricating derived rows.

**Engineering decisions**
- Requirement-row evidence should be stored as Query-derived proof plus source
  candidate identity.
- Counters are planning evidence only in this milestone; execution counters
  belong to Milestone 8.

### Phase 5: Admission Posture And Capability Gap Routing

This phase classifies each declaration's support posture and routes unsupported
or not-yet-executable shapes into typed capability gaps. Milestone 7 may know
that a shape requires persistent index support, paged streaming, async
materialization, store-backed support, or access capability registration. It
must not execute the read locally when that support is missing.

**Relevant subsystems**
- `crates/worth-kernel/src/graph_read_access_inventory/capability_gaps`
- `crates/forge-query/docs/authoring/graph-read-access-planning.md`
- Query support matrix and admission vocabulary

**Relevant APIs**
- `admit_graph_read_access_for_family(...)`
- `plan_admitted_graph_read_access_for_family(...)`
- `ForgeQueryGraphReadAccessAdmission`
- `ForgeQueryGraphReadAccessDenialKind`
- `ForgeQueryGraphReadPersistentArtifactAudit`
- `WorthGraphReadQueryAccessCapabilityGap::expected_denial()`
- `WorthGraphReadQueryAccessCapabilityGap::must_not_exceed_count()`
- `WorthGraphReadQueryAccessCapabilityGap::removal_trigger()`

**Warnings**
- Do not turn `persistent_index_required`, `paged_streaming_required`, or
  `async_materialization_required` into local traversal permission.
- Do not soften a Query denial into "increase a limit and retry".
- Do not call access-plan adoption complete because declaration admission
  evidence exists.

**Test requirements**
- `missing_query_support_becomes_typed_capability_gap`: missing persistent
  index, paged streaming, async materialization, store-backed support, and
  access capability registration cases become typed gap rows with blockers and
  removal triggers.
- `capability_gap_counts_are_capped`: every capability-gap family enforces its
  `must_not_exceed_count()` and fails if new local workaround rows appear.
- `denial_does_not_enable_local_graph_walk`: denied declarations cannot route
  to adjacency loops, broad scans, local graph caches, or operator-local
  fallback plans.
- `admission_posture_is_not_receipt_or_plan_consumption`: admitted,
  required-posture, and denied declaration evidence cannot populate execution
  counters, plan-consumption digests, streaming receipts, or ephemeral index
  receipts.

**Engineering decisions**
- `plan_admitted_graph_read_access_for_family(...)` may be referenced for
  boundary proof, but consumed plans and receipts are Milestone 8 work.
- Gap rows are first-class closeout evidence, not TODO comments.

### Phase 6: Local Declaration Residue Deletion And Firewalls

This phase deletes or mechanically caps the old declaration and requirement
residue now that the registered catalog and Query requirement rows exist. The
goal is hard break, not coexistence.

**Relevant subsystems**
- old graph-read adoption/declaration scaffolding under `worth-kernel`
- topology and spatial local read helpers that still name declaration behavior
- certification source firewalls and public facade contracts
- Milestone 6 deletion ledger

**Relevant APIs**
- `WorthGraphReadDeletionLedgerItem`
- `WorthGraphReadAccessMilestoneSevenSeed::contains_uncapped_old_graph_read_folklore_as_declaration_or_gap()`
- Query hard-prohibition/bypass audit surfaces

**Warnings**
- Do not leave adapters that translate old declaration helpers into the new
  catalog unless they are capped residue with removal trigger.
- Do not preserve old helpers because tests still import them. Update tests to
  prove the public path instead.
- Do not leave "temporary" compatibility wrappers without certification
  failure on growth.

**Test requirements**
- `old_local_declaration_shims_are_deleted_or_capped`: source firewall tests
  fail on old helper names, local declaration constructors, local requirement
  mirrors, local access support rows, and fallback traversal helpers unless
  they appear in capped residue.
- `deletion_ledger_matches_milestone_six_items`: every Milestone 6 deletion
  ledger item is either removed or carried as capped residue with blocker and
  removal trigger.
- `public_api_cannot_construct_local_graph_read_declaration`: public Worth APIs
  can consume catalog declarations but cannot fabricate local declaration
  authority.
- `replacement_phases_carry_deletion_or_residue_proof`: every old surface
  displaced by Phases 2 through 5 has a same-phase deletion proof or capped
  residue row before Phase 6 begins.

**Engineering decisions**
- Deletion should happen after catalog parity exists, not before.
- The firewall should look for behavior/responsibility names, not only exact
  old filenames.

### Phase 7: Public Closeout And Milestone 8 Seed

This phase publishes the final Milestone 7 proof product. The closeout must be
read-only from public callers, must expose declaration catalog identity and
requirement-row evidence, and must seed Milestone 8 without claiming execution.

**Relevant subsystems**
- new graph-read declaration closeout lane
- worth public certification/closeout facade
- touched graph roadmap closeout tests
- Milestone 8 seed construction

**Relevant APIs**
- `ForgeQueryReadFamily`
- `ForgeQueryGraphReadAccessRequirementRow`
- `ForgeQueryGraphReadAccessAdmission`
- `ForgeQueryReadReceipt::graph_read_access_plan_consumption()`
- `ForgeQueryReadReceipt::ephemeral_graph_index_receipt()`
- `ForgeQueryReadReceipt::graph_read_streaming_receipt()`
- `ForgeQueryReadReceipt::live_graph_read_access()`

**Warnings**
- Do not expose mutable declaration constructors through public closeout.
- Do not expose receipt fields as populated in Milestone 7. They are named as
  Milestone 8 requirements only.
- Do not let the closeout hide capability gaps or deletion residue behind a
  single "complete" boolean.

**Test requirements**
- `milestone_seven_closeout_exports_milestone_eight_seed`: closeout produces a
  seed containing declaration catalog digest, read family identities,
  requirement-row digests, capability-gap digests, deletion proof, counters,
  and no execution receipts.
- `closeout_does_not_claim_access_plan_consumption`: receipt accessors such as
  `graph_read_access_plan_consumption`, `ephemeral_graph_index_receipt`,
  `graph_read_streaming_receipt`, and `live_graph_read_access` remain
  unclaimed by Milestone 7 proof.
- `roadmap_counts_match_implementation_state`: certified counts for
  declarations, derived requirement rows, gaps, residue, and deletions match
  the public closeout and the roadmap.
- `milestone_eight_seed_preserves_gap_and_residue_visibility`: unsupported
  declarations and capped residue remain visible in the seed so Milestone 8
  cannot treat absence of execution support as completed adoption.

**Engineering decisions**
- Milestone 8 seed identity should be stable enough that access-plan adoption
  can start without rebuilding declaration catalog state from source.
- Closeout diagnostics should make remaining gaps boring to inspect: what is
  missing, who owns it, what denial is expected, and what removes it.

## Must Ship

- A new responsibility-named graph-read declaration lane built in parallel from
  the Milestone 6 seed.
- A registered declaration catalog for covered Worth graph reads keyed by
  touched authority and read family shape.
- Query-derived access requirement row evidence for every supported declaration
  candidate.
- Typed capability-gap rows for every declaration shape Query cannot support
  yet.
- Deletion or capped residue for old local declaration shims, access
  requirement mirrors, fallback traversal helpers, and fabricated support rows.
- A public Milestone 7 closeout and Milestone 8 seed that cannot claim graph
  read execution authority.

## Must Preserve

- Milestone 6 inventory and deletion evidence as historical proof input.
- Query ownership of graph-read declaration, requirement, admission, denial,
  support posture, and receipt vocabulary.
- Worth ownership of topology/spatial touched authority inputs and closeout
  pressure.
- Current production behavior on paths not yet covered by the declaration
  catalog, provided any residue is capped and certified.

## Must Not Ship

- Operator-local read-family helpers.
- Worth-local access requirement enums or arrays that duplicate Query authority.
- Local fallback graph walks after Query denial.
- Broad scans, adjacency caches, or local no-N+1 contracts presented as
  declaration proof.
- Public constructors that fabricate graph-read declaration authority.
- Access-plan execution, plan consumption, or receipt proof. That is
  Milestone 8.

## Acceptance

Milestone 7 is done when a reviewer can start from the Milestone 6 closeout,
construct the Milestone 7 declaration closeout, inspect every registered read
family and Query-derived requirement row, see every unsupported shape as a
typed capability gap, verify local declaration residue is deleted or capped,
and hand the Milestone 8 seed to access-plan adoption without preserving any
Worth-local graph-read declaration authority.
