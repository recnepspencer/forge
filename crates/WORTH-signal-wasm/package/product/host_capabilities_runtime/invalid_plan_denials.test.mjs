import assert from "node:assert/strict";
import test from "node:test";

import { createMultiCapabilitySignalsCase } from "./runtime_fixture/multi_capability_signals_case.mjs";

test("host capability invalid plans are denied", async () => {
  const {
    cleanup,
    rawSignals,
    wrapSignals,
    clockCapability,
    hostCapabilityPlan,
    onlineCapability,
    persistenceCapability,
    viewportCapability,
    visibilityCapability,
  } = await createMultiCapabilitySignalsCase();
  try {
    assert.throws(
      () => wrapSignals(rawSignals, { hostCapabilities: { visibility: {} } }),
      /hostCapabilities must be created with hostCapabilityPlan/,
    );
    assert.throws(
      () => hostCapabilityPlan({ visibility: { family: "visibility" } }),
      /must be created with visibilityCapability/,
    );
    assert.throws(
      () => hostCapabilityPlan({ viewport: { family: "viewport" } }),
      /must be created with viewportCapability/,
    );
    assert.throws(
      () => hostCapabilityPlan({ online: { family: "online" } }),
      /must be created with onlineCapability/,
    );
    assert.throws(
      () => hostCapabilityPlan({ clock: { family: "clock" } }),
      /must be created with clockCapability/,
    );
    assert.throws(
      () =>
        wrapSignals(rawSignals, {
          hostCapabilities: hostCapabilityPlan({
            visibility: visibilityCapability({
              source: {
                current() {
                  return "unknown";
                },
                subscribe() {
                  return () => {};
                },
              },
            }),
          }),
        }),
      /must return `visible`, `hidden`, true, or false/,
    );
    assert.throws(
      () =>
        wrapSignals(rawSignals, {
          hostCapabilities: hostCapabilityPlan({
            viewport: viewportCapability({
              source: {
                current() {
                  return { width: "wide", height: 720 };
                },
                subscribe() {
                  return () => {};
                },
              },
            }),
          }),
        }),
      /width must be a finite number/,
    );
    assert.throws(
      () =>
        wrapSignals(rawSignals, {
          hostCapabilities: hostCapabilityPlan({
            online: onlineCapability({
              source: {
                current() {
                  return "unknown";
                },
                subscribe() {
                  return () => {};
                },
              },
            }),
          }),
        }),
      /must return `online`, `offline`, true, or false/,
    );
    assert.throws(
      () =>
        wrapSignals(rawSignals, {
          hostCapabilities: hostCapabilityPlan({
            clock: clockCapability({
              source: {
                current() {
                  return Number.NaN;
                },
              },
            }),
          }),
        }),
      /must return a finite number/,
    );
    assert.throws(
      () =>
        clockCapability({
          source: {
            current() {
              return 1;
            },
          },
          pollMs: 0,
        }),
      /pollMs must be a positive integer/,
    );
    assert.throws(
      () => hostCapabilityPlan({ persistence: { family: "persistence" } }),
      /must be created with persistenceCapability/,
    );
  } finally {
    await cleanup();
  }
});
