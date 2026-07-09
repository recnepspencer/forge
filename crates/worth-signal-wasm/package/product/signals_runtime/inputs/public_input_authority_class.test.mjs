import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../runtime_fixture/graph_operational_runtime.mjs";

test("The Public Input Authority Class Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const rawSignals = createGraphOperationalRuntime();
    const signals = wrapSignals(rawSignals);

    const graph = signals.graph("taskEditor", (builder) => {
      const scope = builder.scope("form");
      const serverValue = scope.input(
        {
          id: "task-7",
          title: "Ship docs",
        },
        { id: "serverValue" },
      );
      const draftValue = scope.input(
        {
          title: "Ship docs",
        },
        { id: "draftValue" },
      );
      const externalParams = scope.input(
        {
          taskId: "task-7",
        },
        { id: "externalParams" },
      );
      const effectiveValue = scope.computed(
        () => ({
          ...serverValue(),
          ...draftValue(),
        }),
        { id: "effectiveValue" },
      );

      return builder.expose({
        inputs: {
          serverValue: scope.publicInput(serverValue, {
            authority: "readOnly",
          }),
          draftValue: scope.publicInput(draftValue, { authority: "writable" }),
          externalParams: scope.publicInput(externalParams, {
            authority: "imported",
          }),
        },
        outputs: {
          effectiveValue,
        },
      });
    });

    assert.deepEqual(
      graph.inputDescriptors().map((descriptor) => ({
        inputName: descriptor.inputName,
        authority: descriptor.authority,
        requiredness: descriptor.requiredness,
      })),
      [
        {
          inputName: "serverValue",
          authority: "readOnly",
          requiredness: "required",
        },
        {
          inputName: "draftValue",
          authority: "writable",
          requiredness: "required",
        },
        {
          inputName: "externalParams",
          authority: "imported",
          requiredness: "required",
        },
      ],
    );
    assert.deepEqual(
      {
        serverValue: graph.operationalContract().authorities.serverValue,
        draftValue: graph.operationalContract().authorities.draftValue,
        externalParams: graph.operationalContract().authorities.externalParams,
      },
      {
        serverValue: {
          inputName: "serverValue",
          sourceId: "taskEditor.form.serverValue",
          authority: "readOnly",
          requiredness: "required",
          supportsWrite: false,
          supportsPatch: false,
          supportsReset: false,
        },
        draftValue: {
          inputName: "draftValue",
          sourceId: "taskEditor.form.draftValue",
          authority: "writable",
          requiredness: "required",
          supportsWrite: true,
          supportsPatch: true,
          supportsReset: true,
        },
        externalParams: {
          inputName: "externalParams",
          sourceId: "taskEditor.form.externalParams",
          authority: "imported",
          requiredness: "required",
          supportsWrite: false,
          supportsPatch: false,
          supportsReset: false,
        },
      },
    );

    graph.writeInputs({
      draftValue: {
        title: "Ready to ship",
      },
    });
    assert.deepEqual(graph.readInputs().draftValue, {
      title: "Ready to ship",
    });

    assert.throws(
      () =>
        graph.writeInputs({
          serverValue: {
            id: "task-7",
            title: "Nope",
          },
        }),
      /cannot write public input `serverValue` because its authority is `readOnly`/,
    );
    assert.throws(
      () =>
        graph.patchInputs({
          externalParams: {
            taskId: "task-8",
          },
        }),
      /authority is `imported`/,
    );
    assert.throws(
      () => graph.resetInputs(["serverValue"]),
      /cannot reset public input `serverValue` because its authority is `readOnly`/,
    );
    assert.throws(
      () =>
        graph.transaction((tx) => {
          tx.set("externalParams", { taskId: "task-9" });
        }),
      /authority is `imported`/,
    );
  } finally {
    await cleanup();
  }
});


