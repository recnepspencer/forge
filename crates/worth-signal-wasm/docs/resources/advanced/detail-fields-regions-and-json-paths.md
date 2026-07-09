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

- `signals.resource.response.detail<T>()({...})`
- `signals.resource.detailFields(...)`
- `signals.resource.detailRegions(...)`
- `signals.resource.detailJsonPaths(...)`
- `line.patch(...)`
- `line.reconciliation()`

## Core Mental Model

Detail reconciliation is not generic mutation. It is a declaration of the exact
detail sub-values the runtime can patch honestly.

## How It Executes

Response-owned detail field declarations map exposed field names to top-level
object fields.

Explicit field declarations provide `read(...)` and `write(...)`.
Region declarations provide `read(...)`, `write(...)`, plus:

- `identityBoundary`
- `mergeGranularity`

Json-path declarations provide:

- a typed `path`
- optional `presence`
- proof-carrying `read(...)` and `write(...)`

## Choose The Right Lane

Use these lanes in order:

- `response.detail<T>()({...})` for ordinary top-level object fields
- `detailFields(...)` when field reads or writes need custom logic
- `detailRegions(...)` for named subtree replacement
- `detailJsonPaths(...)` for nested JSON-path slices

The response-owned detail lane is the default because it keeps the field
contract reusable across reads, writes, and mutation-response reconciliation.

## Small Example

```ts
const profile = signals.api({}).url("/profiles/:id")
  .response(signals.resource.response.detail<{
    id: string;
    title: string;
  }>()({
    title: "title",
  }))
  .detail({
    load: ({ id }) => ({ id, title: "Loaded" }),
  });
```

## Real Example

Use the explicit lane when the field behavior is not just "read this object
property and write it back immutably."

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

Use the other explicit lanes when you need structured narrow writes:

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

The explicit detail lanes still matter because the runtime needs proof-bearing
narrow write behavior, not just read convenience. The response-owned field lane
is the common path only for ordinary top-level object fields.

## Related Docs

- [Reconciliation Contract](../../resource-contracts/reconciliation.md)
- [Json Path Effects](../json-effects.md)
