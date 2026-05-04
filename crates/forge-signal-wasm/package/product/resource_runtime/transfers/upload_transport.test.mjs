import assert from "node:assert/strict";
import test from "node:test";

import { loadResourceModule } from "../module_loading/load_resource_module.mjs";
import { createFakeSignalNamespace } from "../runtime_fixture/fake_signal_namespace.mjs";

test("resource upload transport lowers into request truth and diagnostics", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
    let capturedRequest = null;
    const detail = resource.detail({
      params: mod.resourceParams(),
      uploadTransport: mod.resourceUploadTransport.signed({
        method: "PUT",
        finalizeRequired: true,
      }),
      normalizeParams: ({ receiptId }) =>
        mod.resourceParamIdentity({ receiptId }, receiptId),
      load: ({ receiptId }, request) => {
        capturedRequest = request;
        return mod.resourceUploadResult.prepared({
          uploadId: `upload:${receiptId}`,
          descriptor: {
            kind: "signed",
            url: `https://uploads.example/${receiptId}`,
            method: "PUT",
            headers: { "x-upload-token": "demo" },
            fields: {},
            objectKey: `receipts/${receiptId}.png`,
            expiresAt: "2026-05-04T12:00:00Z",
          },
          finalizeRequired: true,
          message: "ready to upload",
        });
      },
    });

    const line = detail.line({ receiptId: "r1" });

    assert.deepEqual(
      line.request().uploadTransport,
      mod.resourceUploadTransport.signed({
        method: "PUT",
        finalizeRequired: true,
      }),
    );
    assert.deepEqual(capturedRequest, line.request());
    assert.deepEqual(line.upload(), {
      kind: "prepared",
      transportKind: "signed",
      uploadId: "upload:r1",
      descriptor: {
        kind: "signed",
        url: "https://uploads.example/r1",
        method: "PUT",
        headers: { "x-upload-token": "demo" },
        fields: {},
        objectKey: "receipts/r1.png",
        expiresAt: "2026-05-04T12:00:00Z",
      },
      finalizeRequired: true,
      awaitingProcessing: false,
      message: "ready to upload",
    });
    assert.equal(line.diagnostics().request.uploadTransport.kind, "signed");
    assert.deepEqual(line.diagnostics().upload, {
      kind: "prepared",
      transportKind: "signed",
      uploadId: "upload:r1",
      descriptor: {
        kind: "signed",
        url: "https://uploads.example/r1",
        method: "PUT",
        headerNames: ["x-upload-token"],
        fieldNames: [],
        objectKey: "receipts/r1.png",
        expiresAt: "2026-05-04T12:00:00Z",
      },
      finalizeRequired: true,
      awaitingProcessing: false,
      message: "ready to upload",
    });
    assert.equal(line.value(), null);
  } finally {
    await mod.cleanup();
  }
});

test("resource uploads can move from prepared to uploaded to ready on refresh", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
    let callCount = 0;
    const detail = resource.detail({
      params: mod.resourceParams(),
      uploadTransport: mod.resourceUploadTransport.directMultipart({
        finalizeRequired: false,
      }),
      normalizeParams: ({ receiptId }) =>
        mod.resourceParamIdentity({ receiptId }, receiptId),
      load: ({ receiptId }) => {
        callCount += 1;
        if (callCount === 1) {
          return mod.resourceUploadResult.prepared({
            uploadId: `upload:${receiptId}`,
            descriptor: {
              kind: "directMultipart",
              url: `https://uploads.example/${receiptId}`,
              method: "POST",
              headers: {},
              fields: { key: receiptId },
              objectKey: null,
              expiresAt: null,
            },
            finalizeRequired: false,
            message: "ready",
          });
        }
        if (callCount === 2) {
          return mod.resourceUploadResult.uploaded({
            uploadId: `upload:${receiptId}`,
            finalizeRequired: false,
            awaitingProcessing: true,
            message: "uploaded",
          });
        }
        return { id: receiptId, status: "ready" };
      },
    });

    const line = detail.line({ receiptId: "r2" });
    line.refresh();
    assert.equal(line.upload().kind, "uploaded");
    assert.equal(line.upload().awaitingProcessing, true);
    assert.equal(line.processing().kind, "ready");

    line.refresh();
    assert.deepEqual(line.upload(), {
      kind: "ready",
      transportKind: "directMultipart",
      uploadId: null,
      descriptor: null,
      finalizeRequired: false,
      awaitingProcessing: false,
      message: null,
    });
    assert.deepEqual(line.value(), { id: "r2", status: "ready" });
  } finally {
    await mod.cleanup();
  }
});

test("resource upload and processing declarations stay coherent while upload awaits downstream processing", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
    let callCount = 0;
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
        callCount += 1;
        if (callCount === 1) {
          return mod.resourceUploadResult.uploaded({
            uploadId: `upload:${receiptId}`,
            finalizeRequired: true,
            awaitingProcessing: true,
            message: "processing upload",
          });
        }
        return { id: receiptId, status: "ready" };
      },
    });

    const line = detail.line({ receiptId: "r6" });

    assert.deepEqual(line.upload(), {
      kind: "uploaded",
      transportKind: "signed",
      uploadId: "upload:r6",
      descriptor: null,
      finalizeRequired: true,
      awaitingProcessing: true,
      message: "processing upload",
    });
    assert.deepEqual(line.processing(), {
      kind: "processing",
      completionKind: "poll",
      jobId: "upload:r6",
      message: "processing upload",
    });

    line.refresh();

    assert.equal(line.processing().kind, "ready");
    assert.equal(line.upload().kind, "ready");
    assert.deepEqual(line.value(), { id: "r6", status: "ready" });
  } finally {
    await mod.cleanup();
  }
});

test("resource upload transport can resolve from params", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
    const detail = resource.detail({
      params: mod.resourceParams(),
      uploadTransport: ({ method }) =>
        mod.resourceUploadTransport.signed({
          method,
          finalizeRequired: true,
        }),
      normalizeParams: ({ method, receiptId }) =>
        mod.resourceParamIdentity({ method, receiptId }, `${method}:${receiptId}`),
      load: ({ receiptId }) =>
        mod.resourceUploadResult.prepared({
          uploadId: `upload:${receiptId}`,
          descriptor: {
            kind: "signed",
            url: `https://uploads.example/${receiptId}`,
            method: "POST",
            headers: {},
            fields: {},
            objectKey: null,
            expiresAt: null,
          },
          finalizeRequired: true,
          message: null,
        }),
    });

    const line = detail.line({ method: "POST", receiptId: "r3" });

    assert.equal(line.request().uploadTransport.kind, "signed");
    assert.equal(line.request().uploadTransport.method, "POST");
    assert.equal(line.upload().transportKind, "signed");
  } finally {
    await mod.cleanup();
  }
});

test("resource upload results are denied on families without declared upload transport", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
    const detail = resource.detail({
      params: mod.resourceParams(),
      normalizeParams: ({ receiptId }) =>
        mod.resourceParamIdentity({ receiptId }, receiptId),
      load: ({ receiptId }) =>
        mod.resourceUploadResult.uploaded({
          uploadId: `upload:${receiptId}`,
          finalizeRequired: true,
          awaitingProcessing: false,
          message: "uploaded",
        }),
    });

    assert.throws(
      () => detail.line({ receiptId: "r4" }),
      /do not admit resourceUploadResult/,
    );
  } finally {
    await mod.cleanup();
  }
});

test("resource uploads reject invalid function-produced transport posture truth", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
    const detail = resource.detail({
      params: mod.resourceParams(),
      uploadTransport: () => ({ kind: "signed", method: "PUT" }),
      normalizeParams: ({ receiptId }) =>
        mod.resourceParamIdentity({ receiptId }, receiptId),
      load: ({ receiptId }) => ({ id: receiptId }),
    });

    assert.throws(
      () => detail.line({ receiptId: "r5" }),
      /uploadTransport created with resourceUploadTransport/,
    );
  } finally {
    await mod.cleanup();
  }
});
