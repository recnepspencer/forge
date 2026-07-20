import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

function assertEquivalentFormTruth(left, right) {
  assert.deepEqual(left.source(), right.source());
  assert.deepEqual(left.draft(), right.draft());
  assert.deepEqual(left.effective(), right.effective());
  assert.deepEqual(left.dirty(), right.dirty());
  assert.deepEqual(left.patchPlan(), right.patchPlan());
  assert.deepEqual(left.readiness(), right.readiness());
  assert.deepEqual(left.validation(), right.validation());
  assert.deepEqual(left.availability(), right.availability());
  assert.deepEqual(left.admission(), right.admission());
  assert.deepEqual(left.actionReadiness("submit"), right.actionReadiness("submit"));
  assert.equal(left.actionPlan("submit").planDigest, right.actionPlan("submit").planDigest);
}

function comparableResourceExecution(execution) {
  return {
    resultKind: execution.resultKind,
    patchCount: execution.resourceSubmission?.patchCount ?? null,
    rollbackKind: execution.resourceSubmission?.rollback?.kind ?? null,
    rollbackMode: execution.resourceSubmission?.rollback?.mode ?? null,
    visibleSelectionKind: execution.resourceSubmission?.visibleSelection?.kind ?? null,
  };
}

function comparableRollback(result) {
  return {
    mode: result.mode,
    resultKind: result.resultKind,
    kind: result.resourceRollback.kind,
    terminalKind: result.resourceRollback.terminalKind ?? null,
    retiredEffectCount: result.resourceRollback.retiredEffectIds?.length ?? null,
    projectionKind: result.resourceRollback.projectionKind ?? null,
  };
}

function comparableReplayRestore(result) {
  return {
    mode: result.mode,
    resultKind: result.resultKind,
    kind: result.resourceReplayRestore.kind,
    replayRestoreMode: result.resourceReplayRestore.mode ?? null,
  };
}

async function rollbackRemainingFormEffects(form, line) {
  const results = [];
  while (line.effects().open().length > 0) {
    results.push(comparableRollback(
      await form.rollbackLastResourceEffect(),
    ));
  }
  return results;
}

test("default worker-first root exposes form factories over active imported-graph handles", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });

  const compatibilitySignals = await createSignals({ deployment: "mainThreadCompatibility" });
  const source = compatibilitySignals.input(
    { title: "Draft", status: "editing" },
    { debugName: "form-source" },
  );
  const graph = compatibilitySignals.graph("workerFirstRootForm", {
    inputs: { document: compatibilitySignals.publicInput(source) },
    outputs: {
      document: source,
    },
  });
  const definition = graph.exportDefinition();
  const snapshot = graph.exportSnapshot();

  const compatibilityImportedSignals = await createSignals({
    deployment: "mainThreadCompatibility",
  });
  const compatibilityImportedGraph = compatibilityImportedSignals.importGraph(definition, snapshot);
  await compatibilityImportedGraph.ready();

  let workerSignals = null;
  try {
    workerSignals = await createSignals();
    const workerImportedGraph = workerSignals.importGraph(definition, snapshot);
    await workerImportedGraph.ready();

    const workerDeclaration = workerSignals.form.define({
      source: workerSignals.form.source.signal(workerImportedGraph.input("document"), {
        id: "worker-form-source",
      }),
      fields: ({ field }) => ({
        title: field("title"),
        status: field("status"),
      }),
    });
    const compatibilityDeclaration = compatibilityImportedSignals.form.define({
      source: compatibilityImportedSignals.form.source.signal(
        compatibilityImportedGraph.input("document"),
        { id: "worker-form-source" },
      ),
      fields: ({ field }) => ({
        title: field("title"),
        status: field("status"),
      }),
    });
    const workerForm = workerSignals.form(workerDeclaration);
    const compatibilityForm = compatibilityImportedSignals.form(compatibilityDeclaration);

    assert.equal(Object.isFrozen(workerDeclaration), true);
    assert.equal(Object.isFrozen(compatibilityDeclaration), true);
    assert.equal(workerForm.sourceAuthority().kind, "signal");
    assert.equal(workerForm.sourceAuthority().sourceId, "worker-form-source");
    assert.deepEqual(workerForm.source(), compatibilityForm.source());
    assert.deepEqual(workerForm.declaration().fieldFamilies, compatibilityForm.declaration().fieldFamilies);

    workerForm.fields.title.set("Worker title");
    compatibilityForm.fields.title.set("Worker title");
    assert.deepEqual(workerForm.draft(), compatibilityForm.draft());
    assert.deepEqual(workerForm.effective(), compatibilityForm.effective());
    assert.equal(
      workerForm.verification().digests.fieldContractDigest,
      compatibilityForm.verification().digests.fieldContractDigest,
    );

    await workerImportedGraph.writeInput("document", {
      title: "Server title",
      status: "published",
    });
    await compatibilityImportedGraph.writeInput("document", {
      title: "Server title",
      status: "published",
    });

    assert.deepEqual(workerForm.source(), compatibilityForm.source());
    assert.equal(workerForm.sourceCompatibility().posture, compatibilityForm.sourceCompatibility().posture);

    const scopedNamespace = workerSignals.scope("wizard");
    const scopedForm = scopedNamespace.form({
      source: scopedNamespace.form.source.signal(workerImportedGraph.input("document"), {
        id: "scoped-worker-form-source",
      }),
      fields: ({ field }) => ({
        title: field("title"),
      }),
    });
    assert.equal(scopedForm.namespace.scopeId, scopedNamespace.scopeId);
    assert.equal(scopedForm.sourceAuthority().sourceId, "scoped-worker-form-source");
    assert.deepEqual(scopedForm.source(), workerForm.source());

    scopedForm.fields.title.set("Scoped title");
    assert.equal(scopedForm.effective().title, "Scoped title");
  } finally {
    await workerSignals?.terminate();
    compatibilityImportedSignals.free();
    compatibilitySignals.free();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

test("default worker-first root preserves resource-line form action and restore parity with explicit compatibility", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });

  let workerSignals = null;
  let compatibilitySignals = null;
  let workerLine = null;
  let compatibilityLine = null;
  try {
    workerSignals = await createSignals();
    compatibilitySignals = await createSignals({ deployment: "mainThreadCompatibility" });

    workerLine = workerSignals.api({
      effects: workerSignals.resource.effects.branchNative(),
    }).url("/tasks/:taskId").response(
      workerSignals.resource.response.detail()({ title: "title", status: "status" }),
    ).detail({
      load: ({ taskId }) => ({ id: taskId, title: "Draft", status: "editing" }),
    }).line({ taskId: "task-1" });
    compatibilityLine = compatibilitySignals.api({
      effects: compatibilitySignals.resource.effects.branchNative(),
    }).url("/tasks/:taskId").response(
      compatibilitySignals.resource.response.detail()({ title: "title", status: "status" }),
    ).detail({
      load: ({ taskId }) => ({ id: taskId, title: "Draft", status: "editing" }),
    }).line({ taskId: "task-1" });
    try {
      await Promise.all([
        workerLine.awaitSettlement({ timeoutMs: 5_000 }),
        compatibilityLine.awaitSettlement({ timeoutMs: 5_000 }),
      ]);
    } catch (error) {
      throw new Error(`resource settlement parity failed: worker=${JSON.stringify(workerLine.status())} compatibility=${JSON.stringify(compatibilityLine.status())}`, { cause: error });
    }

    const workerForm = workerSignals.form({
      source: workerSignals.form.source.resourceLine(workerLine, { id: "worker-resource-form" }),
      fields: ({ field }) => ({
        title: field("title"),
        status: field("status"),
      }),
    });
    const compatibilityForm = compatibilitySignals.form({
      source: compatibilitySignals.form.source.resourceLine(compatibilityLine, { id: "worker-resource-form" }),
      fields: ({ field }) => ({
        title: field("title"),
        status: field("status"),
      }),
    });

    assertEquivalentFormTruth(workerForm, compatibilityForm);
    assert.equal(
      workerForm.sourceAuthority().sourceId,
      compatibilityForm.sourceAuthority().sourceId,
    );

    workerForm.fields.title.set("Published docs");
    workerForm.fields.status.set("review");
    compatibilityForm.fields.title.set("Published docs");
    compatibilityForm.fields.status.set("review");

    const workerExecution = await workerForm.executeAction("submit");
    const compatibilityExecution = await compatibilityForm.executeAction("submit");
    assert.deepEqual(
      comparableResourceExecution(workerExecution),
      comparableResourceExecution(compatibilityExecution),
    );
    assertEquivalentFormTruth(workerForm, compatibilityForm);

    const workerRollback = await workerForm.rollbackLastResourceEffect();
    const compatibilityRollback = await compatibilityForm.rollbackLastResourceEffect();
    assert.deepEqual(
      comparableRollback(workerRollback),
      comparableRollback(compatibilityRollback),
    );
    assertEquivalentFormTruth(workerForm, compatibilityForm);
    assert.equal(workerForm.resetHistory().length, compatibilityForm.resetHistory().length);
    assert.deepEqual(
      await rollbackRemainingFormEffects(workerForm, workerLine),
      await rollbackRemainingFormEffects(compatibilityForm, compatibilityLine),
    );

    workerForm.fields.title.set("Published docs again");
    compatibilityForm.fields.title.set("Published docs again");
    await workerForm.executeAction("submit");
    await compatibilityForm.executeAction("submit");
    workerForm.fields.title.set("Local draft after submit");
    compatibilityForm.fields.title.set("Local draft after submit");

    const workerRestore = await workerForm.restoreExactResourceSource();
    const compatibilityRestore = await compatibilityForm.restoreExactResourceSource();
    assert.deepEqual(
      comparableReplayRestore(workerRestore),
      comparableReplayRestore(compatibilityRestore),
    );
    assertEquivalentFormTruth(workerForm, compatibilityForm);
    assert.equal(
      workerForm.resourceSource().visibleSelection.kind,
      compatibilityForm.resourceSource().visibleSelection.kind,
    );
    assert.equal(
      workerForm.replayRestoreHistory().length,
      compatibilityForm.replayRestoreHistory().length,
    );
    assert.equal(
      workerLine.effects().open().length,
      compatibilityLine.effects().open().length,
    );
    assert.equal(workerLine.effects().open().length > 0, true);
    assert.deepEqual(
      await rollbackRemainingFormEffects(workerForm, workerLine),
      await rollbackRemainingFormEffects(compatibilityForm, compatibilityLine),
    );
    assert.equal(workerLine.effects().open().length, 0);
    assert.equal(compatibilityLine.effects().open().length, 0);
  } finally {
    try {
      workerLine?.free();
      compatibilityLine?.free();
    } finally {
      await workerSignals?.terminate();
      compatibilitySignals?.free();
      await cleanup();
      globalThis.Worker = previousWorker;
    }
  }
});
