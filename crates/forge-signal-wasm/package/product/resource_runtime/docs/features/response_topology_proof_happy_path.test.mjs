import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

import { createRealRequestRuntime } from "../../runtime_fixture/real_request_runtime.mjs";
import { createBranchHead } from "../../runtime_fixture/real_resource_signals.mjs";

const docPath = path.resolve(
  "crates/forge-signal-wasm/docs/resource-contracts/response-topology-proof.md",
);

test("response topology proof doc covers sealed topology proof and map effects", async () => {
  const doc = fs.readFileSync(docPath, "utf8");

  assert.match(doc, /signals\.resource\.response\.\*\(\.\.\.\)/);
  assert.match(doc, /responseLensProof/);
  assert.match(doc, /entityStore/);
  assert.match(doc, /mapCollection/);
  assert.match(doc, /detailResponse/);

  const runtime = await createRealRequestRuntime();
  try {
    const { signals } = runtime;
    createBranchHead(signals, "response-topology-doc");
    const tasks = signals.api({
      effects: signals.resource.effects.branchNative(),
    }).url("/doc-map-tasks")
      .response(signals.resource.response.map()({
        itemId: (task) => task.id,
        entries: (value) => new Map(value.taskMapEntries),
        replaceEntries: (value, tasks) => ({
          ...value,
          taskMapEntries: [...tasks],
        }),
        replaceEntry: (value, itemId, nextItem) => {
          const tasks = new Map(value.taskMapEntries);
          tasks.set(itemId, nextItem);
          return { ...value, taskMapEntries: [...tasks] };
        },
      }))
      .list({
        load: () => ({
          taskMapEntries: [
            ["task:1", { id: "task:1", title: "First" }],
          ],
        }),
      });

    const line = tasks.line({});
    line.patch(tasks.patch.item({
      itemId: "task:1",
      nextItem: { id: "task:1", title: "Mapped" },
    }));

    const effect = line.diagnostics().lastEffect;

    assert.equal(effect.locus.kind, "mapCollection");
    assert.equal(effect.locusProof.topology, "mapCollection");
    assert.equal(effect.locusProof.cost.lookup, "map-key");
  } finally {
    await runtime.cleanup();
  }
});
