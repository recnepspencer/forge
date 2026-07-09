import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createGraphPublicationRuntime } from "../runtime_fixture/graph_publication_runtime.mjs";

test("The Controller Contract Internal Boundary Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const rawSignals = createGraphPublicationRuntime();
    const signals = wrapSignals(rawSignals);

    const graph = signals.graph("itemDetail", (graphBuilder) => {
      const editSession = graphBuilder.scope("editSession");
      const serverItemData = editSession.input(null, { id: "serverItemData" });
      const effectiveItemData = editSession.computed(
        "effectiveItemData",
        () => ({
          ...(serverItemData() ?? {}),
        }),
      );
      const validationTrace = editSession.computed("validationTrace", () => ({
        fieldCount: Object.keys(serverItemData() ?? {}).length,
      }));

      const controller = editSession.controller({
        inputs: {
          serverItemData,
        },
        outputs: {
          effectiveItemData,
        },
        internal: {
          validationTrace,
        },
      });

      return graphBuilder.expose({
        controllers: [controller],
      });
    });

    assert.equal("validationTrace" in graph.contract().inputs, false);
    assert.equal("validationTrace" in graph.contract().outputs, false);
    assert.equal(
      graph
        .inputDescriptors()
        .some((descriptor) => descriptor.inputName === "validationTrace"),
      false,
    );
    assert.equal(
      graph
        .descriptors()
        .some((descriptor) => descriptor.outputName === "validationTrace"),
      false,
    );
    assert.equal("validationTrace" in graph.inspectDiagnostics().inputs, false);
    assert.equal(
      "validationTrace" in graph.inspectDiagnostics().outputs,
      false,
    );
    assert.equal("validationTrace" in graph.inspectHistory().inputs, false);
    assert.equal("validationTrace" in graph.inspectHistory().outputs, false);
    assert.equal(
      "validationTrace" in graph.exportCompatibilityDefinition().inputs,
      false,
    );
    assert.equal(
      "validationTrace" in graph.exportCompatibilityDefinition().outputs,
      false,
    );
  } finally {
    await cleanup();
  }
});


