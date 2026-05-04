import assert from "node:assert/strict";
import test from "node:test";

import { loadResourceModule } from "../module_loading/load_resource_module.mjs";
import { createFakeSignalNamespace } from "../runtime_fixture/fake_signal_namespace.mjs";

test("resource request posture lowers auth and context into line request truth and load input", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
    let capturedRequest = null;
    const detail = resource.detail({
      params: mod.resourceParams(),
      auth: mod.resourceAuth.authenticated(),
      requestContext: mod.resourceRequestContext({
        headers: { "x-workspace-id": "demo" },
        correlationId: "trace-7",
        branchId: 42,
        basisId: "basis-1",
      }),
      normalizeParams: ({ productId }) =>
        mod.resourceParamIdentity({ productId }, productId),
      load: ({ productId }, request) => {
        capturedRequest = request;
        return { id: productId };
      },
    });

    const line = detail.line({ productId: "p1" });

    assert.deepEqual(line.request(), {
      family: line.descriptor().family,
      canonicalParams: line.descriptor().canonicalParams,
      auth: mod.resourceAuth.authenticated(),
      context: mod.resourceRequestContext({
        headers: { "x-workspace-id": "demo" },
        correlationId: "trace-7",
        branchId: 42,
        basisId: "basis-1",
      }),
      continuation: mod.resourceContinuation.none(),
      processingJob: mod.resourceProcessingJob.none(),
      uploadTransport: mod.resourceUploadTransport.none(),
    });
    assert.deepEqual(capturedRequest, line.request());
    assert.deepEqual(line.diagnostics(), {
      policyProfileName: "stable",
      continuity: "preserveVisibleValue",
      freshnessPolicy: "stable",
      request: {
        auth: mod.resourceAuth.authenticated(),
        context: {
          headerNames: ["x-workspace-id"],
          correlationId: "trace-7",
          branchId: 42,
          basisId: "basis-1",
        },
        continuation: mod.resourceContinuation.none(),
        processingJob: mod.resourceProcessingJob.none(),
        uploadTransport: mod.resourceUploadTransport.none(),
      },
      processing: {
        kind: "ready",
        completionKind: "none",
        jobId: null,
        message: null,
      },
      upload: {
        kind: "ready",
        transportKind: "none",
        uploadId: null,
        descriptor: null,
        finalizeRequired: false,
        awaitingProcessing: false,
        message: null,
      },
      lastOperation: "initialLoad",
      lastOutcome: "fulfilled",
      pendingOperation: null,
      refreshCount: 0,
      revalidateCount: 0,
      retryAttemptCount: 0,
      rejectionCount: 0,
      timeoutCount: 0,
      supersessionCount: 0,
      invalidationCount: 0,
      patchCount: 0,
      lastSupersededOperation: null,
      lastInvalidationCause: null,
      lastInvalidationScope: null,
      lastPatchKind: null,
      lastPatchScope: null,
      lastPatchedItemId: null,
      lastPatchedAspect: null,
      lastPatchedSummary: null,
      preservedVisibleValueOnLastRejection: false,
      lastTimeoutOperation: null,
      lastErrorMessage: null,
      visibleValueVersion: 1,
    });
  } finally {
    await mod.cleanup();
  }
});

test("resource request posture can resolve auth and context from params", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
    const detail = resource.detail({
      params: mod.resourceParams(),
      auth: ({ workspaceId }) =>
        workspaceId === "demo"
          ? mod.resourceAuth.workspace()
          : mod.resourceAuth.anonymous(),
      requestContext: ({ workspaceId, branchId }) =>
        mod.resourceRequestContext({
          headers: { "x-workspace-id": workspaceId },
          branchId,
        }),
      normalizeParams: ({ workspaceId, branchId, productId }) =>
        mod.resourceParamIdentity(
          { workspaceId, branchId, productId },
          `${workspaceId}:${branchId}:${productId}`,
        ),
      load: ({ productId }) => ({ id: productId }),
    });

    const line = detail.line({
      workspaceId: "demo",
      branchId: 9,
      productId: "p1",
    });

    assert.equal(line.request().auth.kind, "workspace");
    assert.deepEqual(line.request().context.headers, {
      "x-workspace-id": "demo",
    });
    assert.equal(line.request().context.branchId, 9);
    assert.equal(line.diagnostics().request.auth.kind, "workspace");
    assert.deepEqual(line.diagnostics().request.context.headerNames, [
      "x-workspace-id",
    ]);
    assert.equal(line.request().uploadTransport.kind, "none");
  } finally {
    await mod.cleanup();
  }
});

test("resource request posture rejects invalid function-produced auth and context", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
    const invalidAuth = resource.detail({
      params: mod.resourceParams(),
      auth: () => ({ kind: "authenticated" }),
      normalizeParams: ({ productId }) =>
        mod.resourceParamIdentity({ productId }, productId),
      load: ({ productId }) => ({ id: productId }),
    });
    const invalidContext = resource.detail({
      params: mod.resourceParams(),
      requestContext: () => ({ headers: { "x-trace-id": "trace-1" } }),
      normalizeParams: ({ productId }) =>
        mod.resourceParamIdentity({ productId }, productId),
      load: ({ productId }) => ({ id: productId }),
    });

    assert.throws(
      () => invalidAuth.line({ productId: "p1" }),
      /auth created with resourceAuth/,
    );
    assert.throws(
      () => invalidContext.line({ productId: "p1" }),
      /requestContext created with resourceRequestContext/,
    );
  } finally {
    await mod.cleanup();
  }
});
