import assert from "node:assert/strict";
import test from "node:test";
import { createElement } from "react";
import { createRoot } from "react-dom/client";
import { JSDOM } from "jsdom";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadReactHooksModule } from "../../../host_capabilities_certification/module_loading/load_react_hooks_module.mjs";
import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

async function flush() {
  await Promise.resolve();
  await Promise.resolve();
  await new Promise((resolve) => setTimeout(resolve, 0));
}

function installDom() {
  const dom = new JSDOM("<!doctype html><html><body><div id='root'></div></body></html>", {
    url: "http://localhost/",
  });
  const keys = ["window", "document", "HTMLElement", "Node", "MutationObserver"];
  const previous = Object.fromEntries(keys.map((key) => [key, globalThis[key]]));
  globalThis.window = dom.window;
  globalThis.document = dom.window.document;
  globalThis.HTMLElement = dom.window.HTMLElement;
  globalThis.Node = dom.window.Node;
  globalThis.MutationObserver = dom.window.MutationObserver;
  return () => {
    for (const key of keys) {
      globalThis[key] = previous[key];
    }
    dom.window.close();
  };
}

test("worker-first Provider + useSignalValue/useSignalsDiagnostics render real values", async () => {
  const restoreDom = installDom();
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup: cleanupSignals } = await loadSignalsModule({
    rawSurface: "real",
  });
  const {
    createReactSignalsStore,
    ReactSignalsStoreProvider,
    useSignalValue,
    useSignalsDiagnostics,
    cleanup: cleanupHooks,
  } = await loadReactHooksModule();

  let signals = null;
  let root = null;
  try {
    signals = await createSignals({ deployment: "workerFirst" });
    const quantity = signals.input(2, { debugName: "hooks.quantity" });
    const total = signals.computed(() => quantity() * 3, { debugName: "hooks.total" });
    const store = createReactSignalsStore(signals);

    const observed = { value: null, diagnosticsCount: null };
    function Probe() {
      const value = useSignalValue(total, store);
      const diagnostics = useSignalsDiagnostics();
      observed.value = value;
      observed.diagnosticsCount =
        diagnostics.performanceSummary?.deliveredObservationCount ?? null;
      return createElement("span", null, String(value));
    }

    root = createRoot(document.getElementById("root"));
    root.render(
      createElement(
        ReactSignalsStoreProvider,
        { store },
        createElement(Probe),
      ),
    );
    await flush();
    assert.equal(observed.value, 6);

    await signals.transaction((tx) => {
      tx.set(quantity, 5);
    });
    await flush();
    assert.equal(observed.value, 15);
    assert.equal(typeof observed.diagnosticsCount, "number");

    store.dispose();
  } finally {
    root?.unmount();
    if (signals) {
      await signals.terminate();
    }
    await cleanupHooks();
    await cleanupSignals();
    globalThis.Worker = previousWorker;
    restoreDom();
  }
});
