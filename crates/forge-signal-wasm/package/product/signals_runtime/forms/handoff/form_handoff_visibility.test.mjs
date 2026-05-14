import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../../runtime_fixture/graph_operational_runtime.mjs";

test("signals.form handoff visibility is first-class across route modal and external scopes", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
      presentation: {
        handoff: { scope: "externalHandoff", settlementAcknowledgement: "required" },
      },
    });

    const routeArtifact = form.reportHandoff({
      status: "busy",
      target: "review-route",
      reason: "waiting for route handoff guard",
      scopeKind: "route",
      surfaceId: "review-route",
      operation: "handoff",
    });
    assert.equal(routeArtifact.scopeKind, "route");
    assert.equal(form.handoff().summary.scopeKind, "route");
    assert.equal(form.presentationLifecycle("handoff").scope, "externalHandoff");

    const modalArtifact = form.reportHandoff({
      status: "settling",
      target: "share-modal",
      reason: "waiting for modal acknowledgement",
      token: "handoff-1",
      scopeKind: "modal",
      surfaceId: "share-modal",
      operation: "open",
    });
    assert.equal(modalArtifact.scopeKind, "modal");
    assert.equal(form.handoff().summary.surfaceId, "share-modal");
    assert.equal(form.presentationLifecycle("handoff").status, "settling");

    const externalArtifact = form.reportHandoff({
      status: "unavailable",
      target: "native-share",
      reason: "share target is unavailable",
      token: "handoff-2",
      scopeKind: "external",
      surfaceId: "native-share",
      operation: "handoff",
      unsupportedReason: "native share capability is unavailable",
    });
    assert.equal(externalArtifact.scopeKind, "external");
    assert.equal(form.handoff().summary.unsupportedReason, "native share capability is unavailable");
    assert.equal(form.handoff().counters.externalScopeUpdates, 1);
    assert.ok(form.handoff().history.some((entry) => (
      entry.source === "handoff" &&
      entry.token === "handoff-1" &&
      entry.supersededByToken === "handoff-2"
    )));
    assert.equal(form.presentationLifecycle("handoff").status, "unavailable");
    assert.equal(form.acknowledgePresentation("handoff").resultKind, "ignored");
    assert.equal(typeof form.verification().digests.handoffDigest, "string");
    assert.equal(form.diagnostics().handoff.summary.scopeKind, "external");

    const clear = form.clearHandoff({ reason: "handoff dismissed" });
    assert.equal(clear.source, "clear");
    assert.equal(form.handoff().summary.status, "ready");
  } finally {
    await cleanup();
  }
});

test("signals.form handoff visibility denies malformed scope and operation metadata", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
    });

    assert.throws(
      () =>
        form.reportHandoff({
          status: "busy",
          target: "review-route",
          reason: "bad scope",
          scopeKind: "teleport",
          surfaceId: "review-route",
        }),
      /scope kind is not supported/,
    );

    assert.throws(
      () =>
        form.reportHandoff({
          status: "busy",
          target: "review-route",
          reason: "bad operation",
          scopeKind: "route",
          surfaceId: "review-route",
          operation: "warp",
        }),
      /operation is not supported/,
    );

    assert.throws(
      () =>
        form.reportHandoff({
          status: "busy",
          target: "review-route",
          reason: "missing surface id",
          scopeKind: "route",
        }),
      /handoff presentation surfaceId must be a non-empty string/,
    );
  } finally {
    await cleanup();
  }
});
