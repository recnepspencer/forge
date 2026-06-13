# Worth Milestone 7.0: Planar Boolean Entry And Anti-Theatre Boundary

> **Status:** Draft
>
> **Purpose:** freeze the Query-owned public entry, workload truth, and
> anti-theatre fences for planar B-rep booleans before any split / classify /
> assemble execution is allowed to claim progress.

## Goal

Milestone 7.0 establishes the only honest way planar B-rep boolean work may
enter the system.

By the end of this milestone:

- Query owns the ordinary declaration, admission, readiness, route, receipt,
  envelope, and ordinary-outcome boundary for planar booleans
- `worth-topo`, `worth-spatial`, and `worth-kernel` expose the exact workload
  surfaces needed for boolean setup through real typed artifacts
- workload catalog recipes can build real boolean operand pairs through the
  same workload substrate used by hostile proof
- boolean-specific evidence rows, blocker provenance, and user outcomes exist
  so later milestones cannot hide behind fixture theatre
- compile-fail and public-contract fences make fake boolean proof harder than
  the real path

Milestone 7.0 does **not** implement split, classify, assemble, or cleanup.
It builds the public and mechanical boundary that every later `7.x` milestone
must use.

## Why This Milestone Exists

The main risk at the start of planar booleans is not only incorrect geometry.
The deeper risk is that the team accidentally starts boolean work through a
fake path:

- local kernel routing instead of Query-owned declaration and lowering
- ad hoc body-pair fixtures instead of workload-catalog recipes
- hand-filled boolean evidence rows instead of receipt-backed evidence stages
- boolean tests that exercise projected geometry directly without proving the
  topology, binding, support, transform, replay, diagnostics, and response
  rails were real
- local status enums that flatten supported, unsupported, blocked, denied,
  policy-required, and integrity-mismatch posture into one fake “boolean ran”
  result

If that boundary is not frozen first, every later split milestone becomes
harder to trust and harder to review.

## Governing Summaries

- `MENTALITY.md`: protect the adversarial constraint first. The milestone must
  close the fake-entry and fake-proof problem before execution work starts.
- `arch_laws.md`: protect proof-bearing phase transitions and authority
  separation. Query must own declaration/lowering; topology, spatial, and
  kernel must each consume only their own authority surfaces.
- `composition_laws.md`: protect semantic decomposition. The spec must split
  declaration entry, catalog construction, evidence stages, user outcomes, and
  fences into separate phases rather than one boolean “setup” bucket.
- `domain_structure_laws.md`: protect tree topology as ownership proof. The
  code targets must stay discoverable by crate and module boundary instead of
  hiding boolean setup across helpers.
- `perf_laws.md`: protect visible cost and route honesty. Declaration,
  readiness, support, and evidence boundaries must expose named counters rather
  than broad hidden orchestration.
- `forge-query/docs/AI_README.md`: protect the rule `declare intent once, lower
  it once, execute or inspect it through canonical runtime-owned artifacts`.
  This milestone must not invent a kernel-local router, local pseudo-Query
  lane, or caller-owned boolean identity path.
- `_docs/worth/worth_roadmap.md`: protect `Milestone 7` as the planar B-rep
  boolean band and `Milestone 8` as EMBER.
- `_docs/worth/milestone-7-roadmap.md`: protect `Milestone 7.0` as the entry
  and anti-theatre boundary for the full `7.x` band.
- `_docs/worth-kernel/milestone-6.5.md`: protect the existing workload
  platform. Boolean work must extend that substrate, not bypass it.

## Existing Surface Inventory

Milestone 7.0 must reuse and widen the following live surfaces before adding
 new ones:

- `crates/worth-topo/src/workload_platform/topology_workload.rs`
  - `TopologyWorkload`
  - `TopologyWorkloadDeclaration`
  - `TopologyWorkloadReceipt`
  - `TopologyWorkloadDenial`
- `crates/worth-topo/src/workload_platform/declaration_identity.rs`
  - `TopologyWorkloadDeclarationIdentity`
- `crates/worth-topo/src/workload_platform/support_posture.rs`
  - `TopologyWorkloadFamily`
  - `TopologyWorkloadSupport`
  - `TopologyWorkloadSupportPosture`
- `crates/worth-topo/src/workload_platform/topology_seed/*`
  - `TopologySeed`
  - `TopologySeedReceipt`
  - `TopologySeedCleanFailReceipt`
  - topology-seed posture and neighborhood receipts
- `crates/worth-topo/src/workload_platform/nmt_topology_construction/*`
  - `NmtTopologyConstructionReceipt`
  - `TopologyPostureReceipt`
  - `OpenBoundaryReceipt`
  - `RadialAdjacencyReceipt`
- `crates/worth-spatial/src/facade/workload_vocabulary/mod.rs`
  - `GeometryBindingWorkloadReceipt`
  - `SurfaceSupportWorkloadReceipt`
  - `ProjectionWorkloadReceipt`
  - `TransformWorkloadReceipt`
  - `RetainedReplayWorkloadReceipt`
  - `DiagnosticWorkloadReceipt`
  - `ResponseWorkloadReceipt`
  - `WorkloadStageEnvelope`
  - `WorkloadStageIdentity`
  - `WorkloadStagePosture`
  - `WorkloadStageSupport`
- `crates/worth-spatial/src/facade/projection_workload/mod.rs`
  - `ProjectionWorkload`
  - `ProjectedPlanarWorkload`
  - `ProjectionConsumedWorkloadReceipt`
  - `ProjectionReceiptSet`
  - `CertifiedLocalFrameReceipt`
- `crates/worth-spatial/src/facade/transform_workload/mod.rs`
  - `TransformWorkload`
  - `TransformReceiptSet`
  - `TransformPostureReceipt`
  - `TransformEvidenceSet`
  - `TransformParityReport`
- `crates/worth-spatial/src/facade/retained_replay_workload/mod.rs`
  - `ReplayWorkload`
  - `ReplayReceiptSet`
  - `RetainedArtifactCaptureReceipt`
  - `RetainedArtifactSet`
  - `ReplayParityReport`
- `crates/worth-spatial/src/facade/user_response/mod.rs`
  - `WorthUserOutcome`
  - `WorthUserOutcomeKind`
  - `WorthUserOutcomeCause`
  - `WorthDeniedCause`
  - `WorthUnsupportedCause`
  - `WorthNoOptionsCause`
  - `WorthIntegrityMismatchCause`
  - `WorthPolicyDecision`
  - `WorthUserResponseReceipt`
- `crates/worth-spatial/src/workload_platform/boolean_readiness_workload/*`
  - `PlanarBooleanReadinessWorkload`
  - `PlanarBooleanReadinessWorkloadReceipt`
  - `PlanarBooleanReadinessWorkloadDenial`
  - `PlanarBooleanReadinessWorkloadDenialKind`
  - `PlanarBooleanReadinessEvidenceBasis`
  - `PlanarBooleanReadinessRequiredStage`
- `crates/worth-spatial/src/workload_platform/blocker_provenance/*`
  - `WorkloadBlockerProvenanceReceipt`
  - `WorkloadBlockerSourceKind`
  - `WorkloadBlockerBoundaryKind`
- `crates/worth-kernel/src/workload_composition/worth_workload.rs`
  - `WorthWorkload`
  - `WorthWorkloadParts`
  - `WorkloadCompositionError`
- `crates/worth-kernel/src/workload_composition/stage_requirements.rs`
  - `WorkloadStageRequirement`
- `crates/worth-kernel/src/workload_composition/operator_harness/*`
  - `WorkloadOperator`
  - `WorkloadOperatorFamily`
  - `OperatorDeclarationReceipt`
  - `OperatorSupportReceipt`
  - `OperatorRun`
  - `OperatorOutcome`
- `crates/worth-kernel/src/workload_composition/workload_catalog/*`
  - `WorkloadCatalog`
  - `WorkloadCatalogRecipe`
  - `WorkloadCatalogRecipeKind`
  - `WorkloadCatalogDeclarationReceipt`
  - `WorkloadCatalogSupportReceipt`
  - `BuiltWorkloadCatalogRecipe`
  - `BuiltCleanFailCatalogRecipe`

New 7.0 surfaces are allowed only where this inventory cannot honestly express
Query-owned boolean declaration identity, admitted boolean operand pairs,
boolean evidence stages, boolean blocker provenance, or boolean anti-theatre
closeout proof.

## Adversarial Constraint

An engineer or agent must not be able to claim “planar boolean work has
started” by fabricating a local boolean path outside Query, by constructing a
body-pair test world without workload receipts, by replaying through
re-extraction instead of retained artifacts, by hand-filling boolean evidence
rows, or by collapsing denied / unsupported / blocked / policy-required posture
into one generic boolean result.

For the same declared planar boolean intent, the system must preserve:

- one canonical Query declaration family
- one canonical declaration identity
- one explicit support posture row
- one explicit readiness and route-plan story
- one receipt / envelope path
- one operand workload-construction path
- one boolean evidence-stage ledger story
- one blocker provenance story
- one user-outcome taxonomy

If a supported path can be entered without those artifacts, or if a fake path
can generate equivalent public proof, the milestone has failed.

## Product Decision Lock

- `Milestone 7.0` freezes entry and proof only. It does not implement boolean
  execution stages.
- Query is the ordinary public boundary. Do not design a kernel-owned boolean
  front door.
- B-rep is the only admitted execution lane after 7.0. EMBER must appear only
  as an explicit support/admission posture, never as a fake admitted lane.
- `PlanarBooleanReadinessWorkloadReceipt` is the required entry basis for
  admitted planar boolean execution work.
- Boolean operand recipes must be built through workload-catalog paths that
  terminate in `WorthWorkload`, not through spatial-only helpers.
- Boolean tests must remain workload-backed proofs. No synthetic boolean
  evidence, no kernel summary substitution, no test-local route folklore.

## Phase Plan

### Phase 1: Freeze Query Boolean Declaration Family

Phase 1 creates the Query-owned declaration vocabulary for planar booleans and
binds that vocabulary to explicit support posture before any boolean work can be
constructed.

**Relevant subsystems**
- Query public declaration/admission/readiness/orchestration vocabulary
- `worth-kernel` workload operator declaration and support posture
- `worth-spatial` boolean-readiness receipt family

**Construction requirements**
- Add a first-class planar boolean declaration family beside the existing
  workload/operator declaration families.
- The declaration family must preserve:
  - boolean operation kind (`union`, `intersect`, `subtract`) as typed intent
  - operand-pair declaration identity
  - requested execution lane posture (`BRepNow`, `EmberFuture`)
  - basis link to `PlanarBooleanReadinessWorkloadReceipt`
- Extend `crates/worth-kernel/src/workload_composition/operator_harness/declaration.rs`
  so `WorkloadOperatorFamily` can name planar boolean families explicitly rather
  than collapsing everything future-facing into `UnsupportedOperatorFamily`.
- Add a Query-backed declaration receipt analogous to
  `OperatorDeclarationReceipt` for planar boolean entry rather than teaching
  callers to reuse coplanar-overlap declarations.
- Add matching support-row posture in the same family as:
  - `WorkloadCatalogSupportReceipt`
  - `OperatorSupportReceipt`
- Construction target files:
  - `crates/worth-kernel/src/workload_composition/operator_harness/declaration.rs`
  - `crates/worth-kernel/src/workload_composition/operator_harness/query.rs`
  - new boolean-entry module under
    `crates/worth-kernel/src/workload_composition/`

**Relevant APIs**
- `WorkloadOperatorFamily`
- `OperatorDeclarationReceipt`
- `OperatorSupportReceipt`
- `PlanarBooleanReadinessWorkloadReceipt`
- new `PlanarBooleanDeclarationReceipt`
- new `PlanarBooleanFamily`

**Required Query posture**
- required now:
  - canonical domain declarations
  - declaration family taxonomy
  - declaration family capability matrix
  - declaration legality
  - declaration entry readiness
  - declaration route-plan identity
  - declaration boundary receipts
  - declaration boundary envelopes
- support-gated:
  - EMBER admission as an execution lane
- out:
  - kernel-local string routing
  - caller-owned boolean declaration digests

**Warnings**
- Do not treat `WorkloadOperatorFamily::Unsupported(BooleanDifference)` as an
  adequate long-term boolean entry surface.
- Do not let boolean route identity live only in a human-readable string.
- Do not make visibility imply support; the support row must be explicit.

**Test requirements**
- `planar_boolean_declaration_family_has_explicit_support_rows`
- `planar_boolean_declaration_rejects_blank_query_identity_and_missing_basis`
- `ember_lane_is_visible_but_not_admitted_on_the_7_0_boundary`

**Engineering decisions**
- Query owns the public boolean declaration family.
- B-rep is the only admitted lane in 7.0.
- EMBER is a visible support posture, not an admitted lane.

**Open questions**
- Final public names for the boolean declaration family and lane posture enum.

### Phase 2: Freeze Boolean Entry Basis From Readiness

Phase 2 freezes the exact artifact that later boolean execution milestones must
consume: the planar boolean readiness receipt and nothing weaker.

**Relevant subsystems**
- `worth-spatial` boolean-readiness workload
- `worth-kernel` workload composition
- Query retained artifact / receipt / envelope vocabulary

**Construction requirements**
- Create a boolean-entry basis wrapper that can only be constructed from
  `PlanarBooleanReadinessWorkloadReceipt`.
- The wrapper must expose:
  - readiness receipt identity
  - blocker family / denial identity
  - consumed stage coverage
  - Query declaration identity bound to the readiness result
- Reject construction from:
  - kernel summaries
  - generic workload ledgers
  - hand-built planar facts
  - topology-only seeds
- Construction target files:
  - `crates/worth-spatial/src/workload_platform/boolean_readiness_workload/*`
  - `crates/worth-kernel/src/workload_composition/worth_workload.rs`
  - new boolean-entry basis module under
    `crates/worth-kernel/src/workload_composition/`

**Relevant APIs**
- `PlanarBooleanReadinessWorkloadReceipt`
- `PlanarBooleanReadinessWorkloadDenial`
- `PlanarBooleanReadinessRequiredStage`
- `WorthWorkload`
- `CompleteWorkloadEvidenceLedger`
- new `PlanarBooleanEntryBasis`

**Required Query posture**
- required now:
  - retained artifact to next step
  - declaration boundary receipts
  - declaration boundary envelopes
  - ordinary outcomes
  - projection-consumption identity preservation
- support-gated:
  - cross-lane parity bundles
- out:
  - boolean entry from summaries
  - boolean entry from test-local retained payload bags

**Warnings**
- Do not accept `M7 readiness` as prose or summary instead of receipt identity.
- Do not allow later milestones to re-open missing basis through recovery
  folklore.
- Do not let a denied readiness receipt enter admitted boolean work.

**Test requirements**
- `planar_boolean_entry_basis_accepts_only_real_readiness_receipts`
- `boolean_entry_basis_rejects_kernel_summary_substitution`
- `boolean_entry_basis_preserves_required_stage_identity`

**Engineering decisions**
- Boolean entry consumes readiness receipts, not readiness stories.
- Basis identity must remain Query-visible and replay-visible.

**Open questions**
- Whether the entry basis lives in kernel or a thin spatial facade export.

### Phase 3: Freeze Boolean Admission / Denial / Policy Taxonomy

Phase 3 defines the machine taxonomy of boolean entry outcomes before later
execution work starts.

**Relevant subsystems**
- Query ordinary outcomes
- `worth-spatial` user response and blocker provenance
- `worth-kernel` boolean entry orchestration

**Construction requirements**
- Add a dedicated planar-boolean outcome taxonomy that distinguishes:
  - admitted
  - unsupported
  - blocked
  - denied
  - policy-required
  - integrity-mismatch
  - no-options
- Bind each denied or blocked branch to:
  - `WorthUserOutcome`
  - `WorthUserOutcomeCause`
  - `WorkloadBlockerProvenanceReceipt`
- Require every denial path to carry:
  - source kind
  - boundary kind
  - source identity
  - boundary identity
  - human-readable reason
- Construction target files:
  - `crates/worth-spatial/src/workload_platform/user_response/*`
  - `crates/worth-spatial/src/workload_platform/blocker_provenance/*`
  - new boolean outcome adapter modules under
    `crates/worth-spatial/src/workload_platform/user_response/source_adapters/`

**Relevant APIs**
- `WorthUserOutcome`
- `WorthUserOutcomeKind`
- `WorthUserOutcomeCause`
- `WorkloadBlockerProvenanceReceipt`
- `WorkloadBlockerSourceKind`
- `WorkloadBlockerBoundaryKind`

**Required Query posture**
- required now:
  - ordinary outcomes
  - checked stops
  - stop-to-recovery boundary
  - support matrix / admission
- support-gated:
  - policy branches that require future EMBER comparisons
- out:
  - local `Result<bool, String>` boolean entry APIs

**Warnings**
- Do not flatten unsupported and blocked into denied.
- Do not emit denial prose without machine provenance.
- Do not use user response as the only machine lane.

**Test requirements**
- `planar_boolean_outcome_taxonomy_preserves_all_machine_classes`
- `boolean_blocker_provenance_names_real_boundary_and_source_identities`
- `boolean_no_options_outcomes_cannot_drop_required_provenance`

**Engineering decisions**
- Ordinary outcome taxonomy is part of the boolean contract, not UI garnish.
- Blocker provenance is required for every non-admitted branch.

**Open questions**
- Final grouping of `blocked` versus `unsupported` for public wording.

### Phase 4: Freeze Boolean Workload Catalog Operand Recipes

Phase 4 widens the workload catalog so later split milestones can start from
real boolean operand pairs instead of ad hoc body fixtures.

**Relevant subsystems**
- `worth-kernel` workload catalog
- `worth-topo` topology seeds and NMT construction
- `worth-spatial` bound geometry / projection rails

**Construction requirements**
- Add real planar boolean operand-pair recipe kinds to
  `WorkloadCatalogRecipeKind`.
- Each recipe must build through:
  - `TopologySeed` or `NmtTopologyConstructionReceipt`
  - geometry binding receipt
  - surface support receipt
  - projection receipt
  - transform receipt
  - retained replay receipt where relevant
  - diagnostics and response receipts
  - `WorthWorkload`
- Add at least these recipe families:
  - clean planar body pair
  - coplanar overlap pair
  - thin-feature pair
  - high-valence contact pair
  - dirty clean-fail pair
  - open / unbounded denial pair
- Construction target files:
  - `crates/worth-kernel/src/workload_composition/workload_catalog/recipe_kind.rs`
  - `crates/worth-kernel/src/workload_composition/workload_catalog/catalog.rs`
  - `crates/worth-kernel/src/workload_composition/workload_catalog/recipe_pipeline.rs`
  - `crates/worth-kernel/src/workload_composition/workload_catalog/built_recipe.rs`

**Relevant APIs**
- `WorkloadCatalog`
- `WorkloadCatalogRecipe`
- `WorkloadCatalogRecipeKind`
- `BuiltWorkloadCatalogRecipe`
- `BuiltCleanFailCatalogRecipe`

**Required Query posture**
- required now:
  - declaration support posture per recipe
  - retained artifact consumption for replay-bearing recipes
  - ordinary outcome posture for clean-fail recipes
- support-gated:
  - EMBER-bearing recipe admission
- out:
  - Boolean recipes that build only projected geometry without workload receipts

**Warnings**
- Do not add recipe kinds that bypass `WorthWorkload`.
- Do not add boolean recipe support rows that claim admission without real
  build paths.
- Do not let dirty/open recipes fabricate admitted workloads.

**Test requirements**
- `boolean_catalog_recipes_build_real_workload_operand_pairs`
- `boolean_clean_fail_catalog_recipes_deny_without_fabricating_workloads`
- `boolean_catalog_support_rows_match_real_recipe_posture`

**Engineering decisions**
- Boolean fixtures are catalog recipes or they do not count.
- Clean-fail recipes are first-class products, not missing rows.

**Open questions**
- Final minimum admitted boolean operand-pair recipe set for 7.0.

### Phase 5: Freeze Boolean Evidence Stages And Ledger Requirements

Phase 5 extends the workload-evidence story so later boolean execution stages
can only count if their receipts are mechanically represented in the ledger.

**Relevant subsystems**
- `worth-spatial` workload vocabulary and evidence ledger
- `worth-kernel` workload composition
- Query receipt / envelope identity boundary

**Construction requirements**
- Add explicit boolean evidence stages to the shared evidence-stage vocabulary.
- The stage family must be able to represent, at minimum:
  - boolean declaration entry
  - boolean route plan
  - boolean operand pair construction
  - boolean denial / blocker provenance
  - later split / classify / assemble / cleanup stages
- Extend `CompleteWorkloadEvidenceLedger` and `WorkloadEvidenceStageCounters`
  so boolean stage rows are counted and identity-checked the same way current
  workload stages are checked.
- Extend `WorkloadStageRequirement` only where later boolean composition needs a
  first-class requirement beyond the existing
  `Topology` / `GeometryBinding` / `SurfaceSupport` / `Projection` /
  `Transform` / `RetainedReplay` / `Diagnostics` / `Response` chain.
- Construction target files:
  - `crates/worth-spatial/src/workload_platform/evidence_ledger/*`
  - `crates/worth-spatial/src/facade/workload_vocabulary/mod.rs`
  - `crates/worth-kernel/src/workload_composition/stage_requirements.rs`
  - `crates/worth-kernel/src/workload_composition/worth_workload.rs`

**Relevant APIs**
- `CompleteWorkloadEvidenceLedger`
- `WorkloadEvidenceStage`
- `WorkloadEvidenceStageCounters`
- `WorkloadEvidenceLedgerError`
- `WorkloadStageRequirement`
- `WorthWorkload`

**Required Query posture**
- required now:
  - machine identity from declaration / receipt / envelope artifacts
  - retained artifact to next step
  - projection-consumption identity preservation
- support-gated:
  - future EMBER execution rows
- out:
  - hand-filled boolean evidence rows
  - synthetic “boolean started” ledger rows

**Warnings**
- Do not add boolean evidence rows that are not backed by real receipt
  identities.
- Do not widen `WorkloadStageRequirement` just to mirror anticipated future
  milestones unless 7.0 needs the requirement mechanically now.
- Do not let later stages hide behind generic “boolean evidence.”

**Test requirements**
- `boolean_evidence_ledger_rejects_missing_or_mismatched_boolean_stage_rows`
- `boolean_stage_counters_count_real_receipt_backed_boolean_rows_only`
- `worth_workload_cannot_compose_boolean_operands_without_required_boolean_evidence`

**Engineering decisions**
- Boolean stage rows are first-class workload evidence, not test metadata.
- Identity matching is mandatory at the ledger boundary.

**Open questions**
- Which boolean evidence stages belong in 7.0 versus first appearing in 7.1.

### Phase 6: Freeze Boolean Anti-Theatre Fences

Phase 6 adds the compile-fail and public-contract fences that make the fake path
 mechanically harder than the real path.

**Relevant subsystems**
- `worth-kernel` public facade contracts
- `worth-spatial` public facade contracts
- `worth-topo` public facade contracts
- workload catalog and workload composition

**Construction requirements**
- Add compile-fail or public-contract proof that callers cannot:
  - construct admitted boolean entry from raw topology rows
  - construct boolean entry from kernel summaries
  - fill boolean evidence rows manually
  - bypass blocker provenance
  - bypass catalog recipes with spatial-only boolean fixtures
- Add contract fixtures analogous to existing M6 closeout fences that prove:
  - every boolean closeout target is workload-backed
  - every boolean blocker path is provenance-backed
  - every admitted boolean recipe has a support row and declaration identity
- Construction target files:
  - `crates/worth-kernel/src/certification/public_facade_contracts/*`
  - `crates/worth-spatial/src/certification/public_facade_contracts/*`
  - `crates/worth-topo/src/certification/public_facade_contracts/*`

**Relevant APIs**
- workload catalog public contract surfaces
- workload composition public contract surfaces
- boolean-entry basis surfaces from earlier phases
- blocker provenance receipt surfaces

**Required Query posture**
- required now:
  - support matrix honesty
  - declaration identity boundary honesty
  - lower-runtime boundary receipts / envelopes where surfaced
- support-gated:
  - cross-lane parity fences
- out:
  - test-only boolean helper lanes that have no public contract pressure

**Warnings**
- Do not count “it is inconvenient to fake” as a fence.
- Do not rely on comments or naming to protect the boundary.
- Do not leave a public constructor that could synthesize boolean readiness or
  boolean evidence.

**Test requirements**
- `boolean_public_contract_rejects_raw_topology_or_summary_based_entry`
- `boolean_public_contract_rejects_hand_filled_evidence_and_missing_provenance`
- `boolean_catalog_and_entry_surfaces_require_workload_backed_construction`

**Engineering decisions**
- Anti-theatre is part of the milestone deliverable, not cleanup.
- Public contract proof is the enforcement layer for roadmap honesty.

**Open questions**
- Whether each fence lives best as compile-fail, public-contract, or both.

### Phase 7: Freeze Boolean 7.0 Closeout Registration

Phase 7 closes the milestone by registering the exact `7.0` proof rows and
ensuring later boolean execution milestones inherit the same entry boundary.

**Relevant subsystems**
- `worth-kernel` certification and closeout registration
- `worth-spatial` boolean readiness and user-response certification
- workload catalog and workload composition public contracts

**Construction requirements**
- Add a dedicated 7.0 closeout bundle or registry that records:
  - boolean declaration-family proof
  - boolean entry-basis proof
  - boolean outcome/provenance proof
  - boolean catalog recipe proof
  - boolean evidence-stage proof
  - anti-theatre fence proof
- The registry must make missing rows, duplicate rows, or synthetic rows deny
  closeout explicitly.
- Add a handoff note inside the closeout surface that later `7.1+` milestones
  must consume the registered 7.0 entry basis rather than rebuild it.
- Construction target files:
  - new 7.0 certification modules under
    `crates/worth-kernel/src/certification/public_facade_contracts/`
  - supporting fixtures under the matching spatial/topo public contract trees

**Relevant APIs**
- boolean-entry declaration receipts
- boolean-entry basis wrapper
- workload catalog declaration/support receipts
- workload evidence ledger
- blocker provenance and user outcome receipts

**Required Query posture**
- required now:
  - support/admission closure per admitted family row
  - declaration/receipt/envelope identity closure
  - ordinary-outcome and blocker-provenance closure
- support-gated:
  - EMBER-bearing rows
- out:
  - execution-stage proof rows from 7.1+

**Warnings**
- Do not let 7.0 close with only catalog success and no anti-theatre fence.
- Do not register future split/classify/assemble proof rows here; 7.0 must stop
  at the entry boundary.
- Do not allow a boolean milestone to skip 7.0 proof by constructing a private
  certification fixture.

**Test requirements**
- `m7_0_closeout_bundle_requires_all_boolean_entry_proof_rows`
- `m7_0_closeout_bundle_rejects_duplicate_or_synthetic_boolean_rows`
- `later_boolean_milestones_must_consume_registered_7_0_entry_boundary`

**Engineering decisions**
- 7.0 closes only on public boundary proof.
- Later `7.x` milestones are downstream consumers of 7.0, not alternate entry
  stories.

**Open questions**
- Final closeout module placement and naming convention.

## Must Ship

- a Query-backed planar boolean declaration family
- explicit B-rep admitted posture and explicit future EMBER support posture
- a boolean entry basis that can only be constructed from
  `PlanarBooleanReadinessWorkloadReceipt`
- a boolean ordinary-outcome / blocker-provenance taxonomy
- workload-catalog operand-pair recipes for admitted, denied, and clean-fail
  planar boolean setup
- boolean evidence-stage additions in the workload ledger
- compile-fail and public-contract anti-theatre fences
- a dedicated 7.0 closeout registry or certification bundle

## Must Preserve

- Query as the ordinary public boolean boundary
- `worth-topo` as topology truth and topology-workload authority
- `worth-spatial` as readiness, projection, replay, diagnostics, response, and
  blocker-provenance authority
- `worth-kernel` as workload composition / orchestration consumer, not a second
  runtime
- M6 and M6.5 workload substrate honesty
- EMBER as visible but not admitted in 7.0

## Acceptance Evidence

- `cargo check -p worth-topo -p worth-spatial -p worth-kernel`
- workload composition public-contract proof for the boolean entry basis
- workload catalog public-contract proof for planar boolean operand-pair recipes
- boolean declaration family support-row and readiness proof
- blocker provenance and user-outcome contract proof
- evidence-ledger stage and counter proof for boolean entry rows
- compile-fail or public-contract proof that synthetic boolean entry paths
  cannot count as closeout evidence
- 7.0 closeout bundle proof that all required entry-boundary rows are present
  and real

## Sequencing Notes

- Do not start 7.1 split work until 7.0 closes.
- Do not widen into EMBER in 7.0.
- Do not move split / classify / assemble receipts into this milestone; 7.0 is
  the entry and anti-theatre floor only.
- If a needed public boolean lane is missing from Query, extend the Query-shaped
  declaration surface rather than building a local substitute.

## Required Self-Check

- Does the milestone solve a real structural problem or just package work
  cosmetically? Yes: it freezes the only honest public and mechanical entry
  boundary for planar booleans.
- Is the adversarial constraint precise and load-bearing? Yes: it targets the
  fake-entry and fake-proof failure mode explicitly.
- Does the roadmap justify this milestone now? Yes: every later split milestone
  depends on it.
- Does the spec preserve crate authority boundaries? Yes.
- Are the phases carrying most of the real design information? Yes.
- Is each phase centered on one conceptual detail or boundary? Yes.
- Does each phase contain at least 2 adversarial tests by default? Yes.
- Could a competent engineer map this spec into honest types, modules, and
  tests? Yes.
- Does the milestone belong in this roadmap sequence, or is it out of order?
  It belongs first in the `7.x` band because later geometry work depends on a
  real boolean entry boundary.
