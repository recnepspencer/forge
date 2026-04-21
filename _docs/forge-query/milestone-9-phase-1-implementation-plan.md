# Milestone 9 Phase 1 Implementation Plan: Policy And Tenant Context Admission

> **Parent spec:** [milestone-9.md](./milestone-9.md)
>
> **Phase:** Phase 1 only
>
> **Purpose:** build the proof-bearing admission layer that freezes policy,
> tenant truth basis, tenant schema basis, branch access, execution-mode
> admissibility, and saved-query policy/tenant reuse classification before any
> masking, relationship-proof lowering, planning, execution, live maintenance,
> historical diffing, or delivery metadata exists.

## Phase 1 Goal

Phase 1 creates the admission artifacts that every later Milestone 9 phase must
consume.

It must produce:

- `PolicyBasis` tied to canonical query identity and caller/policy context
- `TenantTruthBasis` tied to the branch-backed tenant truth basis
- `TenantSchemaBasis` tied to the schema basis used for validation
- typed branch, tenant, and execution-mode denial families
- saved-query policy/tenant reuse classification
- support metadata and certification rows proving those admissions are the only
  way into later phases

It must not produce:

- masked projections
- authorized result shapes
- relationship-proof execution
- policy-aware execution plans
- one-shot/live/historical execution
- delivery metadata
- store-backed durable artifact semantics

## Hard Boundary

The Phase 1 artifact is an admitted context, not a narrowed query.

The implementation should stop at:

```rust
RawPolicyAwareIntent
    -> AdmittedPolicyTenantContext
```

Phase 2 starts at:

```rust
AdmittedPolicyTenantContext
    -> NarrowedPolicyQueryArtifact
```

Any patch that reads truth, inspects entity payloads, masks projection fields,
evaluates relationship-proof chains, derives live relevance, or builds delivery
metadata is out of Phase 1.

## Proposed Module Topology

Add the Phase 1 modules under `crates/forge-query/src/policy_basis` and
`crates/forge-query/src/tenant_basis` first. Do not create one large
`policy.rs`.

Recommended files:

```text
crates/forge-query/src/policy_basis/
  mod.rs
  authority.rs
  admission.rs
  artifacts.rs
  counters.rs
  digest.rs
  errors.rs
  saved_reuse.rs
  support.rs
  tests.rs

crates/forge-query/src/tenant_basis/
  mod.rs
  authority.rs
  admission.rs
  artifacts.rs
  counters.rs
  digest.rs
  errors.rs
  support.rs
  tests.rs

crates/forge-query/src/harness/policy_tenant_admission_certification/
  mod.rs
  fixtures.rs
  matrix.rs
  row_catalog.rs
  rows.rs
  tests.rs
```

`policy_basis` owns policy identity and admission. `tenant_basis` owns tenant
truth/schema basis identity and tenant admission. The certification harness owns
only proof rows and fixtures.

Later phases may add `authorized_projection`, `relationship_proof`,
`policy_plan`, `policy_execution`, and `policy_delivery`. Do not create those
early unless the file contains only type placeholders needed for Phase 1
compile boundaries.

## Batch 1: Authority Input Artifacts

Create explicit input snapshots that lower systems may construct and query may
consume.

Required types:

- `PolicyRuleSnapshot`
- `TenantBindingSnapshot`
- `BranchAccessGrant`
- `SchemaVariantSnapshot`

Each must carry:

- stable identity digest
- owning authority family
- epoch where relevant
- enough structured fields for admission to classify without reaching into
  host/session state
- private fields and crate-owned constructors where fabrication matters
- read-only accessors only

Initial allowed constructors may be crate-owned fixture constructors if lower
policy/platform crates do not exist yet. They must be named as synthetic or
fixture authority constructors and must not look like production middleware.

Do not accept:

- raw `user_id`
- raw `tenant_id`
- raw branch strings
- auth callbacks
- session structs
- middleware handles
- schema handles without an admitted `SchemaVariantSnapshot`

## Batch 2: Policy Basis Admission

Implement policy basis admission in `policy_basis::admission`.

Required output:

- `PolicyBasis`
- `PolicyBasisIdentity`
- `PolicyEpoch`
- `PolicyAdmissionDisposition`
- `PolicyTenantAdmissionError` or policy-specific equivalent
- `PolicyBasisCounters`

Required disposition variants:

- `AdmittedUnchanged`
- `AdmittedNarrowed`
- `AdmittedWithNonDisclosingUse`
- `Denied`

For Phase 1, `AdmittedNarrowed` and `AdmittedWithNonDisclosingUse` may classify
intent but must not create masked projections or non-disclosing field-use
witnesses. Those belong to Phase 2. If the code cannot represent that cleanly,
use a Phase 1-specific `PendingNarrowing` payload inside the disposition.

Required denials:

- missing policy basis
- incompatible policy/query family
- unsupported execution-mode composition
- branch access denied where policy owns the denial
- raw callback or middleware policy source forbidden

Counters must distinguish:

- policy basis admitted
- policy basis denied
- branch access denied
- unsupported execution mode denied
- raw middleware/callback source denied

## Batch 3: Tenant Truth And Schema Basis Admission

Implement tenant basis admission in `tenant_basis::admission`.

Required output:

- `TenantTruthBasis`
- `TenantSchemaBasis`
- `TenantTruthBasisIdentity`
- `TenantSchemaBasisIdentity`
- `TenantBasisEpoch`
- `TenantResolutionClass`
- `TenantBasisCounters`

Required `TenantResolutionClass` variants:

- `DirectBinding`
- `CachedBinding`
- `DerivedBinding`

Phase 1 admits:

- `DirectBinding`
- `CachedBinding` only when the snapshot already carries explicit truth and
  schema basis identity

Phase 1 denies or marks explicit debt:

- `DerivedBinding`
- ambiguous tenant context
- hidden tenant filter
- tenant truth basis without paired schema basis
- tenant schema basis without paired truth basis
- global schema fallback when tenant schema disagrees

Counters must distinguish:

- direct tenant binding admitted
- cached tenant binding admitted
- derived binding denied
- ambiguous tenant denied
- hidden tenant filter denied
- global schema fallback denied

## Batch 4: Combined Policy/Tenant Admission Artifact

Implement the combined context artifact that later phases consume.

Required output:

- `AdmittedPolicyTenantContext`
- `PolicyTenantAdmissionBundle`
- `PolicyTenantAdmissionDigest`
- `PolicyTenantAdmissionCounters`
- `PolicyTenantAdmissionFailureClass`

The bundle must include:

- canonical query digest
- policy digest
- policy epoch
- tenant truth basis digest
- tenant schema basis digest
- tenant basis epoch
- branch access digest
- schema variant digest
- execution-mode family requested
- admission disposition
- counters

It must not include:

- entity payloads
- projected field values
- masked projection entries
- relationship-proof success/failure payloads
- delivery metadata

The public API should look like:

```rust
pub fn admit_policy_tenant_context(
    query: CanonicalComposedArtifact,
    policy: PolicyRuleSnapshot,
    tenant: TenantBindingSnapshot,
    branch: BranchAccessGrant,
    schema: SchemaVariantSnapshot,
    mode: PolicyExecutionModeRequest,
) -> Result<AdmittedPolicyTenantContext, PolicyTenantAdmissionError>;
```

If the exact canonical artifact type is awkward to depend on, create a narrow
adapter that exposes only the canonical query digest, result-shape family, and
composition identity needed for Phase 1. Do not depend on raw authored query
state.

## Batch 5: Saved-Query Policy/Tenant Reuse Classification

Extend the saved-query reuse surface without changing saved-query artifact
authority.

Required output:

- `SavedQueryPolicyReuseDescriptor`
- `SavedQueryPolicyReuseDisposition`
- `PolicyReuseEquivalenceContract`
- matrix rows for policy basis, tenant truth basis, tenant schema basis, branch
  access, schema variant, and execution-mode family

Required classifications:

- `LegalNoSemanticChange`
- `LegalRequiresFreshFreeze`
- `IllegalSemanticDrift`

Rules:

- unchanged policy/tenant/schema/branch digests are `LegalNoSemanticChange`
- compatible but semantically visible basis changes are
  `LegalRequiresFreshFreeze`
- branch denial, tenant ambiguity, schema incompatibility, execution-mode
  mismatch, and missing equivalence evidence are `IllegalSemanticDrift`
- saved-query reuse may not jump directly to Phase 2; it must produce or
  re-enter `AdmittedPolicyTenantContext`

Prefer adding Milestone 9-specific reuse descriptors beside existing
`saved_query::reuse` rather than rewriting Milestone 8 reuse logic.

## Batch 6: Support Metadata

Add support reporting for Phase 1 surfaces.

Required support rows:

- policy basis admission
- tenant truth basis admission
- tenant schema basis admission
- branch access admission
- saved-query policy/tenant reuse classification
- deferred relationship-proof lowering
- deferred authorized projection
- deferred execution seam parity
- deferred delivery metadata
- deferred durable store-backed artifacts

Support metadata must be derived from executable admission behavior or an
explicit registry/matrix, not from doc prose.

## Batch 7: Unit Tests

Unit tests should live next to the new subdomains.

Minimum policy tests:

- admits explicit policy snapshot for canonical query digest
- denies raw callback/middleware policy source
- denies unsupported execution-mode composition
- produces stable policy digest for equivalent input
- distinguishes `AdmittedUnchanged`, `AdmittedNarrowed`,
  `AdmittedWithNonDisclosingUse`, and `Denied` without bool shortcuts

Minimum tenant tests:

- admits direct tenant truth/schema basis pair
- admits cached tenant basis only with explicit basis identities
- denies derived binding
- denies ambiguous tenant
- denies hidden tenant filter
- denies tenant truth basis without tenant schema basis
- denies tenant schema basis without tenant truth basis
- denies global schema fallback

Minimum combined-context tests:

- emits bundle with all required digests
- does not expose entity payloads or projection internals
- carries exact counters for admitted context
- carries exact counters for each denial class
- cannot be constructed from raw authored query state

Minimum saved-query tests:

- unchanged policy/tenant basis is `LegalNoSemanticChange`
- changed but compatible policy/tenant basis requires fresh freeze
- incompatible tenant schema is `IllegalSemanticDrift`
- branch denial is `IllegalSemanticDrift`
- reuse re-enters admission rather than bypassing it

## Batch 8: Compile-Fail Boundaries

Add `trybuild` cases under `crates/forge-query/tests/ui`.

Minimum compile-fail files:

- `policy_rule_snapshot_raw_constructor_forbidden.rs`
- `tenant_binding_snapshot_raw_constructor_forbidden.rs`
- `admitted_policy_tenant_context_constructor_private.rs`
- `policy_tenant_context_requires_canonical_query.rs`
- `policy_tenant_context_rejects_raw_tenant_id.rs`
- `policy_tenant_context_rejects_auth_callback.rs`
- `tenant_basis_pair_cannot_be_split.rs`
- `policy_admission_bool_shortcut_forbidden.rs`
- `saved_query_policy_reuse_bypass_forbidden.rs`
- `phase_two_narrowing_requires_admitted_policy_tenant_context.rs`

Update `crates/forge-query/tests/phase_boundaries_compile_fail.rs` to include
the new files.

Compile-fail target:

- external code cannot fabricate authority inputs
- external code cannot fabricate `AdmittedPolicyTenantContext`
- external code cannot call Phase 2 entrypoints without an admitted context
  once Phase 2 stubs exist
- external code cannot represent policy/tenant admission as `bool`
- saved-query reuse cannot bypass fresh policy/tenant admission

## Batch 9: Phase 1 Certification Harness

Add `harness/policy_tenant_admission_certification`.

Minimum row catalog:

- `policy-basis-explicitness`
- `tenant-truth-schema-pair-explicitness`
- `branch-denial-before-truth`
- `derived-tenant-resolution-forbidden`
- `hidden-tenant-filter-forbidden`
- `global-schema-fallback-forbidden`
- `unsupported-execution-mode-denied`
- `saved-query-policy-tenant-rebinding-classification`
- `phase-one-no-truth-touch`
- `support-profile-honesty`

Required bundle fields:

- `query_digest`
- `policy_digest`
- `tenant_basis_digest`
- `schema_basis_digest`
- `branch_access_digest`
- `admission_digest`
- `failure_digest`
- `support_matrix_digest`
- `counter_snapshot`

Certification must prove:

- Phase 1 produces admission artifacts only
- denied lanes fail before truth touch
- support metadata matches real admission behavior
- saved-query policy/tenant reuse classification cannot bypass admission
- deferred Phase 2+ surfaces are advertised as deferred, not supported

## Batch 10: Facade Exposure

Expose only the minimum public surface needed for ordinary consumers and tests.

Recommended approach:

- keep `policy_basis` and `tenant_basis` modules private in `lib.rs`
- expose public types through `facade` or a narrow policy/tenant admission
  facade consistent with existing crate style
- do not expose internal constructors
- do not expose relationship-proof lowering, masking, planning, execution,
  live, historical diff, or delivery entrypoints in Phase 1

If adding public facade methods, prefer explicit names:

- `admit_policy_tenant_context`
- `classify_saved_query_policy_tenant_reuse`

Avoid:

- `with_policy(...)`
- `authorize(...) -> bool`
- `tenant(...)`
- `policy(...)`
- `execute_authorized(...)`
- any API that implies execution or masking already exists

## Batch 11: Verification Commands

Run at minimum:

```powershell
cargo test -p forge-query policy_basis -- --test-threads=1
cargo test -p forge-query tenant_basis -- --test-threads=1
cargo test -p forge-query policy_tenant_admission_certification -- --test-threads=1
cargo test -p forge-query --test phase_boundaries_compile_fail -- --test-threads=1
```

If module names differ, use the final test filters that cover the same
surfaces.

Before closeout, also run:

```powershell
cargo test -p forge-query -- --test-threads=1
```

## Phase 1 Closeout Gate

Phase 1 is complete only when:

- authority input artifacts exist and are sealed
- policy basis admission exists
- tenant truth/schema basis admission exists
- combined `AdmittedPolicyTenantContext` exists
- branch/tenant/execution-mode denial families are typed
- saved-query policy/tenant reuse classification exists
- support metadata reports admitted/deferred/denied surfaces honestly
- unit tests cover admitted and denied lanes
- compile-fail tests seal fabrication and bypass paths
- certification emits canonical machine-checkable bundles
- no Phase 2 behavior has been smuggled in

Phase 1 is not complete if:

- policy can be represented as a callback, middleware hook, or bool
- tenant context can be represented as a raw tenant ID or hidden filter
- tenant truth basis and tenant schema basis can be split
- saved-query reuse can bypass fresh policy/tenant admission
- any code touches truth, masks fields, evaluates relationship proofs, plans
  execution, derives live relevance, computes diffs, or emits delivery metadata

## Suggested Implementation Order

1. Add module skeletons and private `mod` entries in `lib.rs`.
2. Implement authority input snapshot types and digests.
3. Implement policy basis admission and tests.
4. Implement tenant truth/schema basis admission and tests.
5. Implement combined `AdmittedPolicyTenantContext` and counters.
6. Implement saved-query policy/tenant reuse classification.
7. Add support metadata rows.
8. Add compile-fail tests.
9. Add certification harness rows.
10. Run focused tests, then full `forge-query` tests.
11. Write Phase 1 implementation parity note or closeout note after tests pass.

## Open Implementation Questions

- Should Phase 1 expose the admission API through the existing broad
  `facade.rs` or through the newer application capability facade?
- Should `PolicyRuleSnapshot` and `TenantBindingSnapshot` live entirely inside
  `forge-query` for now as synthetic authority snapshots, or should they be
  adapter shells over lower/platform-owned types once those exist?
- Should saved-query policy/tenant reuse extend `saved_query::reuse` directly
  or live in `policy_basis::saved_reuse` and call into the existing saved-query
  reuse matrix?

Default recommendation:

- keep the first implementation local and synthetic but explicitly named as
  authority snapshots;
- expose only a narrow facade;
- keep saved-query policy/tenant reuse in `policy_basis::saved_reuse` until the
  shape proves stable.
