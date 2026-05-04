import assert from "node:assert/strict";
import test from "node:test";

import { loadResourceModule } from "../module_loading/load_resource_module.mjs";
import { createDeferred } from "../runtime_fixture/deferred.mjs";
import { createFakeSignalNamespace } from "../runtime_fixture/fake_signal_namespace.mjs";

test("deferred processing stays inside one lifecycle story across callback, poll, and webhook postures", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
    const cases = [
      {
        kind: "poll",
        posture: mod.resourceProcessingJob.poll(),
      },
      {
        kind: "callback",
        posture: mod.resourceProcessingJob.callback({
          callbackId: "report-ready",
        }),
      },
      {
        kind: "webhook",
        posture: mod.resourceProcessingJob.webhook({
          correlationKey: "stripe:receipt-1",
          provider: "stripe",
        }),
      },
    ];

    for (const processingCase of cases) {
      const firstDeferred = createDeferred();
      const secondDeferred = createDeferred();
      let loadCount = 0;
      const detail = resource.detail({
        params: mod.resourceParams(),
        processingJob: processingCase.posture,
        normalizeParams: ({ reportId }) =>
          mod.resourceParamIdentity({ reportId }, reportId),
        load: ({ reportId }) => {
          loadCount += 1;
          if (loadCount === 1) {
            return mod.resourceProcessingResult.accepted({
              jobId: `job:${reportId}`,
              message: "queued",
            });
          }
          return loadCount === 2 ? firstDeferred.promise : secondDeferred.promise;
        },
      });

      const line = detail.line({ reportId: "r1" });
      assert.equal(line.processing().kind, "accepted");
      assert.equal(line.processing().completionKind, processingCase.kind);
      assert.equal(line.value(), null);

      line.invalidate();
      assert.equal(line.diagnostics().lastInvalidationCause, "manualLineInvalidate");

      line.refresh();
      line.refresh();

      firstDeferred.resolve({ id: "r1", status: "stale-ready" });
      await firstDeferred.promise;
      await Promise.resolve();
      assert.equal(line.processing().kind, "accepted");
      assert.equal(line.status().kind, "pending");
      assert.equal(line.diagnostics().lastSupersededOperation, "refresh");

      secondDeferred.resolve({ id: "r1", status: "ready" });
      await secondDeferred.promise;
      await Promise.resolve();

      assert.deepEqual(line.value(), { id: "r1", status: "ready" });
      assert.deepEqual(line.processing(), {
        kind: "ready",
        completionKind: processingCase.kind,
        jobId: null,
        message: null,
      });
      assert.equal(line.diagnostics().processing.kind, "ready");
      assert.equal(line.history().lifecycle.at(-1)?.event, "fulfilled");
    }
  } finally {
    await mod.cleanup();
  }
});

test("timed-out deferred processing ignores stale completion until a new refresh wins honestly", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
    const deferred = createDeferred();
    let loadCount = 0;
    const detail = resource.detail({
      params: mod.resourceParams(),
      policy: mod.resourcePolicyProfiles.timeoutFast(),
      processingJob: mod.resourceProcessingJob.poll(),
      normalizeParams: ({ reportId }) =>
        mod.resourceParamIdentity({ reportId }, reportId),
      load: ({ reportId }) => {
        loadCount += 1;
        if (loadCount === 1) {
          return mod.resourceProcessingResult.accepted({
            jobId: `job:${reportId}`,
            message: "queued",
          });
        }
        if (loadCount === 2) {
          return deferred.promise;
        }
        return { id: reportId, status: "ready" };
      },
    });

    const line = detail.line({ reportId: "r-timeout" });
    line.refresh();
    await new Promise((resolve) => setTimeout(resolve, 0));

    assert.deepEqual(line.status(), {
      kind: "timedOut",
      operation: "refresh",
      continuity: "noVisibleValueYet",
    });
    assert.equal(line.processing().kind, "accepted");
    assert.equal(line.value(), null);

    deferred.resolve({ id: "r-timeout", status: "stale-ready" });
    await deferred.promise;
    await Promise.resolve();

    assert.deepEqual(line.status(), {
      kind: "timedOut",
      operation: "refresh",
      continuity: "noVisibleValueYet",
    });
    assert.equal(line.processing().kind, "accepted");
    assert.equal(line.value(), null);

    line.refresh();
    assert.deepEqual(line.value(), { id: "r-timeout", status: "ready" });
    assert.deepEqual(line.processing(), {
      kind: "ready",
      completionKind: "poll",
      jobId: null,
      message: null,
    });
  } finally {
    await mod.cleanup();
  }
});
