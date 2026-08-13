/**
 * Worker-first three-part aspect contract:
 * producesAspects + reads:[{id,aspect}] + setWithAspects.
 * Covers empty-root authoring, negative-space values, importGraph, and denials.
 */
import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

const TransferAspect = Object.freeze({
  financialTerms: 0,
  operatorNote: 1,
});

async function withWorkerFirst(run) {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  let signals = null;
  try {
    signals = await createSignals({ deployment: "workerFirst" });
    await run({ signals, createSignals });
  } finally {
    if (signals) {
      await signals.terminate();
    }
    await cleanup();
    globalThis.Worker = previousWorker;
  }
}

function authorTransferGraph(signals, prefix) {
  const transfer = signals.spec.input(
    `${prefix}.transfer`,
    { amount: 8_000, note: "Standard vendor invoice" },
    {
      producesAspects: [
        TransferAspect.financialTerms,
        TransferAspect.operatorNote,
      ],
    },
  );
  const reviewLane = signals.spec.computed(`${prefix}.reviewLane`, {
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
  const notePreview = signals.spec.computed(`${prefix}.notePreview`, {
    reads: [{ id: transfer.id, aspect: TransferAspect.operatorNote }],
    expr: {
      kind: "get",
      target: { kind: "read", id: transfer.id },
      field: "note",
    },
    identity: { kind: "exact" },
  });
  const amountPanel = signals.spec.output(`${prefix}.amountPanel`, {
    reads: [{ id: transfer.id, aspect: TransferAspect.financialTerms }],
    expr: {
      kind: "get",
      target: { kind: "read", id: transfer.id },
      field: "amount",
    },
    identity: { kind: "exact" },
  });
  return { transfer, reviewLane, notePreview, amountPanel };
}

test("worker-first empty root admits aspect-filtered reads and proves negative-space fan-out", async () => {
  await withWorkerFirst(async ({ signals }) => {
    const { transfer, reviewLane, notePreview, amountPanel } = authorTransferGraph(
      signals,
      "empty",
    );
    await signals.settleAuthoredWork();
    assert.equal(reviewLane(), "Automatic");
    assert.equal(notePreview(), "Standard vendor invoice");
    assert.equal(amountPanel(), 8_000);

    await signals.transaction((tx) => {
      tx.setWithAspects(
        transfer,
        { amount: 8_000, note: "Urgent vendor invoice" },
        [TransferAspect.operatorNote],
      );
    });
    await signals.settleAuthoredWork();

    let latest = signals.diagnostics().latestFlow();
    assert.deepEqual(latest?.flow.change.changed_aspects, [TransferAspect.operatorNote]);
    assert.equal(latest?.flow.invalidation.invalidated_direct_subscribers, 1);
    assert.equal(reviewLane(), "Automatic");
    assert.equal(amountPanel(), 8_000);
    assert.equal(notePreview(), "Urgent vendor invoice");

    await signals.transaction((tx) => {
      tx.setWithAspects(
        transfer,
        { amount: 12_500, note: "Urgent vendor invoice" },
        [TransferAspect.financialTerms],
      );
    });
    await signals.settleAuthoredWork();

    latest = signals.diagnostics().latestFlow();
    assert.deepEqual(latest?.flow.change.changed_aspects, [TransferAspect.financialTerms]);
    assert.equal(reviewLane(), "Manual review");
    assert.equal(amountPanel(), 12_500);
    // Critical honesty: host refresh must not pull a freshly evaluated note when
    // only financialTerms changed. Compat keeps this stale; worker-first must too.
    assert.equal(notePreview(), "Urgent vendor invoice");
    assert.equal(latest?.flow.invalidation.invalidated_direct_subscribers, 2);
  });
});

test("worker-first empty root keeps unrelated aspect readers stale across financial writes", async () => {
  await withWorkerFirst(async ({ signals }) => {
    const { transfer, reviewLane, notePreview, amountPanel } = authorTransferGraph(
      signals,
      "stale",
    );
    await signals.settleAuthoredWork();

    await signals.transaction((tx) => {
      tx.setWithAspects(
        transfer,
        { amount: 12_500, note: "Urgent vendor invoice" },
        [TransferAspect.financialTerms],
      );
    });
    await signals.settleAuthoredWork();

    const latest = signals.diagnostics().latestFlow();
    assert.deepEqual(latest?.flow.change.changed_aspects, [TransferAspect.financialTerms]);
    assert.equal(reviewLane(), "Manual review");
    assert.equal(amountPanel(), 12_500);
    assert.equal(notePreview(), "Standard vendor invoice");
    assert.equal(transfer().note, "Urgent vendor invoice");
    assert.equal(latest?.flow.invalidation.invalidated_direct_subscribers, 2);
  });
});

test("worker-first empty root still admits plain string-id reads beside aspect descriptors", async () => {
  await withWorkerFirst(async ({ signals }) => {
    const count = signals.spec.input("plain.count", 2);
    const doubled = signals.spec.computed("plain.doubled", {
      reads: [count.id],
      expr: {
        kind: "sum",
        args: [
          { kind: "read", id: count.id },
          { kind: "read", id: count.id },
        ],
      },
      identity: { kind: "exact" },
    });
    await signals.settleAuthoredWork();
    assert.equal(doubled(), 4);
    await count.set(5);
    await signals.settleAuthoredWork();
    assert.equal(doubled(), 10);
  });
});

test("worker-first empty root denies malformed aspect read descriptors", async () => {
  await withWorkerFirst(async ({ signals }) => {
    const transfer = signals.spec.input("deny.transfer", { amount: 1 }, {
      producesAspects: [0],
    });
    assert.throws(
      () =>
        signals.spec.computed("deny.bad", {
          reads: [{ aspect: 0 }],
          expr: { kind: "value", value: 1 },
          identity: { kind: "exact" },
        }),
      /non-empty signal id or read descriptor/u,
    );
    assert.throws(
      () =>
        signals.spec.computed("deny.unknown", {
          reads: [{ id: "missing.signal", aspect: 0 }],
          expr: { kind: "value", value: 1 },
          identity: { kind: "exact" },
        }),
      /not currently available/u,
    );
    assert.equal(transfer().amount, 1);
  });
});

test("worker-first importGraph preserves aspect-filtered reads from a compatibility definition", async () => {
  await withWorkerFirst(async ({ signals, createSignals }) => {
    const compatibility = await createSignals({ deployment: "mainThreadCompatibility" });
    try {
      const transfer = compatibility.spec.input(
        "imported.transfer",
        { amount: 8_000, note: "Standard vendor invoice" },
        {
          producesAspects: [
            TransferAspect.financialTerms,
            TransferAspect.operatorNote,
          ],
        },
      );
      const reviewLane = compatibility.spec.computed("imported.reviewLane", {
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
      const notePreview = compatibility.spec.computed("imported.notePreview", {
        reads: [{ id: transfer.id, aspect: TransferAspect.operatorNote }],
        expr: {
          kind: "get",
          target: { kind: "read", id: transfer.id },
          field: "note",
        },
        identity: { kind: "exact" },
      });
      const graph = compatibility.graph("importedAspectGraph", {
        inputs: { transfer: compatibility.publicInput(transfer) },
        outputs: { reviewLane, notePreview },
      });

      const imported = signals.importGraph(graph.exportDefinition(), graph.exportSnapshot());
      await imported.ready();

      const importedTransfer = imported.input("transfer");
      const importedReview = imported.output("reviewLane");
      const importedNote = imported.output("notePreview");
      assert.equal(importedReview(), "Automatic");
      assert.equal(importedNote(), "Standard vendor invoice");

      await signals.transaction((tx) => {
        tx.setWithAspects(
          importedTransfer,
          { amount: 12_500, note: "Urgent vendor invoice" },
          [TransferAspect.financialTerms],
        );
      });
      await signals.settleAuthoredWork();

      const latest = signals.diagnostics().latestFlow();
      assert.deepEqual(latest?.flow.change.changed_aspects, [TransferAspect.financialTerms]);
      assert.equal(latest?.flow.invalidation.invalidated_direct_subscribers, 1);
      assert.equal(importedReview(), "Manual review");
      assert.equal(importedNote(), "Standard vendor invoice");
      assert.equal(importedTransfer().note, "Urgent vendor invoice");
    } finally {
      compatibility.free();
    }
  });
});
