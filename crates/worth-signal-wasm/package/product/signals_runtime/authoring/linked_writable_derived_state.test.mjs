import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createLinkedRuntime } from "../runtime_fixture/linked_runtime.mjs";

test("The Linked Writable Derived State Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createLinkedRuntime());
    const shippingOptions = signals.input(
      [
        { id: "ground", label: "Ground" },
        { id: "air", label: "Air" },
      ],
      { debugName: "shippingOptions" },
    );

    const firstOption = signals.linked(() => shippingOptions()[0], {
      debugName: "firstOption",
    });

    const preservedSelection = signals.linked({
      source: () => shippingOptions(),
      computation: (options, previous) =>
        options.find((option) => option.id === previous?.value?.id) ??
        options[0] ??
        null,
      debugName: "preservedSelection",
    });

    const selectionController = signals.controller(({ input, linked }) => {
      const options = input([
        { id: "draft", label: "Draft" },
        { id: "review", label: "Review" },
      ]);
      const selected = linked({
        source: () => options(),
        computation: (nextOptions, previous) =>
          nextOptions.find((option) => option.id === previous?.value?.id) ??
          nextOptions[0],
      });
      return {
        inputs: { options },
        outputs: { selected },
      };
    });

    assert.equal(firstOption.debugName, "firstOption");
    assert.equal(firstOption().id, "ground");
    assert.equal(preservedSelection().id, "ground");
    assert.equal(selectionController.outputs.selected().id, "draft");

    preservedSelection.set({ id: "air", label: "Air" });
    assert.equal(preservedSelection().id, "air");

    shippingOptions.set([
      { id: "ground", label: "Ground" },
      { id: "air", label: "Air" },
      { id: "sea", label: "Sea" },
    ]);

    assert.equal(firstOption().id, "ground");
    assert.equal(preservedSelection().id, "air");
    preservedSelection.relink();
    assert.equal(preservedSelection().id, "air");

    preservedSelection.set({ id: "manual", label: "Manual" });
    preservedSelection.reset();
    assert.equal(preservedSelection().id, "air");

    shippingOptions.set([
      { id: "sea", label: "Sea" },
      { id: "ground", label: "Ground" },
    ]);

    firstOption.set({ id: "manual", label: "Manual" });
    firstOption.reset();
    assert.equal(
      firstOption().id,
      "sea",
      "linked reset should read the current source-derived baseline even before relink",
    );
    firstOption.relink();
    assert.equal(firstOption().id, "sea");

    shippingOptions.set([
      { id: "sea", label: "Sea" },
      { id: "ground", label: "Ground" },
    ]);

    preservedSelection.set({ id: "manual", label: "Manual" });
    preservedSelection.relink();
    assert.equal(preservedSelection().id, "sea");

    const linkedGraph = signals.graph("linkedSelection", (graph) => {
      const selection = graph.scope("selection");
      const available = selection.input([
        { id: "draft", label: "Draft" },
        { id: "review", label: "Review" },
      ]);
      const chosen = selection.linked({
        source: () => available(),
        computation: (options, previous) =>
          options.find((option) => option.id === previous?.value?.id) ??
          options[0] ??
          null,
      });
      return graph.expose({
        inputs: {
          available,
          chosen,
        },
        outputs: {
          chosen,
        },
      });
    });

    linkedGraph.writeInputs({
      chosen: { id: "review", label: "Review" },
    });
    linkedGraph.writeInputs({
      available: [
        { id: "ready", label: "Ready" },
        { id: "review", label: "Review" },
      ],
    });
    linkedGraph.resetInputs(["chosen"]);
    assert.equal(
      linkedGraph.readInputs().chosen?.id,
      "ready",
      "graph reset should honor the current linked baseline rather than a stale initial baseline",
    );
    const linkedRevisionGraph = signals.graph(
      "linkedRevisionSelection",
      (graph) => {
        const selection = graph.scope("selection");
        const available = selection.input({
          revision: 1,
          options: [
            { id: "draft", label: "Draft" },
            { id: "review", label: "Review" },
          ],
        });
        const chosen = selection.linked({
          source: () => available(),
          computation: (source, previous) => {
            const preserved =
              previous && previous.source.revision === source.revision
                ? (source.options.find(
                    (option) => option.id === previous.value?.id,
                  ) ?? null)
                : null;
            return preserved ?? source.options[0] ?? null;
          },
        });
        return graph.expose({
          inputs: {
            available,
            chosen,
          },
          outputs: {
            chosen,
          },
        });
      },
    );

    linkedRevisionGraph.writeInputs({
      available: {
        revision: 2,
        options: [
          { id: "review", label: "Review" },
          { id: "ready", label: "Ready" },
        ],
      },
    });
    linkedRevisionGraph.resetInputs(["chosen"]);
    assert.equal(
      linkedRevisionGraph.readInputs().chosen?.id,
      "review",
      "linked graph reset should re-anchor to the current source-derived baseline",
    );
    linkedRevisionGraph.writeInputs({
      available: {
        revision: 2,
        options: [
          { id: "approved", label: "Approved" },
          { id: "review", label: "Review" },
        ],
      },
    });
    linkedRevisionGraph.resetInputs(["chosen"]);
    assert.equal(
      linkedRevisionGraph.readInputs().chosen?.id,
      "review",
      "graph reset should finalize linked baseline state so later resets preserve the latest valid baseline under the same source revision",
    );

    assert.throws(
      () => signals.linked(() => 1, { id: "count" }),
      /signals\.linked app authoring does not accept id/,
    );
    assert.throws(
      () => signals.linked({ source: () => 1, computation: "nope" }),
      /signals\.linked computation must be a function when provided/,
    );
  } finally {
    await cleanup();
  }
});


