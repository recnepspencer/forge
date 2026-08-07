import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

const delay = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

/**
 * awaitSettlement must drain worker-first authored pubs/mutations so a delayed
 * load cannot race collaborative form reactive inputs.
 */
test("worker-first awaitSettlement drains authored work before delayed-load form submit", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup, resourcePatch } = await loadSignalsModule({
    rawSurface: "real",
  });
  let signals = null;
  let line = null;
  try {
    signals = await createSignals({ deployment: "workerFirst" });
    assert.equal(typeof signals.settleAuthoredWork, "function");

    line = signals.api({
      effects: signals.resource.effects.branchNative(),
    }).url("/tasks/:taskId").response(
      signals.resource.response.detail()({ title: "title", status: "status" }),
    ).detail({
      load: async ({ taskId: id }) => {
        await delay(35);
        return { id, title: "Draft", status: "editing" };
      },
    }).line({ taskId: "drain-proof" });

    assert.equal(line.status().kind, "pending");
    const settled = await line.awaitSettlement({ timeoutMs: 5_000 });
    assert.equal(settled.resultKind, "fulfilled");

    await line.patch(resourcePatch.field({
      field: "title",
      value: "Optimistic",
    }));

    const form = signals.form({
      source: signals.form.source.resourceLine(line, { id: "drain-proof-form" }),
      collaboration: {
        mode: "branchPerActor",
        actorId: "me",
        supportsPresence: true,
      },
      fields: ({ field }) => ({
        title: field("title"),
        status: field("status"),
      }),
      actions: ({ submit }) => ({
        submit: submit({
          resourceEffectProfile: signals.resource.effects.branchNative(),
        }),
      }),
    });

    assert.equal(form.collaboration().resourceProof.admitted, true);
    await form.fields.title.set("Optimistic-2");
    const execution = await form.executeAction("submit");
    assert.equal(execution.resultKind, "fulfilled");
    assert.equal(execution.effectStarted, true);
    assert.equal(line.value().title, "Optimistic-2");
  } finally {
    try {
      line?.free();
    } catch {
      // ignore
    }
    if (signals) {
      await Promise.race([
        signals.terminate().catch(() => {}),
        delay(3_000),
      ]);
    }
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});
