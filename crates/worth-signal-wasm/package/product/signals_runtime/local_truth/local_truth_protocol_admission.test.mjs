import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";

test("worker protocol replays identical commands and rejects reorder, mutation, truncation, and foreign registration", async () => {
  const loaded = await loadSignalsModule();
  const { declareLocalTruthSchema } = await loaded.importProductModule(
    "local_truth/schema/schema_declaration.js",
  );
  const { createWorkerLocalTruthRuntime } = await loaded.importProductModule(
    "local_truth/protocol/worker_local_truth_runtime.js",
  );
  const schema = declareLocalTruthSchema({
    id: "protocol.gear",
    aspects: [{
      id: "teeth",
      field: "teeth",
      valueType: "number",
      equivalence: { kind: "exact" },
      costClass: "constant",
    }],
  });
  const worker = createWorkerLocalTruthRuntime(fakeSignalRuntime());
  const create = {
    authorityId: "protocol-authority",
    registrationId: "registration:1",
    sequence: 0,
    operation: "create",
    request: {
      authorityId: "protocol-authority",
      schema,
      initialEntities: { gear: { teeth: 16 } },
      bindings: [{ entityId: "gear", signalId: "gear-input", aspectMap: { teeth: 0 } }],
    },
  };
  const created = await worker.command(create);
  assert.deepEqual(await worker.command(create), created);

  const inspect = {
    authorityId: create.authorityId,
    registrationId: create.registrationId,
    sequence: 1,
    operation: "inspect",
    request: null,
  };
  const inspected = await worker.command(inspect);
  assert.deepEqual(await worker.command(inspect), inspected);
  await assert.rejects(
    worker.command({ ...inspect, operation: "branch" }),
    /replayed with different content/,
  );
  await assert.rejects(
    worker.command({ ...inspect, sequence: 3 }),
    /out of order/,
  );
  await assert.rejects(
    worker.command({ ...inspect, sequence: undefined }),
    /envelope is invalid/,
  );
  await assert.rejects(
    worker.command({ ...inspect, sequence: 2, registrationId: "registration:foreign" }),
    /unavailable or foreign/,
  );
  assert.equal((await worker.command({ ...inspect, sequence: 2, operation: "branch", request: "branch:main" })).posture, "success");
});

function fakeSignalRuntime() {
  let nextBranchId = 1;
  const basis = (branchId) => ({
    branchId,
    authoredStateDigest: `basis:${branchId}`,
  });
  return {
    async currentBranch() {
      return { id: 0 };
    },
    async workerBranchBasis(branchId) {
      return basis(branchId);
    },
    async forkBranch() {
      const id = nextBranchId;
      nextBranchId += 1;
      return { branch: { id }, createdBasis: basis(id) };
    },
    async applyTransactionToBranch(request) {
      return { afterBasis: basis(request.branchId) };
    },
    async retireBranch() {},
  };
}
