import { createRawReadableHandle } from "./raw_readable_handle.mjs";

export function createGraphPublicationRuntime() {
  return {
    input(id, initial) {
      return createRawReadableHandle(id, initial);
    },
    computedSpec(id, spec) {
      return createRawReadableHandle(id, { id, spec });
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
          return { id, family: "replay" };
        },
        lineage_for(id) {
          return { id, family: "lineage" };
        },
        current_branch() {
          return {
            id: 1,
            name: "main",
            parent_branch_id: null,
            head_snapshot_id: null,
          };
        },
      };
    },
    specialist() {
      return {
        evaluate_dirty() {
          return { touchedNodes: 2, nodesEvaluated: 2 };
        },
        graph_summary() {
          return { profile: "WebDevelopment", active_node_count: 5 };
        },
        read_versions(ids) {
          return ids.map((id, index) => ({
            id,
            value_version: index + 1,
            shape_version: index + 10,
          }));
        },
        free() {},
      };
    },
    adapters() {
      return {
        export_definitions() {
          return {
            policy: null,
            sources: [],
            recipes: [],
            sourceFamilies: [],
            recipeFamilies: [],
            unavailableCallbacks: [],
          };
        },
      };
    },
    compatibilityApp() {
      return {};
    },
    compatibilityRuntime() {
      return {};
    },
    free() {},
  };
}
