import assert from "node:assert/strict";
import test from "node:test";

import { createItemDetailPublicationCase } from "../publication_fixture/item_detail_publication_case.mjs";


test("The Composition Diagnostics And History Parity Test", async () => {
  const { cleanup, graph, readVersionCalls } =
    await createItemDetailPublicationCase();

  try {
    assert.equal(
      graph.output("effectiveItemData").id,
      "itemDetail.effectiveItemData",
    );
    assert.equal(graph.output("dirtyState").id, "itemDetail.dirtyState");
    assert.equal(
      graph.output("submitReadiness").id,
      "itemDetail.submitReadiness",
    );
    assert.deepEqual(graph.summary(), {
      id: "itemDetail",
      inputCount: 0,
      inputNames: [],
      inputSourceIds: [],
      outputCount: 3,
      outputNames: ["effectiveItemData", "dirtyState", "submitReadiness"],
      publishedOutputIds: [
        "itemDetail.effectiveItemData",
        "itemDetail.dirtyState",
        "itemDetail.submitReadiness",
      ],
      sourceIds: ["effectiveItemData", "dirtyState", "submitReadiness"],
      synthesizedOutputCount: 3,
    });
    assert.deepEqual(graph.readVersions(), [
      { id: "itemDetail.effectiveItemData", version: 10 },
      { id: "itemDetail.dirtyState", version: 11 },
      { id: "itemDetail.submitReadiness", version: 12 },
    ]);
    assert.deepEqual(readVersionCalls, [
      [
        "itemDetail.effectiveItemData",
        "itemDetail.dirtyState",
        "itemDetail.submitReadiness",
      ],
    ]);

    const graphDiagnosticsSurface = graph.inspectDiagnostics();
    assert.equal(Object.getPrototypeOf(graphDiagnosticsSurface.inputs), null);
    assert.equal(Object.getPrototypeOf(graphDiagnosticsSurface.outputs), null);
    assert.deepEqual(graphDiagnosticsSurface.contract, graph.contract());
    assert.deepEqual({ ...graphDiagnosticsSurface.inputs }, {});
    assert.deepEqual(graphDiagnosticsSurface.inputVersions, []);
    assert.deepEqual(graphDiagnosticsSurface.outputs.submitReadiness, {
      descriptor: graph.descriptors()[2],
      version: { id: "itemDetail.submitReadiness", version: 12 },
      why: { id: "itemDetail.submitReadiness", family: "why" },
    });
    assert.deepEqual(graphDiagnosticsSurface.runtimeGraph, {
      profile: "WebDevelopment",
      active_node_count: 9,
    });

    const graphHistorySurface = graph.inspectHistory();
    assert.equal(Object.getPrototypeOf(graphHistorySurface.inputs), null);
    assert.equal(Object.getPrototypeOf(graphHistorySurface.outputs), null);
    assert.deepEqual(graphHistorySurface.contract, graph.contract());
    assert.deepEqual({ ...graphHistorySurface.inputs }, {});
    assert.deepEqual(graphHistorySurface.outputs.effectiveItemData, {
      descriptor: graph.descriptors()[0],
      replay: { id: "itemDetail.effectiveItemData", family: "replay" },
      lineage: { id: "itemDetail.effectiveItemData", family: "lineage" },
    });
    assert.deepEqual(graphHistorySurface.recentHistory, [
      {
        profile: "WebDevelopment",
        traced_node_count: 3,
        execution_record_count: 3,
        latest_execution_record_id: 20,
        reuse_origin_counts: {},
        nodes: [],
      },
    ]);

    assert.deepEqual(readVersionCalls, [
      [
        "itemDetail.effectiveItemData",
        "itemDetail.dirtyState",
        "itemDetail.submitReadiness",
      ],
      [],
      [
        "itemDetail.effectiveItemData",
        "itemDetail.dirtyState",
        "itemDetail.submitReadiness",
      ],
    ]);

    const graphCompatibilityDefinition = graph.exportCompatibilityDefinition();
    assert.equal(
      Object.getPrototypeOf(graphCompatibilityDefinition.outputs),
      null,
    );
    assert.deepEqual(graphCompatibilityDefinition.outputs.submitReadiness, "itemDetail.submitReadiness");
    assert.deepEqual(
      graphCompatibilityDefinition.definitions.sources,
      [
        { id: "serverItemData", initial: null },
        { id: "draftEdits", initial: {} },
      ],
    );
    assert.deepEqual(
      graphCompatibilityDefinition.definitions.recipes.map((recipe) => recipe.id),
      [
        "effectiveItemData",
        "dirtyState",
        "submitReadiness",
        "itemDetail.effectiveItemData",
        "itemDetail.dirtyState",
        "itemDetail.submitReadiness",
      ],
    );
  } finally {
    await cleanup();
  }
});
