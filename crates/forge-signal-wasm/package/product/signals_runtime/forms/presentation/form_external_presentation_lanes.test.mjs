import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../../runtime_fixture/graph_operational_runtime.mjs";

test("signals.form external presentation lanes stay outside semantic form truth", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
      actions: ({ submit }) => ({
        submit: submit(),
      }),
      presentation: {
        collaboration: { scope: "wholeForm" },
        attachments: { scope: "section" },
        media: { scope: "modal" },
        handoff: { scope: "externalHandoff", settlementAcknowledgement: "required" },
      },
    });

    const before = form.verification();
    const readinessDigestBefore = before.digests.readinessDigest;
    const validationDigestBefore = before.digests.validationDigest;
    const dirtyDigestBefore = before.digests.semanticEqualityDigest;

    const collaboration = form.reportPresentationLane("collaboration", {
      status: "busy",
      target: "presence",
      reason: "peer review cursor is active",
    });
    const attachments = form.reportPresentationLane("attachments", {
      status: "failed",
      target: "spec.pdf",
      reason: "attachment preview generation failed",
    });
    const attachmentsReport = form.attachments();
    const handoff = form.reportPresentationLane("handoff", {
      status: "settling",
      target: "share-modal",
      reason: "waiting for modal handoff acknowledgement",
      token: "handoff-1",
    });
    const handoffReport = form.handoff();

    assert.equal(collaboration.lane, "collaboration");
    assert.equal(attachments.lane, "attachments");
    assert.equal(handoff.lane, "handoff");
    assert.equal(handoffReport.summary.scopeKind, null);
    assert.equal(handoffReport.summary.activeTarget, "share-modal");
    assert.equal(attachmentsReport.summary.status, "failed");
    assert.equal(attachmentsReport.summary.selectedCount, null);
    assert.equal(attachmentsReport.summary.stagedCount, null);
    assert.equal(form.presentationLifecycle("collaboration").status, "busy");
    assert.equal(form.presentationLifecycle("attachments").status, "failed");
    assert.equal(form.presentationLifecycle("handoff").status, "settling");
    assert.equal(form.verification().digests.readinessDigest, readinessDigestBefore);
    assert.equal(form.verification().digests.validationDigest, validationDigestBefore);
    assert.equal(form.verification().digests.semanticEqualityDigest, dirtyDigestBefore);

    const acknowledgement = form.acknowledgePresentation("handoff");
    assert.equal(acknowledgement.resultKind, "acknowledged");
    assert.equal(form.presentationLifecycle("handoff").status, "ready");

    form.reportPresentationLane("handoff", {
      status: "failed",
      target: "share-modal",
      reason: "modal handoff failed before clear",
      token: "handoff-2",
    });
    const genericHandoffClear = form.clearPresentationLane("handoff", {
      reason: "generic handoff lane cleared",
    });
    assert.equal(genericHandoffClear.lane, "handoff");
    assert.equal(form.handoff().summary.status, "ready");
    assert.equal(form.handoff().summary.activeTarget, null);
    assert.ok(form.handoff().history.some((entry) => (
      entry.source === "clear" &&
      entry.reason === "generic handoff lane cleared"
    )));

    const clear = form.clearPresentationLane("collaboration", {
      reason: "collaboration banner dismissed",
    });
    assert.equal(clear.source, "clear");
    assert.equal(form.presentationLifecycle("collaboration").status, "ready");
    assert.equal(form.presentation().counters.externalLanes, 5);
    assert.ok(form.presentationHistory().length >= 4);

    const after = form.verification();
    assert.notEqual(after.digests.presentationDigest, before.digests.presentationDigest);
    assert.equal(after.digests.readinessDigest, readinessDigestBefore);
    assert.equal(after.digests.validationDigest, validationDigestBefore);
    assert.equal(after.digests.semanticEqualityDigest, dirtyDigestBefore);
  } finally {
    await cleanup();
  }
});

test("signals.form external presentation lane updates deny unsupported lane ids and malformed statuses", async () => {
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
        form.reportPresentationLane("navigation", {
          status: "busy",
          reason: "not an external lane",
        }),
      /not a declared external lane/,
    );

    assert.throws(
      () =>
        form.reportPresentationLane("media", {
          status: "teleport",
          reason: "bad status",
        }),
      /status is not supported/,
    );
  } finally {
    await cleanup();
  }
});
