import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../../runtime_fixture/graph_operational_runtime.mjs";

test("signals.form attachment visibility is first-class and stays outside semantic truth", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
      presentation: {
        attachments: { scope: "section" },
      },
    });

    const before = form.verification();
    const artifact = form.reportAttachments({
      status: "failed",
      section: "evidence",
      target: "spec.pdf",
      reason: "preview generation failed",
      selectedCount: 3,
      stagedCount: 2,
      failedCount: 1,
      operation: "preview",
    });

    assert.equal(artifact.section, "evidence");
    assert.equal(form.attachments().summary.failedCount, 1);
    assert.equal(form.attachments().summary.selectedCount, 3);
    assert.equal(form.presentationLifecycle("attachments").status, "failed");
    assert.equal(form.presentationLifecycle("attachments").scope, "section");
    assert.equal(form.verification().digests.semanticEqualityDigest, before.digests.semanticEqualityDigest);
    assert.notEqual(form.verification().digests.attachmentDigest, before.digests.attachmentDigest);

    const cleared = form.clearAttachments({ reason: "attachment warning dismissed" });
    assert.equal(cleared.operation, "clear");
    assert.equal(form.attachments().summary.status, "ready");
    assert.equal(form.presentationLifecycle("attachments").status, "ready");
  } finally {
    await cleanup();
  }
});

test("signals.form attachment visibility denies malformed counts and operations", async () => {
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
      () => form.reportAttachments({
        status: "busy",
        reason: "bad counts",
        selectedCount: -1,
      }),
      /selectedCount must be a non-negative integer/,
    );

    assert.throws(
      () => form.reportAttachments({
        status: "busy",
        reason: "bad op",
        operation: "teleport",
      }),
      /operation is not supported/,
    );
  } finally {
    await cleanup();
  }
});
