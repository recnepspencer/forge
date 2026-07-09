import { isDeepStrictEqual } from "node:util";

import { createRawReadableHandle } from "./raw_readable_handle.mjs";

export function createLinkedRuntime() {
  const values = new Map();
  const computedCallbacks = new Map();
  const watchers = new Map();

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

  function notify(id) {
    for (const callback of watchers.get(id) ?? []) {
      callback({ id });
    }
  }

  function recomputeDerivedValues() {
    for (const [id, callback] of computedCallbacks) {
      const previousValue = values.get(id);
      const result = callback();
      const nextValue = result?.__WORTHSignalCallbackCapture
        ? result.value
        : result;
      if (!isDeepStrictEqual(previousValue, nextValue)) {
        values.set(id, cloneValue(nextValue));
        notify(id);
      }
    }
  }

  return {
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
      values.set(id, cloneValue(spec));
      return createRawReadableHandle(id, spec);
    },
    computedCallback(id, callback) {
      computedCallbacks.set(id, callback);
      const result = callback();
      values.set(
        id,
        cloneValue(
          result?.__WORTHSignalCallbackCapture ? result.value : result,
        ),
      );
      return {
        id,
        get() {
          return values.get(id);
        },
        peek() {
          return values.get(id);
        },
        free() {
          computedCallbacks.delete(id);
          watchers.delete(id);
        },
      };
    },
    outputSpec(id, spec) {
      values.set(id, cloneValue(spec));
      return {
        id,
        get() {
          const sourceId = spec?.expr?.id;
          return typeof sourceId === "string"
            ? values.get(sourceId)
            : values.get(id);
        },
        peek() {
          const sourceId = spec?.expr?.id;
          return typeof sourceId === "string"
            ? values.get(sourceId)
            : values.get(id);
        },
        free() {},
      };
    },
    read(target) {
      const id = typeof target === "string" ? target : target.id;
      return values.get(id);
    },
    watch(target, callback) {
      const id = typeof target === "string" ? target : target.id;
      const callbacks = watchers.get(id) ?? new Set();
      callbacks.add(callback);
      watchers.set(id, callbacks);
      return {
        free() {
          callbacks.delete(callback);
          if (callbacks.size === 0) {
            watchers.delete(id);
          }
        },
      };
    },
    effect(target, callback) {
      return this.watch(target, callback);
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
      recomputeDerivedValues();
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
          return { profile: "WebDevelopment", active_node_count: values.size };
        },
        historyNow() {
          return {
            history: {
              profile: "WebDevelopment",
              traced_node_count: values.size,
              execution_record_count: values.size,
              latest_execution_record_id: values.size,
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
          return ids.map((id, index) => ({ id, version: index + 1 }));
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
        free() {},
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
