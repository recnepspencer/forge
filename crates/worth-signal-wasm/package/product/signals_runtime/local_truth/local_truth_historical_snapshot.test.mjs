import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";

const initialGear = Object.freeze({ thickness: 0.58, teeth: 18 });

test("historical snapshots preserve retained values with deployment parity", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  try {
    const compatibility = await runHistory("mainThreadCompatibility");
    const worker = await runHistory("workerFirst");
    assert.deepEqual(worker, compatibility);
  } finally {
    globalThis.Worker = previousWorker;
  }
});

async function runHistory(deployment) {
  const loaded = await loadSignalsModule({ rawSurface: "real" });
  const { declareLocalTruthSchema } = await loaded.importProductModule(
    "local_truth/schema/schema_declaration.js",
  );
  const signals = await loaded.createSignals({ deployment });
  const schema = declareLocalTruthSchema({
    id: "historical-snapshot.gear",
    aspects: ["thickness", "teeth"].map((field) => ({
      id: field,
      field,
      valueType: "number",
      equivalence: { kind: "exact" },
      costClass: "constant",
    })),
  });
  const gear = signals.input(initialGear, { producesAspects: [0, 1] });
  const truth = signals.localTruth({
    authorityId: "historical-snapshot",
    schema,
    initialEntities: { gear: initialGear },
    bindings: [{ entityId: "gear", input: gear, aspectMap: { thickness: 0, teeth: 1 } }],
  });
  await truth.ready?.();
  try {
    const main = required(await truth.branch());
    const first = await commit(truth, main.id, "main-thickness", "thickness", 0.34);
    const source = required(await truth.forkBranch({
      parentBranchId: main.id,
      expectedParentBasis: required(await truth.branch(main.id)).basis,
      name: "Design",
    }));
    const mainHead = await commit(truth, main.id, "main-teeth", "teeth", 22);
    const sourceHead = await commit(truth, source.id, "source-teeth", "teeth", 31);

    const firstFromMain = required(await truth.historicalSnapshot({
      branchId: main.id,
      commitId: first.commit.id,
    }));
    const firstFromSource = required(await truth.historicalSnapshot({
      branchId: source.id,
      commitId: first.commit.id,
    }));
    assert.deepEqual(firstFromMain.values.gear, { thickness: 0.34, teeth: 18 });
    assert.deepEqual(firstFromSource.values.gear, firstFromMain.values.gear);
    assert.equal(firstFromMain.counters.visitedCommits, 2);

    const sourceFromMain = await truth.historicalSnapshot({
      branchId: main.id,
      commitId: sourceHead.commit.id,
    });
    const mainFromSource = await truth.historicalSnapshot({
      branchId: source.id,
      commitId: mainHead.commit.id,
    });
    assert.equal(sourceFromMain.code, "commitOutsideBranchHistory");
    assert.equal(mainFromSource.code, "commitOutsideBranchHistory");

    required(await truth.checkpoint(main.id));
    required(await truth.checkpoint(source.id));
    const checkpointSnapshot = required(await truth.historicalSnapshot({
      branchId: source.id,
      commitId: sourceHead.commit.id,
    }));
    assert.deepEqual(checkpointSnapshot.values.gear, { thickness: 0.34, teeth: 31 });
    assert.equal(checkpointSnapshot.counters.visitedCommits, 1);

    return {
      first: firstFromMain.values.gear,
      firstDigest: firstFromMain.digest,
      checkpoint: checkpointSnapshot.values.gear,
      checkpointDigest: checkpointSnapshot.digest,
      siblingDenials: [sourceFromMain.code, mainFromSource.code],
    };
  } finally {
    await truth.terminate();
    await signals.terminate();
    await loaded.cleanup();
  }
}

async function commit(truth, branchId, requestId, aspectId, value) {
  const branch = required(await truth.branch(branchId));
  return required(await truth.commit({
    requestId,
    branchId,
    expectedBasis: branch.basis,
    operations: [{ entityId: "gear", aspectId, value }],
  }));
}

function required(outcome) {
  assert.ok(outcome.posture === "success" || outcome.posture === "advisory", outcome.message);
  return outcome.value;
}
