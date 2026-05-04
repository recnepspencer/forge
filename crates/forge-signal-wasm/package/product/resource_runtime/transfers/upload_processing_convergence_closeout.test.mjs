import assert from "node:assert/strict";
import test from "node:test";

import { loadResourceModule } from "../module_loading/load_resource_module.mjs";
import { createDeferred } from "../runtime_fixture/deferred.mjs";
import { createFakeSignalNamespace } from "../runtime_fixture/fake_signal_namespace.mjs";

test("upload and downstream processing stay one coherent story under superseded refresh", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
    const firstDeferred = createDeferred();
    const secondDeferred = createDeferred();
    let loadCount = 0;
    const detail = resource.detail({
      params: mod.resourceParams(),
      processingJob: mod.resourceProcessingJob.poll(),
      uploadTransport: mod.resourceUploadTransport.signed({
        method: "POST",
        finalizeRequired: true,
      }),
      normalizeParams: ({ receiptId }) =>
        mod.resourceParamIdentity({ receiptId }, receiptId),
      load: ({ receiptId }) => {
        loadCount += 1;
        if (loadCount === 1) {
          return mod.resourceUploadResult.uploaded({
            uploadId: `upload:${receiptId}`,
            finalizeRequired: true,
            awaitingProcessing: true,
            message: "processing upload",
          });
        }
        return loadCount === 2 ? firstDeferred.promise : secondDeferred.promise;
      },
    });

    const line = detail.line({ receiptId: "r1" });
    assert.equal(line.upload().kind, "uploaded");
    assert.equal(line.upload().awaitingProcessing, true);
    assert.deepEqual(line.processing(), {
      kind: "processing",
      completionKind: "poll",
      jobId: "upload:r1",
      message: "processing upload",
    });

    line.refresh();
    line.refresh();

    firstDeferred.resolve({
      id: "r1",
      status: "stale-ready",
    });
    await firstDeferred.promise;
    await Promise.resolve();

    assert.equal(line.status().kind, "pending");
    assert.equal(line.upload().kind, "uploaded");
    assert.equal(line.processing().kind, "processing");
    assert.equal(line.diagnostics().lastSupersededOperation, "refresh");

    secondDeferred.resolve({
      id: "r1",
      status: "ready",
    });
    await secondDeferred.promise;
    await Promise.resolve();

    assert.deepEqual(line.value(), { id: "r1", status: "ready" });
    assert.deepEqual(line.upload(), {
      kind: "ready",
      transportKind: "signed",
      uploadId: null,
      descriptor: null,
      finalizeRequired: false,
      awaitingProcessing: false,
      message: null,
    });
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

test("direct multipart upload posture remains distinct while prepared state keeps transport truth local", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
    const detail = resource.detail({
      params: mod.resourceParams(),
      uploadTransport: mod.resourceUploadTransport.directMultipart({
        finalizeRequired: false,
      }),
      normalizeParams: ({ receiptId }) =>
        mod.resourceParamIdentity({ receiptId }, receiptId),
      load: ({ receiptId }) =>
        mod.resourceUploadResult.prepared({
          uploadId: `upload:${receiptId}`,
          descriptor: {
            kind: "directMultipart",
            url: `https://uploads.example/${receiptId}`,
            method: "POST",
            headers: { authorization: "secret-upload-token" },
            fields: { key: receiptId },
            objectKey: null,
            expiresAt: null,
          },
          finalizeRequired: false,
          message: "ready",
        }),
    });

    const line = detail.line({ receiptId: "r2" });
    assert.equal(line.request().uploadTransport.kind, "directMultipart");
    assert.equal(line.upload().kind, "prepared");
    assert.equal(line.processing().kind, "ready");
    assert.deepEqual(line.diagnostics().upload.descriptor, {
      kind: "directMultipart",
      url: "https://uploads.example/r2",
      method: "POST",
      headerNames: ["authorization"],
      fieldNames: ["key"],
      objectKey: null,
      expiresAt: null,
    });
    assert.deepEqual(line.diagnostics().request.context.headerNames, []);
    assert.equal(
      JSON.stringify(line.diagnostics()).includes("secret-upload-token"),
      false,
    );
  } finally {
    await mod.cleanup();
  }
});

test("timed-out upload completion ignores stale ready settlement until a new refresh wins", async () => {
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
      uploadTransport: mod.resourceUploadTransport.signed({
        method: "POST",
        finalizeRequired: true,
      }),
      normalizeParams: ({ receiptId }) =>
        mod.resourceParamIdentity({ receiptId }, receiptId),
      load: ({ receiptId }) => {
        loadCount += 1;
        if (loadCount === 1) {
          return mod.resourceUploadResult.uploaded({
            uploadId: `upload:${receiptId}`,
            finalizeRequired: true,
            awaitingProcessing: true,
            message: "processing upload",
          });
        }
        if (loadCount === 2) {
          return deferred.promise;
        }
        return { id: receiptId, status: "ready" };
      },
    });

    const line = detail.line({ receiptId: "r-timeout" });
    line.refresh();
    await new Promise((resolve) => setTimeout(resolve, 0));

    assert.deepEqual(line.status(), {
      kind: "timedOut",
      operation: "refresh",
      continuity: "noVisibleValueYet",
    });
    assert.equal(line.upload().kind, "uploaded");
    assert.equal(line.processing().kind, "processing");

    deferred.resolve({ id: "r-timeout", status: "stale-ready" });
    await deferred.promise;
    await Promise.resolve();

    assert.deepEqual(line.status(), {
      kind: "timedOut",
      operation: "refresh",
      continuity: "noVisibleValueYet",
    });
    assert.equal(line.upload().kind, "uploaded");
    assert.equal(line.processing().kind, "processing");

    line.refresh();
    assert.deepEqual(line.value(), { id: "r-timeout", status: "ready" });
    assert.equal(line.upload().kind, "ready");
    assert.equal(line.processing().kind, "ready");
  } finally {
    await mod.cleanup();
  }
});
