import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "./module_loading/load_signals_module.mjs";
import { buildHostRawSignals } from "./runtime_fixture/host_raw_signals.mjs";
import { flushMicrotasks } from "./runtime_fixture/host_runtime_scheduling.mjs";

test("createSignals host capability plan registers visibility and tears it down cleanly", async () => {
  const { wrapSignals, hostCapabilityPlan, visibilityCapability, cleanup } = await loadSignalsModule();
  try {
    const calls = [];
    const runtimeState = { values: new Map() };
    let currentVisibility = "visible";
    let listener = null;
    let unsubscribeCount = 0;
    const rawSignals = buildHostRawSignals(runtimeState, calls);

    const signals = wrapSignals(rawSignals, {
      hostCapabilities: hostCapabilityPlan({
        visibility: visibilityCapability({
          source: {
            current() {
              return currentVisibility;
            },
            subscribe(next) {
              listener = next;
              return () => {
                unsubscribeCount += 1;
                listener = null;
              };
            },
          },
          compatibility: "LiveOnly",
        }),
      }),
    });

    assert.equal(signals.host.visibility.state(), "visible");
    assert.equal(signals.host.visibility.isVisible(), true);
    assert.deepEqual(signals.host.visibility.descriptor(), {
      family: "visibility",
      compatibility: "LiveOnly",
      registrationId: "visibility",
    });
    assert.equal(typeof signals.host.visibility.free, "undefined");
    assert.equal(typeof signals.host.visibility[Symbol.dispose], "undefined");
    assert.equal(calls[0][0], "input");
    assert.match(calls[0][1], /^__WORTHSignal\.host\.visibility\.\d+$/);

    currentVisibility = "hidden";
    listener();
    await flushMicrotasks();

    assert.equal(signals.host.visibility.state(), "hidden");
    assert.equal(signals.host.visibility.isVisible(), false);
    assert.deepEqual(calls[1], ["transaction", [["set", calls[0][1], "hidden"]]]);

    signals.free();

    assert.equal(unsubscribeCount, 1);
    assert.deepEqual(calls.at(-1), ["free"]);

    currentVisibility = "visible";
    assert.equal(listener, null);
  } finally {
    await cleanup();
  }
});
