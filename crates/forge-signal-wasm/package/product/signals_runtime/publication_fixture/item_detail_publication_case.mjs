import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createRawReadableHandle } from "../runtime_fixture/raw_readable_handle.mjs";

function createEditSessionController(namespace) {
  const serverItemData = namespace.spec.input("serverItemData", null);
  const draftEdits = namespace.spec.input("draftEdits", {});
  const effectiveItemData = namespace.spec.computedCallback(
    "effectiveItemData",
    () => ({
      ...(serverItemData() ?? {}),
      ...(draftEdits() ?? {}),
    }),
  );
  const dirtyState = namespace.spec.computedCallback("dirtyState", () => ({
    isDirty: Object.keys(draftEdits()).length > 0,
  }));
  return {
    serverItemData,
    draftEdits,
    effectiveItemData,
    dirtyState,
  };
}

function createWorkflowController(namespace, editSession) {
  const submitReadiness = namespace.spec.computedCallback(
    "submitReadiness",
    () => {
      const item = editSession.effectiveItemData();
      const dirty = editSession.dirtyState();
      return {
        enabled: dirty.isDirty && Boolean(item.workflow_target_state_id),
        targetStateId: item.workflow_target_state_id ?? null,
      };
    },
  );
  return { submitReadiness };
}

export async function createItemDetailPublicationCase() {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  const readVersionCalls = [];
  const rawSignals = {
    input(id, initial) {
      return createRawReadableHandle(id, initial);
    },
    computedSpec(id, spec) {
      return createRawReadableHandle(id, spec);
    },
    computedCallback(id, callback) {
      return createRawReadableHandle(id, callback());
    },
    outputSpec(id, spec) {
      return createRawReadableHandle(id, { id, spec });
    },
    read(target) {
      return typeof target === "string" ? target : target.id;
    },
    watch() {
      return { free() {} };
    },
    effect() {
      return { free() {} };
    },
    transaction(callback) {
      callback({ set() {}, free() {} });
      return {};
    },
    batch(callback) {
      callback({ set() {}, free() {} });
      return {};
    },
    nuke() {
      return true;
    },
    diagnostics() {
      return {
        subscribe() {
          return { free() {} };
        },
        why(id) {
          return { id, family: "why" };
        },
        health() {
          return null;
        },
        summaryNow() {
          return { profile: "WebDevelopment", active_node_count: 9 };
        },
        historyNow() {
          return {
            history: {
              profile: "WebDevelopment",
              traced_node_count: 4,
              execution_record_count: 4,
              latest_execution_record_id: 21,
              reuse_origin_counts: {},
              nodes: [],
            },
            callbackNodes: [],
          };
        },
        latestObservation() {
          return {
            observation: {
              node: "itemDetail.submitReadiness",
              phase: "Apply",
            },
            callbackNodes: [],
          };
        },
        latestFlow() {
          return {
            flow: {
              profile: "WebDevelopment",
              cause_samples: [],
              event_epochs: [],
              observation: null,
              rollback: null,
              explanation: null,
            },
            callbackNodes: [],
          };
        },
        performanceSummary() {
          return {};
        },
        latestFailure() {
          return null;
        },
        latestRollback() {
          return null;
        },
        latestFrontierExecution() {
          return null;
        },
        latestInvalidationTraceRecords() {
          return [];
        },
        recentHistory() {
          return [
            {
              profile: "WebDevelopment",
              traced_node_count: 3,
              execution_record_count: 3,
              latest_execution_record_id: 20,
              reuse_origin_counts: {},
              nodes: [],
            },
          ];
        },
      };
    },
    history() {
      return {
        replay_for(id) {
          return { id, family: "replay" };
        },
        lineage_for(id) {
          return { id, family: "lineage" };
        },
        free() {},
      };
    },
    specialist() {
      return {
        read_versions(ids) {
          readVersionCalls.push(ids);
          return ids.map((id, index) => ({ id, version: index + 10 }));
        },
        free() {},
      };
    },
    adapters() {
      return {
        export_definitions() {
          return {
            policy: { preset: "WebDevelopment" },
            sources: [
              { id: "serverItemData", initial: null },
              { id: "draftEdits", initial: {} },
              { id: "other", initial: 0 },
            ],
            recipes: [
              {
                id: "effectiveItemData",
                reads: ["serverItemData", "draftEdits"],
                expr: {
                  kind: "mergeObjects",
                  args: [
                    { kind: "read", id: "serverItemData" },
                    { kind: "read", id: "draftEdits" },
                  ],
                },
              },
              {
                id: "dirtyState",
                reads: ["draftEdits"],
                expr: {
                  kind: "object",
                  fields: [["isDirty", { kind: "value", value: false }]],
                },
              },
              {
                id: "submitReadiness",
                reads: ["effectiveItemData", "dirtyState"],
                expr: {
                  kind: "object",
                  fields: [["enabled", { kind: "value", value: false }]],
                },
              },
              {
                id: "itemDetail.effectiveItemData",
                reads: ["effectiveItemData"],
                expr: { kind: "read", id: "effectiveItemData" },
              },
              {
                id: "itemDetail.dirtyState",
                reads: ["dirtyState"],
                expr: { kind: "read", id: "dirtyState" },
              },
              {
                id: "itemDetail.submitReadiness",
                reads: ["submitReadiness"],
                expr: { kind: "read", id: "submitReadiness" },
              },
              {
                id: "unrelated",
                reads: ["other"],
                expr: { kind: "read", id: "other" },
              },
            ],
            sourceFamilies: [],
            recipeFamilies: [],
            unavailableCallbacks: [],
          };
        },
        free() {},
      };
    },
    compatibilityApp() {
      return { family: "app" };
    },
    compatibilityRuntime() {
      return { family: "runtime" };
    },
    free() {},
  };

  const signals = wrapSignals(rawSignals);
  const editSession = createEditSessionController(signals);
  const workflow = createWorkflowController(signals, editSession);
  const graph = signals.graph("itemDetail", {
    outputs: {
      effectiveItemData: editSession.effectiveItemData,
      dirtyState: editSession.dirtyState,
      submitReadiness: workflow.submitReadiness,
    },
  });

  return {
    cleanup,
    graph,
    readVersionCalls,
  };
}
