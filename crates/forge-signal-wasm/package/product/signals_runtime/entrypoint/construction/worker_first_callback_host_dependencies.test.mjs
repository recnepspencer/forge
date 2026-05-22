import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

test("worker-first callback host reads lower into typed dependencies and refresh from host ingress", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const {
    createSignals,
    clockCapability,
    hostCapabilityPlan,
    onlineCapability,
    persistenceCapability,
    viewportCapability,
    visibilityCapability,
    cleanup,
  } = await loadSignalsModule({ rawSurface: "real" });
  let workerSignals = null;
  try {
    const onlineSource = createSubscribableSource("online");
    const visibilitySource = createSubscribableSource("visible");
    const viewportSource = createSubscribableSource({ width: 1024, height: 768 });
    const clockSource = createMutablePersistenceSource(10);
    const persistenceSource = createMutablePersistenceSource({ revision: 1 });
    workerSignals = await createSignals({
      hostCapabilities: hostCapabilityPlan({
        online: onlineCapability({ source: onlineSource.source }),
        visibility: visibilityCapability({ source: visibilitySource.source }),
        viewport: viewportCapability({ source: viewportSource.source }),
        clock: clockCapability({ source: clockSource, pollMs: 10 }),
        persistence: persistenceCapability({ source: persistenceSource }),
      }),
    });
    const count = workerSignals.input(2);
    const hostLabel = workerSignals.computed(() => [
      count(),
      workerSignals.host.online.state(),
      workerSignals.host.visibility.state(),
      workerSignals.host.viewport.width(),
      workerSignals.host.clock.now(),
      workerSignals.host.persistence.value().revision,
    ].join(":"));

    assert.equal(hostLabel(), "2:online:visible:1024:10:1");
    viewportSource.set({ width: 640, height: 480 });
    await waitForValue(() => hostLabel(), "2:online:visible:640:10:1");
    clockSource.set(20);
    await waitForValue(() => hostLabel(), "2:online:visible:640:20:1");
    persistenceSource.set({ revision: 3 });
    await workerSignals.host.persistence.commit();
    await waitForValue(() => hostLabel(), "2:online:visible:640:20:3");
    await count.set(5);
    assert.equal(hostLabel(), "5:online:visible:640:20:3");

    const report = workerSignals.diagnostics().hostCapabilityReport();
    assert.equal(report.callbackHostDependencies.totals.dependentCallbackCount, 1);
    assert.equal(report.callbackHostDependencies.totals.distinctDependencyCount, 5);
    assert.match(report.callbackHostDependencies.digest, /^f1a-[0-9a-f]{8}$/);
    assert.match(report.callbackHostDependencies.dependencyDigest, /^f1a-[0-9a-f]{8}$/);
    assert.deepEqual(
      report.callbackHostDependencies.dependencies.map((dependency) => dependency.family).sort(),
      ["clock", "online", "persistence", "viewport", "visibility"],
    );
    assert.equal(
      report.callbackHostReadCertification.artifactFamily,
      "CallbackHostReadDependencyAdmission",
    );
    assert.equal(
      report.callbackHostReadCertification.perReadHostRpcCount,
      0,
    );
    assert.equal(
      report.callbackHostReadCertification.callbackHostReadDependencyDigest,
      report.callbackHostDependencies.digest,
    );
    assert.equal(
      report.callbackHostReadCertification.hostCapabilityIngressDigest,
      report.lineageDigest,
    );
    assert.equal(
      report.callbackHostReadCertification.callbackRecomputationDigest,
      report.callbackHostDependencies.callbackDigest,
    );
    assert.equal(report.callbackHostReadCertification.workerOwnedDependencyEdgeCount, 5);
    assert.equal(report.boundaryPerformanceEnvelope.callbackHostDependencyEdgeCount, 5);
    assert.equal(
      report.boundaryPerformanceEnvelope.hostDependencyRefreshCount,
      report.totals.dependencyRefreshCount,
    );
    assert.equal(report.totals.dependencyRefreshCount, 3);
    assert.equal(report.totals.invalidationCount, 3);
  } finally {
    workerSignals?.free();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

test("worker-first callback host reads reject foreign runtime host handles before publication", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const {
    createSignals,
    hostCapabilityPlan,
    onlineCapability,
    cleanup,
  } = await loadSignalsModule({ rawSurface: "real" });
  let primarySignals = null;
  let foreignSignals = null;
  try {
    primarySignals = await createSignals({
      hostCapabilities: hostCapabilityPlan({
        online: onlineCapability({ source: createSubscribableSource("online").source }),
      }),
    });
    foreignSignals = await createSignals({
      hostCapabilities: hostCapabilityPlan({
        online: onlineCapability({ source: createSubscribableSource("online").source }),
      }),
    });
    const denial = captureThrownError(
      () => primarySignals.computed(() => foreignSignals.host.online.state()),
    );
    assert.match(denial.message, /different Signals runtime/);
    assert.equal(denial.code, "computeCallbackForeignRuntimeReadDenied");
    const report = primarySignals.diagnostics().hostCapabilityReport();
    assert.deepEqual(report.callbackHostDependencies.totals, {
      callbackCount: 0,
      dependentCallbackCount: 0,
      dependencyEdgeCount: 0,
      distinctDependencyCount: 0,
    });
    assert.equal(report.callbackHostReadCertification.workerOwnedDependencyEdgeCount, 0);
    assert.equal(report.callbackHostReadCertification.perReadHostRpcCount, 0);
    assert.deepEqual(report.callbackHostReadCertification.ambientHostReadDenialArtifact, {
      deniedBeforePublication: true,
      errorCode: denial.code,
    });
    assert.match(
      report.callbackHostReadCertification.callbackHostReadDependencyDigest,
      /^f1a-[0-9a-f]{8}$/,
    );
  } finally {
    primarySignals?.free();
    foreignSignals?.free();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

test("worker-first callback host reads reject missing and unsupported host capabilities", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, hostCapabilityPlan, cleanup } =
    await loadSignalsModule({ rawSurface: "real" });
  let workerSignals = null;
  try {
    assert.throws(
      () => hostCapabilityPlan({ geolocation: {} }),
      /does not support capability families: geolocation/,
    );
    workerSignals = await createSignals({ hostCapabilities: hostCapabilityPlan({}) });
    const denial = captureThrownError(
      () => workerSignals.computed(() => workerSignals.host.online.state()),
    );
    assert.equal(denial.code, "computeCallbackMissingHostCapabilityReadDenied");
    assert.match(denial.message, /online/);
    const report = workerSignals.diagnostics().hostCapabilityReport();
    assert.equal(report.callbackHostDependencies.totals.dependencyEdgeCount, 0);
    assert.equal(report.callbackHostReadCertification.workerOwnedDependencyEdgeCount, 0);
    assert.deepEqual(report.callbackHostReadCertification.ambientHostReadDenialArtifact, {
      compatibility: "Unavailable",
      denialReason: "missing-host-capability",
      deniedBeforePublication: true,
      errorCode: denial.code,
      family: "online",
      registrationId: "online",
    });
    assert.equal(report.totals.unavailabilityArtifactCount, 1);
    assert.equal(report.totals.readDenialCount, 1);
  } finally {
    workerSignals?.free();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

test("worker-first and compatibility host-read callbacks converge under the same host ingress sequence", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const loaded = await loadSignalsModule({ rawSurface: "real" });
  let workerSignals = null;
  let compatibilitySignals = null;
  try {
    const workerHost = createHostSources();
    const compatibilityHost = createHostSources();
    workerSignals = await createSignalsWithHostPlan(loaded, workerHost);
    compatibilitySignals = await createSignalsWithHostPlan(
      loaded,
      compatibilityHost,
      "mainThreadCompatibility",
    );
    const workerLabel = createHostLabel(workerSignals);
    const compatibilityLabel = createHostLabel(compatibilitySignals);

    assert.equal(workerLabel(), compatibilityLabel());
    applyHostSequence(workerHost);
    applyHostSequence(compatibilityHost);
    await workerSignals.host.persistence.commit();
    await compatibilitySignals.host.persistence.commit();
    await waitForValue(() => workerLabel(), "online:hidden:900:12:7");
    await waitForValue(() => compatibilityLabel(), "online:hidden:900:12:7");

    const workerReport = workerSignals.diagnostics().hostCapabilityReport();
    const compatibilityReport = compatibilitySignals.diagnostics().hostCapabilityReport();
    assert.equal(workerReport.boundaryPerformanceEnvelope.perReadHostRpcCount, 0);
    assert.equal(workerReport.callbackHostReadCertification.perReadHostRpcCount, 0);
    assert.equal(
      workerReport.boundaryPerformanceEnvelope.hostDependencyRefreshCount,
      workerReport.totals.dependencyRefreshCount,
    );
    assert.equal(workerReport.callbackHostDependencies.totals.distinctDependencyCount, 5);
    assert.match(workerReport.boundaryPerformanceEnvelope.digest, /^f1a-[0-9a-f]{8}$/);
    assert.match(workerReport.callbackHostReadCertification.digest, /^f1a-[0-9a-f]{8}$/);
    assert.equal(
      workerReport.callbackHostReadCertification.boundaryPerformanceEnvelopeDigest,
      workerReport.boundaryPerformanceEnvelope.digest,
    );
    assert.equal(
      digestStableValue(workerLabel()),
      digestStableValue(compatibilityLabel()),
    );
    const verificationPackage = buildCallbackHostReadVerificationPackage({
      workerReport,
      compatibilityReport,
      workerTruth: workerLabel(),
      compatibilityTruth: compatibilityLabel(),
    });
    assert.equal(
      verificationPackage.artifactFamily,
      "CallbackHostReadDependencyVerificationPackage",
    );
    assert.equal(verificationPackage.digest, digestStableValue(verificationPackage.digests));
    assert.deepEqual(
      Object.keys(verificationPackage.digests).sort(),
      [
        "ambientHostReadDenial",
        "boundaryPerformance",
        "callbackHostReadDependency",
        "callbackRecomputation",
        "compatibilityTruth",
        "hostCapabilityIngress",
        "workerFirstTruth",
      ],
    );
    assert.equal(
      verificationPackage.digests.workerFirstTruth,
      verificationPackage.digests.compatibilityTruth,
    );
    assert.equal(
      workerReport.callbackHostDependencies.digest,
      workerSignals.diagnostics().hostCapabilityReport().callbackHostDependencies.digest,
    );
    assert.match(compatibilityReport.digest, /^f1a-[0-9a-f]{8}$/);
  } finally {
    workerSignals?.free();
    compatibilitySignals?.free();
    await loaded.cleanup();
    globalThis.Worker = previousWorker;
  }
});

function createSubscribableSource(initialValue) {
  let currentValue = initialValue;
  const listeners = new Set();
  return {
    source: {
      current() {
        return currentValue;
      },
      subscribe(listener) {
        listeners.add(listener);
        return () => listeners.delete(listener);
      },
    },
    set(nextValue) {
      currentValue = nextValue;
      for (const listener of listeners) listener();
    },
  };
}

function createHostSources() {
  return {
    online: createSubscribableSource("offline"),
    visibility: createSubscribableSource("visible"),
    viewport: createSubscribableSource({ width: 800, height: 600 }),
    clock: createMutablePersistenceSource(10),
    persistence: createMutablePersistenceSource({ revision: 1 }),
  };
}

function createSignalsWithHostPlan(loaded, host, deployment = "workerFirst") {
  return loaded.createSignals({
    deployment,
    hostCapabilities: loaded.hostCapabilityPlan({
      online: loaded.onlineCapability({ source: host.online.source }),
      visibility: loaded.visibilityCapability({ source: host.visibility.source }),
      viewport: loaded.viewportCapability({ source: host.viewport.source }),
      clock: loaded.clockCapability({ source: host.clock, pollMs: 10 }),
      persistence: loaded.persistenceCapability({ source: host.persistence }),
    }),
  });
}

function createHostLabel(signals) {
  return signals.computed(() => [
    signals.host.online.state(),
    signals.host.visibility.state(),
    signals.host.viewport.width(),
    signals.host.clock.now(),
    signals.host.persistence.value().revision,
  ].join(":"));
}

function applyHostSequence(host) {
  host.online.set("online");
  host.visibility.set("hidden");
  host.viewport.set({ width: 900, height: 700 });
  host.clock.set(12);
  host.persistence.set({ revision: 7 });
}

function digestStableValue(value) {
  let hash = 2166136261;
  const input = JSON.stringify(value);
  for (let index = 0; index < input.length; index += 1) {
    hash ^= input.charCodeAt(index);
    hash = Math.imul(hash, 16777619);
  }
  return `f1a-${(hash >>> 0).toString(16).padStart(8, "0")}`;
}

function buildCallbackHostReadVerificationPackage(evidence) {
  const digests = {
    callbackHostReadDependency:
      evidence.workerReport.callbackHostReadCertification.callbackHostReadDependencyDigest,
    hostCapabilityIngress:
      evidence.workerReport.callbackHostReadCertification.hostCapabilityIngressDigest,
    callbackRecomputation:
      evidence.workerReport.callbackHostReadCertification.callbackRecomputationDigest,
    ambientHostReadDenial: digestStableValue(
      evidence.workerReport.callbackHostReadCertification.ambientHostReadDenialArtifact,
    ),
    boundaryPerformance:
      evidence.workerReport.callbackHostReadCertification.boundaryPerformanceEnvelopeDigest,
    workerFirstTruth: digestStableValue(evidence.workerTruth),
    compatibilityTruth: digestStableValue(evidence.compatibilityTruth),
  };
  return {
    artifactFamily: "CallbackHostReadDependencyVerificationPackage",
    digests,
    workerReportDigest: evidence.workerReport.digest,
    compatibilityReportDigest: evidence.compatibilityReport.digest,
    digest: digestStableValue(digests),
  };
}

function captureThrownError(action) {
  try {
    action();
  } catch (error) {
    return error;
  }
  assert.fail("expected action to throw");
}

function createMutablePersistenceSource(initialValue) {
  let currentValue = initialValue;
  return {
    current() {
      return currentValue;
    },
    set(nextValue) {
      currentValue = nextValue;
    },
  };
}

async function waitForValue(read, expected) {
  for (let attempt = 0; attempt < 100; attempt += 1) {
    if (read() === expected) return;
    await new Promise((resolve) => setTimeout(resolve, 10));
  }
  assert.equal(read(), expected);
}
