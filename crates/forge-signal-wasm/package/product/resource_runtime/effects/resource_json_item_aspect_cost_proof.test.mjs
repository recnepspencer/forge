import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../runtime_fixture/real_request_runtime.mjs";
import { createBranchHead } from "../runtime_fixture/real_resource_signals.mjs";

test("JSON path item aspect effects expose path proof and traversal cost counters", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals, signalsMod } = runtime;
    createBranchHead(signals, "json-path-cost-proof");
    const tasks = createJsonPathTaskApi(signals);
    const line = tasks.line({});

    line.patch(tasks.patch.itemAspect({
      itemId: "t1",
      aspect: "firstTagLabel",
      value: "Local",
    }));
    const localEffect = line.diagnostics().lastEffect;

    assertJsonPathProof(localEffect, {
      aspect: "firstTagLabel",
      digest: "json-path/metadata/tags/#0/label",
      traversalBreadth: 4,
      reconstructionBreadth: 4,
    });
    assert.equal(localEffect.counters.jsonPathTraversalBreadth, 4);
    assert.equal(localEffect.counters.jsonPathReconstructionBreadth, 4);
    assert.equal(localEffect.locus.kind, "jsonItemAspect");
    assert.equal(localEffect.patch.jsonPath.policy.containerWrite, "immutableCopy");
    assert.deepEqual(line.value().tasks[0].metadata.tags[0], {
      label: "Local",
    });

    line.deliver(signalsMod.resourceDelivery.patch({
      packetId: "pkt-first-tag-label",
      basisId: null,
      patch: tasks.patch.itemAspect({
        itemId: "t1",
        aspect: "firstTagLabel",
        value: "Delivered",
      }),
    }));
    const deliveryEffect = line.diagnostics().lastEffect;

    assertJsonPathProof(deliveryEffect, {
      aspect: "firstTagLabel",
      digest: "json-path/metadata/tags/#0/label",
      traversalBreadth: 4,
      reconstructionBreadth: 4,
    });
    assert.equal(deliveryEffect.plan.admissionKind, "delivery");
    assert.deepEqual(
      line.history().verificationPackage().lifecycle.lastEffect.patch.jsonPath,
      deliveryEffect.patch.jsonPath,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("JSON path proof names immutable-copy writes over frozen JSON containers", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals } = runtime;
    createBranchHead(signals, "json-path-frozen-copy-proof");
    const frozenTag = Object.freeze({ label: "Frozen" });
    const frozenTags = Object.freeze([frozenTag]);
    const frozenMetadata = Object.freeze({ tags: frozenTags });
    const tasks = createJsonPathTaskApi(signals, frozenMetadata);
    const line = tasks.line({});

    line.patch(tasks.patch.itemAspect({
      itemId: "t1",
      aspect: "firstTagLabel",
      value: "Copied",
    }));
    const effect = line.diagnostics().lastEffect;

    assert.equal(effect.patch.jsonPath.policy.containerWrite, "immutableCopy");
    assert.equal(frozenTag.label, "Frozen");
    assert.equal(line.value().tasks[0].metadata.tags[0].label, "Copied");
    assert.notEqual(line.value().tasks[0].metadata, frozenMetadata);
    assert.notEqual(line.value().tasks[0].metadata.tags, frozenTags);
  } finally {
    await runtime.cleanup();
  }
});

test("resourceItemAspects rejects forged JSON path proof metadata", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signalsMod } = runtime;

    assert.throws(
      () =>
        signalsMod.resourceItemAspects({
          label: {
            read: (item) => item.label,
            write: (item, label) => ({ ...item, label }),
            locus: "jsonItemAspect",
            jsonPathProof: {
              version: "resource-json-path-aspect-proof-v1",
              aspect: "label",
            },
          },
        }),
      /invalid JSON path proof/,
    );
  } finally {
    await runtime.cleanup();
  }
});

function createJsonPathTaskApi(signals, metadata = { tags: [{ label: "Loaded" }] }) {
  const response = signals.resource.response.objectItems()({
    field: "tasks",
    itemId: (task) => task.id,
    aspects: signals.resource.response.jsonPathAspects()({
      firstTagLabel: { field: "metadata", path: ["tags", 0, "label"] },
    }),
  });
  return signals.api({
    effects: signals.resource.effects.branchNative(),
  }).url("/json-path-cost-tasks")
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

function assertJsonPathProof(effect, expected) {
  assert.notEqual(effect.patch.jsonPath, null);
  assert.notEqual(effect.patch.jsonPath.policy, null);
  assert.notEqual(effect.patch.jsonPath.cost, null);
  assert.equal(effect.patch.jsonPath.version, "resource-json-path-aspect-proof-v1");
  assert.equal(effect.patch.jsonPath.aspect, expected.aspect);
  assert.equal(effect.patch.jsonPath.field, "metadata");
  assert.deepEqual(effect.patch.jsonPath.path, ["tags", 0, "label"]);
  assert.equal(effect.patch.jsonPath.parsedPathDigest, expected.digest);
  assert.equal(effect.patch.jsonPath.policy.presence, "required");
  assert.equal(effect.patch.jsonPath.policy.absence, "deny");
  assert.equal(effect.patch.jsonPath.policy.arrayIndex, "explicitExistingIndex");
  assert.equal(effect.patch.jsonPath.policy.accessor, "denyWithoutInvocation");
  assert.equal(effect.patch.jsonPath.cost.traversalBreadth, expected.traversalBreadth);
  assert.equal(
    effect.patch.jsonPath.cost.reconstructionBreadth,
    expected.reconstructionBreadth,
  );
  assert.equal(effect.patch.jsonPath.cost.cloneBreadth, expected.reconstructionBreadth);
  assert.match(effect.patch.jsonPath.proofDigest, /immutable-copy/);
}
