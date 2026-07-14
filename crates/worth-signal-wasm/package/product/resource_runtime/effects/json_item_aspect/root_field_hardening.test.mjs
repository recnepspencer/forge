import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../runtime_fixture/real_request_runtime.mjs";

test("JSON path aspects deny root item field accessors without invoking them", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals } = runtime;
    let getterReadCount = 0;
    const response = createMetadataPriorityResponse(signals);
    const accessorItem = { id: "t1" };
    Object.defineProperty(accessorItem, "metadata", {
      enumerable: true,
      get() {
        getterReadCount += 1;
        return { priority: 1 };
      },
    });

    assert.throws(
      () => response.aspects.definitions.priority.read(accessorItem),
      /rejects accessor JSON item field "metadata"/,
    );
    assert.throws(
      () => response.aspects.definitions.priority.write(accessorItem, 2),
      /rejects accessor JSON item field "metadata"/,
    );
    assert.equal(getterReadCount, 0);
  } finally {
    await runtime.cleanup();
  }
});

test("JSON path aspect writes deny unrelated root item accessors before cloning", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals } = runtime;
    let getterReadCount = 0;
    const response = createMetadataPriorityResponse(signals);
    const accessorItem = { id: "t1", metadata: { priority: 1 } };
    Object.defineProperty(accessorItem, "sideEffect", {
      enumerable: true,
      get() {
        getterReadCount += 1;
        return "side effect";
      },
    });

    assert.throws(
      () => response.aspects.definitions.priority.write(accessorItem, 2),
      /rejects accessor JSON item field "sideEffect"/,
    );
    assert.equal(getterReadCount, 0);
  } finally {
    await runtime.cleanup();
  }
});

test("JSON path declarations reject unsafe root fields", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals } = runtime;

    assert.throws(
      () =>
        signals.resource.response.jsonPathAspects()({
          polluted: { field: "__proto__", path: ["priority"] },
        }),
      /rejects unsafe path segment "__proto__"/,
    );
  } finally {
    await runtime.cleanup();
  }
});

function createMetadataPriorityResponse(signals) {
  return signals.resource.response.objectItems()({
    field: "tasks",
    itemId: (task) => task.id,
    aspects: signals.resource.response.jsonPathAspects()({
      priority: { field: "metadata", path: ["priority"] },
    }),
  });
}
