import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createGraphPublicationRuntime } from "../runtime_fixture/graph_publication_runtime.mjs";

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


