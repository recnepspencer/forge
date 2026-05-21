import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

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

  try {
    const workerSignals = await createSignals();
    const workerImportedGraph = workerSignals.importGraph(definition, snapshot);
    await workerImportedGraph.ready();

    const workerForm = workerSignals.form({
      source: workerSignals.form.source.signal(workerImportedGraph.input("document"), {
        id: "worker-form-source",
      }),
      fields: ({ field }) => ({
        title: field("title"),
        status: field("status"),
      }),
    });
    const compatibilityForm = compatibilityImportedSignals.form({
      source: compatibilityImportedSignals.form.source.signal(
        compatibilityImportedGraph.input("document"),
        { id: "worker-form-source" },
      ),
      fields: ({ field }) => ({
        title: field("title"),
        status: field("status"),
      }),
    });

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
    compatibilityImportedSignals.free();
    compatibilitySignals.free();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});
