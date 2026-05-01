import assert from "node:assert/strict";
import { mkdir, mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { stripTypeScriptTypes } from "node:module";
import { tmpdir } from "node:os";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const productDir = path.dirname(fileURLToPath(import.meta.url));
const packageDir = path.dirname(productDir);
const packageSourceDir = path.join(packageDir, "..", "package-src");

function flushMicrotasks() {
  return new Promise((resolve) => queueMicrotask(resolve));
}

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function loadSignalsModule() {
  const tempDir = await mkdtemp(path.join(tmpdir(), "forge-signal-host-capability-"));
  try {
    const filesToCopy = [
      ["product/signals.ts", "product/signals.js"],
      ["product/callback_frames.ts", "product/callback_frames.js"],
      ["product/controllers.ts", "product/controllers.js"],
      ["product/diagnostics.ts", "product/diagnostics.js"],
      ["product/graph_authoring_support.ts", "product/graph_authoring_support.js"],
      ["product/graph_support.ts", "product/graph_support.js"],
      ["product/graphs.ts", "product/graphs.js"],
      ["product/host_capability_declarations.ts", "product/host_capability_declarations.js"],
      ["product/host_capability_registrations.ts", "product/host_capability_registrations.js"],
      ["product/host_capability_reports.ts", "product/host_capability_reports.js"],
      ["product/host_capabilities.ts", "product/host_capabilities.js"],
      ["product/history.ts", "product/history.js"],
      ["product/handles.ts", "product/handles.js"],
      ["product/public_inputs.ts", "product/public_inputs.js"],
      ["product/scopes.ts", "product/scopes.js"],
      ["product/specialist.ts", "product/specialist.js"],
      ["product/transactions.ts", "product/transactions.js"],
      ["product/symbols.ts", "product/symbols.js"],
    ];

    for (const [sourceRelativePath, outputRelativePath] of filesToCopy) {
      const sourcePath = path.join(packageSourceDir, sourceRelativePath);
      const targetPath = path.join(tempDir, outputRelativePath);
      await mkdir(path.dirname(targetPath), { recursive: true });
      const source = await readFile(sourcePath, "utf8");
      await writeFile(
        targetPath,
        stripTypeScriptTypes(source, { mode: "transform" }),
        "utf8",
      );
    }

    await writeFile(
      path.join(tempDir, "raw_surface.js"),
      "export function createRawSignals() { throw new Error('createRawSignals should not be used in host capability runtime tests'); }\n",
      "utf8",
    );

    const moduleUrl = new URL(`file:///${path.join(tempDir, "product", "signals.js").replace(/\\/g, "/")}`);
    const loaded = await import(moduleUrl.href);
    return { ...loaded, cleanup: () => rm(tempDir, { recursive: true, force: true }) };
  } catch (error) {
    await rm(tempDir, { recursive: true, force: true });
    throw error;
  }
}

function createMutableRawInputHandle(id, runtimeState) {
  return {
    id,
    get() {
      return runtimeState.values.get(id);
    },
    peek() {
      return runtimeState.values.get(id);
    },
    free() {},
  };
}

function buildHostRawSignals(runtimeState, calls) {
  return {
    input(id, initial) {
      runtimeState.values.set(id, initial);
      calls.push(["input", id, initial]);
      return createMutableRawInputHandle(id, runtimeState);
    },
    computedSpec(id, spec) {
      calls.push(["computedSpec", id, spec]);
      return createMutableRawInputHandle(id, spec);
    },
    computedCallback(id, callback) {
      const result = callback();
      calls.push(["computedCallback", id, result]);
      return createMutableRawInputHandle(id, result.value);
    },
    outputSpec(id, spec) {
      calls.push(["outputSpec", id, spec]);
      return createMutableRawInputHandle(id, spec);
    },
    read(target) {
      return typeof target === "string" ? runtimeState.values.get(target) : target.get();
    },
    watch() {
      return { free() {} };
    },
    effect() {
      return { free() {} };
    },
    transaction(callback) {
      const operations = [];
      callback({
        set(target, value) {
          runtimeState.values.set(target.id, value);
          operations.push(["set", target.id, value]);
        },
        free() {},
      });
      calls.push(["transaction", operations]);
      return { touchedNodes: operations.length };
    },
    batch(callback) {
      return this.transaction(callback);
    },
    nuke() {
      return true;
    },
    diagnostics() {
      return {};
    },
    history() {
      return {};
    },
    specialist() {
      return {};
    },
    adapters() {
      return {};
    },
    compatibilityApp() {
      return {};
    },
    compatibilityRuntime() {
      return {};
    },
    free() {
      calls.push(["free"]);
    },
  };
}

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
    assert.match(calls[0][1], /^__forgeSignal\.host\.visibility\.\d+$/);

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

test("host capability invalidation batches push churn and exposes counters honestly", async () => {
  const { wrapSignals, hostCapabilityPlan, visibilityCapability, cleanup } = await loadSignalsModule();
  try {
    const calls = [];
    let currentVisibility = "visible";
    let listener = null;
    const rawSignals = buildHostRawSignals({ values: new Map() }, calls);
    rawSignals.diagnostics = () => ({
      latestObservation() { return null; },
      latestFlow() { return null; },
      latestFailure() { return null; },
      latestRollback() { return null; },
      latestFrontierExecution() { return null; },
      latestInvalidationTraceRecords() { return []; },
      recentHistory() { return []; },
      historyNow() { return { history: {}, callbackNodes: [] }; },
      why() { return null; },
      health() { return null; },
      summaryNow() { return { profile: "Development" }; },
      performanceSummary() { return { activeHandleCount: 0 }; },
      subscribe() { return { free() {} }; },
      free() {},
    });

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
                listener = null;
              };
            },
          },
        }),
      }),
    });

    signals.computed(() => (signals.host.visibility.isVisible() ? "visible" : "hidden"), { id: "visibilityLabel" });
    currentVisibility = "hidden";
    listener();
    currentVisibility = "hidden";
    listener();
    currentVisibility = "hidden";
    listener();
    await flushMicrotasks();

    const summary = signals.diagnostics().performanceSummary();
    const latestHostEvent = signals.diagnostics().latestHostCapabilityEvent();
    const recentHostEvents = signals.diagnostics().recentHostCapabilityEvents();
    const hostReport = signals.diagnostics().hostCapabilityReport();
    assert.equal(summary.hostCapabilityRegistrationCount, 1);
    assert.equal(summary.hostCapabilityReadCount, 1);
    assert.equal(summary.hostCapabilityInvalidationCount, 3);
    assert.equal(summary.hostCapabilityInvalidationBatchFlushCount, 1);
    assert.equal(summary.hostCapabilityReevaluationCount, 1);
    assert.equal(summary.hostCapabilityNoOpInvalidationSuppressedCount, 0);
    assert.equal(summary.hostCapabilityInvalidationTouchedNodeCount, 1);
    assert.equal(typeof hostReport.lineageDigest, "string");
    assert.equal(typeof hostReport.breadthDigest, "string");
    assert.equal(hostReport.breadth.maxTouchedNodes, 1);
    assert.equal(hostReport.breadth.maxReevaluatedNodes, 1);
    assert.deepEqual(latestHostEvent, {
      sequence: 1,
      kind: "InvalidationFlushed",
      family: "visibility",
      registrationId: "visibility",
      compatibility: "LiveOnly",
      invalidationMode: "push-driven",
      queuedInvalidationCount: 3,
      previousState: "visible",
      nextState: "hidden",
      touchedNodes: 1,
      reevaluatedNodes: 1,
    });
    assert.deepEqual(recentHostEvents, [latestHostEvent]);
    assert.deepEqual(calls.filter((call) => call[0] === "transaction"), [
      ["transaction", [["set", calls[0][1], "hidden"]]],
    ]);

    signals.free();
  } finally {
    await cleanup();
  }
});

test("host capability stale invalidations are ignored after runtime disposal", async () => {
  const { wrapSignals, hostCapabilityPlan, visibilityCapability, cleanup } = await loadSignalsModule();
  try {
    const calls = [];
    let currentVisibility = "visible";
    let listener = null;
    const rawSignals = buildHostRawSignals({ values: new Map() }, calls);
    rawSignals.diagnostics = () => ({
      latestObservation() { return null; },
      latestFlow() { return null; },
      latestFailure() { return null; },
      latestRollback() { return null; },
      latestFrontierExecution() { return null; },
      latestInvalidationTraceRecords() { return []; },
      recentHistory() { return []; },
      historyNow() { return { history: {}, callbackNodes: [] }; },
      why() { return null; },
      health() { return null; },
      summaryNow() { return { profile: "Development" }; },
      performanceSummary() { return { activeHandleCount: 0 }; },
      subscribe() { return { free() {} }; },
      free() {},
    });

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
                listener = null;
              };
            },
          },
        }),
      }),
    });

    const diagnostics = signals.diagnostics();
    signals.free();
    currentVisibility = "hidden";
    assert.equal(listener, null);
    const summary = diagnostics.performanceSummary();
    const hostEvents = diagnostics.recentHostCapabilityEvents();
    assert.equal(summary.hostCapabilityDisposalCount, 1);
    assert.equal(summary.hostCapabilityStaleInvalidationIgnoredCount, 0);
    assert.deepEqual(hostEvents, []);
    assert.deepEqual(calls.filter((call) => call[0] === "transaction"), []);
  } finally {
    await cleanup();
  }
});

test("host capability reads lower through wrapped signal capture and invalid plans are denied", async () => {
  const {
    wrapSignals,
    clockCapability,
    hostCapabilityPlan,
    onlineCapability,
    persistenceCapability,
    viewportCapability,
    visibilityCapability,
    cleanup,
  } = await loadSignalsModule();
  try {
    const calls = [];
    const runtimeState = { values: new Map() };
    const rawSignals = buildHostRawSignals(runtimeState, calls);
    rawSignals.diagnostics = () => ({
      why() { return null; },
      health() { return null; },
      summaryNow() { return { profile: "Development" }; },
      historyNow() { return { history: {}, callbackNodes: [] }; },
      latestObservation() { return null; },
      latestFlow() { return null; },
      performanceSummary() { return { activeHandleCount: 0 }; },
      latestFailure() { return null; },
      latestRollback() { return null; },
      latestFrontierExecution() { return null; },
      latestInvalidationTraceRecords() { return []; },
      recentHistory() { return []; },
      subscribe() { return { free() {} }; },
      free() {},
    });

    let currentVisibility = true;
    let currentViewport = { width: 1280, height: 720 };
    let currentOnline = true;
    let clockTick = 0;
    let persistedDraft = { mode: "draft", revision: 1 };
    const signals = wrapSignals(rawSignals, {
      hostCapabilities: hostCapabilityPlan({
        visibility: visibilityCapability({
          source: {
            current() {
              return currentVisibility;
            },
            subscribe() {
              return () => {};
            },
          },
        }),
        viewport: viewportCapability({
          source: {
            current() {
              return currentViewport;
            },
            subscribe() {
              return () => {};
            },
          },
        }),
        online: onlineCapability({
          source: {
            current() {
              return currentOnline;
            },
            subscribe() {
              return () => {};
            },
          },
        }),
        clock: clockCapability({
          source: {
            current() {
              return clockTick;
            },
          },
          pollMs: 5,
        }),
        persistence: persistenceCapability({
          source: {
            current() {
              return persistedDraft;
            },
          },
        }),
      }),
    });

    const computed = signals.computed(() => (
      signals.host.visibility.isVisible() ? "visible" : "hidden"
    ), { id: "visibilityLabel" });
    const viewportComputed = signals.computed(() => (
      `${signals.host.viewport.width()}x${signals.host.viewport.height()}`
    ), { id: "viewportLabel" });
    const onlineComputed = signals.computed(() => (
      signals.host.online.isOnline() ? "online" : "offline"
    ), { id: "onlineLabel" });
    const clockComputed = signals.computed(() => (
      signals.host.clock.now() + 1
    ), { id: "clockLabel" });
    const persistenceComputed = signals.computed(() => (
      signals.host.persistence.value().revision
    ), { id: "persistenceLabel" });

    assert.equal(computed.id, "visibilityLabel");
    assert.equal(viewportComputed.id, "viewportLabel");
    assert.equal(onlineComputed.id, "onlineLabel");
    assert.equal(clockComputed.id, "clockLabel");
    assert.equal(persistenceComputed.id, "persistenceLabel");
    assert.deepEqual(signals.host.viewport.size(), { width: 1280, height: 720 });
    assert.equal(signals.host.viewport.width(), 1280);
    assert.equal(signals.host.viewport.height(), 720);
    assert.deepEqual(signals.host.viewport.descriptor(), {
      family: "viewport",
      compatibility: "Reattachable",
      registrationId: "viewport",
    });
    assert.equal(signals.host.online.state(), "online");
    assert.equal(signals.host.online.isOnline(), true);
    assert.deepEqual(signals.host.online.descriptor(), {
      family: "online",
      compatibility: "Reattachable",
      registrationId: "online",
    });
    assert.equal(signals.host.clock.now(), 0);
    assert.deepEqual(signals.host.clock.descriptor(), {
      family: "clock",
      compatibility: "SnapshotPortable",
      registrationId: "clock",
    });
    assert.deepEqual(signals.host.persistence.descriptor(), {
      family: "persistence",
      compatibility: "ImportDenied",
      registrationId: "persistence",
    });
    const viewportSourceId = calls.find((call) => call[0] === "input" && String(call[1]).startsWith("__forgeSignal.host.viewport."))?.[1];
    const visibilitySourceId = calls.find((call) => call[0] === "input" && String(call[1]).startsWith("__forgeSignal.host.visibility."))?.[1];
    const onlineSourceId = calls.find((call) => call[0] === "input" && String(call[1]).startsWith("__forgeSignal.host.online."))?.[1];
    const clockSourceId = calls.find((call) => call[0] === "input" && String(call[1]).startsWith("__forgeSignal.host.clock."))?.[1];
    const persistenceSourceId = calls.find((call) => call[0] === "input" && String(call[1]).startsWith("__forgeSignal.host.persistence."))?.[1];
    const computedCalls = calls.filter((call) => call[0] === "computedCallback");
    assert.equal(computedCalls[0][1], "visibilityLabel");
    assert.deepEqual(computedCalls[0][2], {
      __forgeSignalCallbackCapture: true,
      value: "visible",
      reads: [visibilitySourceId],
      hostCapabilityReads: [{
        family: "visibility",
        registrationId: "visibility",
        compatibility: "LiveOnly",
      }],
      runtimeReadBreadth: 0,
    });
    assert.equal(computedCalls[1][1], "viewportLabel");
    assert.deepEqual(computedCalls[1][2], {
      __forgeSignalCallbackCapture: true,
      value: "1280x720",
      reads: [viewportSourceId],
      hostCapabilityReads: [{
        family: "viewport",
        registrationId: "viewport",
        compatibility: "Reattachable",
      }],
      runtimeReadBreadth: 0,
    });
    assert.equal(computedCalls[2][1], "onlineLabel");
    assert.deepEqual(computedCalls[2][2], {
      __forgeSignalCallbackCapture: true,
      value: "online",
      reads: [onlineSourceId],
      hostCapabilityReads: [{
        family: "online",
        registrationId: "online",
        compatibility: "Reattachable",
      }],
      runtimeReadBreadth: 0,
    });
    assert.equal(computedCalls[3][1], "clockLabel");
    assert.deepEqual(computedCalls[3][2], {
      __forgeSignalCallbackCapture: true,
      value: 1,
      reads: [clockSourceId],
      hostCapabilityReads: [{
        family: "clock",
        registrationId: "clock",
        compatibility: "SnapshotPortable",
      }],
      runtimeReadBreadth: 0,
    });
    assert.equal(computedCalls[4][1], "persistenceLabel");
    assert.deepEqual(computedCalls[4][2], {
      __forgeSignalCallbackCapture: true,
      value: 1,
      reads: [persistenceSourceId],
      hostCapabilityReads: [{
        family: "persistence",
        registrationId: "persistence",
        compatibility: "ImportDenied",
      }],
      runtimeReadBreadth: 0,
    });

    currentViewport = { width: 1440, height: 900 };
    assert.equal(signals.host.viewport.width(), 1280);
    clockTick = 5;
    await sleep(15);
    await flushMicrotasks();
    assert.deepEqual(signals.host.viewport.size(), { width: 1280, height: 720 });
    persistedDraft = { mode: "draft", revision: 2 };
    const commitSummary = signals.host.persistence.commit();
    assert.equal(typeof commitSummary?.touchedNodes, "number");
    const noOpCommitSummary = signals.host.persistence.commit();
    assert.equal(noOpCommitSummary.touchedNodes, 0);
    const summary = signals.diagnostics().performanceSummary();
    const latestHostEvent = signals.diagnostics().latestHostCapabilityEvent();
    const hostReport = signals.diagnostics().hostCapabilityReport();
    assert.equal(summary.hostCapabilityPollCount > 0, true);
    assert.equal(summary.hostCapabilityInvalidationCount > 0, true);
    assert.equal(summary.hostCapabilityNoOpPollCount >= 0, true);
    assert.equal(summary.hostCapabilityManualCommitCount, 2);
    assert.equal(summary.hostCapabilityNoOpManualCommitCount, 1);
    assert.equal(summary.hostCapabilityUnavailabilityArtifactCount, 0);
    assert.equal(summary.hostCapabilityBroadFanoutDenialCount, 0);
    assert.equal(summary.hostCapabilityReadCount >= 6, true);
    assert.equal(latestHostEvent?.invalidationMode, "manually-committed");
    assert.equal(typeof hostReport.digest, "string");
    assert.equal(typeof hostReport.lineageDigest, "string");
    assert.equal(typeof hostReport.breadthDigest, "string");
    assert.equal(hostReport.totals.manualCommitCount, 2);
    assert.equal(hostReport.totals.unavailabilityArtifactCount, 0);
    assert.equal(hostReport.families.some((family) => family.family === "persistence"), true);

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
      () => wrapSignals(rawSignals, {
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
      () => wrapSignals(rawSignals, {
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
      () => wrapSignals(rawSignals, {
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
      () => wrapSignals(rawSignals, {
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
      () => clockCapability({
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

    signals.free();
  } finally {
    await cleanup();
  }
});
