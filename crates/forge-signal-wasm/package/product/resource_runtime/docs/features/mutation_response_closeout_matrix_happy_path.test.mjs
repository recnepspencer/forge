import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

import { createRealRequestRuntime } from "../../runtime_fixture/real_request_runtime.mjs";

const docPath = path.resolve(
  "docs/resource-contracts/mutation-response-closeout-matrix.md",
);

test("mutation response closeout matrix doc distinguishes ergonomic denial fallback and out-of-scope lanes", async () => {
  const doc = fs.readFileSync(docPath, "utf8");

  assert.match(doc, /mutationResponses\.closeoutMatrix\(\)/);
  assert.match(doc, /deferredErgonomics/);
  assert.match(doc, /Lane Selection Quick Guide/);
  assert.match(doc, /Supported ergonomic happy path/i);
  assert.match(doc, /Supported precise denial/i);
  assert.match(doc, /Supported typed unavailable fallback/i);
  assert.match(doc, /Intentionally out of scope/i);
  assert.match(doc, /Save detail line replace/);
  assert.match(doc, /Update related collection item/);
  assert.match(doc, /Update related summary/);
  assert.match(doc, /Create with identity migration/);
  assert.match(doc, /Delivery awaited/);
  assert.match(doc, /Overclaimed detail\/path\/region\/summary\/placement\/deletion\/identity declarations/);
  assert.match(doc, /Hidden best-effort mutation of undeclared read truth/);

  const runtime = await createRealRequestRuntime();
  try {
    const matrix = runtime.signals.resource.mutationResponses.closeoutMatrix();
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
    const detailLine = taskDetail.line({ taskId: "task:1" });

    const exactSaveLine = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()())
      .update({
        reconciles: [{
          family: taskDetail,
          params: ({ taskId }) => ({ taskId }),
          fallback: "refetchRequired",
          detail: { kind: "replace" },
        }],
        load: ({ taskId }) => ({ id: taskId, status: "published" }),
      })
      .line({
        taskId: "task:1",
        body: {},
      });

    const permissionFields = runtime.signals.resource.detailFields({
      canEdit: {
        read: (value) => value.canEdit,
        write: (value, canEdit) => ({ ...value, canEdit }),
      },
    });
    const permissionsDetail = runtime.signals.api({}).url("/task-permissions/:taskId").detail({
      reconcile: permissionFields,
      load: ({ taskId }) => ({ id: taskId, canEdit: false }),
    });
    permissionsDetail.line({ taskId: "task:1" });
    const fallbackLine = runtime.signals.api({}).url("/task-permissions/:taskId")
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
        load: () => ({ warnings: ["permission delivery expected"] }),
      })
      .line({
        taskId: "task:1",
        body: {},
      });

    assert.ok(matrix.proofLanes.includes("closeout"));
    assert.deepEqual(matrix.deferredErgonomics, []);
    assert.equal(
      matrix.rows.find((row) => row.lane === "deliveryAwaited")?.category,
      "supportedTypedUnavailableFallback",
    );
    assert.equal(
      exactSaveLine.mutationResponse().confirmation.kind,
      "consumedCanonicalTruth",
    );
    assert.equal(
      exactSaveLine.summary().diagnostics.latest.mutationResponseTargetOutcomes[0].outcomeKind,
      "exact",
    );
    assert.equal(
      fallbackLine.summary().diagnostics.latest.mutationResponseFallbackReasonDigest,
      "mutation-response-fallback-reasons|deliveryAwaited:1",
    );
    assert.equal(
      fallbackLine.summary().diagnostics.latest.mutationResponseTargetOutcomes[0].fallbackKind,
      "deliveryAwaited",
    );
    assert.equal(
      detailLine.value().status,
      "published",
    );

    assert.throws(
      () =>
        runtime.signals.api({}).url("/tasks/:taskId")
          .response(runtime.signals.resource.response.detail()())
          .update({
            reconciles: [{
              family: taskDetail,
              params: ({ taskId }) => ({ taskId }),
              fallback: "refetchRequired",
              detail: { kind: "field", field: "missing" },
            }],
            load: ({ taskId }) => ({ id: taskId, status: "published" }),
          }),
      /detail\.field "missing" is not declared on the target detail family/,
    );
  } finally {
    await runtime.cleanup();
  }
});
