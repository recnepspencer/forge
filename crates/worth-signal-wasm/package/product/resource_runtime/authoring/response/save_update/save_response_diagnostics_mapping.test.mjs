import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../../runtime_fixture/real_request_runtime.mjs";

test("save responses map validation and warning fields into typed mutation diagnostics without mutating read truth", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const readFamily = runtime.signals.api({}).url("/workflows/:workflowId").detail({
      load: ({ workflowId }) => ({ id: workflowId, title: "Draft" }),
    });
    const residentLine = readFamily.line({ workflowId: "wf-1" });
    const saveWorkflow = runtime.signals.api({}).url("/workflows/:workflowId")
      .response(runtime.signals.resource.response.detail()({
        workflow: "workflow",
        warnings: "warnings",
        validation: "validation",
      }))
      .update({
        diagnostics: [
          { kind: "warnings", field: "warnings" },
          { kind: "validation", field: "validation" },
        ],
        load: ({ workflowId, body }) => ({
          workflow: { id: workflowId, title: body.title },
          warnings: ["title normalized"],
          validation: [{ path: "title", severity: "info" }],
        }),
      });

    const saveLine = saveWorkflow.line({
      workflowId: "wf-1",
      body: { title: "Saved" },
    });
    const plan = saveLine.mutationResponse();

    assert.deepEqual(residentLine.value(), { id: "wf-1", title: "Draft" });
    assert.equal(plan.targetCount, 0);
    assert.equal(plan.diagnostics.count, 2);
    assert.deepEqual(plan.diagnostics.entries.map((entry) => entry.kind), [
      "warnings",
      "validation",
    ]);
    assert.deepEqual(plan.diagnostics.entries[0].value, ["title normalized"]);
    assert.deepEqual(plan.diagnostics.entries[1].value, [
      { path: "title", severity: "info" },
    ]);
    assert.match(
      plan.diagnostics.digest,
      /mutation-response-diagnostics\|mutationDiagnostic1:warnings:warnings:/,
    );
    assert.equal(plan.counters.diagnosticExtractionBreadth, 2);
    assert.equal(
      saveLine.summary().diagnostics.latest.mutationResponseDiagnosticCount,
      2,
    );
    assert.equal(
      saveLine.summary().diagnostics.latest.mutationResponseDiagnosticDigest,
      plan.diagnostics.digest,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("mutation diagnostic declarations deny undeclared response fields before route materialization", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    assert.throws(
      () =>
        runtime.signals.api({}).url("/workflows/:workflowId")
          .response(runtime.signals.resource.response.detail()({
            warnings: "warnings",
          }))
          .update({
            diagnostics: [
              { kind: "validation", field: "validation" },
            ],
            load: ({ workflowId }) => ({ id: workflowId, warnings: [] }),
          }),
      /diagnostics\[0\] field "validation" is not declared on the mutation response lens/,
    );
    assert.throws(
      () =>
        runtime.signals.api({}).url("/workflow-count")
          .response(runtime.signals.resource.response.summary()())
          .update({
            diagnostics: [
              { kind: "warnings", field: "warnings" },
            ],
            load: () => ({ warnings: [] }),
          }),
      /diagnostics\[0\] requires a detail response lens/,
    );
    assert.throws(
      () =>
        runtime.signals.api({}).url("/workflows")
          .response(runtime.signals.resource.response.detail()({
            warnings: "warnings",
          }))
          .create({
            diagnostics: [
              { kind: "warnings", field: "warnings" },
            ],
            load: ({ body }) => body,
          }),
      /diagnostics are currently admitted only for update\/save responses/,
    );
  } finally {
    await runtime.cleanup();
  }
});
