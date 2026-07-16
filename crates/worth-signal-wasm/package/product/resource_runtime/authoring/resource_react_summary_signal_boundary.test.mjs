import assert from "node:assert/strict";
import test from "node:test";

import { loadStoreModule } from "../../host_capabilities_certification/module_loading/load_store_module.mjs";
import { createRealRequestRuntime } from "../runtime_fixture/real_request_runtime.mjs";

function flushMicrotasks() {
  return new Promise((resolve) => queueMicrotask(resolve));
}

function notificationBarrier() {
  let resolveNotification;
  const notification = new Promise((resolve) => {
    resolveNotification = resolve;
  });
  return {
    notify: resolveNotification,
    wait: () => Promise.race([
      notification,
      new Promise((_, reject) => setTimeout(
        () => reject(new Error("timed out waiting for the React summary signal")),
        5_000,
      )),
    ]),
  };
}

test("resource summary signals stay readable through the React store without eager history explainability reads", async () => {
  const runtime = await createRealRequestRuntime();
  const { createReactSignalsStore, cleanup } = await loadStoreModule();
  try {
    const user = runtime.signals.api({}).url("/users/:userId")
      .response(runtime.signals.resource.response.detail()({
        name: "name",
      }))
      .detail({
        load: ({ userId }) => ({ id: userId, name: "First" }),
      });
    const line = user.line({ userId: "u1" });
    const summarySignal = line.summarySignal();
    const store = createReactSignalsStore(runtime.signals);
    let notificationCount = 0;
    const notification = notificationBarrier();

    const unsubscribe = store.subscribeSignal(summarySignal, () => {
      notificationCount += 1;
      notification.notify();
    });

    const initialSummary = store.getSignalSnapshot(summarySignal);
    assert.equal(initialSummary.current.status.kind, "fulfilled");
    assert.equal(initialSummary.diagnostics.latest.patchKind, null);
    assert.equal(initialSummary.explainability.replay.kind, "unavailable");
    assert.match(initialSummary.explainability.replay.detail, /deferred/i);

    await line.patch(user.patch.field({
      field: "name",
      value: "Updated",
    }));
    await notification.wait();

    const patchedSummary = store.getSignalSnapshot(summarySignal);
    assert.equal(notificationCount, 1);
    assert.equal(patchedSummary.diagnostics.latest.patchKind, "field");
    assert.equal(patchedSummary.diagnostics.latest.patchedField, "name");
    assert.equal(patchedSummary.explainability.replay.kind, "unavailable");

    unsubscribe();
    store.dispose();
  } finally {
    await cleanup();
    await runtime.cleanup();
  }
});

test("resource summary signals stay readable for async initial detail loads without mutable-borrow panics", async () => {
  const runtime = await createRealRequestRuntime();
  const { createReactSignalsStore, cleanup } = await loadStoreModule();
  try {
    let resolveLoad;
    const loadPromise = new Promise((resolve) => {
      resolveLoad = resolve;
    });

    const user = runtime.signals.api({}).url("/users/:userId")
      .response(runtime.signals.resource.response.detail()({
        name: "name",
      }))
      .detail({
        load: ({ userId }) => loadPromise.then(() => ({ id: userId, name: "First" })),
      });
    const line = user.line({ userId: "u1" });
    const summarySignal = line.summarySignal();
    const store = createReactSignalsStore(runtime.signals);
    let notificationCount = 0;
    const notification = notificationBarrier();

    const unsubscribe = store.subscribeSignal(summarySignal, () => {
      notificationCount += 1;
      notification.notify();
    });

    const pendingSummary = store.getSignalSnapshot(summarySignal);
    assert.equal(pendingSummary.current.status.kind, "pending");

    resolveLoad();
    await line.awaitSettlement({ timeoutMs: 5_000 });
    await notification.wait();
    await flushMicrotasks();
    const fulfilledSummary = store.getSignalSnapshot(summarySignal);
    assert.equal(fulfilledSummary.current.status.kind, "fulfilled");
    assert.deepEqual(line.value(), { id: "u1", name: "First" });
    assert.equal(fulfilledSummary.diagnostics.latest.patchKind, null);
    assert.equal(
      store.getSignalSnapshot(summarySignal).explainability.replay.kind,
      "unavailable",
    );

    unsubscribe();
    store.dispose();
  } finally {
    await cleanup();
    await runtime.cleanup();
  }
});
