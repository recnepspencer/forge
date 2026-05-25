# Detail Fields, Regions, And Json Paths

## What This Feature Is

This page covers the narrow detail reconciliation helpers for single-record
resources.

## Why You Use It

Use it when broad replacement is too coarse and you want to declare:

- one writable field
- one writable region
- one writable json-path slice

## Stable Entry Points

- `signals.resource.detailFields(...)`
- `signals.resource.detailRegions(...)`
- `signals.resource.detailJsonPaths(...)`
- `line.patch(...)`
- `line.reconciliation()`

## Core Mental Model

Detail reconciliation is not generic mutation. It is a declaration of the exact
detail sub-values the runtime can patch honestly.

## How It Executes

Field declarations provide `read(...)` and `write(...)`.
Region declarations provide `read(...)`, `write(...)`, plus:

- `identityBoundary`
- `mergeGranularity`

Json-path declarations provide:

- a typed `path`
- optional `presence`
- proof-carrying `read(...)` and `write(...)`

## Small Example

```ts
const profile = signals.resource.detail({
  params: resourceParams(),
  normalizeParams: ({ id }) => resourceParamIdentity({ id }, id),
  reconcile: signals.resource.detailFields({
    title: {
      read: (value) => value.title,
      write: (value, title) => ({ ...value, title }),
    },
  }),
  load: ({ id }) => ({ id, title: "Loaded" }),
});
```

## Real Example

```ts
const profile = signals.resource.detail({
  params: resourceParams(),
  normalizeParams: ({ id }) => resourceParamIdentity({ id }, id),
  reconcile: signals.resource.detailJsonPaths({
    priority: { path: ["metadata", "priority"], presence: "required" },
  }),
  load: ({ id }) => ({
    id,
    metadata: { priority: 1 },
  }),
});

const line = profile.line({ id: "p1" });
line.patch({ kind: "jsonPath", path: "priority", value: 2 });
```

## How It Relates To Other Features

- Use [Partial Updates And Derived Views](../partial-updates/README.md) for the
  task-first explanation of narrow updates.
- Use [Item Aspects And Value Summaries](./item-aspects-and-value-summaries.md)
  for the collection/paged equivalents.

## Inspection And Debugging

Inspect:

- `line.reconciliation()`
- `line.history().verificationPackage().reconciliation`
- `line.diagnostics().lastPatchKind`

## Anti-Patterns

- Do not declare narrow detail patches the runtime cannot write back honestly.
- Do not use json-path declarations as arbitrary object mutation escape hatches.

## Current Limits

The raw detail lane is explicit because the runtime needs proof-bearing narrow
write behavior, not just read convenience.

## Related Docs

- [Reconciliation Contract](../../resource-contracts/reconciliation.md)
- [Json Path Effects](../json-effects.md)
