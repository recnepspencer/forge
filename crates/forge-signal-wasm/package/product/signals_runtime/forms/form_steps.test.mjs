import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../runtime_fixture/graph_operational_runtime.mjs";

test("signals.form derives controller-local step artifacts from form truth", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const rawSignals = createGraphOperationalRuntime();
    const signals = wrapSignals(rawSignals);
    const source = signals.input({
      title: "",
      assignee: "Ada",
      mode: "draft",
      locked: false,
    });

    const form = signals.form({
      source,
      fields: ({ field }) => ({
        title: field("title"),
        assignee: field("assignee"),
        mode: field("mode"),
        locked: field("locked"),
      }),
      validation: ({ field, form }) => ({
        titleRequired: field("title", (value) => (
          value.length > 0
            ? true
            : {
              kind: "invalid",
              message: {
                code: "task.title.required",
                message: "Title is required",
                target: "title",
                severity: "error",
                audience: "user",
                visibility: "visible",
              },
            }
        )),
        formAdvisory: form("formAdvisory", ["title", "assignee"], () => ({
          kind: "warning",
          message: {
            code: "task.form.advisory",
            severity: "warning",
            audience: "user",
            visibility: "summary",
          },
        })),
      }),
      availability: ({ action }) => ({
        submitAvailability: action("submit", ["locked"], (values) => (
          values.locked
            ? { state: "blocked", reason: "record is locked" }
            : "enabled"
        )),
      }),
      admission: ({ field }) => ({
        assigneeReview: field("assignee", "review", ["mode"], (values) => (
          values.mode === "review"
            ? {
              posture: "requiresReview",
              reason: "assignee needs review",
              actorDigest: "actor:reviewer",
              policyDigest: "policy:assignee-review",
            }
            : "admitted"
        )),
      }),
      steps: ({ step }) => ({
        details: step("details", ["title"], {
          order: 1,
          group: "main",
        }),
        assignment: step("assignment", ["assignee"], {
          order: 2,
          group: "main",
          dependencies: ["mode"],
          resolve: (values, context) => {
            assert.equal(context.form.field("assignee").set, undefined);
            return values.mode === "skipAssignment"
              ? { posture: "skipped", reason: "assignment skipped by mode" }
              : "active";
          },
        }),
        archive: step("archive", ["locked"], {
          order: 3,
          dependencies: ["mode"],
          resolve: (values) => (
            values.mode === "archived"
              ? { posture: "removed", reason: "archive step removed" }
              : { posture: "blocked", reason: "archive step is not active" }
          ),
        }),
      }),
    });

    assert.deepEqual(
      form.steps().summary,
      {
        total: 3,
        active: 2,
        optional: 0,
        skipped: 0,
        blocked: 1,
        removed: 0,
        unavailable: 0,
        complete: 1,
        changed: 0,
      },
    );
    assert.deepEqual(form.steps().counters, {
      costBasis: "derivedFullReportScan",
      incrementalStatus: "notIncremental",
      declarations: 3,
      stepFieldMemberships: 3,
      dependencyReads: 3,
      readinessBlockers: 2,
      projectedPatchOperations: 0,
      projectedValidationArtifacts: 6,
      uniqueProjectedValidationArtifacts: 4,
      projectedMessages: 4,
      uniqueProjectedMessages: 2,
    });
    assert.deepEqual(
      form.steps().artifacts.find((step) => step.id === "details").readiness.blockers.map((blocker) => blocker.kind),
      ["validation:invalid"],
    );
    assert.deepEqual(
      form.steps().artifacts.find((step) => step.id === "details").messages.map((message) => message.code),
      ["task.title.required", "task.form.advisory"],
    );

    form.fields.title.set("Ship docs");
    form.fields.mode.set("review");
    assert.equal(form.steps().counters.projectedPatchOperations, 1);
    assert.equal(form.steps().counters.readinessBlockers, 2);
    assert.equal(form.steps().counters.projectedValidationArtifacts, 6);
    assert.equal(form.steps().counters.uniqueProjectedValidationArtifacts, 4);
    assert.equal(form.steps().counters.projectedMessages, 3);
    assert.equal(form.steps().counters.uniqueProjectedMessages, 1);
    assert.deepEqual(
      form.steps().artifacts.find((step) => step.id === "details").patch.operations.map((operation) => operation.field),
      ["title"],
    );
    assert.deepEqual(
      form.steps().artifacts.find((step) => step.id === "assignment").readiness.blockers.map((blocker) => blocker.kind),
      ["admission:requiresReview"],
    );

    form.fields.mode.set("skipAssignment");
    const skippedAssignment = form.steps().artifacts.find((step) => step.id === "assignment");
    assert.equal(skippedAssignment.posture, "skipped");
    assert.equal(skippedAssignment.readiness.canEnter, false);
    assert.equal(skippedAssignment.progress, "skipped");

    form.fields.mode.set("archived");
    const removedArchive = form.steps().artifacts.find((step) => step.id === "archive");
    assert.equal(removedArchive.posture, "removed");
    assert.equal(removedArchive.progress, "removed");
    assert.deepEqual(
      form.steps().dependencyBreadth.find((step) => step.id === "assignment"),
      {
        id: "assignment",
        fields: ["assignee"],
        dependencies: ["mode"],
      },
    );
    assert.equal(form.diagnostics().steps.summary.removed, 1);
  } finally {
    await cleanup();
  }
});

test("signals.form denies invalid step topology and posture artifacts", async () => {
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
          steps: ({ step }) => ({
            missing: step("missing", ["missing"]),
          }),
        }),
      /step declaration references an undeclared field/,
    );

    assert.throws(
      () =>
        signals.form({
          source: { title: "Ship docs" },
          fields: ({ field }) => ({
            title: field("title"),
          }),
          steps: ({ step }) => ({
            duplicateField: step("duplicateField", ["title", "title"]),
          }),
        }),
      /step declaration fields must be unique/,
    );

    const malformed = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
      steps: ({ step }) => ({
        malformed: step("malformed", ["title"], {
          resolve: () => "teleport",
        }),
      }),
    });
    assert.throws(
      () => malformed.steps(),
      /step posture is not supported/,
    );
  } finally {
    await cleanup();
  }
});
