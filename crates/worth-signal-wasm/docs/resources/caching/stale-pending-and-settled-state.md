# Stale, Pending, And Settled State

Use this page when you need to explain a line's current request and freshness
state to yourself or to the UI.

## Stable Entry Points

- `line.status()`
- `line.freshness()`
- `line.summary()`
- `line.diagnostics()`

## The Main Reads

- `line.status()`
  Ordinary request state such as pending, fulfilled, rejected, or timed out.
- `line.freshness()`
  Whether the current visible truth is fresh or stale.
- `line.summary()`
  The grouped first read.

## Example

```ts
const line = userDetail.line({ userId: "u1" });

console.log(line.status());
console.log(line.freshness());
console.log(line.summary());
```

## Common Stale Reasons

The runtime can name stale reasons such as:

- policy-profile-driven staleness
- refresh pending
- manual invalidation
- family-wide invalidation

The exact reason belongs to the freshness read, not app folklore.

## Related Docs

- [Invalidation And Refresh](./invalidation-and-refresh.md)
- [Line Inspection](../line-inspection.md)
