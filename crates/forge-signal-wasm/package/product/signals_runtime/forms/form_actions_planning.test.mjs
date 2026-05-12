import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../runtime_fixture/graph_operational_runtime.mjs";

test("signals.form lowers submit and custom actions into proof-bearing plans", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const source = signals.input({
      title: "Ship docs",
      archived: false,
      locked: false,
    });
    const form = signals.form({
      source,
      fields: ({ field }) => ({
        title: field("title"),
        archived: field("archived"),
        locked: field("locked"),
      }),
      availability: ({ action }) => ({
        submitAvailability: action("submit", ["locked"], (values) => (
          values.locked ? { state: "blocked", reason: "record is locked" } : "enabled"
        )),
      }),
      admission: ({ action }) => ({
        archiveAdmission: action("archive", "approval", ["archived"], () => ({
          posture: "requiresApproval",
          actorDigest: "actor:reviewer",
          policyDigest: "policy:archive",
        })),
      }),
      actions: ({ action }) => ({
        saveDraft: action("saveDraft", {
          patchPolicy: "allowEmpty",
          idempotency: "collapse",
          hostEffect: "draft.store",
        }),
        archive: action("archive", {
          patchPolicy: "ignore",
          destructive: true,
          idempotency: "deny",
          admissionCapability: "approval",
          hostEffect: "task.archive",
        }),
      }),
    });

    form.fields.title.set("Temporarily changed");
    form.fields.title.set("Ship docs");

    const submitPlan = form.actionPlan("submit");
    assert.equal(submitPlan.status, "denied");
    assert.equal(submitPlan.patch.policy, "requiresNonEmpty");
    assert.deepEqual(submitPlan.readiness.blockers.map((blocker) => blocker.kind), ["unchanged"]);
    assert.deepEqual(
      submitPlan.recoveryActions.map((action) => action.kind),
      ["focusFirstActionableBlocker"],
    );
    assert.equal(submitPlan.diagnostics.deniedBeforeEffects, true);
    assert.equal(typeof submitPlan.planDigest, "string");
    assert.ok(submitPlan.planDigest.length > 0);

    const saveDraftPlan = form.actionPlan("saveDraft");
    assert.equal(saveDraftPlan.status, "accepted");
    assert.equal(saveDraftPlan.patch.empty, true);
    assert.equal(saveDraftPlan.idempotency, "collapse");
    assert.equal(saveDraftPlan.hostEffect, "draft.store");
    assert.equal(saveDraftPlan.diagnostics.consumesLoweredPlan, true);

    const archivePlan = form.actionPlan("archive");
    assert.equal(archivePlan.status, "denied");
    assert.equal(archivePlan.destructive, true);
    assert.equal(archivePlan.patch.policy, "ignore");
    assert.equal(archivePlan.idempotency, "deny");
    assert.equal(archivePlan.proof.effectDigest.includes("task.archive"), true);
    assert.deepEqual(
      archivePlan.readiness.blockers.map((blocker) => blocker.kind),
      ["admission:requiresApproval"],
    );
    assert.equal(archivePlan.regulatedActionBindings.length, 1);
    assert.equal(archivePlan.regulatedActionBindings[0].actionPlanDigest, archivePlan.planDigest);
    assert.equal(archivePlan.regulatedActionBindings[0].actorDigest, "actor:reviewer");
    assert.equal(archivePlan.proof.patchDigest, form.patchPlan().equivalenceDigest);
    assert.equal(form.actions().summary.destructive, 1);
    assert.equal(form.actions().counters.deniedPlans, 2);
    assert.equal(form.diagnostics().actions.catalog.length, 3);
  } finally {
    await cleanup();
  }
});

test("signals.form records action result artifacts and repeated-attempt policy", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
      actions: ({ action }) => ({
        collapseDraft: action("collapseDraft", { idempotency: "collapse" }),
        supersedeDraft: action("supersedeDraft", { idempotency: "supersede" }),
        queueDraft: action("queueDraft", { idempotency: "queue" }),
        denyDraft: action("denyDraft", { idempotency: "deny" }),
      }),
    });

    form.fields.title.set("Ship docs now");

    const firstCollapse = form.attemptAction("collapseDraft");
    const secondCollapse = form.attemptAction("collapseDraft");
    assert.equal(firstCollapse.resultKind, "accepted");
    assert.equal(secondCollapse.resultKind, "noOp");
    assert.equal(secondCollapse.repeatedAttempt, "collapse");
    assert.equal(secondCollapse.collapsedIntoAttemptId, firstCollapse.attemptId);

    const firstSupersede = form.attemptAction("supersedeDraft");
    const secondSupersede = form.attemptAction("supersedeDraft");
    assert.equal(secondSupersede.resultKind, "accepted");
    assert.equal(secondSupersede.repeatedAttempt, "supersede");
    assert.equal(secondSupersede.supersededAttemptId, firstSupersede.attemptId);
    assert.equal(
      form.actionHistory().some((entry) => (
        entry.resultKind === "superseded" &&
        entry.supersededAttemptId === firstSupersede.attemptId &&
        entry.supersededByAttemptId === secondSupersede.attemptId
      )),
      true,
    );

    form.attemptAction("queueDraft");
    const queued = form.attemptAction("queueDraft");
    assert.equal(queued.resultKind, "accepted");
    assert.equal(queued.repeatedAttempt, "queue");
    assert.equal(queued.queuePosition, 1);

    form.attemptAction("denyDraft");
    const deniedDuplicate = form.attemptAction("denyDraft");
    assert.equal(deniedDuplicate.resultKind, "denied");
    assert.equal(deniedDuplicate.repeatedAttempt, "deny");
    assert.deepEqual(
      deniedDuplicate.blockers.map((blocker) => blocker.kind),
      ["idempotency:duplicate"],
    );
    assert.equal(form.diagnostics().actionHistory.length, 9);
  } finally {
    await cleanup();
  }
});

test("signals.form does not collapse effectful empty-patch actions into no-op", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
      actions: ({ action, step }) => ({
        approve: action("approve", {
          patchPolicy: "allowEmpty",
          hostEffect: "workflow.approve",
        }),
        inert: action("inert", {
          patchPolicy: "allowEmpty",
          effectPolicy: "none",
        }),
        noopStep: step("noopStep", "details", "custom"),
      }),
      steps: ({ step }) => ({
        details: step("details", ["title"]),
      }),
    });

    assert.equal(form.attemptAction("approve").resultKind, "accepted");
    assert.equal(form.attemptAction("noopStep").resultKind, "accepted");
    assert.equal(form.attemptAction("inert").resultKind, "noOp");
  } finally {
    await cleanup();
  }
});

test("signals.form action result artifacts preserve blocker recovery hints", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
      validation: ({ field }) => ({
        titleRequired: field("title", (value) => (
          value.length > 0
            ? true
            : {
              kind: "invalid",
              message: {
                code: "title.required",
                target: "title",
                severity: "error",
                audience: "user",
                visibility: "visible",
              },
            }
        )),
      }),
    });

    form.fields.title.set("draft");
    form.fields.title.set("");

    const result = form.attemptAction("submit");
    assert.equal(result.resultKind, "denied");
    assert.equal(result.planDigest, form.actionPlan("submit").planDigest);
    assert.deepEqual(
      result.recoveryActions.map((action) => action.kind),
      ["editField", "resetField", "focusFirstActionableBlocker"],
    );
    assert.equal(typeof result.resultDigest, "string");
    assert.ok(result.resultDigest.length > 0);
  } finally {
    await cleanup();
  }
});

test("signals.form declares controller-local step actions without route semantics", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: {
        title: "Ship docs",
        assignee: "Ada",
      },
      fields: ({ field }) => ({
        title: field("title"),
        assignee: field("assignee"),
      }),
      steps: ({ step }) => ({
        details: step("details", ["title"], { order: 1 }),
        assignment: step("assignment", ["assignee"], { order: 2 }),
      }),
      actions: ({ step }) => ({
        nextDetails: step("nextDetails", "details", "next"),
        revisitAssignment: step("revisitAssignment", "assignment", "revisit", {
          idempotency: "supersede",
        }),
      }),
    });

    const nextDetails = form.actionPlan("nextDetails");
    assert.equal(nextDetails.status, "accepted");
    assert.equal(nextDetails.kind, "step");
    assert.deepEqual(nextDetails.step, {
      stepId: "details",
      command: "next",
      routeCoupled: false,
    });
    assert.equal(nextDetails.effectPolicy, "controllerLocal");
    assert.equal(nextDetails.diagnostics.routeSemantics, "controllerLocalOnly");
    assert.equal(form.actions().summary.step, 2);
    assert.equal(form.actions().counters.stepPlans, 2);

    const revisitAssignment = form.actionPlan("revisitAssignment");
    assert.equal(revisitAssignment.idempotency, "supersede");
    assert.equal(revisitAssignment.diagnostics.repeatedAttemptPolicy, "supersede");
  } finally {
    await cleanup();
  }
});

test("signals.form denies malformed action declarations before planning", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());

    assert.throws(
      () =>
        signals.form({
          source: { title: "Ship docs" },
          fields: ({ field }) => ({
            title: field("title"),
          }),
          actions: ({ action }) => ({
            invalid: action("save", { patchPolicy: "sometimes" }),
          }),
        }),
      /action patch policy is not supported/,
    );

    assert.throws(
      () =>
        signals.form({
          source: { title: "Ship docs" },
          fields: ({ field }) => ({
            title: field("title"),
          }),
          actions: ({ submit, action }) => ({
            submit: submit(),
            duplicate: action("submit"),
          }),
        }),
      /action declaration ids must be unique/,
    );

    assert.throws(
      () =>
        signals.form({
          source: { title: "Ship docs" },
          fields: ({ field }) => ({
            title: field("title"),
          }),
          actions: ({ action }) => ({
            impersonator: action("impersonator", { kind: "step" }),
          }),
        }),
      /custom actions cannot impersonate built-in action kinds/,
    );

    assert.throws(
      () =>
        signals.form({
          source: { title: "Ship docs" },
          fields: ({ field }) => ({
            title: field("title"),
          }),
          steps: ({ step }) => ({
            details: step("details", ["title"]),
          }),
          actions: ({ step }) => ({
            missing: step("missing", "review", "next"),
          }),
        }),
      /step action references an undeclared step/,
    );

    const form = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
    });
    assert.throws(
      () => form.actionPlan("missing"),
      /form action is not declared/,
    );
  } finally {
    await cleanup();
  }
});
