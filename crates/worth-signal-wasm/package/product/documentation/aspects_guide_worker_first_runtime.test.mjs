import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadSignalsModule } from "../signals_runtime/module_loading/load_signals_module.mjs";

const TransferAspect = {
  financialTerms: 0,
  operatorNote: 1,
};

test("the aspects guide proves semantic invalidation on worker-first empty-root authoring", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  let signals = null;
  try {
    signals = await createSignals({ deployment: "workerFirst" });

    const transfer = signals.spec.input(
      "documentationTransfer",
      { amount: 8_000, note: "Standard vendor invoice" },
      {
        producesAspects: [
          TransferAspect.financialTerms,
          TransferAspect.operatorNote,
        ],
      },
    );
    const reviewLane = signals.spec.computed("documentationReviewLane", {
      reads: [{ id: transfer.id, aspect: TransferAspect.financialTerms }],
      expr: {
        kind: "if",
        condition: {
          kind: "gte",
          left: {
            kind: "get",
            target: { kind: "read", id: transfer.id },
            field: "amount",
          },
          right: { kind: "value", value: 10_000 },
        },
        thenExpr: { kind: "value", value: "Manual review" },
        elseExpr: { kind: "value", value: "Automatic" },
      },
      identity: { kind: "exact" },
    });
    const notePreview = signals.spec.computed("documentationNotePreview", {
      reads: [{ id: transfer.id, aspect: TransferAspect.operatorNote }],
      expr: {
        kind: "get",
        target: { kind: "read", id: transfer.id },
        field: "note",
      },
      identity: { kind: "exact" },
    });

    await signals.settleAuthoredWork();
    assert.equal(reviewLane(), "Automatic");
    assert.equal(notePreview(), "Standard vendor invoice");

    await signals.transaction((tx) => {
      tx.setWithAspects(
        transfer,
        { ...transfer(), note: "Urgent vendor invoice" },
        [TransferAspect.operatorNote],
      );
    });
    await signals.settleAuthoredWork();

    const latest = signals.diagnostics().latestFlow();
    assert.deepEqual(
      latest?.flow.change.changed_aspects,
      [TransferAspect.operatorNote],
    );
    assert.equal(latest?.flow.invalidation.invalidated_direct_subscribers, 1);
    assert.equal(reviewLane(), "Automatic");
    assert.equal(notePreview(), "Urgent vendor invoice");
  } finally {
    if (signals) {
      await signals.terminate();
    }
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});
