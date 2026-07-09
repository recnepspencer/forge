import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createGraphPublicationRuntime } from "../runtime_fixture/graph_publication_runtime.mjs";

test("The Forms And Resources Dependency Readiness Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const rawSignals = createGraphPublicationRuntime();
    const signals = wrapSignals(rawSignals);

    function createFormController(namespace) {
      const fields = namespace.scope("fields");
      const serverValue = fields.input(
        {
          id: "task-7",
          title: "Ship docs",
          status: "draft",
        },
        { id: "serverValue" },
      );
      const draftValue = fields.input(
        {
          title: "Ship docs",
          status: "ready",
        },
        { id: "draftValue" },
      );
      const effectiveValue = fields.computed(
        () => ({
          ...(serverValue() ?? {}),
          ...draftValue(),
        }),
        { id: "effectiveValue" },
      );
      const dirtyState = fields.computed(
        () => ({
          isDirty: Object.keys(draftValue()).length > 0,
        }),
        { id: "dirtyState" },
      );
      const validation = namespace.computed(
        () => ({
          titleMissing: !effectiveValue().title,
        }),
        { id: "validation" },
      );

      return namespace.controller({
        inputs: {
          serverValue,
          draftValue,
        },
        outputs: {
          effectiveValue,
          dirtyState,
          validation,
        },
      });
    }

    function createResourceController(namespace, form) {
      const routeParams = namespace.input(
        {
          taskId: "task-7",
          workspaceId: "alpha",
        },
        { id: "routeParams" },
      );
      const resourceQuery = namespace.computed(
        () => ({
          taskId: routeParams().taskId,
          workspaceId: routeParams().workspaceId,
          status: form.outputs.effectiveValue().status,
        }),
        { id: "resourceQuery" },
      );
      const submitAvailability = namespace.computed(
        () => ({
          enabled:
            form.outputs.dirtyState().isDirty &&
            !form.outputs.validation().titleMissing,
          taskId: resourceQuery().taskId,
        }),
        { id: "submitAvailability" },
      );

      return namespace.controller({
        inputs: {
          routeParams,
        },
        outputs: {
          resourceQuery,
          submitAvailability,
        },
      });
    }

    const graph = signals.graph("taskEditor", (graphBuilder) => {
      const form = createFormController(graphBuilder.scope("form"));
      const resource = createResourceController(
        graphBuilder.scope("resource"),
        form,
      );

      return graphBuilder.expose({
        controllers: [form, resource],
      });
    });

    assert.deepEqual(
      { ...graph.readInputs() },
      {
        serverValue: {
          id: "task-7",
          title: "Ship docs",
          status: "draft",
        },
        draftValue: {
          title: "Ship docs",
          status: "ready",
        },
        routeParams: {
          taskId: "task-7",
          workspaceId: "alpha",
        },
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
          serverValue: "taskEditor.form.fields.serverValue",
          draftValue: "taskEditor.form.fields.draftValue",
          routeParams: "taskEditor.resource.routeParams",
        },
        outputs: {
          effectiveValue: "taskEditor.effectiveValue",
          dirtyState: "taskEditor.dirtyState",
          validation: "taskEditor.validation",
          resourceQuery: "taskEditor.resourceQuery",
          submitAvailability: "taskEditor.submitAvailability",
        },
        inputDescriptors: graph.inputDescriptors(),
        descriptors: graph.descriptors(),
      },
    );
    assert.equal(
      graph.inspectDiagnostics().inputs.routeParams.why.id,
      "taskEditor.resource.routeParams",
    );
    assert.equal(
      graph.inspectDiagnostics().outputs.resourceQuery.why.id,
      "taskEditor.resourceQuery",
    );
    assert.equal(
      graph.inspectHistory().inputs.serverValue.replay.id,
      "taskEditor.form.fields.serverValue",
    );
    assert.equal(
      graph.inspectHistory().outputs.submitAvailability.lineage.id,
      "taskEditor.submitAvailability",
    );
    assert.equal(
      graph.exportCompatibilityDefinition().contract.inputs.routeParams,
      "taskEditor.resource.routeParams",
    );
    assert.equal(
      graph.exportCompatibilityDefinition().contract.outputs.submitAvailability,
      "taskEditor.submitAvailability",
    );
  } finally {
    await cleanup();
  }
});


