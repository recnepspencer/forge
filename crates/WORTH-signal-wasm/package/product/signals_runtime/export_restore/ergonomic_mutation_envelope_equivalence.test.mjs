import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../runtime_fixture/graph_operational_runtime.mjs";

test("The Ergonomic Mutation Envelope Equivalence Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const rawSignals = createGraphOperationalRuntime();
    const signals = wrapSignals(rawSignals);

    function createEditSessionController(namespace) {
      const serverItemData = namespace.input(null, { id: "serverItemData" });
      const draftEdits = namespace.input({}, { id: "draftEdits" });
      const effectiveItemData = namespace.computed(
        () => ({
          ...(serverItemData() ?? {}),
          ...draftEdits(),
        }),
        { id: "effectiveItemData" },
      );

      return namespace.controller({
        inputs: {
          serverItemData,
          draftEdits,
        },
        outputs: {
          effectiveItemData,
        },
      });
    }

    const graph = signals.graph("itemDetail", (graphBuilder) => {
      const editSession = createEditSessionController(
        graphBuilder.scope("editSession"),
      );
      return graphBuilder.expose({
        controllers: [editSession],
      });
    });
    const foreignGraph = signals.graph("otherDetail", (graphBuilder) => {
      const editSession = createEditSessionController(
        graphBuilder.scope("editSession"),
      );
      return graphBuilder.expose({
        controllers: [editSession],
      });
    });

    assert.deepEqual(
      {
        ...graph.operationalContract(),
        writes: { ...graph.operationalContract().writes },
        patches: { ...graph.operationalContract().patches },
        commands: { ...graph.operationalContract().commands },
        authorities: Object.fromEntries(
          Object.entries(graph.operationalContract().authorities).map(
            ([inputName, authority]) => [inputName, { ...authority }],
          ),
        ),
      },
      {
        graph: graph.summary(),
        writes: {
          serverItemData: "itemDetail.editSession.serverItemData",
          draftEdits: "itemDetail.editSession.draftEdits",
        },
        patches: {
          draftEdits: "itemDetail.editSession.draftEdits",
        },
        commands: {},
        authorities: {
          serverItemData: {
            inputName: "serverItemData",
            sourceId: "itemDetail.editSession.serverItemData",
            authority: "writable",
            requiredness: "required",
            supportsWrite: true,
            supportsPatch: false,
            supportsReset: true,
          },
          draftEdits: {
            inputName: "draftEdits",
            sourceId: "itemDetail.editSession.draftEdits",
            authority: "writable",
            requiredness: "required",
            supportsWrite: true,
            supportsPatch: true,
            supportsReset: true,
          },
        },
        resettableInputNames: ["serverItemData", "draftEdits"],
      },
    );

    graph.writeInputs({
      serverItemData: {
        workflow_target_state_id: 7,
      },
    });
    assert.deepEqual(
      { ...graph.readInputs() },
      {
        serverItemData: {
          workflow_target_state_id: 7,
        },
        draftEdits: {},
      },
    );

    graph.patchInputs({
      draftEdits: {
        title: "Ship docs",
      },
    });
    assert.deepEqual(
      { ...graph.readInputs() },
      {
        serverItemData: {
          workflow_target_state_id: 7,
        },
        draftEdits: {
          title: "Ship docs",
        },
      },
    );

    graph.transaction((tx) => {
      tx.set("draftEdits", {
        title: "Ready to ship",
      });
    });
    assert.deepEqual(graph.readInputs().draftEdits, {
      title: "Ready to ship",
    });

    graph.apply({
      writes: {
        serverItemData: {
          workflow_target_state_id: 12,
        },
      },
      patches: {
        draftEdits: {
          priority: "high",
        },
      },
    });
    assert.deepEqual(
      { ...graph.readInputs() },
      {
        serverItemData: {
          workflow_target_state_id: 12,
        },
        draftEdits: {
          title: "Ready to ship",
          priority: "high",
        },
      },
    );

    graph.resetInputs(["draftEdits"]);
    assert.deepEqual(
      { ...graph.readInputs() },
      {
        serverItemData: {
          workflow_target_state_id: 12,
        },
        draftEdits: {},
      },
    );

    graph.resetInputs();
    assert.deepEqual(
      { ...graph.readInputs() },
      {
        serverItemData: null,
        draftEdits: {},
      },
    );

    assert.throws(
      () =>
        graph.writeInputs({
          missingInput: 7,
        }),
      /itemDetail\.missingInput.*public input contract/,
    );
    assert.throws(
      () =>
        graph.patchInputs({
          serverItemData: {
            title: "Nope",
          },
        }),
      /does not admit patches for it/,
    );
    assert.throws(
      () =>
        graph.apply({
          writes: {
            draftEdits: {},
          },
          reset: ["draftEdits"],
        }),
      /cannot both write and reset public input `draftEdits`/,
    );
    assert.throws(
      () =>
        graph.transaction((tx) => {
          tx.set(foreignGraph.inputs.draftEdits, {});
        }),
      /outside the graph contract/,
    );

    assert.deepEqual(
      rawSignals.callLog
        .filter(([family]) => family === "transaction")
        .map(([, ops]) => ops),
      [
        [
          [
            "set",
            "itemDetail.editSession.serverItemData",
            { workflow_target_state_id: 7 },
          ],
        ],
        [["set", "itemDetail.editSession.draftEdits", { title: "Ship docs" }]],
        [
          [
            "set",
            "itemDetail.editSession.draftEdits",
            { title: "Ready to ship" },
          ],
        ],
        [
          [
            "set",
            "itemDetail.editSession.serverItemData",
            { workflow_target_state_id: 12 },
          ],
          [
            "set",
            "itemDetail.editSession.draftEdits",
            { title: "Ready to ship", priority: "high" },
          ],
        ],
        [["set", "itemDetail.editSession.draftEdits", {}]],
        [
          ["set", "itemDetail.editSession.serverItemData", null],
          ["set", "itemDetail.editSession.draftEdits", {}],
        ],
      ],
    );
  } finally {
    await cleanup();
  }
});


