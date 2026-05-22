import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

test("worker-first detached host handles deny reads instead of returning stale facts", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const {
    createSignals,
    hostCapabilityPlan,
    onlineCapability,
    persistenceCapability,
    cleanup,
  } = await loadSignalsModule({ rawSurface: "real" });
  let workerSignals = null;
  try {
    workerSignals = await createSignals({
      hostCapabilities: hostCapabilityPlan({
        online: onlineCapability({ source: createSubscribableSource("online").source }),
        persistence: persistenceCapability({
          source: createMutablePersistenceSource({ revision: 1 }),
        }),
      }),
    });
    const host = workerSignals.host;
    const diagnostics = workerSignals.diagnostics();
    assert.equal(host.online.state(), "online");
    assert.deepEqual(host.persistence.value(), { revision: 1 });

    await workerSignals.terminate();
    const onlineDenial = captureThrownError(() => host.online.state());
    assert.equal(onlineDenial.code, "computeCallbackDetachedHostCapabilityReadDenied");
    const persistenceDenial = captureThrownError(() => host.persistence.value());
    assert.equal(
      persistenceDenial.code,
      "computeCallbackDetachedHostCapabilityReadDenied",
    );
    const commitDenial = await captureRejectedError(() => host.persistence.commit());
    assert.equal(commitDenial.code, "computeCallbackDetachedHostCapabilityReadDenied");

    const report = diagnostics.hostCapabilityReport();
    assert.equal(report.totals.unavailabilityArtifactCount, 3);
    assert.equal(report.totals.readDenialCount, 3);
    assert.equal(report.callbackHostReadCertification.workerOwnedDependencyEdgeCount, 0);
    assert.deepEqual(report.callbackHostReadCertification.ambientHostReadDenialArtifact, {
      compatibility: "ImportDenied",
      denialReason: "detached-host-capability",
      deniedBeforePublication: false,
      errorCode: commitDenial.code,
      family: "persistence",
      registrationId: "persistence",
    });
    assert.equal(report.boundaryPerformanceEnvelope.perReadHostRpcCount, 0);
  } finally {
    workerSignals?.free();
    await cleanup();
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
  };
}

function createMutablePersistenceSource(initialValue) {
  return {
    current() {
      return initialValue;
    },
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

async function captureRejectedError(action) {
  try {
    await action();
  } catch (error) {
    return error;
  }
  assert.fail("expected action to reject");
}
