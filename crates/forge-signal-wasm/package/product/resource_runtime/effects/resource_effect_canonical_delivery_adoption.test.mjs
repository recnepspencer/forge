import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadSignalsModule } from "../../signals_runtime/module_loading/load_signals_module.mjs";

const EXISTING = Object.freeze({ id: "line-071", label: "Existing", qty: "1", sync: "synced" });
const DRAFT = Object.freeze({ id: "line-073", label: "Tubing", qty: "12", sync: "syncing" });

test("quiescent effect DAG adopts delivered canonical truth as its admission era", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const loaded = await loadSignalsModule({ rawSurface: "real" });
  let signals = null;
  try {
    signals = await loaded.createSignals();
    let serverLines = [EXISTING];
    const family = signals.api({ effects: signals.resource.effects.branchNative() })
      .url("/orders/:orderId/lines")
      .response(signals.resource.response.array({ itemId: (line) => line.id }))
      .list({ load: () => serverLines.map((line) => ({ ...line })) });
    const line = family.line({ orderId: "PO-1142" });
    await line.awaitSettlement();

    const admission = await line.patch(family.patch.insert({
      itemId: DRAFT.id,
      placement: "append",
      nextItem: { ...DRAFT },
    }));
    assert.equal(typeof admission.effectId, "string");
    serverLines = [EXISTING, { ...DRAFT, sync: "synced" }];
    const merged = await line.effects().confirm(admission.effectId, {
      responseId: "adoption:first:confirmed",
      serverPatch: family.patch.insert({
        itemId: DRAFT.id,
        placement: "append",
        nextItem: { ...DRAFT, sync: "synced" },
      }),
    });
    assert.equal(merged.kind, "merged");
    assert.equal(line.effects().open().length, 0);

    // A fresh delivery removes the merged item: server truth moved on without
    // an effect settlement. The DAG must adopt this canonical era instead of
    // judging future admissions against the retired merge fold.
    serverLines = [EXISTING];
    line.invalidate();
    line.refresh();
    const refreshSettlement = await line.awaitSettlement({ timeoutMs: 5_000 });
    assert.equal(refreshSettlement.resultKind, "fulfilled");
    assert.deepEqual(line.value().map((entry) => entry.id), [EXISTING.id]);

    const readmission = await line.patch(family.patch.insert({
      itemId: DRAFT.id,
      placement: "append",
      nextItem: { ...DRAFT },
    }));
    assert.equal(
      typeof readmission.effectId,
      "string",
      "re-inserting an itemId the server no longer holds must be admitted",
    );
    const settled = await line.effects().reject(readmission.effectId, {
      responseId: "adoption:second:rejected",
    });
    assert.equal(settled.kind, "rejectedAndRetired");
    assert.deepEqual(line.value().map((entry) => entry.id), [EXISTING.id]);
    assert.equal(line.effects().open().length, 0);
    assert.equal(line.effects().counters().openEffectCount, 0);
    line.free();
  } finally {
    if (signals) await signals.terminate();
    await loaded.cleanup();
    globalThis.Worker = previousWorker;
  }
});
