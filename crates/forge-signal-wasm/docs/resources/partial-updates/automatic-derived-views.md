# Automatic Derived Views

Use this page when you want a narrower read of one item or one collection to
stay current automatically.

## The User-Facing Idea

You can declare:

- a derived view for one item
- a summary for one whole collection
- a page-window summary for one paged window

Forge calls those derived item views `aspects`, but the practical question is
usually just: "how do I keep this smaller view current automatically?"

## Stable Entry Points

- `.aspect(name, read, write)`
- `.summary(name, read, write)`
- `.pageWindowSummary(name, read, write)`
- `signals.resource.response.objectAspects<T>()(...)`
- `signals.resource.response.jsonPathAspects<T>()(...)`

## Example

```ts
const taskResponse = signals.resource.response.objectItems<{
  tasks: Array<{ id: string; title: string; metadata: { priority: number } }>;
}>()({
  field: "tasks",
  itemId: (task) => task.id,
  aspects: signals.resource.response.jsonPathAspects<{
    id: string;
    title: string;
    metadata: { priority: number };
  }>()({
    priority: { field: "metadata", path: ["priority"] },
  }),
});
```

## When The Word "Aspect" Matters

You can ignore the term until you need the exact API. When you do:

- a derived per-item view is an `aspect`
- a derived collection-wide value is a `summary`

## Related Docs

- [When To Declare Derived Views Explicitly](./when-to-declare-derived-views-explicitly.md)
- [Derived Item Views And Summaries](../lists/derived-item-views-and-summaries.md)
