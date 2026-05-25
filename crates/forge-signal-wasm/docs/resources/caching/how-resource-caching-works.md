# How Resource Caching Works

Use this page when you want the plain-language answer to why a resource line
reused existing truth instead of acting like a brand-new request every time.

## The Simple Model

A resource family is the recipe.
A line is one live member of that recipe.

When you ask the same family for the same canonical line identity again, you
are re-entering the same line identity and continuity lane, not inventing a
second unrelated cache story by accident.

## Stable Entry Points

- `family.line(params)`
- `line.value()`
- `line.summary()`
- `line.status()`
- `line.freshness()`

## What The Runtime Reuses

The runtime reuses:

- the line's current visible value
- freshness state
- diagnostics and history
- pending or settled lifecycle state

That is why caching in Forge feels like "one live line with continuity" rather
than "a bag of disconnected fetch results."

## Related Docs

- [Cache Keys And Resource Identity](./cache-keys-and-resource-identity.md)
- [Stale, Pending, And Settled State](./stale-pending-and-settled-state.md)
