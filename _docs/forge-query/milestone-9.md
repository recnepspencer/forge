# Milestone 9 Engineering Spec: Policy-Aware Narrowing, Tenant Scope, And Delivery Contracts

> **Status:** Draft engineering spec
>
> **Roadmap parent:** [forge_query_roadmap.md](./forge_query_roadmap.md)
>
> **Vision parent:** [forge_query_vision.md](./forge_query_vision.md)
>
> **Prior milestone:** [milestone-8.md](./milestone-8.md)
>
> **Adjacent milestones:** [milestone-7.md](./milestone-7.md) remains in
> progress and still supplies the identity-evolution and correspondence
> surfaces that policy/tenant boundaries must preserve without flattening.
> [milestone-8.md](./milestone-8.md) is complete and remains the authority for
> canonical composition, saved-query freeze, and planner-visible view-shape
> semantics that Milestone 9 must govern rather than bypass.
>
> **Prior closeout:** [milestone-8-closeout.md](./milestone-8-closeout.md)
>
> **Test requirements:** [test-requirements.md](./test-requirements.md)
>
> **Primary architectural driver:** make policy masking, tenant truth/schema
> basis resolution, relationship-proof denial, and delivery-shape metadata
> first-class query-owned artifacts so composed, saved, live, and historical
> query execution can fail closed before truth is read rather than relying on
> post-read redaction or host-local authorization logic
>
> **Companion docs:**
> - [MENTALITY.md](../coding_guidelines/MENTALITY.md)
> - [arch_laws.md](../coding_guidelines/arch_laws.md)
> - [perf_laws.md](../coding_guidelines/perf_laws.md)
> - [domain_laws.md](../coding_guidelines/domain_laws.md)
> - [forge_query_vision.md](./forge_query_vision.md)
> - [forge_query_roadmap.md](./forge_query_roadmap.md)
> - [test-requirements.md](./test-requirements.md)
> - [milestone-7.md](./milestone-7.md)
> - [milestone-8.md](./milestone-8.md)
> - [milestone-8-closeout.md](./milestone-8-closeout.md)

## Goal

Make policy masking, tenant truth/schema basis resolution, relationship-proof
admission, and delivery-shape metadata structural query concerns so the same
canonical query meaning can be narrowed, denied, and delivered consistently
across one-shot, live, historical, composed, and saved-query execution without
ever reading masked or unauthorized truth first.

## Why This Milestone Exists

Milestone 8 froze canonical composition, saved-query freeze, and planner-visible
view semantics. Milestone 7 is freezing identity-evolution semantics so
branch/history/lineage meaning stays explicit instead of host-interpreted.

Those milestones make policy and tenant work harder, not easier, because
`forge-query` now has more places where truth could be widened dishonestly:

- a scope-composed query could expand legally under one tenant schema and
  illegally under another
- a saved-query artifact could look reusable even though tenant schema drift or
  policy basis changed its meaning
- a view-shaped live query could preserve projection meaning in one-shot mode
  but over-read in live maintenance and redact later
- a relationship-proof query could be expressed as a host callback or server
  middleware check instead of a canonical query artifact
- branch access and tenant branch narrowing could drift between one-shot,
  live, and historical paths if they are not sealed into query admission

That gap is now load-bearing.

Without Milestone 9:

- aspect masking remains a redaction story instead of a read-narrowing story
- multi-tenant branch resolution stays ambient and invisible to query
  certification
- tenant-specific schema evolution remains a host concern rather than a query
  validation basis
- relationship proofs devolve into host-local authorization callbacks instead
  of typed query semantics
- delivery metadata can drift from the actually masked/projected query meaning
  that callers saw
- later durable artifact work in Milestone 11 would inherit policy-unsafe
  semantics rather than extending one explicit artifact model

Milestone 9 therefore exists to freeze:

- that policy masking happens before execution and plan lowering, not after
  result materialization
- that tenant scope narrows both truth basis and schema basis explicitly
- that relationship-proof legality is query-authored, typed, and denial-aware
- that delivery-shape metadata is derived from the masked/projected query result
  rather than from unmasked internal truth
- that one-shot, live, historical, scope-composed, and saved-query execution
  all consume the same policy and tenant basis artifacts
- that denied policy, branch, tenant, and relationship-proof lanes fail typed
  and early rather than becoming partial reads with later redaction

## Governing Summaries

- `MENTALITY.md`: the hard problem is not "apply auth to queries." It is making
  unauthorized truth unobservable even under live maintenance, historical
  replay, composed query reuse, and tenant schema drift. The milestone must
  solve fail-closed narrowing before execution.
- `arch_laws.md`: Laws 2, 4, 5, 7, 15, 17, 21, 27, 30, 33, 40, and 41 dominate
  this milestone. Query-owned policy and tenant admission must be explicit,
  planner-owned, proof-bearing, and separate from execution and delivery.
- `perf_laws.md`: policy and tenant support are only honest if masked
  projection width, tenant basis resolution breadth, schema-variation branch
  count, relationship-proof denial count, and delivery-shape derivation cost are
  mechanically visible. Authorization may not hide broad over-read work.
- `domain_laws.md`: policy context, tenant basis resolution, tenant schema
  validation, relationship-proof query families, delivery-shape metadata,
  diagnostics, counters, and certification rows are separate responsibilities
  and must not collapse into one `policy.rs` or `tenant.rs` bag.
- `forge_query_vision.md`: policy-aware aspect masking, branch-level access
  scoping, automatic tenant branch scoping, tenant-scoped schema awareness, and
  graph-native relationship proofs are explicit product pillars. Milestone 9 is
  where those become structural query artifacts rather than server middleware
  conventions.
- `forge_query_roadmap.md`: Milestone 9 must prove tenant schema variation plus
  validation plus delivery-shape parity, and policy masking parity across at
  least one-shot, live, and historical execution, while keeping durable
  tenant/query artifacts explicit store-gated debt.
- `test-requirements.md`: the `Policy, Tenant Schema, And Relationship-Proof
  Boundary Test` is the closeout proof. It requires masked/unmasked policy
  contexts, two tenant schema variants, valid and broken relationship-proof
  chains, and parity across admitted execution modes.
- `milestone-7.md`: identity-evolution and correspondence results must remain
  intact under policy narrowing. Milestone 9 may deny, mask, or narrow, but it
  may not flatten lineage/correspondence classification into generic access
  failures.
- `milestone-8.md` and `milestone-8-closeout.md`: canonical composition,
  ephemeral saved-query freeze, and view-shape semantics are already sealed.
  Milestone 9 must govern those frozen artifacts rather than inventing an
  alternate policy-aware query path around them.

## Adversarial Constraint

Milestone 9 must survive the following hostile condition:

> The same canonical query shape is authored directly, through scopes, through
> templates, and through an admitted saved-query artifact, then executed as a
> one-shot read, a live-maintained query, and an admitted historical read under
> masked and unmasked policy contexts, divergent tenant schemas, and valid or
> broken relationship-proof chains; every admitted lane must preserve one
> canonical query meaning while ensuring that masked or unauthorized truth never
> enters the execution plan, live-maintenance path, result materialization, or
> delivery metadata.

Concretely, the design must remain correct when all of the following are true:

- the same query is legal under one tenant schema and illegal or differently
  projected under another
- the same saved-query artifact is reused under a different policy or tenant
  basis and must classify that reuse honestly
- a relationship-proof query is admitted in one context and denied in another
- one-shot, live, and historical execution all exist for the same narrowed
  query family
- view-shape and delivery metadata from Milestone 8 still need to reflect the
  exact caller-visible masked result
- Milestone 7 identity-evolution surfaces may be present inside the result
  shape and must remain typed rather than turned into opaque "forbidden" gaps
- a naive implementation would be tempted to:
  - read the full projection and redact masked aspects later
  - resolve tenant branch and tenant schema in server middleware instead of the
    query plan
  - treat relationship proofs as side-effectful callbacks rather than typed
    query predicates
  - let live maintenance run against unmasked truth and mask only on delivery
  - reuse saved-query artifacts without re-checking tenant/schema/policy basis
  - derive delivery metadata from unmasked execution artifacts instead of
    caller-visible query meaning

If any supported path:

- reads masked aspects and discards them later
- resolves tenant scope or branch access through hidden ambient filters
- changes masking or denial semantics between one-shot, live, and historical
  execution for the same declared basis
- lets relationship-proof denials happen after unauthorized truth was already
  read
- changes saved-query or scope-composed meaning under a new tenant schema
  without explicit legality classification
- produces delivery metadata that reveals masked or unauthorized structure
- implies durable tenant/query artifact parity before store support exists

then Milestone 9 has failed.

## Product Decision Lock

- `forge-query` owns policy-aware narrowing artifacts, tenant truth/schema basis
  resolution artifacts, relationship-proof query families, denial bundles,
  delivery-shape metadata, diagnostics, support reporting, and certification for
  admitted Milestone 9 surfaces
- schema/platform layers remain authoritative for policy rules, branch access
  rules, tenant resolution inputs, and relationship-proof semantics
- `forge-relational` remains authoritative for truth semantics, schema
  semantics, basis semantics, and any lower truth needed to evaluate admitted
  relationship proofs
- policy masking is plan-owned and pre-execution; it is not a delivery-only or
  host-middleware concern
- policy narrowing must produce an `AuthorizedProjectionArtifact` consumed by
  execution directly:
  - executors may not receive the broader pre-mask projection after narrowing
  - the masked fast path is therefore an architectural input type, not just a
    counter claim
- tenant scope is dual:
  - it narrows truth basis explicitly
  - it narrows schema basis explicitly
- tenant resolution must declare an explicit cost class:
  - `DirectBinding` and `CachedBinding` are the intended admitted postures for
    Milestone 9
  - `DerivedBinding` is explicit debt or denial unless its breadth posture is
    separately admitted later
- relationship-proof queries are typed query semantics and denial semantics;
  they are not closures, callbacks, or server-owned middleware hooks
- relationship-proof admission must declare an explicit topology class:
  - `DirectEdge`, `TwoHopChain`, or `PreLoweredProofSet`
  - arbitrary recursive proof walking is out of scope for Milestone 9
- delivery-shape metadata is derived from the masked/projected query result the
  caller is allowed to see, not from a wider hidden execution result
- masking is shape-changing when necessary:
  - masked aspects may be removed from the admitted result shape
  - they may not survive as `Option::None`, redacted sentinels, empty payloads,
    or placeholder columns unless the schema itself declared that exact
    caller-visible shape independently of policy
- saved-query reuse under Milestone 9 must route through explicit policy/tenant
  equivalence or denial classification rather than ambient "same query"
  heuristics
- one-shot, live, and historical execution must consume the same masked basis
  semantics for the same admitted query family
- live policy drift is re-admission, not incremental reinterpretation:
  - if policy basis or tenant basis changes after live admission, the old live
    lane must be terminated or fully re-admitted from a fresh masked baseline
  - no live lane may preserve cached wider truth and "mask harder" later
- live lanes must declare a `PolicyDriftDisposition`:
  - Milestone 9 intends to admit `NoChange` and
    `FreshAdmissionFromCheckpoint`
  - `FullRestartDebt` must stay explicit debt when unavoidable
- Milestone 9 may compose with admitted mutation, merge, writeback, and stream
  declarations from earlier milestones, but it may only narrow or deny; it may
  not become a second workflow engine
- delivery contracts must declare one `DeliveryWidthClass` so emitted width is
  planner-visible rather than transport-local guesswork
- durable delivery cursors, restart-stable subscription metadata, and persisted
  tenant/query artifacts remain later store-gated work

Normative consequence:

- any implementation path that masks by post-read redaction is out of spec
- any implementation path that resolves tenant branch or tenant schema outside
  the query-owned admission/lowering path is out of spec
- any implementation path that treats relationship proofs as host-local
  callbacks is out of spec
- any implementation path that lets execution consume the broader pre-mask
  projection after policy narrowing is out of spec
- any implementation path that hides expensive tenant discovery behind the same
  admission shape as direct tenant binding is out of spec
- any implementation path that treats open-ended graph walking as an admitted
  Milestone 9 relationship-proof topology is out of spec
- any implementation path that lets live maintenance consume wider truth than
  one-shot execution for the same masked basis is out of spec
- any implementation path that preserves masked fields as placeholder values in
  caller-visible result or delivery shape is out of spec
- any implementation path that treats policy-basis drift on a live lane as a
  patch problem instead of a fresh admission problem is out of spec
- any implementation path that lets delivery metadata reveal masked or denied
  structure is out of spec
- any implementation path that emits delivery width without one explicit
  planner-owned width class is out of spec
- any implementation path that claims durable tenant/query artifact semantics in
  Milestone 9 is out of spec

## Typed Phase Progression Lock

Required phase progression:

- `RawPolicyAwareIntent`
  - canonical query artifacts from Milestones 1 through 8 may exist here
  - policy/tenant/relationship context has not yet been admitted
- `AdmittedPolicyTenantContext`
  - branch access, tenant basis resolution, tenant schema basis, and policy
    basis are proven and attached
  - unsupported or ambiguous policy/tenant contexts are rejected here
- `NarrowedPolicyQueryArtifact`
  - masked projection, authorized projection, relationship-proof clauses, and
    denial posture are fixed
  - no execution has occurred
- `PolicyAwareExecutionPlan`
  - one-shot/live/historical execution posture and delivery-shape derivation
    posture are fixed for the narrowed query
  - tenant resolution class, relationship-proof topology class, and delivery
    width class are fixed here
  - live lanes must also carry a `PolicyEpoch` and `TenantBasisEpoch` binding
    so later drift forces fresh admission instead of silent reinterpretation
- `PolicyAwareExecutionEnvelope`
  - result or denial envelope with policy/tenant metadata, diagnostics, and
    counters
- `PolicyAwareDeliveryEnvelope`
  - server-facing delivery metadata derived from the caller-visible masked
    result, not from hidden wider truth

Rules:

- no API may admit execution directly from a canonical query artifact that has
  not yet passed policy and tenant admission
- no API may mutate masked projection after `NarrowedPolicyQueryArtifact`
- no API may recover the broader pre-mask projection once
  `AuthorizedProjectionArtifact` exists
- no API may mutate tenant truth or schema basis after
  `AdmittedPolicyTenantContext`
- no API may widen tenant resolution class, relationship-proof topology class,
  or delivery width class after `PolicyAwareExecutionPlan`
- live and historical lowering must consume `PolicyAwareExecutionPlan`, not a
  pre-policy ordinary execution plan
- no live patch lane may continue after `PolicyEpoch` or `TenantBasisEpoch`
  drift without fresh re-admission from a newly masked baseline
- delivery metadata must consume `PolicyAwareExecutionEnvelope`, not raw
  internal execution results
- saved-query reuse under Milestone 9 must re-enter at
  `AdmittedPolicyTenantContext`; it may not jump directly to execution based on
  a prior artifact alone

Normative consequence:

- if one-shot execution narrows before planning but live maintenance narrows
  only on delivery, the phase chain is broken
- if tenant schema legality can change after planning without invalidating the
  policy-aware plan, the phase chain is broken
- if delivery metadata can be built from wider unmasked results than the caller
  saw, the phase chain is broken
- if a lower execution layer can swap `DirectBinding` for hidden
  `DerivedBinding` tenant resolution after admission, the phase chain is broken
- if relationship-proof execution can broaden from `DirectEdge` or
  `TwoHopChain` into open-ended graph walking after planning, the phase chain is
  broken
- if delivery width can inflate beyond its admitted class without changing the
  plan artifact, the phase chain is broken
- if a live subscription survives policy drift by patching an already admitted
  wider baseline instead of re-admitting the lane, the phase chain is broken

## Compile-Time Enforcement Policy

Milestone 9 must classify which policy and tenant guarantees become
unrepresentable, uncompilable, or construction-time rejection.

`Unrepresentable` in public types:

- publicly constructible policy-aware query artifacts that do not carry
  canonical query identity, policy basis identity, and tenant truth/schema
  basis identity
- publicly constructible masked execution plans that do not distinguish masked
  projection from denied projection
- publicly constructible execution plans that carry caller-visible result shape
  without one explicit authorized projection artifact
- publicly constructible caller-visible result shapes that encode masked aspects
  as nullable placeholders or redacted sentinels rather than as one explicit
  authorized result-shape disposition
- publicly constructible relationship-proof success artifacts that do not carry
  explicit proof family and denial semantics
- publicly constructible tenant-resolution artifacts that do not carry one
  explicit `TenantResolutionClass`
- publicly constructible relationship-proof plan artifacts that do not carry one
  explicit `RelationshipProofTopology`
- publicly constructible delivery metadata artifacts that do not carry the
  masked result-shape identity they were derived from and one explicit
  `DeliveryWidthClass`
- publicly constructible result bundles that erase whether the outcome was
  admitted masked execution, branch denial, tenant ambiguity, tenant-schema
  incompatibility, relationship-proof denial, or policy/query incompatibility

`Uncompilable` through visibility and compile-fail enforcement:

- external construction of `PolicyAwareQueryContext`,
  `TenantResolvedBasisArtifact`, `PolicyMaskedProjection`,
  `RelationshipProofDescriptor`, `PolicyAwareExecutionPlan`,
  `PolicyAwareExecutionEnvelope`, `PolicyAwareDeliveryMetadata`, or materially
  equivalent proof-bearing types without crate-owned lowering
- public APIs that accept raw authorization callbacks, raw policy closures, raw
  middleware hooks, or raw branch filters as query-owned policy authority
- public APIs that let executors consume raw or pre-mask projection once
  `AuthorizedProjectionArtifact` was created
- public APIs that expose bool-driven shortcuts such as `masked: bool`,
  `tenant_mode: bool`, `check_access: bool`, or `authorize_relations: bool`
- public APIs that hide `TenantResolutionClass`, `RelationshipProofTopology`, or
  `DeliveryWidthClass` behind bag-shaped config or dynamic strings
- public APIs that allow tenant truth basis and tenant schema basis to be
  resolved independently after admission without revalidation
- public APIs that allow a live lane to continue across policy epoch or tenant
  basis epoch drift without constructing a new admitted plan
- public APIs that allow server-facing delivery metadata to be created without
  consuming a policy-aware execution envelope

`Construction-time rejection`:

- denied branch access
- ambiguous tenant context
- tenant branch resolution failure
- tenant schema mismatch for the declared query/result shape
- unsupported relationship-proof family
- broken relationship-proof chain where the declared query requires proof
- unsupported policy/query composition over admitted mutation, merge,
  writeback, or stream declarations
- saved-query reuse attempts whose policy/tenant/schema basis changes require
  fresh freeze or typed denial
- historical or live execution requests whose policy basis is unsupported for
  the admitted family

Rules:

- the strongest available boundary must be used
- policy, tenant, masking, and delivery metadata types must use sealed
  constructors and private fields
- adding a new policy denial class, tenant basis class, or relationship-proof
  family must force compile failures across lowering, execution, diagnostics,
  support reporting, and certification until handled explicitly
- wildcard or catch-all matching over milestone-owned denial or proof families
  is out of spec
- compile-fail coverage is required for:
  - no external fabrication of policy-aware execution or delivery artifacts
- no raw callback-based relationship-proof authority
- no raw projection execution after authorized projection narrowing
- no bag-shaped or stringly typed tenant-resolution, proof-topology, or
  delivery-width admission
- no post-admission tenant basis mutation
- no delivery metadata creation from pre-policy execution paths
- no caller-visible placeholder encoding for masked aspects unless the schema
  declared that exact shape as public truth independent of policy
- no continued live admission across policy or tenant epoch drift
- no bool-driven policy/tenant shortcut routing
- no saved-query execution that bypasses fresh policy/tenant admission

## Scope

### In Scope

- aspect-level policy masking in canonical query lowering
- branch-level access scoping and explicit denial before reads execute
- tenant-scoped truth basis resolution where tenant truth is branch-backed
- tenant-scoped schema validation and projection legality
- graph-native relationship-proof predicate/query families for admitted access
  and legality proofs
- delivery-shape metadata derived from caller-visible masked/projected result
  meaning
- policy composition for admitted mutation, merge, writeback, and streamed
  delivery declarations where those surfaces already exist
- policy/tenant-aware handling for direct, scope-composed, template-instantiated,
  and saved-query-frozen artifacts
- milestone-native certification for masking, tenant schema variation,
  relationship-proof denial, and mode parity

### Initial Admission Matrix

Milestone 9 must not leave policy and tenant support ambient or "general
enough to figure out later." The initial admitted surface must be explicit.

Initial policy-masking-admitted query families:

- detail queries with aspect projection where some aspects are fully visible and
  others are fully masked
- collection/table queries where masking may narrow projected columns without
  changing collection identity semantics
- inspector-oriented detail queries from Milestone 8 where focused projection
  may be further reduced by policy
- admitted historical reads for the same detail/collection families
- admitted live-promoted detail/collection families where the masked projection
  is already frozen before live maintenance begins

Initial tenant-basis-admitted families:

- tenant-to-branch resolution where one tenant maps to exactly one explicit read
  branch at admission time through `DirectBinding`
- tenant schema resolution where one tenant maps to exactly one explicit schema
  basis at admission time through `DirectBinding` or admitted `CachedBinding`
- saved-query reuse only when tenant truth basis, schema basis, and policy
  basis classify as `LegalNoSemanticChange` or `LegalRequiresFreshFreeze`

Initial relationship-proof-admitted families:

- read-authorization proof over one explicit subject-relation-object chain
  lowered as `DirectEdge` or `TwoHopChain`
- branch-read proof over one explicit subject-branch relation lowered as
  `DirectEdge`
- workflow-legality proof for already-admitted query-authored workflow
  declarations from earlier milestones where proof denial must prevent the lane
  from executing, using `PreLoweredProofSet` where the proof set was already
  planner-owned upstream

Required vocabulary artifacts:

- `PolicyBasisIdentity`
- `TenantTruthBasisIdentity`
- `TenantSchemaBasisIdentity`
- `TenantResolutionClass`
- `PolicyProjectionDisposition`
- `SavedQueryPolicyReuseDisposition`
- `PolicyReuseEquivalenceContract`
- `RelationshipProofFamily`
- `RelationshipProofTopology`
- `RelationshipProofDenialClass`
- `PolicyAwareResultShapeDisposition`
- `DeliveryWidthClass`
- `PolicyDriftDisposition`
- `PolicyEpoch`

Required admitted artifacts:

- `PolicyAwareQueryContext`
- `ResolvedPolicyBasisArtifact`
- `ResolvedTenantBasisArtifact`
- `ResolvedTenantSchemaArtifact`
- `PolicyMaskedProjectionArtifact`
- `AuthorizedProjectionArtifact`
- `AuthorizedResultShapeArtifact`
- `RelationshipProofPlanArtifact`
- `PolicyAwareExecutionPlan`
- `PolicyAwareExecutionEnvelope`
- `PolicyAwareDeliveryEnvelope`

Required reuse classifications:

- `LegalNoSemanticChange`
- `LegalRequiresFreshFreeze`
- `IllegalSemanticDrift`

Required tenant resolution classes:

- `DirectBinding`
- `CachedBinding`
- `DerivedBinding`

Required relationship-proof topology classes:

- `DirectEdge`
- `TwoHopChain`
- `PreLoweredProofSet`

Required delivery width classes:

- `ScalarDetail`
- `NarrowCollection`
- `GroupedDelta`

Required policy drift dispositions:

- `NoChange`
- `FreshAdmissionFromCheckpoint`
- `FullRestartDebt`

### Explicitly Out Of Scope

- durable tenant/query artifact persistence
- restart-stable subscription metadata and durable delivery cursors
- store-backed portability and restart parity for tenant/query artifacts
- schema/platform authoring of the policy model itself
- lower-runtime truth semantics for relationship proofs
- post-Milestone-9 durable artifact import/export semantics
- presentation-specific policy UX or server middleware conventions outside the
  canonical query artifact

## Scope

## Phases

### Phase 1: Policy And Tenant Context Admission

Freeze policy and tenant context as proof-bearing query artifacts before
execution planning is allowed to begin.

This phase must produce:

- one explicit `PolicyBasis` artifact tied to the canonical query and caller
  context
- one explicit `TenantTruthBasis` artifact and one explicit
  `TenantSchemaBasis` artifact
- one closed denial family for branch denial, tenant ambiguity, tenant
  resolution failure, and unsupported execution-mode composition
- one explicit legality classification for saved-query reuse under changed
  policy or tenant basis

This phase must not:

- perform truth reads
- evaluate live maintenance
- derive delivery metadata
- let hosts supply hidden tenant filters or middleware-only policy overrides

### Phase 2: Pre-Execution Narrowing And Validation

Lower policy masking, tenant-scoped schema legality, and relationship-proof
requirements into one narrowed query artifact before any one-shot, live, or
historical plan is admitted.

This phase must produce:

- one masked projection artifact
- one relationship-proof descriptor family with admitted and denied lanes
- one narrowed result-shape identity derived from the caller-visible projection
- one explicit incompatibility family for policy/query, tenant-schema/query,
  and relationship-proof/query conflicts

This phase must prove:

- masked aspects never enter the execution plan
- tenant schema variation is reflected in legality and result-shape identity
- broken relationship-proof chains deny before query execution
- composed and saved-query inputs are governed by the same narrowing rules as
  direct construction

### Phase 3: Delivery-Shape And Execution Parity

Execute the already narrowed query across admitted one-shot, live, and
historical modes while preserving one masked basis and one caller-visible
delivery meaning.

This phase must produce:

- one policy-aware one-shot execution path
- one policy-aware live-maintenance path that never widens beyond the same
  masked projection and basis
- one policy-aware historical execution path for admitted families
- one delivery-shape metadata family derived from the masked result and
  narrowed view semantics

This phase must prove:

- one-shot, live, and historical behavior are parity-safe for the same policy
  basis
- live maintenance does not observe wider truth than one-shot execution
- delivery metadata does not reveal masked or unauthorized structure
- branch denial and tenant denial still happen before runtime/store reads

### Phase 4: Certification, Diagnostics, And Support Honesty

Close the milestone with machine-checkable certification, exact denial
diagnostics, support reporting, and compile-time proof boundaries.

This phase must produce:

- the milestone-native certification suite for policy, tenant schema, and
  relationship-proof boundaries
- exact counters and denial diagnostics embedded in result/denial bundles
- support-report truth for admitted versus deferred policy/tenant surfaces
- compile-fail coverage for fabricated policy-aware artifacts and bypassed
  admission paths

## Must Ship

- one dedicated policy/tenant query subdomain rather than host middleware glue
- aspect-level policy masking in planning and narrowing
- branch-level access scoping in validation and execution context admission
- tenant truth-basis resolution where tenant truth is branch-backed
- tenant schema-basis resolution and validation
- relationship-proof predicate/query families for admitted graph-native proof
  semantics
- policy-aware delivery metadata derived from masked/projected result meaning
- saved-query and composed-query policy/tenant equivalence or denial
  classification
- policy composition rules for admitted mutation, merge, writeback, and
  streamed-delivery declarations
- typed denial bundles for:
  - masked projection denial where relevant
  - denied branch access
  - ambiguous tenant context
  - tenant-schema incompatibility
  - relationship-proof denial
  - policy/query incompatibility
- exact counters, diagnostics, support metadata, and certification rows for the
  admitted Milestone 9 surface

## Must Preserve

- canonical query meaning from Milestones 1 through 8 remains authoritative
- policy authority stays with schema/platform layers rather than query host code
- `forge-query` owns narrowing and denial artifacts, not policy truth itself
- masked aspects must not be read and then discarded later
- tenant truth basis and tenant schema basis remain explicit and paired rather
  than ambient hidden filters
- saved-query freeze remains the one artifact model extended by Milestone 9
- Milestone 8 view-shape semantics remain planner-owned and delivery-owned
- Milestone 7 identity-evolution semantics remain typed and may only be denied
  or narrowed explicitly, not flattened
- delivery metadata remains derived from canonical query plus policy/tenant
  basis, never from wider hidden execution results
- unsupported combinations fail typed and early rather than degrading into best
  effort

## Complexity / Proof Obligations

Milestone 9 must name costs and proofs in terms of:

- declared policy masking complexity contract
- declared tenant basis resolution complexity contract
- declared tenant schema validation complexity contract
- declared relationship-proof complexity contract
- declared delivery-shape derivation complexity contract
- authorized projection width
- masked projection entry count
- policy basis resolution count
- denied branch access count
- tenant truth basis resolution count
- tenant schema basis resolution count
- direct tenant binding count
- cached tenant binding count
- derived tenant binding denial count
- tenant-schema validation branch count
- relationship-proof admission count
- relationship-proof denial count
- relationship-proof topology width
- relationship-proof recursive-broadening denial count
- saved-query policy/tenant rebinding classification count
- policy epoch drift re-admission count
- live policy parity comparison count
- historical policy parity comparison count
- delivery metadata derivation count
- delivery emitted field count
- delivery emitted item count
- delivery width inflation denial count
- masked collection broad-scan denial count
- forbidden post-read redaction count
- forbidden host-callback proof count
- executor policy rediscovery count
- delivery metadata overexposure count
- complexity status debt count

Minimum named complexity contracts:

- `policy_masked_projection_lowering`
  - declared Big-O:
    `O(projected_entries + masked_entries + policy_basis_resolution)`
  - forbidden broadening clause:
    no whole-result materialization before masking
- `tenant_truth_and_schema_basis_resolution`
  - declared Big-O:
    `O(tenant_context_resolution + basis_binding)`
  - forbidden broadening clause:
    no ambient multi-branch scan to discover tenant truth
- `authorized_projection_execution`
  - declared Big-O:
    `O(authorized_projection_width)`
  - forbidden broadening clause:
    no reintroduction of pre-mask projection width downstream of narrowing
- `tenant_schema_validation`
  - declared Big-O:
    `O(validated_predicates + validated_projection_entries + schema_variant_branches)`
  - forbidden broadening clause:
    no fallback to global schema when tenant schema disagrees
- `relationship_proof_admission`
  - declared Big-O:
    `O(proof_clauses + admitted_proof_bindings)`
  - forbidden broadening clause:
    no host callback execution or post-read authorization repair
- `bounded_relationship_proof_topology`
  - declared Big-O:
    `O(admitted_proof_bindings + topology_width)`
  - forbidden broadening clause:
    no recursive graph expansion beyond admitted topology class
- `policy_epoch_drift_rebind`
  - declared Big-O:
    `O(policy_epoch_check + tenant_basis_epoch_check + fresh_admission_if_changed)`
  - forbidden broadening clause:
    no in-place patch reinterpretation of a previously admitted wider baseline
- `delivery_shape_derivation_after_masking`
  - declared Big-O:
    `O(masked_result_shape_width + admitted_view_metadata_width)`
  - forbidden broadening clause:
    no derivation from unmasked internal result families
- `masked_collection_execution_scope`
  - declared Big-O:
    `O(admitted_row_scope + authorized_projection_width)`
  - forbidden broadening clause:
    no full row materialization followed by visibility filtering

Minimum required counters:

- `declared_policy_masking_complexity_contract_count`
- `declared_tenant_basis_resolution_contract_count`
- `declared_tenant_schema_validation_contract_count`
- `declared_relationship_proof_contract_count`
- `declared_delivery_shape_derivation_contract_count`
- `declared_authorized_projection_execution_contract_count`
- `declared_bounded_relationship_topology_contract_count`
- `declared_masked_collection_scope_contract_count`
- `authorized_projection_width`
- `masked_projection_entry_count`
- `policy_basis_resolution_count`
- `branch_access_denial_count`
- `tenant_truth_basis_resolution_count`
- `tenant_schema_basis_resolution_count`
- `direct_tenant_binding_count`
- `cached_tenant_binding_count`
- `derived_tenant_binding_denial_count`
- `tenant_schema_validation_branch_count`
- `relationship_proof_admission_count`
- `relationship_proof_denial_count`
- `relationship_proof_topology_width`
- `relationship_proof_recursive_broadening_denial_count`
- `saved_query_policy_tenant_rebinding_classification_count`
- `policy_epoch_drift_readmission_count`
- `one_shot_policy_parity_count`
- `live_policy_parity_count`
- `historical_policy_parity_count`
- `delivery_metadata_derivation_count`
- `delivery_emitted_field_count`
- `delivery_emitted_item_count`
- `delivery_width_inflation_denial_count`
- `masked_collection_broad_scan_denial_count`
- `forbidden_post_read_redaction_count`
- `forbidden_host_callback_proof_count`
- `policy_executor_rediscovery_count`
- `delivery_metadata_overexposure_count`
- `complexity_contract_violation_denial_count`
- `complexity_status_debt_count`

Rules:

- counters belong to admitted result bundles, denial bundles, and certification
  bundles
- representative certification scenarios must assert exact counts
- every admitted lane must emit exactly one policy masking contract, one tenant
  basis contract, and one delivery derivation contract
- every admitted live lane must also emit one policy epoch drift contract
- every admitted execution lane must emit one authorized projection execution
  contract
- every admitted relationship-proof lane must emit one bounded topology
  contract
- `forbidden_post_read_redaction_count` must be exactly zero on every admitted
  lane
- `forbidden_host_callback_proof_count` must be exactly zero on every admitted
  lane
- `policy_executor_rediscovery_count` must be exactly zero on every admitted
  lane
- `delivery_metadata_overexposure_count` must be exactly zero on every admitted
  lane
- `derived_tenant_binding_denial_count` must increment on every denied
  `DerivedBinding` lane admitted out of scope for Milestone 9
- `relationship_proof_recursive_broadening_denial_count` must increment on
  every denied proof request that requires recursive graph walking
- `masked_collection_broad_scan_denial_count` must increment on every denied
  masked collection lane that would require full row materialization before
  narrowing
- `delivery_width_inflation_denial_count` must increment on every denied lane
  whose emitted width exceeds its admitted `DeliveryWidthClass`
- `policy_epoch_drift_readmission_count` must increment whenever policy or
  tenant basis drift forces a fresh live admission
- tenant schema variation must be mechanically visible through
  `tenant_schema_validation_branch_count`, not prose only
- every lane whose durability claim remains incomplete must increment
  `complexity_status_debt_count` rather than implying full closure
- elapsed time alone is not acceptable evidence for any Milestone 9 performance
  or boundedness claim

Minimum certification rows should include:

- `masked-plan-does-not-read-masked-aspects`
- `tenant-branch-resolution-explicitness`
- `tenant-schema-variant-validation-parity`
- `tenant-schema-result-shape-drift-explicitness`
- `authorized-projection-width-explicitness`
- `masked-collection-no-broad-scan`
- `relationship-proof-success-vs-denial-explicitness`
- `relationship-proof-non-leakage`
- `bounded-relationship-topology-explicitness`
- `saved-query-policy-basis-rebinding-classification`
- `live-policy-epoch-drift-readmission`
- `live-policy-masking-parity`
- `historical-policy-masking-parity`
- `delivery-shape-post-mask-parity`
- `delivery-width-class-honesty`
- `policy-composed-workflow-denial-explicitness`
- `support-profile-honesty`

Minimum rejection rows should include:

- `post-read-redaction-forbidden`
- `hidden-tenant-filter-forbidden`
- `tenant-schema-global-fallback-forbidden`
- `derived-tenant-resolution-forbidden`
- `relationship-proof-host-callback-forbidden`
- `recursive-relationship-proof-broadening-forbidden`
- `denied-branch-before-read`
- `ambiguous-tenant-context-denied`
- `delivery-metadata-overexposure-forbidden`
- `delivery-width-inflation-forbidden`
- `saved-query-policy-bypass-forbidden`
- `masked-placeholder-shape-forbidden`
- `live-policy-drift-without-readmission-forbidden`
- `unsupported-policy-workflow-composition`

## Allowed Debt

- durable tenant/query artifacts may remain explicit `Debt`
- durable delivery cursors and restart-stable subscription metadata may remain
  explicit `Debt`
- additional relationship-proof families beyond the initial admitted set may
  remain explicit `Debt`
- policy masking by post-read redaction may not exist as debt
- host-local authorization callbacks may not exist as debt
- hidden tenant truth or schema basis resolution may not exist as debt
- delivery metadata derived from wider hidden results may not exist as debt

## Acceptance Evidence

Milestone 9 is complete only when `forge-query` can prove:

- the `Policy, Tenant Schema, And Relationship-Proof Boundary Test` in
  [test-requirements.md](./test-requirements.md) passes with canonical
  machine-checkable artifacts
- masked aspects never appear in the execution plan, live-maintenance path, or
  result bundle
- denied branch or tenant access fails before runtime or store reads execute
- tenant-scoped schema variation changes validation and projection behavior
  explicitly and deterministically across at least two schema states
- broken relationship-proof chains fail closed without leaking masked or
  unauthorized truth
- the same policy basis produces the same narrowing semantics for one-shot,
  live, and historical execution
- delivery-shape metadata remains parity-safe with the exact masked/projected
  result meaning visible to the caller
- scope-composed, template-instantiated, and saved-query-frozen inputs honor the
  same policy and tenant basis rules as direct construction

Required verification output must include:

- `query_digest`
- `policy_digest`
- `tenant_basis_digest`
- `schema_basis_digest`
- `result_shape_digest`
- `delivery_digest`
- `result_digest`
- `failure_digest`
- `counter_snapshot`

### Representative Scenario Matrix

Minimum representative scenarios:

- `masked-versus-unmasked-plan-parity`
  - one query under masked and unmasked policy basis produces distinct result
    and policy digests
  - the masked lane proves masked aspects never entered the plan
- `tenant-a-schema-versus-tenant-b-schema`
  - one canonical query is legal under one tenant schema and either differently
    shaped or denied under another
  - validation and result-shape drift remain explicit
- `masked-collection-no-broad-scan`
  - one masked collection lane proves execution touched only admitted row scope
    and authorized projected fields
  - one hostile lane that requires full row materialization is denied and
    increments `masked_collection_broad_scan_denial_count`
- `denied-branch-before-read`
  - one hostile lane requests a denied branch and fails before execution begins
- `relationship-proof-broken-chain-denial`
  - one hostile relationship-proof lane fails closed before unauthorized truth
    is read
- `bounded-relationship-topology-explicitness`
  - one proof lane succeeds under `DirectEdge` or `TwoHopChain`
  - one hostile lane requiring recursive proof walking is denied before
    execution and increments
    `relationship_proof_recursive_broadening_denial_count`
- `live-masked-parity`
  - one live-maintained query converges to the same masked result as repeated
    one-shot execution for the same policy basis
- `historical-masked-parity`
  - one admitted historical lane preserves the same masking semantics as the
    equivalent one-shot lane for the same policy basis
- `saved-query-policy-rebinding-classification`
  - one saved-query artifact reused under a different policy or tenant context
    is classified as `LegalNoSemanticChange`, `LegalRequiresFreshFreeze`, or
    `IllegalSemanticDrift`
- `masked-placeholder-shape-forbidden`
  - one hostile lane attempts to preserve masked fields as `None`, sentinel
    strings, or redacted placeholder columns and is denied
- `live-policy-epoch-drift-readmission`
  - one admitted live lane experiences policy or tenant basis drift and proves
    the runtime terminates or re-admits from a fresh masked baseline rather than
    patching an older wider baseline
- `delivery-shape-derived-after-mask`
  - one delivery metadata lane proves the emitted delivery digest matches the
    masked caller-visible result rather than an unmasked internal shape
- `delivery-width-class-honesty`
  - one `ScalarDetail`, one `NarrowCollection`, or one `GroupedDelta` lane
    proves emitted field/item width stayed within the admitted width class
  - one hostile lane exceeding that width is denied and increments
    `delivery_width_inflation_denial_count`
- `relationship-proof-host-callback-forbidden`
  - one hostile lane attempts callback-based proof evaluation and fails typed
- `derived-tenant-resolution-forbidden`
  - one hostile lane attempts `DerivedBinding` tenant discovery outside the
    admitted Milestone 9 surface and fails typed
- `tenant-schema-global-fallback-forbidden`
  - one hostile lane attempts to validate against a global schema when tenant
  schema disagrees and fails typed
- `post-read-redaction-forbidden`
  - one hostile lane attempts wide execution plus later redaction and is denied
- `policy-composed-stream-declaration-denied-or-narrowed`
  - one admitted streamed-delivery or workflow-composed lane proves policy
    composition narrows or denies before execution rather than after delivery

## Architectural Notes

### Policy Must Narrow Before Planning Freezes

The easiest way to fake policy-aware queries is to plan the ordinary query and
hide policy inside delivery filtering. That is out of spec.

The required rule is:

- policy basis must be admitted before execution planning
- masked projection must be part of the narrowed query artifact
- one-shot, live, and historical execution must all consume that narrowed plan
- delivery may only reflect already-masked query meaning

### Tenant Truth Basis And Tenant Schema Basis Are Separate And Paired

Milestone 9 must keep two different tenant concerns explicit:

- which truth basis the tenant is allowed to read
- which schema basis that tenant's query must validate against

They often move together, but they are not the same thing. Any implementation
that collapses them into one ambient tenant flag is out of spec.

### Relationship Proofs Must Be Query Semantics, Not Middleware

If relationship proofs live in middleware, callbacks, or server-only
authorization hooks, then the query system cannot certify them, and live or
historical paths will drift.

The required rule is:

- proof clauses are typed query artifacts
- proof success and proof denial are typed result families
- broken proof chains deny before truth is exposed
- host code may supply context, but not semantic authority

### Delivery Contracts Must Reflect What The Caller Could Actually See

Milestone 8 already made view shape and delivery posture query-owned. Milestone
9 extends that rule:

- delivery metadata must be derived after masking and denial resolution
- delivery metadata may not reveal masked structure, forbidden fields, or
  denied branch/tenant shape
- server-facing transport layers must consume the policy-aware delivery artifact
  rather than reconstructing it from wider internals

### Placeholder Masking Is A Semantic Leak

One of the most tempting naive implementations is:

- keep the original result shape
- read the forbidden field anyway
- replace the forbidden field with `None`, `"REDACTED"`, or an empty payload

That is out of spec unless the schema already declared that exact caller-visible
shape independent of policy.

Why:

- it leaks that the field exists
- it conflates "field absent by shape" with "field forbidden by policy"
- it lets saved-query and delivery-shape semantics drift while pretending the
  query meaning stayed the same

The required rule is:

- policy may shrink the authorized result shape
- caller-visible result and delivery artifacts must encode that authorized shape
  explicitly
- placeholder redaction is not a substitute for narrowing

### Live Policy Drift Must Re-Admit, Not Reinterpret

Another naive trap is to admit a live query under one policy basis, cache the
broader state, and then "tighten" visibility later when the policy changes.

That is out of spec.

The required rule is:

- live lanes bind to explicit policy and tenant basis epochs
- epoch drift invalidates the old live lane
- the system must either terminate that lane or re-admit it from a fresh masked
  baseline
- patches from a superseded policy epoch may not be reinterpreted into a new
  narrower visibility regime

## Sequencing Notes

Milestone 9 belongs immediately after Milestone 8 because policy and tenant
boundaries must govern the full composed-query, saved-query, and view-shaped
surface that Milestone 8 froze.

It also depends on Milestone 7's in-progress identity-evolution work because
policy denial and tenant narrowing must preserve identity result classification
where those surfaces are admitted rather than flattening them into generic auth
errors.

It must land before Milestone 10 because store-backed parity should extend one
already policy-safe and tenant-safe runtime-backed query meaning instead of
discovering policy rules per backend later.

## Parallelization Notes

Once policy/tenant vocabulary and denial/result families are frozen:

- tenant schema validation work can proceed in parallel with relationship-proof
  query-family work
- support-report and compile-fail hardening can proceed in parallel with
  certification matrix construction
- delivery-shape parity work can proceed in parallel with saved-query
  policy/tenant rebinding classification
- final closure should still wait until the composed, saved, live, and
  historical parity rows are all proven together

## Store Dependency

- Core policy-aware narrowing, tenant truth/schema basis resolution, and
  relationship-proof denial are not blocked on `forge-store`.
- Durable delivery cursors, restart-stable subscription metadata, and
  persisted tenant/query artifacts remain blocked on `forge-store` and must stay
  explicit completion debt until that support exists.

## Explicit Failure Taxonomy For Milestone 9

- denied branch access
- ambiguous tenant context
- tenant truth basis resolution failure
- tenant schema incompatibility
- hidden tenant filter
- post-read redaction attempt
- relationship-proof denial
- relationship-proof host-callback misuse
- delivery metadata overexposure
- saved-query policy/tenant bypass
- unsupported policy/workflow composition
- policy-aware execution replay divergence
- policy-aware artifact invariant break

## Anti-Patterns Explicitly Rejected

- post-read masking or response redaction as the primary policy mechanism
- ambient tenant scoping through middleware-only branch filters
- global-schema validation fallback for tenant-variant queries
- relationship-proof evaluation through host callbacks or server-local
  authorization glue
- live maintenance over wider truth than one-shot execution for the same policy
  basis
- delivery metadata derived from unmasked internal results
- saved-query reuse that bypasses fresh policy/tenant admission
- one mega-module mixing policy rules, tenant resolution, proof execution,
  delivery derivation, diagnostics, and certification

## Self-Check

This milestone solves a real structural problem rather than packaging work
cosmetically because it freezes where query legality, policy narrowing, tenant
scope, and relationship-proof denial happen: before truth is read, not after
results exist.

The adversarial constraint is load-bearing because it forbids the naive failure
mode where policy and tenant support appear correct only because hosts over-read
and redact later, or because live and historical paths quietly use different
policy rules than one-shot execution.

The milestone preserves authority boundaries because schema/platform layers own
policy truth, lower runtimes own truth semantics, and `forge-query` owns the
typed admission, narrowing, denial, delivery, and certification surfaces.

The milestone defines proof obligations rather than implementation chores
because masking parity, tenant-schema variation, relationship-proof non-leakage,
delivery-shape parity, compile-fail enforcement, and exact counters are all
required for closeout.

A competent engineer should be able to map this spec into honest `policy`,
`tenant`, `relationship_proof`, `delivery`, `diagnostics`, `support_profile`,
and certification subdomains without inventing the architecture during
implementation.

This milestone belongs at 9 because it is the policy and multi-tenant contract
layer that must govern the already-frozen composition and view surfaces before
store-backed parity and durable artifact work extend them.

## Closeout Standard

Milestone 9 is complete only when all of the following are true:

- policy masking happens before execution and prevents masked aspects from being
  read at all
- branch denial, tenant denial, and relationship-proof denial fail typed and
  early before truth is exposed
- tenant truth basis and tenant schema basis are explicit, paired, and
  certification-visible
- one-shot, live, and historical execution preserve the same narrowed policy
  meaning for the same basis
- delivery metadata reflects only the masked/projected caller-visible result
  meaning
- scope-composed, template-instantiated, and saved-query-frozen artifacts obey
  the same policy and tenant rules as direct construction
- durable tenant/query artifacts and durable continuation support remain
  explicit debt rather than implied completion

If code lands but policy still depends on post-read redaction, tenant scope is
still ambient, relationship proofs still live in callbacks, or live/historical
policy behavior still differs from one-shot execution, Milestone 9 is not
complete.
