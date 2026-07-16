import assert from "node:assert/strict";
import test from "node:test";

import { createBranchHead } from "../runtime_fixture/real_resource_signals.mjs";
import { createRealRequestRuntime } from "../runtime_fixture/real_request_runtime.mjs";

test("resource effect merge planning and execution carry host-declared region evidence", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals } = runtime;
    createBranchHead(signals, "feature/effect-merge-host-region");
    const response = createTaskGroupedResponse(signals);
    const tasks = signals.api({
      effects: signals.resource.effects.branchNative(),
    }).url("/branch-host-region-tasks")
      .response(response)
      .list({
        load: () => ({
          groups: {
            todo: [{ id: "task:1", group: "todo", title: "First" }],
            done: [],
          },
        }),
      });
    const line = tasks.line({});

    await line.patch(tasks.patch.item({
      itemId: "task:1",
      nextItem: { id: "task:1", group: "todo", title: "Region Bound" },
    }));
    const effect = line.diagnostics().lastEffect;
    const merge = {
      source_branch_id: effect.optimistic.branchId,
      target_branch_id: 0,
    };
    const plan = signals.resource.branch.planEffectMerge({ merge, effect });

    assert.equal(plan.kind, "planned");
    assert.deepEqual(plan.resourceEffect.policyBinding.hostRegion, {
      source: "responseLocusProofCost",
      topology: "groupedCollection",
      lookup: "group-key-item-id",
      traversal: "single-group",
      reconstruction: "replaceGroupItem",
    });
    assert.equal(
      plan.resourceEffect.policyBinding.resourceGranularity,
      "hostRegion",
    );
    assert.equal(
      plan.resourceEffect.policyBinding.nativeMapping,
      "hostRegionMappedToNativeNode",
    );
    assert.equal(
      plan.resourceEffect.rebaseArtifact.proof.aspectPolicyDigest,
      "resource-aspect-policy|groupedCollection|none|host-region|groupedCollection|group-key-item-id|single-group|replaceGroupItem|hostRegion|nativeNode|hostRegionMappedToNativeNode|signal.conflict.resolve-source-when-structure-matches|signal.conflict-isolation.per-node",
    );
    assert.match(
      plan.resourceEffect.rebaseArtifact.proof.policyBindingDigest,
      /host-region\|groupedCollection\|group-key-item-id\|single-group\|replaceGroupItem/,
    );

    const result = signals.resource.branch.mergeEffect({ merge, effect });

    assert.equal(result.kind, "merged");
    assert.deepEqual(
      result.resourceEffect.policyBinding.hostRegion,
      plan.resourceEffect.policyBinding.hostRegion,
    );
    assert.equal(
      result.resourceEffect.mergeArtifact.proof.aspectPolicyDigest,
      plan.resourceEffect.rebaseArtifact.proof.aspectPolicyDigest,
    );
    assert.equal(
      result.resourceEffect.mergeArtifact.proof.nativeMergeResultDigest,
      result.proof.resultDigest,
    );
  } finally {
    await runtime.cleanup();
  }
});

function createTaskGroupedResponse(signals) {
  return signals.resource.response.grouped()({
    itemId: (task) => task.id,
    groupId: (task) => task.group,
    groupForItem: () => "todo",
    groups: (value) => value.groups,
    replaceGroups: (value, groups) => ({ ...value, groups }),
    replaceGroupItem: (value, groupId, itemId, nextItem) => ({
      ...value,
      groups: replaceGroupedItem(value.groups, groupId, itemId, nextItem),
    }),
    aspects: signals.resource.response.objectAspects()({
      title: "title",
    }),
  });
}

function replaceGroupedItem(groups, groupId, itemId, nextItem) {
  return Object.fromEntries(
    Object.entries(groups).map(([key, items]) => [
      key,
      key === groupId
        ? items.map((item) => item.id === itemId ? nextItem : item)
        : items,
    ]),
  );
}
