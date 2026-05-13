import assert from "node:assert/strict";
import test from "node:test";

import { createRealLifecycleRuntime } from "../../runtime_fixture/real_lifecycle_runtime.mjs";

function normalizeForProof(value) {
  return JSON.parse(JSON.stringify(value));
}

test("resource lines refresh in place and record diagnostics", async () => {
  const runtime = await createRealLifecycleRuntime();
  try {
    const { mod, resource } = runtime;
    let version = 0;
    const detail = resource.detail({
      params: mod.resourceParams(),
      normalizeParams: ({ productId }) =>
        mod.resourceParamIdentity({ productId }, productId),
      load: ({ productId }) => ({ id: productId, version: ++version }),
    });

    const line = detail.line({ productId: "p1" });
    const refreshStatus = line.refresh();

    assert.deepEqual(line.value(), { id: "p1", version: 2 });
    assert.deepEqual(refreshStatus, {
      kind: "fulfilled",
      operation: "refresh",
    });
    assert.deepEqual(line.status(), {
      kind: "fulfilled",
      operation: "refresh",
    });
    assert.deepEqual(line.freshness(), { kind: "fresh" });
    assert.deepEqual(normalizeForProof(line.diagnostics()), {
      policyProfileName: "stable",
      continuity: "preserveVisibleValue",
      freshnessPolicy: "stable",
      request: {
        baseUrl: null,
        target: {
          baseUrl: null,
          requestPath: null,
          url: null,
        },
        method: "GET",
        bodyPresent: false,
        auth: {
          kind: "anonymous",
        },
        context: {
          headerNames: [],
          correlationId: null,
          branchId: null,
          basisId: null,
        },
        continuation: {
          kind: "none",
        },
        processingJob: {
          kind: "none",
        },
        effects: null,
        sources: {
          baseUrl: null,
          auth: {
            source: "default.auth",
            overridden: false,
          },
          context: {
            headers: {},
            correlationId: null,
            branchId: null,
            basisId: null,
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
          effects: {
            source: "default.effects",
            overridden: false,
          },
        },
        uploadTransport: {
          kind: "none",
        },
      },
      basis: {
        currentBasisId: null,
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
      lastOperation: "refresh",
      lastOutcome: "fulfilled",
      pendingOperation: null,
      refreshCount: 1,
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
      lastPatchedField: null,
      lastPatchedRegion: null,
      lastPatchedPath: null,
      lastPatchedAspect: null,
      lastPatchedSummary: null,
      lastDeliveryKind: null,
      lastDeliveryScope: null,
      lastDeliveryPacketId: null,
      lastDeliveryBasisId: null,
      lastEffect: null,
      preservedVisibleValueOnLastRejection: false,
      lastTimeoutOperation: null,
      lastErrorMessage: null,
      visibleValueVersion: 2,
    });
  } finally {
    await runtime.cleanup();
  }
});
