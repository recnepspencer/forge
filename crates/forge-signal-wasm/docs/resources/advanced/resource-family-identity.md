# Resource Family Identity

## What This Feature Is

This page explains the family and line identity rules underneath resource line
reuse.

## Why You Use It

Use it when you need to answer:

- when do two `line(...)` calls return the same live line?
- what makes one family distinct from another?
- how do canonical params become stable identity?

## Stable Entry Points

- `resourceParamIdentity(...)`
- `family.line(params)`
- `line.descriptor()`

## Core Mental Model

One family plus one canonical parameter identity gives you one live line.

If canonical params are equal, the family reuses one line.
If the family is different, the runtime keeps the lines distinct even when the
canonical key string matches.

## How It Executes

`normalizeParams(...)` returns a `resourceParamIdentity(...)` artifact. That
artifact snapshots:

- the canonical params object
- the canonical key string

The runtime uses that canonical identity for family-owned line reuse.

## Small Example

```ts
const detail = signals.resource.detail({
  params: resourceParams(),
  normalizeParams: ({ id }) => resourceParamIdentity({ id }, id),
  load: ({ id }) => ({ id }),
});

const first = detail.line({ id: "a" });
const second = detail.line({ id: "a" });

console.log(first === second);
```

## Real Example

```ts
const collection = signals.resource.collection({
  params: resourceParams(),
  normalizeParams: ({ workspaceId }) =>
    resourceParamIdentity({ workspaceId }, workspaceId),
  itemIdentity: (item) => item.id,
  load: ({ workspaceId }) => [{ id: `${workspaceId}:1` }],
});

const line = collection.line({ workspaceId: "demo" });
console.log(line.descriptor().family.kind);
console.log(line.descriptor().canonicalParams.canonicalKey);
console.log(line.descriptor().runtimeLineId);
```

## How It Relates To Other Features

- Use [Request Targets And Identity](./request-targets-and-identity.md) when
  identity is part of a fuller raw declaration.
- Use [Caching And Refresh](../caching/README.md) for the user-facing view of
  line reuse and staleness.

## Inspection And Debugging

Check:

- `line.descriptor().family`
- `line.descriptor().canonicalParams`
- `line.descriptor().runtimeLineId`

## Anti-Patterns

- Do not build canonical identity from mutable caller objects after the fact.
- Do not assume equal canonical keys make different family kinds the same line.

## Current Limits

Canonical identity is family-local. Distinct families preserve distinct runtime
line identity even when the canonical key string happens to match.

## Related Docs

- [Family Identity Equivalence Tests](../../package/product/resource_runtime/authoring/family_identity_equivalence.test.mjs)
- [Raw Resource Lines](./raw-resource-lines.md)
