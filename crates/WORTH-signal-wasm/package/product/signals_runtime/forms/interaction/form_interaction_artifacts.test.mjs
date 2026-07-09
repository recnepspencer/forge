import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../../runtime_fixture/graph_operational_runtime.mjs";

test("signals.form exposes replay-honest touched visited and focus interaction facts without mutating semantic truth", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const state = {
      focusedField: "title",
    };
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "Ship docs", approved: false },
      host: {
        focus: () => state.focusedField,
      },
      fields: ({ field }) => ({
        title: field("title"),
        approved: field("approved"),
      }),
    });

    const beforeInteraction = form.verification();
    assert.equal(form.interaction().summary.focusedField, "title");
    assert.equal(form.interaction().summary.touchedFields, 0);
    assert.equal(form.interaction().summary.visitedFields, 0);

    form.fields.title.touch();
    form.fields.approved.visit();

    const interaction = form.interaction();
    assert.equal(interaction.summary.touchedFields, 1);
    assert.equal(interaction.summary.visitedFields, 1);
    assert.equal(interaction.fields.find((entry) => entry.field === "title").touched, true);
    assert.equal(interaction.fields.find((entry) => entry.field === "approved").visited, true);
    assert.equal(interaction.history.length, 2);
    assert.equal(form.dirty().isDirty, false);

    state.focusedField = "approved";
    assert.equal(form.interaction().summary.focusedField, "approved");
    assert.equal(form.fields.approved.diagnostics().interaction.focused, true);

    const afterInteraction = form.verification();
    assert.notEqual(afterInteraction.digests.interactionDigest, beforeInteraction.digests.interactionDigest);
    assert.notEqual(afterInteraction.digests.interactionHistoryDigest, beforeInteraction.digests.interactionHistoryDigest);
    assert.equal(afterInteraction.interactionHistory.operations, 2);
    assert.equal(afterInteraction.performanceEnvelope.interaction.focusedFields, 1);
    assert.equal(afterInteraction.performanceEnvelope.interactionOperations, 2);
    assert.equal(form.diagnostics().interaction.summary.focusedField, "approved");
  } finally {
    await cleanup();
  }
});

test("signals.form interaction presentation stays explicit when host focus support is unavailable", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title", {
          inputAdapter: {
            reportsFocus: false,
          },
        }),
      }),
    });

    const interaction = form.interaction();
    form.fields.title.touch();
    assert.equal(interaction.summary.focusPosture, "unavailable");
    assert.equal(form.presentationLifecycle("interaction").status, "ready");
    assert.match(form.presentationLifecycle("interaction").reason, /without focus support/i);
    assert.equal(form.presentationLifecycle("interaction").target, "title");
  } finally {
    await cleanup();
  }
});

test("signals.form interaction report carries input provenance composition focus intent and submit intent without mutating semantic truth", async () => {
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
    form.fields.title.input("Ship docs draft", { source: "paste" });
    form.fields.title.compose("Ship docs draft");
    form.fields.title.focus();
    form.reportSubmitIntent({ source: "keyboard" });

    const interaction = form.interaction();
    const field = interaction.fields.find((entry) => entry.field === "title");
    assert.ok(field);
    assert.equal(field.lastInputSource, "paste");
    assert.equal(field.composing, true);
    assert.equal(field.focusIntent, true);
    assert.equal(interaction.summary.composingFields, 1);
    assert.equal(interaction.summary.focusIntentField, "title");
    assert.equal(interaction.summary.inputSources.paste, 1);
    assert.equal(interaction.summary.submitIntent.active, true);
    assert.equal(interaction.summary.submitIntent.source, "keyboard");
    assert.equal(form.dirty().isDirty, false);
    assert.equal(form.presentationLifecycle("interaction").target, "title");

    form.clearSubmitIntent({ reason: "submit deferred" });
    form.fields.title.blur();
    const after = form.interaction();
    assert.equal(after.summary.submitIntent.active, false);
    assert.equal(after.fields.find((entry) => entry.field === "title").blurred, true);
    assert.equal(after.history.some((entry) => entry.kind === "submitIntent"), true);
    assert.equal(after.counters.submitIntentArtifacts, 2);
    assert.equal(after.counters.compositionArtifacts, 1);
    assert.equal(after.counters.focusArtifacts, 2);

    const afterVerification = form.verification();
    assert.notEqual(afterVerification.digests.interactionDigest, before.digests.interactionDigest);
    assert.notEqual(afterVerification.digests.interactionHistoryDigest, before.digests.interactionHistoryDigest);
    assert.equal(afterVerification.performanceEnvelope.interaction.submitIntentArtifacts, 2);
  } finally {
    await cleanup();
  }
});

test("signals.form denies malformed interaction provenance declarations", async () => {
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
      () => form.fields.title.input("Ship docs", { source: "scanner" }),
      /input source is not supported/i,
    );
    assert.throws(
      () => form.reportSubmitIntent({ source: "voice" }),
      /submit intent source is not supported/i,
    );
  } finally {
    await cleanup();
  }
});

test("signals.form adapter interaction ingress lowers into the same interaction artifacts as field handles", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
    });

    form.reportFieldInteraction("title", {
      kind: "input",
      source: "drop",
      rawValue: "Dropped title",
    });
    form.reportFieldInteraction("title", {
      kind: "focus",
      source: "adapter",
    });
    form.reportFieldInteraction("title", {
      kind: "compositionStart",
      rawValue: "Dropped title",
    });

    const interaction = form.interaction();
    const field = interaction.fields.find((entry) => entry.field === "title");
    assert.ok(field);
    assert.equal(field.lastInputSource, "drop");
    assert.equal(field.focusIntent, true);
    assert.equal(field.composing, true);
    assert.equal(interaction.summary.inputSources.drop, 1);
    assert.equal(interaction.summary.focusIntentField, "title");
    assert.equal(interaction.counters.inputArtifacts, 1);
    assert.equal(interaction.counters.compositionArtifacts, 1);
    assert.equal(interaction.counters.focusArtifacts, 1);
  } finally {
    await cleanup();
  }
});

test("signals.form bindInput gives common control integrations a task-shaped event bridge", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { query: "" },
      fields: ({ field }) => ({
        query: field("query", {
          input: {
            adapter: {
              tier: "externalImperative",
            },
            parse: (raw) => raw.trim(),
          },
        }),
      }),
    });

    const query = form.bindInput("query", { source: "typing" });
    query.input("  ship docs  ");
    query.focus();
    query.commit();
    query.blur();

    assert.equal(form.effective().query, "ship docs");
    assert.equal(form.interaction().fields.find((entry) => entry.field === "query")?.blurred, true);
    assert.equal(form.fields.query.diagnostics().inputAdapter.tier, "externalImperative");
  } finally {
    await cleanup();
  }
});

test("signals.form interaction report keeps raw-input and composition capability unavailability explicit", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title", {
          inputAdapter: {
            reportsRawInput: false,
            reportsComposition: false,
            reportsFocus: false,
          },
        }),
      }),
    });

    const interaction = form.interaction();
    const field = interaction.fields.find((entry) => entry.field === "title");
    assert.ok(field);
    assert.equal(field.rawInputPosture.posture, "unavailable");
    assert.equal(field.compositionPosture.posture, "unavailable");
    assert.equal(interaction.summary.rawInputUnavailableFields, 1);
    assert.equal(interaction.summary.compositionUnavailableFields, 1);
    assert.equal(interaction.counters.rawInputUnavailableFields, 1);
    assert.equal(interaction.counters.compositionUnavailableFields, 1);

    assert.throws(
      () => form.reportFieldInteraction("title", {
        kind: "input",
        source: "autofill",
        rawValue: "Ship docs",
      }),
      /raw input is unavailable/i,
    );
    assert.throws(
      () => form.reportFieldInteraction("title", {
        kind: "compositionStart",
        rawValue: "Ship docs",
      }),
      /composition is unavailable/i,
    );
    assert.throws(
      () => form.reportFieldInteraction("title", {
        kind: "focus",
        source: "adapter",
      }),
      /focus is unavailable/i,
    );
  } finally {
    await cleanup();
  }
});
