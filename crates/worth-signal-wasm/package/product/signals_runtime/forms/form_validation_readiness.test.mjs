import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../runtime_fixture/graph_operational_runtime.mjs";

test("signals.form derives validation artifacts, messages, and readiness blockers", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const rawSignals = createGraphOperationalRuntime();
    const signals = wrapSignals(rawSignals);
    const source = signals.input({
      title: "Ship docs",
      start: "2026-05-12",
      end: "2026-05-13",
      age: 7,
      priority: "normal",
      approval: "ok",
    });

    const form = signals.form({
      source,
      fields: ({ field }) => ({
        title: field("title"),
        start: field("start"),
        end: field("end"),
        age: field("age", {
          parse(rawValue) {
            const value = Number(rawValue);
            if (Number.isNaN(value)) {
              throw new Error("Age must be numeric");
            }
            return value;
          },
        }),
        priority: field("priority"),
        approval: field("approval"),
      }),
      validation: ({ field, form }) => ({
        titleRequired: field("title", (value) => (
          value.length > 0
            ? { kind: "valid", field: "title", digest: value }
            : {
              kind: "invalid",
              field: "title",
              message: {
                code: "task.title.required",
                message: "Title is required",
                severity: "error",
                target: "title",
                audience: "user",
                visibility: "visible",
                accessibility: {
                  announce: "assertive",
                  focusTarget: "title",
                },
              },
            }
        )),
        dateRange: form("dateRange", ["start", "end"], (values) => (
          values.start <= values.end
            ? { kind: "valid", digest: `${values.start}:${values.end}` }
            : {
              kind: "invalid",
              field: "end",
              message: {
                code: "task.date.range",
                message: "End date must follow start date",
                severity: "error",
                target: "end",
                audience: "user",
                visibility: "summary",
              },
            }
        )),
        priorityWarning: field("priority", (value) => (
          value === "high"
            ? {
              kind: "warning",
              field: "priority",
              message: {
                code: "task.priority.high",
                severity: "warning",
                target: "priority",
                audience: "user",
                visibility: "visible",
              },
            }
            : { kind: "valid", field: "priority", digest: value }
        )),
        approvalCheck: field("approval", (value) => (
          value === "checking"
            ? {
              kind: "pending",
              field: "approval",
              asyncValidationId: "approval-check",
            }
            : { kind: "valid", field: "approval", digest: value }
        )),
        validationIsReadOnly: field("title", (value, context) => {
          assert.equal(context.field.set, undefined);
          assert.equal(context.field.input, undefined);
          assert.equal(context.form.field("title").set, undefined);
          assert.equal(context.form.field("title").input, undefined);
          return { kind: "valid", field: "title", digest: value };
        }, { id: "field:titleReadOnly" }),
      }),
    });

    assert.equal(form.validation().summary.invalid, 0);
    assert.equal(form.validation().summary.pending, 0);
    assert.deepEqual(form.validation().dependencyBreadth, [
      { id: "field:title", breadth: "field", dependencies: ["title"] },
      { id: "dateRange", breadth: "dependencyRegion", dependencies: ["start", "end"] },
      { id: "field:priority", breadth: "field", dependencies: ["priority"] },
      { id: "field:approval", breadth: "field", dependencies: ["approval"] },
      { id: "field:titleReadOnly", breadth: "field", dependencies: ["title"] },
    ]);

    form.fields.priority.set("high");
    assert.equal(form.validation().summary.warning, 1);
    assert.equal(form.readiness().canSubmit, true);
    assert.deepEqual(
      form.visibleMessages().map((message) => message.code),
      ["task.priority.high"],
    );

    form.fields.title.set("");
    assert.equal(form.validation().summary.invalid, 1);
    assert.deepEqual(
      form.readiness().blockers.filter((blocker) => blocker.kind.startsWith("validation:")),
      [
        {
          kind: "validation:invalid",
          field: "title",
          reason: "Title is required",
        },
      ],
    );
    assert.equal(form.visibleMessages()[0].accessibility.focusTarget, "title");

    form.fields.title.set("Ready");
    form.fields.end.set("2026-05-01");
    assert.deepEqual(
      form.visibleMessages().map((message) => message.code),
      ["task.date.range", "task.priority.high"],
    );
    assert.equal(form.readiness().canSubmit, false);

    form.fields.end.set("2026-05-13");
    form.fields.approval.set("checking");
    assert.equal(form.validation().summary.pending, 1);
    assert.equal(form.readiness().blockers.at(-1).kind, "validation:pending");

    form.fields.approval.set("ok");
    form.fields.age.input("not-a-number").commitInput();
    assert.equal(form.fields.age.effectiveValue(), 7);
    assert.equal(form.fields.age.diagnostics().pendingRawInput, false);
    assert.equal(form.validation().summary.parseFailure, 1);
    assert.equal(
      form.readiness().blockers.some((blocker) => blocker.kind === "uncommittedRawInput"),
      false,
    );
    assert.deepEqual(
      form.readiness().blockers.filter((blocker) => blocker.kind === "validation:parseFailure"),
      [
        {
          kind: "validation:parseFailure",
          field: "age",
          reason: "Age must be numeric",
        },
      ],
    );
    assert.equal(form.fields.title.effectiveValue(), "Ready");
    assert.equal(form.diagnostics().validation.counters.fieldLocal, 4);
    assert.equal(form.diagnostics().validation.counters.dependencyRegion, 1);
  } finally {
    await cleanup();
  }
});

test("signals.form denies undeclared validator dependencies and malformed artifacts", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const rawSignals = createGraphOperationalRuntime();
    const signals = wrapSignals(rawSignals);

    assert.throws(
      () =>
        signals.form({
          source: { title: "Ship docs" },
          fields: ({ field }) => ({
            title: field("title"),
          }),
          validation: ({ field }) => ({
            missing: field("missing", () => ({ kind: "valid", digest: "ok" })),
          }),
        }),
      /undeclared form field/,
    );

    assert.throws(
      () =>
        signals.form({
          source: { title: "Ship docs", end: "2026-05-13" },
          fields: ({ field }) => ({
            title: field("title"),
            end: field("end"),
          }),
          validation: ({ form }) => ({
            duplicate: form("duplicate", ["title", "title"], () => ({
              kind: "valid",
              digest: "ok",
            })),
          }),
        }),
      /dependencies must be unique/,
    );

    const form = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
      validation: ({ field }) => ({
        malformed: field("title", () => ({ kind: "mystery" })),
      }),
    });

    assert.throws(
      () => form.validation(),
      /undeclared validation artifact shape/,
    );

    const undeclaredArtifactFieldForm = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
      validation: ({ field }) => ({
        malformed: field("title", () => ({
          kind: "invalid",
          field: "missing",
          message: {
            code: "missing.field",
            severity: "error",
            audience: "user",
            visibility: "visible",
          },
        })),
      }),
    });

    assert.throws(
      () => undeclaredArtifactFieldForm.validation(),
      /undeclared form field/,
    );

    const undeclaredMessageTargetForm = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
      validation: ({ field }) => ({
        malformedTarget: field("title", () => ({
          kind: "invalid",
          field: "title",
          message: {
            code: "missing.target",
            severity: "error",
            target: "missing",
            audience: "user",
            visibility: "visible",
          },
        })),
      }),
    });

    assert.throws(
      () => undeclaredMessageTargetForm.validation(),
      /undeclared form field/,
    );
  } finally {
    await cleanup();
  }
});
