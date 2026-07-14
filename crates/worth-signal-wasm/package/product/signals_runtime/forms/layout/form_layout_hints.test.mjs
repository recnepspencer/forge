import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../../runtime_fixture/graph_operational_runtime.mjs";

test("signals.form layout report derives section row column and track hints from declarations", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "", summary: "", approved: false },
      fields: ({ field }) => ({
        title: field("title", {
          label: "Title",
          layout: {
            row: "hero",
            column: "left",
            minHeight: 44,
            grow: true,
            responsive: ["mobile:stack", "desktop:two-column"],
          },
        }),
        summary: field("summary", {
          label: "Summary",
          row: "hero",
          column: "right",
          wrap: true,
        }),
        approved: field("approved", {
          label: "Approved",
          row: "review",
          column: "full",
          density: "compact",
        }),
      }),
      validation: ({ field }) => ({
        titleRequired: field("title", (value) => (
          value.trim().length === 0
            ? {
                kind: "invalid",
                message: {
                  code: "title.required",
                  severity: "error",
                  target: "title",
                  visibility: "visible",
                },
              }
            : true
        )),
      }),
      steps: ({ step }) => ({
        details: step("details", ["title", "summary"], {
          order: 2,
          density: "spacious",
          responsive: ["desktop:two-column"],
        }),
        review: step("review", ["approved"], {
          order: 1,
          alignment: "center",
        }),
      }),
    });

    const layout = form.layout();
    const titleHint = layout.fields.find((entry) => entry.field === "title");
    const summaryHint = layout.fields.find((entry) => entry.field === "summary");
    assert.ok(titleHint);
    assert.ok(summaryHint);
    assert.equal(titleHint.section, "details");
    assert.equal(titleHint.row, "hero");
    assert.equal(titleHint.column, "left");
    assert.equal(titleHint.tracks.help, "omitted");
    assert.equal(titleHint.tracks.message, "declared");
    assert.equal(titleHint.inputAdapterTier, "signalNative");
    assert.equal(titleHint.capabilities.supportsMessageTrack, true);
    assert.equal(titleHint.minHeight, 44);
    assert.equal(titleHint.grow, true);
    assert.deepEqual(titleHint.responsive, ["mobile:stack", "desktop:two-column"]);
    assert.equal(summaryHint.wrap, true);
    assert.equal(layout.rows.find((row) => row.id === "hero").maxMinHeight, 44);
    assert.deepEqual(layout.rows.find((row) => row.id === "hero").columns, ["left", "right"]);
    assert.deepEqual(layout.sections.map((section) => section.id), ["review", "details"]);
    assert.equal(layout.sections.find((section) => section.id === "details").density, "spacious");
    assert.equal(layout.summary.unavailableFields, 0);
    assert.equal(layout.counters.responsiveTokens, 3);
    assert.equal(form.layoutField("title")?.field, "title");
    assert.equal(form.diagnostics().layout.digest, layout.digest);
  } finally {
    await cleanup();
  }
});

test("signals.form layout report keeps unavailable posture explicit when adapters cannot honor layout capabilities", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "" },
      fields: ({ field }) => ({
        title: field("title", {
          minHeight: 40,
          responsive: ["mobile:stack"],
          adapter: {
            supportsMinHeightSync: false,
            supportsResponsiveTokens: false,
            supportsMessageTrack: false,
          },
        }),
      }),
      validation: ({ field }) => ({
        titleRequired: field("title", () => ({
          kind: "invalid",
          message: {
            code: "title.required",
            severity: "error",
            target: "title",
            visibility: "visible",
          },
        })),
      }),
    });

    const titleHint = form.layout().fields[0];
    assert.equal(titleHint.capabilityPosture.posture, "unavailable");
    assert.equal(titleHint.capabilities.supportsMessageTrack, false);
    assert.deepEqual(titleHint.capabilityPosture.unavailableCapabilities, [
      "minHeightSync",
      "responsiveTokens",
      "messageTrack",
    ]);
    assert.match(titleHint.capabilityPosture.reason, /cannot honor/);
    assert.equal(form.layout().summary.unavailableFields, 1);
  } finally {
    await cleanup();
  }
});

test("signals.form layout declarations deny malformed field and step layout hints", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());

    assert.throws(
      () => signals.form({
        source: { title: "" },
        fields: ({ field }) => ({
          title: field("title", {
            layout: {
              minHeight: -1,
            },
          }),
        }),
      }),
      /minHeight must be a non-negative finite number/,
    );

    assert.throws(
      () => signals.form({
        source: { title: "" },
        fields: ({ field }) => ({
          title: field("title", {
            row: "hero",
            layout: {
              row: "hero",
            },
          }),
        }),
      }),
      /either layout or flat layout hints, not both/,
    );

    assert.throws(
      () => signals.form({
        source: { title: "" },
        fields: ({ field }) => ({
          title: field("title"),
        }),
        steps: ({ step }) => ({
          details: step("details", ["title"], {
            density: "teleport",
          }),
        }),
      }),
      /step layout density is not supported/,
    );
  } finally {
    await cleanup();
  }
});
