/**
 * Publish-gate smoke: empty worker-first roots must author via signals.spec.*
 * without importGraph. This is the QMS-shaped regression that shipped broken
 * before 1.4.2 — keep it on the packed tarball path, not only crate tests.
 */
export function buildEmptyRootSmokeSource(packageName) {
  return `import { Worker as NodeWorker } from "node:worker_threads";
import init, { createSignals } from "${packageName}";
import { createReactSignalsStore } from "${packageName}/react";

const previousWorker = globalThis.Worker;
globalThis.Worker = NodeWorker;

await init();
const signals = await createSignals({ deployment: "workerFirst" });
try {
  const count = signals.spec.input("publish.empty.count", 2);
  const doubled = signals.spec.computed("publish.empty.doubled", {
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
  const panel = signals.spec.output("publish.empty.panel", {
    reads: [doubled.id],
    expr: { kind: "read", id: doubled.id },
    identity: { kind: "exact" },
  });
  const callback = signals.spec.computedCallback(
    "publish.empty.cb",
    () => count() * 3,
  );
  const TransferAspect = { financialTerms: 0, operatorNote: 1 };
  const transfer = signals.spec.input(
    "publish.empty.transfer",
    { amount: 8_000, note: "Standard vendor invoice" },
    {
      producesAspects: [
        TransferAspect.financialTerms,
        TransferAspect.operatorNote,
      ],
    },
  );
  const notePreview = signals.spec.computed("publish.empty.notePreview", {
    reads: [{ id: transfer.id, aspect: TransferAspect.operatorNote }],
    expr: {
      kind: "get",
      target: { kind: "read", id: transfer.id },
      field: "note",
    },
    identity: { kind: "exact" },
  });
  const reviewLane = signals.spec.computed("publish.empty.reviewLane", {
    reads: [{ id: transfer.id, aspect: TransferAspect.financialTerms }],
    expr: {
      kind: "get",
      target: { kind: "read", id: transfer.id },
      field: "amount",
    },
    identity: { kind: "exact" },
  });
  if (count() !== 2 || doubled() !== 4 || panel() !== 4 || callback() !== 6) {
    throw new Error("empty-root spec authoring initial values failed");
  }
  if (notePreview() !== "Standard vendor invoice" || reviewLane() !== 8_000) {
    throw new Error("empty-root aspect-filtered reads failed to construct/evaluate");
  }

  await signals.settleAuthoredWork();
  await count.set(5);
  if (count() !== 5 || doubled() !== 10 || callback() !== 15) {
    throw new Error("empty-root spec.input mutation/settlement failed");
  }

  await signals.transaction((tx) => {
    tx.setWithAspects(
      transfer,
      { amount: 12_500, note: "Urgent vendor invoice" },
      [TransferAspect.financialTerms],
    );
  });
  await signals.settleAuthoredWork();
  if (reviewLane() !== 12_500) {
    throw new Error("empty-root financial aspect reader failed to refresh");
  }
  if (notePreview() !== "Standard vendor invoice") {
    throw new Error(
      "empty-root aspect filter collapsed: notePreview refreshed on financialTerms-only write",
    );
  }

  const form = signals.form({
    source: { title: "A" },
    fields: ({ field }) => ({ title: field("title") }),
    actions: ({ submit }) => ({ submit: submit() }),
  });
  await Promise.resolve(form.fields.title.set("B"));
  if (form.effective().title !== "B") {
    throw new Error("empty-root form field set failed");
  }
  const pending = form.executeAction("submit");
  if (pending?.resultKind !== "pending") {
    throw new Error("empty-root form executeAction must stay a sync receipt");
  }

  const store = createReactSignalsStore(signals);
  if (store.getSignalSnapshot(count) !== 5) {
    throw new Error("empty-root React store snapshot failed");
  }
  store.dispose();

  let graphDenied = false;
  try {
    signals.graph("publish.empty.graph", { inputs: {}, outputs: {} });
  } catch (error) {
    graphDenied = /imported graph/i.test(String(error?.message ?? error));
  }
  if (!graphDenied) {
    throw new Error("empty-root must still deny signals.graph without import");
  }

  console.log(JSON.stringify({
    emptyRootSpecAuthored: true,
    emptyRootSpecMutated: true,
    emptyRootAspectReads: true,
    emptyRootFormSyncReceipt: true,
    emptyRootReactSnapshot: true,
    emptyRootGraphDenied: true,
  }));
} finally {
  await signals.terminate();
  globalThis.Worker = previousWorker;
}
`;
}
