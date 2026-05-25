import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../../module_loading/load_signals_module.mjs";

test("phase-10 hydration handoff reports matched and drifted server route truth explicitly", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const signals = await createSignals({ deployment: "mainThreadCompatibility" });
  const routes = signals.router.define({
    home: signals.router.route("/"),
    detail: signals.router.route("/detail"),
  });

  try {
    const matched = await routes.admitHydrationHandoff(
      signals.router.hydration.server("/detail", {
        serverRouteIdentity: "detail",
        serverHref: "/detail",
      }),
    );
    assert.equal(matched.outcome().kind, "admitted");
    assert.equal(matched.diagnostics().boundarySource, "hydrationHandoff");
    assert.equal(matched.diagnostics().boundaryArtifact, "routeTruthMatchedServer");
    assert.equal(matched.diagnostics().routeId, "detail");
    assert.equal(matched.diagnostics().href, "/detail");

    const drifted = await routes.admitHydrationHandoff(
      signals.router.hydration.server("/detail", {
        serverRouteIdentity: "home",
        serverHref: "/",
      }),
    );
    assert.equal(drifted.outcome().kind, "admitted");
    assert.equal(drifted.diagnostics().boundaryArtifact, "routeTruthDriftedFromServer");
    assert.equal(typeof drifted.verification().hydrationHandoffDigest, "string");
    assert.equal(typeof drifted.verification().routeTruthDigest, "string");
    assert.equal(typeof drifted.verification().hydrationBoundaryDigest, "string");
  } finally {
    signals.free();
    await cleanup();
  }
});

test("phase-10 hydration handoff fails closed when client admission does not reproduce admitted route truth", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const signals = await createSignals({ deployment: "mainThreadCompatibility" });
  const routes = signals.router.define({
    home: signals.router.route("/"),
    detail: signals.router.route("/detail"),
  });

  try {
    const notAdmitted = await routes.admitHydrationHandoff(
      signals.router.hydration.server("/missing", {
        serverRouteIdentity: "detail",
        serverHref: "/detail",
      }),
    );
    assert.equal(notAdmitted.outcome().kind, "notFound");
    assert.equal(notAdmitted.diagnostics().boundaryArtifact, "routeOutcomeNotAdmitted");
    assert.equal(notAdmitted.diagnostics().routeId, null);
    assert.equal(notAdmitted.diagnostics().href, "/missing");
  } finally {
    signals.free();
    await cleanup();
  }
});
