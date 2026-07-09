# Invalidation And Refresh

Use this page when you need to choose between invalidating a line, refreshing
it immediately, or revalidating while preserving visible truth.

## Stable Entry Points

- `line.invalidate()`
- `line.refresh()`
- `line.revalidate()`
- `line.freshness()`
- `line.diagnostics()`

## What Each One Means

- `invalidate()`
  Mark the line stale without pretending new data arrived.
- `refresh()`
  Start a fresh request for the line.
- `revalidate()`
  Re-check the line while preserving the visible value continuity lane.

## Example

```ts
const line = userDetail.line({ userId: "u1" });

line.invalidate();
console.log(line.freshness());

line.refresh();
console.log(line.status());

line.revalidate();
console.log(line.freshness());
```

## Why Revalidate Exists

`revalidate()` is the nicer lane when you want to keep showing the current
visible value while checking whether it is still current.

## Related Docs

- [Stale, Pending, And Settled State](./stale-pending-and-settled-state.md)
- [External Delivery And Compatibility](../external-delivery-and-compatibility.md)
