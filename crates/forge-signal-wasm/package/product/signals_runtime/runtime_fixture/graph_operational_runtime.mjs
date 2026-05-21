import { createRawReadableHandle } from "./raw_readable_handle.mjs";

export function createGraphOperationalRuntime() {
  const values = new Map();
  const callLog = [];

  function cloneValue(value) {
    if (typeof globalThis.structuredClone === "function") {
      try {
        return globalThis.structuredClone(value);
      } catch {
        return value;
      }
    }
    if (Array.isArray(value)) {
      return value.slice();
    }
    if (value && typeof value === "object") {
      return { ...value };
    }
    return value;
  }

  return {
    callLog,
    input(id, initial) {
      values.set(id, cloneValue(initial));
      return {
        id,
        get() {
          return values.get(id);
        },
        peek() {
          return values.get(id);
        },
        free() {},
      };
    },
    computedSpec(id, spec) {
      return createRawReadableHandle(id, { id, spec });
    },
    computedCallback(id, callback) {
      return {
        id,
        get() {
          return callback();
        },
        peek() {
          return callback();
        },
        free() {},
      };
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
      const ops = [];
      callback({
        set(target, value) {
          values.set(target.id, cloneValue(value));
          ops.push(["set", target.id, cloneValue(value)]);
        },
        setWithAspects(target, value, aspects) {
          values.set(target.id, cloneValue(value));
          ops.push(["setWithAspects", target.id, cloneValue(value), aspects]);
        },
        setWithRegions(target, value, changedRegions) {
          values.set(target.id, cloneValue(value));
          ops.push([
            "setWithRegions",
            target.id,
            cloneValue(value),
            changedRegions,
          ]);
        },
        setWithRegionsAndAspects(target, value, changedRegions, aspects) {
          values.set(target.id, cloneValue(value));
          ops.push([
            "setWithRegionsAndAspects",
            target.id,
            cloneValue(value),
            changedRegions,
            aspects,
          ]);
        },
        free() {},
      });
      callLog.push(["transaction", ops]);
      return ops;
    },
    batch(callback) {
      return this.transaction(callback);
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
          return { profile: "WebDevelopment", active_node_count: 3 };
        },
        historyNow() {
          return {
            history: {
              profile: "WebDevelopment",
              traced_node_count: 2,
              execution_record_count: 2,
              latest_execution_record_id: 7,
              reuse_origin_counts: {},
              nodes: [],
            },
            callbackNodes: [],
          };
        },
        latestObservation() {
          return null;
        },
        latestFlow() {
          return null;
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
          return [];
        },
      };
    },
    history() {
      return {
        replay_for(id) {
          return { id, family: "replay", frames: [{ id }] };
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
            sources: [...values.keys()].map((id) => ({ id, initial: null })),
            recipes: [],
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
}
