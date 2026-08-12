# Migrating to worth-signals-wasm 1.5

## Breaking: `awaitSettlement` is tip-status only

In 1.4.x, some callers treated `line.awaitSettlement()` as a global authored
drain (wait for pending publications/mutations). In **1.5.0** the default is
**tip-status only**: it resolves when this line's tip leaves pending. It does
**not** call `settleAuthoredWork()`.

| Need | 1.5 API |
|---|---|
| Wait for this line's load/refresh tip | `await line.awaitSettlement({ timeoutMs? })` |
| Tip-status wait **and** drain authored work | `await line.awaitSettlement({ drainAuthoredWork: true })` |
| Drain authored pubs/mutations before submit/write | `await signals.settleAuthoredWork()` |

React / `useSignalValue` / `useResourceLine` paint from **host tip notify**. Do
not gate dialog open or UI freshness on `awaitSettlement` or
`settleAuthoredWork`.

```ts
// Paint: tip notify (no settle required)
count.set(false);

// Tip-honest handoff before submit / worker proof
await signals.settleAuthoredWork();
// or
await line.awaitSettlement({ drainAuthoredWork: true });
```

See also [worker-first default](../router/runtime_placement/worker_first_default.md)
and [resource line](../api-reference/resource-line.md).
