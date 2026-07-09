import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../runtime_fixture/real_request_runtime.mjs";

test("detail JSON path response contracts expose detail family helpers and narrow reconciliation truth", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const response = runtime.signals.resource.response.detailJsonPaths()({
      title: { path: ["document", "title"] },
    });
    assert.equal(response.kind, "detail");
    assert.deepEqual(response.lensProof.jsonPathNames, ["title"]);
    assert.equal(
      response.lensProof.capabilityRows.some(
        (row) => row.locus === "detailJsonPath" && row.patchScope === "jsonPath",
      ),
      true,
    );

    const workflow = runtime.signals.api({}).url("/workflows/:workflowId").response(response).detail({
      load: ({ workflowId }) => ({
        id: workflowId,
        document: { title: "First" },
      }),
    });
    const pathPatch = workflow.patch.jsonPath({
      path: "title",
      value: "Updated",
    });
    const pathDelivery = workflow.delivery.jsonPath({
      packetId: "pkt-workflow-title",
      basisId: null,
      nextBasisId: "basis-1",
      path: "title",
      value: "Delivered",
    });
    const line = workflow.line({ workflowId: "wf-1" });

    assert.equal(pathPatch.kind, "jsonPath");
    assert.equal(pathDelivery.patch.kind, "jsonPath");
    assert.equal(line.reconciliation().narrowJsonPath, true);
    assert.deepEqual(line.reconciliation().jsonPathNames, ["title"]);

    line.patch(pathPatch);
    assert.deepEqual(line.value(), {
      id: "wf-1",
      document: { title: "Updated" },
    });

    line.deliver(pathDelivery);
    assert.deepEqual(line.value(), {
      id: "wf-1",
      document: { title: "Delivered" },
    });
  } finally {
    await runtime.cleanup();
  }
});

test("detail JSON path response contracts deny accessor-backed detail values without invoking them", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const response = runtime.signals.resource.response.detailJsonPaths()({
      title: { path: ["document", "title"] },
    });
    let getterReadCount = 0;
    const accessorValue = {};
    Object.defineProperty(accessorValue, "document", {
      enumerable: true,
      get() {
        getterReadCount += 1;
        return { title: "Hidden" };
      },
    });

    assert.throws(
      () => response.jsonPaths.definitions.title.read(accessorValue),
      /rejects accessor JSON path segment "document"/,
    );
    assert.throws(
      () => response.jsonPaths.definitions.title.write(accessorValue, "Updated"),
      /rejects accessor JSON path segment "document"/,
    );
    assert.equal(getterReadCount, 0);
  } finally {
    await runtime.cleanup();
  }
});

test("detail JSON path declarations reject unsafe path segments", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    assert.throws(
      () =>
        runtime.signals.resource.response.detailJsonPaths()({
          polluted: { path: ["__proto__", "title"] },
        }),
      /rejects unsafe path segment "__proto__"/,
    );
  } finally {
    await runtime.cleanup();
  }
});
