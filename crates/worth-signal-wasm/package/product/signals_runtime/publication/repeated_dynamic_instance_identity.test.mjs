import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createGraphPublicationRuntime } from "../runtime_fixture/graph_publication_runtime.mjs";

test("The Repeated And Dynamic Instance Identity Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const rawSignals = createGraphPublicationRuntime();
    const signals = wrapSignals(rawSignals);
    const rows = signals.scope("rows");
    const row0 = rows.scope("row-0");
    const row1 = rows.scope("row-1");

    const row0Descriptor = row0.descriptor();
    const row1Descriptor = row1.descriptor();
    const row0Identity = row0.signalIdentity("count");
    const row1Identity = row1.signalIdentity("count");
    const row0Count = row0.input(0, { id: "count" });
    const row1Count = row1.input(1, { id: "count" });

    assert.deepEqual(row0Descriptor.path, [
      { id: "rows", localScopeId: "rows", depth: 1 },
      { id: "rows.row-0", localScopeId: "row-0", depth: 2 },
    ]);
    assert.deepEqual(row1Descriptor.path, [
      { id: "rows", localScopeId: "rows", depth: 1 },
      { id: "rows.row-1", localScopeId: "row-1", depth: 2 },
    ]);
    assert.deepEqual(row0Descriptor.identity, {
      scopeId: "rows.row-0",
      parentScopeId: "rows",
      path: row0Descriptor.path,
      depth: 2,
    });
    assert.equal(row0Identity.localId, "count");
    assert.equal(row0Identity.canonicalId, "rows.row-0.count");
    assert.equal(row0Identity.graphId, null);
    assert.equal(row0Identity.rootScopeId, "rows");
    assert.equal(row1Identity.canonicalId, "rows.row-1.count");
    assert.notDeepEqual(row0Identity.scopePath, row1Identity.scopePath);
    assert.equal(row0Count.id, row0Identity.canonicalId);
    assert.equal(row1Count.id, row1Identity.canonicalId);
    assert.deepEqual(row0Count.signalIdentity(), row0Identity);
    assert.deepEqual(row1Count.signalIdentity(), row1Identity);
  } finally {
    await cleanup();
  }
});


