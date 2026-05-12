import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../runtime_fixture/real_request_runtime.mjs";
import { createBranchHead } from "../../runtime_fixture/real_resource_signals.mjs";

test("JSON path aspect writes deny non-plain object values before effects", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals } = runtime;
    const tasks = createJsonMetadataApi(signals);
    const line = tasks.line({});
    const beforeValue = line.value();
    const beforeEffect = line.diagnostics().lastEffect;

    for (const rejectedValue of [
      new Date("2026-05-11T00:00:00.000Z"),
      new Map([["label", "mapped"]]),
      new CustomJsonLikeValue("custom"),
    ]) {
      assert.throws(
        () =>
          line.patch(tasks.patch.itemAspect({
            itemId: "t1",
            aspect: "payload",
            value: rejectedValue,
          })),
        /rejects non-plain JSON objects/,
      );
      assert.deepEqual(line.value(), beforeValue);
      assert.equal(line.diagnostics().lastEffect, beforeEffect);
    }
  } finally {
    await runtime.cleanup();
  }
});

test("JSON path aspect reads and writes deny non-plain object containers", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals } = runtime;
    const response = createMetadataPayloadResponse(signals);
    const item = {
      id: "t1",
      metadata: new CustomJsonLikeValue("custom"),
    };

    assert.throws(
      () => response.aspects.definitions.payload.read(item),
      /rejects non-plain JSON objects/,
    );
    assert.throws(
      () => response.aspects.definitions.payload.write(item, "next"),
      /rejects non-plain JSON objects/,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("JSON path aspect writes admit null-prototype JSON dictionaries", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals } = runtime;
    const tasks = createJsonMetadataApi(signals);
    const line = tasks.line({});
    const dictionary = Object.create(null);
    dictionary.label = "dictionary";

    line.patch(tasks.patch.itemAspect({
      itemId: "t1",
      aspect: "payload",
      value: dictionary,
    }));

    assert.deepEqual(
      Object.entries(line.value().tasks[0].metadata.payload),
      [["label", "dictionary"]],
    );
  } finally {
    await runtime.cleanup();
  }
});

test("JSON path aspect writes preserve null-prototype containers during reconstruction", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals } = runtime;
    const response = createMetadataPayloadResponse(signals);
    const metadata = Object.create(null);
    const payload = Object.create(null);
    payload.label = "dictionary";
    payload.keep = true;
    metadata.payload = payload;

    const nextItem = response.aspects.definitions.payloadLabel.write(
      { id: "t1", metadata },
      "copied",
    );

    assert.equal(Object.getPrototypeOf(nextItem.metadata), null);
    assert.equal(Object.getPrototypeOf(nextItem.metadata.payload), null);
    assert.equal(nextItem.metadata.payload.label, "copied");
    assert.equal(nextItem.metadata.payload.keep, true);
    assert.equal(payload.label, "dictionary");
    assert.notEqual(nextItem.metadata, metadata);
    assert.notEqual(nextItem.metadata.payload, payload);

    createBranchHead(signals, "json-path-null-prototype-reconstruction-proof");
    const tasks = createJsonMetadataApi(signals, metadata);
    const line = tasks.line({});

    line.patch(tasks.patch.itemAspect({
      itemId: "t1",
      aspect: "payloadLabel",
      value: "line-copied",
    }));
    const effect = line.diagnostics().lastEffect;
    const linePayload = line.value().tasks[0].metadata.payload;

    assertJsonPlainOrNullObject(line.value().tasks[0].metadata);
    assertJsonPlainOrNullObject(linePayload);
    assert.equal(linePayload.label, "line-copied");
    assert.equal(linePayload.keep, true);
    assert.equal(effect.locus.kind, "jsonItemAspect");
    assert.equal(
      effect.patch.jsonPath.policy.prototypeReconstruction,
      "plainOrNullCopy",
    );

    assert.equal(line.history().rollbackLastEffect().kind, "rolledBack");
    assert.equal(line.value().tasks[0].metadata.payload.label, "dictionary");
    assert.equal(line.value().tasks[0].metadata.payload.keep, true);
  } finally {
    await runtime.cleanup();
  }
});

test("JSON path immutable-copy policy covers sealed and non-extensible containers", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals } = runtime;
    createBranchHead(signals, "json-path-sealed-copy-proof");
    const sealedPayload = Object.seal({ label: "sealed", keep: true });
    const sealedMetadata = Object.seal({ payload: sealedPayload });
    const tasks = createJsonMetadataApi(signals, sealedMetadata);
    const line = tasks.line({});

    line.patch(tasks.patch.itemAspect({
      itemId: "t1",
      aspect: "payloadLabel",
      value: "copied",
    }));
    const sealedEffect = line.diagnostics().lastEffect;

    assert.equal(sealedPayload.label, "sealed");
    assert.equal(line.value().tasks[0].metadata.payload.label, "copied");
    assert.notEqual(line.value().tasks[0].metadata, sealedMetadata);
    assert.notEqual(line.value().tasks[0].metadata.payload, sealedPayload);
    assert.equal(sealedEffect.patch.jsonPath.policy.extensibility, "immutableCopy");

    const nonExtensiblePayload = Object.preventExtensions({
      label: "closed",
      keep: true,
    });
    const nonExtensibleMetadata = Object.preventExtensions({
      payload: nonExtensiblePayload,
    });
    const nextTasks = createJsonMetadataApi(signals, nonExtensibleMetadata);
    const nextLine = nextTasks.line({});

    nextLine.patch(nextTasks.patch.itemAspect({
      itemId: "t1",
      aspect: "payloadLabel",
      value: "reconstructed",
    }));

    assert.equal(nonExtensiblePayload.label, "closed");
    assert.equal(nextLine.value().tasks[0].metadata.payload.label, "reconstructed");
    assert.notEqual(nextLine.value().tasks[0].metadata, nonExtensibleMetadata);
    assert.notEqual(nextLine.value().tasks[0].metadata.payload, nonExtensiblePayload);
  } finally {
    await runtime.cleanup();
  }
});

test("JSON path aspect writes deny unrelated container accessors without invoking them", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals } = runtime;
    let objectGetterCalls = 0;
    const response = createMetadataPayloadResponse(signals);
    const payload = { label: "loaded" };
    Object.defineProperty(payload, "other", {
      enumerable: true,
      get() {
        objectGetterCalls += 1;
        return "side-effect";
      },
    });

    assert.throws(
      () =>
        response.aspects.definitions.payloadLabel.write(
          { id: "t1", metadata: { payload } },
          "next",
        ),
      /rejects accessor JSON path segment "other"/,
    );
    assert.equal(objectGetterCalls, 0);

    let arrayGetterCalls = 0;
    const tag = { label: "loaded" };
    const tags = [tag, { label: "kept" }];
    Object.defineProperty(tags, "1", {
      enumerable: true,
      configurable: true,
      get() {
        arrayGetterCalls += 1;
        return { label: "side-effect" };
      },
    });

    assert.throws(
      () =>
        response.aspects.definitions.firstTagLabel.write(
          { id: "t1", metadata: { tags } },
          "next",
        ),
      /rejects accessor JSON path segment "1"/,
    );
    assert.equal(arrayGetterCalls, 0);
  } finally {
    await runtime.cleanup();
  }
});

class CustomJsonLikeValue {
  constructor(label) {
    this.label = label;
  }
}

function createJsonMetadataApi(signals, metadata = { payload: { label: "loaded" } }) {
  const response = createMetadataPayloadResponse(signals);
  return signals.api({
    effects: signals.resource.effects.branchNative(),
  }).url(`/json-write-class-${createJsonMetadataApi.nextId += 1}`)
    .response(response)
    .list({
      load: () => ({
        tasks: [{
          id: "t1",
          metadata,
        }],
      }),
    });
}

createJsonMetadataApi.nextId = 0;

function assertJsonPlainOrNullObject(value) {
  const prototype = Object.getPrototypeOf(value);
  assert.equal(
    prototype === null || Object.getPrototypeOf(prototype) === null,
    true,
  );
}

function createMetadataPayloadResponse(signals) {
  return signals.resource.response.objectItems()({
    field: "tasks",
    itemId: (task) => task.id,
    aspects: signals.resource.response.jsonPathAspects()({
      payload: { field: "metadata", path: ["payload"] },
      payloadLabel: { field: "metadata", path: ["payload", "label"] },
      firstTagLabel: { field: "metadata", path: ["tags", 0, "label"] },
    }),
  });
}
