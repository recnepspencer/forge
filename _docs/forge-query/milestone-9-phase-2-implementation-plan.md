# Milestone 9 Phase 2 Implementation Plan: Pre-Execution Narrowing And Validation

> **Parent spec:** [milestone-9.md](./milestone-9.md)
>
> **Phase:** Phase 2 only
>
> **Purpose:** consume the Phase 1 `AdmittedPolicyTenantContext` and produce the
> single narrowed, policy-aware query artifact that every later one-shot,
> branch, live, historical, diff, optimizer, and delivery surface must consume.

## Governing Context Summaries

- `MENTALITY.md`: ship the smallest honest proof-bearing boundary, make illegal
  states unrepresentable, and prefer explicit failure over ambient convention.
- `arch_laws.md`: preserve authority ownership; `forge-query` owns query
  lowering and result shaping, while policy/schema/truth authorities remain
  external inputs expressed as artifacts.
- `perf_laws.md`: performance claims require named budgets, exact counters,
  bounded algorithms, and tests that prove shape rather than timing vibes.
- `domain_laws.md`: domain meaning must be encoded in typed artifacts and
  denial taxonomies instead of host-local glue.
- `forge_query_vision.md`: query intent is declared once, lowered once, and
  executed against canonical truth without rediscovering semantics later.
- `forge_query_roadmap.md`: Milestone 9 must make policy masking, tenant
  schema variation, and relationship-proof denial structural before execution;
  durable cursor/artifact claims stay blocked on `forge-store`.
- `test-requirements.md`: certification must emit machine-checkable digests,
  counters, parity lanes, hostile lanes, and typed-failure evidence.
- `milestone-8.md` / closeout context: saved queries, scopes, templates, and
  view shapes are already canonical composition inputs; Phase 2 must govern
  those artifacts instead of creating a parallel query model.
- `milestone-9.md`: Phase 2 lowers masking, tenant schema legality, and one
  relationship-proof descriptor family into `NarrowedPolicyQueryArtifact`
  before any execution mode is admitted.

## Adversarial Constraint

Aspect-level masking and Zanzibar-style relationship-proof admission must apply
identically across one-shot reads, live subscriptions, branch reads, historical
reads, historical diffs, saved-query reuse, scope composition, templates, and
delivery lowering.

Phase 2 must therefore build one pre-execution artifact that:

- is derived only from `AdmittedPolicyTenantContext` plus canonical query/view
  inputs
- removes masked aspects before any optimizer input can observe them
- denies hidden field influence before planning or truth access
- admits or denies relationship-proof descriptors without invoking host
  callbacks or graph truth
- binds policy, tenant truth, tenant schema, branch, query, projection, and
  proof digests into one immutable artifact
- carries explicit work budgets and counters for every narrowing decision

If any execution seam later receives a raw canonical query, raw projection,
host callback, mutable mask, or relationship proof that bypasses this artifact,
Phase 2 has failed.

## Phase 2 Goal

Phase 2 implements the transformation:

```rust
AdmittedPolicyTenantContext
    -> NarrowedPolicyQueryArtifact
```

It must produce:

- `AuthorizedProjectionArtifact`
- `MaskedProjectionArtifact`
- `RelationshipProofDescriptor` with one admitted and denied descriptor family
- `PolicyAwareValidationReport`
- `NarrowedPolicyQueryArtifact`
- typed denials for hidden field influence, masked predicate/order/grouping,
  tenant-schema/query incompatibility, and relationship-proof/query conflicts
- support metadata and certification rows proving the narrowed artifact is the
  only legal input to later Milestone 9 lowering

It must not produce:

- runtime reads
- graph proof evaluation
- live subscription maintenance
- historical diff execution
- delivery payload emission
- store-backed durability claims
- optimizer transformations beyond producing optimizer-safe input metadata

## Hard Boundary

The Phase 2 artifact is a narrowed query contract, not an execution plan.

The implementation should stop at:

```rust
AdmittedPolicyTenantContext
    + CanonicalQueryBundle
    + policy/query relationship proof descriptors
    -> NarrowedPolicyQueryArtifact
```

Phase 3 starts at:

```rust
NarrowedPolicyQueryArtifact
    -> PolicyAwareCurrentPlan
    -> PolicyAwareBranchPlan
    -> PolicyAwareHistoricalPlan
    -> PolicyAwareDiffPlan
    -> PolicyAwareLivePlan
    -> PolicyAwareDeliveryShape
```

Any patch that fetches entity payloads, evaluates graph edges, computes live
updates, computes historical deltas, emits transport payloads, or redacts
post-read data is out of Phase 2.

## Proposed Module Topology

Add Phase 2 modules beside the Phase 1 basis modules. Keep ownership narrow:

```text
crates/forge-query/src/authorized_projection/
  mod.rs
  artifacts.rs
  influence.rs
  masks.rs
  counters.rs
  errors.rs
  support.rs
  tests.rs

crates/forge-query/src/relationship_proof/
  mod.rs
  descriptors.rs
  admission.rs
  counters.rs
  errors.rs
  support.rs
  tests.rs

crates/forge-query/src/policy_narrowing/
  mod.rs
  artifact.rs
  lowering.rs
  validation.rs
  counters.rs
  errors.rs
  support.rs
  tests.rs

crates/forge-query/src/harness/milestone_nine_certification/
  phase_two.rs
```

Responsibilities:

- `authorized_projection` owns aspect mask interpretation, caller-visible
  projection derivation, and hidden influence detection.
- `relationship_proof` owns query-authored proof descriptors, topology
  admission, and typed proof denials. It never evaluates truth.
- `policy_narrowing` owns the final `NarrowedPolicyQueryArtifact`, validation
  report, digest binding, and facade entrypoint.
- the Milestone 9 harness owns certification fixtures and rows only.

Avoid a generic `policy_plan` module in this phase. The optimizer-facing output
is metadata on `NarrowedPolicyQueryArtifact`, not an executable plan.

## Batch 1: Phase 2 Skeleton And Facade Gate

Create the modules and a single facade function:

```rust
pub fn narrow_policy_query(
    canonical: &CanonicalQueryBundle,
    admitted: AdmittedPolicyTenantContext,
    mask: PolicyMaskSnapshot,
    influence: PolicyInfluenceSet,
    descriptors: RelationshipProofDescriptorSet,
) -> Result<NarrowedPolicyQueryArtifact, PolicyNarrowingError>;
```

The function must require `AdmittedPolicyTenantContext` by value or by an
unforgeable reference type. It must also require a `PolicyMaskSnapshot` bound
to the admitted policy digest, not a raw host-authored mask, and a
`PolicyInfluenceSet` that explicitly carries non-projection influence from
view-shape grouping, templates, and derived result fields. It must not accept
raw policy, tenant, branch, schema, unbound mask, or ambient hidden-influence
inputs.

Initial implementation may return a typed `UnsupportedRelationshipProofFamily`
or `MissingAuthorizedProjection` denial until later batches fill the internals,
but the compile-time boundary should land first.

Acceptance checks:

- public facade exposes Phase 2 only through admitted context
- `lib.rs` keeps modules private unless a type is intentionally exported
- no caller can construct `NarrowedPolicyQueryArtifact` directly
- no caller can call Phase 2 with a raw canonical query alone

## Batch 2: Authorized Projection And Mask Artifacts

Implement projection artifacts before relationship-proof work so optimizer
inputs have a hard mask boundary.

Required types:

- `PolicyAspectMask`
- `PolicyMaskSnapshot`
- `MaskedProjectionArtifact`
- `AuthorizedProjectionArtifact`
- `AuthorizedProjectionIdentity`
- `PolicyFieldInfluenceSet`
- `PolicyInfluenceSet`
- `ProjectionVisibility`

Required visibility classes:

- `Visible`
- `Masked`
- `NonDisclosingUseOnly`
- `DeniedHiddenInfluence`

Required behavior:

- projection output includes only caller-visible fields
- masked fields are excluded from optimizer-visible projection metadata
- non-disclosing use is represented separately from emitted projection
- authorized projection identity includes canonical query digest, policy
  digest, tenant schema basis digest, and visible projection digest
- masks are immutable after the artifact is built
- policy masks are admitted only through a policy-digest-bound snapshot before
  narrowing can derive caller-visible projection metadata
- grouping, template, and derived-field influences are admitted only through a
  typed influence set consumed by the same hidden-influence pass as ordinary
  projection, predicate, and ordering selectors

Typed denials:

- `MaskedProjectionRequested`
- `MaskedPredicateInfluence`
- `MaskedOrderingInfluence`
- `MaskedGroupingInfluence`
- `MaskedDerivedFieldInfluence`
- `NonDisclosingUseWouldBeEmitted`
- `UnknownAspectMask`

Counters:

- authorized projection width
- masked projection entry count
- hidden predicate denial count
- hidden ordering denial count
- hidden grouping denial count
- hidden derived-field denial count
- forbidden post-read redaction count

## Batch 3: Hidden Influence Validation

Build a validation pass that walks the canonical query and result-shape
structure using typed selectors rather than string matching.

The pass must classify every aspect/field reference used by:

- projection
- predicates
- ordering
- grouping
- derived result fields
- cursor/tie-breaker fields where represented in the current query model
- view-shape inputs where already available from Milestone 8
- saved-query and scope-expanded canonical inputs

Rules:

- a masked field may not influence predicate truth unless policy marks it
  `NonDisclosingUseOnly`
- a `NonDisclosingUseOnly` field may not be emitted, sorted by, grouped by, or
  exposed in result-shape identity
- grouping over masked or non-disclosing fields is denied because group
  membership leaks hidden truth
- ordering over masked or non-disclosing fields is denied because ordering
  leaks hidden truth
- derived fields must declare their influence set before admission; unknown
  influence denies rather than widening
- hidden tenant filters remain forbidden; tenant narrowing must come from the
  admitted tenant basis

Output:

- `PolicyAwareValidationReport`
- deterministic validation digest
- denial list with stable failure digests
- counter snapshot

This validation report should be embedded in the final narrowed artifact. It is
not a log and must not be optional.

## Batch 4: Relationship-Proof Descriptor Admission

Implement one graph-native relationship-proof descriptor family with admitted
and denied lanes.

Required types:

- `RelationshipProofDescriptor`
- `RelationshipProofDescriptorSet`
- `RelationshipProofAdmission`
- `RelationshipProofAdmissionIdentity`
- `RelationshipProofTopologyClass`
- `RelationshipProofDenied`
- `RelationshipProofBudget`

Initial admitted topology classes:

- `DirectEdge`
- `BoundedAncestor`
- `TenantMembership`

Initial denied topology classes:

- `HostCallback`
- `UnboundedRecursiveWalk`
- `MissingProofBasis`
- `QueryShapeMismatch`
- `PolicyMismatch`
- `TenantSchemaMismatch`

Rules:

- descriptor admission may inspect only typed descriptor structure, policy
  basis identity, tenant basis identity, tenant schema identity, and canonical
  query shape
- descriptor admission must not walk graph truth or call host code
- every admitted descriptor carries one topology class, one explicit budget,
  and one digest bound to the final narrowed query artifact
- every denied descriptor carries a typed denial and failure digest
- unsupported proof families deny before narrowing succeeds
- broken descriptor chains deny before query execution; Phase 2 detects broken
  descriptor shape or missing declared basis, while Phase 3 may later evaluate
  admitted proof truth without changing the descriptor model

Counters:

- relationship-proof admission count
- relationship-proof denial count
- relationship-proof topology width
- relationship-proof recursive-broadening denial count
- forbidden host-callback proof count

## Batch 5: Narrowed Policy Query Artifact

Implement the immutable final artifact.

Required type:

```rust
pub struct NarrowedPolicyQueryArtifact { /* private fields */ }
```

Required contents:

- canonical query digest
- canonical result-shape digest before policy narrowing
- narrowed caller-visible result-shape digest
- `AuthorizedProjectionIdentity`
- `PolicyTenantAdmissionDigest`
- policy digest
- tenant truth basis digest
- tenant schema basis digest
- branch access digest
- relationship-proof admission digest
- validation report digest
- policy cost posture
- policy work budget digest
- Phase 2 counter snapshot digest
- support profile digest

Required invariants:

- the artifact cannot be constructed outside `policy_narrowing`
- masked projection cannot be widened after construction
- relationship-proof admission cannot be swapped after construction
- optimizer-facing selectors come only from `AuthorizedProjectionArtifact`
- saved-query/scope/template inputs produce the same artifact shape as direct
  construction after canonicalization
- all failure paths return typed `PolicyNarrowingError` values before artifact
  construction

Do not include raw policy documents, raw tenant IDs, raw branch strings, host
  callbacks, graph handles, entity payloads, or delivery payloads.

## Batch 6: Performance Encoding

Phase 2 must make performance a type-level and counter-level contract, not an
after-the-fact benchmark.

Required types:

- `PolicyNarrowingWorkBudget`
- `AuthorizedProjectionWorkBudget`
- `RelationshipProofWorkBudget`
- `PolicyNarrowingCostPosture`

Required budget dimensions:

- maximum canonical field references inspected
- maximum projected field count
- maximum masked field count
- maximum relationship-proof descriptor count
- maximum relationship-proof topology width
- maximum validation denial count retained
- maximum digest-part count

Required denial classes:

- `ProjectionBudgetExceeded`
- `MaskBudgetExceeded`
- `RelationshipProofBudgetExceeded`
- `UnknownNarrowingCost`
- `UnboundedDerivedInfluence`
- `UnboundedProofTopology`

Performance proof obligations:

- unknown or unbounded cost denies before narrowing succeeds
- validation walks typed references once and records exact inspected counts
- relationship-proof descriptor admission is bounded by descriptor count and
  topology width, not graph size
- final artifact digesting uses declared digest-part counts
- no Phase 2 test may use elapsed time as primary evidence

## Batch 7: Saved Query, Scope, Template, And View-Shape Integration

Saved and composed inputs must enter Phase 2 only after Milestone 8
canonicalization has produced the same canonical query/result-shape surface as
direct construction.

Required behavior:

- saved-query exact reuse may reuse Phase 1 admission only when Phase 1
  classified it as exact
- policy or tenant drift forces re-narrowing
- saved-query artifacts do not carry pre-authorized masked projections unless
  their policy/tenant/schema digests match exactly
- scope/template expansion is validated after expansion, not before
- view-shape fields are subject to the same hidden influence rules as ordinary
  projection, predicate, order, and grouping fields
- tenant schema mismatch denies rather than falling back to a global schema

Add explicit tests for:

- direct construction and saved-query construction narrowing to identical
  digests under exact policy/tenant reuse
- saved-query policy drift denying before narrowed artifact reuse
- template expansion introducing a masked predicate and denying before
  artifact construction
- view-shape grouping over a masked field denying before artifact construction

## Batch 8: Support Profile Honesty

Extend Milestone 9 support metadata with Phase 2 surfaces.

Required support statuses:

- `AuthorizedProjectionVerified`
- `MaskedInfluenceValidationVerified`
- `RelationshipProofDescriptorAdmissionVerified`
- `PolicyNarrowedArtifactVerified`
- `PolicyAwareExecutionDeferred`
- `PolicyAwareLiveDeferred`
- `PolicyAwareHistoricalDiffDeferred`
- `PolicyAwareDeliveryDeferred`
- `StoreBackedDurabilityBlockedOnForgeStore`

The support profile must make clear that Phase 2 proves pre-execution
narrowing only. It does not imply runtime proof evaluation, live parity,
historical diff parity, or durable resume.

## Batch 9: Unit Tests

Add focused unit tests close to each module.

`authorized_projection` tests:

- visible fields survive projection
- masked projection fields are omitted
- masked predicate denies
- masked ordering denies
- masked grouping denies
- non-disclosing use is allowed only when not emitted
- unknown derived influence denies
- counter snapshots are exact

`relationship_proof` tests:

- `DirectEdge` descriptor admits with bounded budget
- `BoundedAncestor` descriptor admits only with explicit bound
- `TenantMembership` descriptor binds tenant basis digest
- host callback descriptor denies
- unbounded recursive topology denies
- query-shape mismatch denies
- proof admission does not touch truth counters

`policy_narrowing` tests:

- narrowed artifact binds policy, tenant, branch, schema, projection, and proof
  identities
- optimizer-facing projection excludes masked fields
- saved exact reuse and direct construction produce equal artifact digests
- policy drift denies saved artifact reuse
- support profile advertises deferred execution surfaces
- unknown narrowing cost denies before artifact construction

## Batch 10: Compile-Fail Guards

Add trybuild cases that make naive bypasses impossible:

- `narrowed_policy_query_artifact_constructor_private.rs`
- `policy_narrowing_requires_admitted_context.rs`
- `authorized_projection_mask_mutation_forbidden.rs`
- `relationship_proof_host_callback_forbidden.rs`
- `relationship_proof_bare_bool_forbidden.rs`
- `optimizer_requires_narrowed_policy_query_artifact.rs`
- `masked_projection_cannot_be_used_as_authorized_projection.rs`
- `policy_aware_validation_report_constructor_private.rs`

Each compile-fail should guard a concrete future footgun, not just private
fields for their own sake.

## Batch 11: Certification Harness Rows

Extend `crates/forge-query/src/harness/milestone_nine_certification/` with
Phase 2 rows. Keep Phase 1 rows intact.

Required canonical rows:

- `authorized-projection-removes-masked-aspect`
- `non-disclosing-use-is-not-delivered`
- `relationship-proof-direct-edge-admission`
- `relationship-proof-tenant-membership-admission`
- `narrowed-artifact-binds-policy-tenant-schema`
- `optimizer-input-excludes-masked-fields`
- `saved-query-exact-reuse-narrows-identically`
- `phase-two-support-profile-honesty`

Required rejection rows:

- `masked-predicate-denies-before-narrowing`
- `masked-ordering-denies-before-narrowing`
- `masked-grouping-denies-before-narrowing`
- `relationship-proof-host-callback-forbidden`
- `relationship-proof-unbounded-recursion-denied`
- `relationship-proof-query-conflict-denied`
- `template-hidden-influence-denied`
- `saved-query-policy-drift-renarrowing-required`
- `unknown-narrowing-cost-denied-before-truth`
- `phase-two-no-truth-touch`

Required bundle fields:

- `query_digest`
- `policy_digest`
- `tenant_truth_basis_digest`
- `tenant_schema_basis_digest`
- `authorized_projection_digest`
- `narrowed_result_shape_digest`
- `relationship_proof_digest`
- `validation_report_digest`
- `failure_digest`
- `counter_snapshot`

Certification must include independent hostile lanes, not self-comparisons.

## Batch 12: Verification Commands

Run these incrementally while implementing:

```powershell
cargo test -p forge-query authorized_projection --lib
cargo test -p forge-query relationship_proof --lib
cargo test -p forge-query policy_narrowing --lib
cargo test -p forge-query milestone_nine_certification --lib
cargo test -p forge-query --test phase_boundaries_compile_fail
cargo test -p forge-query
```

If a narrower test name changes during implementation, update this plan before
closing the batch so the verification recipe remains executable.

## Non-Goals For This Batch

- no policy-aware execution plan
- no runtime graph proof evaluation
- no live maintenance
- no historical diff computation
- no delivery metadata emission
- no store-backed durable cursor or artifact persistence
- no broad relationship-proof language beyond the initial descriptor family
- no optimization rewrite that can observe masked fields
- no public artifact constructors for test convenience

## Done Criteria

This batch is complete when:

- `NarrowedPolicyQueryArtifact` exists and can only be built from an admitted
  Phase 1 context
- masked fields cannot enter optimizer-facing projection metadata
- hidden predicate, ordering, grouping, and derived-field influence fail typed
- one relationship-proof descriptor family has admitted and denied lanes
- proof admission is bounded and does not evaluate truth
- saved-query and composed inputs route through the same narrowing rules as
  direct construction
- support metadata names execution/live/diff/delivery as deferred
- compile-fail tests block raw constructors, callbacks, mutable masks, and
  raw-query bypasses
- certification rows emit stable digests and exact counters
- `cargo test -p forge-query` passes
