import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createGraphPublicationRuntime } from "../runtime_fixture/graph_publication_runtime.mjs";
import { createRawReadableHandle } from "../runtime_fixture/raw_readable_handle.mjs";

test("The Controller Artifact Composition Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const rawSignals = createGraphPublicationRuntime();
    const signals = wrapSignals(rawSignals);

    function createEditSessionController(namespace) {
      return namespace.controller(({ input, computed }) => {
        const serverItemData = input(null, { id: "serverItemData" });
        const draftEdits = input({}, { id: "draftEdits" });
        const effectiveItemData = computed(
          () => ({
            ...(serverItemData() ?? {}),
            ...draftEdits(),
          }),
          { id: "effectiveItemData" },
        );
        const dirtyState = computed(
          () => ({
            isDirty: Object.keys(draftEdits()).length > 0,
          }),
          { id: "dirtyState" },
        );

        return {
          inputs: {
            serverItemData,
            draftEdits,
          },
          outputs: {
            effectiveItemData,
            dirtyState,
          },
        };
      });
    }

    function createWorkflowController(namespace, editSession) {
      return namespace.controller(({ computed }) => {
        const submitReadiness = computed(
          () => ({
            enabled: editSession.outputs.dirtyState().isDirty,
          }),
          { id: "submitReadiness" },
        );

        return {
          outputs: {
            submitReadiness,
          },
        };
      });
    }

    const graph = signals.graph("itemDetail", (graphBuilder) => {
      const editSession = graphBuilder.controller(
        "editSession",
        ({ input, computed }) => {
          const serverItemData = input(null, { id: "serverItemData" });
          const draftEdits = input({}, { id: "draftEdits" });
          const effectiveItemData = computed(
            () => ({
              ...(serverItemData() ?? {}),
              ...draftEdits(),
            }),
            { id: "effectiveItemData" },
          );
          const dirtyState = computed(
            () => ({
              isDirty: Object.keys(draftEdits()).length > 0,
            }),
            { id: "dirtyState" },
          );

          return {
            inputs: {
              serverItemData,
              draftEdits,
            },
            outputs: {
              effectiveItemData,
              dirtyState,
            },
          };
        },
      );
      const workflow = createWorkflowController(
        graphBuilder.scope("workflow"),
        editSession,
      );
      return graphBuilder.expose({
        controllers: [editSession, workflow],
      });
    });

    assert.deepEqual(
      { ...graph.readInputs() },
      {
        serverItemData: null,
        draftEdits: {},
      },
    );
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
          dirtyState: "itemDetail.dirtyState",
          submitReadiness: "itemDetail.submitReadiness",
        },
        inputDescriptors: graph.inputDescriptors(),
        descriptors: graph.descriptors(),
      },
    );

    assert.throws(
      () =>
        signals.graph("broken", (graphBuilder) => {
          const editSession = createEditSessionController(
            graphBuilder.scope("editSession"),
          );
          return graphBuilder.expose({
            controllers: [editSession],
            inputs: {
              serverItemData: editSession.inputs.serverItemData,
            },
          });
        }),
      /duplicate input name `serverItemData`/,
    );

    assert.throws(
      () =>
        signals.graph("broken", (graphBuilder) =>
          graphBuilder.expose({
            controllers: [{}],
            outputs: {
              label: graphBuilder
                .scope("editSession")
                .computed("label", () => "x"),
            },
          }),
        ),
      /must be a controller artifact created by signals\.controller/,
    );

    assert.throws(
      () => signals.controller(() => null),
      /signals\.controller requires a controller definition object/,
    );

    assert.throws(
      () =>
        signals.graph("brokenBuilder", (graphBuilder) =>
          graphBuilder.controller("editSession", () => null),
        ),
      /signals\.controller requires a controller definition object/,
    );

    assert.throws(
      () =>
        signals.controller({
          outputs: {
            leakedAuthority: signals.publicInput(
              signals.input({ taskId: "task-7" }, { debugName: "routeParams" }),
              { authority: "imported" },
            ),
          },
        }),
      /controller\.outputs\.`leakedAuthority` cannot use signals\.publicInput/,
    );

    assert.throws(
      () =>
        signals.controller({
          internal: {
            leakedAuthority: signals.publicInput(
              signals.input(
                { taskId: "task-7" },
                { debugName: "routeParamsInternal" },
              ),
              { authority: "readOnly" },
            ),
          },
        }),
      /controller\.internal\.`leakedAuthority` cannot use signals\.publicInput/,
    );

    assert.throws(
      () =>
        signals.controller({
          inputs: {
            notAnInput: signals.computed(() => "nope", {
              debugName: "notAnInput",
            }),
          },
        }),
      /controller\.inputs\.`notAnInput` must be an input handle or signals\.publicInput/,
    );
  } finally {
    await cleanup();
  }
});

test("nested controller scopes support computedSpec and outputSpec without crashing", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const calls = [];
    const rawSignals = {
      input(id, initial, options) {
        calls.push(["input", id, initial, options]);
        return createRawReadableHandle(id, initial);
      },
      computedSpec(id, spec) {
        calls.push(["computedSpec", id, spec]);
        return createRawReadableHandle(id, { family: "computedSpec", spec });
      },
      computedCallback() {
        throw new Error("computedCallback not needed");
      },
      outputSpec(id, spec) {
        calls.push(["outputSpec", id, spec]);
        return createRawReadableHandle(id, { family: "outputSpec", spec });
      },
      read(target) {
        return typeof target === "string" ? target : target.id;
      },
      watch() {
        throw new Error("watch not needed");
      },
      effect() {
        throw new Error("effect not needed");
      },
      transaction() {
        throw new Error("transaction not needed");
      },
      batch() {
        throw new Error("batch not needed");
      },
      nuke() {
        return true;
      },
      diagnostics() {
        throw new Error("diagnostics not needed");
      },
      history() {
        throw new Error("history not needed");
      },
      specialist() {
        throw new Error("specialist not needed");
      },
      adapters() {
        throw new Error("adapters not needed");
      },
      compatibilityApp() {
        throw new Error("compatibilityApp not needed");
      },
      compatibilityRuntime() {
        throw new Error("compatibilityRuntime not needed");
      },
      free() {},
    };

    const signals = wrapSignals(rawSignals);
    const controller = signals.controller((surface) => {
      const nested = surface.scope("nested");
      const count = nested.input(1, { id: "count" });
      const label = nested.computedSpec("label", { kind: "literal", value: "ok" });
      const panel = nested.outputSpec("panel", { kind: "literal", value: "done" });
      return {
        inputs: { count },
        outputs: { label, panel },
      };
    });

    assert.equal(controller.outputs.label.id, "nested.label");
    assert.equal(controller.outputs.panel.id, "nested.panel");
    assert.deepEqual(
      calls.map(([kind, id]) => [kind, id]),
      [
        ["input", "nested.count"],
        ["computedSpec", "nested.label"],
        ["outputSpec", "nested.panel"],
      ],
    );
  } finally {
    await cleanup();
  }
});


