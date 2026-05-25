# Fetch A Single Record

## What This Feature Is

The normal route-first detail lane for one resource member.

## Why You Use It

- fetch one thing by stable identity
- keep path params and request posture close to the endpoint
- get one line with ordinary reads like `summary()` and `value()`

## Stable Entry Points

- `signals.api(...)`
- `api.url(...)`
- `.detail(...)`
- `family.line(...)`

## Core Mental Model

A detail family represents one logical member per param set. There is no item
identity helper here because the line itself is already the one thing you care
about.

## How It Executes

1. declare a route
2. finish it with `.detail(...)`
3. materialize a line with path params (and request params if needed)
4. read `line.value()` and `line.summary()`

## Small Example

```ts
const userDetail = api.url("/users/:userId").detail({
  load: ({ userId }) => ({ id: userId, name: `User ${userId}` }),
});

const line = userDetail.line({ userId: "u1" });
```

## Real Example

```ts
const receiptDetail = signals.api({
  baseUrl: "/api",
  auth: resourceAuth.workspace(),
}).scope({
  requestContext: ({ workspaceId }) =>
    resourceRequestContext({
      headers: { "x-workspace-id": workspaceId },
      correlationId: `receipt:${workspaceId}`,
    }),
}).url("/workspaces/:workspaceId/receipts/:receiptId")
  .detail({
    load: ({ receiptId }, request) => ({
      id: receiptId,
      authKind: request.auth.kind,
    }),
  });

const line = receiptDetail.line({
  workspaceId: "demo",
  receiptId: "r1",
});
```

## How It Relates To Other Features

- Use [Fetch A Collection](./fetch-a-collection.md) when the value is a list.
- Use [Fetch A Paged List](./fetch-a-paged-list.md) when later pages accumulate.
- Use [Write A Resource](../updating/write-a-resource.md) for create, update,
  or remove flows.

## Inspection And Debugging

Start with:

- `line.summary()`
- `line.request()`
- `line.diagnostics().request`

## Anti-Patterns

- using a collection just because the server technically returns an array of
  one thing
- moving auth or header setup into `load(...)`

## Current Limits

- this page is about fetching
- mutation responses, transfers, downloads, and reconciliation live elsewhere

## Related Docs

- [Request Auth And Context](./request-auth-and-context.md)
- [Request Policies](./request-policies.md)
