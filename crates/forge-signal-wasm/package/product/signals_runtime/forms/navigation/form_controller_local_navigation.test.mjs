import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../../runtime_fixture/graph_operational_runtime.mjs";

test("signals.form executes controller-local step navigation with visible settlement and replay-honest history", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: {
        title: "Ship docs",
        mode: "draft",
      },
      fields: ({ field }) => ({
        title: field("title"),
        mode: field("mode"),
      }),
      steps: ({ step }) => ({
        details: step("details", ["title"], { order: 1 }),
        assignment: step("assignment", ["mode"], { order: 2 }),
        review: step("review", ["title"], { order: 3 }),
      }),
      actions: ({ step }) => ({
        nextDetails: step("nextDetails", "details", "next"),
        skipAssignment: step("skipAssignment", "assignment", "skip"),
        revisitAssignment: step("revisitAssignment", "assignment", "revisit"),
        jumpReview: step("jumpReview", "review", "jump"),
        backReview: step("backReview", "review", "back"),
      }),
      presentation: {
        navigation: { delayedBusyRevealMs: 0, minimumBusyMs: 0 },
      },
    });

    assert.equal(form.navigation().current.stepId, "details");

    const nextExecution = form.executeAction("nextDetails");
    assert.equal(nextExecution.resultKind, "fulfilled");
    assert.equal(form.navigation().current.stepId, "assignment");
    assert.equal(form.presentationLifecycle("navigation:local").status, "settling");
    assert.equal(form.presentationLifecycle("navigation:local").scope, "step");
    assert.equal(form.acknowledgePresentation("navigation:local").resultKind, "acknowledged");
    assert.equal(form.presentationLifecycle("navigation:local").status, "ready");

    const skipExecution = form.executeAction("skipAssignment");
    assert.equal(skipExecution.resultKind, "fulfilled");
    assert.equal(form.navigation().current.stepId, "review");
    assert.deepEqual(form.navigation().current.skippedStepIds, ["assignment"]);

    const backExecution = form.executeAction("backReview");
    assert.equal(backExecution.resultKind, "fulfilled");
    assert.equal(form.navigation().current.stepId, "assignment");

    const jumpExecution = form.executeAction("jumpReview");
    assert.equal(jumpExecution.resultKind, "fulfilled");
    assert.equal(form.navigation().current.stepId, "review");

    const revisitExecution = form.executeAction("revisitAssignment");
    assert.equal(revisitExecution.resultKind, "fulfilled");
    assert.equal(form.navigation().current.stepId, "assignment");
    assert.deepEqual(form.navigation().current.skippedStepIds, []);
    assert.equal(form.navigation().history.length, 5);
    assert.equal(form.navigation().latest.toStepId, "assignment");
    assert.equal(typeof form.verification().digests.navigationDigest, "string");
    assert.equal(typeof form.verification().digests.navigationHistoryDigest, "string");
    assert.equal(form.diagnostics().navigation.summary.currentStepId, "assignment");
  } finally {
    await cleanup();
  }
});

test("signals.form denies controller-local step actions that cannot produce an honest navigation transition", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: {
        title: "Ship docs",
        mode: "active",
      },
      fields: ({ field }) => ({
        title: field("title"),
        mode: field("mode"),
      }),
      steps: ({ step }) => ({
        details: step("details", ["title"], { order: 1 }),
        archive: step("archive", ["mode"], {
          order: 2,
          dependencies: ["mode"],
          resolve: (values) => values.mode === "removed"
            ? { posture: "removed", reason: "archive step is removed" }
            : "active",
        }),
      }),
      actions: ({ step }) => ({
        backDetails: step("backDetails", "details", "back"),
        nextArchive: step("nextArchive", "archive", "next"),
        jumpArchive: step("jumpArchive", "archive", "jump"),
        customArchive: step("customArchive", "archive", "custom"),
      }),
    });

    assert.deepEqual(
      form.actionPlan("backDetails").readiness.blockers.map((blocker) => blocker.kind),
      ["navigation:noBackStep"],
    );
    assert.deepEqual(
      form.actionPlan("nextArchive").readiness.blockers.map((blocker) => blocker.kind),
      ["navigation:notCurrentStep"],
    );

    form.fields.mode.set("removed");
    assert.deepEqual(
      form.actionPlan("jumpArchive").readiness.blockers.map((blocker) => blocker.kind),
      ["navigation:removedTarget"],
    );
    assert.deepEqual(
      form.actionPlan("customArchive").readiness.blockers.map((blocker) => blocker.kind),
      ["action:deferred"],
    );
  } finally {
    await cleanup();
  }
});
