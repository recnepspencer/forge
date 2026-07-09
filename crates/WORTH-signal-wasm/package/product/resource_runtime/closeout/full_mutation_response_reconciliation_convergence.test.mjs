import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../runtime_fixture/real_request_runtime.mjs";
import { createBranchHead } from "../runtime_fixture/real_resource_signals.mjs";
import {
  createDeferred,
  projectMutationCloseoutSnapshot,
  settleRuntime,
} from "./mutation_response_closeout_snapshot_helpers.mjs";

test("full mutation response reconciliation closeout keeps create save partial fallback remove and restore surfaces coherent", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    createBranchHead(runtime.signals, "full-mutation-response-closeout");
    const api = runtime.signals.api({
      effects: runtime.signals.resource.effects.branchNative(),
    });
    const taskFields = runtime.signals.resource.detailFields({
      status: {
        read: (value) => value.status,
        write: (value, status) => ({ ...value, status }),
      },
    });
    const permissionFields = runtime.signals.resource.detailFields({
      canEdit: {
        read: (value) => value.canEdit,
        write: (value, canEdit) => ({ ...value, canEdit }),
      },
    });
    const taskDetail = api.url("/tasks/:taskId").detail({
      reconcile: taskFields,
      load: ({ taskId }) => ({ id: taskId, status: "draft" }),
    });
    const taskList = api.url("/tasks")
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
          items: [{ id: "task:existing", status: "active" }],
          version: 1,
        }),
      });
    const taskCounts = api.url("/task-counts")
      .response(runtime.signals.resource.response.collection({
        itemId: (item) => item.id,
        items: (value) => value.items,
        replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
        summaries: runtime.signalsMod.resourceValueSummaries({
          total: {
            read: (value) => value.total,
            write: (value, total) => ({ ...value, total }),
          },
        }),
      }))
      .list({
        load: () => ({
          items: [],
          total: 1,
        }),
      });
    const permissionsDetail = api.url("/task-permissions/:taskId").detail({
      reconcile: permissionFields,
      load: ({ taskId }) => ({ id: taskId, canEdit: false }),
    });
    const staleDetail = api.url("/stale-tasks/:taskId")
      .response(runtime.signals.resource.response.detail()({ status: "status" }))
      .detail({
        load: ({ taskId }) => ({ id: taskId, status: "draft" }),
      });

    const taskListLine = taskList.line({});
    const taskCountsLine = taskCounts.line({});
    const taskDetailLine = taskDetail.line({ taskId: "tmp-task-1" });
    const permissionsLine = permissionsDetail.line({ taskId: "task:tmp-task-1" });
    const staleDetailLine = staleDetail.line({ taskId: "stale-1" });

    const createLine = api.url("/tasks")
      .response(runtime.signals.resource.response.detail()({ total: "total" }))
      .create({
        reconciles: [{
          family: taskList,
          params: () => ({}),
          fallback: "placementUnavailable",
          collection: { kind: "insert", placement: "append" },
        }, {
          family: taskCounts,
          params: () => ({}),
          fallback: "refetchRequired",
          summary: { kind: "summary", summary: "total" },
        }],
        identity: {
          submitted: ({ body }) => body.id,
          response: (value) => value.id,
          canonical: (value, responseIdentity) => responseIdentity ?? value.id,
          targets: [{
            family: taskDetail,
            params: ({ body }) => ({ taskId: body.id }),
            canonicalParams: (_params, _value, canonicalIdentity) => ({
              taskId: canonicalIdentity,
            }),
            fallback: "identityMigrationUnavailable",
          }],
        },
        load: ({ body }) => ({
          id: `task:${body.id}`,
          status: "created",
          total: 2,
        }),
      })
      .line({
        body: { id: "tmp-task-1" },
      });

    const exactSaveLine = api.url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()({
        status: "status",
        version: "version",
      }))
      .update({
        reconciles: [{
          family: taskDetail,
          params: ({ taskId }) => ({ taskId }),
          fallback: "partialReconciliation",
          detail: { kind: "field", field: "status" },
        }, {
          family: taskList,
          params: () => ({}),
          fallback: "partialReconciliation",
          summary: { kind: "summary", summary: "version" },
        }],
        load: ({ taskId }) => ({ id: taskId, status: "published", version: 2 }),
      })
      .line({
        taskId: "task:tmp-task-1",
        body: {},
      });
    const collectionUpdateLine = api.url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()())
      .update({
        reconciles: [{
          family: taskList,
          params: () => ({}),
          fallback: "refetchRequired",
          collection: { kind: "item" },
        }],
        load: ({ taskId }) => ({ id: taskId, status: "reviewed" }),
      })
      .line({
        taskId: "task:existing",
        body: {},
      });
    assert.equal(taskListLine.value().items.find((item) => item.id === "task:existing")?.status, "reviewed");

    const partialSaveLine = api.url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()({
        status: "status",
        version: "version",
        warnings: "warnings",
      }))
      .update({
        atomicity: "partialAllowed",
        reconciles: [{
          family: taskDetail,
          params: ({ taskId }) => ({ taskId }),
          fallback: "partialReconciliation",
          detail: { kind: "field", field: "status" },
        }, {
          family: taskList,
          params: () => ({}),
          fallback: "partialReconciliation",
          summary: { kind: "summary", summary: "version" },
        }],
        diagnostics: [{ kind: "warnings", field: "warnings" }],
        load: ({ taskId }) => ({ id: taskId, status: "ready", warnings: ["version pending"] }),
      })
      .line({
        taskId: "task:tmp-task-1",
        body: {},
      });

    const deliveryFallbackLine = api.url("/tasks/:taskId/permissions")
      .response(runtime.signals.resource.response.detail()({
        canEdit: "canEdit",
        warnings: "warnings",
      }))
      .update({
        reconciles: [{
          family: permissionsDetail,
          params: ({ taskId }) => ({ taskId }),
          fallback: "deliveryAwaited",
          detail: { kind: "field", field: "canEdit" },
        }],
        diagnostics: [{ kind: "warnings", field: "warnings" }],
        load: () => ({ warnings: ["permission delivery expected"] }),
      })
      .line({
        taskId: "task:tmp-task-1",
        body: {},
      });

    const staleDeferred = createDeferred();
    const staleWrite = api.url("/stale-tasks/:taskId")
      .response(runtime.signals.resource.response.detail()())
      .update({
        reconciles: [{
          family: staleDetail,
          params: ({ taskId }) => ({ taskId }),
          fallback: "refetchRequired",
          detail: { kind: "replace" },
        }],
        load: async ({ taskId }) => {
          await staleDeferred.promise;
          return { id: taskId, status: "server" };
        },
      });
    const staleLine = staleWrite.line({
      taskId: "stale-1",
      body: {},
    });
    staleDetailLine.patch(staleDetail.patch.field({
      field: "status",
      value: "local-newer",
    }));
    staleDeferred.resolve();
    await settleRuntime();
    const staleSaveLine = staleLine;

    const removeLine = api.url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()({ total: "total" }))
      .remove({
        reconciles: [{
          family: taskList,
          params: () => ({}),
          fallback: "deletionUnavailable",
          collection: { kind: "delete" },
        }, {
          family: taskCounts,
          params: () => ({}),
          fallback: "refetchRequired",
          summary: { kind: "summary", summary: "total" },
        }],
        load: ({ taskId }) => ({ id: taskId, total: 1 }),
      })
      .line({
        taskId: "task:tmp-task-1",
      });
    const duplicateDeleteArtifact = removeLine.mutationResponse().executionArtifacts[0];
    taskListLine.deliver(taskList.delivery.delete({
      packetId: duplicateDeleteArtifact.packetId,
      basisId: taskListLine.diagnostics().basis.currentBasisId,
      nextBasisId: taskListLine.diagnostics().basis.currentBasisId,
      itemId: "task:tmp-task-1",
    }));
    const duplicateRemoveLine = removeLine;

    const forwardSnapshot = projectMutationCloseoutSnapshot({
      createLine,
      collectionUpdateLine,
      exactSaveLine,
      partialSaveLine,
      deliveryFallbackLine,
      staleSaveLine,
      removeLine,
      duplicateRemoveLine,
      taskListLine,
      taskCountsLine,
      taskDetailLine,
      permissionsLine,
      staleDetailLine,
    });

    const restoreResult = taskListLine.history().restoreExact();
    await settleRuntime();
    const restoredSnapshot = projectMutationCloseoutSnapshot({
      taskListLine,
      taskCountsLine,
      taskDetailLine,
      permissionsLine,
      staleDetailLine,
    });
    const replayResults = {
      taskList: taskListLine.history().replayExact(),
      taskDetail: taskDetailLine.history().replayExact(),
      staleDetail: staleDetailLine.history().replayExact(),
    };
    await settleRuntime();
    const replayStableSnapshot = projectMutationCloseoutSnapshot({
      taskListLine,
      taskCountsLine,
      taskDetailLine,
      permissionsLine,
      staleDetailLine,
    });

    assert.equal(forwardSnapshot.create.confirmationKind, "consumedCanonicalTruth");
    assert.equal(forwardSnapshot.collectionUpdate.confirmationKind, "consumedCanonicalTruth");
    assert.equal(forwardSnapshot.exactSave.confirmationKind, "consumedCanonicalTruth");
    assert.ok(forwardSnapshot.collectionUpdate.targetDigest.includes("collection"));
    assert.ok(collectionUpdateLine.mutationResponse().executionArtifacts.some((artifact) => artifact.kind === "exactCollectionItem"));
    assert.equal(forwardSnapshot.partialSave.confirmationKind, "partialCanonicalTruth");
    assert.equal(forwardSnapshot.deliveryFallback.confirmationKind, "deliveryAwaited");
    assert.equal(forwardSnapshot.staleSave.confirmationKind, "refetchRequired");
    assert.equal(forwardSnapshot.remove.confirmationKind, "consumedCanonicalTruth");
    assert.match(forwardSnapshot.create.identityMigrationDigest, /mutation-response-identity-submitted\|tmp-task-1/);
    assert.match(forwardSnapshot.partialSave.fallbackReasonDigest, /partialReconciliation:1/);
    assert.match(forwardSnapshot.deliveryFallback.deliveryAwaitedDigest, /deliveryAwaited-targets/);
    assert.match(forwardSnapshot.staleSave.refetchRequiredDigest, /refetchRequired-targets/);
    assert.equal(forwardSnapshot.staleSave.staleTargetReasonDigest, "mutation-response-stale-target-reasons|visibleValueVersionChanged:1");
    assert.match(forwardSnapshot.exactSave.replayExactDigest, /available:SameRuntimeSignalExact/);
    assert.match(forwardSnapshot.exactSave.restoreExactDigest, /available:SameRuntimeBranchExact/);
    assert.equal(forwardSnapshot.exactSave.summaryReplayExactDigest, forwardSnapshot.exactSave.replayExactDigest);
    assert.equal(forwardSnapshot.exactSave.summaryRestoreExactDigest, forwardSnapshot.exactSave.restoreExactDigest);
    assert.match(forwardSnapshot.remove.mergeRebaseDigest, /summary:total/);
    assert.equal(
      forwardSnapshot.duplicateRemove.staleTargetReasonDigest,
      "mutation-response-stale-target-reasons|none",
    );
    assert.equal(
      forwardSnapshot.duplicateRemove.fallbackReasonDigest,
      "mutation-response-fallback-reasons|none",
    );
    assert.deepEqual(forwardSnapshot.exactSave.boundaryPerformanceEnvelope, {
      lifecycleEntryCount: 2,
      downloadDescriptorCount: 0,
      summaryReadShape: "inspectionSummary",
      commonLineReadShape: "groupedLineSummary",
    });
    assert.deepEqual(forwardSnapshot.taskList.boundaryPerformanceEnvelope, {
      lifecycleEntryCount: 5,
      downloadDescriptorCount: 0,
      summaryReadShape: "inspectionSummary",
      commonLineReadShape: "groupedLineSummary",
    });
    assert.equal(forwardSnapshot.taskDetail.restoreExact.kind, "unavailable");
    assert.equal(forwardSnapshot.taskDetail.restoreExact.reason, "identityMigrationUnavailable");
    assert.equal(forwardSnapshot.taskList.restoreExact.kind, "available");
    assert.equal(restoreResult.kind, "restored");
    assert.deepEqual(replayResults.taskList, {
      kind: "unavailable",
      reason: "unsupportedByRuntime",
      detail:
        "resource line exact replay is unavailable because the Signals runtime does not expose replay_signal_by_id(...)",
      basisCurrentId: null,
      basisAdvanceCount: 0,
    });
    assert.deepEqual(replayResults.taskDetail, {
      kind: "unavailable",
      reason: "identityMigrationUnavailable",
      detail: forwardSnapshot.taskDetail.typedReplayExact.detail,
      basisCurrentId: null,
      basisAdvanceCount: 0,
    });
    assert.deepEqual(replayResults.staleDetail, {
      kind: "unavailable",
      reason: "unsupportedByRuntime",
      detail:
        "resource line exact replay is unavailable because the Signals runtime does not expose replay_signal_by_id(...)",
      basisCurrentId: null,
      basisAdvanceCount: 0,
    });
    assert.deepEqual(replayStableSnapshot, restoredSnapshot);
    assert.deepEqual(restoredSnapshot.taskList.committedValue, {
      items: [{ id: "task:existing", status: "active" }],
      version: 1,
    });
    assert.equal(restoredSnapshot.taskList.latestLifecycleEvent, "restored");
    assert.deepEqual(forwardSnapshot.taskCounts.committedValue, {
      items: [],
      total: 1,
    });
    assert.deepEqual(forwardSnapshot.permissions.committedValue, {
      id: "task:tmp-task-1",
      canEdit: false,
    });
    assert.deepEqual(forwardSnapshot.staleDetail.committedValue, {
      id: "stale-1",
      status: "local-newer",
    });
  } finally {
    await runtime.cleanup();
  }
});
