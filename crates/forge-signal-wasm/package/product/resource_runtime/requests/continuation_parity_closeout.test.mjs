import assert from "node:assert/strict";
import test from "node:test";

import { createDeferred } from "../runtime_fixture/async/deferred.mjs";
import { createRealRequestRuntime } from "../runtime_fixture/real_request_runtime.mjs";
import { createRequestArtifactDigest } from "../runtime_fixture/proof/request_artifacts.mjs";

test("explicit continuation absence and equivalent callback declarations stay honest across superseded reloads", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { mod, resource } = runtime;
    const noneFamily = resource.detail({
      params: mod.resourceParams(),
      normalizeParams: ({ invoiceId }) =>
        mod.resourceParamIdentity({ invoiceId }, invoiceId),
      load: ({ invoiceId }) => ({ id: invoiceId, version: 1 }),
    });
    const callbackDirect = resource.detail({
      params: mod.resourceParams(),
      continuation: mod.resourceContinuation.callback({
        callbackId: "invoice-complete",
        returnTo: "/invoices/1",
      }),
      normalizeParams: ({ invoiceId }) =>
        mod.resourceParamIdentity({ invoiceId }, invoiceId),
      load: ({ invoiceId }) => ({ id: invoiceId, version: 1 }),
    });
    const callbackDerived = resource.detail({
      params: mod.resourceParams(),
      continuation: ({ tenant }) =>
        mod.resourceContinuation.callback({
          callbackId: "invoice-complete",
          returnTo: `/invoices/${tenant}`,
        }),
      normalizeParams: ({ tenant, invoiceId }) =>
        mod.resourceParamIdentity(
          { tenant, invoiceId },
          `${tenant}:${invoiceId}`,
        ),
      load: ({ invoiceId }) => ({ id: invoiceId, version: 1 }),
    });

    const noneLine = noneFamily.line({ invoiceId: "inv-1" });
    const directLine = callbackDirect.line({ invoiceId: "inv-1" });
    const derivedLine = callbackDerived.line({
      tenant: "1",
      invoiceId: "inv-1",
    });

    assert.equal(noneLine.request().continuation.kind, "none");
    assert.equal(noneLine.diagnostics().request.continuation.kind, "none");
    assert.equal(noneLine.diagnosticsSummary().request.continuation.kind, "none");

    assert.equal(
      createRequestArtifactDigest(directLine),
      createRequestArtifactDigest(derivedLine),
    );

    const firstDeferred = createDeferred();
    const secondDeferred = createDeferred();
    let loadCount = 0;
    const supersedingFamily = resource.detail({
      params: mod.resourceParams(),
      continuation: mod.resourceContinuation.callback({
        callbackId: "invoice-complete",
        returnTo: "/invoices/1",
      }),
      normalizeParams: ({ invoiceId }) =>
        mod.resourceParamIdentity({ invoiceId }, invoiceId),
      load: ({ invoiceId }) => {
        loadCount += 1;
        if (loadCount === 1) {
          return { id: invoiceId, version: 1 };
        }
        return loadCount === 2 ? firstDeferred.promise : secondDeferred.promise;
      },
    });

    const line = supersedingFamily.line({ invoiceId: "inv-2" });
    line.refresh();
    line.refresh();

    firstDeferred.resolve({ id: "inv-2", version: 2 });
    await firstDeferred.promise;
    await Promise.resolve();
    assert.deepEqual(line.value(), { id: "inv-2", version: 1 });
    assert.equal(line.status().kind, "pending");
    assert.equal(line.request().continuation.kind, "callback");
    assert.equal(line.diagnostics().lastSupersededOperation, "refresh");

    secondDeferred.resolve({ id: "inv-2", version: 3 });
    await secondDeferred.promise;
    await Promise.resolve();
    assert.deepEqual(line.value(), { id: "inv-2", version: 3 });
    assert.equal(line.request().continuation.callbackId, "invoice-complete");
    assert.equal(typeof line.history().availability.replay.kind, "string");
  } finally {
    await runtime.cleanup();
  }
});

test("invalid function-produced continuation posture is denied before load work begins", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { mod, resource } = runtime;
    let loadCalled = false;
    const invalidContinuation = resource.detail({
      params: mod.resourceParams(),
      continuation: () => ({ kind: "callback", callbackId: "cb-1" }),
      normalizeParams: ({ productId }) =>
        mod.resourceParamIdentity({ productId }, productId),
      load: ({ productId }) => {
        loadCalled = true;
        return { id: productId };
      },
    });

    assert.throws(
      () => invalidContinuation.line({ productId: "p1" }),
      /continuation created with resourceContinuation/,
    );
    assert.equal(loadCalled, false);
  } finally {
    await runtime.cleanup();
  }
});
