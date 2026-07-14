# Mutation-Response Readback

## What This Feature Is

This page covers the mutation-response detail attached to a resource-backed
form source.

## Why You Use It

- inspect confirmation, exact-vs-fallback targets, and identity migration
- understand what a resource-backed write actually confirmed
- keep mutation-response detail on one form read instead of reconstructing it
  from resource internals

## Stable Entry Points

- `form.resourceSource()?.mutationResponse`

## Core Mental Model

Mutation-response readback is the resource-backed answer to "what did the write
confirmation actually say?" It is much more specific than plain action success
or settlement.

## How It Executes

1. a resource-backed write completes
2. the resource line derives mutation-response detail and target outcomes
3. the form exposes that readback through `resourceSource()`

## Small Example

```ts
console.log(form.resourceSource()?.mutationResponse?.confirmationKind);
```

## Real Example

```ts
const response = form.resourceSource()?.mutationResponse;

console.log(response?.targetCount);
console.log(response?.exactTargetCount);
console.log(response?.fallbackTargetCount);
console.log(response?.identityMigration);
```

## How It Relates To Other Features

- Read [Resource Settlement](./resource-settlement.md) for the higher-level
  confirmed/failed view above this lane.
- Read [Resource Action Execution](./resource-action-execution.md) for the
  action lifecycle that produced the mutation response.

## Inspection And Debugging

- confirmation kind and target counts are the first reads
- `completion` shows multi-family, placement, and deletion shape
- `identityMigration` shows whether identity migration was needed or denied

## Anti-Patterns

- treating mutation-response detail like a generic success message
- ignoring fallback target counts when the write is not exact

## Current Limits

- deeper resource mutation contracts live in the resource docs

## Related Docs

- [Resource Settlement](./resource-settlement.md)
- [Resource Action Execution](./resource-action-execution.md)
