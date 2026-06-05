# Authority-Scoped Effect Execution

## What This Feature Is

Authority-scoped effect execution is the **effect lifecycle phase pipeline**: eligibility, admission, authority ownership, lowering, and execution receipts for effect families (writeback, mutation, merge, delivery neighbors) tied to **basis families**. [Effects](effects.md) covers **authoring and DX** (`workspace.effect`, triggers, staging); this doc owns **lowering, execution, and support-matrix honesty**.

## Why You Use It

- execute admitted effect intents with correct authority owner and receipt kinds
- read `effect_lifecycle_support_matrix()` before claiming store/durable effect paths
- separate staged write-intent from executed writeback on bridge-backed paths
- align agents with `discover_effect_lifecycle_support` postures per basis/effect pair

## Core Mental Model

Phases in `effect_lifecycle/` (representative):

```text
discover_effect_lifecycle_support(basis, effect)
  → evaluate_effect_eligibility
  → admit_effect_intent
  → lower + execute (authority-scoped)
  → EffectReceiptArtifactKind
```

Postures: `Admitted`, `Advisory`, `Denied`, `RebindRequired`, `Deferred`, `Unsupported`. Causes include `StoreBackedExecutionDeferred`, `DurableReplayDeferred`, `PreviewRebindRequired`, `BranchAuthorityRequired`.

## Main Entry Points

- `effect_lifecycle_support_matrix()`, `discover_effect_lifecycle_support`
- `admit_effect_intent`, `evaluate_effect_eligibility`
- Effect taxonomy: `EffectFamily`, `BasisFamily` pairing in support rows
- Execution tests: `effect_lifecycle/tests/execution/`
- Authoring surface: [effects.md](effects.md) — `workspace.effect`, `next_effect_write_intent`

## Typical Flow

1. Author effect via `workspace.effect` (triggers, delivery, write-intent staging).
2. When consuming staged work, route through intent admission (see [intent admission](../foundations/intent-admission-and-observation.md)).
3. `discover_effect_lifecycle_support` for the basis + effect family.
4. If admitted: eligibility → admit → execute with authority owner receipt.
5. Inspect effect artifacts; do not assume durable replay without matrix row.

## How It Relates

- [Effects](effects.md) — declaration, inspection, staging DX
- [Authoritative mutation evidence](../capabilities/authoritative-mutation-evidence.md) — write provenance vs effect delivery
- [Basis capability lifecycle](../capabilities/basis-capability-lifecycle.md) — basis phases before effect admit
- [Intent admission and observation](../foundations/intent-admission-and-observation.md) — consuming staged write-intent

## Good to Know

- `StoreBacked` / `DurableReload` basis families often yield **Deferred** causes for writeback neighbors.
- `Preview` + `Mutation` may require **RebindRequired** before execution.
- Advisory-only execution is explicit—do not treat as full authority-scoped execute.

## Anti-Patterns

- Documenting “full effect pipeline” in effects.md without checking lifecycle matrix.
- Executing merge/writeback families on basis lanes marked `UnsupportedForBasisFamily`.
- Using effect declaration alone as proof of backend writeback completion.

## Current Limits

From `effect_lifecycle_support_matrix()` / `discover_effect_lifecycle_support` (representative):

| Cause / posture | Meaning |
|-----------------|--------|
| `StoreBackedExecutionDeferred` | Store-backed effect execution **deferred** |
| `DurableReplayDeferred` | Durable replay **deferred** |
| `AdvisoryOnlyExecution` | Execute path **advisory** only |
| `PreviewRebindRequired` | Rebind before execute |
| `Denied` / `Unsupported` | Do not execute on public contract |

See `effect_lifecycle/support_matrix_rows.rs` for the full row inventory.

## Related Docs

- [Effects](effects.md)
- [Support matrix and admission](../foundations/support-matrix-and-admission.md)
- [Query operating modes](../foundations/query-operating-modes.md)
- [Authoritative mutation evidence](../capabilities/authoritative-mutation-evidence.md)
