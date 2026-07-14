import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../../runtime_fixture/graph_operational_runtime.mjs";

test("signals.form accessibility report derives labels, relationships, and order hints from canonical form artifacts", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "", summary: "", approved: false },
      fields: ({ field }) => ({
        title: field("title", {
          label: "Document title",
          description: "Shown in release notes",
          summaryOrder: 0,
        }),
        summary: field("summary", {
          label: "Summary",
          readingOrder: 1,
          focusOrder: 1,
          summaryOrder: 1,
        }),
        approved: field("approved", {
          label: "Approval",
          readingOrder: 2,
          focusOrder: 2,
          summaryOrder: 2,
        }),
      }),
      validation: ({ field }) => ({
        titleRequired: field("title", (value) => (
          value.trim().length === 0
            ? {
                kind: "invalid",
                message: {
                  code: "title.required",
                  message: "Title is required",
                  severity: "error",
                  target: "title",
                  visibility: "summary",
                  accessibility: {
                    describedBy: ["title-help"],
                    focusTarget: "title",
                  },
                },
              }
            : true
        )),
      }),
      availability: ({ field }) => ({
        titleRequired: field("title", ["approved"], (values) => (
          values.approved ? "enabled" : "required"
        )),
        summaryReadonly: field("summary", ["approved"], () => "readonly"),
      }),
      steps: ({ step }) => ({
        details: step("details", ["title", "summary"], { order: 2 }),
        approval: step("approval", ["approved"], { order: 1 }),
      }),
    });

    const accessibility = form.accessibility();
    const titleField = accessibility.fields.find((entry) => entry.field === "title");
    const summaryField = accessibility.fields.find((entry) => entry.field === "summary");
    assert.ok(titleField);
    assert.ok(summaryField);
    assert.equal(titleField.label, "Document title");
    assert.equal(titleField.description, "Shown in release notes");
    assert.equal(titleField.required, true);
    assert.equal(titleField.invalid, true);
    assert.deepEqual(titleField.describedBy, [ "title-help", accessibility.messages[0].id ]);
    assert.equal(summaryField.readonly, true);
    assert.equal(accessibility.messages[0].focusTarget, "title");
    assert.equal(accessibility.focusTarget.posture, "ready");
    assert.equal(accessibility.focusTarget.field, "title");
    assert.deepEqual(accessibility.orderHints.readingOrder, ["title", "summary", "approved"]);
    assert.deepEqual(accessibility.orderHints.focusOrder, ["title", "summary", "approved"]);
    assert.deepEqual(accessibility.orderHints.sectionOrder, ["approval", "details"]);
    assert.deepEqual(accessibility.orderHints.summaryOrder, [accessibility.messages[0].id]);
    assert.equal(typeof accessibility.orderDigest, "string");
    assert.equal(accessibility.summary.invalidFields, 1);
    assert.equal(accessibility.summary.requiredFields, 1);
    assert.equal(accessibility.counters.sections, 2);
    assert.equal(form.diagnostics().accessibility.digest, accessibility.digest);
    assert.equal(form.verification().digests.accessibilityDigest, accessibility.digest);
    assert.equal(form.verification().digests.presentationOrderHintDigest, accessibility.orderDigest);
    assert.equal(form.verification().performanceEnvelope.accessibility.invalidFields, 1);
  } finally {
    await cleanup();
  }
});

test("signals.form accessibility focus target stays typed unavailable when the adapter cannot report focus", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "" },
      fields: ({ field }) => ({
        title: field("title", {
          label: "Title",
          adapter: {
            reportsFocus: false,
          },
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
    });

    const accessibility = form.accessibility();
    assert.equal(accessibility.focusTarget.posture, "unavailable");
    assert.equal(accessibility.focusTarget.field, "title");
    assert.match(accessibility.focusTarget.reason, /does not report focus/);
    assert.equal(accessibility.counters.focusUnavailableFields, 1);
  } finally {
    await cleanup();
  }
});

test("signals.form accessibility denies undeclared validation message focus targets", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
      validation: ({ field }) => ({
        malformed: field("title", () => ({
          kind: "invalid",
          message: {
            code: "title.required",
            severity: "error",
            target: "title",
            visibility: "visible",
            accessibility: {
              focusTarget: "missing",
            },
          },
        })),
      }),
    });

    assert.throws(
      () => form.accessibility(),
      /focusTarget must reference a declared field/,
    );
  } finally {
    await cleanup();
  }
});

test("signals.form only emits a dedicated order-hint digest when order hints are actually declared", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "", summary: "" },
      fields: ({ field }) => ({
        title: field("title", {
          label: "Title",
        }),
        summary: field("summary", {
          label: "Summary",
        }),
      }),
      steps: ({ step }) => ({
        details: step("details", ["title", "summary"]),
      }),
    });

    const accessibility = form.accessibility();
    assert.deepEqual(accessibility.orderHints.readingOrder, ["title", "summary"]);
    assert.equal(accessibility.orderDigest, null);
    assert.equal(form.verification().digests.presentationOrderHintDigest, null);
    assert.equal(typeof accessibility.digest, "string");
  } finally {
    await cleanup();
  }
});
