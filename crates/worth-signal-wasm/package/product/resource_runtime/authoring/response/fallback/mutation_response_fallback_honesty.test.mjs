import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../../runtime_fixture/real_request_runtime.mjs";

test("create placementUnavailable fallback stays explicit across summary history and verification reads", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const taskList = runtime.signals.api({}).url("/tasks")
      .response(runtime.signals.resource.response.array({
        itemId: (item) => item.id,
      }))
      .list({
        load: () => [],
      });
    const createTask = runtime.signals.api({}).url("/tasks")
      .response(runtime.signals.resource.response.detail()())
      .create({
        reconciles: [{
          family: taskList,
          params: () => ({}),
          fallback: "placementUnavailable",
          collection: { kind: "insert", placement: "append" },
        }],
        load: ({ body }) => ({ id: body.id, title: body.title }),
      });

    const createLine = createTask.line({
      body: { id: "t2", title: "Second" },
    });
    const latest = createLine.summary().diagnostics.latest;

    assert.equal(latest.mutationResponseFallbackReasonDigest,
      "mutation-response-fallback-reasons|placementUnavailable:1");
    assert.equal(latest.mutationResponseFallbackAffectedTargetDigest,
      "mutation-response-fallback-targets|mutationTarget1:collection:"
      + `${taskList.line({}).descriptor().family.familyId}:/tasks:placementUnavailable`);
    assert.equal(latest.mutationResponseFreshnessPostureDigest,
      "mutation-response-freshness-posture|partialCanonicalTruth|targets:1|exact:0|fallback:1");
    assert.equal(latest.mutationResponseDeliveryAwaitedDigest,
      "mutation-response-deliveryAwaited-targets|none");
    assert.equal(latest.mutationResponseRefetchRequiredDigest,
      "mutation-response-refetchRequired-targets|none");
    assert.equal(latest.mutationResponseNoHiddenMutationDigest,
      "mutation-response-no-hidden-mutation|allDeclaredTargetsAccountedFor|declared:1|accounted:1|exact:0|fallback:1");
    assert.equal(
      createLine.history().verificationPackage().diagnostics.summary.latest
        .mutationResponseFallbackReasonDigest,
      latest.mutationResponseFallbackReasonDigest,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("refetchRequired fallback emits explicit freshness posture instead of hidden stale state", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const profileRead = runtime.signals.api({}).url("/profiles/:profileId").detail({
      load: ({ profileId }) => ({ id: profileId, name: "First" }),
    });
    profileRead.line({ profileId: "p1" });
    const saveProfile = runtime.signals.api({}).url("/profiles/:profileId")
      .response(runtime.signals.resource.response.detail()())
      .update({
        reconciles: [{
          family: profileRead,
          params: ({ profileId }) => ({ profileId }),
          fallback: "refetchRequired",
        }],
        load: ({ profileId, body }) => ({ id: profileId, name: body.name }),
      });

    const saveLine = saveProfile.line({
      profileId: "p1",
      body: { name: "Server" },
    });
    const latest = saveLine.summary().diagnostics.latest;

    assert.equal(latest.mutationResponseFallbackReasonDigest,
      "mutation-response-fallback-reasons|refetchRequired:1");
    assert.equal(latest.mutationResponseRefetchRequiredDigest,
      "mutation-response-refetchRequired-targets|mutationTarget1:detail:"
      + `${profileRead.line({ profileId: "p1" }).descriptor().family.familyId}:/profiles/p1:none:none`);
    assert.equal(latest.mutationResponseDeliveryAwaitedDigest,
      "mutation-response-deliveryAwaited-targets|none");
    assert.equal(latest.mutationResponseFreshnessPostureDigest,
      "mutation-response-freshness-posture|refetchRequired|targets:1|exact:0|fallback:1");
    assert.equal(latest.mutationResponseNoHiddenMutationDigest,
      "mutation-response-no-hidden-mutation|allDeclaredTargetsAccountedFor|declared:1|accounted:1|exact:0|fallback:1");
    assert.equal(
      saveLine.history().lifecycle.at(-1)?.mutationResponseRefetchRequiredDigest,
      latest.mutationResponseRefetchRequiredDigest,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("deliveryAwaited fallback emits explicit freshness posture instead of hidden manual delivery knowledge", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const profileRead = runtime.signals.api({}).url("/profiles/:profileId").detail({
      load: ({ profileId }) => ({ id: profileId, name: "First" }),
    });
    profileRead.line({ profileId: "p1" });
    const queueProfile = runtime.signals.api({}).url("/profiles/:profileId/queue")
      .response(runtime.signals.resource.response.detail()())
      .update({
        reconciles: [{
          family: profileRead,
          params: ({ profileId }) => ({ profileId }),
          fallback: "deliveryAwaited",
        }],
        load: ({ profileId, body }) => ({ id: profileId, name: body.name }),
      });

    const queueLine = queueProfile.line({
      profileId: "p1",
      body: { name: "Queued" },
    });
    const latest = queueLine.summary().diagnostics.latest;

    assert.equal(latest.mutationResponseFallbackReasonDigest,
      "mutation-response-fallback-reasons|deliveryAwaited:1");
    assert.equal(latest.mutationResponseDeliveryAwaitedDigest,
      "mutation-response-deliveryAwaited-targets|mutationTarget1:detail:"
      + `${profileRead.line({ profileId: "p1" }).descriptor().family.familyId}:/profiles/p1:none:none`);
    assert.equal(latest.mutationResponseRefetchRequiredDigest,
      "mutation-response-refetchRequired-targets|none");
    assert.equal(latest.mutationResponseFreshnessPostureDigest,
      "mutation-response-freshness-posture|deliveryAwaited|targets:1|exact:0|fallback:1");
    assert.equal(
      queueLine.history().verificationPackage().diagnostics.summary.latest
        .mutationResponseDeliveryAwaitedDigest,
      latest.mutationResponseDeliveryAwaitedDigest,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("partial reconciliation fallback still proves every declared target is accounted for", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const detailFields = runtime.signals.resource.detailFields({
      status: {
        read: (value) => value.status,
        write: (value, status) => ({ ...value, status }),
      },
    });
    const workflowDetail = runtime.signals.api({}).url("/workflows/:workflowId").detail({
      reconcile: detailFields,
      load: ({ workflowId }) => ({ id: workflowId, status: "draft" }),
    });
    const workflowSummaries = runtime.signals.api({}).url("/workflow-search")
      .response(runtime.signals.resource.response.collection({
        itemId: (item) => item.id,
        items: (value) => value.items,
        replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
        summaries: runtime.signalsMod.resourceValueSummaries({
          version: {
            read: (value) => value.version,
            write: (value, version) => ({ ...value, version }),
          },
        }),
      }))
      .list({
        load: () => ({
          items: [{ id: "wf-1", status: "draft" }],
          version: 1,
        }),
      });
    workflowDetail.line({ workflowId: "wf-1" });
    workflowSummaries.line({});
    const saveWorkflow = runtime.signals.api({}).url("/workflows/:workflowId")
      .response(runtime.signals.resource.response.detail()({
        status: "status",
        version: "version",
      }))
      .update({
        atomicity: "partialAllowed",
        reconciles: [{
          family: workflowDetail,
          params: ({ workflowId }) => ({ workflowId }),
          fallback: "partialReconciliation",
          detail: { kind: "field", field: "status" },
        }, {
          family: workflowSummaries,
          params: () => ({}),
          fallback: "partialReconciliation",
          summary: { kind: "summary", summary: "version" },
        }],
        load: ({ workflowId }) => ({
          id: workflowId,
          status: "published",
        }),
      });

    const saveLine = saveWorkflow.line({
      workflowId: "wf-1",
      body: {},
    });
    const latest = saveLine.summary().diagnostics.latest;

    assert.equal(latest.mutationResponseFallbackReasonDigest,
      "mutation-response-fallback-reasons|partialReconciliation:1");
    assert.equal(latest.mutationResponseFreshnessPostureDigest,
      "mutation-response-freshness-posture|partialCanonicalTruth|targets:2|exact:1|fallback:1");
    assert.equal(latest.mutationResponseNoHiddenMutationDigest,
      "mutation-response-no-hidden-mutation|allDeclaredTargetsAccountedFor|declared:2|accounted:2|exact:1|fallback:1");
  } finally {
    await runtime.cleanup();
  }
});

test("identityMigrationUnavailable remains explicit alongside route-level fallback honesty reads", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const draftDetail = runtime.signals.api({}).url("/drafts/:draftId").detail({
      load: ({ draftId }) => ({ id: draftId, title: "Draft" }),
    });
    draftDetail.line({ draftId: "draft-7" });
    draftDetail.line({ draftId: "published:draft-7" });
    const publishDraft = runtime.signals.api({}).url("/publish-draft")
      .response(runtime.signals.resource.response.detail()())
      .create({
        identity: {
          submitted: ({ body }) => body.draftId,
          response: (value) => value.id,
          canonical: (value, responseIdentity) => responseIdentity ?? value.id,
          targets: [{
            family: draftDetail,
            params: ({ body }) => ({ draftId: body.draftId }),
            canonicalParams: (_params, _value, canonicalIdentity) => ({
              draftId: canonicalIdentity,
            }),
            fallback: "identityMigrationUnavailable",
          }],
        },
        load: ({ body }) => ({
          id: `published:${body.draftId}`,
          title: body.title,
        }),
      });

    const publishLine = publishDraft.line({
      body: { draftId: "draft-7", title: "Published" },
    });
    const latest = publishLine.summary().diagnostics.latest;

    assert.equal(latest.mutationResponseFallbackReasonDigest,
      "mutation-response-fallback-reasons|none");
    assert.equal(latest.mutationResponseNoHiddenMutationDigest,
      "mutation-response-no-hidden-mutation|allDeclaredTargetsAccountedFor|declared:0|accounted:0|exact:0|fallback:0");
    assert.equal(
      latest.mutationResponseIdentityMigrationFallbackDigest,
      "mutation-response-identity-fallbacks|migrationTarget1:identityMigrationUnavailable:/drafts/draft-7",
    );
  } finally {
    await runtime.cleanup();
  }
});
