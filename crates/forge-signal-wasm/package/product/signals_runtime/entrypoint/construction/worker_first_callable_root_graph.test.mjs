import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

function comparableExportedGraphDefinition(definition) {
  return {
    summary: {
      ...definition.summary,
      id: "<graph>",
    },
    contract: {
      ...definition.contract,
      graph: {
        ...definition.contract.graph,
        id: "<graph>",
      },
    },
    operationalContract: {
      ...definition.operationalContract,
      graph: {
        ...definition.operationalContract.graph,
        id: "<graph>",
      },
    },
    inputDescriptors: definition.inputDescriptors,
    descriptors: definition.descriptors,
  };
}

function comparableGraphContract(contract) {
  return {
    ...contract,
    graph: {
      ...contract.graph,
      id: "<graph>",
    },
  };
}

function comparableWhy(why) {
  return {
    ...why,
    id: "<id>",
    node: "<node>",
    upstream: Array.isArray(why.upstream) ? why.upstream.map(() => "<upstream>") : why.upstream,
  };
}

function comparableReplay(replay) {
  return {
    frameCount: replay.frames.length,
    graphId: "<graph>",
    frames: replay.frames.map((frame) => ({
      kind: frame.kind,
      branchId: frame.branchId,
      snapshotId: frame.snapshotId ?? null,
      id: "<id>",
      node: "<node>",
    })),
  };
}

function comparableLineage(lineage) {
  const entries = Array.isArray(lineage.nodes)
    ? lineage.nodes
    : Array.isArray(lineage.frames)
      ? lineage.frames
      : [];
  return {
    entryCount: entries.length,
    graphId: "<graph>",
    entries: entries.map((entry) => ({
      kind: entry.kind,
      branchId: entry.branchId,
      snapshotId: entry.snapshotId ?? null,
      id: "<id>",
      node: "<node>",
    })),
  };
}

function comparableContractSummary(summary) {
  return {
    inputNames: summary.inputNames,
    outputNames: summary.outputNames,
    dependencyOutputs: Object.keys(summary.dependencies),
  };
}

test("default worker-first root graph exposes contract, export, and inspection parity for active imported-graph handles", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });

  const compatibilitySignals = await createSignals({ deployment: "mainThreadCompatibility" });
  const count = compatibilitySignals.input(2, { debugName: "count" });
  const graph = compatibilitySignals.graph("workerFirstRootGraphBase", {
    inputs: { count },
    outputs: {
      doubled: compatibilitySignals.computedSpec("worker:first:root:graph:doubled", {
        reads: [count.id],
        expr: {
          kind: "sum",
          args: [
            { kind: "read", id: count.id },
            { kind: "read", id: count.id },
          ],
        },
        identity: { kind: "exact" },
      }),
    },
  });
  graph.writeInput("count", 8);
  const definition = graph.exportDefinition();
  const snapshot = graph.exportSnapshot();

  const compatibilityImportedSignals = await createSignals({
    deployment: "mainThreadCompatibility",
  });
  const compatibilityImportedGraph = compatibilityImportedSignals.importGraph(definition, snapshot);
  await compatibilityImportedGraph.ready();

  try {
    const workerSignals = await createSignals();
    const importedGraph = workerSignals.importGraph(definition, snapshot);
    await importedGraph.ready();

    const compatibilityAlias = compatibilitySignals.graph("compatAlias", {
      inputs: { count },
      outputs: { doubled: graph.output("doubled") },
    });
    const workerAlias = workerSignals.graph("compatAlias", {
      inputs: { count: workerSignals.publicInput(importedGraph.input("count")) },
      outputs: { doubled: importedGraph.output("doubled") },
    });
    const workerBuilderAlias = workerSignals.graph("compatAliasBuilder", (alias) => alias.expose({
      controllers: [
        alias.controller({
          inputs: { count: alias.input.required(importedGraph.input("count")) },
          outputs: { doubled: importedGraph.output("doubled") },
        }),
      ],
    }));

    assert.deepEqual(workerAlias.contract().graph.inputNames, compatibilityAlias.contract().graph.inputNames);
    assert.deepEqual(workerAlias.contract().graph.outputNames, compatibilityAlias.contract().graph.outputNames);
    assert.deepEqual(
      comparableGraphContract(workerBuilderAlias.contract()),
      comparableGraphContract(workerAlias.contract()),
    );
    assert.deepEqual(workerAlias.readInputs(), compatibilityAlias.readInputs());
    assert.deepEqual(workerBuilderAlias.readInputs(), compatibilityAlias.readInputs());
    assert.deepEqual(workerAlias.read(), compatibilityAlias.read());
    assert.deepEqual(workerBuilderAlias.read(), compatibilityAlias.read());
    assert.deepEqual(workerAlias.summary().inputNames, compatibilityAlias.summary().inputNames);
    assert.deepEqual(workerAlias.summary().outputNames, compatibilityAlias.summary().outputNames);
    assert.deepEqual(workerAlias.inputDescriptors(), compatibilityAlias.inputDescriptors());
    assert.equal(workerAlias.descriptors()[0].publicationKind, "existingOutput");
    assert.equal(workerAlias.descriptors()[0].publishedId, importedGraph.output("doubled").id);
    assert.deepEqual(workerBuilderAlias.descriptors(), workerAlias.descriptors());
    assert.deepEqual(
      comparableExportedGraphDefinition(workerAlias.exportDefinition()),
      comparableExportedGraphDefinition(workerBuilderAlias.exportDefinition()),
    );
    assert.deepEqual(
      comparableExportedGraphDefinition(workerAlias.exportSnapshot().definition),
      comparableExportedGraphDefinition(workerAlias.exportDefinition()),
    );
    assert.deepEqual(comparableWhy(workerAlias.why("doubled")), comparableWhy(compatibilityAlias.why("doubled")));
    assert.deepEqual(
      comparableReplay(workerAlias.replayFor("doubled")),
      comparableReplay(compatibilityAlias.replayFor("doubled")),
    );
    assert.deepEqual(
      comparableLineage(workerAlias.lineageFor("doubled")),
      comparableLineage(compatibilityAlias.lineageFor("doubled")),
    );
    assert.deepEqual(
      comparableContractSummary(workerAlias.inspectDiagnostics().contractSummary()),
      comparableContractSummary(compatibilityAlias.inspectDiagnostics().contractSummary()),
    );
    assert.deepEqual(
      comparableContractSummary(workerAlias.inspectHistory().contractSummary()),
      comparableContractSummary(compatibilityAlias.inspectHistory().contractSummary()),
    );
  } finally {
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

test("default worker-first root graph mutates through async worker-owned lanes, keeps sync transaction unavailable, and invalidates on superseding import", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });

  const compatibilitySignals = await createSignals({ deployment: "mainThreadCompatibility" });
  const left = compatibilitySignals.input(1, { debugName: "left" });
  const sourceGraph = compatibilitySignals.graph("workerFirstRootGraphSource", {
    inputs: { left },
    outputs: {
      mirrored: compatibilitySignals.computedSpec("worker:first:root:graph:mirrored", {
        reads: [left.id],
        expr: { kind: "read", id: left.id },
        identity: { kind: "exact" },
      }),
    },
  });
  const firstDefinition = sourceGraph.exportDefinition();
  const firstSnapshot = sourceGraph.exportSnapshot();
  sourceGraph.writeInput("left", 3);
  const secondSnapshot = sourceGraph.exportSnapshot();

  try {
    const workerSignals = await createSignals();
    const importedGraph = workerSignals.importGraph(firstDefinition, firstSnapshot);
    await importedGraph.ready();
    const workerAlias = workerSignals.graph("workerAlias", {
      inputs: { left: workerSignals.publicInput(importedGraph.input("left")) },
      outputs: { mirrored: importedGraph.output("mirrored") },
    });

    await workerAlias.writeInput("left", 4);
    assert.equal(workerAlias.readInputs().left, 4);
    assert.equal(workerAlias.read().mirrored, 4);

    await workerAlias.writeInputs({ left: 5 });
    assert.equal(workerAlias.readInputs().left, 5);

    await workerAlias.transactionAsync((tx) => {
      tx.set("left", 6);
    });
    assert.equal(workerAlias.read().mirrored, 6);

    await workerAlias.batchAsync((tx) => {
      tx.set(workerAlias.input("left"), 7);
    });
    assert.equal(workerAlias.readInputs().left, 7);

    await workerAlias.resetInput("left");
    assert.equal(workerAlias.readInputs().left, 1);

    assert.throws(
      () => workerAlias.transaction(() => {}),
      (error) => error?.name === "WorkerFirstGraphMutationUnavailable",
    );
    await assert.rejects(
      () => workerAlias.transactionAsync(() => {}),
      /requires at least one staged mutation/,
    );
    assert.throws(
      () => workerSignals.graph("badAlias", {
        inputs: { left: workerSignals.publicInput(importedGraph.input("left")) },
        outputs: { mirrored: importedGraph.input("left") },
      }),
      /must be an active imported-graph published output handle/,
    );

    const replacementGraph = workerSignals.importGraph(firstDefinition, secondSnapshot);
    await replacementGraph.ready();
    assert.throws(
      () => workerAlias.read(),
      /superseded by a newer root importGraph/,
    );
  } finally {
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});
