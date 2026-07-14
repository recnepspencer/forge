import assert from "node:assert/strict";
import test from "node:test";

import { withSignals } from "../../action_execution_test_helpers.mjs";
import {
  createDetailPatchLineFixture,
  createReadOnlyResourceLineFixture,
} from "../fixtures/resource_line_fixture.mjs";

function assertEquivalentOperationalTruth(left, right) {
  assert.deepEqual(left.source(), right.source());
  assert.deepEqual(left.draft(), right.draft());
  assert.deepEqual(left.effective(), right.effective());
  assert.deepEqual(left.dirty(), right.dirty());
  assert.deepEqual(left.patchPlan(), right.patchPlan());
  assert.deepEqual(left.readiness(), right.readiness());
  assert.deepEqual(left.validation(), right.validation());
  assert.deepEqual(left.availability(), right.availability());
  assert.deepEqual(left.admission(), right.admission());
  assert.deepEqual(left.actionReadiness("submit"), right.actionReadiness("submit"));
  assert.equal(left.actionPlan("submit").planDigest, right.actionPlan("submit").planDigest);
}

test("signals.form replayExactResourceSource records exact replay without mutating local draft truth", async () => {
  await withSignals((signals) => {
    const source = createReadOnlyResourceLineFixture({
      status: Object.freeze({ kind: "fulfilled", operation: "initialLoad" }),
      freshness: Object.freeze({ kind: "fresh" }),
    });
    const form = signals.form({
      source: signals.form.source.resourceLine(source, { id: "task-resource-replay" }),
      fields: ({ field }) => ({
        title: field("title"),
      }),
    });

    form.fields.title.set("Local draft");
    const replay = form.replayExactResourceSource();

    assert.equal(replay.mode, "resourceReplayExact");
    assert.equal(replay.resultKind, "replayed");
    assert.equal(replay.resourceReplayRestore.kind, "replayed");
    assert.equal(replay.resourceReplayRestore.mode, "SameRuntimeSignalExact");
    assert.deepEqual(form.source(), { title: "Resource task" });
    assert.deepEqual(form.draft(), { title: "Local draft" });
    assert.deepEqual(form.effective(), { title: "Local draft" });
    assert.equal(form.replayRestoreHistory().length, 1);
    assert.equal(
      form.replayRestoreHistory()[0].replayRestoreDigest,
      replay.replayRestoreDigest,
    );
    assert.equal(form.diagnostics().replayRestoreHistory.length, 1);
    assert.equal(typeof form.verification().digests.replayRestoreDigest, "string");
    assert.equal(
      form.verification().digests.replayRestoreHistoryDigest,
      form.verification().replayRestoreHistory.digest,
    );
    assert.equal(form.verification().performanceEnvelope.replayRestoreOperations, 1);
  });
});

test("signals.form replayExactResourceSource preserves the same operational truth as an equivalent forward-only draft", async () => {
  await withSignals((signals) => {
    const replaySource = createReadOnlyResourceLineFixture({
      status: Object.freeze({ kind: "fulfilled", operation: "initialLoad" }),
      freshness: Object.freeze({ kind: "fresh" }),
    });
    const replayedForm = signals.form({
      source: signals.form.source.resourceLine(replaySource, { id: "task-resource-replay-equivalent" }),
      fields: ({ field }) => ({
        title: field("title"),
      }),
    });

    const forwardOnlySource = createReadOnlyResourceLineFixture({
      status: Object.freeze({ kind: "fulfilled", operation: "initialLoad" }),
      freshness: Object.freeze({ kind: "fresh" }),
    });
    const forwardOnlyForm = signals.form({
      source: signals.form.source.resourceLine(forwardOnlySource, { id: "task-resource-replay-forward" }),
      fields: ({ field }) => ({
        title: field("title"),
      }),
    });

    replayedForm.fields.title.set("Local draft");
    forwardOnlyForm.fields.title.set("Local draft");
    replayedForm.replayExactResourceSource();

    assertEquivalentOperationalTruth(replayedForm, forwardOnlyForm);
    assert.equal(replayedForm.replayRestoreHistory().length, 1);
    assert.equal(forwardOnlyForm.replayRestoreHistory().length, 0);
  });
});

test("signals.form restoreExactResourceSource restores resource line source history and preserves local draft truth", async () => {
  await withSignals((signals) => {
    const source = createDetailPatchLineFixture({
      effectProfile: signals.resource.effects.branchNative(),
      initialValue: {
        title: "Ship docs",
        status: "draft",
      },
    });
    const form = signals.form({
      source: signals.form.source.resourceLine(source, { id: "task-resource-restore" }),
      fields: ({ field }) => ({
        title: field("title"),
        status: field("status"),
      }),
    });

    form.fields.title.set("Published docs");
    const execution = form.executeAction("submit");
    assert.equal(execution.resultKind, "fulfilled");
    assert.equal(form.resourceSource().visibleSelection.kind, "speculative");

    form.fields.title.set("Local draft after submit");
    const restore = form.restoreExactResourceSource();

    assert.equal(restore.mode, "resourceRestoreExact");
    assert.equal(restore.resultKind, "restored");
    assert.equal(restore.resourceReplayRestore.kind, "restored");
    assert.equal(restore.resourceReplayRestore.mode, "SameRuntimeBranchExact");
    assert.deepEqual(form.source(), {
      title: "Ship docs",
      status: "draft",
    });
    assert.deepEqual(form.draft(), {
      title: "Local draft after submit",
    });
    assert.deepEqual(form.effective(), {
      title: "Local draft after submit",
      status: "draft",
    });
    assert.equal(form.resourceSource().visibleSelection.kind, "restored");
    assert.equal(form.replayRestoreHistory().length, 1);
    assert.equal(typeof form.verification().digests.replayRestoreDigest, "string");
    assert.equal(
      form.verification().digests.replayRestoreHistoryDigest,
      form.verification().replayRestoreHistory.digest,
    );
    assert.equal(form.verification().performanceEnvelope.replayRestoreOperations, 1);
  });
});

test("signals.form restoreExactResourceSource preserves the same operational truth as an equivalent restored baseline plus local draft", async () => {
  await withSignals((signals) => {
    const source = createDetailPatchLineFixture({
      effectProfile: signals.resource.effects.branchNative(),
      initialValue: {
        title: "Ship docs",
        status: "draft",
      },
    });
    const restoredForm = signals.form({
      source: signals.form.source.resourceLine(source, { id: "task-resource-restore-equivalent" }),
      fields: ({ field }) => ({
        title: field("title"),
        status: field("status"),
      }),
    });

    const forwardOnlySource = createDetailPatchLineFixture({
      effectProfile: signals.resource.effects.branchNative(),
      initialValue: {
        title: "Ship docs",
        status: "draft",
      },
    });
    const forwardOnlyForm = signals.form({
      source: signals.form.source.resourceLine(forwardOnlySource, { id: "task-resource-restore-forward" }),
      fields: ({ field }) => ({
        title: field("title"),
        status: field("status"),
      }),
    });

    restoredForm.fields.title.set("Published docs");
    const execution = restoredForm.executeAction("submit");
    assert.equal(execution.resultKind, "fulfilled");
    restoredForm.fields.title.set("Local draft after submit");
    forwardOnlyForm.fields.title.set("Local draft after submit");

    restoredForm.restoreExactResourceSource();

    assertEquivalentOperationalTruth(restoredForm, forwardOnlyForm);
    assert.equal(restoredForm.resourceSource().visibleSelection.kind, "restored");
    assert.equal(restoredForm.replayRestoreHistory().length, 1);
    assert.equal(forwardOnlyForm.replayRestoreHistory().length, 0);
  });
});

test("signals.form keeps exact replay and exact restore explicit when resource authority is unavailable", async () => {
  await withSignals((signals) => {
    const form = signals.form({
      source: { title: "Plain source" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
    });

    const replay = form.replayExactResourceSource();
    const restore = form.restoreExactResourceSource();

    assert.equal(replay.resultKind, "unavailable");
    assert.equal(replay.resourceReplayRestore.kind, "unavailable");
    assert.equal(replay.resourceReplayRestore.reason, "resourceSourceUnavailable");
    assert.equal(restore.resultKind, "unavailable");
    assert.equal(restore.resourceReplayRestore.kind, "unavailable");
    assert.equal(restore.resourceReplayRestore.reason, "resourceSourceUnavailable");
    assert.equal(form.replayRestoreHistory().length, 2);
    assert.equal(form.verification().performanceEnvelope.replayRestoreOperations, 2);
    assert.equal(
      form.verification().digests.replayRestoreHistoryDigest,
      form.verification().replayRestoreHistory.digest,
    );
  });
});

test("signals.form keeps retained-history unavailability explicit without rewriting current truth", async () => {
  await withSignals((signals) => {
    const source = createReadOnlyResourceLineFixture({
      status: Object.freeze({ kind: "fulfilled", operation: "initialLoad" }),
      freshness: Object.freeze({ kind: "fresh" }),
      replayExactResult: Object.freeze({
        kind: "unavailable",
        reason: "exactHistoryUnavailable",
        detail: "resource line replay history was evicted before exact replay could be performed",
        basisCurrentId: "basis-9",
        basisAdvanceCount: 3,
      }),
      restoreExactResult: Object.freeze({
        kind: "unavailable",
        reason: "exactHistoryUnavailable",
        detail: "resource line branch history was evicted before exact restore could be performed",
        basisCurrentId: "basis-9",
        basisAdvanceCount: 3,
      }),
    });
    const form = signals.form({
      source: signals.form.source.resourceLine(source, { id: "task-resource-history-unavailable" }),
      fields: ({ field }) => ({
        title: field("title"),
      }),
    });

    form.fields.title.set("Local draft");
    const before = {
      source: form.source(),
      draft: form.draft(),
      effective: form.effective(),
      dirty: form.dirty(),
      patchPlan: form.patchPlan(),
      readiness: form.readiness(),
    };

    const replay = form.replayExactResourceSource({ reason: "history was evicted" });
    const restore = form.restoreExactResourceSource({ reason: "history was evicted" });

    assert.equal(replay.resultKind, "unavailable");
    assert.equal(replay.resourceReplayRestore.kind, "unavailable");
    assert.equal(replay.resourceReplayRestore.reason, "exactHistoryUnavailable");
    assert.equal(restore.resultKind, "unavailable");
    assert.equal(restore.resourceReplayRestore.kind, "unavailable");
    assert.equal(restore.resourceReplayRestore.reason, "exactHistoryUnavailable");
    assert.deepEqual(form.source(), before.source);
    assert.deepEqual(form.draft(), before.draft);
    assert.deepEqual(form.effective(), before.effective);
    assert.deepEqual(form.dirty(), before.dirty);
    assert.deepEqual(form.patchPlan(), before.patchPlan);
    assert.deepEqual(form.readiness(), before.readiness);
    assert.equal(form.replayRestoreHistory().length, 2);
    assert.equal(form.verification().performanceEnvelope.replayRestoreOperations, 2);
  });
});
