import assert from "node:assert/strict";
import test from "node:test";

import { createGraphPublicationCase } from "../publication_fixture/graph_publication_case.mjs";


test("The Graph Publication Compatibility Export Surface Test", async () => {
  const {
    cleanup,
    rawSignals,
    graph,
    count,
    doubled,
    panel,
    calls,
  } = await createGraphPublicationCase();

  try {
    const compatibilityDefinition = graph.exportCompatibilityDefinition();
    assert.equal(Object.getPrototypeOf(compatibilityDefinition.outputs), null);
    assert.deepEqual(
      {
        ...compatibilityDefinition,
        contract: {
          ...compatibilityDefinition.contract,
          inputs: { ...compatibilityDefinition.contract.inputs },
          outputs: { ...compatibilityDefinition.contract.outputs },
        },
        inputs: { ...compatibilityDefinition.inputs },
        outputs: { ...compatibilityDefinition.outputs },
      },
      {
        id: "itemDetail",
        contract: {
          graph: graph.summary(),
          inputs: {},
          outputs: {
            count: "itemDetail.count",
            doubled: "itemDetail.doubled",
            panel: "itemDetail.panel",
          },
          inputDescriptors: [],
          descriptors: graph.descriptors(),
        },
        inputs: {},
        outputs: {
          count: "itemDetail.count",
          doubled: "itemDetail.doubled",
          panel: "itemDetail.panel",
        },
        inputSourceIds: [],
        publishedOutputIds: [
          "itemDetail.count",
          "itemDetail.doubled",
          "itemDetail.panel",
        ],
        sourceIds: [count.id, doubled.id, panel.id],
        inputDescriptors: [],
        descriptors: graph.descriptors(),
        definitions: {
          policy: { preset: "webDevelopment" },
          sources: [{ id: "count", initial: 1 }],
          recipes: [
            {
              id: "doubled",
              reads: ["count"],
              expr: {
                kind: "multiply",
                args: [
                  { kind: "read", id: "count" },
                  { kind: "value", value: 2 },
                ],
              },
            },
            {
              id: "itemDetail.count",
              reads: ["count"],
              expr: { kind: "read", id: "count" },
            },
            {
              id: "itemDetail.doubled",
              reads: ["doubled"],
              expr: { kind: "read", id: "doubled" },
            },
          ],
          sourceFamilies: [],
          recipeFamilies: [],
          unavailableCallbacks: [],
        },
      },
    );

    rawSignals.adapters = () => ({
      export_definitions() {
        return {
          policy: { preset: "webDevelopment" },
          sources: [{ id: "count", initial: 1 }],
          recipes: [
            {
              id: "doubled",
              reads: ["count"],
              expr: {
                kind: "multiply",
                args: [
                  { kind: "read", id: "count" },
                  { kind: "value", value: 2 },
                ],
              },
            },
            {
              id: "itemDetail.count",
              reads: ["count"],
              expr: { kind: "read", id: "count" },
            },
            {
              id: "itemDetail.doubled",
              reads: ["doubled"],
              expr: { kind: "read", id: "doubled" },
            },
            {
              id: "panel",
              reads: ["__WORTHSignal.outputProjection.panel.1"],
              expr: {
                kind: "read",
                id: "__WORTHSignal.outputProjection.panel.1",
              },
            },
          ],
          sourceFamilies: [],
          recipeFamilies: [],
          unavailableCallbacks: [
            {
              id: "__WORTHSignal.outputProjection.panel.1",
              signalKind: "computed",
              reason: "computeCallbackUnavailableForPortableExport",
              currentReads: ["count"],
              hostCapabilityReads: [],
              hostCapabilityTransports: [],
            },
          ],
        };
      },
      free() {},
    });

    const refreshedCompatibilityDefinition =
      graph.exportCompatibilityDefinition();
    assert.deepEqual(
      refreshedCompatibilityDefinition.definitions.unavailableCallbacks[0]
        ?.currentReads,
      undefined,
    );
    assert.equal(typeof graph.diagnostics().performanceSummary, "function");
    assert.equal(typeof graph.history, "function");
    assert.equal(typeof graph.specialist, "function");
    assert.equal(typeof graph.adapters, "function");
    assert.equal(typeof graph.compatibilityApp, "function");
    assert.equal(typeof graph.compatibilityRuntime, "function");

    const panelProjectionId = calls[2][1];
    assert.deepEqual(calls.slice(0, 7), [
      ["input", count.id, 1, {}],
      ["computedCallback", doubled.id, "function"],
      ["computedCallback", panelProjectionId, "function"],
      [
        "outputSpec",
        panel.id,
        {
          reads: [panelProjectionId],
          expr: {
            kind: "read",
            id: panelProjectionId,
          },
        },
      ],
      [
        "outputSpec",
        "itemDetail.count",
        {
          reads: [count.id],
          expr: {
            kind: "read",
            id: count.id,
          },
        },
      ],
      [
        "outputSpec",
        "itemDetail.doubled",
        {
          reads: [doubled.id],
          expr: {
            kind: "read",
            id: doubled.id,
          },
        },
      ],
      [
        "outputSpec",
        "itemDetail.panel",
        {
          reads: [panel.id],
          expr: {
            kind: "read",
            id: panel.id,
          },
        },
      ],
    ]);
  } finally {
    await cleanup();
  }
});
