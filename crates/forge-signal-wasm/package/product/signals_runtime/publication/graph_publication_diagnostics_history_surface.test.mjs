import assert from "node:assert/strict";
import test from "node:test";

import { createGraphPublicationCase } from "../publication_fixture/graph_publication_case.mjs";


test("The Graph Publication Diagnostics And History Surface Test", async () => {
  const {
    cleanup,
    graph,
    panel,
    readVersionCalls,
    whyCalls,
    replayCalls,
    lineageCalls,
  } = await createGraphPublicationCase();

  try {
    assert.deepEqual(graph.why("count"), {
      id: "itemDetail.count",
      family: "why",
    });
    assert.deepEqual(graph.replayFor("doubled"), {
      id: "itemDetail.doubled",
      family: "replay",
    });
    assert.deepEqual(graph.lineageFor("panel"), {
      id: "itemDetail.panel",
      family: "lineage",
    });
    assert.deepEqual(graph.readVersions(), [
      { id: "itemDetail.count", version: 1 },
      { id: "itemDetail.doubled", version: 2 },
      { id: "itemDetail.panel", version: 3 },
    ]);

    const diagnosticsSurface = graph.inspectDiagnostics();
    assert.equal(Object.getPrototypeOf(diagnosticsSurface.inputs), null);
    assert.equal(Object.getPrototypeOf(diagnosticsSurface.outputs), null);
    assert.deepEqual(diagnosticsSurface.contract, graph.contract());
    assert.deepEqual(diagnosticsSurface.inputVersions, []);
    assert.deepEqual(diagnosticsSurface.dependenciesForOutput("panel"), {
      graphId: "itemDetail",
      outputName: "panel",
      publishedId: "itemDetail.panel",
      sourceId: panel.id,
      publicInputNames: [],
      publicInputSourceIds: [],
      transitiveSignalIds: ["itemDetail.panel", panel.id],
    });
    assert.deepEqual(diagnosticsSurface.contractSummary(), {
      graph: graph.summary(),
      contract: graph.contract(),
      inputCount: 0,
      outputCount: 3,
      inputNames: [],
      outputNames: ["count", "doubled", "panel"],
      dependencies: diagnosticsSurface.dependencies,
    });
    assert.deepEqual(diagnosticsSurface.outputs.count, {
      descriptor: graph.descriptors()[0],
      version: { id: "itemDetail.count", version: 1 },
      why: { id: "itemDetail.count", family: "why" },
    });
    assert.deepEqual(diagnosticsSurface.outputs.panel, {
      descriptor: graph.descriptors()[2],
      version: { id: "itemDetail.panel", version: 3 },
      why: { id: "itemDetail.panel", family: "why" },
    });
    assert.deepEqual(diagnosticsSurface.runtimeGraph, {
      profile: "WebDevelopment",
      active_node_count: 5,
    });
    assert.deepEqual(diagnosticsSurface.executionHistory.history.latest_execution_record_id, 12);
    assert.deepEqual(diagnosticsSurface.latestObservation.observation.phase, "Apply");

    const historySurface = graph.inspectHistory();
    assert.equal(Object.getPrototypeOf(historySurface.inputs), null);
    assert.equal(Object.getPrototypeOf(historySurface.outputs), null);
    assert.deepEqual(historySurface.contract, graph.contract());
    assert.deepEqual({ ...historySurface.inputs }, {});
    assert.deepEqual(
      historySurface.dependenciesForOutput("panel"),
      diagnosticsSurface.dependenciesForOutput("panel"),
    );
    assert.deepEqual(
      historySurface.contractSummary(),
      diagnosticsSurface.contractSummary(),
    );
    assert.deepEqual(historySurface.outputs.doubled, {
      descriptor: graph.descriptors()[1],
      replay: { id: "itemDetail.doubled", family: "replay" },
      lineage: { id: "itemDetail.doubled", family: "lineage" },
    });
    assert.deepEqual(historySurface.recentHistory[0].latest_execution_record_id, 11);

    assert.deepEqual(readVersionCalls, [
      ["itemDetail.count", "itemDetail.doubled", "itemDetail.panel"],
      [],
      ["itemDetail.count", "itemDetail.doubled", "itemDetail.panel"],
    ]);
    assert.deepEqual(whyCalls, [
      "itemDetail.count",
      "itemDetail.count",
      "itemDetail.doubled",
      "itemDetail.panel",
    ]);
    assert.deepEqual(replayCalls, [
      "itemDetail.doubled",
      "itemDetail.count",
      "itemDetail.doubled",
      "itemDetail.panel",
    ]);
    assert.deepEqual(lineageCalls, [
      "itemDetail.panel",
      "itemDetail.count",
      "itemDetail.doubled",
      "itemDetail.panel",
    ]);
  } finally {
    await cleanup();
  }
});
