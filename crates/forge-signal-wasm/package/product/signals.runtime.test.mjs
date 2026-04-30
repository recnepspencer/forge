import assert from "node:assert/strict";
import { mkdir, mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const productDir = path.dirname(fileURLToPath(import.meta.url));
const packageDir = path.dirname(productDir);

async function loadSignalsModule() {
  const tempDir = await mkdtemp(path.join(tmpdir(), "forge-signal-product-"));
  try {
    const filesToCopy = [
      ["product/signals.js", "product/signals.js"],
      ["product/callback_frames.js", "product/callback_frames.js"],
      ["product/diagnostics.js", "product/diagnostics.js"],
      ["product/host_capability_reports.js", "product/host_capability_reports.js"],
      ["product/host_capabilities.js", "product/host_capabilities.js"],
      ["product/history.js", "product/history.js"],
      ["product/handles.js", "product/handles.js"],
      ["product/specialist.js", "product/specialist.js"],
      ["product/transactions.js", "product/transactions.js"],
      ["product/symbols.js", "product/symbols.js"],
    ];

    for (const [sourceRelativePath, outputRelativePath] of filesToCopy) {
      const sourcePath = path.join(packageDir, sourceRelativePath);
      const targetPath = path.join(tempDir, outputRelativePath);
      await mkdir(path.dirname(targetPath), { recursive: true });
      await writeFile(targetPath, await readFile(sourcePath, "utf8"), "utf8");
    }

    await writeFile(
      path.join(tempDir, "raw_surface.js"),
      "export function createRawSignals() { throw new Error('createRawSignals should not be used in signals product runtime tests'); }\n",
      "utf8",
    );

    const moduleUrl = new URL(`file:///${path.join(tempDir, "product", "signals.js").replace(/\\/g, "/")}`);
    const loaded = await import(moduleUrl.href);
    return { ...loaded, cleanup: () => rm(tempDir, { recursive: true, force: true }) };
  } catch (error) {
    await rm(tempDir, { recursive: true, force: true });
    throw error;
  }
}

function createRawReadableHandle(id, value) {
  return {
    id,
    get() {
      return value;
    },
    peek() {
      return value;
    },
    free() {},
  };
}

test("wrapSignals supports symmetric metadata-style input/computed/output authoring", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const calls = [];
    const rawSignals = {
      input(id, initial, options) {
        calls.push(["input", id, initial, options]);
        return createRawReadableHandle(id, initial);
      },
      computedSpec(id, spec) {
        calls.push(["computedSpec", id, spec]);
        return createRawReadableHandle(id, { spec });
      },
      computedCallback(id, callback) {
        calls.push(["computedCallback", id, callback()]);
        return createRawReadableHandle(id, id.length);
      },
      outputSpec(id, spec) {
        calls.push(["outputSpec", id, spec]);
        return createRawReadableHandle(id, { spec });
      },
      read(target) {
        return typeof target === "string" ? target : target.id;
      },
      watch() {
        throw new Error("watch not needed");
      },
      effect() {
        throw new Error("effect not needed");
      },
      transaction() {
        throw new Error("transaction not needed");
      },
      batch() {
        throw new Error("batch not needed");
      },
      nuke() {
        return true;
      },
      diagnostics() {
        throw new Error("diagnostics not needed");
      },
      history() {
        throw new Error("history not needed");
      },
      specialist() {
        throw new Error("specialist not needed");
      },
      adapters() {
        throw new Error("adapters not needed");
      },
      compatibilityApp() {
        throw new Error("compatibilityApp not needed");
      },
      compatibilityRuntime() {
        throw new Error("compatibilityRuntime not needed");
      },
      free() {},
    };

    const signals = wrapSignals(rawSignals);
    const doubledSpec = { expr: { kind: "value", value: 2 } };
    const labelSpec = { expr: { kind: "value", value: "label" } };

    const count = signals.input(1, { id: "count", producesAspects: [1] });
    const doubled = signals.computed(doubledSpec, { id: "doubled" });
    const label = signals.output(labelSpec, { id: "label" });
    const callbackLabel = signals.output(() => "callback-label", { id: "callbackLabel" });
    const generated = signals.computed(() => count() + 1, { id: "generated" });

    assert.equal(count.id, "count");
    assert.equal(doubled.id, "doubled");
    assert.equal(label.id, "label");
    assert.equal(callbackLabel.id, "callbackLabel");
    assert.equal(generated.id, "generated");
    assert.deepEqual(label(), { spec: labelSpec });

    assert.deepEqual(calls[0], ["input", "count", 1, { producesAspects: [1] }]);
    assert.deepEqual(calls[1], ["computedSpec", "doubled", doubledSpec]);
    assert.deepEqual(calls[2], ["outputSpec", "label", labelSpec]);
    assert.equal(calls[3][0], "computedCallback");
    assert.equal(calls[3][1], "__forgeSignal.outputProjection.callbackLabel.1");
    assert.deepEqual(calls[4], [
      "outputSpec",
      "callbackLabel",
      {
        reads: ["__forgeSignal.outputProjection.callbackLabel.1"],
        expr: {
          kind: "read",
          id: "__forgeSignal.outputProjection.callbackLabel.1",
        },
      },
    ]);
    assert.equal(calls[5][0], "computedCallback");
    assert.equal(calls[5][1], "generated");
  } finally {
    await cleanup();
  }
});

test("wrapSignals keeps callback forms and rejects malformed metadata mixes", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const calls = [];
    const rawSignals = {
      input(id, initial, options) {
        calls.push(["input", id, initial, options]);
        return createRawReadableHandle(id, initial);
      },
      computedSpec(id, spec) {
        calls.push(["computedSpec", id, spec]);
        return createRawReadableHandle(id, spec);
      },
      computedCallback(id, callback) {
        calls.push(["computedCallback", id, typeof callback]);
        return createRawReadableHandle(id, id);
      },
      outputSpec(id, spec) {
        calls.push(["outputSpec", id, spec]);
        return createRawReadableHandle(id, spec);
      },
      read(target) {
        return typeof target === "string" ? target : target.id;
      },
      watch() {
        throw new Error("watch not needed");
      },
      effect() {
        throw new Error("effect not needed");
      },
      transaction() {
        throw new Error("transaction not needed");
      },
      batch() {
        throw new Error("batch not needed");
      },
      nuke() {
        return true;
      },
      diagnostics() {
        throw new Error("diagnostics not needed");
      },
      history() {
        throw new Error("history not needed");
      },
      specialist() {
        throw new Error("specialist not needed");
      },
      adapters() {
        throw new Error("adapters not needed");
      },
      compatibilityApp() {
        throw new Error("compatibilityApp not needed");
      },
      compatibilityRuntime() {
        throw new Error("compatibilityRuntime not needed");
      },
      free() {},
    };

    const signals = wrapSignals(rawSignals);
    const deferred = signals.output(() => 1, { id: "panel" });
    const explicit = signals.output("panelExplicit", () => 2);
    const namedComputed = signals.computed("named", () => 3);

    assert.equal(deferred.id, "panel");
    assert.equal(explicit.id, "panelExplicit");
    assert.equal(namedComputed.id, "named");

    assert.deepEqual(calls.slice(0, 5), [
      ["computedCallback", "__forgeSignal.outputProjection.panel.1", "function"],
      ["outputSpec", "panel", {
        reads: ["__forgeSignal.outputProjection.panel.1"],
        expr: {
          kind: "read",
          id: "__forgeSignal.outputProjection.panel.1",
        },
      }],
      ["computedCallback", "__forgeSignal.outputProjection.panelExplicit.2", "function"],
      ["outputSpec", "panelExplicit", {
        reads: ["__forgeSignal.outputProjection.panelExplicit.2"],
        expr: {
          kind: "read",
          id: "__forgeSignal.outputProjection.panelExplicit.2",
        },
      }],
      ["computedCallback", "named", "function"],
    ]);

    assert.throws(
      () => signals.input(1),
      /input options must be an object when provided/,
    );
    assert.throws(
      () => signals.computed({ expr: { kind: "value", value: 1 } }, {}),
      /computed metadata form requires a non-empty string id/,
    );
    assert.throws(
      () => signals.output("label", { expr: { kind: "value", value: 1 } }, { id: "extra" }),
      /output spec form does not accept a third argument/,
    );
    assert.throws(
      () => signals.output(() => 1, "panel"),
      /output callback options must be an object when provided/,
    );
  } finally {
    await cleanup();
  }
});

test("wrapSignals rejects raw handles, foreign-runtime handles, and non-input mutations", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const firstCalls = [];
    const secondCalls = [];

    function buildRawSignals(callLog) {
      return {
        input(id, initial, options) {
          callLog.push(["input", id, initial, options]);
          return createRawReadableHandle(id, initial);
        },
        computedSpec(id, spec) {
          callLog.push(["computedSpec", id, spec]);
          return createRawReadableHandle(id, spec);
        },
        computedCallback(id, callback) {
          callLog.push(["computedCallback", id, typeof callback]);
          return createRawReadableHandle(id, id);
        },
        outputSpec(id, spec) {
          callLog.push(["outputSpec", id, spec]);
          return createRawReadableHandle(id, spec);
        },
        read(target) {
          callLog.push(["read", target.id ?? target]);
          return typeof target === "string" ? target : target.id;
        },
        watch(target) {
          callLog.push(["watch", target.id ?? target]);
          return { free() {} };
        },
        effect(target) {
          callLog.push(["effect", target.id ?? target]);
          return { free() {} };
        },
        transaction(callback) {
          const ops = [];
          callback({
            set(target, value) {
              ops.push(["set", target.id, value]);
            },
            setWithAspects(target, value, aspects) {
              ops.push(["setWithAspects", target.id, value, aspects]);
            },
            setWithRegions(target, value, changedRegions) {
              ops.push(["setWithRegions", target.id, value, changedRegions]);
            },
            setWithRegionsAndAspects(target, value, changedRegions, aspects) {
              ops.push(["setWithRegionsAndAspects", target.id, value, changedRegions, aspects]);
            },
            free() {},
          });
          callLog.push(["transaction", ops]);
          return ops;
        },
        batch(callback) {
          return this.transaction(callback);
        },
        nuke() {
          return true;
        },
        diagnostics() {
          throw new Error("diagnostics not needed");
        },
        history() {
          throw new Error("history not needed");
        },
        specialist() {
          throw new Error("specialist not needed");
        },
        adapters() {
          throw new Error("adapters not needed");
        },
        compatibilityApp() {
          throw new Error("compatibilityApp not needed");
        },
        compatibilityRuntime() {
          throw new Error("compatibilityRuntime not needed");
        },
        free() {},
      };
    }

    const firstSignals = wrapSignals(buildRawSignals(firstCalls));
    const secondSignals = wrapSignals(buildRawSignals(secondCalls));

    const firstInput = firstSignals.input(1, { id: "count" });
    const secondInput = secondSignals.input(2, { id: "other" });
    const computed = firstSignals.computed({ expr: { kind: "value", value: 4 } }, { id: "double" });
    const rawHandle = createRawReadableHandle("raw", 9);

    assert.throws(
      () => firstSignals.read(rawHandle),
      /signals\.read expects a string id or a product signal handle created by this package/,
    );
    assert.throws(
      () => firstSignals.watch(secondInput, () => {}),
      /signals\.watch cannot use signal `other` from a different Signals runtime/,
    );
    assert.throws(
      () => firstSignals.effect(secondInput, () => {}),
      /signals\.effect cannot use signal `other` from a different Signals runtime/,
    );

    assert.throws(
      () => firstSignals.transaction((tx) => tx.set(computed, 4)),
      /transaction\.set expects an input handle, but received a computed handle for `double`/,
    );
    assert.throws(
      () => firstSignals.transaction((tx) => tx.set(secondInput, 4)),
      /transaction\.set cannot use signal `other` from a different Signals runtime/,
    );

    const commit = firstSignals.transaction((tx) => tx.set(firstInput, 7));
    assert.deepEqual(commit, [["set", "count", 7]]);
  } finally {
    await cleanup();
  }
});

test("wrapSignals exposes a typed specialist wrapper without dropping legacy expert methods", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const rawSignals = {
      input() {
        throw new Error("input not needed");
      },
      computedSpec() {
        throw new Error("computedSpec not needed");
      },
      computedCallback() {
        throw new Error("computedCallback not needed");
      },
      outputSpec() {
        throw new Error("outputSpec not needed");
      },
      read() {
        throw new Error("read not needed");
      },
      watch() {
        throw new Error("watch not needed");
      },
      effect() {
        throw new Error("effect not needed");
      },
      transaction() {
        throw new Error("transaction not needed");
      },
      batch() {
        throw new Error("batch not needed");
      },
      nuke() {
        return true;
      },
      diagnostics() {
        throw new Error("diagnostics not needed");
      },
      history() {
        throw new Error("history not needed");
      },
      specialist() {
        return {
          evaluate_dirty() {
            return { touchedNodes: 3, nodesEvaluated: 2 };
          },
          graph_summary() {
            return { profile: "Development", activeNodeCount: 4 };
          },
          read_versions(ids) {
            return ids.map((id, index) => ({ id, version: index + 1 }));
          },
          free() {},
        };
      },
      adapters() {
        throw new Error("adapters not needed");
      },
      compatibilityApp() {
        throw new Error("compatibilityApp not needed");
      },
      compatibilityRuntime() {
        throw new Error("compatibilityRuntime not needed");
      },
      free() {},
    };

    const signals = wrapSignals(rawSignals);
    const specialist = signals.specialist();

    assert.equal(specialist.graphSummary().profile, "Development");
    assert.equal(specialist.graph_summary().activeNodeCount, 4);
    assert.equal(specialist.evaluateDirty().touchedNodes, 3);
    assert.equal(specialist.evaluate_dirty().nodesEvaluated, 2);
    assert.deepEqual(specialist.readVersions(["a", "b"]), [
      { id: "a", version: 1 },
      { id: "b", version: 2 },
    ]);
    assert.deepEqual(specialist.read_versions(["c"]), [{ id: "c", version: 1 }]);
  } finally {
    await cleanup();
  }
});

test("wrapSignals history wrapper accepts numeric branch ids and normalizes preview requests", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const calls = [];
    const rawHistory = {
      current_branch() {
        return { id: 7, name: "main", parent_branch_id: null, head_snapshot_id: null };
      },
      replay_for_branch(branchId) {
        calls.push(["replay_for_branch", typeof branchId, branchId]);
        return { frames: [] };
      },
      branch_snapshot(branchId) {
        calls.push(["branch_snapshot", typeof branchId, branchId]);
        return { meta: { branch_id: 7 } };
      },
      branch_snapshot_wire(branchId) {
        calls.push(["branch_snapshot_wire", typeof branchId, branchId]);
        return JSON.stringify({ meta: { branch_id: 7 } });
      },
      branch_snapshot_portable_wire(branchId) {
        calls.push(["branch_snapshot_portable_wire", typeof branchId, branchId]);
        return JSON.stringify({ meta: { branch_id: 7 } });
      },
      branch_snapshot_envelope(branchId) {
        calls.push(["branch_snapshot_envelope", typeof branchId, branchId]);
        return { snapshot: { meta: { branch_id: 7 } }, state: { sources: [], recipes: [] } };
      },
      branch_snapshot_envelope_wire(branchId) {
        calls.push(["branch_snapshot_envelope_wire", typeof branchId, branchId]);
        return JSON.stringify({ snapshot: { meta: { branch_id: 7 } }, state: { sources: [], recipes: [] } });
      },
      branch_snapshot_envelope_portable_wire(branchId) {
        calls.push(["branch_snapshot_envelope_portable_wire", typeof branchId, branchId]);
        return JSON.stringify({ snapshot: { meta: { branch_id: 7 } }, state: { sources: [], recipes: [] } });
      },
      restore_snapshot(snapshot) {
        calls.push(["restore_snapshot", snapshot.snapshot.meta.branch_id]);
      },
      restore_snapshot_wire(snapshot) {
        calls.push(["restore_snapshot_wire", JSON.parse(snapshot).snapshot.meta.branch_id]);
      },
      restore_branch_snapshot(branchId, snapshot) {
        calls.push(["restore_branch_snapshot", typeof branchId, branchId, snapshot.meta.branch_id]);
      },
      restore_branch_snapshot_wire(branchId, snapshot) {
        calls.push(["restore_branch_snapshot_wire", typeof branchId, branchId, JSON.parse(snapshot).meta.branch_id]);
      },
      restore_branch_snapshot_portable_wire(branchId, snapshot) {
        calls.push(["restore_branch_snapshot_portable_wire", typeof branchId, branchId, JSON.parse(snapshot).meta.branch_id]);
      },
      branch_state_proof(branchId) {
        calls.push(["branch_state_proof", typeof branchId, branchId]);
        return { stateDigest: "digest" };
      },
      replay_parity_proof(expectedBranchId, replayedBranchId) {
        calls.push(["replay_parity_proof", typeof expectedBranchId, expectedBranchId, replayedBranchId]);
        return { parity: true, mismatch_classes: [] };
      },
      replay_artifact_proof(expected, replayedBranchId) {
        calls.push(["replay_artifact_proof", expected.branchStateDigest, typeof replayedBranchId, replayedBranchId]);
        return { parity: true, mismatch_classes: [] };
      },
      plan_merge_policy_preview(request) {
        calls.push(["plan_merge_policy_preview", request]);
        return { source_branch_id: request.source_branch_id };
      },
      plan_merge_policy_preview_with_proof(request) {
        calls.push(["plan_merge_policy_preview_with_proof", request]);
        return { plan: { source_branch_id: request.source_branch_id }, proof: { planDigest: "plan" } };
      },
      merge_branches_policy_preview(request) {
        calls.push(["merge_branches_policy_preview", request]);
        return { target_branch: request.target_branch_id };
      },
      merge_branches_policy_preview_with_proof(request) {
        calls.push(["merge_branches_policy_preview_with_proof", request]);
        return { result: { target_branch: request.target_branch_id }, proof: { resultDigest: "result" } };
      },
      free() {},
    };

    const rawSignals = {
      input(id, initial) {
        return createRawReadableHandle(id, initial);
      },
      computedSpec(id, spec) {
        return createRawReadableHandle(id, spec);
      },
      computedCallback(id) {
        return createRawReadableHandle(id, id);
      },
      outputSpec(id, spec) {
        return createRawReadableHandle(id, spec);
      },
      read(target) {
        return typeof target === "string" ? target : target.id;
      },
      watch() {
        return { free() {} };
      },
      effect() {
        return { free() {} };
      },
      transaction(callback) {
        callback({ set() {}, free() {} });
        return {};
      },
      batch(callback) {
        callback({ set() {}, free() {} });
        return {};
      },
      nuke() {
        return true;
      },
      diagnostics() {
        return {
          why() { return null; },
          health() { return null; },
          summaryNow() { return null; },
          historyNow() { return null; },
          latestObservation() { return null; },
          latestFlow() { return null; },
          performanceSummary() { return {}; },
          latestFailure() { return null; },
          latestRollback() { return null; },
          latestFrontierExecution() { return null; },
          latestInvalidationTraceRecords() { return []; },
          recentHistory() { return []; },
          subscribe() { return { free() {} }; },
        };
      },
      history() {
        return rawHistory;
      },
      specialist() {
        return {};
      },
      adapters() {
        return { free() {} };
      },
      compatibilityApp() {
        return {};
      },
      compatibilityRuntime() {
        return {};
      },
      free() {},
    };

    const signals = wrapSignals(rawSignals);
    const history = signals.history();
    const currentBranch = history.current_branch();
    const snapshot = history.branch_snapshot(currentBranch.id);
    const envelope = history.branch_snapshot_envelope(currentBranch.id);
    history.restore_exact_snapshot(envelope);
    history.restore_exact_branch_snapshot(currentBranch.id, snapshot);
    const proof = history.branch_state_proof(currentBranch.id);
    const parity = history.replay_parity_proof(currentBranch.id, currentBranch.id);
    const artifact = history.replay_artifact_proof({ branchStateDigest: proof.stateDigest }, currentBranch.id);
    const previewPlan = history.plan_merge_policy_preview({
      source_branch_id: currentBranch.id,
      target_branch_id: currentBranch.id,
    });
    const previewPlanProof = history.plan_merge_policy_preview_with_proof({
      source_branch_id: currentBranch.id,
      target_branch_id: currentBranch.id,
    });
    const previewResult = history.merge_branches_policy_preview({
      source_branch_id: currentBranch.id,
      target_branch_id: currentBranch.id,
    });
    const previewResultProof = history.merge_branches_policy_preview_with_proof({
      source_branch_id: currentBranch.id,
      target_branch_id: currentBranch.id,
    });

    assert.equal(previewPlan.source_branch_id, 7);
    assert.equal(previewPlanProof.proof.planDigest, "plan");
    assert.equal(previewResult.target_branch, 7);
    assert.equal(previewResultProof.proof.resultDigest, "result");
    assert.equal(parity.parity, true);
    assert.equal(artifact.parity, true);
    assert.equal(typeof snapshot.snapshotRestoreToken, "string");
    assert.equal(snapshot.snapshotRestoreMode, "SameRuntimeExact");
    assert.equal(typeof snapshot.snapshotPortableWire, "string");
    assert.equal(typeof envelope.snapshotEnvelopeRestoreToken, "string");
    assert.equal(envelope.snapshotEnvelopeRestoreMode, "SameRuntimeExact");
    assert.equal(typeof envelope.snapshotEnvelopePortableWire, "string");

    assert.deepEqual(calls, [
      ["branch_snapshot", "bigint", 7n],
      ["branch_snapshot_wire", "bigint", 7n],
      ["branch_snapshot_portable_wire", "bigint", 7n],
      ["branch_snapshot_envelope", "bigint", 7n],
      ["branch_snapshot_envelope_wire", "bigint", 7n],
      ["branch_snapshot_envelope_portable_wire", "bigint", 7n],
      ["restore_snapshot_wire", 7],
      ["restore_branch_snapshot_wire", "bigint", 7n, 7],
      ["branch_state_proof", "bigint", 7n],
      ["replay_parity_proof", "bigint", 7n, 7n],
      ["replay_artifact_proof", "digest", "bigint", 7n],
      ["plan_merge_policy_preview", { source_branch_id: 7, target_branch_id: 7 }],
      ["plan_merge_policy_preview_with_proof", { source_branch_id: 7, target_branch_id: 7 }],
      ["merge_branches_policy_preview", { source_branch_id: 7, target_branch_id: 7 }],
      ["merge_branches_policy_preview_with_proof", { source_branch_id: 7, target_branch_id: 7 }],
    ]);

    assert.throws(
      () => history.switch_branch(-1),
      /history\.switch_branch expects a non-negative safe integer branch id/,
    );
    assert.throws(
      () => history.plan_merge_policy_preview("bad"),
      /history\.plan_merge_policy_preview expects a merge preview request object/,
    );
    assert.throws(
      () => history.restore_exact_snapshot({ snapshot: { meta: { branch_id: 7 } }, state: { sources: [], recipes: [] } }),
      /history\.restore_exact_snapshot expects an artifact returned by history\.snapshot\(\) or history\.branch_snapshot_envelope\(\)/,
    );
    assert.throws(
      () => history.plan_merge_policy_preview({
        source_branch_id: 9007199254740992n,
        target_branch_id: currentBranch.id,
      }),
      /exceeds the safe integer range supported by merge preview requests/,
    );
  } finally {
    await cleanup();
  }
});

test("wrapSignals adapters wrapper marks same-runtime exact restore while preserving portable host-capability denial artifacts", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const rawEnvelope = {
      definitions: {
        policy: { preset: "WebDevelopment" },
        sources: [],
        recipes: [],
        sourceFamilies: [],
        recipeFamilies: [],
        unavailableCallbacks: [{
          id: "visibleLabel",
          signalKind: "computed",
          reason: "computeCallbackUnavailableForPortableExport",
          currentReads: ["count"],
          hostCapabilityReads: [{
            family: "visibility",
            registrationId: "visibility",
            compatibility: "LiveOnly",
          }],
          hostCapabilityTransports: [{
            family: "visibility",
            registrationId: "visibility",
            compatibility: "LiveOnly",
            exactRestoreOutcome: "Live",
            portableImportOutcome: "Denied",
            portableImportReason: "live-only host capabilities require the exact originating runtime",
          }],
        }],
      },
      snapshot: {
        snapshot: { meta: { branch_id: 0 } },
        state: { sources: [], recipes: [] },
      },
    };
    const calls = [];
    const rawSignals = {
      input(id, initial) {
        return createRawReadableHandle(id, initial);
      },
      computedSpec(id, spec) {
        return createRawReadableHandle(id, spec);
      },
      computedCallback(id) {
        return createRawReadableHandle(id, id);
      },
      outputSpec(id, spec) {
        return createRawReadableHandle(id, spec);
      },
      read(target) {
        return typeof target === "string" ? target : target.id;
      },
      watch() {
        return { free() {} };
      },
      effect() {
        return { free() {} };
      },
      transaction(callback) {
        callback({ set() {}, free() {} });
        return {};
      },
      batch(callback) {
        callback({ set() {}, free() {} });
        return {};
      },
      nuke() {
        return true;
      },
      diagnostics() {
        return {
          why() { return null; },
          health() { return null; },
          summaryNow() { return null; },
          historyNow() { return null; },
          latestObservation() { return null; },
          latestFlow() { return null; },
          performanceSummary() { return {}; },
          latestFailure() { return null; },
          latestRollback() { return null; },
          latestFrontierExecution() { return null; },
          latestInvalidationTraceRecords() { return []; },
          recentHistory() { return []; },
          subscribe() { return { free() {} }; },
        };
      },
      history() {
        return { free() {} };
      },
      specialist() {
        return {};
      },
      adapters() {
        return {
          export_definitions() {
            return rawEnvelope.definitions;
          },
          export_runtime_envelope() {
            return structuredClone(rawEnvelope);
          },
          export_runtime_envelope_wire() {
            return "restore-token";
          },
          export_runtime_envelope_portable_wire() {
            return "{\"portable\":true}";
          },
          replace_runtime_envelope(envelope) {
            calls.push(["replace_runtime_envelope", envelope.definitions.unavailableCallbacks[0].id]);
          },
          replace_runtime_envelope_portable_wire(envelope) {
            calls.push(["replace_runtime_envelope_portable_wire", envelope]);
          },
          replace_runtime_envelope_wire(token) {
            calls.push(["replace_runtime_envelope_wire", token]);
          },
          runtime_proof_report() {
            return { proofSchemaVersion: "1" };
          },
          free() {},
        };
      },
      compatibilityApp() {
        return {};
      },
      compatibilityRuntime() {
        return {};
      },
      free() {},
    };

    const signals = wrapSignals(rawSignals);
    const adapters = signals.adapters();
    const diagnostics = signals.diagnostics();
    const envelope = adapters.exportRuntimeEnvelope();
    const secondEnvelope = adapters.exportRuntimeEnvelope();
    const transportReport = adapters.hostCapabilityTransportReport(envelope);
    const implicitTransportReport = adapters.hostCapabilityTransportReport();

    assert.equal(envelope.runtimeEnvelopeRestoreToken, "restore-token");
    assert.equal(secondEnvelope.runtimeEnvelopeRestoreToken, "restore-token");
    assert.equal(envelope.runtimeEnvelopeRestoreMode, "SameRuntimeExact");
    assert.equal(envelope.runtimeEnvelopePortableWire, "{\"portable\":true}");
    assert.equal(
      envelope.definitions.unavailableCallbacks[0].hostCapabilityTransports[0].portableImportOutcome,
      "Denied",
    );
    assert.equal(
      envelope.definitions.unavailableCallbacks[0].hostCapabilityTransports[0].exactRestoreOutcome,
      "Live",
    );
    assert.equal(typeof transportReport.digest, "string");
    assert.equal(transportReport.totals.unavailableArtifactCount, 1);
    assert.deepEqual(transportReport.families[0]?.deniedCallbackIds, ["visibleLabel"]);
    assert.equal(typeof implicitTransportReport.digest, "string");
    assert.equal(implicitTransportReport.totals.unavailableArtifactCount, 1);

    assert.throws(
      () => adapters.replaceRuntimeEnvelope(envelope),
      (error) => error?.code === "computeCallbackUnavailableForRuntimeEnvelopeImport" &&
        error?.message === "runtime envelope import cannot restore callback-backed nodes without live callback registrations: visibleLabel",
    );
    assert.deepEqual(diagnostics.latestHostCapabilityEvent(), {
      sequence: 1,
      kind: "PortableImportDenied",
      family: "visibility",
      registrationId: "visibility",
      compatibility: "LiveOnly",
      invalidationMode: null,
      queuedInvalidationCount: 0,
      previousState: null,
      nextState: null,
      touchedNodes: 0,
      reevaluatedNodes: 0,
      portableImportOutcome: "Denied",
      portableImportReason: "live-only host capabilities require the exact originating runtime",
      deniedCallbackIds: ["visibleLabel"],
    });
    assert.equal(diagnostics.performanceSummary().hostCapabilityCompatibilityDenialCount, 1);
    assert.equal(diagnostics.performanceSummary().hostCapabilityUnavailabilityArtifactCount, 1);
    assert.equal(diagnostics.hostCapabilityReport().totals.compatibilityDenialCount, 1);
    assert.equal(typeof diagnostics.hostCapabilityReport().lineageDigest, "string");
    assert.equal(typeof diagnostics.hostCapabilityReport().breadthDigest, "string");
    adapters.restoreExactRuntimeEnvelope(envelope);

    assert.deepEqual(calls, [["replace_runtime_envelope_wire", "restore-token"]]);
    assert.throws(
      () => adapters.restoreExactRuntimeEnvelope(rawEnvelope),
      /adapters\.restoreExactRuntimeEnvelope expects an artifact returned by adapters\.exportRuntimeEnvelope\(\)/,
    );
  } finally {
    await cleanup();
  }
});
