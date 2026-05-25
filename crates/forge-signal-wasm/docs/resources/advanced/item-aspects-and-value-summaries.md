# Item Aspects And Value Summaries

## What This Feature Is

This page covers the raw collection and paged declarations for narrow item and
summary updates.

## Why You Use It

Use it when you need to declare:

- item aspects
- line-level summaries
- page-window summaries
- collection shapes beyond the pleasant `items(...).aspect(...).summary(...)`
  lane

## Stable Entry Points

- `resourceItemAspects(...)`
- `resourceValueSummaries(...)`
- `resourceValueSummaries.pageWindow(...)`
- `resourceCollectionShape(...)`

## Core Mental Model

Item aspects are the runtime's declared narrow item views.
Value summaries are declared aggregate views.

They let local patches and delivered patches stay narrower than broad
replacement.

## How It Executes

`resourceCollectionShape(...)` owns:

- `items(...)`
- `replaceItems(...)`
- `aspects`
- `summaries`
- optional `responseLensProof`

Each aspect and summary carries `read(...)` and `write(...)`, and summaries also
carry patch scope.

## Small Example

```ts
const reconcile = resourceCollectionShape({
  items: (value) => value,
  replaceItems: (_value, nextItems) => [...nextItems],
  aspects: resourceItemAspects({
    title: {
      read: (item) => item.title,
      write: (item, title) => ({ ...item, title }),
    },
  }),
});
```

## Real Example

```ts
const reconcile = resourceCollectionShape({
  items: (value) => value.items,
  replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
  aspects: resourceItemAspects({
    title: {
      read: (item) => item.title,
      write: (item, title) => ({ ...item, title }),
    },
  }),
  summaries: resourceValueSummaries.pageWindow({
    visibleCount: {
      read: (value) => value.items.length,
      write: (value, visibleCount) => ({
        ...value,
        items: value.items.slice(0, Number(visibleCount)),
      }),
    },
  }),
});
```

## How It Relates To Other Features

- Use [Working With Lists](../lists/README.md) for the task-first list docs.
- Use [Detail Fields, Regions, And Json Paths](./detail-fields-regions-and-json-paths.md)
  for the detail-resource version of narrow reconciliation.

## Inspection And Debugging

Inspect:

- `line.reconciliation()`
- `line.history().verificationPackage().capabilities`
- `line.history().verificationPackage().reconciliation`

## Anti-Patterns

- Do not declare summaries that cannot be written back honestly.
- Do not use item aspects to hide broad replacement behavior.

## Current Limits

Paged page-window summaries and line-wide summaries are distinct patch scopes.
The runtime keeps that distinction explicit.

## Related Docs

- [Collections And Delivery](../collections-and-delivery.md)
- [Partial Updates And Derived Views](../partial-updates/README.md)
