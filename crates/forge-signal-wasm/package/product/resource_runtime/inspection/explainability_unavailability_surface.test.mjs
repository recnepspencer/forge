import assert from "node:assert/strict";
import test from "node:test";

import { loadResourceModule } from "../module_loading/load_resource_module.mjs";
import { createFakeSignalNamespace } from "../runtime_fixture/fake_signal_namespace.mjs";

test("diagnostics summary and history surface retained explainability loss as named unavailable artifacts", async () => {
  const mod = await loadResourceModule();
  try {
    let replayReads = 0;
    let lineageReads = 0;
    const signalNamespace = createFakeSignalNamespace("root", {
      replay_for() {
        replayReads += 1;
        throw new Error("history() should not call replay_for(...) when availability denies it");
      },
      replay_availability_for(signalId) {
        return {
          kind: "unavailable",
          reason: "runtimeRejected",
          detail: `retained replay history for ${signalId} was truncated`,
        };
      },
      lineage_for() {
        lineageReads += 1;
        throw new Error("history() should not call lineage_for(...) when availability denies it");
      },
      lineage_availability_for() {
        throw new Error("retained lineage history was truncated");
      },
    });
    const resource = mod.createResourceNamespace(signalNamespace, {});
    const detail = resource.detail({
      params: mod.resourceParams(),
      normalizeParams: ({ id }) => mod.resourceParamIdentity({ id }, id),
      load: ({ id }) => ({ id }),
    });

    const line = detail.line({ id: "retained" });
    const summary = line.diagnosticsSummary();
    const history = line.history();

    assert.equal(replayReads, 0);
    assert.equal(lineageReads, 0);
    assert.deepEqual(summary.explainability, history.availability);
    assert.deepEqual(history.replay, null);
    assert.deepEqual(history.lineage, null);
    assert.deepEqual(history.availability.lineage, {
      kind: "unavailable",
      reason: "runtimeRejected",
      detail:
        "resource line lineage history is unavailable because lineage_availability_for(...) rejected explainability: retained lineage history was truncated",
    });
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
    assert.equal(history.availability.replay.kind, "unavailable");
    assert.equal(history.availability.replay.reason, "runtimeRejected");
    assert.equal(
      history.availability.replay.detail.endsWith(" was truncated"),
      true,
    );
  } finally {
    await mod.cleanup();
  }
});

test("history downgrades replay and lineage artifacts when rich reads reject at materialization time", async () => {
  const mod = await loadResourceModule();
  try {
    let replayReads = 0;
    let lineageReads = 0;
    const signalNamespace = createFakeSignalNamespace("root", {
      replay_for() {
        replayReads += 1;
        throw new Error("replay artifact was evicted");
      },
      lineage_for() {
        lineageReads += 1;
        throw new Error("lineage artifact was evicted");
      },
    });
    const resource = mod.createResourceNamespace(signalNamespace, {});
    const detail = resource.detail({
      params: mod.resourceParams(),
      normalizeParams: ({ id }) => mod.resourceParamIdentity({ id }, id),
      load: ({ id }) => ({ id }),
    });

    const history = detail.line({ id: "evicted" }).history();

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
    await mod.cleanup();
  }
});

test("branch explainability rejection becomes explicit history and restore unavailability", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace("root", {
      current_branch() {
        throw new Error("retained branch snapshots are unavailable");
      },
      restore_exact_branch_snapshot() {},
    });
    const resource = mod.createResourceNamespace(signalNamespace, {});
    const detail = resource.detail({
      params: mod.resourceParams(),
      normalizeParams: ({ id }) => mod.resourceParamIdentity({ id }, id),
      load: ({ id }) => ({ id }),
    });

    const line = detail.line({ id: "branch-retained" });
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
    await mod.cleanup();
  }
});
