# Request Targets And Identity

## What This Feature Is

This page covers the raw family declaration fields that shape request posture
and canonical identity together.

## Why You Use It

Use it when you need:

- direct `signals.resource.detail(...)`, `collection(...)`, or `paged(...)`
- request target control beyond the pleasant route builder
- explicit `normalizeParams(...)`, `requestContext`, `auth`, `policy`, or
  `effects`

## Stable Entry Points

- `signals.resource.detail(...)`
- `signals.resource.collection(...)`
- `signals.resource.paged(...)`
- `resourceParams()`
- `resourceParamIdentity(...)`
- `resourceRequestContext(...)`

## Core Mental Model

Raw family declarations combine two things:

- request behavior
- canonical family identity

Those are separate concerns, but they meet in one declaration.

## How It Executes

A raw declaration can carry:

- `params`
- `baseUrl`
- `method`
- `policy`
- `auth`
- `requestContext`
- `continuation`
- `requestBody`
- `effects`
- `normalizeParams`
- `load`

Collection and paged declarations also carry `itemIdentity`, and paged
declarations carry `accumulatePage(...)`.

## Small Example

```ts
const reportDetail = signals.resource.detail({
  params: resourceParams(),
  requestContext: ({ reportId }) =>
    resourceRequestContext({
      headers: { "x-report-id": String(reportId) },
    }),
  normalizeParams: ({ reportId }) =>
    resourceParamIdentity({ reportId }, `/reports/${reportId}`),
  load: ({ reportId }) => ({ id: reportId }),
});
```

## Real Example

```ts
const search = signals.resource.paged({
  params: resourceParams(),
  auth: resourceAuth.workspace(),
  policy: resourcePolicyProfiles.retryOnce(),
  continuation: resourceContinuation.redirect({
    returnTo: "/workspaces/demo/tasks",
  }),
  normalizeParams: ({ workspaceId, query }) =>
    resourceParamIdentity(
      { workspaceId, query },
      `${workspaceId}:${query}`,
    ),
  itemIdentity: (item) => item.id,
  accumulatePage: (existing, next) => [...existing, ...next],
  load: ({ workspaceId, query }, request) => {
    console.log(request.context.correlationId);
    return [{ id: `${workspaceId}:${query}:1`, title: "First" }];
  },
});
```

## How It Relates To Other Features

- Use [Resource Family Identity](./resource-family-identity.md) when the real
  question is line reuse.
- Use [Raw Resource Lines](./raw-resource-lines.md) when you want the bigger
  "when to drop lower" decision.

## Inspection And Debugging

Inspect:

- `line.request()`
- `line.descriptor().canonicalParams`
- `line.history().verificationPackage().requestPosture`

## Anti-Patterns

- Do not use the raw lane just to rediscover what `signals.api(...).url(...)`
  already gives you.
- Do not let request posture and identity leak into separate hand-maintained
  caches.

## Current Limits

The raw lane gives more control, but it still lowers into the same grouped
line/history/verification boundaries as the pleasant lane.

## Related Docs

- [Raw Resource Lines](./raw-resource-lines.md)
- [Request Posture And Policy](../fetching/request-policies.md)
