# Request Posture And Policy

Use this page when the main question is not "what value does this endpoint
load?" but "what request posture and lifecycle posture does this line admit?"

## What This Covers

- `resourceAuth.*(...)`
- `resourceRequestContext(...)`
- `resourcePolicyProfiles.*()`
- `resourceContinuation.*(...)`
- `signals.api(...)`, `signals.apiScope(...)`, and `api.scope(...)` for shared defaults
- request and policy inspection through `line.request()` and diagnostics

## Happy Path

```ts
import {
  createSignals,
  resourceAuth,
  resourceContinuation,
  resourcePolicyProfiles,
  resourceRequestContext,
} from "forge-signal-wasm";

const signals = await createSignals();

const receiptApi = signals.apiScope("receipt-api", {
  auth: resourceAuth.workspace(),
}).scope({
  requestContext: ({ workspaceId }) =>
    resourceRequestContext({
      headers: { "x-workspace-id": workspaceId },
      correlationId: `receipt:${workspaceId}`,
    }),
});

const receiptDetail = receiptApi
  .url("/workspaces/:workspaceId/receipts/:receiptId")
  .detail({
    policy: resourcePolicyProfiles.retryOnce(),
    continuation: resourceContinuation.callback({
      callbackId: "receipt-finished",
      returnTo: "/receipts",
    }),
    load: ({ receiptId }, request) => ({
      id: receiptId,
      authKind: request.auth.kind,
    }),
  });

const line = receiptDetail.line({
  workspaceId: "demo",
  receiptId: "r1",
});

console.log(line.request());
console.log(line.diagnostics().request);
```

## When To Reach For This Feature

- one feature area shares auth, headers, or correlation ids
- stale/retry behavior should be explicit instead of incidental
- the request must carry basis or branch context
- the endpoint returns a continuation contract that the host needs to inspect

Use `signals.apiScope(...)` when those shared defaults should have one stable
runtime-scoped identity. Keep `api.scope({...})` for param-dependent defaults
that cannot honestly participate in identity reuse.

## What To Inspect

- `line.request()`
- `line.diagnostics().request`
- `line.diagnosticsSummary().request`
- `line.diagnostics().policyProfileName`
- `line.diagnostics().freshnessPolicy`

## Where To Go Next

- ordinary route-first declarations:
  [Fetch And Write Resources](./fetch-and-write.md)
- upload and processing posture:
  [Transfers](./transfers.md)
- lower-level reference details:
  [Resource Request And Policy Reference](../api-reference/resource-request-and-policy.md)
