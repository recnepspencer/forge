# History And Restore

Use this page when the question is not just "what is the line doing now?" but
"what history does it retain, can it restore exactly, and what does the runtime
 actually admit here?"

## What This Covers

- `line.history().availability`
- `line.history().lifecycle`
- `line.history().basis`
- `line.history().branch`
- `line.history().replay`
- `line.history().lineage`
- `line.history().replayExact()`
- `line.history().restoreExact()`
- `line.history().rollbackLastEffect()`
- `line.history().verificationPackage()`

## Happy Path

```ts
import { createSignals } from "forge-signal-wasm";

const signals = await createSignals();

const productDetail = signals.api({
  baseUrl: "/api",
}).url("/products/:productId").detail({
  load: ({ productId }) => ({ id: productId }),
});

const line = productDetail.line({ productId: "p1" });

console.log(line.history().availability);
console.log(line.history().verificationPackage());
```

Start here when you need to know:

- whether exact replay or exact restore are supported on this runtime
- what lifecycle and basis events happened over time
- whether two runs can be compared through one stable verification artifact
- whether the last resource effect can roll back exactly, by compact inverse,
  or not at all

## Exact Restore Mental Model

`restoreExact()` is a real supported same-runtime action when the runtime can
resolve a branch snapshot target.

That means:

- support is explicit in `line.history().availability.restoreExact`
- the action result is typed
- restore still goes through the line model, so request basis and diagnostics
  stay coherent

## Exact Replay Mental Model

`replayExact()` is also a typed action surface, but on the shipped wasm Signals
runtime it currently reports typed unavailability rather than pretending exact
signal replay exists.

That is still useful because the runtime gives a typed unavailable result
instead of leaving callers to guess.

## Effect Rollback Mental Model

`rollbackLastEffect()` is the resource-effect rollback surface. It consumes the
last runtime-issued effect envelope recorded on the line and returns a typed
result.

The result can be:

- `rolledBack` with mode `SameRuntimeBranchExact`
- `rolledBack` with mode `CompactInversePatch`
- `rollbackUnavailable`
- `noEffect`

The effect envelope tells you what rollback can do before you call it:

```ts
const effect = line.diagnostics().lastEffect;

console.log(effect?.optimistic.rollback.kind);

const rollback = line.history().rollbackLastEffect();

console.log(rollback.kind);
console.log(rollback.rollback);
```

Exact branch rollback is preferred when the same runtime branch snapshot is
available. Compact inverse rollback is used only when the effect captured an
exact compact preimage for the changed item, aspect, or summary. If neither is
available, rollback returns a typed unavailable result and preserves the visible
value.

## Rollback Kinds

- `exactBranchRestoreAvailable`: the effect carries a same-runtime branch and
  snapshot that can restore the line exactly.
- `compactInverseAvailable`: exact branch restore is unavailable, but the
  runtime retained a compact inverse patch.
- `unavailable`: the effect wanted optimism, but restore or inverse data was
  not available.
- `notApplicable`: the effect was committed-only, delivery-authoritative, or
  otherwise not speculative.

Optional JSON path writes are deliberately conservative. If an absent optional
terminal would require a lossy inverse that restores `null` instead of absence,
the runtime refuses to claim compact inverse rollback.

## Restore Versus Rollback

Use `restoreExact()` when you are restoring the line to an explicit retained
history target. Use `rollbackLastEffect()` when you are undoing the last
resource effect according to the effect envelope's rollback data.

Both surfaces are typed and inspectable. Neither asks application code to guess
from the current visible value.

## Where To Go Next

- grouped current-state reads:
  [Line Inspection](../resources/line-inspection.md)
- branch-native effects and profile selection:
  [Branch-Native Resource Effects](../resources/branch-native-effects.md)
- effect envelope fields:
  [Effect Envelope Contract](./effect-envelope.md)
- effect merge and rebase:
  [Effect Merge And Rebase](../resources/merge-and-rebase.md)
- external basis movement and compatibility delivery:
  [External Delivery And Compatibility](../resources/external-delivery-and-compatibility.md)
- lower-level history reference:
  [Inspection And History Contract](./inspection-and-history.md)
