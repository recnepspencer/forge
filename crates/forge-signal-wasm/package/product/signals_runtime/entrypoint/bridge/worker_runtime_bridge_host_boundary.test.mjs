import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

test("createWorkerRuntimeBridge exposes host ingress, host effect, and host-bridge certification lanes", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const {
    createWorkerRuntimeBridge,
    importProductModule,
    cleanup,
  } = await loadSignalsModule({ rawSurface: "real" });
  const { createRouterNamespace } = await importProductModule("router/router_namespace.js");
  const bridge = createWorkerRuntimeBridge();
  const workerRouter = createRouterNamespace();
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
    const browserHistoryReport = await bridge.admitBrowserHistoryIngress(
      workerRouter.browserHistory.pop("/search?q=forge", {
        routeIdentity: "searchRoute:forge",
        runtimeRouteSourceId: "routeIdentity",
        routeValue: "searchRoute:forge",
        runtimeContinuitySourceId: "routeContinuity",
        continuityValue: "restored",
        coherence: workerRouter.browserHistory.coherence.sameTab({
          channelId: "workspace-main",
        }),
      }),
    );
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
    assert.equal(browserHistoryReport.rawLocationHref, "/search?q=forge");
    assert.equal(browserHistoryReport.routeIdentity, "searchRoute:forge");
    assert.equal(browserHistoryReport.runtimeAdmittedRouteCount, 1);
    assert.equal(browserHistoryReport.runtimeAdmittedContinuityCount, 1);
    assert.equal(browserHistoryReport.outcome().kind, "admitted");
    assert.equal(browserHistoryReport.outcome().routeIdentity, "searchRoute:forge");
    assert.equal(browserHistoryReport.diagnostics().boundarySource, "browserHistoryIngress");
    assert.equal(browserHistoryReport.diagnostics().coherenceKind, "sameTab");
    assert.equal(browserHistoryReport.diagnostics().routeId, "searchRoute:forge");
    assert.equal(
      browserHistoryReport.verification().browserHistoryEnvelopeDigest,
      browserHistoryReport.browserHistoryEnvelopeDigest,
    );
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

test("createWorkerRuntimeBridge applies browser-history writeback and tracks one worker-side boundary story", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const {
    createWorkerRuntimeBridge,
    importProductModule,
    cleanup,
  } = await loadSignalsModule({ rawSurface: "real" });
  const { createRouterNamespace } = await importProductModule("router/router_namespace.js");
  const bridge = createWorkerRuntimeBridge();
  const workerRouter = createRouterNamespace();
  try {
    await bridge.publishPortableGraph(mainThreadHostBridgePublication());

    const homeIngress = await bridge.admitBrowserHistoryIngress(
      workerRouter.browserHistory.load("/", {
        routeIdentity: "homeRoute",
        runtimeRouteSourceId: "routeIdentity",
        routeValue: "homeRoute",
      }),
    );
    const localWriteback = await bridge.applyBrowserHistoryWriteback(
      workerRouter.browserHistory.writeback.push("/search?q=forge", {
        routeIdentity: "searchRoute:forge",
        runtimeRouteSourceId: "routeIdentity",
        routeValue: "searchRoute:forge",
        runtimeContinuitySourceId: "routeContinuity",
        continuityValue: "restored",
      }),
    );
    const routeProjection = await bridge.readSignals({ signalIds: ["routeProjection"] });
    const externalTruthBefore = await bridge.readDiagnosticsSummary();
    const externalWriteback = await bridge.applyBrowserHistoryWriteback(
      workerRouter.browserHistory.writeback.external("https://example.com/docs/router"),
    );
    const externalTruthAfter = await bridge.readDiagnosticsSummary();
    const settingsIngress = await bridge.admitBrowserHistoryIngress(
      workerRouter.browserHistory.push("/settings", {
        routeIdentity: "settingsRoute",
        runtimeRouteSourceId: "routeIdentity",
        routeValue: "settingsRoute",
      }),
    );
    const crossTabDrift = await bridge.admitBrowserHistoryIngress(
      workerRouter.browserHistory.external("/settings", {
        routeIdentity: "settingsRoute",
        runtimeRouteSourceId: "routeIdentity",
        routeValue: "settingsRoute",
        coherence: workerRouter.browserHistory.coherence.crossTab("workspace-main", {
          sourceTabId: "tab-b",
          expectedRouteId: "homeRoute",
        }),
      }),
    );
    const externalMiss = await bridge.admitBrowserHistoryIngress(
      workerRouter.browserHistory.external("/missing", {
        routeIdentity: "settingsRoute",
        coherence: workerRouter.browserHistory.coherence.externalNavigation({
          channelId: "workspace-main",
        }),
      }),
    );
    const story = bridge.browserHistoryStory(homeIngress);

    const searchEvent = story.record(localWriteback);
    const externalEvent = story.record(externalWriteback);
    const settingsEvent = story.record(settingsIngress);
    const crossTabEvent = story.record(crossTabDrift);
    const externalMissEvent = story.record(externalMiss);

    assert.equal(localWriteback.envelopeFamily, "browserHistoryWriteback");
    assert.equal(localWriteback.targetKind, "local");
    assert.equal(localWriteback.targetHref, "/search?q=forge");
    assert.equal(localWriteback.outcome()?.kind, "admitted");
    assert.equal(localWriteback.outcome()?.routeIdentity, "searchRoute:forge");
    assert.equal(localWriteback.diagnostics().boundaryArtifact, "routeTruthConverged");
    assert.equal(routeProjection.signals[0].value, "searchRoute:forge");

    assert.equal(externalWriteback.targetKind, "external");
    assert.equal(externalWriteback.outcome(), null);
    assert.equal(externalWriteback.diagnostics().boundaryArtifact, "externalNavigationEscaped");
    assert.equal(
      externalTruthAfter.workerFirstTruthDigest,
      externalTruthBefore.workerFirstTruthDigest,
    );

    assert.equal(crossTabDrift.diagnostics().boundaryArtifact, "routeTruthDriftedFromAuthority");
    assert.equal(crossTabDrift.diagnostics().coherenceKind, "crossTab");
    assert.equal(externalMiss.diagnostics().boundaryArtifact, "routeOutcomeNotAdmitted");
    assert.equal(externalMiss.diagnostics().coherenceKind, "externalNavigation");

    assert.equal(searchEvent.routeTruthEntry?.routeId, "searchRoute:forge");
    assert.equal(externalEvent.routeTruthEntry, null);
    assert.equal(settingsEvent.routeTruthEntry?.routeId, "settingsRoute");
    assert.equal(crossTabEvent.routeTruthEntry?.routeId, "settingsRoute");
    assert.equal(crossTabEvent.boundaryArtifact, "routeTruthDriftedFromAuthority");
    assert.equal(externalMissEvent.routeTruthEntry, null);
    assert.equal(externalMissEvent.boundaryArtifact, "routeOutcomeNotAdmitted");
    assert.equal(story.current()?.routeId, "settingsRoute");
    assert.equal(story.back()?.routeId, "settingsRoute");
    assert.equal(story.latestBoundaryEvent()?.boundaryArtifact, "routeOutcomeNotAdmitted");
    assert.equal(story.currentRouteTruthEvent()?.routeTruthEntry?.routeId, "settingsRoute");
    assert.equal(story.currentRouteTruthEvent()?.coherenceKind, "crossTab");
    const replayHistory = {
      replay_for(id) {
        return { id, family: "replay", frames: [{ id }] };
      },
    };
    assert.deepEqual(
      story.breadcrumbTrail().entries.map((entry) => ({
        crumbId: entry.crumbId,
        label: entry.label,
        status: entry.status,
        sourceKind: entry.sourceKind,
        replayAvailability: entry.provenance().replayAvailability,
      })),
      [
        {
          crumbId: "history:homeRoute",
          label: "homeRoute",
          status: "fallback",
          sourceKind: "historyFallback",
          replayAvailability: "replayHistory",
        },
        {
          crumbId: "history:searchRoute:forge",
          label: "searchRoute:forge",
          status: "fallback",
          sourceKind: "historyFallback",
          replayAvailability: "replayHistory",
        },
        {
          crumbId: "history:settingsRoute",
          label: "settingsRoute",
          status: "fallback",
          sourceKind: "historyFallback",
          replayAvailability: "replayHistory",
        },
        {
          crumbId: "history:settingsRoute",
          label: "settingsRoute",
          status: "fallback",
          sourceKind: "historyFallback",
          replayAvailability: "replayHistory",
        },
      ],
    );
    assert.deepEqual(
      story.current()?.replay(replayHistory).routeReplay,
      { id: "routeIdentity", family: "replay", frames: [{ id: "routeIdentity" }] },
    );
    assert.deepEqual(
      story.breadcrumbTrail().entries[0].replay(replayHistory).routeReplay,
      { id: "routeIdentity", family: "replay", frames: [{ id: "routeIdentity" }] },
    );
    assert.deepEqual(
      story.breadcrumbTrail().entries[1].replay(replayHistory).continuityReplay,
      { id: "routeContinuity", family: "replay", frames: [{ id: "routeContinuity" }] },
    );
    assert.deepEqual(story.inspection().summary(), {
      currentEntryAvailable: true,
      currentEntryRestoreAvailability: "unavailable",
      currentEntryReplayAvailability: "replayHistory",
      backProvenanceAvailable: true,
      backRestoreAvailability: "unavailable",
      backReplayAvailability: "replayHistory",
      currentOutletCompositionAvailable: false,
      backOutletCompositionAvailable: false,
      breadcrumbEntryCount: 4,
      breadcrumbRestoreAvailability: "none",
      breadcrumbReplayAvailability: "all",
      resolvedBreadcrumbCount: 0,
      recomputedBreadcrumbCount: 0,
      carriedBreadcrumbCount: 0,
      restoredBreadcrumbCount: 0,
      fallbackBreadcrumbCount: 4,
      routeDeclarationBreadcrumbPresent: false,
      recomputedBreadcrumbPresent: false,
      carriedBreadcrumbPresent: false,
      restoredBreadcrumbPresent: false,
      fallbackBreadcrumbPresent: false,
      historyFallbackBreadcrumbPresent: true,
      latestBoundaryCoherenceKind: "externalNavigation",
      currentRouteTruthCoherenceKind: "crossTab",
      sameTabCoherencePresent: false,
      crossTabCoherencePresent: true,
      externalNavigationCoherencePresent: true,
      convergedBoundaryEventCount: 1,
      driftedBoundaryEventCount: 1,
      notAdmittedBoundaryEventCount: 1,
    });
    assert.deepEqual(story.auditability().summary(), {
      hydrationBoundaryPresent: false,
      hydrationBoundaryArtifact: null,
      hydrationMatchesCurrentVisibleRoute: null,
      historyCurrentEntryPresent: true,
      currentVisibleRouteSource: "routeHistoryEntry",
      currentVisibilityExplanation: "routeHistoryEntry",
      currentBoundarySource: "browserHistoryIngress",
      currentBoundaryArtifact: "routeTruthDriftedFromAuthority",
      currentNavigationIntent: "external",
      currentCoherenceKind: "crossTab",
      currentRouteId: "settingsRoute",
      currentHref: "/settings",
      currentRestoreAvailability: "unavailable",
      currentReplayAvailability: "replayHistory",
      routeHistoryExplainsCurrent: true,
      restoreBoundaryExplainsCurrent: false,
      latestBoundarySource: "browserHistoryIngress",
      latestBoundaryArtifact: "routeOutcomeNotAdmitted",
      latestBoundaryCoherenceKind: "externalNavigation",
      sameTabCoherencePresent: false,
      crossTabCoherencePresent: true,
      externalNavigationCoherencePresent: true,
      convergedBoundaryEventCount: 1,
      driftedBoundaryEventCount: 1,
      notAdmittedBoundaryEventCount: 1,
    });
    assert.deepEqual(
      story.breadcrumbs().map((entry) => entry.routeId),
      ["homeRoute", "searchRoute:forge", "settingsRoute", "settingsRoute"],
    );
    assert.equal(story.current()?.outletComposition(), null);
    assert.match(story.verification().historyStoryDigest, /browser-history-story/);

    await assert.rejects(
      () => bridge.applyBrowserHistoryWriteback({
        navigationKind: "pushstate",
        targetKind: "local",
        targetHref: "/search?q=forge",
      }),
      /requires routeIdentity for local graph-issued writeback/,
    );
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
