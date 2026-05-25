# How Partial Updates Affect Caching And Delivery

Use this page when you want to know what stays stable when only part of a line
changes.

## The Short Version

Partial updates do not create a second cache story.

They update the current line while preserving the same continuity, freshness,
history, and delivery surfaces.

## What Usually Changes

- the committed visible value
- the visible value version when the structured truth really changed
- the latest patch or delivery diagnostics

## What Usually Stays The Same

- the line identity
- the same grouped summary surface
- any unrelated binary download descriptors

## Example

If one collection row title changes through `itemAspect(...)`, the line keeps
the same history and download surfaces while recording a narrower patch scope.

## Related Docs

- [How Resource Caching Works](../caching/how-resource-caching-works.md)
- [Line Inspection](../line-inspection.md)
