import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../../../runtime_fixture/graph_operational_runtime.mjs";
import { createReadOnlyResourceLineFixture } from "../fixtures/resource_line_fixture.mjs";

test("signals.form normalizes resource line visible selection proof across committed confirmed restored and merged truth", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());

    const confirmedForm = signals.form({
      source: signals.form.source.resourceLine(
        createReadOnlyResourceLineFixture({
          status: { kind: "fulfilled", operation: "delivery" },
          freshness: { kind: "fresh" },
          visibleSelection: Object.freeze({
            kind: "confirmed",
            source: "delivery",
            effectId: "effect-1",
            branchId: 7,
            snapshotId: null,
            basisId: "basis-2",
            confirmationKind: "consumedCanonicalServerTruth",
            previousEffectId: "effect-0",
            detail: "resource line visible truth is showing confirmed branch-backed delivery truth",
          }),
        }),
        { id: "resource-task-confirmed-selection" },
      ),
      fields: ({ field }) => ({ title: field("title") }),
    });
    assert.equal(confirmedForm.resourceSource().visibleSelection.kind, "confirmed");
    assert.equal(confirmedForm.resourceSource().visibleSelection.branchProof.admitted, true);
    assert.equal(confirmedForm.resourceSource().visibleSelection.rebaseProof.admitted, false);
    assert.equal(
      confirmedForm.resourceSource().visibleSelection.rebaseProof.reason,
      "resource line visible selection does not carry admitted merge/rebase-visible proof",
    );

    const restoredForm = signals.form({
      source: signals.form.source.resourceLine(
        createReadOnlyResourceLineFixture({
          status: { kind: "fulfilled", operation: "restore" },
          freshness: { kind: "fresh" },
          visibleSelection: Object.freeze({
            kind: "restored",
            source: "exactBranchRestore",
            effectId: null,
            branchId: 7,
            snapshotId: 11,
            basisId: "basis-1",
            rollbackKind: "exactBranchRestoreAvailable",
            detail: "resource line visible truth was restored through exact branch restore",
          }),
        }),
        { id: "resource-task-restored-selection" },
      ),
      fields: ({ field }) => ({ title: field("title") }),
    });
    assert.equal(restoredForm.resourceSource().visibleSelection.kind, "restored");
    assert.equal(restoredForm.resourceSource().visibleSelection.branchProof.admitted, true);
    assert.equal(restoredForm.resourceSource().visibleSelection.rebaseProof.admitted, false);

    const mergedForm = signals.form({
      source: signals.form.source.resourceLine(
        createReadOnlyResourceLineFixture({
          status: { kind: "fulfilled", operation: "refresh" },
          freshness: { kind: "fresh" },
          visibleSelection: Object.freeze({
            kind: "merged",
            source: "refresh",
            effectId: null,
            branchId: 7,
            snapshotId: null,
            basisId: "basis-2",
            detail: "resource line visible truth advanced through admitted merged branch proof",
          }),
        }),
        { id: "resource-task-merged-selection" },
      ),
      fields: ({ field }) => ({ title: field("title") }),
    });
    assert.equal(mergedForm.resourceSource().visibleSelection.kind, "merged");
    assert.equal(mergedForm.resourceSource().visibleSelection.branchProof.admitted, true);
    assert.equal(mergedForm.resourceSource().visibleSelection.rebaseProof.admitted, true);
    assert.equal(
      mergedForm.verification().digests.resourceVisibleBranchSelectionDigest,
      mergedForm.resourceSource().visibleSelection.digest,
    );

    const committedForm = signals.form({
      source: signals.form.source.resourceLine(
        createReadOnlyResourceLineFixture({
          status: {
            kind: "rejected",
            operation: "refresh",
            message: "network down",
            continuity: "preservedVisibleValue",
          },
          freshness: { kind: "fresh" },
        }),
        { id: "resource-task-committed-selection" },
      ),
      fields: ({ field }) => ({ title: field("title") }),
    });
    assert.equal(committedForm.resourceSource().visibleSelection.kind, "committed");
    assert.equal(committedForm.resourceSource().visibleSelection.branchProof.admitted, false);
    assert.equal(
      committedForm.resourceSource().visibleSelection.branchProof.reason,
      "resource line visible selection does not carry admitted native branch-visible proof",
    );
  } finally {
    await cleanup();
  }
});
