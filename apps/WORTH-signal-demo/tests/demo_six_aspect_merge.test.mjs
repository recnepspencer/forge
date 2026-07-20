import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { createGearScenario } from "../src/local-truth-gear/gear_scenario.ts";

test("Demo 6 composes disjoint Main and Design aspect commits", async () => {
  await withGearScenario(async (scenario) => {
    await scenario.forkDesignBranch();
    await scenario.commitBranchPatch("main", { thickness: 0.46 });
    await scenario.commitBranchPatch("design", { teeth: 24, innerRadius: 0.72 });

    const merged = await scenario.mergeBranches();

    assert.equal(merged.phase, "merged");
    assert.deepEqual(designValues(merged.main), {
      thickness: 0.46,
      teeth: 24,
      innerRadius: 0.72,
    });
    assert.equal(merged.history[0].kind, "merge");
    assert.equal(merged.history.some(({ title }) => title === "thickness → 0.46"), true);
    assert.equal(merged.history.some(({ title }) => (
      title.includes("teeth → 24") && title.includes("innerRadius → 0.72")
    )), true);
  });
});

test("Demo 6 exposes and resolves only a divergent overlapping aspect", async () => {
  await withGearScenario(async (scenario) => {
    await scenario.forkDesignBranch();
    await scenario.commitBranchPatch("main", { thickness: 0.44, teeth: 20 });
    await scenario.commitBranchPatch("design", { thickness: 0.86, innerRadius: 0.78 });

    const review = await scenario.mergeBranches();

    assert.equal(review.phase, "review");
    assert.equal(review.conflicts.length, 1);
    assert.equal(review.conflicts[0].aspectId, "thickness");
    assert.equal(review.conflicts[0].mainValue, 0.44);
    assert.equal(review.conflicts[0].designValue, 0.86);

    const merged = await scenario.resolveMerge([{
      conflictId: review.conflicts[0].id,
      choice: "design",
    }]);

    assert.equal(merged.phase, "merged");
    assert.deepEqual(designValues(merged.main), {
      thickness: 0.86,
      teeth: 20,
      innerRadius: 0.78,
    });
  });
});

test("Demo 6 preserves successive caller-supplied slider values without substitution", async () => {
  await withGearScenario(async (scenario) => {
    await scenario.forkDesignBranch();

    for (const thickness of [0.22, 0.48, 0.94, 1.18]) {
      const view = await scenario.commitBranchPatch("main", { thickness });
      assert.equal(view.main.thickness, thickness);
      assert.equal(view.design.thickness, 0.58);
    }

    for (const teeth of [11, 17, 29, 35]) {
      const view = await scenario.commitBranchPatch("design", { teeth });
      assert.equal(view.design.teeth, teeth);
      assert.equal(view.main.teeth, 18);
    }
  });
});

test("Demo 6 inspects any retained commit without moving its live head", async () => {
  await withGearScenario(async (scenario) => {
    await scenario.forkDesignBranch();
    await scenario.commitBranchPatch("main", { thickness: 0.42 });
    const live = await scenario.commitBranchPatch("main", { thickness: 0.88 });
    const earlier = live.history.find(({ title }) => title === "thickness → 0.42");
    const liveHead = live.history.find(({ title }) => title === "thickness → 0.88");
    assert.ok(earlier);
    assert.ok(liveHead?.isLiveHead);

    const inspected = await scenario.selectHistoryCommit(earlier.branchId, earlier.id);
    assert.equal(inspected.main.thickness, 0.42);
    assert.equal(inspected.historySelection.commitId, earlier.id);
    assert.equal(inspected.history.find(({ id }) => id === liveHead.id).isLiveHead, true);
    await assert.rejects(
      () => scenario.commitBranchPatch("main", { thickness: 0.5 }),
      /Select a live history head/u,
    );

    const resumed = await scenario.selectHistoryCommit(liveHead.branchId, liveHead.id);
    assert.equal(resumed.historySelection, null);
    assert.equal(resumed.main.thickness, 0.88);
  });
});

test("Demo 6 retains source-branch inspection after its merge commits", async () => {
  await withGearScenario(async (scenario) => {
    await scenario.forkDesignBranch();
    await scenario.commitBranchPatch("design", { teeth: 26 });
    const beforeMerge = await scenario.commitBranchPatch("design", { teeth: 31 });
    const earlierDesignCommit = beforeMerge.history.find(({ title }) => title === "teeth → 26");
    assert.ok(earlierDesignCommit);

    const merged = await scenario.mergeBranches();
    assert.equal(merged.phase, "merged");
    assert.equal(merged.main.teeth, 31);

    const inspected = await scenario.selectHistoryCommit(
      earlierDesignCommit.branchId,
      earlierDesignCommit.id,
    );
    assert.equal(inspected.design.teeth, 26);
    assert.equal(inspected.main.teeth, 31);
    assert.equal(inspected.historySelection.commitId, earlierDesignCommit.id);
    assert.equal(inspected.history[0].kind, "merge");
    assert.equal(inspected.history[0].isLiveHead, true);
  });
});

async function withGearScenario(run) {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const scenario = await createGearScenario();
  try {
    await run(scenario);
  } finally {
    await scenario.terminate();
    globalThis.Worker = previousWorker;
  }
}

function designValues(gear) {
  return {
    thickness: gear.thickness,
    teeth: gear.teeth,
    innerRadius: gear.innerRadius,
  };
}
