import assert from "node:assert/strict";
import test from "node:test";

import { createRealTransferRuntime } from "../../runtime_fixture/real_transfer_runtime.mjs";

test("transfers doc happy path covers signed upload with poll processing", async () => {
  const runtime = await createRealTransferRuntime();
  try {
    const receiptUpload = runtime.signals.api({}).url("/receipts/upload")
      .signedUpload({
        method: "POST",
        finalizeRequired: true,
      })
      .processing("poll")
      .create({
        load: ({ body }) => ({ receiptId: body.receiptId }),
      });

    const line = receiptUpload.line({ body: { receiptId: "r1" } });

    assert.equal(line.request().method, "POST");
    assert.equal(line.upload().transportKind, "signed");
    assert.equal(line.processing().completionKind, "poll");
  } finally {
    await runtime.cleanup();
  }
});

test("transfers doc happy path covers multipart upload detail posture", async () => {
  const runtime = await createRealTransferRuntime();
  try {
    const taskUpload = runtime.signals.api({}).url("/tasks/upload")
      .multipartUpload({
        finalizeRequired: false,
      })
      .detail({
        load: () => ({ accepted: true }),
      });

    const line = taskUpload.line({});

    assert.equal(line.upload().transportKind, "directMultipart");
    assert.deepEqual(line.value(), { accepted: true });
  } finally {
    await runtime.cleanup();
  }
});
