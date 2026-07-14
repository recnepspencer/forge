import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../runtime_fixture/graph_operational_runtime.mjs";

test("The Public Input Requiredness Contract Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const rawSignals = createGraphOperationalRuntime();
    const signals = wrapSignals(rawSignals);

    const graph = signals.graph("taskRequiredness", (builder) => {
      const scope = builder.scope("requiredness");
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
      const effectiveValue = scope.computed(
        () => ({
          ...serverValue(),
          ...draftValue(),
        }),
        { id: "effectiveValue" },
      );

      return builder.expose({
        inputs: {
          serverValue: builder.input.required(serverValue, {
            authority: "readOnly",
          }),
          draftValue: builder.input.optional(draftValue),
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
          requiredness: "optional",
        },
      ],
    );
    assert.equal(
      graph.operationalContract().authorities.serverValue.requiredness,
      "required",
    );
    assert.equal(
      graph.operationalContract().authorities.draftValue.requiredness,
      "optional",
    );
    assert.equal(graph.contract().inputDescriptors[0].requiredness, "required");
    assert.equal(
      graph.exportDefinition().inputDescriptors[1].requiredness,
      "optional",
    );
    assert.deepEqual(
      graph.contractDelta({
        ...graph.contract(),
        inputDescriptors: graph
          .contract()
          .inputDescriptors.map((descriptor) =>
            descriptor.inputName === "draftValue"
              ? { ...descriptor, requiredness: "required" }
              : descriptor,
          ),
      }).inputDescriptorsChanged,
      [
        {
          inputName: "draftValue",
          previousSourceId: "taskRequiredness.requiredness.draftValue",
          currentSourceId: "taskRequiredness.requiredness.draftValue",
          previousAuthority: "writable",
          currentAuthority: "writable",
          previousRequiredness: "required",
          currentRequiredness: "optional",
        },
      ],
    );
    assert.throws(
      () =>
        signals.graph("invalidRequiredness", (invalidBuilder) => {
          const scope = invalidBuilder.scope("boundary");
          const source = scope.input(1, { id: "source" });
          return invalidBuilder.expose({
            inputs: {
              source: invalidBuilder.input.required(source, {
                authority: "readOnly",
                requiredness: "optional",
              }),
            },
            outputs: {
              sourceEcho: scope.output(() => source(), { id: "sourceEcho" }),
            },
          });
        }),
      {
        name: "TypeError",
        message:
          "signals.graph `invalidRequiredness` input.required(...) does not accept an explicit requiredness override; use input.required(...) to choose the boundary contract form",
      },
    );
  } finally {
    await cleanup();
  }
});


