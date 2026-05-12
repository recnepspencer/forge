import assert from "node:assert/strict";
import test from "node:test";

import {
  createBranchHead,
  createRealResourceSignals,
} from "../../runtime_fixture/real_resource_signals.mjs";

test("history and restore doc happy path covers availability, verification, and exact restore posture honestly", async () => {
  const runtime = await createRealResourceSignals();
  try {
    createBranchHead(runtime.signals, "history-doc");
    const productDetail = runtime.signals.api({
      baseUrl: "/api",
    }).url("/products/:productId").detail({
      load: ({ productId }) => ({ id: productId }),
    });

    const line = productDetail.line({ productId: "p1" });
    const availability = line.history().availability;
    const verification = line.history().verificationPackage();
    const restoreResult = line.history().restoreExact();

    assert.equal(typeof availability.replay.kind, "string");
    assert.equal(typeof availability.restoreExact.kind, "string");
    assert.equal(verification.boundaryPerformanceEnvelope.summaryReadShape, "inspectionSummary");
    assert.equal(restoreResult.kind, "restored");
  } finally {
    await runtime.cleanup();
  }
});
