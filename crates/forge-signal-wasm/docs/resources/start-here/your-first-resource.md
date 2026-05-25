# Your First Resource

## What This Feature Is

This is the normal first resource: one route-first family, one line, one
summary read.

## Why You Use It

- start on the common path
- keep request shape, canonical identity, and line behavior together
- get to `line.summary()` and `line.value()` without learning the whole resource
  system first

## Stable Entry Points

- `signals.api(...)`
- `api.url(...)`
- `.detail(...)`
- `family.line(...)`
- `line.summary()`
- `line.value()`

## Core Mental Model

The route declaration creates a family.
Calling `family.line(...)` creates one live line from that family.

For ordinary app work, you usually do not need the raw
`signals.resource.detail(...)` lane first.

## How It Executes

1. declare shared defaults with `signals.api(...)`
2. declare one route with `api.url(...)`
3. finish it with `.detail(...)`
4. materialize a line with `family.line(...)`
5. read `line.summary()` or `line.value()`

## Small Example

```ts
const userDetail = signals.api({
  baseUrl: "/api",
}).url("/users/:userId").detail({
  load: ({ userId }) => ({ id: userId, name: `User ${userId}` }),
});

const line = userDetail.line({ userId: "u1" });

console.log(line.summary());
console.log(line.value());
```

## Real Example

```ts
const workspaceApi = signals.api({
  baseUrl: "/api",
  auth: resourceAuth.workspace(),
}).scope({
  requestContext: ({ workspaceId }) =>
    resourceRequestContext({
      headers: { "x-workspace-id": workspaceId },
      correlationId: `workspace:${workspaceId}`,
    }),
});

const productDetail = workspaceApi
  .url("/workspaces/:workspaceId/products/:productId")
  .detail({
    load: ({ productId }, request) => ({
      id: productId,
      authKind: request.auth.kind,
    }),
  });

const line = productDetail.line({
  workspaceId: "demo",
  productId: "p1",
});
```

## How It Relates To Other Features

- If you are unsure which family shape to use next, go to
  [Choose A Resource Shape](./choose-a-resource-shape.md).
- If the main question is fetching behavior, continue into
  [Fetching Data](../fetching/README.md).
- If the main question is writes, continue into
  [Updating Data](../updating/README.md).

## Inspection And Debugging

Start with:

- `line.summary()`
- `line.request()`
- `line.status()`
- `line.freshness()`

That gives you the admitted request, the current value state, and whether the
line is fresh or stale.

## Anti-Patterns

- starting with the raw `signals.resource.*(...)` lane when the route-first
  lane already fits
- hiding auth, headers, or request body logic deep inside `load(...)`
- treating `line.value()` as the only useful read and skipping
  `line.summary()`

## Current Limits

- this page stays on the route-first lane
- uploads, downloads, mutation-response reconciliation, and advanced patching
  live in later sections

## Related Docs

- [Choose A Resource Shape](./choose-a-resource-shape.md)
- [Fetch A Single Record](../fetching/fetch-a-single-record.md)
- [Older Flat Resource Overview](../overview.md)
