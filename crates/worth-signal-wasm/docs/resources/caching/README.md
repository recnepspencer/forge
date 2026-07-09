# Caching And Refresh

Use this section when you want to understand why a line reused existing truth,
why it became stale, or how refresh and invalidation actually work.

## Start Here

- [How Resource Caching Works](./how-resource-caching-works.md)
  The practical mental model for families, lines, and reused truth.
- [Cache Keys And Resource Identity](./cache-keys-and-resource-identity.md)
  What makes one line the same line.
- [Stale, Pending, And Settled State](./stale-pending-and-settled-state.md)
  How to read ordinary runtime state.
- [Invalidation And Refresh](./invalidation-and-refresh.md)
  Manual invalidation, refresh, and revalidate.
- [Authoritative Vs Derived Resource Truth](./authoritative-vs-derived-resource-truth.md)
  Which resource truth is canonical and which reads are derived.

## Common Questions

- "Why didn't this refetch?"
  [How Resource Caching Works](./how-resource-caching-works.md)
- "Why does this line say stale?"
  [Stale, Pending, And Settled State](./stale-pending-and-settled-state.md)
- "What should I call: invalidate, refresh, or revalidate?"
  [Invalidation And Refresh](./invalidation-and-refresh.md)

## Deeper History

- [Line Inspection](../line-inspection.md)
- [External Delivery And Compatibility](../external-delivery-and-compatibility.md)
