import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createRawReadableHandle } from "../runtime_fixture/raw_readable_handle.mjs";

test("The Graph Operational Mutation Rejection Precedes Transaction Entry Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    let transactionBegins = 0;
    let inputReads = 0;
    const rawSignals = {
      input(id, initial) {
        return {
          id,
          get() {
            inputReads += 1;
            return initial;
          },
          peek() {
            return initial;
          },
          free() {},
        };
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
        transactionBegins += 1;
        callback({
          set() {},
          setWithAspects() {},
          setWithRegions() {},
          setWithRegionsAndAspects() {},
          free() {},
        });
        return {};
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
            return { profile: "WebDevelopment", active_node_count: 0 };
          },
          historyNow() {
            return {
              history: {
                profile: "WebDevelopment",
                traced_node_count: 0,
                execution_record_count: 0,
                latest_execution_record_id: 0,
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
            return { id, family: "replay", frames: [] };
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
        return { family: "app" };
      },
      compatibilityRuntime() {
        return { family: "runtime" };
      },
      free() {},
    };

    const signals = wrapSignals(rawSignals);
    const graph = signals.graph("itemDetail", (graphBuilder) => {
      const scoped = graphBuilder.scope("editSession");
      const draftEdits = scoped.input({}, { id: "draftEdits" });
      return graphBuilder.expose({
        inputs: { draftEdits },
        outputs: {
          draftEdits,
        },
      });
    });

    let callbackMutationError = null;
    assert.throws(() => {
      try {
        signals.computed(
          () => {
            graph.patchInputs({
              draftEdits: {
                title: "Nope",
              },
            });
            return 1;
          },
          { debugName: "illegalPatch" },
        );
      } catch (error) {
        callbackMutationError = error;
        throw error;
      }
    });
    assert.equal(callbackMutationError?.code, "computeCallbackMutationDenied");
    assert.equal(
      transactionBegins,
      0,
      "graph-native mutation rejection should happen before transaction entry",
    );
    assert.equal(
      inputReads,
      0,
      "graph-native mutation rejection should happen before patch planning reads current input state",
    );
  } finally {
    await cleanup();
  }
});


