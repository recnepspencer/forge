# Resource Request And Policy Reference

If your question is "how do I set auth, request headers, retry policy, or
continuation posture?", start with
[Request Posture And Policy](../resources/request-posture-and-policy.md)
before using this lower-level reference page.

## What This Feature Is

This is the part of the resource surface that controls:

- auth
- headers and request context
- stale/retry/timeout behavior
- continuation behavior
- processing posture
- upload transport posture

In plain English: this is where you define how the resource should behave when
it talks to the outside world.

## Why You Use It

- keep auth and headers next to the resource that needs them
- make retry and stale behavior explicit
- pass branch, correlation, or basis context through the request cleanly
- model redirects, callbacks, uploads, and deferred processing without ad hoc
  helper code

## Stable Entry Points

Recommended shared-default lane:

- `signals.api(...)`
- `api.scope(...)`

Lower-level policy helpers:

- `resourcePolicyProfiles.stable()`
- `resourcePolicyProfiles.immediatelyStale()`
- `resourcePolicyProfiles.retryOnce()`
- `resourcePolicyProfiles.timeoutFast()`

Request helpers:

- `resourceAuth.anonymous()`
- `resourceAuth.authenticated()`
- `resourceAuth.workspace()`
- `resourceRequestContext(...)`
- `resourceContinuation.none()`
- `resourceContinuation.redirect(...)`
- `resourceContinuation.callback(...)`
- `resourceContinuation.webhook(...)`

Transfer posture helpers:

- `resourceProcessingJob.none()`
- `resourceProcessingJob.poll()`
- `resourceProcessingJob.callback(...)`
- `resourceProcessingJob.webhook(...)`
- `resourceUploadTransport.none()`
- `resourceUploadTransport.directMultipart(...)`
- `resourceUploadTransport.signed(...)`

## Core Mental Model

These helpers are not "random options".

They define real typed posture that becomes part of the line's admitted request
state.

That means:

- `line.request()` reflects these decisions
- diagnostics reflect these decisions
- lifecycle behavior reflects these decisions

So if you want to know how a line was configured, you should be able to inspect
the line and see it.

## How It Executes

When you materialize a line, the family resolves:

1. policy
2. auth
3. request context
4. continuation posture
5. processing posture
6. upload posture

Those then become:

- part of the request passed to `load(...)`
- part of `line.request()`
- part of line diagnostics
- part of line upload/processing state

## Small Example

```ts
import {
  createSignals,
  resourceAuth,
  resourceRequestContext,
} from "forge-signal-wasm";

const signals = await createSignals();

const workspaceApi = signals.api({
  auth: resourceAuth.workspace(),
}).scope({
  requestContext: ({ workspaceId }) =>
    resourceRequestContext({
      headers: { "x-workspace-id": workspaceId },
      correlationId: `product:${workspaceId}`,
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
```

This is the smallest honest example because it shows the most common setup:

- auth
- scoped request headers
- correlation id

## Real Example

```ts
import {
  createSignals,
  resourceAuth,
  resourceContinuation,
  resourcePolicyProfiles,
  resourceRequestContext,
} from "forge-signal-wasm";

const signals = await createSignals();

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
      basisId: `basis:${receiptId}`,
    }),
});

const receiptPipeline = receiptApi.url(
  "/workspaces/:workspaceId/receipts/:receiptId",
)
  .signedUpload({
    method: "POST",
    finalizeRequired: true,
  })
  .processing("poll")
  .detail({
    policy: resourcePolicyProfiles.retryOnce(),
    continuation: resourceContinuation.callback({
      callbackId: "receipt-finished",
      returnTo: "/receipts",
    }),
    load: ({ receiptId }) => ({ id: receiptId }),
  });

const line = receiptPipeline.line({
  workspaceId: "demo",
  receiptId: "r1",
});

console.log(line.request());
console.log(line.diagnostics().request);
```

What is authoritative:

- the API root, scopes, and endpoint declaration together decide the posture
- the line exposes the admitted result

What gets inspected:

- `line.request()` for the raw admitted request state
- `line.diagnostics().request` for request-specific diagnostics

## How It Relates To Other Features

- Family docs explain where these helpers are declared.
- Transfer docs explain how upload and processing posture show up later on the
  line.
- Binary/download docs are separate because download descriptors are visible
  value-adjacent state, not request posture.

## Inspection And Debugging

Use:

- `line.request()`
- `line.diagnostics().request`
- `line.diagnosticsSummary().request`
- `line.request().sources`
- `line.diagnostics().policyProfileName`
- `line.diagnostics().freshnessPolicy`

These tell you both what was configured and what actually got admitted.

## Anti-Patterns

- building auth or headers deep inside `load(...)`
- handwriting plain objects instead of using the exported posture helpers
- treating policy names as labels instead of real lifecycle behavior

## Current Limits

- the stable policy set is intentionally small
- this surface controls request and lifecycle posture, not routing or mutation
  orchestration

## Related Docs

- [Resource Overview](../resources/overview.md)
- [Route Authoring Reference](./route-authoring.md)
- [Resource Family Authoring Reference](./resource-family-authoring.md)
- [Resource Transfers Reference](./resource-transfers.md)
- [Resource Line Reference](./resource-line.md)
