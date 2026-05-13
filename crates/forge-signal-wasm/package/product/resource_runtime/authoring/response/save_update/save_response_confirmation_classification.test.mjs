import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../../runtime_fixture/real_request_runtime.mjs";

test("save response confirmation classifies consumed canonical truth for exact updates", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const readFamily = runtime.signals.api({}).url("/users/:userId").detail({
      load: ({ userId }) => ({ id: userId, name: "First" }),
    });
    readFamily.line({ userId: "u1" });
    const saveUser = runtime.signals.api({}).url("/users/:userId")
      .response(runtime.signals.resource.response.detail()())
      .update({
        reconciles: [
          {
            family: readFamily,
            params: ({ userId }) => ({ userId }),
            fallback: "refetchRequired",
            detail: { kind: "replace" },
          },
        ],
        load: ({ userId, body }) => ({ id: userId, name: body.name }),
      });

    const plan = saveUser.line({
      userId: "u1",
      body: { name: "Updated" },
    }).mutationResponse();

    assert.equal(plan.confirmation.kind, "consumedCanonicalTruth");
    assert.equal(plan.confirmation.exactTargetCount, 1);
    assert.equal(plan.confirmation.fallbackTargetCount, 0);
    assert.equal(plan.counters.confirmationClassificationBreadth, 1);
    assert.equal(
      plan.confirmation.digest,
      "mutation-response-confirmation|consumedCanonicalTruth|exact:1|fallbacks:none|diagnostics:mutation-response-diagnostics|none",
    );
  } finally {
    await runtime.cleanup();
  }
});

test("save response confirmation classifies preserved optimistic truth for diagnostics-only responses", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const saveWorkflow = runtime.signals.api({}).url("/workflows/:workflowId")
      .response(runtime.signals.resource.response.detail()({
        warnings: "warnings",
      }))
      .update({
        diagnostics: [
          { kind: "warnings", field: "warnings" },
        ],
        load: () => ({ warnings: ["server accepted local truth"] }),
      });

    const saveLine = saveWorkflow.line({
      workflowId: "wf-1",
      body: { title: "Local" },
    });
    const plan = saveLine.mutationResponse();

    assert.equal(plan.confirmation.kind, "preservedOptimisticTruth");
    assert.equal(plan.confirmation.exactTargetCount, 0);
    assert.equal(plan.confirmation.fallbackTargetCount, 0);
    assert.equal(plan.confirmation.diagnosticCount, 1);
    assert.equal(plan.counters.confirmationClassificationBreadth, 1);
    assert.equal(
      saveLine.summary().diagnostics.latest.mutationResponseConfirmationKind,
      "preservedOptimisticTruth",
    );
    assert.equal(
      saveLine.summary().diagnostics.latest.mutationResponseConfirmationDigest,
      plan.confirmation.digest,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("save response confirmation classifies explicit refetch and delivery fallback posture", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const readFamily = runtime.signals.api({}).url("/profiles/:profileId").detail({
      load: ({ profileId }) => ({ id: profileId, name: "First" }),
    });
    readFamily.line({ profileId: "p1" });
    const saveProfile = runtime.signals.api({}).url("/profiles/:profileId")
      .response(runtime.signals.resource.response.detail()())
      .update({
        reconciles: [
          {
            family: readFamily,
            params: ({ profileId }) => ({ profileId }),
            fallback: "refetchRequired",
          },
        ],
        load: ({ profileId, body }) => ({ id: profileId, name: body.name }),
      });
    const queueProfile = runtime.signals.api({}).url("/profiles/:profileId/queue")
      .response(runtime.signals.resource.response.detail()())
      .update({
        reconciles: [
          {
            family: readFamily,
            params: ({ profileId }) => ({ profileId }),
            fallback: "deliveryAwaited",
          },
        ],
        load: ({ profileId, body }) => ({ id: profileId, name: body.name }),
      });

    const refetchPlan = saveProfile.line({
      profileId: "p1",
      body: { name: "Server" },
    }).mutationResponse();
    const deliveryPlan = queueProfile.line({
      profileId: "p1",
      body: { name: "Queued" },
    }).mutationResponse();

    assert.equal(refetchPlan.confirmation.kind, "refetchRequired");
    assert.deepEqual(refetchPlan.confirmation.fallbackKinds, ["refetchRequired"]);
    assert.equal(refetchPlan.counters.confirmationClassificationBreadth, 1);
    assert.equal(deliveryPlan.confirmation.kind, "deliveryAwaited");
    assert.deepEqual(deliveryPlan.confirmation.fallbackKinds, ["deliveryAwaited"]);
    assert.equal(deliveryPlan.counters.confirmationClassificationBreadth, 1);
  } finally {
    await runtime.cleanup();
  }
});
