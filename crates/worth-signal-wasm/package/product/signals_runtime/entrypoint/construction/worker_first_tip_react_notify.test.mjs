import assert from "node:assert/strict";
import test from "node:test";

import {
  assertTipPaintsWithoutSettle,
  flushMicrotasks,
  queueCompetingMutation,
  withTipNotifyWorld,
} from "./worker_first_tip_react_notify_support.mjs";

test("ingress matrix: authored set tip-notifies without settle", async () => {
  await withTipNotifyWorld(async ({ signals, store }) => {
    const open = signals.input(true, { debugName: "tipNotify.authoredOpen" });
    await assertTipPaintsWithoutSettle({
      signals,
      store,
      signal: open,
      expected: false,
      label: "authored.set",
      exactNotifications: 1,
      mutate: () => {
        open.set(false);
      },
    });
  });
});

test("ingress matrix: transaction setWithAspects tip-notifies before transaction await", async () => {
  await withTipNotifyWorld(async ({ signals, store }) => {
    const open = signals.input(true, { debugName: "tipNotify.txOpen" });
    let notifications = 0;
    const unsubscribe = store.subscribeSignal(open, () => {
      notifications += 1;
    });
    queueCompetingMutation(signals, "tx");

    const pending = Promise.resolve(
      signals.transaction((tx) => {
        tx.setWithAspects(open, false, []);
      }),
    );
    await flushMicrotasks();
    assert.equal(store.getSignalSnapshot(open), false);
    assert.equal(notifications, 1);
    await pending;
    await flushMicrotasks();
    assert.equal(
      notifications,
      1,
      "worker confirmation must not re-notify when tip already matches",
    );
    unsubscribe();
  });
});

test("ingress matrix: resource fulfill tip-notifies line.signal before awaitSettlement", async () => {
  await withTipNotifyWorld(async ({
    signals,
    store,
    resourceParams,
    resourceParamIdentity,
  }) => {
    const payload = { id: "task-tip", title: "TipNotify" };
    let resolveLoad;
    const loadGate = new Promise((resolve) => {
      resolveLoad = resolve;
    });
    const detail = signals.resource.detail({
      params: resourceParams(),
      normalizeParams: ({ taskId }) => resourceParamIdentity({ taskId }, taskId),
      load: async () => {
        await loadGate;
        return payload;
      },
    });
    const line = detail.line({ taskId: "task-tip" });
    try {
      const valueSignal = line.signal();
      let notifications = 0;
      const unsubscribe = store.subscribeSignal(valueSignal, () => {
        notifications += 1;
      });
      queueCompetingMutation(signals, "resource");

      resolveLoad();
      for (let i = 0; i < 32; i += 1) {
        await flushMicrotasks();
        if (
          store.getSignalSnapshot(valueSignal) != null
          && notifications >= 1
        ) {
          break;
        }
      }

      assert.deepEqual(
        store.getSignalSnapshot(valueSignal),
        payload,
        "resource tip/local-truth must reach React before awaitSettlement",
      );
      assert.ok(notifications >= 1, "resource tip must notify React store");

      const settlement = line.awaitSettlement({ timeoutMs: 10_000 });
      assert.deepEqual(store.getSignalSnapshot(valueSignal), payload);
      await settlement;
      unsubscribe();
    } finally {
      line?.free?.();
    }
  });
});

test("ingress matrix: imported-graph input set tip-notifies before write await", async () => {
  await withTipNotifyWorld(async ({ signals, store, openCompatibility }) => {
    const compatibility = await openCompatibility();
    const count = compatibility.input(2, { debugName: "tipNotify.importCount" });
    const graph = compatibility.graph("tipNotifyImport", {
      inputs: { count },
      outputs: { count },
    });
    const imported = signals.importGraph(graph.exportDefinition(), graph.exportSnapshot());
    await imported.ready();
    const importedCount = imported.input("count");
    let notifications = 0;
    const unsubscribe = store.subscribeSignal(importedCount, () => {
      notifications += 1;
    });

    const pending = imported.writeInput("count", 9);
    await flushMicrotasks();
    assert.equal(
      store.getSignalSnapshot(importedCount),
      9,
      "imported.writeInput must tip-notify before worker apply await",
    );
    assert.ok(notifications >= 1);
    await pending;
    unsubscribe();
    await imported.terminate();
  });
});

test("ingress matrix: imported-graph input patch tips merged complete value", async () => {
  await withTipNotifyWorld(async ({ signals, store, openCompatibility }) => {
    const compatibility = await openCompatibility();
    const record = compatibility.input(
      { a: 1, b: 2 },
      { debugName: "tipNotify.importPatch" },
    );
    const graph = compatibility.graph("tipNotifyImportPatch", {
      inputs: { record },
      outputs: { record },
    });
    const imported = signals.importGraph(graph.exportDefinition(), graph.exportSnapshot());
    await imported.ready();
    const importedRecord = imported.input("record");
    let notifications = 0;
    const unsubscribe = store.subscribeSignal(importedRecord, () => {
      notifications += 1;
    });

    const pending = importedRecord.patch({ a: 9 });
    await flushMicrotasks();
    assert.deepEqual(
      store.getSignalSnapshot(importedRecord),
      { a: 9, b: 2 },
      "imported patch must tip the merged object, not the fragment",
    );
    assert.ok(notifications >= 1);
    await pending;
    unsubscribe();
    await imported.terminate();
  });
});

test("ingress matrix: signals.graph transaction tip-notifies before transaction await", async () => {
  await withTipNotifyWorld(async ({ signals, store, openCompatibility }) => {
    const compatibility = await openCompatibility();
    const left = compatibility.input(1, { debugName: "tipNotify.graphLeft" });
    const source = compatibility.graph("tipNotifyGraphSource", {
      inputs: { left },
      outputs: {
        mirrored: compatibility.computedSpec("tipNotify.mirrored", {
          reads: [left.id],
          expr: { kind: "read", id: left.id },
          identity: { kind: "exact" },
        }),
      },
    });
    const imported = signals.importGraph(source.exportDefinition(), source.exportSnapshot());
    await imported.ready();
    const alias = signals.graph("tipNotifyAlias", {
      inputs: { left: signals.publicInput(imported.input("left")) },
      outputs: { mirrored: imported.output("mirrored") },
    });
    const leftInput = alias.input("left");
    let notifications = 0;
    const unsubscribe = store.subscribeSignal(leftInput, () => {
      notifications += 1;
    });

    const pending = Promise.resolve(
      alias.transaction((tx) => {
        tx.set("left", 6);
      }),
    );
    await flushMicrotasks();
    assert.equal(store.getSignalSnapshot(leftInput), 6);
    assert.ok(notifications >= 1);
    await pending;
    unsubscribe();
    await imported.terminate();
  });
});

test("ingress matrix: callback dependent tip-projects without settle", async () => {
  await withTipNotifyWorld(async ({ signals, store }) => {
    const base = signals.input(1, { debugName: "tipNotify.callbackBase" });
    const doubled = signals.computed(() => base() * 2);
    await assertTipPaintsWithoutSettle({
      signals,
      store,
      signal: doubled,
      expected: 10,
      label: "callback.dependent",
      exactNotifications: 1,
      mutate: () => {
        base.set(5);
      },
    });
  });
});

test("ingress matrix: declarative then callback tip-projects without settle", async () => {
  await withTipNotifyWorld(async ({ signals, store }) => {
    const base = signals.input(3, { debugName: "tipNotify.declBase" });
    const mid = signals.spec.computed("tipNotify.declMid", {
      reads: [base.id],
      expr: { kind: "multiply", args: [{ kind: "read", id: base.id }, { kind: "value", value: 2 }] },
      identity: { kind: "exact" },
    });
    const doubled = signals.computed(() => mid() * 2);
    await assertTipPaintsWithoutSettle({
      signals,
      store,
      signal: doubled,
      expected: 24,
      label: "declarative.then.callback",
      mutate: () => {
        base.set(6);
      },
    });
  });
});

test("ingress matrix: authored reset tip-notifies without settle", async () => {
  await withTipNotifyWorld(async ({ signals, store }) => {
    const count = signals.input(7, { debugName: "tipNotify.resetCount" });
    await count.set(99);
    await signals.settleAuthoredWork();
    await assertTipPaintsWithoutSettle({
      signals,
      store,
      signal: count,
      expected: 7,
      label: "authored.reset",
      exactNotifications: 1,
      mutate: () => {
        count.reset();
      },
    });
  });
});
