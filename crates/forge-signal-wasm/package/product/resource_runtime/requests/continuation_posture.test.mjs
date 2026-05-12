import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../runtime_fixture/real_request_runtime.mjs";

function normalizeForProof(value) {
  return JSON.parse(JSON.stringify(value));
}

test("resource continuation posture lowers callback continuation into request truth and diagnostics", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { mod, resource } = runtime;
    let capturedRequest = null;
    const detail = resource.detail({
      params: mod.resourceParams(),
      continuation: mod.resourceContinuation.callback({
        callbackId: "invoice-complete",
        returnTo: "/invoices/1",
      }),
      normalizeParams: ({ invoiceId }) =>
        mod.resourceParamIdentity({ invoiceId }, invoiceId),
      load: ({ invoiceId }, request) => {
        capturedRequest = request;
        return { id: invoiceId };
      },
    });

    const line = detail.line({ invoiceId: "inv-1" });

    assert.deepEqual(
      normalizeForProof(line.request().continuation),
      {
        kind: "callback",
        callbackId: "invoice-complete",
        returnTo: "/invoices/1",
      },
    );
    assert.deepEqual(
      normalizeForProof(capturedRequest),
      normalizeForProof(line.request()),
    );
    assert.equal(line.diagnostics().request.continuation.kind, "callback");
    assert.equal(
      line.diagnostics().request.continuation.callbackId,
      "invoice-complete",
    );
    assert.equal(
      line.diagnostics().request.continuation.returnTo,
      "/invoices/1",
    );
    assert.equal(line.diagnostics().request.continuation.kind, "callback");
  } finally {
    await runtime.cleanup();
  }
});

test("resource continuation posture can resolve webhook continuation from params", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { mod, resource } = runtime;
    const detail = resource.detail({
      params: mod.resourceParams(),
      continuation: ({ provider, receiptId }) =>
        mod.resourceContinuation.webhook({
          correlationKey: `${provider}:${receiptId}`,
          provider,
        }),
      normalizeParams: ({ provider, receiptId }) =>
        mod.resourceParamIdentity(
          { provider, receiptId },
          `${provider}:${receiptId}`,
        ),
      load: ({ receiptId }) => ({ id: receiptId }),
    });

    const line = detail.line({ provider: "stripe", receiptId: "rcpt-1" });

    assert.deepEqual(
      normalizeForProof(line.request().continuation),
      {
        kind: "webhook",
        correlationKey: "stripe:rcpt-1",
        provider: "stripe",
      },
    );
    assert.equal(line.diagnostics().request.continuation.kind, "webhook");
    assert.equal(
      line.diagnostics().request.continuation.correlationKey,
      "stripe:rcpt-1",
    );
    assert.equal(line.diagnostics().request.continuation.provider, "stripe");
  } finally {
    await runtime.cleanup();
  }
});

test("resource continuation posture rejects invalid function-produced continuation truth", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { mod, resource } = runtime;
    const invalidContinuation = resource.detail({
      params: mod.resourceParams(),
      continuation: () => ({ kind: "callback", callbackId: "cb-1" }),
      normalizeParams: ({ productId }) =>
        mod.resourceParamIdentity({ productId }, productId),
      load: ({ productId }) => ({ id: productId }),
    });

    assert.throws(
      () => invalidContinuation.line({ productId: "p1" }),
      /continuation created with resourceContinuation/,
    );
  } finally {
    await runtime.cleanup();
  }
});
