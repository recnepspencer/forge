# Query Operating Modes

## What This Feature Is

Query operating modes describe **how execution, artifacts, and subscriptions are backed** in the current runtimeâ€”not a second public API surface. Today the honest default is **runtime-backed**: plans, receipts, live state, and inspection evidence live in the in-process Query runtime. **Store-backed execution**, **durable cursors**, and **restart-stable subscription metadata** are explicit deferred debt, not implied by type names or facade exports.

Use this doc when you need to know whether a capability assumes process-local runtime state or durable store semantics.

## Why You Use It

- choose APIs that match what is actually admitted today
- avoid building restart/recovery flows on debt labeled `Deferred` or `BlockedOnWORTHStore`
- read support-matrix rows with the right mental model (`Verified` vs debt)
- explain why saved-query persistence and query-context reload behave as ephemeral/runtime-owned

## Core Mental Model

Three layers stack:

1. **Capability families** (`WorthQueryCapabilityFamily` in application support) â€” coarse lanes like `DurableArtifacts` marked deferred at the registry level.
2. **Runtime public support** (`WorthQueryRuntimePublicSupportMatrix` / `runtime/support/profile.rs`) â€” `StoreBackedExecution`, `DurableArtifacts`, and related rows.
3. **Lane-specific profiles** â€” saved-query (`runtime_backed_saved_query_support_profile`), query-context, subscription matrix rows.

**Runtime-backed** means admission, execution, and evidence are coherent inside the current workspace/runtime instance. **Store-backed** means reload, cross-process continuity, or worth-store durabilityâ€”largely **not** verified on the public path today.

## Main Entry Points

| Area | Symbols / modules |
|------|-------------------|
| Capability registry | `application/support/registry.rs` â€” `WorthQueryCapabilityFamily` |
| Runtime profile | `runtime/support/profile.rs` â€” `StoreBackedExecution`, `DurableArtifacts` |
| Saved queries | `saved_query/support.rs` â€” `SavedQueryPersistenceFamily::EphemeralProcessOwned` (debt) |
| Query context | `query_context/support.rs` â€” reload/store rows |
| Facade workspace | `WorthQueryRuntime`, `WorthQueryWorkspace` â€” same runtime instance for compose/execute/live |

There is no separate `query_operating_mode()` switch on the facade; mode is **derived from support profiles** and admission receipts, not a user-facing enum you set at startup.

## Typical Flow

1. Open or build a `WorthQueryRuntime` / workspace (runtime-backed instance).
2. Admit and execute through normal compose/execute or live/subscription lanes.
3. If you need durability, read `runtime_backed_*_support_profile()` for the lane **before** assuming persistence.
4. For saved queries or query context, treat process-owned artifacts as **convenience**, not restart contracts, until store-backed rows flip to `Verified`.

```text
Host app
  â””â”€â”€ WorthQueryRuntime (process-local)
        â”œâ”€â”€ plans / receipts / live graph
        â”œâ”€â”€ inspection evidence (retained)
        â””â”€â”€ [deferred] worth-store reload / durable cursors
```

## How It Relates

- [Support matrix and admission](support-matrix-and-admission.md) â€” authoritative Verified / Deferred / Forbidden tables
- [Saved query and query context](../authoring/read-composition.md) â€” authoring that may imply persistence; check profiles first
- [Subscription selection and diagnostics](../capabilities/subscription-selection-and-diagnostics.md) â€” durable replay deferred in subscription matrix
- [Basis capability lifecycle](../capabilities/basis-capability-lifecycle.md) â€” basis envelopes: store-backed reload deferred

## Good to Know

- API names containing â€œsavedâ€, â€œcontextâ€, or â€œsubscriptionâ€ do **not** guarantee worth-store durability.
- Certification and harness tests often run runtime-backed; do not generalize to multi-process deployment without checking matrix rows.
- `DurableArtifacts` at the capability-family level is deferredâ€”treat durable artifact stories as roadmap debt unless a lane-specific profile says `Verified`.

## Anti-Patterns

- Assuming subscription identity survives process restart without checking subscription support matrix.
- Building â€œwrite to DB then reload query context from storeâ€ on `BlockedOnWORTHStore` rows.
- Treating `Debt` or `Deferred` in a profile as â€œworks in dev onlyâ€â€”it means **not a supported public contract** yet.

## Current Limits

From application/runtime/saved-query/query-context support profiles (representative rows):

| Surface / family | Status |
|------------------|--------|
| Runtime-backed execution, live, inspection | **Verified** (default honest path) |
| `WorthQueryCapabilityFamily::DurableArtifacts` | **Deferred** |
| `StoreBackedExecution` (runtime public matrix) | **Deferred** / store-blocked neighbors |
| Saved query `EphemeralProcessOwned` persistence | **Debt** |
| Query-context store reload | **Deferred** / blocked per `query_context/support.rs` |
| Durable subscription replay metadata | **Deferred** (see subscription matrix) |

Temporal/time-aware operating extensions are **not** a separate mode docâ€”see [support matrix](support-matrix-and-admission.md) until 9.4+ lanes ship.

## Related Docs

- [Support matrix and admission](support-matrix-and-admission.md)
- [Policy, tenant, and relationship-proof narrowing](policy-tenant-and-relationship-proof-narrowing.md)
- [Region-scoped live invalidation and stream contracts](../runtime-surfaces/region-scoped-live-invalidation-and-stream-contracts.md)
- [AI agent orientation](../AI_README.md) â€” Operating modes category
