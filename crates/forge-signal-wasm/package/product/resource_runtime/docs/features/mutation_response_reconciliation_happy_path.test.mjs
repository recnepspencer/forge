import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

import { createRealRequestRuntime } from "../../runtime_fixture/real_request_runtime.mjs";

const docPath = path.resolve(
  "docs/resources/mutation-response-reconciliation.md",
);

test("mutation response reconciliation doc happy path covers exact and fallback target evidence", async () => {
  const doc = fs.readFileSync(docPath, "utf8");

  assert.match(doc, /Stable Entry Points/);
  assert.match(doc, /How It Executes/);
  assert.match(doc, /Update Related Collection Items/);
  assert.match(doc, /Patch Related Summaries/);
  assert.match(doc, /How To Choose A Lane/);
  assert.match(doc, /Inspection And Debugging/);
  assert.match(doc, /Save And Replace Detail Lines/);
  assert.match(doc, /Delete, Exact Removal, And Tombstone Posture/);
  assert.match(doc, /Multi-Family Reconciliation/);
  assert.match(doc, /Closeout Matrix/);
  assert.match(doc, /collection: \{ kind: "item" \}/);
  assert.match(doc, /summary: \{ kind: "summary", summary: "\.\.\." \}/);
  assert.match(doc, /line\.history\(\)\.restoreExact\(\)/);
  assert.match(doc, /line\.history\(\)\.replayExact\(\)/);
  assert.match(doc, /signals\.resource\.mutationResponses\.closeoutMatrix\(\)/);
  assert.match(doc, /Supported ergonomic happy paths/i);
  assert.match(doc, /Supported precise denials/i);
  assert.match(doc, /Supported typed unavailable fallbacks/i);
  assert.match(doc, /Intentionally out-of-scope work/i);
  assert.match(doc, /Deferred product ergonomics/i);
  assert.match(doc, /deferredErgonomics/);

  const runtime = await createRealRequestRuntime();
  try {
    const taskFields = runtime.signals.resource.detailFields({
      status: {
        read: (value) => value.status,
        write: (value, status) => ({ ...value, status }),
      },
    });
    const taskDetail = runtime.signals.api({}).url("/tasks/:taskId").detail({
      reconcile: taskFields,
      load: ({ taskId }) => ({ id: taskId, status: "draft" }),
    });
    const taskList = runtime.signals.api({}).url("/tasks")
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
          items: [{ id: "task:1", status: "draft" }],
          version: 1,
        }),
      });

    taskDetail.line({ taskId: "task:1" });
    taskList.line({});
    const line = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()({
        status: "status",
        version: "version",
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
        load: ({ taskId }) => ({ id: taskId, status: "published" }),
      })
      .line({
        taskId: "task:1",
        body: {},
      });

    assert.equal(line.mutationResponse().partialAdmission, "admitted");
    assert.equal(
      line.summary().diagnostics.latest.mutationResponseTargetOutcomes.length,
      2,
    );
    assert.equal(
      line.summary().diagnostics.latest.mutationResponseFallbackReasonDigest,
      "mutation-response-fallback-reasons|partialReconciliation:1",
    );
  } finally {
    await runtime.cleanup();
  }
});
