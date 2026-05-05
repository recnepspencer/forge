import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../runtime_fixture/real_request_runtime.mjs";

function normalizeForProof(value) {
  return JSON.parse(JSON.stringify(value));
}

test("resource request posture lowers auth and context into line request truth and load input", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { mod, resource } = runtime;
    let capturedRequest = null;
    const detail = resource.detail({
      params: mod.resourceParams(),
      baseUrl: "/api",
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

    assert.deepEqual(normalizeForProof(line.request()), {
      family: line.descriptor().family,
      canonicalParams: normalizeForProof(line.descriptor().canonicalParams),
      target: {
        baseUrl: "/api",
        requestPath: null,
        url: null,
      },
      baseUrl: "/api",
      auth: {
        kind: "authenticated",
      },
      context: {
        headers: { "x-workspace-id": "demo" },
        correlationId: "trace-7",
        branchId: 42,
        basisId: "basis-1",
      },
      continuation: {
        kind: "none",
      },
      processingJob: {
        kind: "none",
      },
      uploadTransport: {
        kind: "none",
      },
      sources: {
        baseUrl: {
          sources: ["endpoint.baseUrl"],
        },
        auth: {
          source: "endpoint.auth",
          overridden: false,
        },
        context: {
          headers: {
            "x-workspace-id": {
              source: "endpoint.requestContext",
              overridden: false,
            },
          },
          correlationId: {
            source: "endpoint.requestContext",
            overridden: false,
          },
          branchId: {
            source: "endpoint.requestContext",
            overridden: false,
          },
          basisId: {
            source: "endpoint.requestContext",
            overridden: false,
          },
        },
        continuation: {
          source: "default.continuation",
          overridden: false,
        },
        processingJob: {
          source: "default.processingJob",
          overridden: false,
        },
        uploadTransport: {
          source: "default.uploadTransport",
          overridden: false,
        },
      },
    });
    assert.deepEqual(
      normalizeForProof(capturedRequest),
      normalizeForProof(line.request()),
    );
    assert.deepEqual(normalizeForProof(line.diagnostics()), {
      policyProfileName: "stable",
      continuity: "preserveVisibleValue",
      freshnessPolicy: "stable",
      request: {
        baseUrl: "/api",
        target: {
          baseUrl: "/api",
          requestPath: null,
          url: null,
        },
        auth: {
          kind: "authenticated",
        },
        context: {
          headerNames: ["x-workspace-id"],
          correlationId: "trace-7",
          branchId: 42,
          basisId: "basis-1",
        },
        continuation: {
          kind: "none",
        },
        processingJob: {
          kind: "none",
        },
        uploadTransport: {
          kind: "none",
        },
        sources: {
          baseUrl: {
            sources: ["endpoint.baseUrl"],
          },
          auth: {
            source: "endpoint.auth",
            overridden: false,
          },
          context: {
            headers: {
              "x-workspace-id": {
                source: "endpoint.requestContext",
                overridden: false,
              },
            },
            correlationId: {
              source: "endpoint.requestContext",
              overridden: false,
            },
            branchId: {
              source: "endpoint.requestContext",
              overridden: false,
            },
            basisId: {
              source: "endpoint.requestContext",
              overridden: false,
            },
          },
          continuation: {
            source: "default.continuation",
            overridden: false,
          },
          processingJob: {
            source: "default.processingJob",
            overridden: false,
          },
          uploadTransport: {
            source: "default.uploadTransport",
            overridden: false,
          },
        },
      },
      basis: {
        currentBasisId: "basis-1",
        advanceCount: 0,
        lastAdvanceFromBasisId: null,
        lastAdvanceToBasisId: null,
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
      download: {
        count: 0,
        readyCount: 0,
        unavailableCount: 0,
        incompatibleCount: 0,
        descriptors: [],
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
      deliveryCount: 0,
      lastSupersededOperation: null,
      lastInvalidationCause: null,
      lastInvalidationScope: null,
      lastPatchKind: null,
      lastPatchScope: null,
      lastPatchedItemId: null,
      lastPatchedAspect: null,
      lastPatchedSummary: null,
      lastDeliveryKind: null,
      lastDeliveryScope: null,
      lastDeliveryPacketId: null,
      lastDeliveryBasisId: null,
      preservedVisibleValueOnLastRejection: false,
      lastTimeoutOperation: null,
      lastErrorMessage: null,
      visibleValueVersion: 1,
    });
  } finally {
    await runtime.cleanup();
  }
});

test("resource request posture can resolve auth and context from params", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { mod, resource } = runtime;
    const detail = resource.detail({
      params: mod.resourceParams(),
      baseUrl: ({ workspaceId }) => `/tenants/${workspaceId}`,
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
    assert.equal(line.request().baseUrl, "/tenants/demo");
    assert.deepEqual(line.request().sources.baseUrl, {
      sources: ["endpoint.baseUrl"],
    });
    assert.deepEqual(line.request().context.headers, {
      "x-workspace-id": "demo",
    });
    assert.deepEqual(line.request().sources.context.headers, {
      "x-workspace-id": {
        source: "endpoint.requestContext",
        overridden: false,
      },
    });
    assert.equal(line.request().context.branchId, 9);
    assert.equal(line.request().sources.auth.source, "endpoint.auth");
    assert.equal(line.diagnostics().request.auth.kind, "workspace");
    assert.deepEqual(line.diagnostics().request.context.headerNames, [
      "x-workspace-id",
    ]);
    assert.equal(
      line.diagnostics().request.sources.context.branchId.source,
      "endpoint.requestContext",
    );
    assert.equal(line.request().uploadTransport.kind, "none");
  } finally {
    await runtime.cleanup();
  }
});

test("resource request posture rejects invalid function-produced auth and context", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { mod, resource } = runtime;
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
    await runtime.cleanup();
  }
});
