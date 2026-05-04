import assert from "node:assert/strict";
import test from "node:test";

import { createGraphPublicationCase } from "../publication_fixture/graph_publication_case.mjs";
import { createGraphPublicationRuntime } from "../runtime_fixture/graph_publication_runtime.mjs";
import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";


test("The Graph Publication Contract Surface Test", async () => {
  const {
    cleanup,
    graph,
    count,
    doubled,
    panel,
  } = await createGraphPublicationCase();

  try {
    assert.equal(graph.id, "itemDetail");
    assert.equal(graph.outputs.count.id, "itemDetail.count");
    assert.equal(graph.outputs.doubled.id, "itemDetail.doubled");
    assert.equal(graph.outputs.panel.id, "itemDetail.panel");

    const graphSnapshot = graph.read();
    assert.equal(Object.getPrototypeOf(graphSnapshot), null);
    assert.deepEqual(
      { ...graphSnapshot },
      {
        count: {
          id: "itemDetail.count",
          spec: {
            reads: [count.id],
            expr: {
              kind: "read",
              id: count.id,
            },
          },
        },
        doubled: {
          id: "itemDetail.doubled",
          spec: {
            reads: [doubled.id],
            expr: {
              kind: "read",
              id: doubled.id,
            },
          },
        },
        panel: {
          id: "itemDetail.panel",
          spec: {
            reads: [panel.id],
            expr: {
              kind: "read",
              id: panel.id,
            },
          },
        },
      },
    );

    assert.deepEqual(graph.output("count")(), {
      id: "itemDetail.count",
      spec: {
        reads: [count.id],
        expr: {
          kind: "read",
          id: count.id,
        },
      },
    });

    assert.deepEqual(graph.summary(), {
      id: "itemDetail",
      inputCount: 0,
      inputNames: [],
      inputSourceIds: [],
      outputCount: 3,
      outputNames: ["count", "doubled", "panel"],
      publishedOutputIds: [
        "itemDetail.count",
        "itemDetail.doubled",
        "itemDetail.panel",
      ],
      sourceIds: [count.id, doubled.id, panel.id],
      synthesizedOutputCount: 3,
    });

    assert.deepEqual(graph.descriptors(), [
      {
        outputName: "count",
        sourceId: count.id,
        sourceKind: "input",
        publishedId: "itemDetail.count",
        publicationKind: "synthesizedOutput",
      },
      {
        outputName: "doubled",
        sourceId: doubled.id,
        sourceKind: "computed",
        publishedId: "itemDetail.doubled",
        publicationKind: "synthesizedOutput",
      },
      {
        outputName: "panel",
        sourceId: panel.id,
        sourceKind: "output",
        publishedId: "itemDetail.panel",
        publicationKind: "synthesizedOutput",
      },
    ]);
  } finally {
    await cleanup();
  }
});
