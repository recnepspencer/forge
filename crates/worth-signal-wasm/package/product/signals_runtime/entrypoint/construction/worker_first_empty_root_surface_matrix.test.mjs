/**
 * Publish gate: empty worker-first roots must admit ordinary app authoring.
 * Import/portable-only surfaces must still deny with an honest import error.
 */
import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadStoreModule } from "../../../host_capabilities_certification/module_loading/load_store_module.mjs";
import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

const IMPORT_REQUIRED = /active imported graph|imported graph/u;

async function withEmptyWorkerFirst(run) {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup: cleanupSignals, resourceParams, resourceParamIdentity } =
    await loadSignalsModule({ rawSurface: "real" });
  const { createReactSignalsStore, cleanup: cleanupStore } = await loadStoreModule();
  let signals = null;
  try {
    signals = await createSignals({ deployment: "workerFirst" });
    await run({
      signals,
      createReactSignalsStore,
      resourceParams,
      resourceParamIdentity,
    });
  } finally {
    if (signals) {
      await signals.terminate();
    }
    await cleanupStore();
    await cleanupSignals();
    globalThis.Worker = previousWorker;
  }
}

test("PUBLISH GATE: empty worker-first admits ordinary authoring / form / resource / React / history branch", async () => {
  await withEmptyWorkerFirst(async ({
    signals,
    createReactSignalsStore,
    resourceParams,
    resourceParamIdentity,
  }) => {
    // Spec lane (the prior QMS gap)
    const count = signals.spec.input("matrix.count", 2);
    const scoped = signals.scope("item").spec.input("title", "x");
    const doubled = signals.spec.computed("matrix.doubled", {
      reads: [count.id],
      expr: {
        kind: "sum",
        args: [
          { kind: "read", id: count.id },
          { kind: "read", id: count.id },
        ],
      },
      identity: { kind: "exact" },
    });
    const panel = signals.spec.output("matrix.panel", {
      reads: [doubled.id],
      expr: { kind: "read", id: doubled.id },
      identity: { kind: "exact" },
    });
    const callback = signals.spec.computedCallback("matrix.cb", () => count() * 3);
    const TransferAspect = { financialTerms: 0, operatorNote: 1 };
    const transfer = signals.spec.input(
      "matrix.transfer",
      { amount: 8_000, note: "n" },
      { producesAspects: [TransferAspect.financialTerms, TransferAspect.operatorNote] },
    );
    const notePreview = signals.spec.computed("matrix.notePreview", {
      reads: [{ id: transfer.id, aspect: TransferAspect.operatorNote }],
      expr: {
        kind: "get",
        target: { kind: "read", id: transfer.id },
        field: "note",
      },
      identity: { kind: "exact" },
    });
    assert.equal(count(), 2);
    assert.equal(scoped(), "x");
    assert.equal(doubled(), 4);
    assert.equal(panel(), 4);
    assert.equal(callback(), 6);
    assert.equal(notePreview(), "n");

    // Callable opaque lane (no explicit id — use spec.input for structural names)
    const opaque = signals.input(1);
    assert.equal(opaque(), 1);
    // Scoped callable still accepts { id } options
    const scopedOpaque = signals.scope("item").input("y", { id: "scopedTitle" });
    assert.equal(scopedOpaque.id, "item.scopedTitle");
    assert.equal(scopedOpaque(), "y");

    await signals.settleAuthoredWork();
    await count.set(5);
    assert.equal(count(), 5);
    assert.equal(doubled(), 10);
    assert.equal(callback(), 15);

    // Watch / transaction
    let watched = 0;
    const watchHandle = signals.watch(count, () => {
      watched += 1;
    });
    await signals.transaction((tx) => {
      tx.set(count, 7);
    });
    const deadline = Date.now() + 2000;
    while (watched < 1 && Date.now() < deadline) {
      await new Promise((resolve) => setTimeout(resolve, 10));
    }
    assert.equal(count(), 7);
    assert.ok(watched >= 1, "watch must observe authored spec.input mutations");
    signals.nuke(watchHandle);

    // Form + React
    const form = signals.form({
      source: { title: "A" },
      fields: ({ field }) => ({ title: field("title") }),
      actions: ({ submit }) => ({ submit: submit() }),
    });
    await Promise.resolve(form.fields.title.set("B"));
    assert.equal(form.effective().title, "B");
    const store = createReactSignalsStore(signals);
    assert.equal(store.getSignalSnapshot(count), 7);
    store.dispose();

    // Resource line on empty root
    const detail = signals.resource.detail({
      params: resourceParams(),
      normalizeParams: ({ id }) => resourceParamIdentity({ id }, id),
      load: ({ id }) => ({ id, title: `loaded:${id}` }),
    });
    const line = detail.line({ id: "1" });
    const settled = await line.awaitSettlement({ timeoutMs: 10_000 });
    assert.equal(settled.resultKind, "fulfilled");
    assert.equal(line.value()?.title, "loaded:1");
    assert.deepEqual(line.signal()(), line.value());

    // History branch cache (not import-cached replay/snapshot)
    const history = signals.history();
    assert.ok(history.current_branch() != null);
    assert.ok(Array.isArray(history.branches()));

    // Diagnostics: empty roots start with null; after watch/resource work an
    // empty observation packet is honest (not an import requirement).
    const latest = signals.diagnostics().latestObservation();
    assert.ok(
      latest === null
      || (typeof latest === "object" && latest.observation != null),
    );
    assert.ok(await Promise.resolve(signals.diagnostics().why(count.id)) != null);

    // Controller with authored handles
    const controller = signals.controller({
      inputs: { count },
      outputs: { panel },
      internal: {},
    });
    assert.equal(controller.inputs.count.id, count.id);
    assert.equal(controller.outputs.panel.id, panel.id);
  });
});

test("PUBLISH GATE: empty worker-first still denies import/portable-only surfaces", async () => {
  await withEmptyWorkerFirst(async ({ signals }) => {
    assert.throws(() => signals.adapters().exportRuntimeEnvelope(), IMPORT_REQUIRED);
    assert.throws(() => signals.specialist().graphSummary(), IMPORT_REQUIRED);
    assert.throws(() => signals.graph("x", { inputs: {}, outputs: {} }), IMPORT_REQUIRED);
    assert.throws(() => signals.history().snapshot(), IMPORT_REQUIRED);
    assert.throws(() => signals.history().recentHistory(), IMPORT_REQUIRED);
    assert.throws(
      () => signals.diagnostics().why("missing-empty-root-id"),
      /known authored signal id/u,
    );
  });
});
