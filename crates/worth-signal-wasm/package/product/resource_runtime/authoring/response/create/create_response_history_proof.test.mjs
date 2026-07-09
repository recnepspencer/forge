import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../../runtime_fixture/real_request_runtime.mjs";
import { createBranchHead } from "../../../runtime_fixture/real_resource_signals.mjs";

test("create exact placement remains exact-restorable while the migrated draft line denies exact replay and restore", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    createBranchHead(runtime.signals, "create-history-placement-and-migration");
    const taskList = runtime.signals.api({
      effects: runtime.signals.resource.effects.branchNative(),
    }).url("/tasks")
      .response(runtime.signals.resource.response.array({
        itemId: (item) => item.id,
      }))
      .list({
        load: () => [{ id: "task:existing", title: "Existing" }],
      });
    const draftDetail = runtime.signals.api({
      effects: runtime.signals.resource.effects.branchNative(),
    }).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()())
      .detail({
        load: ({ taskId }) => ({ id: taskId, title: "Draft" }),
      });
    const listLine = taskList.line({});
    const draftLine = draftDetail.line({ taskId: "tmp-create-history-1" });

    runtime.signals.api({
      effects: runtime.signals.resource.effects.branchNative(),
    }).url("/tasks")
      .response(runtime.signals.resource.response.detail()())
      .create({
        reconciles: [{
          family: taskList,
          params: () => ({}),
          fallback: "placementUnavailable",
          collection: { kind: "insert", placement: "append" },
        }],
        identity: {
          submitted: ({ body }) => body.id,
          response: (value) => value.id,
          canonical: (value, responseIdentity) => responseIdentity ?? value.id,
          targets: [{
            family: draftDetail,
            params: ({ body }) => ({ taskId: body.id }),
            canonicalParams: (_params, _value, canonicalIdentity) => ({
              taskId: canonicalIdentity,
            }),
            fallback: "identityMigrationUnavailable",
          }],
        },
        load: ({ body }) => ({ id: `task:${body.id}`, title: body.title }),
      })
      .line({
        body: { id: "tmp-create-history-1", title: "Created" },
      })
      .mutationResponse();

    const listRestoreAvailability = listLine.history().availability.restoreExact;
    const draftReplayAvailability = draftLine.history().availability.replayExact;
    const draftRestoreAvailability = draftLine.history().availability.restoreExact;
    const listRestoreResult = listLine.history().restoreExact();

    assert.equal(listRestoreAvailability.kind, "available");
    assert.equal(listRestoreAvailability.mode, "SameRuntimeBranchExact");
    assert.equal(draftReplayAvailability.kind, "unavailable");
    assert.equal(draftReplayAvailability.reason, "identityMigrationUnavailable");
    assert.match(draftReplayAvailability.detail, /identity migration rewrote/);
    assert.equal(draftRestoreAvailability.kind, "unavailable");
    assert.equal(draftRestoreAvailability.reason, "identityMigrationUnavailable");
    assert.match(draftRestoreAvailability.detail, /resident rematerialization/);
    assert.equal(listRestoreResult.kind, "restored");
    assert.deepEqual(listLine.value(), [{ id: "task:existing", title: "Existing" }]);
    assert.equal(listLine.history().lifecycle.at(-1)?.event, "restored");

    const verification = draftLine.history().verificationPackage();
    assert.deepEqual(verification.typedDenials.replayExact, draftReplayAvailability);
    assert.deepEqual(verification.typedDenials.restoreExact, draftRestoreAvailability);
    assert.deepEqual(
      verification.historyReplayRestore.availability.restoreExact,
      draftRestoreAvailability,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("create effect-backed detail-child migration remains exact-restorable on the parent detail line", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    createBranchHead(runtime.signals, "create-history-detail-child-restore");
    const taskRegions = runtime.signals.resource.detailRegions({
      children: {
        read: (value) => value.children,
        write: (value, children) => ({ ...value, children }),
        identityBoundary: "inside",
        mergeGranularity: "child-list",
        cost: {
          traversalBreadth: 1,
          reconstructionBreadth: 1,
        },
      },
    });
    const taskDetail = runtime.signals.api({
      effects: runtime.signals.resource.effects.branchNative(),
    }).url("/tasks/:taskId")
      .detail({
        reconcile: taskRegions,
        load: ({ taskId }) => ({
          id: taskId,
          title: "Task",
          children: [{ id: "tmp-create-history-child", title: "Draft child" }],
        }),
      });
    const taskLine = taskDetail.line({ taskId: "task-create-history-2" });

    runtime.signals.api({
      effects: runtime.signals.resource.effects.branchNative(),
    }).url("/tasks/:taskId/children")
      .response(runtime.signals.resource.response.detailRegions()(taskRegions))
      .create({
        identity: {
          submitted: ({ body }) => body.id,
          response: (value) => value.children.at(-1)?.id ?? value.id,
          canonical: (value, responseIdentity) => responseIdentity ?? value.id,
          targets: [{
            family: taskDetail,
            params: ({ taskId }) => ({ taskId }),
            fallback: "identityMigrationUnavailable",
            detailChild: {
              kind: "detailChild",
              region: "children",
            },
          }],
        },
        load: ({ taskId, body }) => ({
          id: taskId,
          title: "Task",
          children: [{ id: `child:${body.id}`, title: body.title }],
        }),
      })
      .line({
        taskId: "task-create-history-2",
        body: { id: "tmp-create-history-child", title: "Draft child" },
      })
      .mutationResponse();

    const restoreAvailability = taskLine.history().availability.restoreExact;
    const restoreResult = taskLine.history().restoreExact();
    const verification = taskLine.history().verificationPackage();

    assert.equal(restoreAvailability.kind, "available");
    assert.equal(restoreAvailability.mode, "SameRuntimeBranchExact");
    assert.equal(restoreResult.kind, "restored");
    assert.deepEqual(taskLine.value().children, [{
      id: "tmp-create-history-child",
      title: "Draft child",
    }]);
    assert.equal(taskLine.history().lifecycle.at(-1)?.event, "restored");
    assert.equal(verification.typedDenials.restoreExact, null);
    assert.deepEqual(
      verification.historyReplayRestore.availability.restoreExact,
      restoreAvailability,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("create exact detail materialization remains exact-restorable on the created detail line", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    createBranchHead(runtime.signals, "create-history-detail-materialization");
    const taskDetail = runtime.signals.api({
      effects: runtime.signals.resource.effects.branchNative(),
    }).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()())
      .detail({
        load: ({ taskId }) => ({ id: taskId, title: "Draft" }),
      });

    runtime.signals.api({
      effects: runtime.signals.resource.effects.branchNative(),
    }).url("/tasks")
      .response(runtime.signals.resource.response.detail()())
      .create({
        reconciles: [{
          family: taskDetail,
          params: ({ body }) => ({ taskId: body.id }),
          fallback: "refetchRequired",
          detail: { kind: "replace" },
        }],
        load: ({ body }) => ({ id: body.id, title: body.title }),
      })
      .line({
        body: { id: "task-create-history-3", title: "Created" },
      })
      .mutationResponse();

    const createdLine = taskDetail.line({ taskId: "task-create-history-3" });
    const restoreAvailability = createdLine.history().availability.restoreExact;
    const replayAvailability = createdLine.history().availability.replayExact;
    const restoreResult = createdLine.history().restoreExact();
    const verification = createdLine.history().verificationPackage();

    assert.equal(restoreAvailability.kind, "available");
    assert.equal(restoreAvailability.mode, "SameRuntimeBranchExact");
    assert.equal(replayAvailability.kind, "unavailable");
    assert.equal(replayAvailability.reason, "unsupportedByRuntime");
    assert.equal(restoreResult.kind, "restored");
    assert.deepEqual(createdLine.value(), {
      id: "task-create-history-3",
      title: "Draft",
    });
    assert.deepEqual(
      createdLine.history().lifecycle.map((entry) => entry.event),
      ["materialized", "delivered", "restored"],
    );
    assert.equal(verification.typedDenials.restoreExact, null);
    assert.deepEqual(
      verification.historyReplayRestore.availability.restoreExact,
      restoreAvailability,
    );
  } finally {
    await runtime.cleanup();
  }
});
