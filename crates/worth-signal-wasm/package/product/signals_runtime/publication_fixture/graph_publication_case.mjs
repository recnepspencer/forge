import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createRawReadableHandle } from "../runtime_fixture/raw_readable_handle.mjs";

export async function createGraphPublicationCase() {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  const calls = [];
  const whyCalls = [];
  const replayCalls = [];
  const lineageCalls = [];
  const readVersionCalls = [];
  const rawSignals = {
    input(id, initial, options) {
      calls.push(["input", id, initial, options]);
      return createRawReadableHandle(id, initial);
    },
    computedSpec(id, spec) {
      calls.push(["computedSpec", id, spec]);
      return createRawReadableHandle(id, spec);
    },
    computedCallback(id, callback) {
      calls.push(["computedCallback", id, typeof callback]);
      return createRawReadableHandle(id, callback());
    },
    outputSpec(id, spec) {
      calls.push(["outputSpec", id, spec]);
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
          whyCalls.push(id);
          return { id, family: "why" };
        },
        health() {
          return null;
        },
        summaryNow() {
          return { profile: "WebDevelopment", active_node_count: 5 };
        },
        historyNow() {
          return {
            history: {
              profile: "WebDevelopment",
              traced_node_count: 3,
              execution_record_count: 3,
              latest_execution_record_id: 12,
              reuse_origin_counts: {},
              nodes: [],
            },
            callbackNodes: [],
          };
        },
        latestObservation() {
          return {
            observation: {
              node: "panel",
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
        latestInvalidationPlanningEstimate() {
          return null;
        },
        latestInvalidationTraceRecords() {
          return [];
        },
        recentHistory() {
          return [
            {
              profile: "WebDevelopment",
              traced_node_count: 2,
              execution_record_count: 2,
              latest_execution_record_id: 11,
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
          replayCalls.push(id);
          return { id, family: "replay" };
        },
        lineage_for(id) {
          lineageCalls.push(id);
          return { id, family: "lineage" };
        },
        free() {},
      };
    },
    specialist() {
      return {
        read_versions(ids) {
          readVersionCalls.push(ids);
          return ids.map((id, index) => ({ id, version: index + 1 }));
        },
        free() {},
      };
    },
    adapters() {
      return {
        export_definitions() {
          return {
            policy: { preset: "webDevelopment" },
            sources: [
              { id: "count", initial: 1 },
              { id: "other", initial: 99 },
            ],
            recipes: [
              {
                id: "doubled",
                reads: ["count"],
                expr: {
                  kind: "multiply",
                  args: [
                    { kind: "read", id: "count" },
                    { kind: "value", value: 2 },
                  ],
                },
              },
              {
                id: "itemDetail.count",
                reads: ["count"],
                expr: { kind: "read", id: "count" },
              },
              {
                id: "itemDetail.doubled",
                reads: ["doubled"],
                expr: { kind: "read", id: "doubled" },
              },
              {
                id: "panel",
                reads: ["__WorthSignal.outputProjection.panel.1"],
                expr: {
                  kind: "read",
                  id: "__WorthSignal.outputProjection.panel.1",
                },
              },
              {
                id: "unrelated",
                reads: ["other"],
                expr: { kind: "read", id: "other" },
              },
            ],
            sourceFamilies: [],
            recipeFamilies: [],
            unavailableCallbacks: [
              {
                id: "__WorthSignal.outputProjection.panel.1",
                signalKind: "computed",
                reason: "computeCallbackUnavailableForPortableExport",
                currentReads: ["count", "doubled"],
                hostCapabilityReads: [],
                hostCapabilityTransports: [],
              },
              {
                id: "__WorthSignal.outputProjection.unrelated.2",
                signalKind: "computed",
                reason: "computeCallbackUnavailableForPortableExport",
                currentReads: ["other"],
                hostCapabilityReads: [],
                hostCapabilityTransports: [],
              },
            ],
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
  const count = signals.input(1, { debugName: "count" });
  const doubled = signals.computed(() => count() * 2, {
    debugName: "doubled",
  });
  const panel = signals.output(
    () => ({ count: count(), doubled: doubled() }),
    { debugName: "panel" },
  );
  const graph = signals.graph("itemDetail", {
    outputs: {
      count,
      doubled,
      panel,
    },
  });

  return {
    cleanup,
    rawSignals,
    graph,
    count,
    doubled,
    panel,
    calls,
    whyCalls,
    replayCalls,
    lineageCalls,
    readVersionCalls,
  };
}
