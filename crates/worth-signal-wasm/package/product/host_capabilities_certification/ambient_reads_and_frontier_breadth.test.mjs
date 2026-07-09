import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "./module_loading/load_signals_module.mjs";
import { createReactiveRawSignals } from "./runtime_fixture/reactive_raw_signals.mjs";
import { flushMicrotasks } from "./runtime_fixture/scheduling.mjs";

test("host capability certification keeps ambient reads non-reactive and bounds invalidation to the affected frontier", async () => {
  const {
    hostCapabilityPlan,
    viewportCapability,
    visibilityCapability,
    wrapSignals,
    cleanup,
  } = await loadSignalsModule();
  try {
    const runtime = createReactiveRawSignals();
    let ambientBreakpoint = "wide";
    let visibilityState = "visible";
    let viewportState = { width: 1280, height: 720 };
    let visibilityListener = null;
    let viewportListener = null;

    const signals = wrapSignals(runtime.rawSignals, {
      hostCapabilities: hostCapabilityPlan({
        visibility: visibilityCapability({
          source: {
            current() {
              return visibilityState;
            },
            subscribe(next) {
              visibilityListener = next;
              return () => {
                visibilityListener = null;
              };
            },
          },
          compatibility: "LiveOnly",
        }),
        viewport: viewportCapability({
          source: {
            current() {
              return viewportState;
            },
            subscribe(next) {
              viewportListener = next;
              return () => {
                viewportListener = null;
              };
            },
          },
        }),
      }),
    });

    const count = signals.spec.input("count", 1);
    const ambientMixed = signals.spec.computedCallback(
      "ambientMixed",
      () => `${ambientBreakpoint}:${signals.host.visibility.state()}`,
    );
    const visibilityOnly = signals.spec.computedCallback(
      "visibilityOnly",
      () => (signals.host.visibility.isVisible() ? "onscreen" : "hidden"),
    );
    const viewportOnly = signals.spec.computedCallback(
      "viewportOnly",
      () => `${signals.host.viewport.width()}x${signals.host.viewport.height()}`,
    );
    const signalOnly = signals.spec.computedCallback(
      "signalOnly",
      () => count() * 2,
    );

    assert.equal(signals.read(ambientMixed), "wide:visible");
    assert.equal(signals.read(visibilityOnly), "onscreen");
    assert.equal(signals.read(viewportOnly), "1280x720");
    assert.equal(signals.read(signalOnly), 2);

    ambientBreakpoint = "narrow";
    await flushMicrotasks();

    assert.equal(
      signals.read(ambientMixed),
      "wide:visible",
      "ambient-only closure changes must not invalidate callback-authored host-capability nodes",
    );
    assert.equal(
      signals.diagnostics().performanceSummary().hostCapabilityInvalidationCount,
      0,
      "ambient closure churn must not charge host invalidation counters",
    );

    visibilityState = "hidden";
    visibilityListener();
    await flushMicrotasks();

    const flow = signals.diagnostics().latestFlow();
    const report = signals.diagnostics().hostCapabilityReport();
    const visibilitySourceId = runtime.calls.find(
      (call) => call[0] === "input" && String(call[1]).startsWith("__WorthSignal.host.visibility."),
    )?.[1];

    assert.equal(
      signals.read(ambientMixed),
      "narrow:hidden",
      "ambient closure values may affect the next declared-capability-driven recomputation without becoming reactive themselves",
    );
    assert.equal(signals.read(visibilityOnly), "hidden");
    assert.equal(signals.read(viewportOnly), "1280x720");
    assert.equal(signals.read(signalOnly), 2);
    assert.equal(report.breadth.maxTouchedNodes, 3);
    assert.equal(report.breadth.maxReevaluatedNodes, 3);
    assert.equal(
      report.families.find((family) => family.family === "visibility")
        ?.maxTouchedNodes,
      3,
    );
    assert.equal(typeof report.lineageDigest, "string");
    assert.equal(typeof report.breadthDigest, "string");
    assert.deepEqual(
      flow.callbackNodes.find((node) => node.id === "ambientMixed")?.hostCapabilityReads,
      [{ family: "visibility", registrationId: "visibility", compatibility: "LiveOnly" }],
    );
    assert.deepEqual(
      flow.callbackNodes.find((node) => node.id === "ambientMixed")?.currentReads,
      [visibilitySourceId],
      "ambient closure state must not appear as a dependency edge",
    );
    signals.free();
  } finally {
    await cleanup();
  }
});
