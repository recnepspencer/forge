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
- `line.history().verificationPackage()`

## Happy Path

```ts
import { createSignals } from "forge-signal-wasm";

const signals = createSignals();

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

That is still a feature worth documenting because the runtime is honest about
the boundary instead of hiding it.

## Where To Go Next

- grouped current-state reads:
  [feature_line_inspection.md](./feature_line_inspection.md)
- external basis movement and compatibility delivery:
  [feature_external_delivery_and_compatibility.md](./feature_external_delivery_and_compatibility.md)
- lower-level history reference:
  [resource_inspection_and_history_reference.md](./resource_inspection_and_history_reference.md)
