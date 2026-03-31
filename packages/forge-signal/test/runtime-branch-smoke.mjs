import { createSignalRuntime, define } from "../src/index.ts";

function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}

function assertNear(actual, expected, message) {
  if (Math.abs(actual - expected) > 1e-9) {
    throw new Error(`${message} (expected ${expected}, got ${actual})`);
  }
}

const runtime = await createSignalRuntime();
const history = runtime.history();

runtime.defineSource(define.source("gearTeeth").initial(16));
runtime.defineSource(define.source("gearThickness").initial(0.42));

const main = history.currentBranch();
const feature = history.createBranch("what-if");

assert(typeof history.branchSnapshotId === "function", "missing branchSnapshotId export");
assert(
  typeof history.restoreBranchSnapshotById === "function",
  "missing restoreBranchSnapshotById export",
);

runtime.transaction([{ kind: "set", id: "gearTeeth", value: 8 }]);
history.switchBranch(feature.id);
runtime.transaction([{ kind: "set", id: "gearTeeth", value: 32 }]);
history.switchBranch(main.id);
runtime.transaction([{ kind: "set", id: "gearThickness", value: 0.1 }]);

assert(runtime.read("gearTeeth") === 8, "main branch teeth leaked from feature branch");
assertNear(runtime.read("gearThickness"), 0.1, "main branch thickness edit did not stick");

history.switchBranch(feature.id);

assert(runtime.read("gearTeeth") === 32, "feature branch teeth were not preserved");
assertNear(runtime.read("gearThickness"), 0.42, "feature branch thickness was contaminated");

history.switchBranch(main.id);
const mainSnapshotId = history.branchSnapshotId(main.id);
history.switchBranch(feature.id);
const featureSnapshotId = history.branchSnapshotId(feature.id);
history.restoreBranchSnapshotById(main.id, mainSnapshotId);
history.switchBranch(main.id);
assert(runtime.read("gearTeeth") === 8, "main restore-by-id did not preserve main teeth");
assertNear(runtime.read("gearThickness"), 0.1, "main restore-by-id did not preserve main thickness");
history.restoreBranchSnapshotById(feature.id, featureSnapshotId);
history.switchBranch(feature.id);
assert(runtime.read("gearTeeth") === 32, "feature restore-by-id did not preserve branch teeth");
assertNear(runtime.read("gearThickness"), 0.42, "feature restore-by-id did not preserve branch thickness");

console.log("[forge-signal] runtime branch smoke passed");
