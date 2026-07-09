# Breadcrumb Parent Strategies

## What This Feature Is

Breadcrumb parent strategies are the ordered ancestry rules declared with
`signals.router.breadcrumbParent(...)`.

## Why You Use It

- recompute durable parent context when it exists
- opt into carried provenance only where it makes sense
- define explicit fallback ancestry for deep links

## Stable Entry Points

- `signals.router.breadcrumbParent(...)`
- `recompute`
- `carry: true`
- `fallback`

## Core Mental Model

Parent strategies are ordered, not magical. The router can:

1. recompute ancestry from durable truth
2. consume carried or restored provenance if the route opted in
3. fall back to explicit breadcrumb entries or trails

This keeps the router from faking missing ancestry.

## How It Executes

1. evaluate `recompute(...)` if present
2. if no recomputed ancestry exists, check carried or restored provenance
3. if none applies, use the fallback declaration

## Small Example

```ts
parent: signals.router.breadcrumbParent({
  fallback: signals.router.breadcrumbEntry({
    id: "search-results",
    label: "Search Results",
    target: "/search",
  }),
})
```

## Real Example

```ts
parent: signals.router.breadcrumbParent({
  recompute: ({ params }) => (
    params.resultId === "durable"
      ? signals.router.breadcrumbTrail([
          signals.router.breadcrumbEntry({
            id: "search-context",
            label: "Saved Search",
            target: "/search",
          }),
        ])
      : null
  ),
  carry: true,
  fallback: signals.router.breadcrumbEntry({
    id: "search-results",
    label: "Search Results",
    target: "/search",
  }),
})
```

## How It Relates To Other Features

- carried ancestry is covered in [Carried Provenance](./carried_provenance.md)
- restored ancestry is covered in [Restored Provenance](./restored_provenance.md)

## Inspection And Debugging

- `entry.status`
- `entry.sourceKind`
- `entry.provenance().sourceKind`

## Anti-Patterns

- forcing a leaf route to reconstruct parents it does not semantically own
- making `carry` implicit for every route
- using fallback as if it were canonical parent truth

## Current Limits

- recompute returns breadcrumb entry or trail declarations, not runtime entries
- `carry` is opt-in at the route declaration level

## Related Docs

- [Breadcrumb Declarations](./breadcrumb_declarations.md)
- [Carried Provenance](./carried_provenance.md)
- [Restored Provenance](./restored_provenance.md)
