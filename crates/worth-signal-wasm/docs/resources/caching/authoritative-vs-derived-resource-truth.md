# Authoritative Vs Derived Resource Truth

Use this page when you need to separate canonical resource truth from the
derived views that help you render or update it efficiently.

## The Short Version

The loaded line value is authoritative resource truth.

Other reads may be derived from that truth, such as:

- item views inside a list
- summaries
- visible selection
- grouped summary and diagnostics reads

Those derived views are still first-class. They just are not a separate source
of canonical server truth.

## Why This Matters

This keeps two ideas distinct:

- what the server-backed line truth currently is
- what the runtime derives from it for targeting, rendering, or debugging

That distinction is especially important when you are reading:

- list summaries
- narrow item updates
- mutation response outcomes
- visible selection during optimistic effects

## Related Docs

- [Derived Item Views And Summaries](../lists/derived-item-views-and-summaries.md)
- [Visible Selection](../lists/visible-selection.md)
- [How Resource Caching Works](./how-resource-caching-works.md)
