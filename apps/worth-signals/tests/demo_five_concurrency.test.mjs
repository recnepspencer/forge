import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadSignalsModule } from "../../../crates/forge-signal-wasm/package/product/signals_runtime/module_loading/load_signals_module.mjs";
import {
  buildTenRequestScenario,
  createRuntimeReceipt,
} from "../src/ui/resourcesSectionSupport.ts";

test("Demo 5 builds a reproducible ten-request mixed dependency scenario", () => {
  const first = buildTenRequestScenario(7_031);
  const repeated = buildTenRequestScenario(7_031);
  const varied = buildTenRequestScenario(7_032);

  assert.equal(first.requests.length, 10);
  assert.equal(first.requests.filter((request) => request.accepted).length, 5);
  assert.equal(first.requests.filter((request) => !request.accepted).length, 5);
  assert.equal(first.requests.filter((request) => request.dependsOnLineId).length, 4);
  assert.deepEqual(first.settlementOrder, repeated.settlementOrder);
  assert.notDeepEqual(first.settlementOrder, varied.settlementOrder);
});

test("Demo 5 settles ten worker-first effects to server truth without residue", async () => {
  await withWorkerSignals(async ({ signals, resourcePatch }) => {
    const scenario = buildTenRequestScenario(7_031);
    const family = createLinesFamily(signals);
    const line = family.line({ orderId: "PO-1142" });
    await line.awaitSettlement();
    const baselineBranches = (await signals.history().branches()).length;
    const effectByLine = new Map();

    for (const request of scenario.requests) {
      const insert = family.patch.insert({
        itemId: request.line.id,
        placement: "append",
        nextItem: request.line,
      });
      const parentEffectId = request.dependsOnLineId
        ? effectByLine.get(request.dependsOnLineId)
        : null;
      const patch = parentEffectId
        ? resourcePatch.dependsOn(insert, [parentEffectId])
        : insert;
      const result = await line.patch(patch);
      assert.equal(typeof result.effectId, "string");
      effectByLine.set(request.line.id, result.effectId);
    }

    assert.equal(line.effects().open().length, 10);
    assert.equal(line.effects().projection().kind, "derivedEffectProjectionBranch");

    const requestByLine = new Map(scenario.requests.map((request) => [request.line.id, request]));
    for (const lineId of scenario.settlementOrder) {
      const request = requestByLine.get(lineId);
      const effectId = effectByLine.get(lineId);
      if (line.effects().get(effectId) === null) continue;
      if (request.accepted) {
        await line.effects().confirm(effectId, {
          responseId: `demo:${lineId}:accepted`,
          serverPatch: family.patch.insert({
            itemId: lineId,
            placement: "append",
            nextItem: { ...request.line, sync: "synced" },
          }),
        });
      } else {
        await line.effects().reject(effectId, {
          responseId: `demo:${lineId}:rejected`,
        });
      }
    }

    const acceptedIds = scenario.requests
      .filter((request) => request.accepted)
      .map((request) => request.line.id)
      .sort();
    assert.deepEqual(
      line.value().filter((entry) => entry.id !== "line-071").map((entry) => entry.id).sort(),
      acceptedIds,
    );
    assert.equal(line.effects().open().length, 0);
    assert.equal(line.effects().projection().kind, "canonical");
    assert.deepEqual(line.effects().counters(), {
      effectLookupCount: 10,
      pendingAdmissionCount: 0,
      openEffectCount: 0,
      dependencyIndexKeyCount: 0,
      locusIndexKeyCount: 0,
      retryLineageIndexKeyCount: 0,
    });
    assert.equal((await signals.history().branches()).length, baselineBranches);
    line.free();
  });
});

test("Demo 5 create-edit dependencies handle transformed success and rejection", async () => {
  await withWorkerSignals(async ({ signals, resourcePatch }) => {
    const family = createLinesFamily(signals);
    const line = family.line({ orderId: "PO-1142" });
    await line.awaitSettlement();

    const createResult = await line.patch(family.patch.insert({
      itemId: "line-success",
      placement: "append",
      nextItem: { id: "line-success", label: "Draft", qty: "1 lot", sync: "syncing" },
    }));
    const editResult = await line.patch(resourcePatch.dependsOn(
      family.patch.item({
        itemId: "line-success",
        nextItem: { id: "line-success", label: "Validated", qty: "1 lot", sync: "syncing" },
      }),
      [createResult.effectId],
    ));
    const recorded = await line.effects().confirm(editResult.effectId, {
      serverPatch: family.patch.item({
        itemId: "line-success",
        nextItem: { id: "line-success", label: "VALIDATED", qty: "1 lot", sync: "synced" },
      }),
    });
    assert.equal(recorded.kind, "responseRecorded");
    const parent = await line.effects().confirm(createResult.effectId);
    assert.deepEqual(parent.automaticallySettled.map((entry) => entry.effectId), [editResult.effectId]);
    assert.equal(line.value().find((entry) => entry.id === "line-success").label, "VALIDATED");

    const rejectedCreate = await line.patch(family.patch.insert({
      itemId: "line-rejected",
      placement: "append",
      nextItem: { id: "line-rejected", label: "Rejected", qty: "1 lot", sync: "syncing" },
    }));
    const rejectedEdit = await line.patch(resourcePatch.dependsOn(
      family.patch.item({
        itemId: "line-rejected",
        nextItem: { id: "line-rejected", label: "Edited", qty: "1 lot", sync: "syncing" },
      }),
      [rejectedCreate.effectId],
    ));
    await line.effects().confirm(rejectedEdit.effectId);
    const rejected = await line.effects().reject(rejectedCreate.effectId);
    assert.deepEqual(rejected.retired.map((entry) => entry.effectId), [rejectedEdit.effectId, rejectedCreate.effectId]);
    assert.equal(line.value().some((entry) => entry.id === "line-rejected"), false);
    assert.equal(line.effects().open().length, 0);
    line.free();
  });
});

test("Demo 5 has no compatibility or rejection-time repair shortcut", async () => {
  const forgePanel = await readFile(new URL("../src/ui/ResourcesForgePanel.tsx", import.meta.url), "utf8");
  const section = await readFile(new URL("../src/ui/ResourcesSection.tsx", import.meta.url), "utf8");
  const bridge = await readFile(new URL("../src/runtime/forgeSignalWasmBridge.ts", import.meta.url), "utf8");

  assert.match(forgePanel, /createSignals\(\)/u);
  assert.doesNotMatch(forgePanel, /mainThreadCompatibility/u);
  assert.doesNotMatch(forgePanel, /linesFamily\.patch\.delete|setLines/u);
  assert.match(forgePanel, /effects\(\)\.confirm/u);
  assert.match(forgePanel, /effects\(\)\.reject/u);
  assert.match(forgePanel, /resourcePatch\.dependsOn/u);
  assert.match(forgePanel, /createRuntimeReceipt/u);
  assert.doesNotMatch(forgePanel, /dependencyCancelled\.current|for \(let attempt/u);
  assert.match(section, /evidence === "refetchCompleted"/u);
  assert.match(bridge, /createPackagedSignals/u);
});

test("Demo 5 ledger claims are runtime-issued receipt fields", async () => {
  await withWorkerSignals(async ({ signals, resourcePatch }) => {
    const family = createLinesFamily(signals);
    const line = family.line({ orderId: "PO-1142" });
    await line.awaitSettlement();
    const parent = await line.patch(family.patch.insert({
      itemId: "line-receipt",
      placement: "append",
      nextItem: { id: "line-receipt", label: "Draft", qty: "1", sync: "syncing" },
    }));
    const child = await line.patch(resourcePatch.dependsOn(
      family.patch.item({
        itemId: "line-receipt",
        nextItem: { id: "line-receipt", label: "Reviewed", qty: "1", sync: "syncing" },
      }),
      [parent.effectId],
    ));

    const admitted = createRuntimeReceipt(line.effects(), child.effectId);
    assert.deepEqual(admitted.effect, line.effects().get(child.effectId));
    assert.deepEqual(
      admitted.effect.dependencyEffectIds,
      [parent.effectId],
    );
    assert.equal(
      admitted.effect.branchId,
      line.effects().get(child.effectId).branchId,
    );
    assert.deepEqual(admitted.projection, line.effects().projection());
    assert.deepEqual(admitted.counters, line.effects().counters());

    const settlement = await line.effects().reject(parent.effectId, {
      responseId: "demo:receipt:rejected",
    });
    const retired = createRuntimeReceipt(
      line.effects(),
      parent.effectId,
      settlement,
    );
    assert.deepEqual(retired.settlement, settlement);
    assert.deepEqual(
      retired.settlement.retired.map((entry) => entry.effectId),
      [child.effectId, parent.effectId],
    );
    assert.equal(retired.effect.terminal.kind, "rejectedAndRetired");
    assert.deepEqual(retired.projection, line.effects().projection());
    assert.deepEqual(retired.counters, line.effects().counters());
    line.free();
  });
});

function createLinesFamily(signals) {
  return signals.api({ effects: signals.resource.effects.branchNative() })
    .url("/orders/:orderId/lines")
    .response(signals.resource.response.array({ itemId: (line) => line.id }))
    .list({
      load: () => [{ id: "line-071", label: "Existing", qty: "1 lot", sync: "synced" }],
    });
}

async function withWorkerSignals(run) {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const loaded = await loadSignalsModule({ rawSurface: "real" });
  let signals = null;
  try {
    signals = await loaded.createSignals();
    await run({ signals, resourcePatch: loaded.resourcePatch });
  } finally {
    if (signals) await signals.terminate();
    await loaded.cleanup();
    globalThis.Worker = previousWorker;
  }
}
