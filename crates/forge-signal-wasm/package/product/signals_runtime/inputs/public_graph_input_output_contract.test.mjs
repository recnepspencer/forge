import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createGraphPublicationRuntime } from "../runtime_fixture/graph_publication_runtime.mjs";

test("The Public Graph Input And Output Contract Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const rawSignals = createGraphPublicationRuntime();
    rawSignals.adapters = () => ({
      export_definitions() {
        return {
          policy: { preset: "webDevelopment" },
          sources: [
            { id: "itemDetail.editSession.serverItemData", initial: null },
            { id: "itemDetail.editSession.draftEdits", initial: {} },
          ],
          recipes: [
            {
              id: "itemDetail.editSession.effectiveItemData",
              reads: [
                "itemDetail.editSession.serverItemData",
                "itemDetail.editSession.draftEdits",
              ],
              expr: {
                kind: "mergeObjects",
                args: [
                  { kind: "read", id: "itemDetail.editSession.serverItemData" },
                  { kind: "read", id: "itemDetail.editSession.draftEdits" },
                ],
              },
            },
            {
              id: "itemDetail.effectiveItemData",
              reads: ["itemDetail.editSession.effectiveItemData"],
              expr: {
                kind: "read",
                id: "itemDetail.editSession.effectiveItemData",
              },
            },
          ],
          sourceFamilies: [],
          recipeFamilies: [],
          unavailableCallbacks: [],
        };
      },
    });
    const signals = wrapSignals(rawSignals);

    const graph = signals.graph("itemDetail", (builder) => {
      const edit = builder.scope("editSession");
      const serverItemData = edit.input(null, { id: "serverItemData" });
      const draftEdits = edit.input({}, { id: "draftEdits" });
      const effectiveItemData = edit.computed("effectiveItemData", () => ({
        ...(serverItemData() ?? {}),
        ...draftEdits(),
      }));

      return builder.expose({
        inputs: {
          serverItemData,
          draftEdits,
        },
        outputs: {
          effectiveItemData,
        },
      });
    });

    assert.equal(
      graph.input("serverItemData").id,
      "itemDetail.editSession.serverItemData",
    );
    assert.equal(
      graph.inputs.draftEdits.id,
      "itemDetail.editSession.draftEdits",
    );
    assert.equal(
      graph.output("effectiveItemData").id,
      "itemDetail.effectiveItemData",
    );
    assert.deepEqual(
      { ...graph.readInputs() },
      {
        serverItemData: null,
        draftEdits: {},
      },
    );
    assert.deepEqual(
      { ...graph.read() },
      {
        effectiveItemData: {
          id: "itemDetail.effectiveItemData",
          spec: {
            reads: ["itemDetail.editSession.effectiveItemData"],
            expr: {
              kind: "read",
              id: "itemDetail.editSession.effectiveItemData",
            },
          },
        },
      },
    );
    assert.deepEqual(graph.summary(), {
      id: "itemDetail",
      inputCount: 2,
      inputNames: ["serverItemData", "draftEdits"],
      inputSourceIds: [
        "itemDetail.editSession.serverItemData",
        "itemDetail.editSession.draftEdits",
      ],
      outputCount: 1,
      outputNames: ["effectiveItemData"],
      publishedOutputIds: ["itemDetail.effectiveItemData"],
      sourceIds: [
        "itemDetail.editSession.serverItemData",
        "itemDetail.editSession.draftEdits",
        "itemDetail.editSession.effectiveItemData",
      ],
      synthesizedOutputCount: 1,
    });
    assert.deepEqual(graph.inputDescriptors(), [
      {
        inputName: "serverItemData",
        sourceId: "itemDetail.editSession.serverItemData",
        sourceKind: "input",
        authority: "writable",
        requiredness: "required",
      },
      {
        inputName: "draftEdits",
        sourceId: "itemDetail.editSession.draftEdits",
        sourceKind: "input",
        authority: "writable",
        requiredness: "required",
      },
    ]);
    assert.deepEqual(
      {
        ...graph.contract(),
        inputs: { ...graph.contract().inputs },
        outputs: { ...graph.contract().outputs },
      },
      {
        graph: graph.summary(),
        inputs: {
          serverItemData: "itemDetail.editSession.serverItemData",
          draftEdits: "itemDetail.editSession.draftEdits",
        },
        outputs: {
          effectiveItemData: "itemDetail.effectiveItemData",
        },
        inputDescriptors: graph.inputDescriptors(),
        descriptors: graph.descriptors(),
      },
    );
    const previousContractSnapshot = {
      ...graph.contract(),
      outputs: {},
    };
    assert.deepEqual(graph.contractDelta(previousContractSnapshot), {
      graphId: "itemDetail",
      previousGraphId: "itemDetail",
      changed: true,
      inputs: {
        added: [],
        removed: [],
        remapped: [],
      },
      outputs: {
        added: ["effectiveItemData"],
        removed: [],
        remapped: [],
      },
      inputDescriptorsChanged: [],
      outputDescriptorsChanged: [],
    });
    const graphDiagnosticsSurface = graph.inspectDiagnostics();
    assert.equal(Object.getPrototypeOf(graphDiagnosticsSurface.inputs), null);
    assert.deepEqual(graphDiagnosticsSurface.contract, graph.contract());
    assert.deepEqual(graphDiagnosticsSurface.inputVersions, [
      {
        id: "itemDetail.editSession.serverItemData",
        value_version: 1,
        shape_version: 10,
      },
      {
        id: "itemDetail.editSession.draftEdits",
        value_version: 2,
        shape_version: 11,
      },
    ]);
    assert.deepEqual(
      graphDiagnosticsSurface.dependenciesForOutput("effectiveItemData"),
      {
        graphId: "itemDetail",
        outputName: "effectiveItemData",
        publishedId: "itemDetail.effectiveItemData",
        sourceId: "itemDetail.editSession.effectiveItemData",
        publicInputNames: ["serverItemData", "draftEdits"],
        publicInputSourceIds: [
          "itemDetail.editSession.serverItemData",
          "itemDetail.editSession.draftEdits",
        ],
        transitiveSignalIds: [
          "itemDetail.effectiveItemData",
          "itemDetail.editSession.effectiveItemData",
          "itemDetail.editSession.serverItemData",
          "itemDetail.editSession.draftEdits",
        ],
      },
    );
    assert.deepEqual(graphDiagnosticsSurface.contractSummary(), {
      graph: graph.summary(),
      contract: graph.contract(),
      inputCount: 2,
      outputCount: 1,
      inputNames: ["serverItemData", "draftEdits"],
      outputNames: ["effectiveItemData"],
      dependencies: graphDiagnosticsSurface.dependencies,
    });
    assert.deepEqual(graphDiagnosticsSurface.inputs.serverItemData, {
      descriptor: graph.inputDescriptors()[0],
      version: {
        id: "itemDetail.editSession.serverItemData",
        value_version: 1,
        shape_version: 10,
      },
      why: { id: "itemDetail.editSession.serverItemData", family: "why" },
    });
    assert.deepEqual(graphDiagnosticsSurface.outputs.effectiveItemData, {
      descriptor: graph.descriptors()[0],
      version: {
        id: "itemDetail.effectiveItemData",
        value_version: 1,
        shape_version: 10,
      },
      why: { id: "itemDetail.effectiveItemData", family: "why" },
    });
    const graphHistorySurface = graph.inspectHistory();
    assert.equal(Object.getPrototypeOf(graphHistorySurface.inputs), null);
    assert.deepEqual(graphHistorySurface.contract, graph.contract());
    assert.deepEqual(
      graphHistorySurface.dependenciesForOutput("effectiveItemData"),
      graphDiagnosticsSurface.dependenciesForOutput("effectiveItemData"),
    );
    assert.deepEqual(
      graphHistorySurface.contractSummary(),
      graphDiagnosticsSurface.contractSummary(),
    );
    assert.deepEqual(graphHistorySurface.inputs.draftEdits, {
      descriptor: graph.inputDescriptors()[1],
      replay: { id: "itemDetail.editSession.draftEdits", family: "replay" },
      lineage: { id: "itemDetail.editSession.draftEdits", family: "lineage" },
    });
    const compatibilityDefinition = graph.exportCompatibilityDefinition();
    assert.deepEqual(compatibilityDefinition.contract, graph.contract());
    assert.deepEqual(
      { ...compatibilityDefinition.inputs },
      {
        serverItemData: "itemDetail.editSession.serverItemData",
        draftEdits: "itemDetail.editSession.draftEdits",
      },
    );
    assert.throws(
      () =>
        signals.graph("broken", (builder) =>
          builder.expose({
            inputs: {
              notAnInput: builder.scope("edit").computed("label", () => "x"),
            },
            outputs: {
              label: builder.scope("edit").computed("label2", () => "y"),
            },
          }),
        ),
      /expects an input handle/,
    );
  } finally {
    await cleanup();
  }
});

