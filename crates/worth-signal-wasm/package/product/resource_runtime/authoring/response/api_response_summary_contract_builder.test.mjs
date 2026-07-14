import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../runtime_fixture/real_request_runtime.mjs";

test("summary response contracts own the single-response detail finalizer lane", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const response = runtime.signals.resource.response.summary()();
    assert.equal(response.kind, "summary");
    assert.equal(response.lensProof.topology, "summary");
    assert.equal(
      response.lensProof.capabilityRows.some(
        (row) => row.locus === "summaryResponse" && row.patchScope === "line",
      ),
      true,
    );

    const route = runtime.signals.api({}).url("/task-count").response(response);
    const taskCountRead = runtime.signals.api({}).url("/task-count").detail({
      load: () => ({ total: 1 }),
    });
    const count = route.detail({
      load: () => ({ total: 1 }),
    });
    assert.deepEqual(count.line({}).value(), { total: 1 });

    assert.throws(
      () => route.list({ load: () => [] }),
      /summary response lane; use detail/,
    );
    assert.throws(
      () => route.paged({ accumulatePage: (_existing, next) => next, load: () => [] }),
      /summary response lane; use detail/,
    );
    const publishCount = route.create({
      reconciles: [
        {
          family: taskCountRead,
          params: () => ({}),
          fallback: "refetchRequired",
        },
      ],
      load: ({ body }) => ({ total: body.total }),
    });
    const readLine = taskCountRead.line({});
    const publishLine = publishCount.line({
      body: { total: 2 },
    });
    const publishSettlement = await publishLine.awaitSettlement();
    const submittedTarget = publishLine.mutationResponse().submittedTargets[0];
    assert.deepEqual(publishLine.value(), { total: 2 });
    assert.equal(publishSettlement.resultKind, "partial");
    assert.equal(publishSettlement.confirmationKind, "refetchRequired");
    assert.equal(publishLine.mutationResponse().method, "POST");
    assert.equal(publishLine.mutationResponse().atomicity, "singleTarget");
    assert.deepEqual(publishLine.mutationResponse().targets, [{
      targetId: "mutationTarget1",
      family: {
        kind: "detail",
        familyId: readLine.descriptor().family.familyId,
      },
      line: {
        familyKind: "detail",
        familyId: readLine.descriptor().family.familyId,
        canonicalKey: readLine.descriptor().canonicalParams.canonicalKey,
        runtimeLineId: readLine.descriptor().runtimeLineId,
        residency: "resident",
      },
      fallback: {
        kind: "refetchRequired",
        detail:
          `detail ${readLine.descriptor().family.familyId} line ${readLine.descriptor().canonicalParams.canonicalKey} stays in refetchRequired posture until a later mutation-response phase admits exact reconciliation`,
      },
      reconciliation: null,
      submittedTarget,
      execution: {
        artifactId: "mutationTarget1:fallback",
        targetId: "mutationTarget1",
        kind: "fallback",
        fallback: "refetchRequired",
        familyKind: "detail",
        familyId: readLine.descriptor().family.familyId,
        canonicalKey: readLine.descriptor().canonicalParams.canonicalKey,
        runtimeLineId: readLine.descriptor().runtimeLineId,
        residency: "resident",
        submittedTarget,
        staleness: null,
        partial: null,
        detail:
          `detail ${readLine.descriptor().family.familyId} line ${readLine.descriptor().canonicalParams.canonicalKey} stays in refetchRequired posture until a later mutation-response phase admits exact reconciliation`,
      },
      targetDigest:
        `mutationTarget1|detail|${readLine.descriptor().family.familyId}|${readLine.descriptor().canonicalParams.canonicalKey}|refetchRequired`,
    }]);
    assert.deepEqual(publishLine.mutationResponse().executionArtifacts, [{
      artifactId: "mutationTarget1:fallback",
      targetId: "mutationTarget1",
      kind: "fallback",
      fallback: "refetchRequired",
      familyKind: "detail",
      familyId: readLine.descriptor().family.familyId,
      canonicalKey: readLine.descriptor().canonicalParams.canonicalKey,
      runtimeLineId: readLine.descriptor().runtimeLineId,
      residency: "resident",
      submittedTarget,
      staleness: null,
      partial: null,
      detail:
        `detail ${readLine.descriptor().family.familyId} line ${readLine.descriptor().canonicalParams.canonicalKey} stays in refetchRequired posture until a later mutation-response phase admits exact reconciliation`,
    }]);
    assert.equal(
      publishLine.history().verificationPackage().mutationResponse.plan.response.topology,
      "summary",
    );
    assert.equal(
      publishLine.history().verificationPackage().mutationResponse.plan.counters.executionBreadth,
      1,
    );
  } finally {
    await runtime.cleanup();
  }
});
