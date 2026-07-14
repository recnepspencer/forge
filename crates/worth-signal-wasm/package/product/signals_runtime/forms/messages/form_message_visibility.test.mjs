import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../../runtime_fixture/graph_operational_runtime.mjs";

test("signals.form message visibility is first-class and stays outside semantic truth", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
    });

    const before = form.verification();
    const artifact = form.reportMessages({
      status: "settling",
      reason: "save toast is still visible",
      channel: "toast",
      audience: "user",
      visibleCount: 1,
      operation: "show",
    });

    assert.equal(artifact.channel, "toast");
    assert.equal(form.messages().summary.activeChannel, "toast");
    assert.equal(form.messages().summary.externalVisibleCount, 1);
    assert.equal(form.presentationLifecycle("messages").status, "settling");
    assert.equal(form.presentationHistory().at(-1)?.laneId, "messages");
    assert.equal(form.verification().digests.semanticEqualityDigest, before.digests.semanticEqualityDigest);
    assert.notEqual(form.verification().digests.messageDigest, before.digests.messageDigest);

    form.clearMessages({ reason: "toast dismissed" });
    assert.equal(form.messages().summary.status, "ready");
    assert.equal(form.presentationLifecycle("messages").status, "ready");
    assert.equal(form.presentationHistory().at(-1)?.laneId, "messages");
  } finally {
    await cleanup();
  }
});

test("signals.form message visibility keeps undeclared counts explicit instead of inventing zero or one", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
    });

    form.reportMessages({
      status: "busy",
      reason: "save banner visible",
      channel: "banner",
    });

    assert.equal(form.messages().summary.externalVisibleCount, null);
  } finally {
    await cleanup();
  }
});

test("signals.form message visibility requires an explicit target for scoped updates", async () => {
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
      () => form.reportMessages({
        status: "busy",
        reason: "field message is visible",
        scope: "field",
      }),
      /must be a non-empty string/,
    );

    const artifact = form.reportMessages({
      status: "busy",
      reason: "field message is visible",
      scope: "field",
      target: "title",
      channel: "inline",
    });
    assert.equal(artifact.target, "title");
    assert.equal(form.messages().summary.activeTarget, "title");
  } finally {
    await cleanup();
  }
});

test("signals.form message visibility requires declared field, step, and action targets for scoped updates", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
      steps: ({ step }) => ({
        review: step("review", ["title"]),
      }),
      actions: ({ submit }) => ({
        submit: submit(),
      }),
    });

    assert.throws(
      () => form.reportMessages({
        status: "busy",
        reason: "missing field message",
        scope: "field",
        target: "missing",
      }),
      /undeclared form field/,
    );

    assert.throws(
      () => form.reportMessages({
        status: "busy",
        reason: "missing step message",
        scope: "step",
        target: "missing-step",
      }),
      /declared step/,
    );

    assert.throws(
      () => form.reportMessages({
        status: "busy",
        reason: "missing action message",
        scope: "action",
        target: "missing-action",
      }),
      /declared action/,
    );

    const artifact = form.reportMessages({
      status: "settling",
      reason: "submit toast is still visible",
      scope: "action",
      target: "submit",
      channel: "toast",
    });
    assert.equal(artifact.target, "submit");
    assert.equal(form.messages().current?.scope, "action");
    assert.equal(form.presentationHistory().at(-1)?.scope, "action");
  } finally {
    await cleanup();
  }
});

test("signals.form message visibility requires declared section and control targets for scoped updates", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
      availability: ({ section, control }) => ({
        evidenceSection: section("evidence", ["title"], ["title"], () => true),
        saveControl: control("saveButton", ["title"], () => true),
      }),
    });

    assert.throws(
      () => form.reportMessages({
        status: "busy",
        reason: "missing section message",
        scope: "section",
        target: "missing-section",
      }),
      /declared section/,
    );

    assert.throws(
      () => form.reportMessages({
        status: "busy",
        reason: "missing control message",
        scope: "control",
        target: "missing-control",
      }),
      /declared control/,
    );

    const sectionArtifact = form.reportMessages({
      status: "busy",
      reason: "evidence section banner visible",
      scope: "section",
      target: "evidence",
      channel: "banner",
    });
    assert.equal(sectionArtifact.target, "evidence");

    const controlArtifact = form.reportMessages({
      status: "settling",
      reason: "save control toast visible",
      scope: "control",
      target: "saveButton",
      channel: "toast",
    });
    assert.equal(controlArtifact.target, "saveButton");
    assert.equal(form.messages().current?.scope, "control");
  } finally {
    await cleanup();
  }
});

test("signals.form message visibility denies malformed channels and counts", async () => {
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
      () => form.reportMessages({
        status: "busy",
        reason: "bad channel",
        channel: "snackbar",
      }),
      /channel is not supported/,
    );

    assert.throws(
      () => form.reportMessages({
        status: "busy",
        reason: "bad count",
        visibleCount: -1,
      }),
      /visibleCount must be a non-negative integer/,
    );
  } finally {
    await cleanup();
  }
});
