# Basis Capability Lifecycle

## What This Feature Is

The basis capability lifecycle is the **phase-typed pipeline** for turning raw basis intent into admissible, scoped, usable capability tokens: normalize → eligibility → admit → scope → use receipt. It covers current, branch, preview, historical, tenant, and policy lanes on the **runtime-backed** path. **Store-backed reload** and **durable basis envelopes** are deferred; **temporal basis intent is denied** today (pointer to support matrix, not a temporal feature doc).

## Why You Use It

- admit basis without skipping eligibility or scoping phases
- reuse basis across query-context and saved-query flows with honest reuse classification
- obtain receipts that downstream compose/execute/live lanes can consume
- avoid treating “I passed a string basis” as a fully scoped capability token

## Core Mental Model

Two module trees cooperate:

| Tree | Role |
|------|------|
| `basis_lifecycle/` | Core phase APIs, lanes, receipts, `basis_lifecycle_support_matrix()` |
| `query_basis_lifecycle/` | Query-context lane adapters |

Phases are **ordered obligations**, not optional helpers. A token from an earlier phase does not automatically imply later surfaces (e.g. historical materialization) are admitted.

```text
RawBasisIntent
  → normalize_raw_basis_intent
  → eligibility (counters / denial)
  → admit (lane-specific)
  → scope (world / tenant / policy bindings)
  → use receipt → compose / execute / live
```

## Main Entry Points

Facade and `basis_lifecycle` exports (representative):

- `normalize_raw_basis_intent`, `RawBasisIntent`
- Admission and scoping types under `basis_lifecycle::admission`, `scoping`, `receipts`
- `basis_lifecycle_support_matrix()` in `basis_lifecycle/support.rs`
- Query-context: `query_basis_lifecycle/` adapters
- DX: inspection/summarize helpers in `basis_lifecycle::dx`

Tests: `basis_lifecycle/tests.rs`, `query_basis_lifecycle/tests.rs`, UI `tests/ui/basis_lifecycle/`.

## Typical Flow

1. Provide raw basis intent for the lane (current, branch, preview, historical, tenant, policy).
2. `normalize_raw_basis_intent` → normalized intent artifact.
3. Run eligibility; handle denials with explicit counters/receipts.
4. Admit into the lane; receive admission receipt.
5. Scope to the query world/tenant/policy binding required for the next operator.
6. Attach **use receipt** to read composition or live admission that consumes the basis.

Historical **diff** semantics stay in [historical diff and basis](historical-diff-and-basis.md); this doc owns **lifecycle phases and tokens**.

## How It Relates

- [Historical diff and basis](historical-diff-and-basis.md) — diff/historical materialization, not phase typing
- [Policy, tenant, and relationship-proof narrowing](../foundations/policy-tenant-and-relationship-proof-narrowing.md) — policy/tenant basis lanes
- [Intent admission and observation](../foundations/intent-admission-and-observation.md) — observation intent vs basis phases
- [Query operating modes](../foundations/query-operating-modes.md) — store-backed basis reload deferred

## Good to Know

- Current/branch/preview/historical/tenant/policy lanes are largely **admitted** on runtime-backed paths per `basis_lifecycle_support_matrix()`.
- Temporal basis intent: **denied**—do not plan time-travel basis until matrix and milestone ship a public lane.
- UI golden tests (`tests/ui/basis_lifecycle/`) document stable DX shapes for agents and humans.

## Anti-Patterns

- Skipping scope after admit because “the basis string looks right.”
- Using historical diff APIs without a scoped historical basis use receipt.
- Creating a separate temporal basis doc or API assumption while temporal intent remains denied.

## Current Limits

From `basis_lifecycle_support_matrix()` (representative; see source for full rows):

| Lane / capability | Status |
|-------------------|--------|
| Current, branch, preview, historical, tenant, policy (runtime-backed phases) | **Admitted** / verified neighbors |
| Store-backed basis reload | **Deferred** |
| Durable basis envelopes | **Deferred** |
| Temporal basis intent | **Denied** — see [support matrix](../foundations/support-matrix-and-admission.md) |

## Related Docs

- [Historical diff and basis](historical-diff-and-basis.md)
- [Support matrix and admission](../foundations/support-matrix-and-admission.md)
- [Policy, tenant, and relationship-proof narrowing](../foundations/policy-tenant-and-relationship-proof-narrowing.md)
- [Read composition](../authoring/read-composition.md)
