import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../runtime_fixture/graph_operational_runtime.mjs";

test("signals.form records raw input and committed draft transitions in state history", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
    });

    form.fields.title.input("Ship docs now", { source: "typing" });
    const afterInput = form.stateHistory();
    assert.equal(afterInput.length, 1);
    assert.equal(afterInput[0].entryKind, "rawInput");
    assert.equal(afterInput[0].operation, "reported");
    assert.equal(afterInput[0].source, "typing");

    form.fields.title.commitInput();
    const history = form.stateHistory();
    assert.deepEqual(
      history.map((entry) => [entry.entryKind, entry.operation]),
      [
        ["rawInput", "reported"],
        ["rawInput", "committed"],
        ["draftWrite", "commitInput"],
      ],
    );
    assert.notEqual(history[0].patchPlanDigest, history[2].patchPlanDigest);
    assert.notEqual(history[0].readinessDigest, history[2].readinessDigest);
  } finally {
    await cleanup();
  }
});

test("signals.form records pending raw input supersession before imperative draft writes", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
    });

    form.fields.title.input("Ship docs now", { source: "typing" });
    form.fields.title.set("Published docs");

    const history = form.stateHistory();
    assert.deepEqual(
      history.map((entry) => [entry.entryKind, entry.operation, entry.reason]),
      [
        ["rawInput", "reported", null],
        ["rawInput", "clearedBySet", null],
        ["draftWrite", "setValue", null],
      ],
    );
    assert.equal(history[1].rawValueDigest, history[0].rawValueDigest);
    assert.equal(history[2].previousDraftDigest, history[1].previousDraftDigest);
    assert.equal(form.verification().performanceEnvelope.rawInputOperations, 2);
    assert.equal(form.verification().performanceEnvelope.fieldWriteOperations, 1);
  } finally {
    await cleanup();
  }
});

test("signals.form diagnostics surfaces report retained state transition counts", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
    });

    form.fields.title.set("Ship docs now");

    const diagnostics = form.diagnostics();
    assert.equal(diagnostics.stateHistory.length, form.stateHistory().length);
    assert.equal(diagnostics.summary.histories.stateTransitions, form.stateHistory().length);
  } finally {
    await cleanup();
  }
});

test("signals.form verification certifies state history digests and operation counters", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
    });

    form.fields.title.input("Ship docs now", { source: "typing" });
    form.fields.title.commitInput();
    form.fields.title.set("Ship docs later");

    const verification = form.verification();
    const stateHistory = form.stateHistory();
    assert.equal(verification.stateHistory.operations, stateHistory.length);
    assert.equal(verification.stateHistory.digest, verification.digests.stateHistoryDigest);
    assert.equal(verification.performanceEnvelope.rawInputOperations, 2);
    assert.equal(verification.performanceEnvelope.fieldWriteOperations, 2);
  } finally {
    await cleanup();
  }
});

test("signals.form state history digest does not depend on observation wall clock", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  const originalNow = Date.now;
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    Date.now = () => 1000;
    const firstForm = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
    });
    firstForm.fields.title.set("Ship docs now");
    const firstDigest = firstForm.stateHistory()[0].stateHistoryDigest;
    const firstVerification = firstForm.verification();
    const firstDiagnostics = firstForm.diagnostics();

    Date.now = () => 9000;
    const secondForm = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
    });
    secondForm.fields.title.set("Ship docs now");
    const secondArtifact = secondForm.stateHistory()[0];
    const secondVerification = secondForm.verification();
    const secondDiagnostics = secondForm.diagnostics();
    assert.equal(firstDigest, secondArtifact.stateHistoryDigest);
    assert.equal(
      firstVerification.digests.stateHistoryDigest,
      secondVerification.digests.stateHistoryDigest,
    );
    assert.equal(
      firstVerification.diagnosticsHistory.digest,
      secondVerification.diagnosticsHistory.digest,
    );
    assert.equal(firstDiagnostics.digest, secondDiagnostics.digest);
    assert.equal(
      firstVerification.digests.diagnosticsDigest,
      secondVerification.digests.diagnosticsDigest,
    );
    assert.notEqual(firstForm.stateHistory()[0].observedAtMs, secondArtifact.observedAtMs);
  } finally {
    Date.now = originalNow;
    await cleanup();
  }
});
