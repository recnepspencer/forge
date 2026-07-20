import assert from "node:assert/strict";
import { readFile, stat } from "node:fs/promises";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadSignalsModule } from "../../../crates/worth-signal-wasm/package/product/signals_runtime/module_loading/load_signals_module.mjs";
import {
  LINE_DEPENDENT,
  LINE_RISKY,
  LINE_SAFE,
  createPoServer,
  createRuntimeReceipt,
} from "../src/ui/resourcesSectionSupport.ts";

test("Demo 5 reset rejects pending server work instead of abandoning promises", async () => {
  const server = createPoServer();
  const pendingSave = server.save({ ...LINE_RISKY, attempt: 1 });

  server.reset();

  await assert.rejects(pendingSave, /Scenario reset before the server responded/u);
});

test("Demo 5 preserves an independent success while retiring a rejected parent and child", async () => {
  await withWorkerSignals(async ({ signals, resourcePatch }) => {
    const family = createLinesFamily(signals);
    const line = family.line({ orderId: "PO-1142" });
    await line.awaitSettlement();
    const baselineBranches = (await signals.history().branches()).length;

    const parent = await line.patch(family.patch.insert({
      itemId: LINE_RISKY.id,
      placement: "append",
      nextItem: LINE_RISKY,
    }));
    const sibling = await line.patch(family.patch.insert({
      itemId: LINE_SAFE.id,
      placement: "append",
      nextItem: LINE_SAFE,
    }));
    const child = await line.patch(resourcePatch.dependsOn(
      family.patch.insert({
        itemId: LINE_DEPENDENT.id,
        placement: "append",
        nextItem: LINE_DEPENDENT,
      }),
      [parent.effectId],
    ));

    assert.deepEqual(
      line.value().map((entry) => entry.id),
      ["line-071", LINE_RISKY.id, LINE_SAFE.id, LINE_DEPENDENT.id],
    );
    assert.equal(line.effects().open().length, 3);
    assert.equal(line.effects().projection().kind, "derivedEffectProjectionBranch");

    const confirmed = await line.effects().confirm(sibling.effectId, {
      responseId: "demo:safety-goggles:accepted",
      serverPatch: family.patch.insert({
        itemId: LINE_SAFE.id,
        placement: "append",
        nextItem: { ...LINE_SAFE, sync: "synced" },
      }),
    });
    assert.equal(confirmed.kind, "merged");
    assert.equal(line.effects().get(sibling.effectId).terminal.kind, "merged");

    const rejected = await line.effects().reject(parent.effectId, {
      responseId: "demo:controlled-solvent:rejected",
    });
    assert.deepEqual(
      rejected.retired.map((entry) => entry.effectId),
      [child.effectId, parent.effectId],
    );
    assert.deepEqual(
      line.value().map((entry) => entry.id),
      ["line-071", LINE_SAFE.id],
    );
    assert.equal(line.effects().open().length, 0);
    assert.equal(line.effects().projection().kind, "canonical");
    assert.deepEqual(line.effects().counters(), {
      effectLookupCount: 3,
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
  const WORTHPanel = await readFile(new URL("../src/ui/ResourcesWORTHPanel.tsx", import.meta.url), "utf8");
  const section = await readFile(new URL("../src/ui/ResourcesSection.tsx", import.meta.url), "utf8");
  const packageJson = await readFile(new URL("../package.json", import.meta.url), "utf8");
  const viteConfig = await readFile(new URL("../vite.config.ts", import.meta.url), "utf8");

  assert.match(WORTHPanel, /createSignals\(\)/u);
  assert.doesNotMatch(WORTHPanel, /mainThreadCompatibility/u);
  assert.doesNotMatch(WORTHPanel, /linesFamily\.patch\.delete|setLines/u);
  assert.match(WORTHPanel, /effects\(\)\.confirm/u);
  assert.match(WORTHPanel, /effects\(\)\.reject/u);
  assert.match(WORTHPanel, /resourcePatch\.dependsOn/u);
  assert.match(WORTHPanel, /createRuntimeReceipt/u);
  assert.doesNotMatch(WORTHPanel, /dependencyCancelled\.current|for \(let attempt/u);
  assert.doesNotMatch(section, /TanStack|QueryClient|ten requests/iu);
  assert.match(section, /ResourcesWORTHPanel/u);
  assert.match(section, /LINE_DEPENDENT/u);
  assert.match(section, /handleSubmitLines/u);
  assert.match(section, /handleApproveGoggles/u);
  assert.match(section, /handleRejectSolvent/u);
  assert.doesNotMatch(section, /setTimeout|BEAT_|runGenerationRef|siblingSettlementRef/u);
  assert.doesNotMatch(packageJson, /@tanstack\/react-query/u);
  assert.doesNotMatch(viteConfig, /find: "worth-signals-wasm/u);
  assert.doesNotMatch(viteConfig, /WORTHSignalWasmBridge/u);
});

test("Demo 5 presents one medical inventory product surface instead of a branch diagram", async () => {
  const WORTHPanel = await readFile(new URL("../src/ui/ResourcesWORTHPanel.tsx", import.meta.url), "utf8");
  const section = await readFile(new URL("../src/ui/ResourcesSection.tsx", import.meta.url), "utf8");
  const sectionParts = await readFile(new URL("../src/ui/ResourcesSectionParts.tsx", import.meta.url), "utf8");
  const scenarioGuide = await readFile(new URL("../src/ui/ResourcesScenarioGuide.tsx", import.meta.url), "utf8");
  const panelCss = await readFile(new URL("../src/ui/resourcesPanel.css", import.meta.url), "utf8");
  const sectionCss = await readFile(new URL("../src/ui/resourcesSection.css", import.meta.url), "utf8");

  assert.doesNotMatch(section, /ResourcesModelStrips|ServerTruthStrip|branch diagram/iu);
  assert.doesNotMatch(WORTHPanel, /BranchDagStrip|effect graph|branch-native patch/iu);
  assert.match(sectionParts, /Northstar Medical Center/u);
  assert.match(sectionParts, /Meridian Clinical Supply/u);
  assert.match(sectionParts, /className="po-line-image"/u);
  assert.match(scenarioGuide, /Add request lines/u);
  assert.match(scenarioGuide, /Approve goggles/u);
  assert.match(scenarioGuide, /Reject solvent/u);
  assert.doesNotMatch(scenarioGuide, /po-step-button|po-step-actions/u);
  assert.match(panelCss, /\.po-window-body \{[^}]*align-content: start;/u);
  assert.match(panelCss, /\.po-order-metadata \{[^}]*display: flex;/u);
  assert.doesNotMatch(panelCss, /\.po-order-metadata \{[^}]*grid-template-columns: repeat\(4/u);
  assert.match(sectionCss, /grid-template-columns: 220px minmax\(0, 1fr\) 300px/u);
  assert.match(sectionCss, /\.po-column \.po-app-sidebar \{ display: none; \}/u);
  assert.match(panelCss, /\.po-window \{[^}]*overflow: hidden;/u);
  assert.match(panelCss, /container-name: purchase-order; container-type: inline-size;/u);
  assert.match(panelCss, /@container purchase-order \(max-width: 900px\)/u);
  assert.match(panelCss, /\.po-line-statuses \{ grid-column: 3; grid-row: 1 \/ span 5;/u);
  assert.match(panelCss, /\.po-server-state \{ max-width: 96px; white-space: normal; text-align: center; \}/u);

  const productImages = [
    "nitrile-gloves.jpg",
    "controlled-solvent.jpg",
    "safety-goggles.jpg",
    "solvent-handling-kit.jpg",
  ];
  for (const image of productImages) {
    const metadata = await stat(new URL(`../public/products/${image}`, import.meta.url));
    assert.ok(metadata.size > 30_000, `${image} should be a real product photograph asset`);
  }
});

test("code typography applies one readable block size instead of nested em scaling", async () => {
  const mainSource = await readFile(new URL("../src/main.tsx", import.meta.url), "utf8");
  const typographyCss = await readFile(new URL("../src/ui/codeTypography.css", import.meta.url), "utf8");

  assert.match(mainSource, /import "\.\/index\.css";\s*import "\.\/ui\/codeTypography\.css";/u);
  assert.match(typographyCss, /pre code\s*\{\s*font-size: inherit;/su);
  assert.match(typographyCss, /text-size-adjust: 100%/u);
  assert.match(typographyCss, /pre\.signals-code-block\s*\{[^}]*font-size: var\(--code-block-size\)/su);
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
