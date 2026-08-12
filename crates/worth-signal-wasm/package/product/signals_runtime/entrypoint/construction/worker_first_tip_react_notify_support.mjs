import assert from "node:assert/strict";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadStoreModule } from "../../../host_capabilities_certification/module_loading/load_store_module.mjs";
import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

export async function flushMicrotasks() {
  await Promise.resolve();
  await Promise.resolve();
}

export async function withTipNotifyWorld(run) {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const signalsModule = await loadSignalsModule({ rawSurface: "real" });
  const { createReactSignalsStore, cleanup: cleanupStore } = await loadStoreModule();
  let signals = null;
  let store = null;
  let compatibility = null;
  try {
    signals = await signalsModule.createSignals({ deployment: "workerFirst" });
    store = createReactSignalsStore(signals);
    await run({
      signals,
      store,
      createSignals: signalsModule.createSignals,
      resourceParams: signalsModule.resourceParams,
      resourceParamIdentity: signalsModule.resourceParamIdentity,
      async openCompatibility() {
        compatibility = await signalsModule.createSignals({
          deployment: "mainThreadCompatibility",
        });
        return compatibility;
      },
    });
  } finally {
    store?.dispose();
    if (signals) {
      await signals.terminate();
    }
    compatibility?.free?.();
    await cleanupStore();
    await signalsModule.cleanup();
    globalThis.Worker = previousWorker;
  }
}

/**
 * Queue unrelated authored work so the serial mutation lane is non-empty.
 * This is pressure, not a hard gate — tip assertions must not await settle.
 */
export function queueCompetingMutation(signals, label) {
  const blocker = signals.input(0, { debugName: `tipNotify.blocker.${label}` });
  void blocker.set(1);
  return blocker;
}

/**
 * Prove tip notify/snapshot without ever awaiting settleAuthoredWork.
 * Calling settle afterward is cleanup only — paint claims are closed before it.
 */
export async function assertTipPaintsWithoutSettle({
  signals,
  store,
  signal,
  expected,
  mutate,
  label,
  exactNotifications = null,
}) {
  let notifications = 0;
  const unsubscribe = store.subscribeSignal(signal, () => {
    notifications += 1;
  });
  queueCompetingMutation(signals, label);

  mutate();
  await flushMicrotasks();

  assert.equal(
    store.getSignalSnapshot(signal),
    expected,
    `${label}: React tip snapshot must paint without settleAuthoredWork`,
  );
  if (exactNotifications === null) {
    assert.ok(
      notifications >= 1,
      `${label}: React store must notify on tip advance without settleAuthoredWork`,
    );
  } else {
    assert.equal(
      notifications,
      exactNotifications,
      `${label}: React store must notify exactly ${exactNotifications} time(s) on tip ingress`,
    );
  }

  await signals.settleAuthoredWork();
  unsubscribe();
}
