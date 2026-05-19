import assert from "node:assert/strict";
import test from "node:test";

import { withSignals } from "../../action_execution_test_helpers.mjs";
import {
  createDetailPatchLineFixture,
  createReadOnlyResourceLineFixture,
} from "../fixtures/resource_line_fixture.mjs";

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
