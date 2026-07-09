# Lower-Runtime Capability Routing

## What This Feature Is

Lower-runtime capability routing is Queryâ€™s **declared path for contacting** relational, bridge, and signal runtimes: route plans, boundary envelopes, eligibility receipts, and `worth_query_lower_runtime_support_matrix()`â€”without domain code importing lower crates directly. **Compatibility debt** rows still exist; `TemporalQueryBasisRoutingNeighbor` is **deferred**.

## Why You Use It

- route live, subscription, write-authority, and signal invalidation through boundary receipts
- inspect closeout and boundary summaries for agents (`inspect_lower_runtime_boundary`)
- stay on certification-tested public inventory rows
- avoid bypassing envelopes to call `worth-relational` / `worth-signal` from domain modules

## Core Mental Model

`lower_runtime_routing/` modules:

| Piece | Role |
|-------|------|
| `plans/` / `protocol/` | Route plans and protocol shapes |
| `envelopes/` | `WorthQueryLowerRuntimeBoundaryEnvelope`, cost/failure topology |
| `eligibility/` | Capability eligibility and posture |
| `receipts/` | Boundary receipts (live, subscription, write authority, signal) |
| `support/` | `worth_query_lower_runtime_support_matrix()` |
| `dx/` | `inspect_lower_runtime_boundary`, `summarize_lower_runtime_boundary` |

This is the **execution contact** layer; [declaration bridge continuation routing](declaration-bridge-continuation-routing.md) covers declaration-entry continuationâ€”link both, do not merge.

```text
Domain declaration (no lower imports)
  â†’ lower_runtime route plan + eligibility
  â†’ boundary envelope + receipt
  â†’ relational / bridge / signal runtime
```

## Main Entry Points

Public exports (`lower_runtime_routing/mod.rs`):

- Adapters: `LiveViewDeclarationAdmissionBoundaryReceipt`, `SubscriptionActivationBoundaryReceipt`, `WriteAuthorityExecutionReceipt`, `SignalInvalidationBoundaryReceipt`
- DX: `inspect_lower_runtime_boundary`, `WorthQueryLowerRuntimeRoutingInspection`, `WorthQueryLowerRuntimeBoundarySummary`
- Support: `worth_query_lower_runtime_support_matrix()`
- Certification suite types (for harness/CI, not app hot path)

Tests: `lower_runtime_routing/certification/tests.rs`, UI `tests/ui/lower_runtime_routing/`.

## Typical Flow

1. Admit domain capability with lower-runtime eligibility for the target family.
2. Build route plan with boundary envelope category and cost posture.
3. Execute through boundary receipt (live view, subscription activation, or write authority).
4. On failure, read failure topology from envelopeâ€”do not bypass to raw runtime APIs.
5. For diagnostics: `inspect_lower_runtime_boundary` / `summarize_lower_runtime_boundary`.

## How It Relates

- [Declaration bridge continuation routing](declaration-bridge-continuation-routing.md) â€” continuation at declaration entry
- [Live views](../runtime-surfaces/live-views.md) â€” live admission consumes boundary receipts
- [Region-scoped live](../runtime-surfaces/region-scoped-live-invalidation-and-stream-contracts.md) â€” locality + stream lowering
- [Cross-runtime causal inspection](../capabilities/cross-runtime-causal-inspection.md) â€” materialized detail vs bridge envelopes

## Good to Know

- `certify_lower_runtime_routing` and acceptance suite are certification artifactsâ€”apps use receipts and support matrix, not closeout manifests, in production paths.
- Compile-fail boundary digests in certification enforce **non-bypass** from domain code.
- UI golden transcripts stabilize inspection DX for agents.

## Anti-Patterns

- `use worth_relational::*` (or signal/bridge) inside domain capability modules.
- Treating routing as permission to skip Query admission/support profiles.
- Assuming temporal query basis routing works because a neighbor name existsâ€”check deferred row.

## Current Limits

From `worth_query_lower_runtime_support_matrix()` (see `lower_runtime_routing/support.rs` for full rows):

| Neighbor / concern | Status |
|--------------------|--------|
| Relational / bridge / signal boundary contact (runtime-backed) | **Admitted** on verified rows |
| Compatibility debt families | **Debt** â€” check matrix before claiming parity |
| `TemporalQueryBasisRoutingNeighbor` | **Deferred** |
| Direct lower-crate import from domains | **Forbidden** (certification non-bypass) |

## Related Docs

- [Declaration bridge continuation routing](declaration-bridge-continuation-routing.md)
- [Support matrix and admission](../foundations/support-matrix-and-admission.md)
- [Contribution composed orchestration](contribution-composed-orchestration.md)
- [Public doc coverage](public-doc-coverage.md)
