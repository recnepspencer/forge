# Raw Resource Lines

## What This Feature Is

This is the advanced authoring lane for direct `signals.resource.*(...)`
families and the lines they produce.

## Why You Use It

Use it when you need:

- direct family declarations
- manual canonical identity control
- raw reconcile declarations
- compatibility-owned external definitions

## Stable Entry Points

- `signals.resource.detail(...)`
- `signals.resource.collection(...)`
- `signals.resource.paged(...)`
- `family.line(...)`

## Core Mental Model

The raw lane is still the same runtime, line, diagnostics, and history model.
You are taking on more authoring responsibility, not switching to a different
engine.

## How It Executes

Raw lines still expose the same stable line facade:

- `value()`
- `request()`
- `summary()`
- `diagnosticsSummary()`
- `history()`

Depending on the family declaration, they may also expose:

- `patch(...)`
- `deliver(...)`
- `reconciliation()`

## Small Example

```ts
const detail = signals.resource.detail({
  params: resourceParams(),
  normalizeParams: ({ id }) => resourceParamIdentity({ id }, id),
  load: ({ id }) => ({ id }),
});

const line = detail.line({ id: "a" });
console.log(line.summary());
```

## Real Example

```ts
const tasks = signals.resource.collection({
  params: resourceParams(),
  normalizeParams: ({ workspaceId }) =>
    resourceParamIdentity({ workspaceId }, workspaceId),
  itemIdentity: (item) => item.id,
  reconcile: resourceCollectionShape({
    items: (value) => value,
    replaceItems: (_value, nextItems) => [...nextItems],
  }),
  load: ({ workspaceId }) => [{ id: `${workspaceId}:1`, title: "First" }],
});

const line = tasks.line({ workspaceId: "demo" });
console.log(line.history().verificationPackage().capabilities);
```

## How It Relates To Other Features

- Use [Fetching Data](../fetching/README.md) when the pleasant lane is enough.
- Use [Verification](../verification/README.md) when you need to explain the
  line's proof-bearing package after authoring it.

## Inspection And Debugging

The DX closeout tests prove the route-built and raw-built lines share the same
grouped summary, diagnostics, history, and capability truth. If you are
debugging a raw line, the same line-inspection and history docs still apply.

## Anti-Patterns

- Do not drop to the raw lane just because it feels "more powerful."
- Do not build your own line cache beside the raw family.

## Current Limits

The raw lane exposes more shape choices, but the capability surface still stays
honest. If a line does not admit patch or delivery, the verification package
must say so.

## Related Docs

- [Raw Escape Hatch](../raw-escape-hatch.md)
- [Line Facade Stability Test](../../package/product/resource_runtime/authoring/line_facade_stability.test.mjs)
