# Request Auth And Context

## What This Feature Is

The typed request posture for auth, headers, correlation ids, branch ids, and
basis ids.

## Why You Use It

- keep auth and request metadata declared next to the feature
- make request context inspectable through `line.request()`
- stop headers and basis metadata from turning into ad hoc `load(...)` code

## Stable Entry Points

- `resourceAuth.anonymous()`
- `resourceAuth.authenticated()`
- `resourceAuth.workspace()`
- `resourceRequestContext(...)`
- `signals.api(...)`
- `api.scope(...)`

## Core Mental Model

Request posture is not hidden helper state. It becomes admitted line state.

That means:

- `line.request()` should show it
- `line.diagnostics().request` should summarize it

## How It Executes

When the line materializes, the family resolves:

- auth
- request context headers
- correlation id
- branch id
- basis id

Those become part of the request descriptor passed into `load(...)`.

## Small Example

```ts
const workspaceApi = signals.api({
  auth: resourceAuth.workspace(),
}).scope({
  requestContext: ({ workspaceId }) =>
    resourceRequestContext({
      headers: { "x-workspace-id": workspaceId },
      correlationId: `workspace:${workspaceId}`,
    }),
});
```

## Real Example

```ts
const receiptApi = signals.api({
  auth: ({ workspaceId }: { workspaceId: string }) =>
    workspaceId === "demo"
      ? resourceAuth.workspace()
      : resourceAuth.authenticated(),
}).scope({
  requestContext: ({ workspaceId, receiptId }) =>
    resourceRequestContext({
      headers: {
        "x-workspace-id": workspaceId,
        "x-receipt-id": receiptId,
      },
      correlationId: `receipt:${workspaceId}:${receiptId}`,
      branchId: 42,
      basisId: `basis:${receiptId}`,
    }),
});

const receiptDetail = receiptApi
  .url("/workspaces/:workspaceId/receipts/:receiptId")
  .detail({
    load: ({ receiptId }, request) => ({
      id: receiptId,
      authKind: request.auth.kind,
    }),
  });
```

## How It Relates To Other Features

- Use [Request Policies](./request-policies.md) for stale and retry posture.
- Use [Write A Resource](../updating/write-a-resource.md) once the main problem
  becomes create/update/remove behavior.

## Inspection And Debugging

Use:

- `line.request()`
- `line.diagnostics().request`
- `line.diagnosticsSummary().request`

Those surfaces show the admitted posture without leaking hidden helper state.

## Anti-Patterns

- handwriting plain auth or context objects instead of the exported helpers
- burying auth or header logic inside `load(...)`

## Current Limits

- this page covers auth and request context
- continuation, upload, and processing posture stay in the older request and
  transfer docs for now

## Related Docs

- [Request Policies](./request-policies.md)
- [Older Request Posture And Policy](../request-posture-and-policy.md)
