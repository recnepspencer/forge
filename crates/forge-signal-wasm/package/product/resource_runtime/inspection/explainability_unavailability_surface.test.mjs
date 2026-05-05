import assert from "node:assert/strict";
import test from "node:test";

import {
  createRealResourceNamespace,
  createRealResourceRuntime,
} from "../runtime_fixture/real_resource_signals.mjs";

function createDetailLine(resourceMod, signals, historyOverrides = null, id = "detail") {
  return createRealResourceNamespace(resourceMod, signals, historyOverrides)
    .detail({
      params: resourceMod.resourceParams(),
      normalizeParams: ({ id }) => resourceMod.resourceParamIdentity({ id }, id),
      load: ({ id }) => ({ id }),
    })
    .line({ id });
}

test("diagnostics summary and history surface stay aligned when branch explainability is unsupported by the real runtime boundary", async () => {
  const runtime = await createRealResourceRuntime();
  try {
    const line = createDetailLine(
      runtime.resourceMod,
      runtime.signals,
      {
        current_branch: undefined,
      },
      "retained",
    );

    const summary = line.diagnosticsSummary();
    const history = line.history();

    assert.deepEqual(summary.explainability, history.availability);
    assert.deepEqual(history.availability.branch, {
      kind: "unavailable",
      reason: "unsupportedByRuntime",
      detail:
        "resource line branch history is unavailable because the Signals runtime does not expose current_branch(...)",
    });
    assert.deepEqual(history.availability.restoreExact, {
      kind: "unavailable",
      reason: "unsupportedByRuntime",
      detail:
        "resource line exact branch restore is unavailable because the Signals runtime does not expose current_branch(...)",
    });
  } finally {
    await runtime.cleanup();
  }
});

test("history downgrades replay and lineage artifacts when real rich reads reject at materialization time", async () => {
  const runtime = await createRealResourceRuntime();
  try {
    let replayReads = 0;
    let lineageReads = 0;
    const history = createDetailLine(
      runtime.resourceMod,
      runtime.signals,
      {
        replay_for() {
          replayReads += 1;
          throw new Error("replay artifact was evicted");
        },
        lineage_for() {
          lineageReads += 1;
          throw new Error("lineage artifact was evicted");
        },
      },
      "evicted",
    ).history();

    assert.equal(replayReads, 1);
    assert.equal(lineageReads, 1);
    assert.equal(history.replay, null);
    assert.equal(history.lineage, null);
    assert.deepEqual(history.availability.replay, {
      kind: "unavailable",
      reason: "runtimeRejected",
      detail:
        "resource line replay history is unavailable because replay_for(...) rejected explainability: replay artifact was evicted",
    });
    assert.deepEqual(history.availability.lineage, {
      kind: "unavailable",
      reason: "runtimeRejected",
      detail:
        "resource line lineage history is unavailable because lineage_for(...) rejected explainability: lineage artifact was evicted",
    });
  } finally {
    await runtime.cleanup();
  }
});

test("branch explainability rejection becomes explicit history and restore unavailability", async () => {
  const runtime = await createRealResourceRuntime();
  try {
    const line = createDetailLine(
      runtime.resourceMod,
      runtime.signals,
      {
        current_branch() {
          throw new Error("retained branch snapshots are unavailable");
        },
      },
      "branch-retained",
    );

    const history = line.history();
    const summary = line.diagnosticsSummary();

    assert.equal(history.branch, null);
    assert.deepEqual(history.availability.branch, {
      kind: "unavailable",
      reason: "runtimeRejected",
      detail:
        "resource line branch history is unavailable because current_branch(...) failed: retained branch snapshots are unavailable",
    });
    assert.deepEqual(history.availability.restoreExact, {
      kind: "unavailable",
      reason: "runtimeRejected",
      detail:
        "resource line exact branch restore is unavailable because branch explainability could not be read: resource line branch history is unavailable because current_branch(...) failed: retained branch snapshots are unavailable",
    });
    assert.deepEqual(summary.explainability, history.availability);
  } finally {
    await runtime.cleanup();
  }
});
