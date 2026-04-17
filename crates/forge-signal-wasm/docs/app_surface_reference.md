# App Surface Reference

This document is the reference for the primary app-first `forge-signal-wasm`
surface.

## Entry Point

### `createSignals(): Signals`

Creates a framework-agnostic web runtime instance.

```ts
import { createSignals } from "@aust-group/forge-signal-wasm";

const signals = createSignals();
```

`Signals` is the main app-facing object.

### `start(): void`

The package also exports `start()`.

This is a low-level wasm start hook retained for completeness and compatibility.
Normal app code should not need to call it directly before `createSignals()`.

```ts
import { start } from "@aust-group/forge-signal-wasm";
```

## Value Types

### `SignalValue`

`SignalValue` is the JSON-like value model used by the app-first surface:

- `null`
- `boolean`
- `number`
- `string`
- arrays of `SignalValue`
- objects whose values are `SignalValue`

## Handles

### `InputSignal`

Represents mutable source state.

Properties and methods:

- `id: string`
- `get(): SignalValue`
- `free()`
- `[Symbol.dispose]()`

### `ComputedSignal`

Represents derived internal state.

Properties and methods:

- `id: string`
- `get(): SignalValue`
- `free()`
- `[Symbol.dispose]()`

### `OutputSignal`

Represents a public derived projection intended for host/framework consumption.

Properties and methods:

- `id: string`
- `get(): SignalValue`
- `free()`
- `[Symbol.dispose]()`

### `DisposableHandle`

Represents a watcher/effect lifecycle handle returned by `watch(...)` and
`effect(...)`.

Properties and methods:

- `free()`
- `[Symbol.dispose]()`

This handle is also accepted by `nuke(...)`.

### `SignalsTransaction`

Represents the write lane inside `transaction(...)` and `batch(...)`.

Methods:

- `set(input: InputSignal, value: SignalValue): void`
- `setWithRegions(input: InputSignal, value: SignalValue, changedRegions: unknown): void`

## Core Methods On `Signals`

### `input(id, initial): InputSignal`

Registers mutable source state.

```ts
const count = signals.input("count", 1);
```

Use `input` for app-owned values that are explicitly mutated through
transactions.

### `computed(id, spec): ComputedSignal`

Registers derived internal state.

```ts
const doubled = signals.computed("doubled", {
  reads: ["count"],
  expr: {
    kind: "multiply",
    args: [
      { kind: "read", id: "count" },
      { kind: "value", value: 2 },
    ],
  },
});
```

Use `computed` when the value is part of internal derivation rather than a
public projection boundary.

### `output(id, spec): OutputSignal`

Registers a public derived projection.

```ts
const panel = signals.output("panel", {
  reads: ["count", "doubled"],
  expr: {
    kind: "object",
    fields: [
      ["count", { kind: "read", id: "count" }],
      ["doubled", { kind: "read", id: "doubled" }],
    ],
  },
});
```

Use `output` for values that are intended for external consumption:

- React/Vue/Angular view models
- editor panels
- tables and rows
- structured trace payloads
- public app projections

`output` is not just a naming alias of `computed`. It is the public projection
concept in the web runtime.

### `transaction(callback): RunSummary`

Executes a committed write boundary.

```ts
signals.transaction((tx) => {
  tx.set(count, 2);
});
```

The callback receives a `SignalsTransaction`.

### `batch(callback): RunSummary`

Exact ergonomic alias of `transaction(...)`.

It is not a weaker semantic lane. It uses the same committed transaction
boundary.

### `watch(target, callback): DisposableHandle`

Observes committed change for a signal target.

Accepted targets:

- string signal id
- `InputSignal`
- `ComputedSignal`
- `OutputSignal`

Example:

```ts
const handle = signals.watch(panel, (notice) => {
  console.log(notice.signalId, notice.meaningfulChange);
});
```

The callback receives a `WebObservationNotice`.

### `effect(target, callback): DisposableHandle`

Registers a host-side committed reaction.

```ts
const handle = signals.effect(panel, () => {
  console.log("panel changed");
});
```

Like `watch(...)`, `effect(...)` inherits committed observation semantics from
the core runtime.

### `nuke(handle): boolean`

Tears down future deliveries for a watcher/effect handle.

```ts
signals.nuke(handle);
```

This affects future deliveries only.

### `diagnostics(): SignalDiagnostics`

Returns the diagnostics surface.

### `history(): SignalHistory`

Returns the history and branching surface.

### `specialist(): SignalSpecialist`

Returns specialist and lower-level runtime accessors.

### `adapters(): SignalAdapters`

Returns export/import and runtime envelope helpers.

### `compatibilityApp(): SignalApp`

Returns the lower-level compatibility app surface.

### `compatibilityRuntime(): SignalRuntime`

Returns the lower-level compatibility runtime surface.

## `RunSummary`

`transaction(...)` and `batch(...)` return:

- `touchedNodes`
- `nodesEvaluated`
- `nodesRecomputed`
- `nodesSuppressed`
- `plansBuilt`
- `stagesExecuted`
- `totalNanos`
- `evaluationNanos`
- `commitNanos`

This is the first runtime summary for the committed boundary, not the full
diagnostics archive.

## `ComputedSpec` And `OutputSpec`

Both `computed(...)` and `output(...)` use spec-driven authoring.

Fields:

- `reads?: ReadonlyArray<RecipeReadSpec>`
- `expr: Expr`
- `when?: ConditionSpec`
- `identity?: IdentitySpec`

### `RecipeReadSpec`

- simple string id
- or object form:

```ts
{ id: "part", scope: ... }
```

### `ConditionSpec`

```ts
{
  expr: Expr;
}
```

### `IdentitySpec`

- `{ kind: "exact" }`
- `{ kind: "expr", expr: Expr }`

`output(...)` defaults to exact identity behavior so public projections behave
like real externally observed values.

## Expression Reference

`Expr` supports:

- constants: `value`
- reads: `read`
- object/array shaping: `object`, `array`, `mergeObjects`, `pick`, `omit`
- property/index access: `get`, `at`, `first`, `last`, `slice`
- collection helpers: `join`, `flatten`, `length`, `contains`, `keys`, `values`, `append`
- arithmetic: `sum`, `multiply`, `subtract`, `divide`, `abs`, `min`, `max`, `sqrt`, `sin`, `cos`, `floor`, `mod`, `clamp`, `atan2`
- string/object combination: `concat`, `coalesce`
- comparisons: `eq`, `neq`, `gt`, `gte`, `lt`, `lte`
- boolean logic: `and`, `or`, `not`
- conditional branching: `if`

Example object projection:

```ts
{
  kind: "object",
  fields: [
    ["count", { kind: "read", id: "count" }],
    ["isLarge", {
      kind: "gt",
      left: { kind: "read", id: "count" },
      right: { kind: "value", value: 10 },
    }],
  ],
}
```

## Observation Types

### `WebObservationNotice`

Watcher callbacks receive:

- `observerId`
- `handleId`
- `signalId`
- `branchId`
- `policy`
- `touched`
- `recomputed`
- `meaningfulChange`
- `triggerMatched`

This is a web-facing observation notice, not the full retained diagnostics
archive.

## Semantics Summary

- `input` is mutable source state
- `computed` is derived internal state
- `output` is a public projection
- `watch` observes committed change
- `effect` reacts to committed change
- rollback suppresses normal delivery
- `transaction` is the write boundary
- `batch` is an alias of `transaction`

## Related Docs

- [diagnostics_and_history_reference.md](/C:/Users/shepworth/Documents/programming/forge/crates/forge-signal-wasm/docs/diagnostics_and_history_reference.md)
- [compatibility_surface_reference.md](/C:/Users/shepworth/Documents/programming/forge/crates/forge-signal-wasm/docs/compatibility_surface_reference.md)
- [react_adapter_reference.md](/C:/Users/shepworth/Documents/programming/forge/crates/forge-signal-wasm/docs/react_adapter_reference.md)
