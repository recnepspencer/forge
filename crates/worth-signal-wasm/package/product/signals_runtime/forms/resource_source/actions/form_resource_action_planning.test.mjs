import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../../../runtime_fixture/graph_operational_runtime.mjs";
import {
  createDetailPatchLineFixture,
  createReadOnlyResourceLineFixture,
} from "../fixtures/resource_line_fixture.mjs";

test("signals.form plans declared resource-line custom patch actions through the same resource binding authority as submit", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const line = createDetailPatchLineFixture({
      effectProfile: signals.resource.effects.branchNative(),
      initialValue: { title: "Ship docs" },
    });
    const resourceForm = signals.form({
      source: signals.form.source.resourceLine(line, { id: "resource-custom-plan" }),
      fields: ({ field }) => ({
        title: field("title"),
      }),
      actions: ({ action }) => ({
        saveResourceDraft: action("saveResourceDraft", {
          resourceAction: { kind: "patchPlan" },
          resourceEffectProfile: signals.resource.effects.branchNative(),
        }),
      }),
    });

    resourceForm.fields.title.set("Queued draft");
    const plan = resourceForm.actionPlan("saveResourceDraft");
    assert.equal(plan.status, "accepted");
    assert.equal(plan.resourceAction.declared, true);
    assert.equal(plan.resourceAction.source, "declaredPatchPlan");
    assert.equal(plan.resourceEffectProfile.source, "declaredMatchesResourceLine");

    const plainForm = signals.form({
      source: { title: "Plain task" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
      actions: ({ action }) => ({
        saveResourceDraft: action("saveResourceDraft", {
          resourceAction: { kind: "patchPlan" },
        }),
      }),
    });
    const denied = plainForm.actionPlan("saveResourceDraft");
    assert.equal(denied.status, "denied");
    assert.equal(
      denied.readiness.blockers.some((blocker) => blocker.kind === "resource:actionUnavailable"),
      true,
    );
  } finally {
    await cleanup();
  }
});

test("signals.form admits resource-line lifecycle actions on resource lines and lets them recover stale or rejected source posture", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const line = createReadOnlyResourceLineFixture({
      effectProfile: signals.resource.effects.branchNative(),
      status: Object.freeze({
        kind: "rejected",
        operation: "refresh",
        message: "network down",
        continuity: "preservedVisibleValue",
      }),
      freshness: Object.freeze({ kind: "fresh" }),
    });
    const form = signals.form({
      source: signals.form.source.resourceLine(line, { id: "resource-lifecycle-plan" }),
      fields: ({ field }) => ({
        title: field("title"),
      }),
      actions: ({ action }) => ({
        refreshResourceSource: action("refreshResourceSource", {
          resourceAction: { kind: "refresh" },
        }),
        revalidateResourceSource: action("revalidateResourceSource", {
          resourceAction: { kind: "revalidate" },
        }),
      }),
    });

    const refreshPlan = form.actionPlan("refreshResourceSource");
    assert.equal(refreshPlan.status, "accepted");
    assert.equal(refreshPlan.patch.policy, "ignore");
    assert.equal(refreshPlan.resourceAction.source, "declaredRefresh");
    assert.deepEqual(refreshPlan.readiness.blockers, []);

    const revalidatePlan = form.actionPlan("revalidateResourceSource");
    assert.equal(revalidatePlan.status, "accepted");
    assert.equal(revalidatePlan.resourceAction.source, "declaredRevalidate");
    assert.deepEqual(revalidatePlan.readiness.blockers, []);

    const submitPlan = form.actionPlan("submit");
    assert.equal(submitPlan.status, "denied");
    assert.equal(
      submitPlan.readiness.blockers.some((blocker) => blocker.kind === "resource:rejected"),
      true,
    );
    assert.equal(
      submitPlan.readiness.blockers.some((blocker) => blocker.kind === "resource:actionUnavailable"),
      true,
    );
    assert.equal(
      submitPlan.recoveryActions.some((action) => action.kind === "refreshResourceSource"),
      true,
    );
    assert.equal(
      submitPlan.recoveryActions.find((action) => action.kind === "refreshResourceSource")?.action,
      "refreshResourceSource",
    );
    const submitAttempt = form.attemptAction("submit");
    assert.equal(
      submitAttempt.recoveryActions.some((action) => action.kind === "refreshResourceSource"),
      true,
    );
    assert.equal(
      submitAttempt.recoveryActions.find((action) => action.kind === "refreshResourceSource")?.action,
      "refreshResourceSource",
    );

    const plainForm = signals.form({
      source: { title: "Plain task" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
      actions: ({ action }) => ({
        refreshResourceSource: action("refreshResourceSource", {
          resourceAction: { kind: "refresh" },
        }),
      }),
    });
    const denied = plainForm.actionPlan("refreshResourceSource");
    assert.equal(denied.status, "denied");
    assert.equal(
      denied.readiness.blockers.some((blocker) => blocker.kind === "resource:actionUnavailable"),
      true,
    );
  } finally {
    await cleanup();
  }
});

test("signals.form exposes resource-aware recovery hints for stale and merge-conflict submit posture", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const staleLine = createReadOnlyResourceLineFixture({
      effectProfile: signals.resource.effects.branchNative(),
      status: Object.freeze({ kind: "fulfilled", operation: "initialLoad" }),
      freshness: Object.freeze({ kind: "stale", reason: "revalidationRequired" }),
    });
    const staleForm = signals.form({
      source: signals.form.source.resourceLine(staleLine, { id: "resource-stale-recovery" }),
      fields: ({ field }) => ({
        title: field("title"),
      }),
      actions: ({ action }) => ({
        revalidateResourceSource: action("revalidateResourceSource", {
          resourceAction: { kind: "revalidate" },
        }),
      }),
    });

    const stalePlan = staleForm.actionPlan("submit");
    assert.equal(stalePlan.status, "denied");
    assert.deepEqual(
      stalePlan.recoveryActions.map((action) => action.kind),
      ["focusFirstActionableBlocker", "revalidateResourceSource", "replayExactResourceSource"],
    );
    assert.equal(
      stalePlan.recoveryActions.find((action) => action.kind === "revalidateResourceSource")?.action,
      "revalidateResourceSource",
    );
  } finally {
    await cleanup();
  }
});

test("signals.form plans resource-line recovery actions through the shared resource action authority", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const line = createDetailPatchLineFixture({
      effectProfile: signals.resource.effects.branchNative(),
      initialValue: {
        title: "Ship docs",
        status: "draft",
      },
    });
    const form = signals.form({
      source: signals.form.source.resourceLine(line, { id: "resource-recovery-plan" }),
      fields: ({ field }) => ({
        title: field("title"),
        status: field("status"),
      }),
      actions: ({ action }) => ({
        replayResourceSource: action("replayResourceSource", {
          resourceAction: { kind: "replayExact" },
        }),
        restoreResourceSource: action("restoreResourceSource", {
          resourceAction: { kind: "restoreExact" },
        }),
        rollbackResourceEffect: action("rollbackResourceEffect", {
          resourceAction: { kind: "rollbackLastEffect" },
        }),
      }),
    });

    const replayPlan = form.actionPlan("replayResourceSource");
    assert.equal(replayPlan.status, "accepted");
    assert.equal(replayPlan.patch.policy, "ignore");
    assert.equal(replayPlan.resourceAction.source, "declaredReplayExact");

    const restorePlan = form.actionPlan("restoreResourceSource");
    assert.equal(restorePlan.status, "accepted");
    assert.equal(restorePlan.resourceAction.source, "declaredRestoreExact");

    const rollbackPlan = form.actionPlan("rollbackResourceEffect");
    assert.equal(rollbackPlan.status, "denied");
    assert.equal(rollbackPlan.resourceAction.source, "declaredRollbackLastEffect");
    assert.deepEqual(
      rollbackPlan.readiness.blockers.map((blocker) => blocker.kind),
      ["resource:actionUnavailable"],
    );

    form.fields.title.set("Published docs");
    form.fields.status.set("review");
    const submitExecution = form.executeAction("submit");
    assert.equal(submitExecution.resultKind, "fulfilled");

    const rollbackAfterSubmit = form.actionPlan("rollbackResourceEffect");
    assert.equal(rollbackAfterSubmit.status, "accepted");
    assert.equal(rollbackAfterSubmit.resourceAction.source, "declaredRollbackLastEffect");
    assert.deepEqual(rollbackAfterSubmit.readiness.blockers, []);
  } finally {
    await cleanup();
  }
});

test("signals.form denies malformed declared resource-line custom actions before planning", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    assert.throws(
      () => signals.form({
        source: { title: "Ship docs" },
        fields: ({ field }) => ({
          title: field("title"),
        }),
        actions: ({ action }) => ({
          invalidResourceAction: action("invalidResourceAction", {
            hostEffect: "draft.store",
            resourceAction: { kind: "patchPlan" },
          }),
        }),
      }),
      /cannot also declare hostEffect/,
    );
  } finally {
    await cleanup();
  }
});

test("signals.form denies resource-line submit planning before effects when the backing line is not patch-capable", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const line = createReadOnlyResourceLineFixture({
      effectProfile: signals.resource.effects.branchNative(),
      status: Object.freeze({ kind: "fulfilled", operation: "initialLoad" }),
      freshness: Object.freeze({ kind: "fresh" }),
    });
    const form = signals.form({
      source: signals.form.source.resourceLine(line, { id: "resource-submit-read-only" }),
      fields: ({ field }) => ({
        title: field("title"),
      }),
    });

    form.fields.title.set("Queued draft");
    const plan = form.actionPlan("submit");
    assert.equal(plan.status, "denied");
    assert.equal(plan.resourceAction.source, "submitWithoutPatchCapability");
    assert.equal(form.readiness().canSubmit, false);
    assert.deepEqual(
      plan.readiness.blockers.map((blocker) => blocker.kind),
      ["resource:actionUnavailable"],
    );
    assert.deepEqual(
      form.readiness().blockers.map((blocker) => blocker.kind),
      ["resource:actionUnavailable"],
    );
    const execution = form.executeAction("submit");
    assert.equal(execution.resultKind, "denied");
    assert.equal(execution.effectStarted, false);
    assert.equal("resourceSubmission" in execution, true);
    assert.equal(execution.resourceSubmission, undefined);
  } finally {
    await cleanup();
  }
});

test("signals.form denies patch-based resource actions before effects on collection and paged resource lines", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const collectionLine = createReadOnlyResourceLineFixture({
      effectProfile: signals.resource.effects.branchNative(),
      status: Object.freeze({ kind: "fulfilled", operation: "initialLoad" }),
      freshness: Object.freeze({ kind: "fresh" }),
      familyKind: "collection",
      familyId: "task-list",
      runtimeLineId: "task:list",
      canonicalKey: "workspace=current",
    });
    const collectionForm = signals.form({
      source: signals.form.source.resourceLine(collectionLine, { id: "resource-submit-collection" }),
      fields: ({ field }) => ({
        title: field("title"),
      }),
      actions: ({ action }) => ({
        saveResourceDraft: action("saveResourceDraft", {
          resourceAction: { kind: "patchPlan" },
        }),
      }),
    });

    collectionForm.fields.title.set("Queued collection edit");
    const collectionSubmitPlan = collectionForm.actionPlan("submit");
    assert.equal(collectionSubmitPlan.status, "denied");
    assert.equal(
      collectionSubmitPlan.resourceAction.source,
      "submitWithoutPatchCapability",
    );
    assert.equal(collectionForm.readiness().canSubmit, false);
    const collectionCustomPlan = collectionForm.actionPlan("saveResourceDraft");
    assert.equal(collectionCustomPlan.status, "denied");
    assert.equal(
      collectionCustomPlan.resourceAction.source,
      "declaredWithoutPatchCapability",
    );
    const collectionExecution = collectionForm.executeAction("submit");
    assert.equal(collectionExecution.resultKind, "denied");
    assert.equal(collectionExecution.effectStarted, false);
    assert.equal(collectionExecution.resourceSubmission, undefined);

    const pagedLine = createReadOnlyResourceLineFixture({
      effectProfile: signals.resource.effects.branchNative(),
      status: Object.freeze({ kind: "fulfilled", operation: "initialLoad" }),
      freshness: Object.freeze({ kind: "fresh" }),
      familyKind: "paged",
      familyId: "task-search",
      runtimeLineId: "task:page:1",
      canonicalKey: "query=task&page=1",
    });
    const pagedForm = signals.form({
      source: signals.form.source.resourceLine(pagedLine, { id: "resource-submit-paged" }),
      fields: ({ field }) => ({
        title: field("title"),
      }),
    });

    pagedForm.fields.title.set("Queued paged edit");
    const pagedPlan = pagedForm.actionPlan("submit");
    assert.equal(pagedPlan.status, "denied");
    assert.equal(
      pagedPlan.resourceAction.source,
      "submitWithoutPatchCapability",
    );
    assert.equal(pagedForm.readiness().canSubmit, false);
    assert.deepEqual(
      pagedPlan.readiness.blockers.map((blocker) => blocker.kind),
      ["resource:actionUnavailable"],
    );
    assert.deepEqual(
      pagedForm.readiness().blockers.map((blocker) => blocker.kind),
      ["resource:actionUnavailable"],
    );
  } finally {
    await cleanup();
  }
});
