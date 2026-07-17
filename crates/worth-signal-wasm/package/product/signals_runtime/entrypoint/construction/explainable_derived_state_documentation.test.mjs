import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

const testDir = path.dirname(fileURLToPath(import.meta.url));
const crateDir = path.resolve(testDir, "..", "..", "..", "..", "..");
const workspaceDir = path.resolve(crateDir, "..", "..");
const guidePath = path.join(
  crateDir,
  "docs",
  "app-surface",
  "explainable-derived-state.md",
);

test("the explainable derived-state guide matches the real transfer behavior", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const signals = await createSignals({ deployment: "mainThreadCompatibility" });
  try {
    const amount = signals.input(8_000, { debugName: "transfer.requestedAmount" });
    const fee = signals.computed(
      () => Math.round(amount() * 0.004 * 100) / 100,
      { debugName: "transfer.processingFee" },
    );
    const reviewLane = signals.computed(
      () => amount() >= 10_000 ? "Manual review" : "Automatic",
      { debugName: "transfer.reviewLane" },
    );

    fee();
    reviewLane();

    signals.transaction((tx) => tx.set(amount, 9_800));
    assert.equal(fee(), 39.2);
    assert.equal(reviewLane(), "Automatic");
    assert.equal(signals.diagnostics().why(reviewLane.id).outputChange, "Unchanged");
    assert.ok(signals.diagnostics().latestFlow()?.flow);

    signals.transaction((tx) => tx.set(amount, 14_500));
    assert.equal(fee(), 58);
    assert.equal(reviewLane(), "Manual review");
    assert.equal(signals.diagnostics().why(reviewLane.id).outputChange, "Refreshed");
  } finally {
    signals.free();
    await cleanup();
  }
});

test("Demo 1 documentation states its deployment and retention boundaries", async () => {
  const [guide, demoData, featureIndex, startHere, signalsSection, workbench] = await Promise.all([
    readFile(guidePath, "utf8"),
    readFile(path.join(workspaceDir, "apps", "WORTH-signal-demo", "src", "state", "demoData.ts"), "utf8"),
    readFile(path.join(crateDir, "docs", "learn", "feature-index.md"), "utf8"),
    readFile(path.join(crateDir, "docs", "start_here.md"), "utf8"),
    readFile(path.join(workspaceDir, "apps", "WORTH-signal-demo", "src", "ui", "SignalsSection.tsx"), "utf8"),
    readFile(path.join(workspaceDir, "apps", "WORTH-signal-demo", "src", "ui", "signals-demo", "SignalsTransferWorkbench.tsx"), "utf8"),
  ]);

  assert.match(guide, /deployment: "mainThreadCompatibility"/u);
  assert.match(guide, /default `createSignals\(\)` deployment is worker-first/u);
  assert.match(guide, /visible list of diagnostic snapshots/u);
  assert.match(guide, /not an\s+application audit database/u);
  assert.match(guide, /recomputed but unchanged/u);
  assert.match(demoData, /app-surface\/explainable-derived-state/u);
  assert.match(featureIndex, /app-surface\/explainable-derived-state\.md/u);
  assert.match(startHere, /app-surface\/explainable-derived-state\.md/u);
  assert.match(signalsSection, /#\/docs\/app-surface\/explainable-derived-state/u);
  assert.match(workbench, /This demo keeps the visible list in UI state/u);
  assert.doesNotMatch(workbench, /Nothing here is kept by the UI/u);
});
