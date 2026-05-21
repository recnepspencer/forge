import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

test("createWorkerRuntimeBridge exposes host ingress, host effect, and host-bridge certification lanes", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createWorkerRuntimeBridge, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const bridge = createWorkerRuntimeBridge();
  try {
    await bridge.publishPortableGraph(mainThreadHostBridgePublication());

    const hostCapabilityReport = await bridge.admitHostCapabilityIngress({
      updates: [
        {
          family: "visibility",
          registrationId: "documentVisibility",
          semanticValueIdentity: "hidden",
          boundaryArtifact: "admitted",
          runtimeSourceId: "documentVisibility",
          runtimeValue: "hidden",
        },
        {
          family: "online",
          registrationId: "navigatorOnline",
          semanticValueIdentity: "unavailable",
          boundaryArtifact: "unavailable",
        },
      ],
    });
    const browserHistoryReport = await bridge.admitBrowserHistoryIngress({
      navigationKind: "popstate",
      rawLocation: "/search?q=forge",
      routeIdentity: "searchRoute:forge",
      runtimeRouteSourceId: "routeIdentity",
      routeValue: "searchRoute:forge",
      runtimeContinuitySourceId: "routeContinuity",
      continuityValue: "restored",
    });
    const hostEffectRequest = await bridge.issueHostEffectRequest({
      effectId: "focusSearchInput",
      hostCapabilityFamily: "domFocus",
      closedPayloadIdentity: "focusSearchInputPayload",
    });
    const hostEffectAcknowledgement = await bridge.admitHostEffectAcknowledgement({
      requestDigest: hostEffectRequest.requestDigest,
      outcome: "unavailable",
      artifactIdentity: "searchInputUnavailable",
    });
    const certification = await bridge.certifyMainThreadHostBridge();

    assert.equal(hostCapabilityReport.envelopeFamily, "hostCapabilityIngress");
    assert.equal(hostCapabilityReport.runtimeAdmittedUpdateCount, 1);
    assert.equal(browserHistoryReport.envelopeFamily, "browserHistoryIngress");
    assert.equal(browserHistoryReport.runtimeAdmittedRouteCount, 1);
    assert.equal(browserHistoryReport.runtimeAdmittedContinuityCount, 1);
    assert.equal(hostEffectRequest.envelopeFamily, "hostEffectEgress");
    assert.equal(
      hostEffectAcknowledgement.hostEffectLifecycleArtifact,
      "hostEffectUnavailable",
    );
    assert.equal(certification.certificationFamily, "mainThreadHostBridgeCertification");
    assert.equal(
      certification.hostCapabilityEnvelopeDigest,
      hostCapabilityReport.hostCapabilityEnvelopeDigest,
    );
    assert.equal(
      certification.browserHistoryReplayRestoreDigest,
      browserHistoryReport.replayRestoreDigest,
    );
    assert.equal(
      certification.hostEffectAcknowledgedRequestDigest,
      hostEffectAcknowledgement.acknowledgedRequestDigest,
    );
    assert.equal(certification.ambientHostReadDenied, true);
  } finally {
    await bridge.terminate();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

test("createWorkerRuntimeBridge delivers committed outputs from worker-owned truth", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createWorkerRuntimeBridge, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const bridge = createWorkerRuntimeBridge();
  try {
    await bridge.publishPortableGraph(counterPublicationWithOutput());
    await bridge.applyTransaction([{ kind: "set", id: "counter", value: 7 }]);

    const packet = await bridge.deliverOutputs({ outputIds: ["doubleCounter"] });

    assert.equal(packet.envelopeFamily, "outputDelivery");
    assert.equal(packet.runtimeAuthority, "workerOwnedRuntime");
    assert.equal(packet.outputDeliveryBreadth, 1);
    assert.equal(packet.outputs[0].id, "doubleCounter");
    assert.equal(packet.outputs[0].value, 14);
    assert.equal(typeof packet.packetDigest, "string");
  } finally {
    await bridge.terminate();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

function counterPublicationWithOutput() {
  return {
    policy: { preset: "development" },
    sources: [{ id: "counter", initial: 1 }],
    recipes: [
      {
        id: "doubleCounter",
        reads: ["counter"],
        expr: {
          kind: "sum",
          args: [
            { kind: "read", id: "counter" },
            { kind: "read", id: "counter" },
          ],
        },
        identity: { kind: "exact" },
      },
    ],
    outputIds: ["doubleCounter"],
  };
}

function mainThreadHostBridgePublication() {
  return {
    policy: { preset: "development" },
    sources: [
      { id: "documentVisibility", initial: "visible" },
      { id: "routeIdentity", initial: "homeRoute" },
      { id: "routeContinuity", initial: "fresh" },
      { id: "searchFocusLifecycle", initial: "pending" },
    ],
    recipes: [
      {
        id: "routeProjection",
        reads: ["routeIdentity"],
        expr: { kind: "read", id: "routeIdentity" },
        identity: { kind: "exact" },
      },
      {
        id: "searchFocusProjection",
        reads: ["searchFocusLifecycle"],
        expr: { kind: "read", id: "searchFocusLifecycle" },
        identity: { kind: "exact" },
      },
    ],
  };
}
