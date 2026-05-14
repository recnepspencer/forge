import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../runtime_fixture/graph_operational_runtime.mjs";

test("signals.form executes effect-backed actions through pending lifecycle artifacts", async () => {
  await withSignals((signals) => {
    const form = createTitleActionForm(signals, "publish", {
      patchPolicy: "allowEmpty",
      hostEffect: "workflow.publish",
    });

    const pending = form.executeAction("publish");
    assertActionExecution(pending, {
      resultKind: "pending",
      action: "publish",
      effectStarted: true,
      planDigest: form.actionPlan("publish").planDigest,
    });

    const fulfilled = form.fulfillAction(pending.operationId, {
      reason: "server accepted publication",
      canonicalValue: { title: "Ship docs", status: "published" },
    });
    assertActionExecution(fulfilled, {
      resultKind: "fulfilled",
      operationId: pending.operationId,
      stale: false,
    });
    assert.deepEqual(fulfilled.canonicalValue, { title: "Ship docs", status: "published" });
    assertHistoryKinds(form, ["pending", "fulfilled"]);
    assert.equal(form.verification().actionExecutionHistory.operations, 2);
  });
});

test("signals.form denies action execution before effects when plan is blocked", async () => {
  await withSignals((signals) => {
    const form = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
    });

    const denied = form.executeAction("submit");
    assertActionExecution(denied, {
      resultKind: "denied",
      action: "submit",
      effectStarted: false,
      attemptResultKind: "denied",
      planDigest: form.actionPlan("submit").planDigest,
    });
    assertHistoryKinds(form, ["denied"]);
  });
});

test("signals.form records stale completion attempts after terminal settlement", async () => {
  await withSignals((signals) => {
    const form = createTitleActionForm(signals, "exportPdf", {
      patchPolicy: "allowEmpty",
      hostEffect: "document.export",
    });

    const pending = form.executeAction("exportPdf");
    const cancelled = form.cancelAction(pending.operationId, {
      reason: "user closed dialog",
    });
    const stale = form.fulfillAction(pending.operationId);
    assertActionExecution(cancelled, {
      resultKind: "cancelled",
      operationId: pending.operationId,
    });
    assertActionExecution(stale, {
      resultKind: "staleCompletion",
      stale: true,
      targetOperationId: pending.operationId,
      targetAction: "exportPdf",
      targetPlanDigest: pending.planDigest,
      targetExecutionDigest: cancelled.executionDigest,
    });
    assertHistoryKinds(form, ["pending", "cancelled", "staleCompletion"]);
    assert.equal(form.actionExecutionHistory().at(-1).executionDigest, stale.executionDigest);
  });
});

test("signals.form rejects completions whose plan digest was invalidated by newer form truth", async () => {
  await withSignals((signals) => {
    const form = createTitleActionForm(signals, "publish", {
      patchPolicy: "allowEmpty",
      hostEffect: "workflow.publish",
    });

    const pending = form.executeAction("publish");
    form.fields.title.set("Ship docs after edit");
    const stale = form.fulfillAction(pending.operationId);
    assertActionExecution(stale, {
      resultKind: "staleCompletion",
      stale: true,
      targetOperationId: pending.operationId,
      targetAction: "publish",
      targetPlanDigest: pending.planDigest,
      targetExecutionDigest: pending.executionDigest,
      reason: "action execution completion targeted a superseded form truth snapshot",
    });
    assertHistoryKinds(form, ["pending", "staleCompletion"]);
  });
});

test("signals.form action fulfillment snapshots canonical values", async () => {
  await withSignals((signals) => {
    const form = createTitleActionForm(signals, "publish", {
      patchPolicy: "allowEmpty",
      hostEffect: "workflow.publish",
    });

    const pending = form.executeAction("publish");
    const canonicalValue = { title: "Server title" };
    const fulfilled = form.fulfillAction(pending.operationId, { canonicalValue });
    canonicalValue.title = "mutated after settlement";
    assert.deepEqual(fulfilled.canonicalValue, { title: "Server title" });
    assert.equal(Object.isFrozen(fulfilled.canonicalValue), true);
    assert.equal(Reflect.set(fulfilled.canonicalValue, "title", "mutated through artifact"), false);
    assert.deepEqual(fulfilled.canonicalValue, { title: "Server title" });
    assertHistoryKinds(form, ["pending", "fulfilled"]);
  });
});

test("signals.form fulfillment canonicalization updates source effective and draft truth", async () => {
  await withSignals((signals) => {
    const form = createTitleActionForm(signals, "publish", {
      patchPolicy: "allowEmpty",
      hostEffect: "workflow.publish",
    });

    form.fields.title.set("Client title");
    const pending = form.executeAction("publish");
    const fulfilled = form.fulfillAction(pending.operationId, {
      reason: "server canonicalized title",
      canonicalValue: { title: "Server title", status: "published" },
    });

    assertActionExecution(fulfilled, {
      resultKind: "fulfilled",
      operationId: pending.operationId,
    });
    assert.deepEqual(form.source(), { title: "Server title", status: "published" });
    assert.deepEqual(form.draft(), {});
    assert.deepEqual(form.effective(), { title: "Server title", status: "published" });
    assert.equal(form.dirty().isDirty, false);
    assert.equal(form.canonicalizationHistory().length, 1);
    assert.equal(form.canonicalizationHistory()[0].operationId, fulfilled.operationId);
    assert.deepEqual(
      form.canonicalizationHistory()[0].canonicalValue,
      { title: "Server title", status: "published" },
    );
    assert.equal(Object.isFrozen(form.canonicalizationHistory()[0].canonicalValue), true);
    assert.equal(form.verification().canonicalizationHistory.operations, 1);
    assert.equal(form.verification().performanceEnvelope.canonicalizationOperations, 1);
  });
});

test("signals.form canonical source projection yields to newer authoritative source drift", async () => {
  await withSignals((signals) => {
    const source = signals.input({ title: "Ship docs" });
    const form = signals.form({
      source,
      fields: ({ field }) => ({
        title: field("title"),
      }),
      actions: ({ action }) => ({
        publish: action("publish", {
          patchPolicy: "allowEmpty",
          hostEffect: "workflow.publish",
        }),
      }),
    });

    const pending = form.executeAction("publish");
    form.fulfillAction(pending.operationId, {
      canonicalValue: { title: "Server title" },
    });
    assert.deepEqual(form.source(), { title: "Server title" });

    source.set({ title: "Remote source drift" });
    assert.deepEqual(form.source(), { title: "Remote source drift" });
    assert.deepEqual(form.effective(), { title: "Remote source drift" });
    assert.equal(form.canonicalizationHistory()[0].sourceProjection, "serverCanonicalUntilAuthoritativeSourceDrift");
    assert.equal(form.presentationLifecycle("resourceDrift").status, "busy");
  });
});

test("signals.form migrates long-lived drafts across source schema drift with explicit evidence", async () => {
  await withSignals((signals) => {
    const source = signals.input({ title: "Ship docs" });
    const schemaVersion = signals.input("v1");
    const form = signals.form({
      source: {
        value: source,
        schemaVersion,
        migrateDraft(draft, context) {
          assert.equal(context.previousSchemaVersion, "v1");
          assert.equal(context.currentSchemaVersion, "v2");
          return {
            kind: "migrated",
            draft: { title: `${draft.title} (migrated)` },
            reason: "normalized draft to v2",
          };
        },
      },
      fields: ({ field }) => ({
        title: field("title"),
      }),
    });

    form.fields.title.set("Client title");
    source.set({ title: "Server title" });
    schemaVersion.set("v2");

    const compatibility = form.sourceCompatibility();
    assert.equal(compatibility.posture, "migrated");
    assert.equal(compatibility.reason, "normalized draft to v2");
    assert.deepEqual(form.draft(), { title: "Client title (migrated)" });
    assert.deepEqual(form.effective(), { title: "Client title (migrated)" });
    assert.equal(form.sourceCompatibilityHistory().length, 1);
    assert.equal(form.sourceCompatibilityHistory()[0].posture, "migrated");
    assert.equal(form.sourceCompatibilityHistory()[0].reason, "normalized draft to v2");
  });
});

test("signals.form blocks stale long-lived drafts when source schema drift has no migration policy", async () => {
  await withSignals((signals) => {
    const source = signals.input({ title: "Ship docs" });
    const schemaVersion = signals.input("v1");
    const form = signals.form({
      source: {
        value: source,
        schemaVersion,
      },
      fields: ({ field }) => ({
        title: field("title"),
      }),
      actions: ({ action }) => ({
        saveDraft: action("saveDraft", {
          patchPolicy: "allowEmpty",
          hostEffect: "draft.save",
        }),
      }),
    });

    form.fields.title.set("Client title");
    source.set({ title: "Server title" });
    schemaVersion.set("v2");

    const compatibility = form.sourceCompatibility();
    assert.equal(compatibility.posture, "unavailable");
    assert.equal(form.readiness().canSubmit, false);
    assert.deepEqual(
      form.readiness().blockers.map((blocker) => blocker.kind),
      ["schema:drift"],
    );
    assert.equal(form.actionPlan("saveDraft").status, "denied");
    assert.equal(form.executeAction("saveDraft").resultKind, "denied");
    assert.throws(
      () => form.fields.title.set("Blocked by drift"),
      /source schema changed and no draft migration policy is declared/,
    );
    assert.equal(form.sourceCompatibilityHistory().length, 1);
    assert.equal(form.sourceCompatibilityHistory()[0].posture, "unavailable");
  });
});

test("signals.form supersedes pending action executions with typed lifecycle history", async () => {
  await withSignals((signals) => {
    const form = createTitleActionForm(signals, "route", {
      patchPolicy: "allowEmpty",
      hostEffect: "workflow.route",
      idempotency: "supersede",
    });

    const first = form.executeAction("route");
    const second = form.executeAction("route");
    const superseded = form.actionExecutionHistory().find((entry) => (
      entry.resultKind === "superseded" &&
      entry.operationId === first.operationId
    ));
    assertActionExecution(first, { resultKind: "pending", action: "route" });
    assertActionExecution(second, { resultKind: "pending", action: "route" });
    assert.equal(superseded.supersededByOperationId, second.operationId);
    assert.equal(form.fulfillAction(first.operationId).resultKind, "staleCompletion");
    assert.equal(form.fulfillAction(second.operationId).resultKind, "fulfilled");
    assertHistoryKinds(form, [
      "pending",
      "superseded",
      "pending",
      "staleCompletion",
      "fulfilled",
    ]);
  });
});

test("signals.form maps server rejection messages without local validator authorship", async () => {
  await withSignals((signals) => {
    const form = createTitleActionForm(signals, "submitReview", {
      patchPolicy: "allowEmpty",
      hostEffect: "review.submit",
    });

    const pending = form.executeAction("submitReview");
    const rejected = form.rejectAction(pending.operationId, {
      reason: "server rejected field",
      messages: [{
        code: "title.not_unique",
        target: "title",
        scope: "field",
        severity: "error",
      }],
    });
    const retry = form.retryAction(rejected.operationId);
    assertActionExecution(rejected, {
      resultKind: "rejected",
      operationId: pending.operationId,
    });
    assert.deepEqual(rejected.serverMessages, [{
      code: "title.not_unique",
      target: "title",
      scope: "field",
      severity: "error",
      source: "server",
    }]);
    assertActionExecution(retry, {
      resultKind: "pending",
      retryOfOperationId: rejected.operationId,
    });
    assertHistoryKinds(form, ["pending", "rejected", "pending"]);
  });
});

test("signals.form records invalid and stale retries as terminal lifecycle artifacts", async () => {
  await withSignals((signals) => {
    const form = createTitleActionForm(signals, "publish", {
      patchPolicy: "allowEmpty",
      hostEffect: "workflow.publish",
    });

    const pending = form.executeAction("publish");
    const fulfilled = form.fulfillAction(pending.operationId);
    const invalidRetry = form.retryAction(fulfilled.operationId);
    assertActionExecution(invalidRetry, {
      resultKind: "staleCompletion",
      reason: "retry target is not retryable",
    });

    const retryablePending = form.executeAction("publish");
    const rejected = form.rejectAction(retryablePending.operationId);
    form.fields.title.set("Changed before retry");
    const staleRetry = form.retryAction(rejected.operationId);
    assertActionExecution(staleRetry, {
      resultKind: "staleCompletion",
      reason: "action execution completion targeted a superseded form truth snapshot",
    });
    assertHistoryKinds(form, [
      "pending",
      "fulfilled",
      "staleCompletion",
      "pending",
      "rejected",
      "staleCompletion",
    ]);
  });
});

async function withSignals(assertion) {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    await assertion(signals);
  } finally {
    await cleanup();
  }
}

function createTitleActionForm(signals, actionId, options) {
  return signals.form({
    source: { title: "Ship docs" },
    fields: ({ field }) => ({
      title: field("title"),
    }),
    actions: ({ action }) => ({
      [actionId]: action(actionId, options),
    }),
  });
}

function assertActionExecution(artifact, expected) {
  for (const [key, value] of Object.entries(expected)) {
    assert.deepEqual(artifact[key], value, `action execution ${key}`);
  }
  assert.equal(typeof artifact.executionDigest, "string");
  assert.ok(artifact.executionDigest.length > 0);
}

function assertHistoryKinds(form, expectedKinds) {
  assert.deepEqual(
    form.actionExecutionHistory().map((entry) => entry.resultKind),
    expectedKinds,
  );
}
