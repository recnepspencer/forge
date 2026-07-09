import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../../module_loading/load_signals_module.mjs";

test("phase-10 cross-tab and external navigation coherence stays explicit at the browser authority boundary", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const signals = await createSignals({ deployment: "mainThreadCompatibility" });
  const routes = signals.router.define({
    home: signals.router.route("/"),
    settings: signals.router.route("/settings"),
  });
  const story = signals.router.browserHistory.story();

  try {
    const sameTab = signals.router.browserHistory.coherence.sameTab({
      channelId: "workspace-main",
    });
    const crossTab = signals.router.browserHistory.coherence.crossTab("workspace-main", {
      sourceTabId: "tab-b",
      expectedRouteId: "home",
    });
    const externalNavigation = signals.router.browserHistory.coherence.externalNavigation({
      channelId: "workspace-main",
    });

    const homeWriteback = await routes.applyBrowserHistoryWriteback(
      signals.router.browserHistory.writeback.push("/", {
        routeIdentity: "home",
        coherence: sameTab,
      }),
    );
    const crossTabDrift = await routes.admitBrowserHistoryIngress(
      signals.router.browserHistory.external("/settings", {
        routeIdentity: "home",
        coherence: crossTab,
      }),
    );
    const externalMiss = await routes.admitBrowserHistoryIngress(
      signals.router.browserHistory.external("/missing", {
        routeIdentity: "settings",
        coherence: externalNavigation,
      }),
    );

    assert.equal(homeWriteback.diagnostics().coherenceKind, "sameTab");
    assert.equal(homeWriteback.diagnostics().boundaryArtifact, "routeTruthConverged");
    assert.equal(crossTabDrift.diagnostics().coherenceKind, "crossTab");
    assert.equal(crossTabDrift.diagnostics().boundaryArtifact, "routeTruthDriftedFromAuthority");
    assert.equal(externalMiss.diagnostics().coherenceKind, "externalNavigation");
    assert.equal(externalMiss.diagnostics().boundaryArtifact, "routeOutcomeNotAdmitted");
    assert.equal(
      crossTabDrift.coherence()?.verification().browserAuthorityCoherenceDigest,
      crossTab.verification().browserAuthorityCoherenceDigest,
    );

    story.record(homeWriteback);
    story.record(crossTabDrift);
    story.record(externalMiss);

    const inspection = story.inspection();
    assert.equal(inspection.latestBoundaryEvent?.coherenceKind, "externalNavigation");
    assert.equal(inspection.currentRouteTruthEvent?.coherenceKind, "crossTab");
    assert.equal(inspection.currentEntry?.coherenceKind, "crossTab");
    assert.equal(inspection.backProvenance.previous?.coherenceKind, "sameTab");
    assert.equal(inspection.summary().latestBoundaryCoherenceKind, "externalNavigation");
    assert.equal(inspection.summary().currentRouteTruthCoherenceKind, "crossTab");
    assert.equal(inspection.summary().sameTabCoherencePresent, true);
    assert.equal(inspection.summary().crossTabCoherencePresent, true);
    assert.equal(inspection.summary().externalNavigationCoherencePresent, true);
    assert.ok(inspection.summary().driftedBoundaryEventCount >= 1);
    assert.ok(inspection.summary().notAdmittedBoundaryEventCount >= 1);
  } finally {
    signals.free();
    await cleanup();
  }
});
