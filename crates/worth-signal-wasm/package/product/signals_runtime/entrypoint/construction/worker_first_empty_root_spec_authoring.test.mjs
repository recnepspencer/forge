import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadStoreModule } from "../../../host_capabilities_certification/module_loading/load_store_module.mjs";
import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

const IMPORT_REQUIRED_MASK = /active imported graph|binds only to input ids from the active imported graph/u;

test("worker-first empty root admits full signals.spec authoring lane", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  let signals = null;
  try {
    signals = await createSignals({ deployment: "workerFirst" });

    const serverItemData = signals.spec.input("serverItemData", null);
    const draftEdits = signals.spec.input("draftEdits", {});
    const scopedCount = signals.scope("wizard").spec.input("count", 2);
    const doubled = signals.spec.computed("doubled", {
      reads: [scopedCount.id],
      expr: {
        kind: "sum",
        args: [
          { kind: "read", id: scopedCount.id },
          { kind: "read", id: scopedCount.id },
        ],
      },
      identity: { kind: "exact" },
    });
    const panel = signals.spec.output("panel", {
      reads: [doubled.id],
      expr: { kind: "read", id: doubled.id },
      identity: { kind: "exact" },
    });
    const viaComputedSpec = signals.computedSpec("viaComputedSpec", {
      reads: [scopedCount.id],
      expr: { kind: "read", id: scopedCount.id },
      identity: { kind: "exact" },
    });
    const viaOutputSpec = signals.outputSpec("viaOutputSpec", {
      reads: [viaComputedSpec.id],
      expr: { kind: "read", id: viaComputedSpec.id },
      identity: { kind: "exact" },
    });
    const scopedComputedSpec = signals.scope("wizard").computedSpec("scopedDoubled", {
      reads: [scopedCount.id],
      expr: {
        kind: "sum",
        args: [
          { kind: "read", id: scopedCount.id },
          { kind: "read", id: scopedCount.id },
        ],
      },
      identity: { kind: "exact" },
    });
    const scopedOutputSpec = signals.scope("wizard").outputSpec("scopedPanel", {
      reads: [scopedComputedSpec.id],
      expr: { kind: "read", id: scopedComputedSpec.id },
      identity: { kind: "exact" },
    });
    const callback = signals.spec.computedCallback(
      "callbackTotal",
      () => (scopedCount() ?? 0) * 2,
    );
    const outputCallback = signals.scope("wizard").spec.outputCallback(
      "callbackPanel",
      () => callback(),
    );

    assert.equal(serverItemData.id, "serverItemData");
    assert.equal(draftEdits.id, "draftEdits");
    assert.equal(scopedCount.id, "wizard.count");
    assert.equal(scopedComputedSpec.id, "wizard.scopedDoubled");
    assert.equal(scopedOutputSpec.id, "wizard.scopedPanel");
    assert.equal(outputCallback.id, "wizard.callbackPanel");
    assert.equal(serverItemData(), null);
    assert.deepEqual(draftEdits(), {});
    assert.equal(scopedCount(), 2);
    assert.equal(doubled(), 4);
    assert.equal(panel(), 4);
    assert.equal(viaComputedSpec(), 2);
    assert.equal(viaOutputSpec(), 2);
    assert.equal(scopedComputedSpec(), 4);
    assert.equal(scopedOutputSpec(), 4);
    assert.equal(callback(), 4);
    assert.equal(outputCallback(), 4);

    await signals.settleAuthoredWork();
    // Worker-backed why must succeed for authored spec ids on empty roots.
    const why = await Promise.resolve(signals.diagnostics().why(scopedCount.id));
    assert.ok(why != null, "authored spec.input must be known to diagnostics.why");

    await scopedCount.set(9);
    assert.equal(scopedCount(), 9);
    assert.equal(doubled(), 18);
    assert.equal(panel(), 18);
    assert.equal(scopedComputedSpec(), 18);
    assert.equal(scopedOutputSpec(), 18);
    assert.equal(callback(), 18);
    assert.equal(outputCallback(), 18);
  } finally {
    if (signals) {
      await signals.terminate();
    }
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

test("worker-first empty root QMS-shaped spec.input composition does not require importGraph", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  let signals = null;
  try {
    signals = await createSignals({ deployment: "workerFirst" });
    const surface = signals.scope("workspaceItemDetail");

    // Mirrors QMS workspaceItemDetail.composition.ts style call sites.
    let serverItemData;
    assert.doesNotThrow(() => {
      serverItemData = surface.spec.input("serverItemData", { id: "seed" });
    }, IMPORT_REQUIRED_MASK);

    const draftEdits = surface.spec.input("draftEdits", {});
    const title = surface.spec.input("title", "");
    const merged = surface.spec.computedCallback("mergedPreview", () => ({
      ...(serverItemData() ?? {}),
      ...draftEdits(),
      title: title(),
    }));

    assert.equal(serverItemData.id, "workspaceItemDetail.serverItemData");
    assert.equal(draftEdits.id, "workspaceItemDetail.draftEdits");
    assert.equal(title.id, "workspaceItemDetail.title");
    assert.equal(merged.id, "workspaceItemDetail.mergedPreview");

    await signals.settleAuthoredWork();
    await title.set("Optimistic");
    await draftEdits.set({ status: "draft" });
    assert.equal(title(), "Optimistic");
    assert.deepEqual(draftEdits(), { status: "draft" });
    assert.deepEqual(merged(), {
      id: "seed",
      status: "draft",
      title: "Optimistic",
    });
    assert.ok(
      (await Promise.resolve(signals.diagnostics().why(title.id))) != null,
      "empty-root spec.input must be worker-known after settleAuthoredWork",
    );
  } finally {
    if (signals) {
      await signals.terminate();
    }
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

test("worker-first empty root React store reads/writes signals.spec.input handles", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup: cleanupSignals } = await loadSignalsModule({
    rawSurface: "real",
  });
  const { createReactSignalsStore, cleanup: cleanupStore } = await loadStoreModule();
  let signals = null;
  try {
    signals = await createSignals({ deployment: "workerFirst" });
    const count = signals.spec.input("reactCount", 1);
    const store = createReactSignalsStore(signals);

    assert.equal(store.getSignalSnapshot(count), 1);
    let notified = 0;
    const unsubscribe = store.subscribeSignal(count, () => {
      notified += 1;
    });

    await signals.settleAuthoredWork();
    await count.set(5);

    const deadline = Date.now() + 2000;
    while (notified < 1 && Date.now() < deadline) {
      await new Promise((resolve) => setTimeout(resolve, 10));
    }
    assert.equal(store.getSignalSnapshot(count), 5);
    assert.ok(notified >= 1, "React store must observe spec.input mutations on empty roots");
    unsubscribe();
    store.dispose();
  } finally {
    if (signals) {
      await signals.terminate();
    }
    await cleanupStore();
    await cleanupSignals();
    globalThis.Worker = previousWorker;
  }
});
