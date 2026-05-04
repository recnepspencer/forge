import assert from "node:assert/strict";
import test from "node:test";

import { loadResourceModule } from "../module_loading/load_resource_module.mjs";
import { createFakeSignalNamespace } from "../runtime_fixture/fake_signal_namespace.mjs";

test("resource continuation posture lowers callback continuation into request truth and diagnostics", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
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
      line.request().continuation,
      mod.resourceContinuation.callback({
        callbackId: "invoice-complete",
        returnTo: "/invoices/1",
      }),
    );
    assert.deepEqual(capturedRequest, line.request());
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
    await mod.cleanup();
  }
});

test("resource continuation posture can resolve webhook continuation from params", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
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
      line.request().continuation,
      mod.resourceContinuation.webhook({
        correlationKey: "stripe:rcpt-1",
        provider: "stripe",
      }),
    );
    assert.equal(line.diagnostics().request.continuation.kind, "webhook");
    assert.equal(
      line.diagnostics().request.continuation.correlationKey,
      "stripe:rcpt-1",
    );
    assert.equal(line.diagnostics().request.continuation.provider, "stripe");
  } finally {
    await mod.cleanup();
  }
});

test("resource continuation posture rejects invalid function-produced continuation truth", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
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
    await mod.cleanup();
  }
});
