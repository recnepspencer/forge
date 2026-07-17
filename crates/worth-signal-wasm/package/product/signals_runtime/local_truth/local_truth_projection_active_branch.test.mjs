import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";

const initialGear = Object.freeze({ teeth: 16, thickness: 0.5 });

async function runActiveBranchScenario(deployment) {
  const loaded = await loadSignalsModule({ rawSurface: "real" });
  const { declareLocalTruthSchema } = await loaded.importProductModule(
    "local_truth/schema/schema_declaration.js",
  );
  const signals = await loaded.createSignals({ deployment });
  const schema = declareLocalTruthSchema({
    id: "active-branch.gear",
    aspects: [
      { id: "teeth", field: "teeth", valueType: "number", equivalence: { kind: "exact" }, costClass: "constant" },
      { id: "thickness", field: "thickness", valueType: "number", equivalence: { kind: "exact" }, costClass: "constant" },
    ],
  });
  const gear = signals.input(initialGear, {
    debugName: "activeBranchGear",
    producesAspects: [0, 1],
  });
  const truth = signals.localTruth({
    authorityId: "active-branch-projection",
    schema,
    initialEntities: { gear: initialGear },
    bindings: [{ entityId: "gear", input: gear, aspectMap: { teeth: 0, thickness: 1 } }],
  });
  try {
    const main = (await truth.branch()).value;
    const design = (await truth.forkBranch({
      parentBranchId: main.id,
      expectedParentBasis: main.basis,
      name: "design",
    })).value;
    const forkBinding = design.derivation?.binding;
    assert.ok(forkBinding, "fork derivation must carry a Signal projection binding");

    // Make the projected native branch the ambient branch, the way a consumer
    // does to render the projection, then commit on the projected truth branch.
    await signals.history().switch_branch(forkBinding.signalBranchId);

    const designBranch = (await truth.branch(design.id)).value;
    const committed = await truth.commit({
      requestId: "active-branch-commit",
      branchId: design.id,
      expectedBasis: designBranch.basis,
      operations: [{ entityId: "gear", aspectId: "teeth", value: 24 }],
    });
    assert.equal(committed.posture, "success");

    const receipt = await awaitCurrentDerivation(truth, design.id);
    assert.equal(receipt.posture, "Current", `derivation failed: ${receipt.reason ?? "unknown"}`);
    assert.ok(receipt.binding);
    assert.notEqual(
      receipt.binding.signalBasisDigest,
      forkBinding.signalBasisDigest,
      "the projected basis digest must advance with the commit",
    );

    // A second commit proves the driver keeps a coherent basis on the ambient path.
    const advancedBranch = (await truth.branch(design.id)).value;
    const second = await truth.commit({
      requestId: "active-branch-commit-2",
      branchId: design.id,
      expectedBasis: advancedBranch.basis,
      operations: [{ entityId: "gear", aspectId: "thickness", value: 0.75 }],
    });
    assert.equal(second.posture, "success");
    const secondReceipt = await awaitCurrentDerivation(truth, design.id);
    assert.equal(secondReceipt.posture, "Current", `derivation failed: ${secondReceipt.reason ?? "unknown"}`);
    assert.notEqual(secondReceipt.binding.signalBasisDigest, receipt.binding.signalBasisDigest);
    return {
      firstDigest: receipt.binding.signalBasisDigest,
      secondDigest: secondReceipt.binding.signalBasisDigest,
    };
  } finally {
    await truth.terminate?.();
    await signals.terminate();
    await loaded.cleanup();
  }
}

async function awaitCurrentDerivation(truth, branchId) {
  let receipt = null;
  for (let attempt = 0; attempt < 40; attempt += 1) {
    receipt = await truth.derivation(branchId);
    if (receipt?.posture === "Current" || receipt?.posture === "RebuildRequired") return receipt;
    await new Promise((resolve) => setTimeout(resolve, 50));
  }
  return receipt;
}

test("worker-first projections stay current after switching onto the projected branch", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  try {
    await runActiveBranchScenario("workerFirst");
  } finally {
    globalThis.Worker = previousWorker;
  }
});

test("compatibility projections stay current after switching onto the projected branch", async () => {
  await runActiveBranchScenario("mainThreadCompatibility");
});

test("projection failure receipts preserve structured denial reasons", async () => {
  const loaded = await loadSignalsModule();
  const { declareLocalTruthSchema } = await loaded.importProductModule(
    "local_truth/schema/schema_declaration.js",
  );
  const { createLocalTruthSignalProjection } = await loaded.importProductModule(
    "local_truth/projection/signal_projection.js",
  );
  const schema = declareLocalTruthSchema({
    id: "denial-reason.gear",
    aspects: [
      { id: "teeth", field: "teeth", valueType: "number", equivalence: { kind: "exact" }, costClass: "constant" },
    ],
  });
  const structuredDenial = {
    code: "invalidInput",
    message: "plan targeted worker transaction denied: Denied(ActiveBranchTarget { branch_id: SignalBranchId(2) })",
    branchId: 2n,
  };
  const projection = createLocalTruthSignalProjection({
    schema,
    bindings: [{ entityId: "gear", signalId: "gear-signal", aspectMap: { teeth: 0 } }],
    driver: {
      async initialize() {
        throw structuredDenial;
      },
    },
  });
  const receipt = await projection.initialize(
    { id: "branch:main" },
    { values: { gear: { teeth: 16 } } },
  );
  assert.equal(receipt.posture, "RebuildRequired");
  assert.equal(
    receipt.reason,
    `${structuredDenial.code}: ${structuredDenial.message}`,
    "structured denials must not collapse to [object Object]",
  );

  const bigintOnlyDenial = { denialKind: "stub", branchId: 7n };
  const bigintProjection = createLocalTruthSignalProjection({
    schema,
    bindings: [{ entityId: "gear", signalId: "gear-signal", aspectMap: { teeth: 0 } }],
    driver: {
      async initialize() {
        throw bigintOnlyDenial;
      },
    },
  });
  const bigintReceipt = await bigintProjection.initialize(
    { id: "branch:main" },
    { values: { gear: { teeth: 16 } } },
  );
  assert.equal(bigintReceipt.posture, "RebuildRequired");
  assert.equal(bigintReceipt.reason, '{"denialKind":"stub","branchId":"7"}');
  await loaded.cleanup();
});
