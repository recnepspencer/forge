# Request Policies

## What This Feature Is

The typed freshness and retry posture for a resource family.

## Why You Use It

- choose stale behavior explicitly
- choose retry behavior explicitly
- make those choices inspectable through line diagnostics

## Stable Entry Points

- `resourcePolicyProfiles.stable()`
- `resourcePolicyProfiles.immediatelyStale()`
- `resourcePolicyProfiles.retryOnce()`
- `resourcePolicyProfiles.timeoutFast()`

## Core Mental Model

The policy profile is the runtime's declared freshness / retry posture. It is
not just a label.

You should be able to inspect the admitted result later.

## How It Executes

The policy becomes part of the line's admitted freshness and retry posture. It
does not show up as a field on `line.request()`, but it does influence:

- freshness
- retry attempts
- timeout behavior

## Small Example

```ts
const receiptDetail = api.url("/receipts/:receiptId").detail({
  policy: resourcePolicyProfiles.retryOnce(),
  load: ({ receiptId }) => ({ id: receiptId }),
});
```

## Real Example

```ts
const searchResults = signals.api({ baseUrl: "/api" })
  .url("/workspaces/:workspaceId/search")
  .params<{ query: string }>()
  .items((item: { id: string }) => item.id)
  .list({
    policy: resourcePolicyProfiles.immediatelyStale(),
    load: ({ workspaceId, params }) => [
      { id: `${workspaceId}:${params.query}:1` },
    ],
  });

const line = searchResults.line({
  workspaceId: "demo",
  params: { query: "open" },
});

console.log(line.freshness());
console.log(line.diagnostics().policyProfileName);
```

## How It Relates To Other Features

- Use [Request Auth And Context](./request-auth-and-context.md) for auth and
  request metadata.
- Use [Choose An Effect Profile](../updating/choose-an-effect-profile.md) for
  write behavior rather than fetch freshness.

## Inspection And Debugging

Read:

- `line.freshness()`
- `line.diagnostics().policyProfileName`
- `line.diagnostics().freshnessPolicy`

Those are the stable reads for policy behavior. Use `line.request()` for auth,
headers, continuation, processing, upload, and effect posture instead.

## Anti-Patterns

- treating policy choice as a naming detail instead of runtime behavior
- expecting effect profiles to solve fetch freshness questions

## Current Limits

- this page stays on the stable built-in policy set

## Related Docs

- [Request Auth And Context](./request-auth-and-context.md)
- [Older Request Posture And Policy](../request-posture-and-policy.md)
