import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

import { createRealRequestRuntime } from "../../runtime_fixture/real_request_runtime.mjs";
import { createBranchHead } from "../../runtime_fixture/real_resource_signals.mjs";

const docPath = path.resolve(
  "crates/forge-signal-wasm/docs/resources/branch-native-effects.md",
);

test("branch-native resource effects doc happy path covers topology, JSON, merge, and lifecycle examples", async () => {
  const doc = fs.readFileSync(docPath, "utf8");
  assert.match(doc, /branch-native optimistic effects/i);
  assert.match(doc, /response-lens topology declarations/i);
  assert.match(doc, /JSON effects/i);
  assert.match(doc, /UI lifecycle event consumption/i);

  const runtime = await createRealRequestRuntime();
  try {
    const { signals } = runtime;
    createBranchHead(signals, "branch-native-effects-doc");
    const api = signals.api({
      effects: signals.resource.effects.branchNative(),
    });
    const standardTasks = api.url("/doc-tasks")
      .items((task) => task.id)
      .aspect("title", (task) => task.title, (task, title) => ({
        ...task,
        title,
      }))
      .list({
        load: () => [{ id: "task:1", title: "First" }],
      });
    const standardLine = standardTasks.line({});

    standardLine.patch(standardTasks.patch.itemAspect({
      itemId: "task:1",
      aspect: "title",
      value: "Draft",
    }));
    const standardEffect = standardLine.diagnostics().lastEffect;
    const uiLifecycleEvents = standardLine.history().lifecycle.map((entry) => ({
      operation: entry.lastOperation,
      outcome: entry.lastOutcome,
      effect: entry.lastEffect?.effectId ?? null,
    }));

    assert.equal(standardLine.value()[0].title, "Draft");
    assert.equal(standardEffect.optimistic.rollback.kind, "exactBranchRestoreAvailable");
    assert.equal(standardEffect.profile.rebase, "nativeMergePlan");
    assert.equal(uiLifecycleEvents.at(-1).effect, standardEffect.effectId);

    const jsonTasks = api.url("/doc-json-tasks")
      .response(createJsonPathResponse(signals))
      .list({
        load: () => ({
          tasks: [{ id: "task:json", metadata: { priority: 1 } }],
        }),
      });
    const jsonLine = jsonTasks.line({});
    jsonLine.patch(jsonTasks.patch.itemAspect({
      itemId: "task:json",
      aspect: "priority",
      value: 2,
    }));
    assert.equal(jsonLine.value().tasks[0].metadata.priority, 2);
    assert.equal(jsonLine.diagnostics().lastEffect.locus.kind, "jsonItemAspect");

    const groupedTasks = api.url("/doc-grouped-tasks")
      .response(createGroupedResponse(signals))
      .list({
        load: () => ({
          groups: {
            todo: [{ id: "task:grouped", group: "todo", title: "First" }],
          },
        }),
      });
    const groupedLine = groupedTasks.line({});
    groupedLine.patch(groupedTasks.patch.item({
      itemId: "task:grouped",
      nextItem: { id: "task:grouped", group: "todo", title: "Grouped" },
    }));
    const groupedEffect = groupedLine.diagnostics().lastEffect;
    const mergePlan = signals.resource.branch.planMerge({
      source_branch_id: groupedEffect.optimistic.branchId,
      target_branch_id: 0,
    });

    assert.equal(groupedEffect.locus.kind, "groupedCollection");
    assert.equal(groupedEffect.locusProof.topology, "groupedCollection");
    assert.equal(groupedEffect.locusProof.cost.traversal, "single-group");
    assert.equal(mergePlan.kind, "planned");
  } finally {
    await runtime.cleanup();
  }
});

function createJsonPathResponse(signals) {
  return signals.resource.response.objectItems()({
    field: "tasks",
    itemId: (task) => task.id,
    aspects: signals.resource.response.jsonPathAspects()({
      priority: { field: "metadata", path: ["priority"] },
    }),
  });
}

function createGroupedResponse(signals) {
  return signals.resource.response.grouped()({
    itemId: (task) => task.id,
    groupId: (task) => task.group,
    groupForItem: () => "todo",
    groups: (value) => value.groups,
    replaceGroups: (value, groups) => ({ ...value, groups }),
    replaceGroupItem: (value, groupId, itemId, nextItem) => ({
      ...value,
      groups: {
        ...value.groups,
        [groupId]: value.groups[groupId].map((item) =>
          item.id === itemId ? nextItem : item),
      },
    }),
    aspects: signals.resource.response.objectAspects()({
      title: "title",
    }),
  });
}
