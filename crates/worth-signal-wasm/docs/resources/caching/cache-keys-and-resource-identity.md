# Cache Keys And Resource Identity

Use this page when you want to know what makes two resource reads the same live
line.

## The Practical Answer

Line identity comes from:

- the declared family
- the family's canonical param normalization

That is the user-facing answer to "what cache entry is this?"

## Why This Matters

Stable identity is what lets the runtime keep:

- one continuity story
- one history story
- one freshness story

for the same line over time.

## What To Reach For

Start with the family declaration and the params you pass to `family.line(...)`.

If you need lower-level identity details, the deeper modeling docs are the
right place, not ordinary app code.

## Related Docs

- [How Resource Caching Works](./how-resource-caching-works.md)
- [Resource Family Authoring Reference](../../api-reference/resource-family-authoring.md)
