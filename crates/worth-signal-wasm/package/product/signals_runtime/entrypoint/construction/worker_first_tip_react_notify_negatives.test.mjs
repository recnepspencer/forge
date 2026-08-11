import assert from "node:assert/strict";
import test from "node:test";

import {
  flushMicrotasks,
  queueCompetingMutation,
  withTipNotifyWorld,
} from "./worker_first_tip_react_notify_support.mjs";

test("negative: older failed tip rollback does not clobber newer tip", async () => {
  await withTipNotifyWorld(async ({ signals, store }) => {
    const count = signals.input(0, { debugName: "tipNotify.rollbackEpoch" });
    store.subscribeSignal(count, () => {});

    const tipA = signals.commitHostTipAndNotify([{ id: count.id, value: 10 }]);
    const tipB = signals.commitHostTipAndNotify([{ id: count.id, value: 20 }]);
    await flushMicrotasks();
    assert.equal(store.getSignalSnapshot(count), 20);
    tipA.rollback();
    await flushMicrotasks();
    assert.equal(
      store.getSignalSnapshot(count),
      20,
      "older tip rollback must not clobber newer tip epoch",
    );
    void tipB;
    await signals.settleAuthoredWork();
  });
});

test("negative: older worker apply does not clobber newer tip", async () => {
  await withTipNotifyWorld(async ({ signals, store }) => {
    const count = signals.input(0, { debugName: "tipNotify.applyEpoch" });
    let notifications = 0;
    const unsubscribe = store.subscribeSignal(count, () => {
      notifications += 1;
    });

    const first = count.set(1);
    await flushMicrotasks();
    assert.equal(store.getSignalSnapshot(count), 1);
    assert.ok(notifications >= 1);

    const second = count.set(2);
    await flushMicrotasks();
    assert.equal(store.getSignalSnapshot(count), 2);
    const notificationsAfterNewerTip = notifications;

    await first;
    await flushMicrotasks();
    assert.equal(
      store.getSignalSnapshot(count),
      2,
      "older apply confirmation must not clobber newer tip",
    );
    assert.equal(
      notifications,
      notificationsAfterNewerTip,
      "older apply must not force a second observer pulse after newer tip",
    );

    await second;
    await flushMicrotasks();
    assert.equal(store.getSignalSnapshot(count), 2);
    assert.equal(notifications, notificationsAfterNewerTip);
    await signals.settleAuthoredWork();
    unsubscribe();
  });
});

test("negative: tip snapshot correct without calling settleAuthoredWork", async () => {
  await withTipNotifyWorld(async ({ signals, store }) => {
    const open = signals.input(true, { debugName: "tipNotify.negativeOpen" });
    let notifications = 0;
    const unsubscribe = store.subscribeSignal(open, () => {
      notifications += 1;
    });
    for (let i = 0; i < 8; i += 1) {
      queueCompetingMutation(signals, `neg-${i}`);
    }
    open.set(false);
    await flushMicrotasks();
    assert.equal(store.getSignalSnapshot(open), false);
    assert.equal(notifications, 1);
    unsubscribe();
  });
});

test("negative: default awaitSettlement is tip-status only (no authored drain)", async () => {
  await withTipNotifyWorld(async ({
    signals,
    store,
    resourceParams,
    resourceParamIdentity,
  }) => {
    const payload = { id: "tip-status-only", title: "NoDrain" };
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
    const line = detail.line({ taskId: "tip-status-only" });
    try {
      // Authored work that would be awaited by settleAuthoredWork / drainAuthoredWork.
      const noise = signals.input(0, { debugName: "tipNotify.noDrainNoise" });
      const pendingA = noise.set(1);
      const pendingB = noise.set(2);
      const valueSignal = line.signal();
      store.subscribeSignal(valueSignal, () => {});
      const settleCountBefore = signals.authoredSettleInvocationCount();

      resolveLoad();
      const settled = await line.awaitSettlement({ timeoutMs: 10_000 });
      assert.equal(settled.resultKind, "fulfilled");
      assert.deepEqual(store.getSignalSnapshot(valueSignal), payload);
      assert.equal(
        signals.authoredSettleInvocationCount(),
        settleCountBefore,
        "default awaitSettlement must not invoke settleAuthoredWork / authored drain",
      );

      // Old drain-by-default would have awaited these before returning.
      // Explicit settle is the only drain starter after tip-status settlement.
      await signals.settleAuthoredWork();
      assert.equal(
        signals.authoredSettleInvocationCount(),
        settleCountBefore + 1,
      );
      await Promise.all([pendingA, pendingB]);
    } finally {
      line?.free?.();
    }
  });
});
