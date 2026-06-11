# Worth Kernel Milestone 6.5: Operational Workload Platform

> **Status:** Draft
>
> **Purpose:** build the reusable Worth workload platform that makes real
> topology-to-geometry-to-operator proof easier than synthetic fixture staging.

## Goal

Milestone 6.5 builds a production-shaped workload platform for Worth. The
platform starts from topology truth, binds geometry through the ordinary
spatial path, certifies surface support, projects into local coordinates,
captures retained artifacts, applies real transforms, runs operator contracts,
records diagnostics, exposes user-facing outcomes, and emits an evidence ledger
that can mechanically prove the path was real.

The milestone exists because pre-MetaBoss tests exposed a structural weakness:
the fake path was easier than the real path. M6.5 reverses that. Future hostile
tests and future operators should consume reusable workload rails instead of
building private miniature geometry worlds.

## Why This Milestone Exists

Milestone 6 freezes exact planar contracts and boolean-readiness facts, but its
pre-MetaBoss closeout bar requires real end-to-end proof. The current shape
risks repeated synthetic setup: hand-built planar rectangles, identity-string
motion, replay by re-extraction, fixture-cardinality counters, and branch
matrices disconnected from topology and geometry truth.

M6.5 is inserted between M6 and M7 so the hostile proof program has a real
operational substrate before the boolean milestone consumes it. It is a
foundation milestone, not a test-helper cleanup. Its output should be useful to
later boolean, surface, feature, replay, regeneration, and interaction work
without pretending those later operators are implemented here.

## Governing Summaries

- `MENTALITY.md`: protects adversarial, foundation-first engineering. The spec
  must build the real rails before future tests and operators can keep
  inventing shortcuts.
- `arch_laws.md`: protects proof-bearing phase transitions and typed boundary
  crossings. The workload platform must be a chain of typed artifacts, not a
  helper bag.
- `composition_laws.md`: protects one responsibility per file, function, and
  test surface. Workload setup, transform semantics, retained replay, outcome
  mapping, and evidence guards must be separate named responsibilities.
- `domain_structure_laws.md`: protects authority and tree topology. Topology
  seeds belong to topology authority, geometry binding and projection belong to
  spatial authority, and kernel composition must not become a shadow runtime.
- `perf_laws.md`: protects visible breadth and honest cost. Every workload
  boundary must expose counters for topology breadth, binding breadth,
  projection breadth, transform breadth, replay breadth, and operator breadth.
- `_docs/worth/worth_roadmap.md`: protects the sequence from binding and exact
  planar contracts into booleans and later hostile proof. M6.5 belongs between
  M6 and M7 because boolean work needs a reusable, real workload substrate.

## Adversarial Constraint

Under hostile setup pressure, an agent or engineer must not be able to claim
end-to-end Worth proof by building spatial-only fixtures, changing identity
strings instead of geometry, replaying by re-running the same extractor,
counting generator arithmetic as truth, or hand-filling evidence ledgers.

Every admitted workload must prove, through typed artifacts and receipts, which
topology truth, geometry binding, surface support, projection, transform,
retained replay, operator execution, diagnostics, user response, and counters
were actually used. Unsupported future surface or operator families must be
typed unsupported with reasons, not stubbed as admitted capability.

## Product Decision Lock

- M6.5 is a platform milestone. It must produce reusable workload surfaces, not
  one-off MB test fixtures.
- The platform composes existing Query-native `worth-topo`, `worth-spatial`,
  and `worth-kernel` surfaces. It must not become a second runtime beside Query.
- Planar surface support is the admitted certification family for M6.5.
  Broader surface families may be classified and denied, but they must not be
  represented as supported operators.
- Topology seeds may be convenient to call, but they must build real topology
  truth and receipts. A cube or tetrahedron seed is a workload recipe, not a
  geometric prop.
- MB tests must remain at the end of this milestone. They are platform
  consumers and closeout proofs, not the mechanism that defines the platform.

## Phase Plan

Each phase uses the same requirement sections, and those sections are
requirements, not notes:

- **Relevant subsystems** names the owners that must participate. If a listed
  subsystem is skipped, the phase is incomplete because the workload proof has
  lost an authority boundary.
- **Relevant APIs** names the public or near-public shapes the code must expose
  or consume. These are not final naming locks, but each concept must map to a
  real type, module, facade export, or certification surface before the phase
  can close.
- **Required Query posture** explains which Query lanes must exist now, which
  lanes are intentionally support-gated, and which shortcuts are out. A phase
  may not replace a missing Query lane with local Worth ceremony.
- **Warnings** are failure modes the phase is specifically designed to prevent.
  They should become tests, compile-fail checks, visibility restrictions, or
  guard APIs where possible.
- **Test requirements** are adversarial proof rows. They must exercise real
  production or production-shaped surfaces; a passing happy path or branch-only
  mapper does not satisfy them.
- **Engineering decisions** are architectural commitments created by the phase.
  Later phases must consume them rather than silently choosing a different
  shape.
- **Resolved decision** closes phase uncertainty into an implementation
  recommendation. If later implementation proves a decision wrong, revise the
  spec first rather than building around it with a fake or local shortcut.

### Phase 1: Inventory Existing Seeds And Fixture Worlds

Phase 1 classifies the existing topology seeds, primitive corpus support,
spatial proof fixtures, and MB harnesses before new platform code is added.
Nothing should be rebuilt blindly if an existing production-shaped surface can
be elevated.

This phase must produce a decision record that future work can enforce. The
inventory has to distinguish real topology or Query-backed setup from local
spatial fixture convenience, and it has to name the migration fate of every
setup surface that could be mistaken for end-to-end proof. A seed without an
authority decision remains a risk because later phases may accidentally promote
it by reuse.

**Relevant subsystems**
- `worth-topo` topology seed and corpus support
- `worth-spatial` planar proof fixtures and Query-native geometry paths
- `worth-kernel` primitive construction and certification harnesses
- existing MB-M6 test harnesses

**Relevant APIs**
- `MinimalTopologySeed`
- `SeededTopologyCommit`
- topology primitive corpus support
- spatial planar `proof_fixture` files
- MB-M6 overlap storm support modules

**Required Query posture**
- required now:
  - identify which existing seeds already enter through Query-native topology
  - identify which fixtures bypass Query or topology truth
  - identify which receipts are production-owned versus test-local
- support-gated:
  - automatic migration tooling
- out:
  - deleting fixtures before their replacement responsibility is named

**Warnings**
- Do not create a third fixture layer beside existing seeds and proof fixtures.
- Do not preserve a helper because it is convenient if its responsibility is
  synthetic end-to-end staging.
- Do not mark a fixture as reusable unless its authority, receipts, and
  admitted scope are explicit.

**Test requirements**
- `workload_seed_inventory_classifies_existing_topology_and_spatial_setup`
- prove every existing seed or fixture touched by M6.5 is classified as
  elevate, wrap locally, delete after replacement, or leave unit-only
- `legacy_fixture_inventory_rejects_unowned_end_to_end_claims`
- prove spatial-only fixtures and re-extraction replay helpers cannot be
  classified as MB-capable workload sources

**Engineering decisions**
- Inventory is a platform input, not documentation garnish.
- Existing real seeds should be elevated rather than reimplemented.
- Unit fixtures may remain only when they are named as unit fixtures.

**Resolved decision**
- Use `SeedInventoryReport` as certification output and
  `LegacyFixtureClassification` beside the workload platform. The report is
  audit evidence; the classification vocabulary must be reusable by guards,
  compile-fail tests, and migration records.

### Phase 2: Freeze Workload Vocabulary

Phase 2 names the reusable workload concepts and their authority boundaries.
The type vocabulary must make the real path visible before any operator-specific
storm proof is refactored.

This phase must make the workload pipeline legible in code. Each named workload
stage needs a distinct proof status and construction boundary so later phases
cannot pass raw geometry or partially built setup as if it were a completed
platform artifact. The vocabulary must also be broad enough to serve future
operator families without claiming those families are admitted in M6.5.

**Relevant subsystems**
- `worth-topo` topology workload authority
- `worth-spatial` geometry binding, surface support, projection, transform,
  retained replay, diagnostics, and response authority
- `worth-kernel` operator composition and certification authority
- Forge Query runtime artifacts

**Relevant APIs**
- `WorthWorkload`
- `TopologyWorkload`
- `GeometryBindingWorkload`
- `SurfaceSupportWorkload`
- `ProjectionWorkload`
- `TransformWorkload`
- `RetainedReplayWorkload`
- `OperatorWorkload`
- `WorkloadEvidenceLedger`

**Required Query posture**
- required now:
  - declaration identities for workload construction stages
  - receipt/envelope vocabulary for every stage that crosses authority
  - support posture for admitted, unsupported, and blocked workload families
- support-gated:
  - broad public builder ergonomics
- out:
  - generic `helpers`, `fixtures`, or `common` workload bags

**Warnings**
- Do not collapse topology, geometry binding, and projection into one
  `Workload` struct with optional fields.
- Do not create names that describe provenance such as `mb_workload`.
- Do not let a later phase consume raw collections when an earlier phase should
  have produced proof-bearing workload types.

**Test requirements**
- `workload_vocabulary_preserves_authority_boundaries`
- prove topology, geometry binding, surface support, projection, transform,
  replay, operator, diagnostics, response, and evidence artifacts are distinct
  types with distinct construction paths
- `workload_vocabulary_blocks_raw_spatial_fixture_as_operator_input`
- prove an operator harness cannot accept hand-built spatial-only geometry as a
  complete workload

**Engineering decisions**
- The workload platform is a proof-widening pipeline.
- The same vocabulary should serve tests and future production operators, but
  certification adapters remain thin consumers of the platform.

**Resolved decision**
- Put `WorthWorkload` in `worth-kernel` as the cross-crate composition wrapper.
  Keep `TopologyWorkload` in `worth-topo`, and keep geometry binding,
  projection, transform, retained replay, response, and evidence workload
  artifacts in `worth-spatial` unless a later phase proves kernel ownership is
  required.

### Phase 3: Build Topology Workload Seeds

Phase 3 creates real topology-backed workload seeds for common and hostile
model shapes. These seeds are convenience rails over topology truth, not
geometry props.

This phase must make real topology the default starting point for future tests.
The seed APIs should be easy enough that a caller reaches for them instead of
hand-building local fake faces, but strict enough that every seed carries
topology receipts, admitted or clean-fail posture, and entity identities. The
seed catalog is allowed to be convenient; it is not allowed to hide whether the
topology is valid, open, dirty, non-manifold, or unsupported.

**Relevant subsystems**
- `worth-topo` topology truth and construction surfaces
- `worth-topo` topology validation and corpus support
- `worth-kernel` primitive construction consumers
- Forge Query topology declaration and receipt lanes

**Relevant APIs**
- `TopologySeed::cube`
- `TopologySeed::tetrahedron`
- `TopologySeed::single_face_loop`
- `TopologySeed::multi_face_shell`
- `TopologySeed::open_sheet`
- `TopologySeed::open_wire`
- `TopologySeed::high_valence_vertex`
- `TopologySeed::self_intersecting_loop`
- `TopologySeed::non_manifold_wire`
- `TopologyWorkload`

**Required Query posture**
- required now:
  - topology declaration receipt
  - topology entity identity receipt
  - validation or clean-fail receipt for dirty/open seeds
  - topology-local neighborhood receipt for high-valence seeds
- support-gated:
  - arbitrary production authoring UI paths
- out:
  - building seed topology by directly filling spatial or kernel structs

**Warnings**
- Do not call a cube seed complete because coordinates exist.
- Do not let invalid topology seeds become admitted topology workloads.
- Do not hide non-manifold or open topology posture as fixture metadata.

**Test requirements**
- `topology_workload_seeds_build_real_topology_truth`
- prove cube, tetrahedron, loop, shell, open, high-valence, dirty, and
  non-manifold seeds produce topology receipts and entity identities
- `topology_workload_seeds_fail_closed_for_invalid_topology`
- prove invalid seeds produce typed topology clean-fail receipts before spatial
  binding can consume them

**Engineering decisions**
- Shape seeds are workload recipes with receipts.
- Topology seeds are owned by topology, even when spatial and kernel tests use
  them.

**Resolved decision**
- Admit topology loop seeds with `3..=64` edges and shell seeds with `4..=64`
  faces for M6.5. Larger workloads belong in later stress profiles after the
  platform rails exist.

### Phase 4: Build Geometry Binding Workloads

Phase 4 binds topology workload entities to geometry carriers through the
ordinary spatial binding path. Binding is the bridge from topology truth to
spatial meaning and must stay receipt-backed.

This phase must prevent topology identity from masquerading as geometry truth.
Every spatial workload consumed after this point needs binding evidence that
states which topology entities were bound, what geometry carriers were attached,
and why unsupported or dirty binding cases were denied. If a caller can project
or operate on topology without binding receipts, the phase has failed.

**Relevant subsystems**
- `worth-spatial` geometry binding and rebinding
- `worth-topo` topology identity and opaque handles
- Forge Query binding declaration and receipt lanes
- `worth-kernel` primitive construction consumers

**Relevant APIs**
- `GeometryBindingWorkload`
- `BoundGeometryWorkload`
- `BoundFaceGeometry`
- `BoundEdgeGeometry`
- `GeometryBindingReceiptSet`
- `UnsupportedGeometryBinding`

**Required Query posture**
- required now:
  - binding declaration receipt
  - target topology identity receipt
  - geometry carrier identity receipt
  - unsupported binding receipt for unadmitted geometry families
- support-gated:
  - rich interactive binding policy
- out:
  - topology ids standing in for geometry identity

**Warnings**
- Do not let a topology seed become a spatial workload until binding receipts
  exist.
- Do not introduce future geometry-family stubs that pretend to be admitted.
- Do not silently bind dirty topology as usable geometry.

**Test requirements**
- `geometry_binding_workload_consumes_topology_seed_receipts`
- prove bound workloads cannot be constructed without topology receipts and
  geometry binding receipts
- `unsupported_geometry_binding_is_typed_and_non_consumable`
- prove unsupported geometry families produce typed denial posture and cannot
  enter projection or operator execution as admitted bindings

**Engineering decisions**
- Binding workload types carry the bridge proof from topology to spatial.
- Unsupported families are explicit product facts, not missing branches.

**Resolved decision**
- Use `BoundPlanarFaceGeometry`, `BoundPlanarEdgeGeometry`,
  `BoundPlanarLoopGeometry`, `GeometryCarrierIdentity`, and
  `GeometryBindingReceiptSet` for the first binding vocabulary.

### Phase 5: Build Surface Support Workloads

Phase 5 classifies bound geometry by surface support. Planes are certified in
M6.5; other surface families are structurally locatable but unsupported unless
real support already exists.

This phase must separate future-shaped architecture from fake future support.
The platform should know that surface families beyond planes exist and should
be able to explain why they are not admitted, but only certified planes may
produce the local-frame and projection proofs required by this milestone.
Unsupported support is a typed product result, not an absent branch or TODO.

**Relevant subsystems**
- `worth-spatial` surface support classification
- `worth-spatial` planar local frame and precision certification
- Forge Query support matrix
- `worth-kernel` operator support checks

**Relevant APIs**
- `CertifiedSurfaceSupport`
- `CertifiedPlaneSupport`
- `UnsupportedSurfaceSupport`
- `SurfaceFamily`
- `SurfaceSupportReceipt`

**Required Query posture**
- required now:
  - surface support declaration receipt
  - support matrix row for every classified family
  - certified plane support receipt
  - typed unsupported receipt for non-admitted families
- support-gated:
  - non-planar surface certification
- out:
  - empty future receipt types or operator stubs for unsupported families

**Warnings**
- Do not mention future operators as admitted M6.5 capability.
- Do not let unsupported surface posture be absence of a row.
- Do not merge surface family, support status, and operator readiness into one
  binary flag.

**Test requirements**
- `surface_support_workload_certifies_planes_and_denies_unadmitted_families`
- prove planar support is certified and every non-admitted family has typed
  unsupported posture
- `surface_support_workload_blocks_future_family_stubs`
- prove unsupported surface support cannot be consumed as certified local
  frame, projection, or operator input

**Engineering decisions**
- Planes are the admitted support family for this milestone.
- The platform is future-shaped by classification and denial, not fake support.

**Resolved decision**
- Use `SurfaceFamily::Plane`, `SurfaceFamily::AnalyticNonPlanar`,
  `SurfaceFamily::Freeform`, `SurfaceFamily::GeneratedFeature`, and
  `SurfaceFamily::Unknown`. Only `Plane` may produce `CertifiedPlaneSupport`
  in M6.5; every other family must produce typed unsupported posture.

### Phase 6: Build Local Frame And Projection Workloads

Phase 6 turns certified plane support into local coordinate and projection
workloads. This is the shared substrate for planar overlap, winding, signed
area, segment contact, thin features, and later profile-like planar work.

This phase must turn projection into a proof-bearing stage instead of fixture
setup. A projected workload must preserve the topology entity, binding identity,
surface support, local basis, and projection-consumption evidence that produced
its coordinates. Operator-specific code should receive projected facts, not
loose point arrays it can reinterpret.

**Relevant subsystems**
- `worth-spatial` local frame certification
- `worth-spatial` certified projection
- `worth-spatial` projection-consumed facts
- Forge Query projection declaration and receipt lanes

**Relevant APIs**
- `CertifiedLocalFrameWorkload`
- `ProjectedPlanarWorkload`
- `ProjectedFace`
- `ProjectedLoop`
- `ProjectedEdgeSet`
- `ProjectionReceiptSet`

**Required Query posture**
- required now:
  - local frame declaration and receipt
  - projection declaration and receipt
  - projection-consumption receipt
  - counters for projected topology entities and local basis parts
- support-gated:
  - non-planar parameter-space projection
- out:
  - loose point arrays as operator-ready projection input

**Warnings**
- Do not recompute local frame basis in operator-specific code.
- Do not allow projection without a certified surface support receipt.
- Do not let projection hide topology-local entity identity.

**Test requirements**
- `projection_workload_preserves_topology_binding_and_plane_basis`
- prove projected faces, loops, and edges retain topology identity, binding
  identity, plane support identity, and local frame receipts
- `projection_workload_blocks_loose_point_operator_inputs`
- prove operator harnesses cannot consume raw coordinate arrays as complete
  projected workloads

**Engineering decisions**
- Projection is a workload stage, not fixture setup.
- Projected workloads carry both local coordinates and proof of their origin.

**Resolved decision**
- `ProjectionWorkload` is the executable projection stage/request.
  `ProjectionConsumedPlanarFacts` is the durable proof artifact that downstream
  operators consume. Operators may consume the latter, not raw projection
  mechanics.

### Phase 7: Build Transform Workloads

Phase 7 makes movement, rotation, reorientation, and cancellation real workload
semantics. A transform workload changes geometry when it claims to and records
the posture that later stages consume.

This phase must eliminate label-only motion as an admissible proof technique.
Transform workloads need to carry both geometric change evidence and semantic
posture evidence, including the cases where transforms intentionally converge
and the cases where orientation or order changes should perturb meaning. Later
replay, projection, and operator phases should consume the transform receipts
rather than inspecting ad hoc identity strings.

**Relevant subsystems**
- `worth-spatial` movement and rotation posture
- `worth-spatial` structural identity
- `worth-spatial` projection-consumed facts
- Forge Query transform and retained artifact lanes

**Relevant APIs**
- `TransformSequence`
- `TransformedWorkload`
- `TransformPostureReceipt`
- `TransformParityReport`
- `PlanarMotionPosture`

**Required Query posture**
- required now:
  - transform declaration receipt
  - transform posture receipt
  - changed-coordinate evidence for non-identity transforms
  - equivalence/divergence evidence for transform parity
- support-gated:
  - rich feature-regeneration transform semantics
- out:
  - motion labels standing in for geometry transforms

**Warnings**
- Do not treat identity strings as motion.
- Do not accept a rotation proof unless coordinates, basis, or posture evidence
  changed as required.
- Do not collapse equivalent transform convergence and semantic transform
  divergence into one test path.

**Test requirements**
- `transform_workload_changes_geometry_and_records_posture`
- prove translation, rotation, reorientation, and cancellation chains emit
  transform receipts and coordinate/posture evidence
- `transform_workload_rejects_identity_label_motion`
- prove a workload that changes only labels or identities cannot satisfy
  transform evidence guards

**Engineering decisions**
- Movement and rotation are product semantics, not fixture variation.
- Transform equivalence must be proven through evidence, not naming.

**Resolved decision**
- Require `16` transform steps for M6.5 acceptance cancellation workloads and
  add a hostile catalog profile with `64` steps. The acceptance path proves
  cancellation/order/replay behavior; the hostile profile adds pressure without
  turning platform bring-up into a benchmark milestone.

### Phase 8: Build Retained And Replay Workloads

Phase 8 makes retained replay consume retained Query artifacts rather than
re-running operator extraction. Replay is a workload stage with its own
evidence, not an implementation shortcut.

This phase must make replay materially different from executing the live path
again. A replay workload has to prove which retained artifact it consumed, what
checkpoint or retained basis it represents, and how its output compares to live
and projection-consumed facts. If the replay code can silently repair missing
facts by calling the extractor, the phase has not closed.

**Relevant subsystems**
- Forge Query retained artifact lanes
- `worth-spatial` retained planar facts
- `worth-spatial` projection-consumed facts
- `worth-kernel` replay certification

**Relevant APIs**
- `RetainedWorkload`
- `RetainedArtifactSet`
- `ReplayWorkload`
- `ReplayParityReport`
- retained planar fact receipts

**Required Query posture**
- required now:
  - retained artifact capture receipt
  - replay declaration and receipt
  - live-vs-retained-vs-replayed parity evidence
  - counters for retained artifact breadth and replay breadth
- support-gated:
  - branch/merge replay beyond current admitted histories
- out:
  - second extractor calls with identical inputs as replay proof

**Warnings**
- Do not call re-extraction replay.
- Do not let retained artifacts be hand-filled by tests.
- Do not let replay rebuild missing topology or projection facts locally.

**Test requirements**
- `retained_replay_workload_consumes_retained_artifacts`
- prove replay evidence is derived from retained Query artifacts and not from a
  second live extraction
- `retained_replay_workload_detects_live_retained_projection_drift`
- prove drift between live, retained, replayed, and projection-consumed views is
  localized with typed evidence

**Engineering decisions**
- Replay is a first-class workload consumer.
- Retained artifacts are evidence-bearing inputs, not cache conveniences.

**Resolved decision**
- Keep retained replay artifacts under `worth-spatial` workload APIs because
  retained planar facts are spatial authority. Kernel may expose certification
  wrappers such as `ReplayCertification`, but it must not own retained spatial
  artifact truth.

### Phase 9: Build Product User Response Layer

Phase 9 promotes the overlap-specific user outcome pattern into a reusable
Worth response surface. Operators should not invent incompatible policy and
no-options vocabularies.

This phase must define the product language for modeler-facing outcomes before
later MB suites multiply their own local response types. The response layer must
separate admitted, policy-required, unsupported, denied, predicate-uncertain,
integrity-mismatch, and no-options states, and it must make available choices
or the lack of choices readable to a human. It is not enough to expose machine
reason codes; those codes are evidence, not the response.

**Relevant subsystems**
- `worth-spatial` diagnostics and policy outcomes
- `worth-kernel` workflow composition and readiness outcomes
- future UI/DSL interaction consumers
- Forge Query ordinary outcome and diagnostic lanes

**Relevant APIs**
- `WorthUserOutcome`
- `WorthUserOutcomeKind`
- `WorthPolicyDecision`
- `WorthNoOptionsCause`
- `WorthUnsupportedCause`
- `WorthDeniedCause`
- `WorthIntegrityMismatchCause`
- `HumanReadableResponse`

**Required Query posture**
- required now:
  - ordinary outcome projection for admitted and policy-required outcomes
  - diagnostic receipt for denied and no-options outcomes
  - support posture receipt for unsupported outcomes
  - evidence digest for every human-readable response
- support-gated:
  - interactive UI/DSL resolution loops
- out:
  - test-local matrices and machine-token-only user messages

**Warnings**
- Do not offer policy choices when no safe choices exist.
- Do not return empty choices without explaining the no-options cause.
- Do not let overlap-only outcome names become the only public response shape
  if later workload families share the taxonomy.

**Test requirements**
- `worth_user_outcome_classifies_admitted_policy_unsupported_denied_uncertain_integrity_and_no_options`
- prove every shared outcome class has typed cause, evidence, and display text
  where applicable
- `worth_user_outcome_rejects_machine_token_only_messages`
- prove public human-readable responses cannot be satisfied by internal
  hyphenated reason tokens alone

**Engineering decisions**
- Product response is platform infrastructure.
- Domain-specific outcomes may wrap or specialize the shared response shape, but
  they must not fork the policy semantics.

**Resolved decision**
- Start `WorthUserOutcome` under `worth_spatial::workloads::response`. Immediate
  M6.5 users are spatial and planar. Promote later only when kernel or broader
  Worth interaction code needs cross-domain response composition.

### Phase 10: Build Evidence Ledger And Honesty Guards

Phase 10 makes proof honesty mechanical. The evidence ledger records which
platform stages actually ran, and guard APIs reject synthetic end-to-end claims.

This phase must turn "real end-to-end" from a reviewer judgment into a checked
artifact. The ledger needs to be constructed from source receipts emitted by
the earlier stages, not filled in by tests after the fact. The guard APIs should
fail in the exact ways the MB1 audit exposed: fake replay, label-only motion,
spatial-only setup, fixture arithmetic, and unsupported families pretending to
be admitted.

**Relevant subsystems**
- workload platform evidence
- `worth-topo` topology receipts
- `worth-spatial` binding, projection, transform, replay, diagnostics, and
  response receipts
- `worth-kernel` certification assertions

**Relevant APIs**
- `WorkloadEvidenceLedger`
- `assert_uses_real_topology`
- `assert_binding_is_receipt_backed`
- `assert_projection_is_receipt_backed`
- `assert_transform_changed_geometry`
- `assert_replay_consumed_retained_artifact`
- `assert_counters_are_receipt_backed`
- `assert_no_fixture_arithmetic_as_truth`
- `assert_no_synthetic_end_to_end_claim`

**Required Query posture**
- required now:
  - receipt references for every ledger row
  - structural counters per workload stage
  - guard failures with typed diagnostic causes
  - public contract proof that the ledger cannot be hand-filled as complete
- support-gated:
  - full aircraft-grade audit bundles
- out:
  - ledger structs that tests populate manually without source receipts

**Warnings**
- Do not create a ceremonial ledger with optional fields.
- Do not let a counter be asserted only against catalog/generator cardinality.
- Do not let a passing operator result imply end-to-end proof without ledger
  guards.

**Test requirements**
- `evidence_ledger_requires_source_receipts_for_every_completed_stage`
- prove completed ledger stages require real receipt references and counters
- `honesty_guards_reject_synthetic_replay_motion_and_fixture_arithmetic`
- prove fake replay, label-only motion, spatial-only fixtures, and generator
  arithmetic fail before MB closeout can consume them

**Engineering decisions**
- The ledger is a mechanical proof surface, not logging.
- Guard APIs are part of platform DX.

**Resolved decision**
- Use stable stage counter names:
  - `topology_entity_count`
  - `topology_relation_count`
  - `binding_target_count`
  - `surface_support_count`
  - `projected_entity_count`
  - `local_basis_part_count`
  - `transform_step_count`
  - `retained_artifact_count`
  - `replay_checkpoint_count`
  - `operator_input_count`
  - `operator_receipt_count`
  - `diagnostic_count`
  - `user_outcome_count`

### Phase 11: Build Operator Harness

Phase 11 lets operators run against workload bundles. The harness should make
operator-specific code consume proof-bearing workload stages rather than
rebuilding topology, projection, transform, replay, or response setup.

This phase must make operators boring consumers of platform proof. The operator
harness should declare what workload stages an operator requires, reject
incomplete or unsupported bundles before execution, and return outcomes linked
to the evidence ledger. Operator code may classify geometry; it may not become
the place that invents topology, binding, projection, replay, or user-response
setup.

**Relevant subsystems**
- `worth-kernel` operator composition
- `worth-spatial` planar operators and diagnostics
- workload platform stages
- Forge Query operator declaration and receipt lanes

**Relevant APIs**
- `WorkloadOperator`
- `OperatorRun`
- `OperatorReceiptSet`
- `OperatorOutcome`
- `CoplanarOverlapWorkloadOperator`
- `UnsupportedOperatorFamily`

**Required Query posture**
- required now:
  - operator declaration receipt
  - operator support/admission row
  - operator receipt set
  - operator outcome linked to workload evidence
- support-gated:
  - non-M6 operator families
- out:
  - operator harnesses that accept raw geometry or bypass ledger guards

**Warnings**
- Do not implement later operator families here.
- Do not let unsupported operator family be absence of a method.
- Do not let an operator own topology or binding setup.

**Test requirements**
- `operator_harness_consumes_projected_retained_transformed_workloads`
- prove the overlap operator consumes platform artifacts rather than rebuilding
  fixture setup internally
- `operator_harness_denies_unsupported_family_without_stub_execution`
- prove unsupported operator families fail with typed support posture and no
  fake receipt

**Engineering decisions**
- Operators are consumers of workload proof.
- The first concrete operator consumer is coplanar overlap.

**Resolved decision**
- Put `WorkloadOperator` in `worth-kernel` because it orchestrates cross-crate
  workload execution. Keep concrete spatial operators, including
  `CoplanarOverlapWorkloadOperator`, under `worth-spatial`.

### Phase 12: Build Canonical Workload Catalog

Phase 12 creates named workload recipes for common and hostile shapes. Catalog
entries are real workload builders with evidence, not static fixtures.

This phase must make the real path ergonomic. A catalog recipe should give
future tests a single obvious way to request a cube, tetrahedron, overlap storm,
thin feature, dirty loop, high-valence neighborhood, open sheet, transform
cycle, or retained cancellation chain, while still routing through the same
topology, binding, surface support, projection, transform, replay, and evidence
stages as any manually assembled workload. The catalog is DX, but it is also an
enforcement surface.

**Relevant subsystems**
- workload topology seeds
- workload geometry binding and surface support
- workload transforms and retained replay
- workload evidence ledger

**Relevant APIs**
- `WorkloadCatalog::cube`
- `WorkloadCatalog::tetrahedron`
- `WorkloadCatalog::single_face_loop`
- `WorkloadCatalog::coplanar_overlap_storm`
- `WorkloadCatalog::thin_feature_wall`
- `WorkloadCatalog::dirty_self_intersecting_loop`
- `WorkloadCatalog::high_valence_vertex`
- `WorkloadCatalog::open_sheet`
- `WorkloadCatalog::transform_cycle`
- `WorkloadCatalog::retained_cancellation_chain`

**Required Query posture**
- required now:
  - catalog recipe declaration
  - emitted ledger for each recipe
  - support posture for admitted and unsupported recipe branches
  - counters proving topology, binding, projection, transform, and replay
    breadth where relevant
- support-gated:
  - product UI selection of workload recipes
- out:
  - static fixture structs with no receipt path

**Warnings**
- Do not let catalog entries be generator arithmetic with nicer names.
- Do not expose recipe names that imply unsupported operators are admitted.
- Do not hide dirty/open posture as a test-only flag.

**Test requirements**
- `workload_catalog_recipes_emit_complete_evidence_ledgers`
- prove cube, tetrahedron, loop, overlap storm, thin-feature, dirty, high
  valence, open, transform, and retained recipes emit appropriate evidence
- `workload_catalog_blocks_static_fixture_substitution`
- prove catalog recipes cannot be replaced by static coordinate fixtures in
  operator closeout suites

**Engineering decisions**
- Catalog recipes are part of the platform DX.
- The catalog should make ordinary real setup boring and repeatable.

**Resolved decision**
- Expose `WorkloadCatalog` through a public kernel facade so future tests and
  certification have one obvious real entry point. Keep recipe internals in the
  crate-specific workload modules that own their authority.

### Phase 13: Refactor MB-M6-1 Coplanar Overlap Storm Onto The Platform

Phase 13 moves MB-M6-1 onto the platform. The overlap storm is the first
consumer that proves the rails work; it is not allowed to define private rails
of its own.

This phase must turn the current overlap proof from a spatial production-path
exercise into a real topology-to-operator workload. The storm may keep hostile
generated scale as an input source, but topology, binding, plane support,
projection, transforms, retained replay, operator execution, user response, and
evidence must all be platform-owned. The old synthetic shortcuts either become
unit-only support or disappear.

**Outcome matrix**
- admitted certified overlap
- policy-required overlap before imprint
- no-options dirty input
- no-options unsupported/open input
- no-options denied movement or rotation
- no-options predicate uncertainty or predicate authority failure
- integrity failure when topology/binding/projection evidence is missing

**Missing production features to add**
- topology-backed `coplanar_overlap_storm` workload recipe
- real retained replay driver for overlap receipts
- transform receipts proving geometry changed for overlap variants
- shared `WorthUserOutcome` wrapping or replacing overlap-only outcome types
- evidence guard proving overlap counters are receipt-backed, not generator
  arithmetic

**Relevant subsystems**
- workload catalog
- operator harness
- evidence ledger and honesty guards
- `worth-spatial` coplanar overlap contracts
- `worth-kernel` certification

**Relevant APIs**
- `WorkloadCatalog::coplanar_overlap_storm`
- `CoplanarOverlapWorkloadOperator`
- `WorkloadEvidenceLedger`
- `WorthUserOutcome`
- `CoplanarOverlapContractExtractor`

**Required Query posture**
- required now:
  - topology-backed overlap storm workload
  - binding, plane support, projection, transform, retained replay, operator,
    diagnostics, response, and evidence receipts
  - proof that replay consumed retained artifacts
  - proof that equivalent transforms changed geometry and converged
  - proof that fixture arithmetic did not satisfy truth claims
- support-gated:
  - M7 boolean imprint or split/classify decisions
- out:
  - procedural-only rectangles, fixed synthetic world anchors, label-only
    motion, or double-extraction replay as MB closeout proof

**Warnings**
- Do not make MB1 green by weakening the evidence guards.
- Do not leave branch-matrix outcome tests disconnected from the storm workload.
- Do not let the old proof fixture remain named as end-to-end support.

**Test requirements**
- `mb_m6_1_coplanar_overlap_storm_end_to_end_receipts`
- prove the overlap storm starts from real topology-backed workload recipes and
  emits a complete evidence ledger
- `mb_m6_1_user_outcome_matrix_branches_every_stop`
- prove every matrix row is production-owned and human-readable where public
- `mb_m6_1_equivalent_motion_subset_converges_without_full_storm_replay`
- prove movement/rotation variants actually transform geometry and consume
  retained artifacts for replay
- `mb_m6_1_fixture_arithmetic_cannot_satisfy_storm_truth`
- prove generator cardinality and fixture counters cannot replace receipt-backed
  topology, geometry, projection, transform, and replay evidence

**Engineering decisions**
- MB1 is the first closeout consumer of the workload platform.
- Generated hostile regions may remain only as workload input, not proof.

**Resolved decision**
- Keep the old storm generator only as `CoplanarStormRegionGenerator`, a hostile
  region-parameter expansion strategy. It may produce region parameters, but
  every region must be materialized through topology seeds, binding, projection,
  transforms, retained replay, and ledger evidence before MB proof can consume
  it.

### Phase 14: Add MB-M6-2 High-Valence Singularity Workload

Phase 14 gives MB-M6-2 its own platform phase. High-valence pressure must start
from real topology neighborhoods and then bind/project spatial evidence; it
cannot be modeled as already-projected points around a fake center.

This phase must prove that topology-local valence, spatial predicate authority,
and local rebuild/rebinding posture are all present in the workload ledger.
The user-facing result must explain whether the singularity is admitted,
policy-required, unsupported, topology-blocked, or predicate-uncertain.

**Outcome matrix**
- admitted high-valence planar neighborhood
- policy-required ambiguous singularity
- no-options predicate uncertainty
- no-options topology contract failure
- no-options unsupported valence posture
- no-options movement/rebuild incompatibility
- integrity mismatch between topology neighborhood and projected facts

**Missing production features to add**
- `TopologySeed::high_valence_vertex` backed by real topology receipts
- high-valence workload catalog recipe with topology-local neighborhood receipt
- singularity-capable `WorthUserOutcome` detail payload
- local rebuild/rebinding evidence row for high-valence neighborhoods
- diagnostics naming the singular vertex/neighborhood rather than only a digest

**Relevant subsystems**
- topology workload seeds
- geometry binding workloads
- local frame and projection workloads
- local rebuild/rebinding parity
- user response and diagnostics

**Relevant APIs**
- `WorkloadCatalog::high_valence_vertex`
- `TopologySeed::high_valence_vertex`
- `ProjectedPlanarWorkload`
- `WorthUserOutcome`
- `PlanarLocalRebuildParity`

**Required Query posture**
- required now:
  - topology neighborhood declaration and receipt
  - binding and projection receipts for every incident face/edge
  - predicate receipt or predicate uncertainty envelope
  - diagnostic receipt naming the singularity source
- support-gated:
  - topology repair or valence healing
- out:
  - low-valence fake fixtures relabeled as singularity proof

**Warnings**
- Do not bypass topology by building a coordinate fan.
- Do not conflate topology failure and predicate uncertainty.
- Do not let rebinding continuity hide an unsupported neighborhood.

**Test requirements**
- `mb_m6_2_high_valence_planar_singularity_contract`
- prove the high-valence workload enters through topology and projection
- `mb_m6_2_singularity_no_options_matrix_names_exact_blocker`
- prove every denied matrix row names the exact blocker
- `mb_m6_2_rebuild_movement_break_denies_before_correspondence`
- prove rebuild/motion incompatibility fails before fake continuity

**Engineering decisions**
- High-valence MB proof owns a topology-neighborhood workload recipe.
- Singularity user outcomes are production product language, not test labels.

**Resolved decision**
- Admit high-valence platform proof for valence `3..=16`, and add a hostile
  catalog profile at valence `32`. Higher valence remains typed unsupported
  until a later widening phase adds explicit support and cost proof.

### Phase 15: Add MB-M6-3 Thin-Feature Scale-Separation Workload

Phase 15 gives MB-M6-3 its own platform phase. Thin features must prove local
scale, world magnitude, projection basis, precision escalation, and transform
posture through receipts.

This phase must stop micro-feature tests from becoming loose coordinate stress
cases. The workload needs topology-bound geometry, certified plane support,
local frame receipts, precision counters, and user responses that distinguish
local scale failure from global-coordinate magnitude.

**Outcome matrix**
- admitted scale-separated thin feature
- policy-required ambiguous micro-feature
- no-options precision basis failure
- no-options predicate uncertainty
- no-options unsupported tiny-rotation posture
- integrity mismatch between local frame and projection-consumed facts

**Missing production features to add**
- `WorkloadCatalog::thin_feature_wall`
- topology-bound thin-feature geometry recipe
- precision/local-frame evidence rows in `WorkloadEvidenceLedger`
- thin-feature `WorthUserOutcome` detail payload
- guard proving projection consumption preserved the certified local basis

**Relevant subsystems**
- topology and binding workloads
- certified plane support
- local frame and projection workloads
- precision certification
- transform and response layers

**Relevant APIs**
- `WorkloadCatalog::thin_feature_wall`
- `CertifiedLocalFrameWorkload`
- `ProjectionReceiptSet`
- `WorthUserOutcome`
- precision escalation counters

**Required Query posture**
- required now:
  - precision declaration and receipt
  - local-frame receipt with local feature scale and world magnitude
  - projection-consumption receipt
  - transform receipt for tiny rotation pressure
- support-gated:
  - M7 micro-feature boolean execution
- out:
  - global epsilon fixtures or hidden snapping

**Warnings**
- Do not assert elapsed time as precision proof.
- Do not test a micro-feature disconnected from topology.
- Do not recompute local basis inside assertions.

**Test requirements**
- `mb_m6_3_thin_feature_scale_separation_contract`
- prove thin-feature scale separation through topology-bound local frame facts
- `mb_m6_3_micro_feature_outcome_matrix_is_production_owned`
- prove every outcome branch is production response data
- `mb_m6_3_projection_consumption_preserves_local_basis`
- prove projection-consumed facts retain the certified local basis

**Engineering decisions**
- Thin-feature proof is a local-frame workload, not a coordinate fixture.
- Precision counters are part of evidence, not debug output.

**Resolved decision**
- Require `12` thin features across at least `3` local scales, including one
  large-world coordinate case and one tiny-rotation case. This proves local
  scale separation rather than a single clever micro-feature.

### Phase 16: Add MB-M6-4 Retained History Cancellation Workload

Phase 16 gives MB-M6-4 its own platform phase. Cancellation must be retained
history, not final coordinate equality and not repeated live extraction.

This phase must create a retained cancellation-chain recipe that records every
transform step, retained artifact, replay checkpoint, projection-consumed fact,
and near-graze trigger. User responses must name the retained step where the
chain becomes policy-required, uncertain, mismatched, or denied.

**Outcome matrix**
- admitted exact cancellation chain
- policy-required near-graze step
- no-options predicate uncertainty
- no-options retained replay mismatch
- no-options transform invalidation
- integrity mismatch between retained checkpoint and projection-consumed facts

**Missing production features to add**
- `WorkloadCatalog::retained_cancellation_chain`
- retained artifact capture for every cancellation checkpoint
- replay comparison that cannot call live extraction
- retained-history `WorthUserOutcome` detail payload
- evidence guard localizing the exact trigger step

**Relevant subsystems**
- transform workloads
- retained and replay workloads
- projection workloads
- structural identity
- diagnostics and response layers

**Relevant APIs**
- `RetainedWorkload`
- `ReplayWorkload`
- `TransformSequence`
- `ReplayParityReport`
- `WorthUserOutcome`

**Required Query posture**
- required now:
  - retained artifact receipt per checkpoint
  - replay receipt per sampled checkpoint
  - transform receipt per movement/rotation step
  - diagnostic receipt for near-graze or mismatch
- support-gated:
  - M7 boolean chain execution
- out:
  - final-coordinate-only cancellation proof

**Warnings**
- Do not let replay re-extract live data.
- Do not hide near-graze at final summary.
- Do not use coordinate equality as structural identity.

**Test requirements**
- `mb_m6_4_retained_planar_history_cancellation_chain`
- prove cancellation through retained artifacts and transform receipts
- `mb_m6_4_retained_outcome_matrix_branches_each_history_stop`
- prove every retained stop has production outcome coverage
- `mb_m6_4_projection_consumed_facts_match_retained_checkpoints`
- prove projection-consumed facts match retained basis per checkpoint

**Engineering decisions**
- Retained cancellation is a replay workload family.
- Trigger localization is required evidence.

**Resolved decision**
- Require `32` retained transform checkpoints for acceptance and a hostile
  catalog profile with `128`. Replay every fourth checkpoint plus the
  trigger-local checkpoints so the proof covers breadth and exact localization.

### Phase 17: Add MB-M6-5 Dirty Planar Input Clean-Fail Workload

Phase 17 gives MB-M6-5 its own platform phase. Dirty input must be built as
real topology and binding pressure, not as a spatial fixture with a dirty label.

This phase must prove that dirty loops, non-manifold wires, orientation
inconsistency, and transform pressure fail through clean-fail posture before
projection, overlap, recovery, or boolean-readiness can treat them as admitted.
Human-readable responses must say why no options exist when repair is not an
M6.5 capability.

**Outcome matrix**
- no-options dirty self-intersection
- no-options non-manifold wire where unsupported
- no-options thin wall or invalid local basis
- no-options orientation inconsistency
- policy-required dirty class only if a safe production policy exists
- integrity mismatch when stable topology IDs hide dirty geometry
- transform-preserved dirty failure class

**Missing production features to add**
- `TopologySeed::self_intersecting_loop`
- `TopologySeed::non_manifold_wire`
- dirty input workload catalog recipe
- dirty-input `WorthUserOutcome` detail payload
- clean-fail evidence guard proving dirty input never upgrades through recovery

**Relevant subsystems**
- topology workload seeds
- clean-fail boundary
- recovery posture
- transform workloads
- diagnostics and response layers

**Relevant APIs**
- `WorkloadCatalog::dirty_self_intersecting_loop`
- `TopologySeed::self_intersecting_loop`
- `TopologySeed::non_manifold_wire`
- `WorthUserOutcome`
- `PlanarCleanFailBoundary`

**Required Query posture**
- required now:
  - dirty topology declaration and receipt
  - clean-fail receipt naming first blocker
  - recovery posture receipt that cannot synthesize admitted truth
  - transform receipt preserving or exposing dirty failure
- support-gated:
  - repair or healing operators
- out:
  - hidden repair, topology-only success, or clean-fail after operator work

**Warnings**
- Do not reuse one dirty fixture under many labels.
- Do not let stable topology identity hide dirty geometry.
- Do not offer user choices when no safe policy exists.

**Test requirements**
- `mb_m6_5_dirty_planar_input_clean_fail_localization`
- prove dirty classes fail cleanly through topology-backed workloads
- `mb_m6_5_dirty_outcome_matrix_branches_each_dirty_kind`
- prove every dirty kind has production outcome and readable cause
- `mb_m6_5_dirty_transform_pressure_preserves_failure_class`
- prove transforms cannot hide or repair dirty input

**Engineering decisions**
- Dirty input is a clean-fail workload family.
- Recovery may explain next steps; it cannot make dirty truth admitted.

**Resolved decision**
- Default every dirty class to no-options in M6.5 unless a safe production
  policy decision is explicitly named. Policy-required posture may apply only
  to ambiguous orientation intent where both choices are topologically legal.
  Self-intersection, unsupported non-manifold wires, and invalid local basis
  failures are no-options.

### Phase 18: Add MB-M6-6 Unbounded And Open Planar Posture Workload

Phase 18 gives MB-M6-6 its own platform phase. Open and unbounded cases must be
classified honestly without bounded conversion or surrogate closed geometry.

This phase must prove the platform can build open topology workloads, bind
their spatial posture, classify surface support, and explain whether bounded
planar operators are admitted, unsupported, policy-required, or predicate
uncertain. A half-space or open sheet cannot be clipped to make the test pass.

**Outcome matrix**
- admitted open/unbounded class if explicitly supported
- unsupported open sheet or half-space class
- policy-required half-space interpretation
- no-options predicate uncertainty
- no-options bounded-operator incompatibility
- integrity mismatch when finite surrogate geometry replaces open truth
- transform divergence for semantic inversions

**Missing production features to add**
- `TopologySeed::open_sheet`
- `TopologySeed::open_wire`
- open/unbounded workload catalog recipe
- open-posture `WorthUserOutcome` detail payload
- guard proving no bounded surrogate or clipping path satisfied the workload

**Relevant subsystems**
- topology workload seeds
- geometry binding workloads
- surface support classification
- clean-fail and recovery posture
- transform workloads

**Relevant APIs**
- `WorkloadCatalog::open_sheet`
- `TopologySeed::open_sheet`
- `TopologySeed::open_wire`
- `UnsupportedSurfaceSupport`
- `WorthUserOutcome`

**Required Query posture**
- required now:
  - open topology receipt
  - support/admission receipt for open or unbounded class
  - clean-fail or admitted posture receipt
  - diagnostic receipt naming bounded-operator incompatibility
- support-gated:
  - bounded conversion and later open-sheet handling
- out:
  - clipping, inferred closure, or finite surrogate geometry

**Warnings**
- Do not silently convert open domains to bounded domains.
- Do not leave unsupported posture as missing catalog entry.
- Do not canonicalize orientation-changing transforms as equivalent.

**Test requirements**
- `mb_m6_6_unbounded_half_space_planar_posture`
- prove open/unbounded posture is classified through real workload evidence
- `mb_m6_6_unbounded_outcome_matrix_explains_no_options`
- prove unsupported and no-options rows are readable and typed
- `mb_m6_6_half_space_transform_canonicalization_and_divergence`
- prove equivalent and divergent transforms are distinguished

**Engineering decisions**
- Open/unbounded posture is a workload support result.
- Surrogate bounded geometry is forbidden as proof.

**Resolved decision**
- Do not admit open or unbounded classes as operator inputs in M6.5. Catalog
  construction and support classification are allowed, but open/unbounded
  workloads must produce typed support posture or clean-fail outcomes until a
  later milestone explicitly admits them.

### Phase 19: Add MB-M6-7 Projection-Consumed Fact Parity Workload

Phase 19 gives MB-M6-7 its own platform phase. Projection parity must compare
live, retained, replayed, recovered, transformed, and projection-consumed facts
from the same workload evidence basis.

This phase must make projection-consumption mismatches product-visible. It is
not enough to compare live and projected coordinates; the workload must expose
which lane broke and whether the correct result is admitted parity, denied
parity, policy-required posture, or no-options integrity failure.

**Outcome matrix**
- admitted parity across all lanes
- denied parity preserved across all lanes
- no-options live/projection mismatch
- no-options retained/replay mismatch
- no-options recovery mismatch
- no-options transform parity mismatch
- no-options local rebuild mismatch
- policy-required parity branch only if a safe production policy exists

**Missing production features to add**
- cross-lane parity recipe over live, retained, replayed, recovered, projected,
  transformed, and rebuild facts
- parity `WorthUserOutcome` detail payload naming the failed lane
- evidence ledger lane IDs for every parity surface
- guard preventing denied paths from upgrading through projection or recovery

**Relevant subsystems**
- projection workloads
- retained and replay workloads
- recovery posture
- transform workloads
- local rebuild/rebinding parity
- evidence ledger

**Relevant APIs**
- `ProjectedPlanarWorkload`
- `RetainedWorkload`
- `ReplayWorkload`
- `ReplayParityReport`
- `WorthUserOutcome`
- `WorkloadEvidenceLedger`

**Required Query posture**
- required now:
  - live, retained, replayed, recovered, transformed, and projected receipts
  - parity declaration and receipt
  - diagnostic receipt naming the failed lane
  - counters for lane breadth
- support-gated:
  - M7 boolean result materialization
- out:
  - projection success upgrading a denied path

**Warnings**
- Do not compare only live and projected facts.
- Do not let recovery synthesize missing projection truth.
- Do not accept parity based on a helper-rebuilt basis.

**Test requirements**
- `mb_m6_7_projection_consumed_planar_fact_parity`
- prove admitted parity across every workload lane
- `mb_m6_7_denied_paths_remain_denied_across_all_views`
- prove denied workloads cannot upgrade through projection or recovery
- `mb_m6_7_parity_outcome_matrix_localizes_each_mismatch_surface`
- prove mismatch outcomes name the exact lane

**Engineering decisions**
- Projection parity is a platform-wide workload property.
- Integrity mismatches are no-options unless policy is real and safe.

**Resolved decision**
- Use four representative parity workloads for acceptance:
  - clean cube planar face set
  - coplanar overlap storm subset
  - thin-feature wall
  - retained cancellation chain
  Each must run live, projected, retained, replayed, transformed, recovered, and
  local-rebuild lanes where applicable.

### Phase 20: Add MB-M6-8 Boolean-Readiness Final-Boss Workload

Phase 20 gives MB-M6-8 its own platform phase. The final boss must compose the
platform rails and stop at boolean-readiness, not sneak into M7 execution.

This phase must prove the platform can assemble a complete boolean-readiness
workload from topology, binding, surface support, projection, transform,
retained replay, parity, diagnostics, and user-response evidence. The output is
either a complete readiness bundle or a typed blocker with exact human-readable
cause and evidence lineage.

**Outcome matrix**
- admitted complete boolean-readiness bundle
- policy-required final-boss branch
- no-options typed clean failure
- unsupported workload family
- no-options predicate uncertainty
- no-options projection/parity mismatch
- no-options recovery or replay mismatch
- no-options orientation-flip localization
- integrity mismatch when kernel summary substitutes for receipts

**Missing production features to add**
- platform-backed boolean-readiness workload recipe
- final-boss `WorthUserOutcome` detail payload saying whether M7 may proceed
- readiness evidence ledger that requires every M6.5 stage
- guard preventing kernel summaries from replacing spatial/topology receipts
- support matrix row proving unsupported families cannot enter readiness

**Relevant subsystems**
- all workload platform stages
- M6 boolean-readiness contract bundles
- `worth-kernel` certification
- diagnostics and response layers

**Relevant APIs**
- `PlanarBooleanReadinessBundle`
- `WorkloadEvidenceLedger`
- `OperatorRun`
- `WorthUserOutcome`
- support matrix surfaces

**Required Query posture**
- required now:
  - complete workload evidence ledger
  - readiness declaration and receipt
  - typed clean-fail or unsupported receipt for blockers
  - diagnostic receipt for exact blocker localization
- support-gated:
  - M7 split/classify/assemble
- out:
  - any boolean result or kernel-only summary as readiness proof

**Warnings**
- Do not execute M7 boolean work.
- Do not accept a partial ledger as readiness.
- Do not hide unsupported sub-workloads behind final success.

**Test requirements**
- `mb_m6_8_boolean_readiness_final_boss`
- prove output is complete readiness or exact typed blocker
- `mb_m6_8_final_boss_outcome_matrix_is_production_owned`
- prove final-boss matrix uses production readiness and response surfaces
- `mb_m6_8_no_kernel_summary_can_substitute_for_readiness_receipts`
- prove kernel summaries cannot replace platform evidence

**Engineering decisions**
- MB-M6-8 is the final platform consumer before M7.
- Readiness is an evidence-complete bundle, not a workflow summary.

**Resolved decision**
- Name the platform input/execution shape `PlanarBooleanReadinessWorkload` and
  keep `PlanarBooleanReadinessBundle` as the M7-consumable result artifact.

### Phase 21: Fence Legacy Synthetic Fixture Paths

Phase 21 prevents the old fake-friendly paths from remaining available as
future precedent after every MB phase has an explicit platform target.

This phase must make the new platform the only route for MB closeout authority.
Legacy fixtures that still serve narrow unit tests can remain with honest names
and narrow visibility, but any helper that looks end-to-end while bypassing
topology, binding, projection, transform, retained replay, or evidence guards
must be fenced, renamed, compile-failed, or deleted.

**Outcome matrix**
- admitted unit-only fixture
- admitted workload-platform recipe
- no-options synthetic end-to-end claim
- no-options hand-filled ledger
- no-options label-only transform helper
- no-options re-extraction replay helper
- no-options static coordinate fixture claiming MB proof

**Missing production features to add**
- `LegacyFixtureClassification`
- `SyntheticEndToEndBlocked`
- MB registration gate requiring workload platform evidence
- compile-fail/public-contract fixtures for fake end-to-end paths
- migration records for renamed or deleted unit fixtures

**Relevant subsystems**
- workload evidence guards
- legacy planar proof fixtures
- MB-M6 closeout suites
- public API and compile-fail certification

**Relevant APIs**
- `LegacyFixtureClassification`
- `SyntheticEndToEndBlocked`
- `WorkloadEvidenceLedger`
- MB-M6 suite registration
- public facade compile-fail fixtures

**Required Query posture**
- required now:
  - compile-fail or public-contract proof that synthetic MB harnesses cannot
    claim end-to-end evidence
  - suite registration proof that MB tests consume workload platform artifacts
  - diagnostic proof for blocked legacy fixture paths
- support-gated:
  - none for synthetic MB claims
- out:
  - deleting useful unit fixtures that still serve narrow local proof

**Warnings**
- Do not delete useful unit support just because it is not end-to-end.
- Do not leave any helper with a name that implies MB or end-to-end authority
  if it bypasses topology, binding, projection, replay, or transforms.
- Do not let later MB tests fork new private setup worlds.

**Test requirements**
- `legacy_synthetic_fixture_paths_cannot_register_as_metaboss_closeout`
- prove spatial-only fixtures, label-only transforms, hand-filled ledgers, and
  re-extraction replay cannot register as MB closeout proof
- `all_metaboss_tests_have_platform_evidence_targets`
- prove MB-M6-1 through MB-M6-8 each has a workload platform phase and
  registration target

**Engineering decisions**
- Unit fixtures may stay unit-scoped; end-to-end fixture claims must die.
- Later MB tests are end-phase platform consumers by design.

**Resolved decision**
- Move surviving fixtures under explicit unit-support locations such as
  `certification/unit_fixtures` or `certification/local_proof_fixtures`. Use
  names like `PlanarOverlapUnitFixture`, `ProjectedPointUnitFixture`, and
  `SegmentContactUnitFixture`. Forbid names that imply MB or end-to-end
  authority, including `metaboss_fixture`, `end_to_end_fixture`,
  `storm_fixture`, and `real_world_fixture`.


## Must Ship

- seed and fixture inventory with elevate/wrap/delete/unit-only decisions
- workload vocabulary preserving topology, binding, surface support, projection,
  transform, retained replay, operator, diagnostics, response, and evidence
  boundaries
- topology workload seeds for cube, tetrahedron, loops, shells, open classes,
  high-valence neighborhoods, dirty loops, and non-manifold wires
- geometry binding workloads with receipt-backed topology-to-spatial bridge
- certified plane surface support and typed unsupported posture for other
  surface families
- local frame and projection workloads with projection-consumed facts
- transform workloads for translation, rotation, reorientation, and
  cancellation chains
- retained and replay workloads that consume retained Query artifacts
- shared Worth user-response layer for admitted, policy-required, unsupported,
  denied, predicate-uncertain, integrity-mismatch, and no-options outcomes
- evidence ledger and honesty guards rejecting synthetic end-to-end claims
- operator harness with coplanar overlap as first concrete consumer
- canonical workload catalog with receipt-backed recipes
- MB-M6-1 through MB-M6-8 each represented by its own workload-platform phase
  with explicit outcome matrix and missing production features
- MB-M6-1 refactored onto the workload platform as the first executed consumer
- MB-M6-2 through MB-M6-8 blocked from closeout until their named platform
  workloads, production response matrices, and missing production features are
  implemented
- legacy synthetic fixture fence and MB registration proof

## Must Preserve

- `worth-topo` as topology truth and topology seed authority
- `worth-spatial` as geometry binding, surface support, projection, transform,
  retained spatial fact, diagnostics, and response authority
- `worth-kernel` as operator composition and certification authority
- Forge Query as the ordinary runtime for declaration, receipts, envelopes,
  retained artifacts, projection consumption, recovery, inspection, support,
  and ordinary outcomes
- M6 exact planar contracts and boolean-readiness semantics
- unit fixtures that remain honestly named and locally scoped
- unsupported future families as typed unsupported posture, not fake support

## Acceptance Evidence

Milestone 6.5 is accepted only when all of the following evidence exists:

- `cargo check -p worth-topo -p worth-spatial -p worth-kernel`
- public API contract tests for workload vocabulary, topology seeds, geometry
  binding workloads, surface support workloads, projection workloads,
  transform workloads, retained replay workloads, user outcomes, evidence
  ledger, operator harness, and catalog recipes
- UI/compile-fail or public-contract tests proving synthetic end-to-end paths
  cannot register as MB proof
- focused workload catalog tests proving cube, tetrahedron, loop, shell, open,
  dirty, high-valence, transform, retained, and overlap storm recipes emit
  receipt-backed evidence
- `MB-M6-1` tests passing on the workload platform, with no ignored tests and
  no synthetic-only proof
- MB-M6-2 through MB-M6-8 each has an executed platform-target proof row for
  its workload recipe, outcome matrix, and missing-production-feature blockers;
  a suite may remain non-closeout only if its missing production features are
  typed, registered, and blocked from being counted as success
- registration proof that MB-M6-1 through MB-M6-8 require workload platform
  evidence before they can become closeout suites

## Sequencing Notes

- M6.5 belongs after M6 exact planar contracts because it consumes those
  contracts as operator-ready workload facts.
- M6.5 belongs before M7 because boolean split/classify/assemble must not begin
  on a test substrate that can still fake topology, replay, transforms, or
  user-response evidence.
- Do not implement M7 boolean execution in M6.5.
- Do not implement future surface or feature operators in M6.5. Classify
  unsupported families honestly and leave real implementation to their later
  milestones.
- MB-M6 tests are end-phase consumers. Each MB-M6 suite owns one end phase with
  an explicit matrix and production-feature gap list; do not collapse them into
  a generic registration bucket.

## Required Self-Check

- Does the milestone solve a real structural problem or just package work
  cosmetically? Yes: it replaces private synthetic setup worlds with a reusable
  proof-bearing workload platform.
- Is the adversarial constraint precise and load-bearing? Yes: the milestone
  exists to prevent spatial-only fixtures, fake transforms, fake replay,
  fixture arithmetic, and hand-filled evidence from claiming end-to-end proof.
- Does the roadmap justify this milestone now? Yes: M6 exact planar contracts
  and M7 booleans need a real workload substrate between them.
- Does the spec preserve crate authority boundaries? Yes: topology seeds stay
  topo-owned, spatial owns geometry and planar facts, kernel composes
  operators, and Query remains runtime authority.
- Are the phases carrying most of the real design information? Yes.
- Is each phase centered on one conceptual detail or boundary? Yes: inventory,
  vocabulary, topology, binding, surface support, projection, transforms,
  replay, response, evidence, operator harness, catalog, MB-M6-1 through
  MB-M6-8 workload consumers, and legacy fence.
- Does each phase contain at least 2 adversarial tests by default? Yes.
- Could a competent engineer map this spec into honest types, modules, and
  tests? Yes, with placement questions left explicit.
- Does the milestone belong in this roadmap sequence, or is it out of order?
  It belongs between M6 and M7 as the operational bridge from exact planar
  facts to real hostile and boolean workloads.
