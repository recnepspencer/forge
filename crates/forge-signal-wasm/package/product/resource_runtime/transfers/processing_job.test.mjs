import assert from "node:assert/strict";
import test from "node:test";

import { loadResourceModule } from "../module_loading/load_resource_module.mjs";
import { createFakeSignalNamespace } from "../runtime_fixture/fake_signal_namespace.mjs";

test("resource processing job posture lowers into request truth and diagnostics", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
    let capturedRequest = null;
    const detail = resource.detail({
      params: mod.resourceParams(),
      processingJob: mod.resourceProcessingJob.callback({
        callbackId: "report-ready",
      }),
      normalizeParams: ({ reportId }) =>
        mod.resourceParamIdentity({ reportId }, reportId),
      load: ({ reportId }, request) => {
        capturedRequest = request;
        return mod.resourceProcessingResult.accepted({
          jobId: `job:${reportId}`,
          message: "queued",
        });
      },
    });

    const line = detail.line({ reportId: "r1" });

    assert.deepEqual(
      line.request().processingJob,
      mod.resourceProcessingJob.callback({
        callbackId: "report-ready",
      }),
    );
    assert.deepEqual(capturedRequest, line.request());
    assert.deepEqual(line.processing(), {
      kind: "accepted",
      completionKind: "callback",
      jobId: "job:r1",
      message: "queued",
    });
    assert.equal(line.diagnostics().request.processingJob.kind, "callback");
    assert.equal(line.diagnostics().processing.kind, "accepted");
    assert.equal(line.diagnostics().visibleValueVersion, 0);
    assert.equal(line.value(), null);
  } finally {
    await mod.cleanup();
  }
});

test("resource processing jobs can move from accepted to ready on refresh", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
    let callCount = 0;
    const detail = resource.detail({
      params: mod.resourceParams(),
      processingJob: mod.resourceProcessingJob.poll(),
      normalizeParams: ({ reportId }) =>
        mod.resourceParamIdentity({ reportId }, reportId),
      load: ({ reportId }) => {
        callCount += 1;
        if (callCount === 1) {
          return mod.resourceProcessingResult.accepted({
            jobId: `job:${reportId}`,
            message: "queued",
          });
        }
        return { id: reportId, status: "ready" };
      },
    });

    const line = detail.line({ reportId: "r2" });
    const refreshStatus = line.refresh();

    assert.deepEqual(refreshStatus, {
      kind: "fulfilled",
      operation: "refresh",
    });
    assert.deepEqual(line.processing(), {
      kind: "ready",
      completionKind: "poll",
      jobId: null,
      message: null,
    });
    assert.deepEqual(line.value(), { id: "r2", status: "ready" });
    assert.equal(line.diagnostics().processing.kind, "ready");
    assert.equal(line.diagnostics().visibleValueVersion, 1);
  } finally {
    await mod.cleanup();
  }
});

test("resource processing jobs can resolve processing posture from params", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
    const detail = resource.detail({
      params: mod.resourceParams(),
      processingJob: ({ provider, receiptId }) =>
        mod.resourceProcessingJob.webhook({
          correlationKey: `${provider}:${receiptId}`,
          provider,
        }),
      normalizeParams: ({ provider, receiptId }) =>
        mod.resourceParamIdentity(
          { provider, receiptId },
          `${provider}:${receiptId}`,
        ),
      load: ({ receiptId }) =>
        mod.resourceProcessingResult.processing({
          jobId: `job:${receiptId}`,
          message: "waiting for provider",
        }),
    });

    const line = detail.line({ provider: "stripe", receiptId: "rcpt-9" });

    assert.equal(line.request().processingJob.kind, "webhook");
    assert.equal(line.request().processingJob.provider, "stripe");
    assert.equal(line.processing().kind, "processing");
    assert.equal(line.processing().completionKind, "webhook");
  } finally {
    await mod.cleanup();
  }
});

test("resource processing results are denied on families without declared processing jobs", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
    const detail = resource.detail({
      params: mod.resourceParams(),
      normalizeParams: ({ reportId }) =>
        mod.resourceParamIdentity({ reportId }, reportId),
      load: ({ reportId }) =>
        mod.resourceProcessingResult.accepted({
          jobId: `job:${reportId}`,
        }),
    });

    assert.throws(
      () => detail.line({ reportId: "r3" }),
      /do not admit resourceProcessingResult/,
    );
  } finally {
    await mod.cleanup();
  }
});

test("resource processing jobs reject invalid function-produced posture truth", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
    const detail = resource.detail({
      params: mod.resourceParams(),
      processingJob: () => ({ kind: "poll" }),
      normalizeParams: ({ reportId }) =>
        mod.resourceParamIdentity({ reportId }, reportId),
      load: ({ reportId }) => ({ id: reportId }),
    });

    assert.throws(
      () => detail.line({ reportId: "r4" }),
      /processingJob created with resourceProcessingJob/,
    );
  } finally {
    await mod.cleanup();
  }
});
