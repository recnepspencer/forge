import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

import { createRealRequestRuntime } from "../../runtime_fixture/real_request_runtime.mjs";
import { createBranchHead } from "../../runtime_fixture/real_resource_signals.mjs";

const docPath = path.resolve(
  "crates/forge-signal-wasm/docs/resources/merge-and-rebase.md",
);

test("effect merge doc covers planEffectMerge mergeEffect conflicts and mapping unavailable", async () => {
  const doc = fs.readFileSync(docPath, "utf8");

  assert.match(doc, /planEffectMerge/);
  assert.match(doc, /mergeEffect/);
  assert.match(doc, /mappingUnavailable/);
  assert.match(doc, /policyBinding/);
  assert.match(doc, /forged or tampered/i);

  const runtime = await createRealRequestRuntime();
  try {
    const { signals } = runtime;
    createBranchHead(signals, "effect-merge-doc");
    const tasks = signals.api({
      effects: signals.resource.effects.branchNative(),
    }).url("/doc-effect-merge")
      .items((task) => task.id)
      .aspect("title", (task) => task.title, (task, title) => ({
        ...task,
        title,
      }))
      .list({
        load: () => [{ id: "task:1", title: "First" }],
      });

    const line = tasks.line({});
    line.patch(tasks.patch.itemAspect({
      itemId: "task:1",
      aspect: "title",
      value: "Merged",
    }));

    const effect = line.diagnostics().lastEffect;
    const plan = signals.resource.branch.planEffectMerge({
      merge: {
        source_branch_id: effect.optimistic.branchId,
        target_branch_id: 0,
      },
      effect,
    });

    assert.equal(plan.kind, "planned");
    assert.equal(plan.resourceEffect.effectId, effect.effectId);
    assert.equal(plan.resourceEffect.rebaseArtifact.kind, "rebaseAvailable");
    assert.equal(plan.resourceEffect.policyBinding.source, "resourceEffectLocus");
  } finally {
    await runtime.cleanup();
  }
});
