import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../runtime_fixture/graph_operational_runtime.mjs";

test("signals.form derives availability and admission facts without mutating draft truth", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const rawSignals = createGraphOperationalRuntime();
    const signals = wrapSignals(rawSignals);
    const source = signals.input({
      title: "Ship docs",
      mode: "editable",
      role: "editor",
      locked: false,
      note: "Keep draft",
      clearable: "Keep until clear",
    });

    const form = signals.form({
      source,
      fields: ({ field }) => ({
        title: field("title"),
        mode: field("mode"),
        role: field("role"),
        locked: field("locked"),
        note: field("note"),
        clearable: field("clearable"),
      }),
      availability: ({ field, action }) => ({
        titleAvailability: field("title", ["mode"], (values, context) => {
          assert.equal(context.form.field("title").set, undefined);
          return values.mode === "archived"
            ? {
              state: "omitted",
              draftPolicy: "omit",
              reason: "archived records omit title edits",
            }
            : "enabled";
        }),
        submitAvailability: action("submit", ["locked"], (values) => (
          values.locked
            ? { state: "blocked", reason: "record is locked" }
            : "enabled"
        )),
        noteFreeze: field("note", ["mode"], (values) => (
          values.mode === "lockedEdit"
            ? { state: "disabled", draftPolicy: "freeze", reason: "note is frozen" }
            : "enabled"
        )),
        clearableClear: field("clearable", ["mode"], (values) => (
          values.mode === "clearDraft"
            ? { state: "disabled", draftPolicy: "clear", reason: "clearable draft is cleared" }
            : "enabled"
        )),
      }),
      admission: ({ field, action }) => ({
        titleEdit: field("title", "edit", ["role"], (values, context) => {
          assert.equal(context.form.field("title").input, undefined);
          return values.role === "editor"
            ? {
              posture: "admitted",
              actorDigest: "actor:editor",
              policyDigest: "policy:title-edit",
            }
            : {
              posture: "denied",
              reason: "role cannot edit title",
              actorDigest: `actor:${values.role}`,
              policyDigest: "policy:title-edit",
            };
        }),
        submitLock: action("submit", "submit", ["locked"], (values) => (
          values.locked
            ? { posture: "blocked", reason: "locked records cannot submit" }
            : "admitted"
        )),
      }),
    });

    form.fields.title.set("Ready");
    form.fields.note.set("Draft note");
    form.fields.clearable.set("Draft clearable");
    assert.equal(form.readiness().canSubmit, true);
    assert.equal(form.actionReadiness("submit").canRun, true);
    assert.deepEqual(form.availability().summary, {
      enabled: 4,
      disabled: 0,
      hidden: 0,
      readonly: 0,
      required: 0,
      omitted: 0,
      blocked: 0,
      unavailable: 0,
      byScope: {
        field: 3,
        action: 1,
        control: 0,
        group: 0,
        section: 0,
      },
    });

    form.fields.role.set("viewer");
    assert.equal(form.fields.title.effectiveValue(), "Ready");
    assert.throws(
      () => form.fields.title.set("Denied by role"),
      /role cannot edit title/,
    );
    assert.equal(form.fields.title.effectiveValue(), "Ready");
    assert.deepEqual(
      form.readiness().blockers.filter((blocker) => blocker.kind === "admission:denied"),
      [
        {
          kind: "admission:denied",
          field: "title",
          action: undefined,
          capability: "edit",
          reason: "role cannot edit title",
        },
      ],
    );

    form.fields.role.set("editor");
    form.fields.mode.set("lockedEdit");
    assert.throws(
      () => form.fields.note.set("Blocked by freeze"),
      /note is frozen/,
    );
    assert.equal(form.fields.note.effectiveValue(), "Draft note");
    assert.deepEqual(
      form.patchPlan().operations.map((operation) => operation.field),
      ["title", "mode", "note", "clearable"],
    );

    form.fields.mode.set("clearDraft");
    assert.throws(
      () => form.fields.clearable.set("Blocked by clear"),
      /clearable draft is cleared/,
    );
    assert.equal(form.fields.clearable.effectiveValue(), "Draft clearable");
    assert.deepEqual(
      form.dirty().fields.map((field) => field.field),
      ["title", "mode", "note"],
    );
    assert.equal(form.dirty().breadth.comparedFields, 5);
    assert.equal(form.dirty().breadth.clearedFields, 1);
    assert.deepEqual(
      form.patchPlan().operations.map((operation) => operation.field),
      ["title", "mode", "note"],
    );
    assert.equal(form.patchPlan().breadth.comparedFields, 5);
    assert.equal(form.patchPlan().breadth.clearedFields, 1);

    form.fields.mode.set("archived");
    assert.equal(form.fields.title.effectiveValue(), "Ready");
    assert.deepEqual(
      form.dirty().fields.map((field) => field.field),
      ["mode", "note", "clearable"],
    );
    assert.equal(form.dirty().breadth.comparedFields, 5);
    assert.equal(form.dirty().breadth.omittedFields, 1);
    assert.deepEqual(
      form.availability().artifacts.find((artifact) => artifact.ownerId === "title"),
      {
        kind: "availability",
        id: "field:title",
        scope: "field",
        ownerId: "title",
        fields: [],
        state: "omitted",
        draftPolicy: "omit",
        dependencies: ["mode"],
        reason: "archived records omit title edits",
      },
    );
    assert.deepEqual(
      form.patchPlan().operations.map((operation) => operation.field),
      ["mode", "note", "clearable"],
    );
    assert.equal(form.patchPlan().breadth.comparedFields, 5);
    assert.equal(form.patchPlan().breadth.omittedFields, 1);

    form.fields.locked.set(true);
    assert.deepEqual(
      form.actionReadiness("submit").blockers.map((blocker) => blocker.kind),
      ["availability:blocked", "admission:blocked"],
    );
    assert.equal(form.diagnostics().availability.summary.omitted, 1);
    assert.equal(form.diagnostics().admission.summary.blocked, 1);
  } finally {
    await cleanup();
  }
});

test("signals.form denies invalid availability and admission topology", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const rawSignals = createGraphOperationalRuntime();
    const signals = wrapSignals(rawSignals);

    assert.throws(
      () =>
        signals.form({
          source: { a: true, b: true },
          fields: ({ field }) => ({
            a: field("a"),
            b: field("b"),
          }),
          availability: ({ field }) => ({
            a: field("a", ["b"], () => "enabled"),
            b: field("b", ["a"], () => "enabled"),
          }),
        }),
      /availability dependency cycle denied/,
    );

    assert.throws(
      () =>
        signals.form({
          source: { title: "Ship docs" },
          fields: ({ field }) => ({
            title: field("title"),
          }),
          availability: ({ field }) => ({
            missing: field("title", ["missing"], () => "enabled"),
          }),
        }),
      /undeclared dependency field/,
    );

    assert.throws(
      () =>
        signals.form({
          source: { title: "Ship docs" },
          fields: ({ field }) => ({
            title: field("title"),
          }),
          admission: ({ field }) => ({
            invalidCapability: field("title", "teleport", ["title"], () => "admitted"),
          }),
        }),
      /admission capability is not supported/,
    );

    const malformed = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
      availability: ({ field }) => ({
        malformedState: field("title", ["title"], () => "teleport"),
      }),
    });
    assert.throws(
      () => malformed.availability(),
      /availability artifact state is not supported/,
    );

    const validForm = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
    });
    assert.throws(
      () => validForm.fieldWritePosture("missing"),
      /form field is not declared/,
    );
  } finally {
    await cleanup();
  }
});
