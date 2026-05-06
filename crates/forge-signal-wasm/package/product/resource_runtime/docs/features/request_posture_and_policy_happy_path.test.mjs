import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../runtime_fixture/real_request_runtime.mjs";

test("request posture and policy doc happy path covers shared auth, request context, policy, and continuation", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const {
      resourceAuth,
      resourceContinuation,
      resourcePolicyProfiles,
      resourceRequestContext,
    } = runtime.signalsMod;
    const receiptApi = runtime.signals.api({
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

    assert.equal(line.value().authKind, "workspace");
    assert.equal(line.request().auth.kind, "workspace");
    assert.equal(line.request().continuation.kind, "callback");
    assert.equal(line.diagnostics().policyProfileName, "retryOnce");
    assert.deepEqual(line.request().context.headers, {
      "x-workspace-id": "demo",
    });
  } finally {
    await runtime.cleanup();
  }
});
