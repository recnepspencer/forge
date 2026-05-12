import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

import { createRealRequestRuntime } from "../../runtime_fixture/real_request_runtime.mjs";
import { createBranchHead } from "../../runtime_fixture/real_resource_signals.mjs";

const docPath = path.resolve(
  "crates/forge-signal-wasm/docs/resources/json-effects.md",
);

test("JSON path effects doc covers path proof optional policy and rollback posture", async () => {
  const doc = fs.readFileSync(docPath, "utf8");

  assert.match(doc, /jsonPathAspects/);
  assert.match(doc, /jsonItemAspect/);
  assert.match(doc, /presence: "optional"/);
  assert.match(doc, /accessors are denied/i);
  assert.match(doc, /compact inverse rollback/i);

  const runtime = await createRealRequestRuntime();
  try {
    const { signals } = runtime;
    createBranchHead(signals, "json-effects-doc");
    const tasks = signals.api({
      effects: signals.resource.effects.branchNative(),
    }).url("/doc-json-effects")
      .response(signals.resource.response.objectItems()({
        field: "tasks",
        itemId: (task) => task.id,
        aspects: signals.resource.response.jsonPathAspects()({
          priority: { field: "metadata", path: ["priority"] },
        }),
      }))
      .list({
        load: () => ({
          tasks: [{ id: "task:1", metadata: { priority: 1 } }],
        }),
      });

    const line = tasks.line({});
    line.patch(tasks.patch.itemAspect({
      itemId: "task:1",
      aspect: "priority",
      value: 2,
    }));

    const effect = line.diagnostics().lastEffect;

    assert.equal(line.value().tasks[0].metadata.priority, 2);
    assert.equal(effect.locus.kind, "jsonItemAspect");
    assert.equal(effect.patch.jsonPath.aspect, "priority");
    assert.deepEqual(effect.patch.jsonPath.path, ["priority"]);
    assert.equal(effect.optimistic.rollback.kind, "exactBranchRestoreAvailable");
  } finally {
    await runtime.cleanup();
  }
});
