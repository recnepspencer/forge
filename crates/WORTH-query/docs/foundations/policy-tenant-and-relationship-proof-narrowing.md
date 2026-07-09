# Policy, Tenant, and Relationship-Proof Narrowing

## What This Feature Is

Policy and tenant narrowing **shrinks what a query may see or assert** under declared policy/tenant basis and relationship-proof descriptorsâ€”before and during read composition. The shipped path is **runtime-backed descriptor admission and masking validation**, not full policy-aware execution/live/historical parity across every surface.

Relationship-proof admission validates **descriptors** on the runtime-backed path; **host authorization callbacks** are forbidden on the public contract.

## Why You Use It

- attach policy/tenant basis to queries without bypassing admission
- produce narrowed policy query artifacts and authorized projection receipts
- admit relationship-proof descriptors with an honest support profile
- avoid conflating â€œpolicy narrowed at plan timeâ€ with â€œevery live/historical lane is policy-aware todayâ€

## Core Mental Model

Four cooperating areas in the crate (not four separate apps):

| Module area | Role |
|-------------|------|
| `policy_narrowing/` | Masking, influence validation, narrowed policy query artifacts |
| `policy_basis/` / `tenant_basis/` | Basis tokens tied to policy/tenant lanes |
| `relationship_proof/` | Descriptor admission, runtime-backed proof profile |
| Read composition | Operator receipts that record policy/tenant narrowing outcomes |

Narrowing **reduces** admissible shape; it does not replace domain authorization in host code. Store-backed policy durability is **blocked on worth-store**, not a silent fallback.

## Main Entry Points

Facade (`exports_policy.rs` and related):

- `narrow_policy_query` â€” narrowed policy query artifact path
- `admit_policy_tenant_context` â€” policy/tenant context admission
- `classify_saved_query_policy_tenant_reuse` / `classify_saved_policy_narrowing_reuse` â€” reuse classification
- `admit_relationship_proofs` â€” relationship-proof descriptor admission
- `runtime_backed_relationship_proof_support_profile()` â€” honesty table

Read composition records narrowing via runtime surface receipts (`runtime/surface/read_composition.rs`); see [read composition](../authoring/read-composition.md) for compose/executeâ€”**policy detail lives here**.

## Typical Flow

1. Declare or load policy/tenant basis (see [basis capability lifecycle](../capabilities/basis-capability-lifecycle.md)).
2. `admit_policy_tenant_context` (or saved-query reuse classification) for the workspace posture.
3. `narrow_policy_query` where the plan requires a narrowed policy artifact.
4. For relationship proofs: `admit_relationship_proofs` with descriptor admission onlyâ€”no host callback channel.
5. Execute read composition; inspect receipts for masking/authorized projection outcomes.

```text
Basis (policy/tenant)
  â†’ admit context / narrow policy query
  â†’ relationship-proof descriptors (if any)
  â†’ read composition execute
  â†’ receipts (masking, projection admission)
```

## How It Relates

- [Read composition](../authoring/read-composition.md) â€” execution surface; cross-link for policy receipts only
- [Basis capability lifecycle](../capabilities/basis-capability-lifecycle.md) â€” phase-typed basis tokens
- [Intent admission and observation](intent-admission-and-observation.md) â€” intent families vs policy-aware execution debt
- [Support matrix and admission](support-matrix-and-admission.md) â€” `PolicyTenantAdmissionSupportProfile`

## Good to Know

- **Verified** today: authorized projection, masked influence validation, relationship-proof **descriptor** admission, narrowed policy query artifacts (runtime-backed).
- **Deferred**: policy-aware execution, live, historical diff, deliveryâ€”parity across surfaces is not shipped.
- **BlockedOnWORTHStore**: store-backed durability for policy narrowing.
- Relationship-proof does not expose a public â€œcall my authz hookâ€ API; descriptors + runtime profile define the contract.

## Anti-Patterns

- Assuming live subscriptions automatically honor the same policy narrowing as a one-shot read plan.
- Using relationship-proof admission as proof that an external IAM system approved the request.
- Importing store APIs from domain code to â€œpersist policy contextâ€ while matrix rows are blocked.

## Current Limits

`runtime_backed_policy_narrowing_support_profile()` (`policy_narrowing/support.rs`):

| Surface | Status |
|---------|--------|
| AuthorizedProjection | **Verified** |
| MaskedInfluenceValidation | **Verified** |
| RelationshipProofDescriptorAdmission | **Verified** |
| NarrowedPolicyQueryArtifact | **Verified** |
| PolicyAwareExecution | **Deferred** |
| PolicyAwareLive | **Deferred** |
| PolicyAwareHistoricalDiff | **Deferred** |
| PolicyAwareDelivery | **Deferred** |
| StoreBackedDurability | **BlockedOnWORTHStore** |

Relationship-proof: descriptors **verified** on runtime-backed profile; host authorization callbacks **forbidden** on public API.

## Related Docs

- [Read composition](../authoring/read-composition.md)
- [Basis capability lifecycle](../capabilities/basis-capability-lifecycle.md)
- [Support matrix and admission](support-matrix-and-admission.md)
- [Historical diff and basis](../capabilities/historical-diff-and-basis.md) â€” historical lane; policy-aware historical diff deferred
