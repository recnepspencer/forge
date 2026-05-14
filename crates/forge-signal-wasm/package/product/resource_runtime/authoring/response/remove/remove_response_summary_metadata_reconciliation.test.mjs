import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../../runtime_fixture/real_request_runtime.mjs";

test("remove responses can patch count status version and modified metadata summaries in one canonical delete plan", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const taskList = runtime.signals.api({}).url("/tasks")
      .response(runtime.signals.resource.response.collection({
        itemId: (item) => item.id,
        items: (value) => value.items,
        replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
        summaries: runtime.signalsMod.resourceValueSummaries({
          total: {
            read: (value) => value.total,
            write: (value, total) => ({ ...value, total }),
          },
          status: {
            read: (value) => value.status,
            write: (value, status) => ({ ...value, status }),
          },
          version: {
            read: (value) => value.version,
            write: (value, version) => ({ ...value, version }),
          },
          modifiedAt: {
            read: (value) => value.modifiedAt,
            write: (value, modifiedAt) => ({ ...value, modifiedAt }),
          },
        }),
      }))
      .list({
        load: () => ({
          items: [
            { id: "t1", title: "First" },
            { id: "t2", title: "Second" },
          ],
          total: 2,
          status: "active",
          version: 7,
          modifiedAt: "2026-05-12T12:00:00Z",
        }),
      });
    const taskLine = taskList.line({});
    const removeTask = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()({
        total: "total",
        status: "status",
        version: "version",
        modifiedAt: "modifiedAt",
      }))
      .remove({
        reconciles: [{
          family: taskList,
          params: () => ({}),
          fallback: "deletionUnavailable",
          collection: { kind: "delete" },
        }, {
          family: taskList,
          params: () => ({}),
          fallback: "refetchRequired",
          summary: { kind: "summary", summary: "total" },
        }, {
          family: taskList,
          params: () => ({}),
          fallback: "refetchRequired",
          summary: { kind: "summary", summary: "status" },
        }, {
          family: taskList,
          params: () => ({}),
          fallback: "refetchRequired",
          summary: { kind: "summary", summary: "version" },
        }, {
          family: taskList,
          params: () => ({}),
          fallback: "refetchRequired",
          summary: { kind: "summary", summary: "modifiedAt" },
        }],
        load: ({ taskId }) => ({
          id: taskId,
          total: 1,
          status: "archived",
          version: 8,
          modifiedAt: "2026-05-13T09:30:00Z",
        }),
      });

    const removeLine = removeTask.line({ taskId: "t1" });
    const plan = removeLine.mutationResponse();

    assert.deepEqual(taskLine.value(), {
      items: [{ id: "t2", title: "Second" }],
      total: 1,
      status: "archived",
      version: 8,
      modifiedAt: "2026-05-13T09:30:00Z",
    });
    assert.equal(plan.executionArtifacts[0].kind, "exactCollectionDelete");
    assert.deepEqual(
      plan.executionArtifacts.slice(1).map((artifact) => artifact.summary),
      ["total", "status", "version", "modifiedAt"],
    );
    assert.equal(plan.confirmation.kind, "consumedCanonicalTruth");
    assert.equal(plan.confirmation.exactTargetCount, 5);
    assert.equal(plan.counters.appliedTargetBreadth, 5);
    assert.equal(
      removeLine.summary().diagnostics.latest.mutationResponseTargetCount,
      5,
    );
    assert.match(
      removeLine.summary().diagnostics.latest.mutationResponseExecutionDigest,
      /exactSummary/,
    );
  } finally {
    await runtime.cleanup();
  }
});
