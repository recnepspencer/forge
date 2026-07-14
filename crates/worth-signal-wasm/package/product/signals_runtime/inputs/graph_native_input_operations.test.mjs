import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../runtime_fixture/graph_operational_runtime.mjs";

test("The Graph-Native Input Operations Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const rawSignals = createGraphOperationalRuntime();
    const signals = wrapSignals(rawSignals);
    const localDraft = signals.input(
      {
        title: "Ship docs",
        done: false,
      },
      { debugName: "localDraft" },
    );

    const graph = signals.graph("taskAuthority", (graphBuilder) => {
      const scope = graphBuilder.scope("authority");
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
          taskId: externalParams().taskId,
        }),
        { id: "effectiveValue" },
      );

      return graphBuilder.expose({
        controllers: [
          scope.controller({
            inputs: {
              serverValue: scope.publicInput(serverValue, {
                authority: "readOnly",
              }),
              draftValue: scope.publicInput(draftValue),
              externalParams: scope.publicInput(externalParams, {
                authority: "imported",
              }),
            },
            outputs: {
              effectiveValue,
            },
          }),
        ],
      });
    });

    assert.equal(
      graph.input("draftValue").id,
      "taskAuthority.authority.draftValue",
    );
    assert.equal(
      graph.inputs.externalParams.id,
      "taskAuthority.authority.externalParams",
    );
    assert.equal(
      graph.output("effectiveValue").id,
      "taskAuthority.effectiveValue",
    );
    assert.deepEqual(
      { ...graph.readInputs() },
      {
        serverValue: {
          id: "task-7",
          title: "Ship docs",
        },
        draftValue: {
          title: "Ship docs",
        },
        externalParams: {
          taskId: "task-7",
        },
      },
    );

    localDraft.patch({
      done: true,
    });
    localDraft.assign({
      title: "Ready to ship",
    });
    signals.transaction((tx) => {
      tx.patch(localDraft, {
        status: "queued",
      });
    });
    assert.deepEqual(localDraft(), {
      title: "Ready to ship",
      done: true,
      status: "queued",
    });
    assert.throws(
      () => signals.input(1, { debugName: "primitiveCount" }).patch(2),
      /input\.patch\(\.\.\.\) requires object or array values/,
    );

    graph.writeInputs({
      draftValue: {
        title: "Ready to ship",
      },
    });
    graph.writeInput("draftValue", {
      title: "Reviewed",
    });
    graph.patchInputs({
      draftValue: {
        status: "queued",
      },
    });
    graph.patchInput("draftValue", {
      priority: "high",
    });
    graph.transaction((tx) => {
      tx.set("draftValue", {
        title: "Queued",
        status: "queued",
        priority: "high",
      });
    });
    graph.transaction((tx) => {
      tx.patch("draftValue", {
        reviewer: "Avery",
      });
    });
    graph.apply({
      writes: {
        draftValue: {
          title: "Approved",
          status: "approved",
          priority: "high",
          reviewer: "Avery",
        },
      },
      commands: {},
    });
    graph.resetInput("draftValue");

    assert.deepEqual(
      { ...graph.readInputs() },
      {
        serverValue: {
          id: "task-7",
          title: "Ship docs",
        },
        draftValue: {
          title: "Ship docs",
        },
        externalParams: {
          taskId: "task-7",
        },
      },
    );
    assert.equal(
      graph.read().effectiveValue.id,
      "taskAuthority.effectiveValue",
    );
    assert.throws(
      () =>
        graph.transaction((tx) => {
          tx.patch("serverValue", {
            title: "Nope",
          });
        }),
      /cannot patch public input `serverValue`/,
    );
  } finally {
    await cleanup();
  }
});


