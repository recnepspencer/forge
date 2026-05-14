import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../../runtime_fixture/graph_operational_runtime.mjs";

test("signals.form media visibility is first-class and carries modal session truth", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
      presentation: {
        media: { scope: "modal", settlementAcknowledgement: "required" },
      },
    });

    const before = form.verification();
    const artifact = form.reportMedia({
      status: "settling",
      target: "hero-image",
      reason: "waiting for crop modal acknowledgement",
      token: "media-1",
      mode: "crop",
      surfaceId: "cropper-modal",
      operation: "open",
    });

    assert.equal(artifact.mode, "crop");
    assert.equal(artifact.scopeKind, "modal");
    assert.equal(form.media().summary.mode, "crop");
    assert.equal(form.media().summary.scopeKind, "modal");
    assert.equal(form.media().summary.surfaceId, "cropper-modal");
    assert.equal(form.presentationLifecycle("media").status, "settling");
    assert.equal(form.presentationLifecycle("media").scope, "modal");
    assert.equal(form.verification().digests.semanticEqualityDigest, before.digests.semanticEqualityDigest);
    assert.notEqual(form.verification().digests.mediaDigest, before.digests.mediaDigest);

    form.acknowledgePresentation("media");
    assert.equal(form.presentationLifecycle("media").status, "ready");
  } finally {
    await cleanup();
  }
});

test("signals.form media visibility denies unsupported modes", async () => {
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
      () => form.reportMedia({
        status: "busy",
        reason: "bad mode",
        mode: "teleport",
        surfaceId: "cropper-modal",
      }),
      /mode is not supported/,
    );

    assert.throws(
      () => form.reportMedia({
        status: "busy",
        reason: "missing modal surface",
      }),
      /media presentation surfaceId must be a non-empty string/,
    );
  } finally {
    await cleanup();
  }
});
