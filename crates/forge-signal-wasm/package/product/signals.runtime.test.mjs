import assert from "node:assert/strict";
import { mkdir, mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { stripTypeScriptTypes } from "node:module";
import { tmpdir } from "node:os";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";
import { isDeepStrictEqual } from "node:util";

const productDir = path.dirname(fileURLToPath(import.meta.url));
const packageDir = path.dirname(productDir);
const packageSourceDir = path.join(packageDir, "..", "package-src");

async function loadSignalsModule() {
  const tempDir = await mkdtemp(path.join(tmpdir(), "forge-signal-product-"));
  try {
    const filesToCopy = [
      ["product/signals.ts", "product/signals.js"],
      ["product/callback_frames.ts", "product/callback_frames.js"],
      ["product/controllers.ts", "product/controllers.js"],
      ["product/diagnostics.ts", "product/diagnostics.js"],
      ["product/graph_authoring_support.ts", "product/graph_authoring_support.js"],
      ["product/graph_support.ts", "product/graph_support.js"],
      ["product/graphs.ts", "product/graphs.js"],
      ["product/host_capability_declarations.ts", "product/host_capability_declarations.js"],
      ["product/host_capability_registrations.ts", "product/host_capability_registrations.js"],
      ["product/host_capability_reports.ts", "product/host_capability_reports.js"],
      ["product/host_capabilities.ts", "product/host_capabilities.js"],
      ["product/history.ts", "product/history.js"],
      ["product/handles.ts", "product/handles.js"],
      ["product/linked.ts", "product/linked.js"],
      ["product/public_inputs.ts", "product/public_inputs.js"],
      ["product/scopes.ts", "product/scopes.js"],
      ["product/specialist.ts", "product/specialist.js"],
      ["product/transactions.ts", "product/transactions.js"],
      ["product/symbols.ts", "product/symbols.js"],
    ];

    for (const [sourceRelativePath, outputRelativePath] of filesToCopy) {
      const sourcePath = path.join(packageSourceDir, sourceRelativePath);
      const targetPath = path.join(tempDir, outputRelativePath);
      await mkdir(path.dirname(targetPath), { recursive: true });
      const source = await readFile(sourcePath, "utf8");
      await writeFile(
        targetPath,
        stripTypeScriptTypes(source, { mode: "transform" }),
        "utf8",
      );
    }

    await writeFile(
      path.join(tempDir, "raw_surface.js"),
      "export function createRawSignals() { throw new Error('createRawSignals should not be used in signals product runtime tests'); }\n",
      "utf8",
    );

    const moduleUrl = new URL(`file:///${path.join(tempDir, "product", "signals.js").replace(/\\/g, "/")}`);
    const loaded = await import(moduleUrl.href);
    return { ...loaded, cleanup: () => rm(tempDir, { recursive: true, force: true }) };
  } catch (error) {
    await rm(tempDir, { recursive: true, force: true });
    throw error;
  }
}

function materializeGraphDiagnosticsSurface(surface) {
  return {
    graph: surface.graph,
    contract: surface.contract,
    dependencies: { ...surface.dependencies },
    inputDescriptors: surface.inputDescriptors,
    descriptors: surface.descriptors,
    inputVersions: surface.inputVersions,
    outputVersions: surface.outputVersions,
    inputs: { ...surface.inputs },
    outputs: { ...surface.outputs },
    runtimeGraph: surface.runtimeGraph,
    executionHistory: surface.executionHistory,
    latestFlow: surface.latestFlow,
    latestObservation: surface.latestObservation,
  };
}

function materializeGraphHistorySurface(surface) {
  return {
    graph: surface.graph,
    contract: surface.contract,
    dependencies: { ...surface.dependencies },
    inputDescriptors: surface.inputDescriptors,
    descriptors: surface.descriptors,
    inputs: { ...surface.inputs },
    outputs: { ...surface.outputs },
    executionHistory: surface.executionHistory,
    recentHistory: surface.recentHistory,
  };
}

function createRawReadableHandle(id, value) {
  return {
    id,
    get() {
      return value;
    },
    peek() {
      return value;
    },
    free() {},
  };
}

function createGraphPublicationRuntime() {
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
        health() { return null; },
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
        performanceSummary() { return {}; },
        latestFailure() { return null; },
        latestRollback() { return null; },
        latestFrontierExecution() { return null; },
        latestInvalidationTraceRecords() { return []; },
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
          return ids.map((id, index) => ({ id, value_version: index + 1, shape_version: index + 10 }));
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

function createGraphOperationalRuntime() {
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
          ops.push(["setWithRegions", target.id, cloneValue(value), changedRegions]);
        },
        setWithRegionsAndAspects(target, value, changedRegions, aspects) {
          values.set(target.id, cloneValue(value));
          ops.push(["setWithRegionsAndAspects", target.id, cloneValue(value), changedRegions, aspects]);
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
        health() { return null; },
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
        performanceSummary() { return {}; },
        latestFailure() { return null; },
        latestRollback() { return null; },
        latestFrontierExecution() { return null; },
        latestInvalidationTraceRecords() { return []; },
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
            policy: { preset: "WebDevelopment" },
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

function createGraphExportImportRuntime() {
  const values = new Map();
  const sourceIds = new Set();
  const callbackRecipes = new Map();
  const projectionRecipeReads = new Map();

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

  function recomputeDerivedValues() {
    for (const [id, recipe] of callbackRecipes) {
      const result = recipe.callback();
      const value = result?.__forgeSignalCallbackCapture ? result.value : result;
      if (result?.__forgeSignalCallbackCapture) {
        recipe.reads = [...result.reads];
      }
      values.set(id, cloneValue(value));
    }
    for (const [id, sourceId] of projectionRecipeReads) {
      values.set(id, cloneValue(values.get(sourceId)));
    }
  }

  function exportDefinitions() {
    return {
      policy: { preset: "WebDevelopment" },
      sources: [...sourceIds].map((id) => ({ id, initial: null })),
      recipes: [
        ...[...callbackRecipes.entries()].map(([id, recipe]) => ({
          id,
          reads: recipe.reads,
        })),
        ...[...projectionRecipeReads.entries()].map(([id, sourceId]) => ({
          id,
          reads: [sourceId],
        })),
      ],
      sourceFamilies: [],
      recipeFamilies: [],
      unavailableCallbacks: [],
    };
  }

  function exportRuntimeEnvelope() {
    return {
      values: Object.fromEntries([...values.entries()].map(([id, value]) => [id, cloneValue(value)])),
      definitions: exportDefinitions(),
    };
  }

  function restoreRuntimeEnvelope(envelope) {
    values.clear();
    sourceIds.clear();
    callbackRecipes.clear();
    projectionRecipeReads.clear();
    for (const [id, value] of Object.entries(envelope?.values ?? {})) {
      values.set(id, cloneValue(value));
    }
    for (const source of envelope?.definitions?.sources ?? []) {
      if (typeof source?.id === "string" && source.id.length > 0) {
        sourceIds.add(source.id);
      }
    }
    for (const recipe of envelope?.definitions?.recipes ?? []) {
      if (typeof recipe?.id !== "string" || recipe.id.length === 0) {
        continue;
      }
      if (
        Array.isArray(recipe.reads)
        && recipe.reads.length === 1
        && typeof recipe.reads[0] === "string"
        && recipe.id.includes(".")
      ) {
        projectionRecipeReads.set(recipe.id, recipe.reads[0]);
        continue;
      }
      callbackRecipes.set(recipe.id, {
        callback() {
          return values.get(recipe.id);
        },
        reads: Array.isArray(recipe.reads) ? [...recipe.reads] : [],
      });
    }
  }

  return {
    input(id, initial) {
      sourceIds.add(id);
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
      if (spec?.expr?.kind === "read" && typeof spec.expr.id === "string") {
        projectionRecipeReads.set(id, spec.expr.id);
        values.set(id, cloneValue(values.get(spec.expr.id)));
      }
      return {
        id,
        get() {
          const sourceId = projectionRecipeReads.get(id);
          return sourceId ? values.get(sourceId) : values.get(id);
        },
        peek() {
          const sourceId = projectionRecipeReads.get(id);
          return sourceId ? values.get(sourceId) : values.get(id);
        },
        free() {},
      };
    },
    computedCallback(id, callback) {
      const result = callback();
      callbackRecipes.set(id, {
        callback,
        reads: result?.__forgeSignalCallbackCapture ? [...result.reads] : [],
      });
      values.set(
        id,
        cloneValue(result?.__forgeSignalCallbackCapture ? result.value : result),
      );
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
    outputSpec(id, spec) {
      const sourceId = spec?.expr?.id;
      if (typeof sourceId === "string") {
        projectionRecipeReads.set(id, sourceId);
        values.set(id, cloneValue(values.get(sourceId)));
      }
      return {
        id,
        get() {
          const currentSourceId = projectionRecipeReads.get(id);
          return currentSourceId ? values.get(currentSourceId) : values.get(id);
        },
        peek() {
          const currentSourceId = projectionRecipeReads.get(id);
          return currentSourceId ? values.get(currentSourceId) : values.get(id);
        },
        free() {},
      };
    },
    read(target) {
      const id = typeof target === "string" ? target : target.id;
      return values.get(id);
    },
    watch() {
      return { free() {} };
    },
    effect() {
      return { free() {} };
    },
    transaction(callback) {
      callback({
        set(target, value) {
          values.set(target.id, cloneValue(value));
        },
        setWithAspects(target, value) {
          values.set(target.id, cloneValue(value));
        },
        setWithRegions(target, value) {
          values.set(target.id, cloneValue(value));
        },
        setWithRegionsAndAspects(target, value) {
          values.set(target.id, cloneValue(value));
        },
        free() {},
      });
      recomputeDerivedValues();
      return { committed: true };
    },
    batch(callback) {
      return this.transaction(callback);
    },
    nuke() {
      return true;
    },
    diagnostics() {
      return {
        subscribe() { return { free() {} }; },
        why(id) { return { id, family: "why" }; },
        health() { return null; },
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
        latestObservation() { return null; },
        latestFlow() { return null; },
        performanceSummary() { return {}; },
        latestFailure() { return null; },
        latestRollback() { return null; },
        latestFrontierExecution() { return null; },
        latestInvalidationTraceRecords() { return []; },
        recentHistory() { return []; },
      };
    },
    history() {
      return {
        replay_for(id) { return { id, family: "replay", frames: [{ id }] }; },
        lineage_for(id) { return { id, family: "lineage" }; },
        snapshot() {
          return { snapshot: { meta: { branch_id: 0 } } };
        },
        snapshot_wire() {
          return JSON.stringify({ snapshot: { meta: { branch_id: 0 } } });
        },
        snapshot_portable_wire() {
          return JSON.stringify({ portable: true, snapshot: { meta: { branch_id: 0 } } });
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
          return exportDefinitions();
        },
        export_runtime_envelope() {
          return exportRuntimeEnvelope();
        },
        export_runtime_envelope_wire() {
          return JSON.stringify(exportRuntimeEnvelope());
        },
        export_runtime_envelope_portable_wire() {
          return JSON.stringify({ portable: true, ...exportRuntimeEnvelope() });
        },
        replace_runtime_envelope(envelope) {
          restoreRuntimeEnvelope(envelope);
        },
        replace_runtime_envelope_wire(envelope) {
          restoreRuntimeEnvelope(JSON.parse(envelope));
        },
        replace_runtime_envelope_portable_wire(envelope) {
          restoreRuntimeEnvelope(JSON.parse(envelope));
        },
        runtime_proof_report() {
          return { kind: "proof" };
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

function createLinkedRuntime() {
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
      const nextValue = result?.__forgeSignalCallbackCapture ? result.value : result;
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
      values.set(id, cloneValue(result?.__forgeSignalCallbackCapture ? result.value : result));
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
          return typeof sourceId === "string" ? values.get(sourceId) : values.get(id);
        },
        peek() {
          const sourceId = spec?.expr?.id;
          return typeof sourceId === "string" ? values.get(sourceId) : values.get(id);
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
          ops.push(["setWithRegions", target.id, cloneValue(value), changedRegions]);
        },
        setWithRegionsAndAspects(target, value, changedRegions, aspects) {
          values.set(target.id, cloneValue(value));
          ops.push(["setWithRegionsAndAspects", target.id, cloneValue(value), changedRegions, aspects]);
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
        subscribe() { return { free() {} }; },
        why(id) { return { id, family: "why" }; },
        health() { return null; },
        summaryNow() { return { profile: "WebDevelopment", active_node_count: values.size }; },
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
        latestObservation() { return null; },
        latestFlow() { return null; },
        performanceSummary() { return {}; },
        latestFailure() { return null; },
        latestRollback() { return null; },
        latestFrontierExecution() { return null; },
        latestInvalidationTraceRecords() { return []; },
        recentHistory() { return []; },
      };
    },
    history() {
      return {
        replay_for(id) { return { id, family: "replay" }; },
        lineage_for(id) { return { id, family: "lineage" }; },
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

test("The Linked Writable Derived State Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createLinkedRuntime());
    const shippingOptions = signals.input([
      { id: "ground", label: "Ground" },
      { id: "air", label: "Air" },
    ], { debugName: "shippingOptions" });

    const firstOption = signals.linked(() => shippingOptions()[0], {
      debugName: "firstOption",
    });

    const preservedSelection = signals.linked({
      source: () => shippingOptions(),
      computation: (options, previous) => (
        options.find((option) => option.id === previous?.value?.id) ?? options[0] ?? null
      ),
      debugName: "preservedSelection",
    });

    const selectionController = signals.controller(({ input, linked }) => {
      const options = input([
        { id: "draft", label: "Draft" },
        { id: "review", label: "Review" },
      ]);
      const selected = linked({
        source: () => options(),
        computation: (nextOptions, previous) => (
          nextOptions.find((option) => option.id === previous?.value?.id) ?? nextOptions[0]
        ),
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
        computation: (options, previous) => (
          options.find((option) => option.id === previous?.value?.id) ?? options[0] ?? null
        ),
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
    const linkedRevisionGraph = signals.graph("linkedRevisionSelection", (graph) => {
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
          const preserved = previous && previous.source.revision === source.revision
            ? source.options.find((option) => option.id === previous.value?.id) ?? null
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
    });

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

test("The Portable Lane Explicit Naming Test", async () => {
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
        return createRawReadableHandle(id, { spec });
      },
      computedCallback(id, callback) {
        calls.push(["computedCallback", id, callback()]);
        return createRawReadableHandle(id, id.length);
      },
      outputSpec(id, spec) {
        calls.push(["outputSpec", id, spec]);
        return createRawReadableHandle(id, { spec });
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
    const doubledSpec = { expr: { kind: "value", value: 2 } };
    const labelSpec = { expr: { kind: "value", value: "label" } };

    const count = signals.spec.input("count", 1, { producesAspects: [1] });
    const doubled = signals.spec.computed("doubled", doubledSpec);
    const label = signals.spec.output("label", labelSpec);
    const callbackLabel = signals.spec.outputCallback("callbackLabel", () => "callback-label");
    const generated = signals.spec.computedCallback("generated", () => count() + 1);

    assert.equal(count.id, "count");
    assert.equal(doubled.id, "doubled");
    assert.equal(label.id, "label");
    assert.equal(callbackLabel.id, "callbackLabel");
    assert.equal(generated.id, "generated");
    assert.deepEqual(label(), { spec: labelSpec });
    assert.throws(
      () => signals.input("count", 1),
      /input app authoring does not accept an explicit id; use signals\.spec\.input/,
    );
    assert.throws(
      () => signals.computed("doubled", doubledSpec),
      /computed app authoring does not accept an explicit id; use signals\.spec\.computed/,
    );

    assert.deepEqual(calls[0], ["input", "count", 1, { producesAspects: [1] }]);
    assert.deepEqual(calls[1], ["computedSpec", "doubled", doubledSpec]);
    assert.deepEqual(calls[2], ["outputSpec", "label", labelSpec]);
    assert.equal(calls[3][0], "computedCallback");
    assert.equal(calls[3][1], "__forgeSignal.outputProjection.callbackLabel.1");
    assert.deepEqual(calls[4], [
      "outputSpec",
      "callbackLabel",
      {
        reads: ["__forgeSignal.outputProjection.callbackLabel.1"],
        expr: {
          kind: "read",
          id: "__forgeSignal.outputProjection.callbackLabel.1",
        },
      },
    ]);
    assert.equal(calls[5][0], "computedCallback");
    assert.equal(calls[5][1], "generated");
  } finally {
    await cleanup();
  }
});

test("wrapSignals keeps callback forms and rejects malformed metadata mixes", async () => {
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
        return createRawReadableHandle(id, spec);
      },
      computedCallback(id, callback) {
        calls.push(["computedCallback", id, typeof callback]);
        return createRawReadableHandle(id, id);
      },
      outputSpec(id, spec) {
        calls.push(["outputSpec", id, spec]);
        return createRawReadableHandle(id, spec);
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
    const deferred = signals.spec.outputCallback("panel", () => 1);
    const explicit = signals.spec.outputCallback("panelExplicit", () => 2);
    const namedComputed = signals.spec.computedCallback("named", () => 3);

    assert.equal(deferred.id, "panel");
    assert.equal(explicit.id, "panelExplicit");
    assert.equal(namedComputed.id, "named");

    assert.deepEqual(calls.slice(0, 5), [
      ["computedCallback", "__forgeSignal.outputProjection.panel.1", "function"],
      ["outputSpec", "panel", {
        reads: ["__forgeSignal.outputProjection.panel.1"],
        expr: {
          kind: "read",
          id: "__forgeSignal.outputProjection.panel.1",
        },
      }],
      ["computedCallback", "__forgeSignal.outputProjection.panelExplicit.2", "function"],
      ["outputSpec", "panelExplicit", {
        reads: ["__forgeSignal.outputProjection.panelExplicit.2"],
        expr: {
          kind: "read",
          id: "__forgeSignal.outputProjection.panelExplicit.2",
        },
      }],
      ["computedCallback", "named", "function"],
    ]);

    assert.throws(
      () => signals.input(1, "nope"),
      /input options must be an object when provided/,
    );
    assert.throws(
      () => signals.computed("named", { expr: { kind: "value", value: 1 } }, {}),
      /computed app authoring does not accept an explicit id; use signals\.spec\.computed/,
    );
    assert.throws(
      () => signals.output("label", { expr: { kind: "value", value: 1 } }, { id: "extra" }),
      /output app authoring does not accept an explicit id; use signals\.spec\.output/,
    );
    assert.throws(
      () => signals.output(() => 1, "panel"),
      /output callback options must be an object when provided/,
    );
  } finally {
    await cleanup();
  }
});

test("wrapSignals accepts string-valued metadata-style inputs without misparsing them as id-first authoring", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const calls = [];
    const rawSignals = {
      input(id, initial, options) {
        calls.push(["input", id, initial, options]);
        return createRawReadableHandle(id, initial);
      },
      computedSpec() {
        throw new Error("computedSpec not needed");
      },
      computedCallback() {
        throw new Error("computedCallback not needed");
      },
      outputSpec() {
        throw new Error("outputSpec not needed");
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
    const emptyStringInput = signals.spec.input("emptyStringInput", "");
    const namedStringInput = signals.spec.input("name", "Ada");
    const objectWithOwnIdValue = signals.input({ id: "gear-7", name: "Gear 7" }, { debugName: "draft" });

    assert.equal(emptyStringInput.id, "emptyStringInput");
    assert.equal(emptyStringInput(), "");
    assert.equal(namedStringInput.id, "name");
    assert.equal(namedStringInput(), "Ada");
    assert.notEqual(objectWithOwnIdValue.id, "gear-7");
    assert.deepEqual(objectWithOwnIdValue(), { id: "gear-7", name: "Gear 7" });

    assert.deepEqual(calls, [
      ["input", "emptyStringInput", "", undefined],
      ["input", "name", "Ada", undefined],
      ["input", objectWithOwnIdValue.id, { id: "gear-7", name: "Gear 7" }, {}],
    ]);
  } finally {
    await cleanup();
  }
});

test("wrapSignals rejects raw handles, foreign-runtime handles, and non-input mutations", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const firstCalls = [];
    const secondCalls = [];

    function buildRawSignals(callLog) {
      return {
        input(id, initial, options) {
          callLog.push(["input", id, initial, options]);
          return createRawReadableHandle(id, initial);
        },
        computedSpec(id, spec) {
          callLog.push(["computedSpec", id, spec]);
          return createRawReadableHandle(id, spec);
        },
        computedCallback(id, callback) {
          callLog.push(["computedCallback", id, typeof callback]);
          return createRawReadableHandle(id, id);
        },
        outputSpec(id, spec) {
          callLog.push(["outputSpec", id, spec]);
          return createRawReadableHandle(id, spec);
        },
        read(target) {
          callLog.push(["read", target.id ?? target]);
          return typeof target === "string" ? target : target.id;
        },
        watch(target) {
          callLog.push(["watch", target.id ?? target]);
          return { free() {} };
        },
        effect(target) {
          callLog.push(["effect", target.id ?? target]);
          return { free() {} };
        },
        transaction(callback) {
          const ops = [];
          callback({
            set(target, value) {
              ops.push(["set", target.id, value]);
            },
            setWithAspects(target, value, aspects) {
              ops.push(["setWithAspects", target.id, value, aspects]);
            },
            setWithRegions(target, value, changedRegions) {
              ops.push(["setWithRegions", target.id, value, changedRegions]);
            },
            setWithRegionsAndAspects(target, value, changedRegions, aspects) {
              ops.push(["setWithRegionsAndAspects", target.id, value, changedRegions, aspects]);
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
    }

    const firstSignals = wrapSignals(buildRawSignals(firstCalls));
    const secondSignals = wrapSignals(buildRawSignals(secondCalls));

    const firstInput = firstSignals.input(1, { debugName: "count" });
    const secondInput = secondSignals.input(2, { debugName: "other" });
    const computed = firstSignals.computed({ expr: { kind: "value", value: 4 } }, { debugName: "double" });
    const rawHandle = createRawReadableHandle("raw", 9);

    assert.throws(
      () => firstSignals.read(rawHandle),
      /signals\.read expects a string id or a product signal handle created by this package/,
    );
    assert.throws(
      () => firstSignals.watch(secondInput, () => {}),
      /signals\.watch cannot use signal `other` from a different Signals runtime/,
    );
    assert.throws(
      () => firstSignals.effect(secondInput, () => {}),
      /signals\.effect cannot use signal `other` from a different Signals runtime/,
    );

    assert.throws(
      () => firstSignals.transaction((tx) => tx.set(computed, 4)),
      /transaction\.set expects an input handle, but received a computed handle for `double`/,
    );
    assert.throws(
      () => firstSignals.transaction((tx) => tx.set(secondInput, 4)),
      /transaction\.set cannot use signal `other` from a different Signals runtime/,
    );

    const commit = firstSignals.transaction((tx) => tx.set(firstInput, 7));
    assert.deepEqual(commit, [["set", firstInput.id, 7]]);
  } finally {
    await cleanup();
  }
});

test("The Graph Publication Output Synthesis Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
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
          health() { return null; },
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
          performanceSummary() { return {}; },
          latestFailure() { return null; },
          latestRollback() { return null; },
          latestFrontierExecution() { return null; },
          latestInvalidationTraceRecords() { return []; },
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
              policy: { preset: "WebDevelopment" },
              sources: [
                { id: "count", initial: 1 },
                { id: "other", initial: 99 },
              ],
              recipes: [
                {
                  id: "doubled",
                  reads: ["count"],
                  expr: { kind: "multiply", args: [{ kind: "read", id: "count" }, { kind: "value", value: 2 }] },
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
                  reads: ["__forgeSignal.outputProjection.panel.1"],
                  expr: { kind: "read", id: "__forgeSignal.outputProjection.panel.1" },
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
                  id: "__forgeSignal.outputProjection.panel.1",
                  signalKind: "computed",
                  reason: "computeCallbackUnavailableForPortableExport",
                  currentReads: ["count", "doubled"],
                  hostCapabilityReads: [],
                  hostCapabilityTransports: [],
                },
                {
                  id: "__forgeSignal.outputProjection.unrelated.2",
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
    const doubled = signals.computed(() => count() * 2, { debugName: "doubled" });
    const panel = signals.output(() => ({ count: count(), doubled: doubled() }), { debugName: "panel" });

    const graph = signals.graph("itemDetail", {
      outputs: {
        count,
        doubled,
        panel,
      },
    });

    assert.equal(graph.id, "itemDetail");
    assert.equal(graph.outputs.count.id, "itemDetail.count");
    assert.equal(graph.outputs.doubled.id, "itemDetail.doubled");
    assert.equal(graph.outputs.panel.id, "itemDetail.panel");
    const graphSnapshot = graph.read();
    assert.equal(Object.getPrototypeOf(graphSnapshot), null);
    assert.deepEqual({ ...graphSnapshot }, {
      count: {
        id: "itemDetail.count",
        spec: {
          reads: [count.id],
          expr: {
            kind: "read",
            id: count.id,
          },
        },
      },
      doubled: {
        id: "itemDetail.doubled",
        spec: {
          reads: [doubled.id],
          expr: {
            kind: "read",
            id: doubled.id,
          },
        },
      },
      panel: {
        id: "itemDetail.panel",
        spec: {
          reads: [panel.id],
          expr: {
            kind: "read",
            id: panel.id,
          },
        },
      },
    });
    assert.deepEqual(graph.output("count")(), {
      id: "itemDetail.count",
      spec: {
        reads: [count.id],
        expr: {
          kind: "read",
          id: count.id,
        },
      },
    });
    assert.deepEqual(graph.why("count"), { id: "itemDetail.count", family: "why" });
    assert.deepEqual(graph.replayFor("doubled"), { id: "itemDetail.doubled", family: "replay" });
    assert.deepEqual(graph.lineageFor("panel"), { id: "itemDetail.panel", family: "lineage" });
    assert.deepEqual(graph.readVersions(), [
      { id: "itemDetail.count", version: 1 },
      { id: "itemDetail.doubled", version: 2 },
      { id: "itemDetail.panel", version: 3 },
    ]);
    const diagnosticsSurface = graph.inspectDiagnostics();
    assert.equal(Object.getPrototypeOf(diagnosticsSurface.inputs), null);
    assert.equal(Object.getPrototypeOf(diagnosticsSurface.outputs), null);
    assert.deepEqual(diagnosticsSurface.contract, graph.contract());
    assert.deepEqual(diagnosticsSurface.inputVersions, []);
    assert.deepEqual(
      diagnosticsSurface.dependenciesForOutput("panel"),
      {
        graphId: "itemDetail",
        outputName: "panel",
        publishedId: "itemDetail.panel",
        sourceId: panel.id,
        publicInputNames: [],
        publicInputSourceIds: [],
        transitiveSignalIds: ["itemDetail.panel", panel.id],
      },
    );
    assert.deepEqual(
      diagnosticsSurface.contractSummary(),
      {
        graph: graph.summary(),
        contract: graph.contract(),
        inputCount: 0,
        outputCount: 3,
        inputNames: [],
        outputNames: ["count", "doubled", "panel"],
        dependencies: diagnosticsSurface.dependencies,
      },
    );
    assert.deepEqual(diagnosticsSurface.outputs.count, {
      descriptor: graph.descriptors()[0],
      version: { id: "itemDetail.count", version: 1 },
      why: { id: "itemDetail.count", family: "why" },
    });
    assert.deepEqual(diagnosticsSurface.outputs.panel, {
      descriptor: graph.descriptors()[2],
      version: { id: "itemDetail.panel", version: 3 },
      why: { id: "itemDetail.panel", family: "why" },
    });
    assert.deepEqual(diagnosticsSurface.runtimeGraph, {
      profile: "WebDevelopment",
      active_node_count: 5,
    });
    assert.deepEqual(diagnosticsSurface.executionHistory, {
      history: {
        profile: "WebDevelopment",
        traced_node_count: 3,
        execution_record_count: 3,
        latest_execution_record_id: 12,
        reuse_origin_counts: {},
        nodes: [],
      },
      callbackNodes: [],
    });
    assert.deepEqual(diagnosticsSurface.latestObservation, {
      observation: {
        node: "panel",
        phase: "Apply",
      },
      callbackNodes: [],
    });
    const historySurface = graph.inspectHistory();
    assert.equal(Object.getPrototypeOf(historySurface.inputs), null);
    assert.equal(Object.getPrototypeOf(historySurface.outputs), null);
    assert.deepEqual(historySurface.contract, graph.contract());
    assert.deepEqual({ ...historySurface.inputs }, {});
    assert.deepEqual(
      historySurface.dependenciesForOutput("panel"),
      diagnosticsSurface.dependenciesForOutput("panel"),
    );
    assert.deepEqual(historySurface.contractSummary(), diagnosticsSurface.contractSummary());
    assert.deepEqual(historySurface.outputs.doubled, {
      descriptor: graph.descriptors()[1],
      replay: { id: "itemDetail.doubled", family: "replay" },
      lineage: { id: "itemDetail.doubled", family: "lineage" },
    });
    assert.deepEqual(historySurface.recentHistory, [
      {
        profile: "WebDevelopment",
        traced_node_count: 2,
        execution_record_count: 2,
        latest_execution_record_id: 11,
        reuse_origin_counts: {},
        nodes: [],
      },
    ]);
    assert.deepEqual(graph.summary(), {
      id: "itemDetail",
      inputCount: 0,
      inputNames: [],
      inputSourceIds: [],
      outputCount: 3,
      outputNames: ["count", "doubled", "panel"],
      publishedOutputIds: ["itemDetail.count", "itemDetail.doubled", "itemDetail.panel"],
      sourceIds: [count.id, doubled.id, panel.id],
      synthesizedOutputCount: 3,
    });
    const compatibilityDefinition = graph.exportCompatibilityDefinition();
    assert.equal(Object.getPrototypeOf(compatibilityDefinition.outputs), null);
    assert.deepEqual({
      ...compatibilityDefinition,
      contract: {
        ...compatibilityDefinition.contract,
        inputs: { ...compatibilityDefinition.contract.inputs },
        outputs: { ...compatibilityDefinition.contract.outputs },
      },
      inputs: { ...compatibilityDefinition.inputs },
      outputs: { ...compatibilityDefinition.outputs },
    }, {
      id: "itemDetail",
      contract: {
        graph: graph.summary(),
        inputs: {},
        outputs: {
          count: "itemDetail.count",
          doubled: "itemDetail.doubled",
          panel: "itemDetail.panel",
        },
        inputDescriptors: [],
        descriptors: graph.descriptors(),
      },
      inputs: {},
      outputs: {
        count: "itemDetail.count",
        doubled: "itemDetail.doubled",
        panel: "itemDetail.panel",
      },
      inputSourceIds: [],
      publishedOutputIds: ["itemDetail.count", "itemDetail.doubled", "itemDetail.panel"],
      sourceIds: [count.id, doubled.id, panel.id],
      inputDescriptors: [],
      descriptors: graph.descriptors(),
      definitions: {
        policy: { preset: "WebDevelopment" },
        sources: [{ id: "count", initial: 1 }],
        recipes: [
          {
            id: "doubled",
            reads: ["count"],
            expr: { kind: "multiply", args: [{ kind: "read", id: "count" }, { kind: "value", value: 2 }] },
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
        ],
        sourceFamilies: [],
        recipeFamilies: [],
        unavailableCallbacks: [],
      },
    });
    rawSignals.adapters = () => ({
      export_definitions() {
        return {
          policy: { preset: "WebDevelopment" },
          sources: [
            { id: "count", initial: 1 },
          ],
          recipes: [
            {
              id: "doubled",
              reads: ["count"],
              expr: { kind: "multiply", args: [{ kind: "read", id: "count" }, { kind: "value", value: 2 }] },
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
              reads: ["__forgeSignal.outputProjection.panel.1"],
              expr: { kind: "read", id: "__forgeSignal.outputProjection.panel.1" },
            },
          ],
          sourceFamilies: [],
          recipeFamilies: [],
          unavailableCallbacks: [
            {
              id: "__forgeSignal.outputProjection.panel.1",
              signalKind: "computed",
              reason: "computeCallbackUnavailableForPortableExport",
              currentReads: ["count"],
              hostCapabilityReads: [],
              hostCapabilityTransports: [],
            },
          ],
        };
      },
      free() {},
    });
    const refreshedCompatibilityDefinition = graph.exportCompatibilityDefinition();
    assert.deepEqual(
      refreshedCompatibilityDefinition.definitions.unavailableCallbacks[0]?.currentReads,
      undefined,
    );
    assert.equal(typeof graph.diagnostics().performanceSummary, "function");
    assert.equal(typeof graph.history, "function");
    assert.equal(typeof graph.specialist, "function");
    assert.equal(typeof graph.adapters, "function");
    assert.equal(typeof graph.compatibilityApp, "function");
    assert.equal(typeof graph.compatibilityRuntime, "function");
    assert.deepEqual(graph.descriptors(), [
      {
        outputName: "count",
        sourceId: count.id,
        sourceKind: "input",
        publishedId: "itemDetail.count",
        publicationKind: "synthesizedOutput",
      },
      {
        outputName: "doubled",
        sourceId: doubled.id,
        sourceKind: "computed",
        publishedId: "itemDetail.doubled",
        publicationKind: "synthesizedOutput",
      },
      {
        outputName: "panel",
        sourceId: panel.id,
        sourceKind: "output",
        publishedId: "itemDetail.panel",
        publicationKind: "synthesizedOutput",
      },
    ]);
    assert.deepEqual(readVersionCalls, [
      ["itemDetail.count", "itemDetail.doubled", "itemDetail.panel"],
      [],
      ["itemDetail.count", "itemDetail.doubled", "itemDetail.panel"],
    ]);
    assert.deepEqual(whyCalls, [
      "itemDetail.count",
      "itemDetail.count",
      "itemDetail.doubled",
      "itemDetail.panel",
    ]);
    assert.deepEqual(replayCalls, [
      "itemDetail.doubled",
      "itemDetail.count",
      "itemDetail.doubled",
      "itemDetail.panel",
    ]);
    assert.deepEqual(lineageCalls, [
      "itemDetail.panel",
      "itemDetail.count",
      "itemDetail.doubled",
      "itemDetail.panel",
    ]);
    const panelProjectionId = calls[2][1];
    assert.deepEqual(calls.slice(0, 7), [
      ["input", count.id, 1, {}],
      ["computedCallback", doubled.id, "function"],
      ["computedCallback", panelProjectionId, "function"],
      ["outputSpec", panel.id, {
        reads: [panelProjectionId],
        expr: {
          kind: "read",
          id: panelProjectionId,
        },
      }],
      ["outputSpec", "itemDetail.count", {
        reads: [count.id],
        expr: {
          kind: "read",
          id: count.id,
        },
      }],
      ["outputSpec", "itemDetail.doubled", {
        reads: [doubled.id],
        expr: {
          kind: "read",
          id: doubled.id,
        },
      }],
      ["outputSpec", "itemDetail.panel", {
        reads: [panel.id],
        expr: {
          kind: "read",
          id: panel.id,
        },
      }],
    ]);
  } finally {
    await cleanup();
  }
});

test("The Opaque Authoring Equivalence Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    function defineCanonicalGraph(signals) {
      const count = signals.input(1, { debugName: "count" });
      const doubled = signals.computed(() => count() * 2, { debugName: "doubled" });
      const panel = signals.output(() => ({
        count: count(),
        doubled: doubled(),
      }), { debugName: "panel" });

      return signals.graph("counter", {
        inputs: { count },
        outputs: { doubled, panel },
      });
    }

    function defineCompatibilityGraph(signals) {
      const count = signals.spec.input("count", 1);
      const doubled = signals.spec.computedCallback("doubled", () => count() * 2);
      const panel = signals.spec.outputCallback("panel", () => ({ count: count(), doubled: doubled() }));

      return signals.graph("counter", {
        inputs: { count },
        outputs: { doubled, panel },
      });
    }

    const canonicalGraph = defineCanonicalGraph(wrapSignals(createGraphPublicationRuntime()));
    const compatibilityGraph = defineCompatibilityGraph(wrapSignals(createGraphPublicationRuntime()));

    assert.equal(canonicalGraph.summary().id, compatibilityGraph.summary().id);
    assert.deepEqual(canonicalGraph.summary().inputNames, compatibilityGraph.summary().inputNames);
    assert.deepEqual(canonicalGraph.summary().outputNames, compatibilityGraph.summary().outputNames);
    assert.equal(canonicalGraph.contract().inputs.count, canonicalGraph.inputDescriptors()[0].sourceId);
    assert.equal(canonicalGraph.contract().outputs.panel, canonicalGraph.descriptors()[1].publishedId);
    assert.deepEqual(
      canonicalGraph.inputDescriptors().map(({ inputName, sourceKind, authority }) => ({
        inputName,
        sourceKind,
        authority,
      })),
      compatibilityGraph.inputDescriptors().map(({ inputName, sourceKind, authority }) => ({
        inputName,
        sourceKind,
        authority,
      })),
    );
    assert.deepEqual(
      canonicalGraph.descriptors().map(({ outputName, sourceKind, publicationKind }) => ({
        outputName,
        sourceKind,
        publicationKind,
      })),
      compatibilityGraph.descriptors().map(({ outputName, sourceKind, publicationKind }) => ({
        outputName,
        sourceKind,
        publicationKind,
      })),
    );
    assert.deepEqual(canonicalGraph.readInputs(), compatibilityGraph.readInputs());
    assert.deepEqual(
      Object.keys(canonicalGraph.read()),
      Object.keys(compatibilityGraph.read()),
    );
    const canonicalDiagnostics = materializeGraphDiagnosticsSurface(canonicalGraph.inspectDiagnostics());
    const compatibilityDiagnostics = materializeGraphDiagnosticsSurface(compatibilityGraph.inspectDiagnostics());
    assert.deepEqual(canonicalDiagnostics.graph.id, compatibilityDiagnostics.graph.id);
    assert.deepEqual(Object.keys(canonicalDiagnostics.inputs), Object.keys(compatibilityDiagnostics.inputs));
    assert.deepEqual(Object.keys(canonicalDiagnostics.outputs), Object.keys(compatibilityDiagnostics.outputs));
    assert.deepEqual(
      Object.keys(canonicalDiagnostics.dependencies),
      Object.keys(compatibilityDiagnostics.dependencies),
    );
    const canonicalHistory = materializeGraphHistorySurface(canonicalGraph.inspectHistory());
    const compatibilityHistory = materializeGraphHistorySurface(compatibilityGraph.inspectHistory());
    assert.equal(canonicalHistory.contract.graph.id, compatibilityHistory.contract.graph.id);
    assert.deepEqual(
      Object.keys(canonicalHistory.contract.inputs),
      Object.keys(compatibilityHistory.contract.inputs),
    );
    assert.deepEqual(
      Object.keys(canonicalHistory.contract.outputs),
      Object.keys(compatibilityHistory.contract.outputs),
    );
    assert.deepEqual(
      Object.keys(canonicalGraph.exportCompatibilityDefinition()),
      Object.keys(compatibilityGraph.exportCompatibilityDefinition()),
    );
    assert.deepEqual(
      Object.keys(canonicalGraph.exportCompatibilityDefinition().outputs),
      Object.keys(compatibilityGraph.exportCompatibilityDefinition().outputs),
    );
  } finally {
    await cleanup();
  }
});

test("The Debug Name Is Not Identity Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphPublicationRuntime());
    const count = signals.input(1, { debugName: "shared" });
    const countMirror = signals.computed(() => count(), { debugName: "shared" });
    const graph = signals.graph("counter", {
      inputs: {
        count,
      },
      outputs: {
        countValue: countMirror,
      },
    });

    assert.equal(count.debugName, "shared");
    assert.equal(countMirror.debugName, "shared");
    assert.notEqual(count.id, countMirror.id);
    assert.notEqual(count.id, "shared");
    assert.notEqual(countMirror.id, "shared");
    assert.equal(graph.output("countValue").id, "counter.countValue");
    assert.equal(graph.contract().outputs.countValue, "counter.countValue");
    assert.equal(
      graph.exportCompatibilityDefinition().contract.outputs.countValue,
      "counter.countValue",
    );
  } finally {
    await cleanup();
  }
});

test("The Debug Name Is Not Addressability Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphPublicationRuntime());
    const count = signals.input(1, { debugName: "count" });
    const doubled = signals.computed(() => count() * 2, { debugName: "count" });

    assert.equal(count.debugName, "count");
    assert.equal(doubled.debugName, "count");
    assert.notEqual(count.id, "count");
    assert.notEqual(doubled.id, "count");
    assert.notEqual(count.id, doubled.id);
    assert.notEqual(signals.read("count"), count());
  } finally {
    await cleanup();
  }
});

test("The Same-Runtime Controller Ownership Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    function buildRawSignals() {
      return {
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
          return createRawReadableHandle(id, spec);
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
          return {};
        },
        history() {
          return {};
        },
        specialist() {
          return {};
        },
        adapters() {
          return {};
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

    const firstSignals = wrapSignals(buildRawSignals());
    const secondSignals = wrapSignals(buildRawSignals());
    const count = firstSignals.input(1, { debugName: "count" });
    const other = secondSignals.input(2, { debugName: "other" });

    assert.throws(
      () => firstSignals.graph("", { outputs: { count } }),
      /signals\.graph requires a non-empty string graph id/,
    );
    assert.throws(
      () => firstSignals.graph("itemDetail"),
      /signals\.graph requires a graph definition object/,
    );
    assert.throws(
      () => firstSignals.graph("itemDetail", { outputs: {} }),
      /signals\.graph requires at least one published output/,
    );
    assert.throws(
      () => firstSignals.graph("itemDetail", { outputs: { count: "count" } }),
      /signals\.graph output `itemDetail\.count` expects a product signal handle created by this package/,
    );
    assert.throws(
      () => firstSignals.graph("itemDetail", { outputs: { other } }),
      /signals\.graph output `itemDetail\.other` cannot use signal `other` from a different Signals runtime/,
    );

    const graph = firstSignals.graph("itemDetail", { outputs: { count } });
    assert.throws(
      () => graph.output("missing"),
      /signals\.graph output `itemDetail\.missing` is not part of the published graph/,
    );
    assert.throws(
      () => graph.why("missing"),
      /signals\.graph output `itemDetail\.missing` is not part of the published graph/,
    );
  } finally {
    await cleanup();
  }
});

test("The Composition Diagnostics And History Parity Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
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
          health() { return null; },
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
          performanceSummary() { return {}; },
          latestFailure() { return null; },
          latestRollback() { return null; },
          latestFrontierExecution() { return null; },
          latestInvalidationTraceRecords() { return []; },
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
                  expr: { kind: "mergeObjects", args: [{ kind: "read", id: "serverItemData" }, { kind: "read", id: "draftEdits" }] },
                },
                {
                  id: "dirtyState",
                  reads: ["draftEdits"],
                  expr: { kind: "object", fields: [["isDirty", { kind: "value", value: false }]] },
                },
                {
                  id: "submitReadiness",
                  reads: ["effectiveItemData", "dirtyState"],
                  expr: { kind: "object", fields: [["enabled", { kind: "value", value: false }]] },
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

    function createEditSessionController(namespace) {
      const serverItemData = namespace.spec.input("serverItemData", null);
      const draftEdits = namespace.spec.input("draftEdits", {});

      const effectiveItemData = namespace.spec.computedCallback("effectiveItemData", () => ({
        ...(serverItemData() ?? {}),
        ...(draftEdits() ?? {}),
      }));

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
      const submitReadiness = namespace.spec.computedCallback("submitReadiness", () => {
        const item = editSession.effectiveItemData();
        const dirty = editSession.dirtyState();

        return {
          enabled: dirty.isDirty && Boolean(item.workflow_target_state_id),
          targetStateId: item.workflow_target_state_id ?? null,
        };
      });

      return {
        submitReadiness,
      };
    }

    const editSession = createEditSessionController(signals);
    const workflow = createWorkflowController(signals, editSession);

    const graph = signals.graph("itemDetail", {
      outputs: {
        effectiveItemData: editSession.effectiveItemData,
        dirtyState: editSession.dirtyState,
        submitReadiness: workflow.submitReadiness,
      },
    });

    assert.equal(graph.output("effectiveItemData").id, "itemDetail.effectiveItemData");
    assert.equal(graph.output("dirtyState").id, "itemDetail.dirtyState");
    assert.equal(graph.output("submitReadiness").id, "itemDetail.submitReadiness");
    assert.deepEqual(graph.summary(), {
      id: "itemDetail",
      inputCount: 0,
      inputNames: [],
      inputSourceIds: [],
      outputCount: 3,
      outputNames: ["effectiveItemData", "dirtyState", "submitReadiness"],
      publishedOutputIds: [
        "itemDetail.effectiveItemData",
        "itemDetail.dirtyState",
        "itemDetail.submitReadiness",
      ],
      sourceIds: ["effectiveItemData", "dirtyState", "submitReadiness"],
      synthesizedOutputCount: 3,
    });
    assert.deepEqual(graph.descriptors(), [
      {
        outputName: "effectiveItemData",
        sourceId: "effectiveItemData",
        sourceKind: "computed",
        publishedId: "itemDetail.effectiveItemData",
        publicationKind: "synthesizedOutput",
      },
      {
        outputName: "dirtyState",
        sourceId: "dirtyState",
        sourceKind: "computed",
        publishedId: "itemDetail.dirtyState",
        publicationKind: "synthesizedOutput",
      },
      {
        outputName: "submitReadiness",
        sourceId: "submitReadiness",
        sourceKind: "computed",
        publishedId: "itemDetail.submitReadiness",
        publicationKind: "synthesizedOutput",
      },
    ]);
    assert.deepEqual(graph.readVersions(), [
      { id: "itemDetail.effectiveItemData", version: 10 },
      { id: "itemDetail.dirtyState", version: 11 },
      { id: "itemDetail.submitReadiness", version: 12 },
    ]);
    assert.deepEqual(readVersionCalls, [[
      "itemDetail.effectiveItemData",
      "itemDetail.dirtyState",
      "itemDetail.submitReadiness",
    ]]);
    const graphDiagnosticsSurface = graph.inspectDiagnostics();
    assert.equal(Object.getPrototypeOf(graphDiagnosticsSurface.inputs), null);
    assert.equal(Object.getPrototypeOf(graphDiagnosticsSurface.outputs), null);
    assert.deepEqual(graphDiagnosticsSurface.contract, graph.contract());
    assert.deepEqual({ ...graphDiagnosticsSurface.inputs }, {});
    assert.deepEqual(graphDiagnosticsSurface.inputVersions, []);
    assert.deepEqual(graphDiagnosticsSurface.outputs.submitReadiness, {
      descriptor: graph.descriptors()[2],
      version: { id: "itemDetail.submitReadiness", version: 12 },
      why: { id: "itemDetail.submitReadiness", family: "why" },
    });
    assert.deepEqual(graphDiagnosticsSurface.runtimeGraph, {
      profile: "WebDevelopment",
      active_node_count: 9,
    });
    const graphHistorySurface = graph.inspectHistory();
    assert.equal(Object.getPrototypeOf(graphHistorySurface.inputs), null);
    assert.equal(Object.getPrototypeOf(graphHistorySurface.outputs), null);
    assert.deepEqual(graphHistorySurface.contract, graph.contract());
    assert.deepEqual({ ...graphHistorySurface.inputs }, {});
    assert.deepEqual(graphHistorySurface.outputs.effectiveItemData, {
      descriptor: graph.descriptors()[0],
      replay: { id: "itemDetail.effectiveItemData", family: "replay" },
      lineage: { id: "itemDetail.effectiveItemData", family: "lineage" },
    });
    assert.deepEqual(graphHistorySurface.recentHistory, [
      {
        profile: "WebDevelopment",
        traced_node_count: 3,
        execution_record_count: 3,
        latest_execution_record_id: 20,
        reuse_origin_counts: {},
        nodes: [],
      },
    ]);
    assert.deepEqual(readVersionCalls, [
      [
        "itemDetail.effectiveItemData",
        "itemDetail.dirtyState",
        "itemDetail.submitReadiness",
      ],
      [],
      [
        "itemDetail.effectiveItemData",
        "itemDetail.dirtyState",
        "itemDetail.submitReadiness",
      ],
    ]);
    const graphCompatibilityDefinition = graph.exportCompatibilityDefinition();
    assert.equal(Object.getPrototypeOf(graphCompatibilityDefinition.outputs), null);
    assert.deepEqual({
      ...graphCompatibilityDefinition,
      contract: {
        ...graphCompatibilityDefinition.contract,
        inputs: { ...graphCompatibilityDefinition.contract.inputs },
        outputs: { ...graphCompatibilityDefinition.contract.outputs },
      },
      inputs: { ...graphCompatibilityDefinition.inputs },
      outputs: { ...graphCompatibilityDefinition.outputs },
    }, {
      id: "itemDetail",
      contract: {
        graph: graph.summary(),
        inputs: {},
        outputs: {
          effectiveItemData: "itemDetail.effectiveItemData",
          dirtyState: "itemDetail.dirtyState",
          submitReadiness: "itemDetail.submitReadiness",
        },
        inputDescriptors: [],
        descriptors: graph.descriptors(),
      },
      inputs: {},
      outputs: {
        effectiveItemData: "itemDetail.effectiveItemData",
        dirtyState: "itemDetail.dirtyState",
        submitReadiness: "itemDetail.submitReadiness",
      },
      inputSourceIds: [],
      publishedOutputIds: [
        "itemDetail.effectiveItemData",
        "itemDetail.dirtyState",
        "itemDetail.submitReadiness",
      ],
      sourceIds: ["effectiveItemData", "dirtyState", "submitReadiness"],
      inputDescriptors: [],
      descriptors: graph.descriptors(),
      definitions: {
        policy: { preset: "WebDevelopment" },
        sources: [
          { id: "serverItemData", initial: null },
          { id: "draftEdits", initial: {} },
        ],
        recipes: [
          {
            id: "effectiveItemData",
            reads: ["serverItemData", "draftEdits"],
            expr: { kind: "mergeObjects", args: [{ kind: "read", id: "serverItemData" }, { kind: "read", id: "draftEdits" }] },
          },
          {
            id: "dirtyState",
            reads: ["draftEdits"],
            expr: { kind: "object", fields: [["isDirty", { kind: "value", value: false }]] },
          },
          {
            id: "submitReadiness",
            reads: ["effectiveItemData", "dirtyState"],
            expr: { kind: "object", fields: [["enabled", { kind: "value", value: false }]] },
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
        ],
        sourceFamilies: [],
        recipeFamilies: [],
        unavailableCallbacks: [],
      },
    });
  } finally {
    await cleanup();
  }
});

test("The Controller Composition And Flat Runtime Equivalence Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    function buildRawSignals() {
      return {
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
            health() { return null; },
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
            performanceSummary() { return {}; },
            latestFailure() { return null; },
            latestRollback() { return null; },
            latestFrontierExecution() { return null; },
            latestInvalidationTraceRecords() { return []; },
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
              return { frames: [{ node: id }], family: "replay" };
            },
            lineage_for(id) {
              return { events: [{ node: id }], family: "lineage" };
            },
            free() {},
          };
        },
        specialist() {
          return {
            read_versions(ids) {
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
                ],
                recipes: [
                  {
                    id: "effectiveItemData",
                    reads: ["serverItemData", "draftEdits"],
                    expr: { kind: "mergeObjects", args: [{ kind: "read", id: "serverItemData" }, { kind: "read", id: "draftEdits" }] },
                  },
                  {
                    id: "dirtyState",
                    reads: ["draftEdits"],
                    expr: { kind: "object", fields: [["isDirty", { kind: "value", value: false }]] },
                  },
                  {
                    id: "submitReadiness",
                    reads: ["effectiveItemData", "dirtyState"],
                    expr: { kind: "object", fields: [["enabled", { kind: "value", value: false }]] },
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
    }

    function defineFlatGraph(signals) {
      const serverItemData = signals.spec.input("serverItemData", null);
      const draftEdits = signals.spec.input("draftEdits", {});
      const effectiveItemData = signals.spec.computedCallback("effectiveItemData", () => ({
        ...(serverItemData() ?? {}),
        ...(draftEdits() ?? {}),
      }));
      const dirtyState = signals.spec.computedCallback("dirtyState", () => ({
        isDirty: Object.keys(draftEdits()).length > 0,
      }));
      const submitReadiness = signals.spec.computedCallback("submitReadiness", () => {
        const item = effectiveItemData();
        const dirty = dirtyState();
        return {
          enabled: dirty.isDirty && Boolean(item.workflow_target_state_id),
          targetStateId: item.workflow_target_state_id ?? null,
        };
      });
      return signals.graph("itemDetail", {
        outputs: {
          effectiveItemData,
          dirtyState,
          submitReadiness,
        },
      });
    }

    function createEditSessionController(namespace) {
      const serverItemData = namespace.spec.input("serverItemData", null);
      const draftEdits = namespace.spec.input("draftEdits", {});
      const effectiveItemData = namespace.spec.computedCallback("effectiveItemData", () => ({
        ...(serverItemData() ?? {}),
        ...(draftEdits() ?? {}),
      }));
      const dirtyState = namespace.spec.computedCallback("dirtyState", () => ({
        isDirty: Object.keys(draftEdits()).length > 0,
      }));
      return { serverItemData, draftEdits, effectiveItemData, dirtyState };
    }

    function createWorkflowController(namespace, editSession) {
      const submitReadiness = namespace.spec.computedCallback("submitReadiness", () => {
        const item = editSession.effectiveItemData();
        const dirty = editSession.dirtyState();
        return {
          enabled: dirty.isDirty && Boolean(item.workflow_target_state_id),
          targetStateId: item.workflow_target_state_id ?? null,
        };
      });
      return { submitReadiness };
    }

    function defineControllerGraph(signals) {
      const editSession = createEditSessionController(signals);
      const workflow = createWorkflowController(signals, editSession);
      return signals.graph("itemDetail", {
        outputs: {
          effectiveItemData: editSession.effectiveItemData,
          dirtyState: editSession.dirtyState,
          submitReadiness: workflow.submitReadiness,
        },
      });
    }

    const flatSignals = wrapSignals(buildRawSignals());
    const controllerSignals = wrapSignals(buildRawSignals());
    const flatGraph = defineFlatGraph(flatSignals);
    const controllerGraph = defineControllerGraph(controllerSignals);

    assert.deepEqual(controllerGraph.summary(), flatGraph.summary());
    assert.deepEqual(controllerGraph.contract(), flatGraph.contract());
    assert.deepEqual(controllerGraph.descriptors(), flatGraph.descriptors());
    assert.deepEqual(controllerGraph.read(), flatGraph.read());
    assert.deepEqual(
      controllerGraph.inspectDiagnostics().outputs.submitReadiness,
      flatGraph.inspectDiagnostics().outputs.submitReadiness,
    );
    assert.deepEqual(
      controllerGraph.inspectHistory().outputs.submitReadiness,
      flatGraph.inspectHistory().outputs.submitReadiness,
    );
    assert.deepEqual(
      controllerGraph.exportCompatibilityDefinition(),
      flatGraph.exportCompatibilityDefinition(),
    );
  } finally {
    await cleanup();
  }
});

test("The Opaque Local Identity Collision Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const rawSignals = createGraphPublicationRuntime();
    const signals = wrapSignals(rawSignals);

    function createCounterController(namespace) {
      const count = namespace.input(0, { debugName: "count" });
      const doubled = namespace.computed(() => count() * 2, { debugName: "count" });
      return { count, doubled };
    }

    const left = createCounterController(signals.scope("leftPanel"));
    const right = createCounterController(signals.scope("rightPanel"));

    assert.equal(left.count.debugName, "count");
    assert.equal(left.doubled.debugName, "count");
    assert.equal(right.count.debugName, "count");
    assert.equal(right.doubled.debugName, "count");
    assert.notEqual(left.count.id, "count");
    assert.notEqual(left.doubled.id, "count");
    assert.notEqual(right.count.id, "count");
    assert.notEqual(right.doubled.id, "count");
    assert.notEqual(left.count.id, left.doubled.id);
    assert.notEqual(right.count.id, right.doubled.id);
    assert.notEqual(left.count.id, right.count.id);
    assert.notEqual(left.doubled.id, right.doubled.id);
  } finally {
    await cleanup();
  }
});

test("The Repeated And Dynamic Instance Identity Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const rawSignals = createGraphPublicationRuntime();
    const signals = wrapSignals(rawSignals);
    const rows = signals.scope("rows");
    const row0 = rows.scope("row-0");
    const row1 = rows.scope("row-1");

    const row0Descriptor = row0.descriptor();
    const row1Descriptor = row1.descriptor();
    const row0Identity = row0.signalIdentity("count");
    const row1Identity = row1.signalIdentity("count");
    const row0Count = row0.input(0, { id: "count" });
    const row1Count = row1.input(1, { id: "count" });

    assert.deepEqual(row0Descriptor.path, [
      { id: "rows", localScopeId: "rows", depth: 1 },
      { id: "rows.row-0", localScopeId: "row-0", depth: 2 },
    ]);
    assert.deepEqual(row1Descriptor.path, [
      { id: "rows", localScopeId: "rows", depth: 1 },
      { id: "rows.row-1", localScopeId: "row-1", depth: 2 },
    ]);
    assert.deepEqual(row0Descriptor.identity, {
      scopeId: "rows.row-0",
      parentScopeId: "rows",
      path: row0Descriptor.path,
      depth: 2,
    });
    assert.equal(row0Identity.localId, "count");
    assert.equal(row0Identity.canonicalId, "rows.row-0.count");
    assert.equal(row0Identity.graphId, null);
    assert.equal(row0Identity.rootScopeId, "rows");
    assert.equal(row1Identity.canonicalId, "rows.row-1.count");
    assert.notDeepEqual(row0Identity.scopePath, row1Identity.scopePath);
    assert.equal(row0Count.id, row0Identity.canonicalId);
    assert.equal(row1Count.id, row1Identity.canonicalId);
    assert.deepEqual(row0Count.signalIdentity(), row0Identity);
    assert.deepEqual(row1Count.signalIdentity(), row1Identity);
  } finally {
    await cleanup();
  }
});

test("The Scoped Graph And Manual Scope Equivalence Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    function createScopedGraph() {
      const rawSignals = createGraphPublicationRuntime();
      const signals = wrapSignals(rawSignals);

      function createEditSessionController(namespace) {
        const count = namespace.input(1, { id: "count" });
        const label = namespace.computed(() => `count:${count()}`, { id: "label" });
        return namespace.controller({
          inputs: { count },
          outputs: { label },
        });
      }

      return signals.graph("itemDetail", (graph) => {
        const controller = createEditSessionController(graph.scope("editSession"));
        return graph.expose({
          controllers: [controller],
          outputs: {
            count: controller.inputs.count,
          },
        });
      });
    }

    function createManualGraph() {
      const rawSignals = createGraphPublicationRuntime();
      const signals = wrapSignals(rawSignals);
      const count = signals.spec.input("itemDetail.editSession.count", 1);
      const label = signals.spec.computedCallback(
        "itemDetail.editSession.label",
        () => `count:${count()}`,
      );
      return signals.graph("itemDetail", {
        inputs: {
          count,
        },
        outputs: {
          label,
          count,
        },
      });
    }

    const scopedGraph = createScopedGraph();
    const manualGraph = createManualGraph();

    assert.deepEqual(scopedGraph.read(), manualGraph.read());
    assert.deepEqual(scopedGraph.readInputs(), manualGraph.readInputs());
    assert.deepEqual(scopedGraph.summary(), manualGraph.summary());
    assert.deepEqual(scopedGraph.contract(), manualGraph.contract());
    assert.deepEqual(scopedGraph.inputDescriptors(), manualGraph.inputDescriptors());
    assert.deepEqual(scopedGraph.descriptors(), manualGraph.descriptors());
    assert.deepEqual(
      materializeGraphDiagnosticsSurface(scopedGraph.inspectDiagnostics()),
      materializeGraphDiagnosticsSurface(manualGraph.inspectDiagnostics()),
    );
    assert.deepEqual(
      materializeGraphHistorySurface(scopedGraph.inspectHistory()),
      materializeGraphHistorySurface(manualGraph.inspectHistory()),
    );
    assert.deepEqual(
      scopedGraph.exportCompatibilityDefinition(),
      manualGraph.exportCompatibilityDefinition(),
    );
  } finally {
    await cleanup();
  }
});

test("The Public Graph Input And Output Contract Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const rawSignals = createGraphPublicationRuntime();
    rawSignals.adapters = () => ({
      export_definitions() {
        return {
          policy: { preset: "WebDevelopment" },
          sources: [
            { id: "itemDetail.editSession.serverItemData", initial: null },
            { id: "itemDetail.editSession.draftEdits", initial: {} },
          ],
          recipes: [
            {
              id: "itemDetail.editSession.effectiveItemData",
              reads: [
                "itemDetail.editSession.serverItemData",
                "itemDetail.editSession.draftEdits",
              ],
              expr: {
                kind: "mergeObjects",
                args: [
                  { kind: "read", id: "itemDetail.editSession.serverItemData" },
                  { kind: "read", id: "itemDetail.editSession.draftEdits" },
                ],
              },
            },
            {
              id: "itemDetail.effectiveItemData",
              reads: ["itemDetail.editSession.effectiveItemData"],
              expr: { kind: "read", id: "itemDetail.editSession.effectiveItemData" },
            },
          ],
          sourceFamilies: [],
          recipeFamilies: [],
          unavailableCallbacks: [],
        };
      },
    });
    const signals = wrapSignals(rawSignals);

    const graph = signals.graph("itemDetail", (builder) => {
      const edit = builder.scope("editSession");
      const serverItemData = edit.input(null, { id: "serverItemData" });
      const draftEdits = edit.input({}, { id: "draftEdits" });
      const effectiveItemData = edit.computed("effectiveItemData", () => ({
        ...(serverItemData() ?? {}),
        ...draftEdits(),
      }));

      return builder.expose({
        inputs: {
          serverItemData,
          draftEdits,
        },
        outputs: {
          effectiveItemData,
        },
      });
    });

    assert.equal(graph.input("serverItemData").id, "itemDetail.editSession.serverItemData");
    assert.equal(graph.inputs.draftEdits.id, "itemDetail.editSession.draftEdits");
    assert.equal(graph.output("effectiveItemData").id, "itemDetail.effectiveItemData");
    assert.deepEqual({ ...graph.readInputs() }, {
      serverItemData: null,
      draftEdits: {},
    });
    assert.deepEqual({ ...graph.read() }, {
      effectiveItemData: {
        id: "itemDetail.effectiveItemData",
        spec: {
          reads: ["itemDetail.editSession.effectiveItemData"],
          expr: {
            kind: "read",
            id: "itemDetail.editSession.effectiveItemData",
          },
        },
      },
    });
    assert.deepEqual(graph.summary(), {
      id: "itemDetail",
      inputCount: 2,
      inputNames: ["serverItemData", "draftEdits"],
      inputSourceIds: [
        "itemDetail.editSession.serverItemData",
        "itemDetail.editSession.draftEdits",
      ],
      outputCount: 1,
      outputNames: ["effectiveItemData"],
      publishedOutputIds: ["itemDetail.effectiveItemData"],
      sourceIds: [
        "itemDetail.editSession.serverItemData",
        "itemDetail.editSession.draftEdits",
        "itemDetail.editSession.effectiveItemData",
      ],
      synthesizedOutputCount: 1,
    });
    assert.deepEqual(graph.inputDescriptors(), [
      {
        inputName: "serverItemData",
        sourceId: "itemDetail.editSession.serverItemData",
        sourceKind: "input",
        authority: "writable",
        requiredness: "required",
      },
      {
        inputName: "draftEdits",
        sourceId: "itemDetail.editSession.draftEdits",
        sourceKind: "input",
        authority: "writable",
        requiredness: "required",
      },
    ]);
    assert.deepEqual({
      ...graph.contract(),
      inputs: { ...graph.contract().inputs },
      outputs: { ...graph.contract().outputs },
    }, {
      graph: graph.summary(),
      inputs: {
        serverItemData: "itemDetail.editSession.serverItemData",
        draftEdits: "itemDetail.editSession.draftEdits",
      },
      outputs: {
        effectiveItemData: "itemDetail.effectiveItemData",
      },
      inputDescriptors: graph.inputDescriptors(),
      descriptors: graph.descriptors(),
    });
    const previousContractSnapshot = {
      ...graph.contract(),
      outputs: {},
    };
    assert.deepEqual(graph.contractDelta(previousContractSnapshot), {
      graphId: "itemDetail",
      previousGraphId: "itemDetail",
      changed: true,
      inputs: {
        added: [],
        removed: [],
        remapped: [],
      },
      outputs: {
        added: ["effectiveItemData"],
        removed: [],
        remapped: [],
      },
      inputDescriptorsChanged: [],
      outputDescriptorsChanged: [],
    });
    const graphDiagnosticsSurface = graph.inspectDiagnostics();
    assert.equal(Object.getPrototypeOf(graphDiagnosticsSurface.inputs), null);
    assert.deepEqual(graphDiagnosticsSurface.contract, graph.contract());
    assert.deepEqual(graphDiagnosticsSurface.inputVersions, [
      { id: "itemDetail.editSession.serverItemData", value_version: 1, shape_version: 10 },
      { id: "itemDetail.editSession.draftEdits", value_version: 2, shape_version: 11 },
    ]);
    assert.deepEqual(
      graphDiagnosticsSurface.dependenciesForOutput("effectiveItemData"),
      {
        graphId: "itemDetail",
        outputName: "effectiveItemData",
        publishedId: "itemDetail.effectiveItemData",
        sourceId: "itemDetail.editSession.effectiveItemData",
        publicInputNames: ["serverItemData", "draftEdits"],
        publicInputSourceIds: [
          "itemDetail.editSession.serverItemData",
          "itemDetail.editSession.draftEdits",
        ],
        transitiveSignalIds: [
          "itemDetail.effectiveItemData",
          "itemDetail.editSession.effectiveItemData",
          "itemDetail.editSession.serverItemData",
          "itemDetail.editSession.draftEdits",
        ],
      },
    );
    assert.deepEqual(
      graphDiagnosticsSurface.contractSummary(),
      {
        graph: graph.summary(),
        contract: graph.contract(),
        inputCount: 2,
        outputCount: 1,
        inputNames: ["serverItemData", "draftEdits"],
        outputNames: ["effectiveItemData"],
        dependencies: graphDiagnosticsSurface.dependencies,
      },
    );
    assert.deepEqual(graphDiagnosticsSurface.inputs.serverItemData, {
      descriptor: graph.inputDescriptors()[0],
      version: { id: "itemDetail.editSession.serverItemData", value_version: 1, shape_version: 10 },
      why: { id: "itemDetail.editSession.serverItemData", family: "why" },
    });
    assert.deepEqual(graphDiagnosticsSurface.outputs.effectiveItemData, {
      descriptor: graph.descriptors()[0],
      version: { id: "itemDetail.effectiveItemData", value_version: 1, shape_version: 10 },
      why: { id: "itemDetail.effectiveItemData", family: "why" },
    });
    const graphHistorySurface = graph.inspectHistory();
    assert.equal(Object.getPrototypeOf(graphHistorySurface.inputs), null);
    assert.deepEqual(graphHistorySurface.contract, graph.contract());
    assert.deepEqual(
      graphHistorySurface.dependenciesForOutput("effectiveItemData"),
      graphDiagnosticsSurface.dependenciesForOutput("effectiveItemData"),
    );
    assert.deepEqual(graphHistorySurface.contractSummary(), graphDiagnosticsSurface.contractSummary());
    assert.deepEqual(graphHistorySurface.inputs.draftEdits, {
      descriptor: graph.inputDescriptors()[1],
      replay: { id: "itemDetail.editSession.draftEdits", family: "replay" },
      lineage: { id: "itemDetail.editSession.draftEdits", family: "lineage" },
    });
    const compatibilityDefinition = graph.exportCompatibilityDefinition();
    assert.deepEqual(compatibilityDefinition.contract, graph.contract());
    assert.deepEqual({ ...compatibilityDefinition.inputs }, {
      serverItemData: "itemDetail.editSession.serverItemData",
      draftEdits: "itemDetail.editSession.draftEdits",
    });
    assert.throws(
      () => signals.graph("broken", (builder) => builder.expose({
        inputs: {
          notAnInput: builder.scope("edit").computed("label", () => "x"),
        },
        outputs: {
          label: builder.scope("edit").computed("label2", () => "y"),
        },
      })),
      /expects an input handle/,
    );
  } finally {
    await cleanup();
  }
});

test("The Graph-Owned Lifecycle Boundary Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const rawSignals = createGraphPublicationRuntime();
    const signals = wrapSignals(rawSignals);

    const ambientCount = signals.spec.input("ambient.count", 1);

    assert.throws(
      () => signals.graph("itemDetail", () => ({
        outputs: {
          count: ambientCount,
        },
      })),
      /must return the result of graph\.expose/,
    );

    assert.throws(
      () => signals.graph("itemDetail", (graph) => graph.expose({
        outputs: {
          count: ambientCount,
        },
      })),
      /must come from graph-owned scope `itemDetail`/,
    );

    assert.throws(
      () => signals.graph("itemDetail", (graph) => {
        const edit = signals.scope("itemDetail.editSession");
        const count = edit.input(1, { id: "count" });
        return graph.expose({
          inputs: {
            count,
          },
          outputs: {
            count,
          },
        });
      }),
      /must come from graph-owned scope `itemDetail`/,
    );
  } finally {
    await cleanup();
  }
});

test("The Forms And Resources Dependency Readiness Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const rawSignals = createGraphPublicationRuntime();
    const signals = wrapSignals(rawSignals);

    function createFormController(namespace) {
      const fields = namespace.scope("fields");
      const serverValue = fields.input({
        id: "task-7",
        title: "Ship docs",
        status: "draft",
      }, { id: "serverValue" });
      const draftValue = fields.input({
        title: "Ship docs",
        status: "ready",
      }, { id: "draftValue" });
      const effectiveValue = fields.computed(() => ({
        ...(serverValue() ?? {}),
        ...draftValue(),
      }), { id: "effectiveValue" });
      const dirtyState = fields.computed(() => ({
        isDirty: Object.keys(draftValue()).length > 0,
      }), { id: "dirtyState" });
      const validation = namespace.computed(() => ({
        titleMissing: !effectiveValue().title,
      }), { id: "validation" });

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
      const routeParams = namespace.input({
        taskId: "task-7",
        workspaceId: "alpha",
      }, { id: "routeParams" });
      const resourceQuery = namespace.computed(() => ({
        taskId: routeParams().taskId,
        workspaceId: routeParams().workspaceId,
        status: form.outputs.effectiveValue().status,
      }), { id: "resourceQuery" });
      const submitAvailability = namespace.computed(() => ({
        enabled: form.outputs.dirtyState().isDirty && !form.outputs.validation().titleMissing,
        taskId: resourceQuery().taskId,
      }), { id: "submitAvailability" });

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
      const resource = createResourceController(graphBuilder.scope("resource"), form);

      return graphBuilder.expose({
        controllers: [form, resource],
      });
    });

    assert.deepEqual({ ...graph.readInputs() }, {
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
    });
    assert.deepEqual({
      ...graph.contract(),
      inputs: { ...graph.contract().inputs },
      outputs: { ...graph.contract().outputs },
    }, {
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
    });
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

test("The Controller Artifact Composition Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const rawSignals = createGraphPublicationRuntime();
    const signals = wrapSignals(rawSignals);

    function createEditSessionController(namespace) {
      return namespace.controller(({ input, computed }) => {
        const serverItemData = input(null, { id: "serverItemData" });
        const draftEdits = input({}, { id: "draftEdits" });
        const effectiveItemData = computed(() => ({
          ...(serverItemData() ?? {}),
          ...draftEdits(),
        }), { id: "effectiveItemData" });
        const dirtyState = computed(() => ({
          isDirty: Object.keys(draftEdits()).length > 0,
        }), { id: "dirtyState" });

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
        const submitReadiness = computed(() => ({
          enabled: editSession.outputs.dirtyState().isDirty,
        }), { id: "submitReadiness" });

        return {
          outputs: {
            submitReadiness,
          },
        };
      });
    }

    const graph = signals.graph("itemDetail", (graphBuilder) => {
      const editSession = graphBuilder.controller("editSession", ({ input, computed }) => {
        const serverItemData = input(null, { id: "serverItemData" });
        const draftEdits = input({}, { id: "draftEdits" });
        const effectiveItemData = computed(() => ({
          ...(serverItemData() ?? {}),
          ...draftEdits(),
        }), { id: "effectiveItemData" });
        const dirtyState = computed(() => ({
          isDirty: Object.keys(draftEdits()).length > 0,
        }), { id: "dirtyState" });

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
      const workflow = createWorkflowController(graphBuilder.scope("workflow"), editSession);
      return graphBuilder.expose({
        controllers: [editSession, workflow],
      });
    });

    assert.deepEqual({ ...graph.readInputs() }, {
      serverItemData: null,
      draftEdits: {},
    });
    assert.deepEqual({
      ...graph.contract(),
      inputs: { ...graph.contract().inputs },
      outputs: { ...graph.contract().outputs },
    }, {
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
    });

    assert.throws(
      () => signals.graph("broken", (graphBuilder) => {
        const editSession = createEditSessionController(graphBuilder.scope("editSession"));
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
      () => signals.graph("broken", (graphBuilder) => graphBuilder.expose({
        controllers: [{}],
        outputs: {
          label: graphBuilder.scope("editSession").computed("label", () => "x"),
        },
      })),
      /must be a controller artifact created by signals\.controller/,
    );

    assert.throws(
      () => signals.controller(() => null),
      /signals\.controller requires a controller definition object/,
    );

    assert.throws(
      () => signals.graph("brokenBuilder", (graphBuilder) => graphBuilder.controller("editSession", () => null)),
      /signals\.controller requires a controller definition object/,
    );

    assert.throws(
      () => signals.controller({
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
      () => signals.controller({
        internal: {
          leakedAuthority: signals.publicInput(
            signals.input({ taskId: "task-7" }, { debugName: "routeParamsInternal" }),
            { authority: "readOnly" },
          ),
        },
      }),
      /controller\.internal\.`leakedAuthority` cannot use signals\.publicInput/,
    );

    assert.throws(
      () => signals.controller({
        inputs: {
          notAnInput: signals.computed(() => "nope", { debugName: "notAnInput" }),
        },
      }),
      /controller\.inputs\.`notAnInput` must be an input handle or signals\.publicInput/,
    );
  } finally {
    await cleanup();
  }
});

test("The Controller Contract Internal Boundary Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const rawSignals = createGraphPublicationRuntime();
    const signals = wrapSignals(rawSignals);

    const graph = signals.graph("itemDetail", (graphBuilder) => {
      const editSession = graphBuilder.scope("editSession");
      const serverItemData = editSession.input(null, { id: "serverItemData" });
      const effectiveItemData = editSession.computed("effectiveItemData", () => ({
        ...(serverItemData() ?? {}),
      }));
      const validationTrace = editSession.computed("validationTrace", () => ({
        fieldCount: Object.keys(serverItemData() ?? {}).length,
      }));

      const controller = editSession.controller({
        inputs: {
          serverItemData,
        },
        outputs: {
          effectiveItemData,
        },
        internal: {
          validationTrace,
        },
      });

      return graphBuilder.expose({
        controllers: [controller],
      });
    });

    assert.equal("validationTrace" in graph.contract().inputs, false);
    assert.equal("validationTrace" in graph.contract().outputs, false);
    assert.equal(
      graph.inputDescriptors().some((descriptor) => descriptor.inputName === "validationTrace"),
      false,
    );
    assert.equal(
      graph.descriptors().some((descriptor) => descriptor.outputName === "validationTrace"),
      false,
    );
    assert.equal("validationTrace" in graph.inspectDiagnostics().inputs, false);
    assert.equal("validationTrace" in graph.inspectDiagnostics().outputs, false);
    assert.equal("validationTrace" in graph.inspectHistory().inputs, false);
    assert.equal("validationTrace" in graph.inspectHistory().outputs, false);
    assert.equal("validationTrace" in graph.exportCompatibilityDefinition().inputs, false);
    assert.equal("validationTrace" in graph.exportCompatibilityDefinition().outputs, false);
  } finally {
    await cleanup();
  }
});

test("The Public Input Requiredness Contract Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const rawSignals = createGraphOperationalRuntime();
    const signals = wrapSignals(rawSignals);

    const graph = signals.graph("taskRequiredness", (builder) => {
      const scope = builder.scope("requiredness");
      const serverValue = scope.input({
        id: "task-7",
        title: "Ship docs",
      }, { id: "serverValue" });
      const draftValue = scope.input({
        title: "Ship docs",
      }, { id: "draftValue" });
      const effectiveValue = scope.computed(() => ({
        ...serverValue(),
        ...draftValue(),
      }), { id: "effectiveValue" });

      return builder.expose({
        inputs: {
          serverValue: builder.input.required(serverValue, { authority: "readOnly" }),
          draftValue: builder.input.optional(draftValue),
        },
        outputs: {
          effectiveValue,
        },
      });
    });

    assert.deepEqual(
      graph.inputDescriptors().map((descriptor) => ({
        inputName: descriptor.inputName,
        authority: descriptor.authority,
        requiredness: descriptor.requiredness,
      })),
      [
        { inputName: "serverValue", authority: "readOnly", requiredness: "required" },
        { inputName: "draftValue", authority: "writable", requiredness: "optional" },
      ],
    );
    assert.equal(
      graph.operationalContract().authorities.serverValue.requiredness,
      "required",
    );
    assert.equal(
      graph.operationalContract().authorities.draftValue.requiredness,
      "optional",
    );
    assert.equal(graph.contract().inputDescriptors[0].requiredness, "required");
    assert.equal(graph.exportDefinition().inputDescriptors[1].requiredness, "optional");
    assert.deepEqual(
      graph.contractDelta({
        ...graph.contract(),
        inputDescriptors: graph.contract().inputDescriptors.map((descriptor) => (
          descriptor.inputName === "draftValue"
            ? { ...descriptor, requiredness: "required" }
            : descriptor
        )),
      }).inputDescriptorsChanged,
      [
        {
          inputName: "draftValue",
          previousSourceId: "taskRequiredness.requiredness.draftValue",
          currentSourceId: "taskRequiredness.requiredness.draftValue",
          previousAuthority: "writable",
          currentAuthority: "writable",
          previousRequiredness: "required",
          currentRequiredness: "optional",
        },
      ],
    );
    assert.throws(
      () => signals.graph("invalidRequiredness", (invalidBuilder) => {
        const scope = invalidBuilder.scope("boundary");
        const source = scope.input(1, { id: "source" });
        return invalidBuilder.expose({
          inputs: {
            source: invalidBuilder.input.required(source, {
              authority: "readOnly",
              requiredness: "optional",
            }),
          },
          outputs: {
            sourceEcho: scope.output(() => source(), { id: "sourceEcho" }),
          },
        });
      }),
      {
        name: "TypeError",
        message:
          "signals.graph `invalidRequiredness` input.required(...) does not accept an explicit requiredness override; use input.required(...) to choose the boundary contract form",
      },
    );
  } finally {
    await cleanup();
  }
});

test("The Public Input Authority Class Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const rawSignals = createGraphOperationalRuntime();
    const signals = wrapSignals(rawSignals);

    const graph = signals.graph("taskEditor", (builder) => {
      const scope = builder.scope("form");
      const serverValue = scope.input({
        id: "task-7",
        title: "Ship docs",
      }, { id: "serverValue" });
      const draftValue = scope.input({
        title: "Ship docs",
      }, { id: "draftValue" });
      const externalParams = scope.input({
        taskId: "task-7",
      }, { id: "externalParams" });
      const effectiveValue = scope.computed(() => ({
        ...serverValue(),
        ...draftValue(),
      }), { id: "effectiveValue" });

      return builder.expose({
        inputs: {
          serverValue: scope.publicInput(serverValue, { authority: "readOnly" }),
          draftValue: scope.publicInput(draftValue, { authority: "writable" }),
          externalParams: scope.publicInput(externalParams, { authority: "imported" }),
        },
        outputs: {
          effectiveValue,
        },
      });
    });

    assert.deepEqual(
      graph.inputDescriptors().map((descriptor) => ({
        inputName: descriptor.inputName,
        authority: descriptor.authority,
        requiredness: descriptor.requiredness,
      })),
      [
        { inputName: "serverValue", authority: "readOnly", requiredness: "required" },
        { inputName: "draftValue", authority: "writable", requiredness: "required" },
        { inputName: "externalParams", authority: "imported", requiredness: "required" },
      ],
    );
    assert.deepEqual(
      {
        serverValue: graph.operationalContract().authorities.serverValue,
        draftValue: graph.operationalContract().authorities.draftValue,
        externalParams: graph.operationalContract().authorities.externalParams,
      },
      {
        serverValue: {
          inputName: "serverValue",
          sourceId: "taskEditor.form.serverValue",
          authority: "readOnly",
          requiredness: "required",
          supportsWrite: false,
          supportsPatch: false,
          supportsReset: false,
        },
        draftValue: {
          inputName: "draftValue",
          sourceId: "taskEditor.form.draftValue",
          authority: "writable",
          requiredness: "required",
          supportsWrite: true,
          supportsPatch: true,
          supportsReset: true,
        },
        externalParams: {
          inputName: "externalParams",
          sourceId: "taskEditor.form.externalParams",
          authority: "imported",
          requiredness: "required",
          supportsWrite: false,
          supportsPatch: false,
          supportsReset: false,
        },
      },
    );

    graph.writeInputs({
      draftValue: {
        title: "Ready to ship",
      },
    });
    assert.deepEqual(graph.readInputs().draftValue, {
      title: "Ready to ship",
    });

    assert.throws(
      () => graph.writeInputs({
        serverValue: {
          id: "task-7",
          title: "Nope",
        },
      }),
      /cannot write public input `serverValue` because its authority is `readOnly`/,
    );
    assert.throws(
      () => graph.patchInputs({
        externalParams: {
          taskId: "task-8",
        },
      }),
      /authority is `imported`/,
    );
    assert.throws(
      () => graph.resetInputs(["serverValue"]),
      /cannot reset public input `serverValue` because its authority is `readOnly`/,
    );
    assert.throws(
      () => graph.transaction((tx) => {
        tx.set("externalParams", { taskId: "task-9" });
      }),
      /authority is `imported`/,
    );
  } finally {
    await cleanup();
  }
});

test("The Graph-Native Input Operations Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const rawSignals = createGraphOperationalRuntime();
    const signals = wrapSignals(rawSignals);
    const localDraft = signals.input({
      title: "Ship docs",
      done: false,
    }, { debugName: "localDraft" });

    const graph = signals.graph("taskAuthority", (graphBuilder) => {
      const scope = graphBuilder.scope("authority");
      const serverValue = scope.input({
        id: "task-7",
        title: "Ship docs",
      }, { id: "serverValue" });
      const draftValue = scope.input({
        title: "Ship docs",
      }, { id: "draftValue" });
      const externalParams = scope.input({
        taskId: "task-7",
      }, { id: "externalParams" });
      const effectiveValue = scope.computed(() => ({
        ...serverValue(),
        ...draftValue(),
        taskId: externalParams().taskId,
      }), { id: "effectiveValue" });

      return graphBuilder.expose({
        controllers: [
          scope.controller({
            inputs: {
              serverValue: scope.publicInput(serverValue, { authority: "readOnly" }),
              draftValue: scope.publicInput(draftValue),
              externalParams: scope.publicInput(externalParams, { authority: "imported" }),
            },
            outputs: {
              effectiveValue,
            },
          }),
        ],
      });
    });

    assert.equal(graph.input("draftValue").id, "taskAuthority.authority.draftValue");
    assert.equal(graph.inputs.externalParams.id, "taskAuthority.authority.externalParams");
    assert.equal(graph.output("effectiveValue").id, "taskAuthority.effectiveValue");
    assert.deepEqual({ ...graph.readInputs() }, {
      serverValue: {
        id: "task-7",
        title: "Ship docs",
      },
      draftValue: {
        title: "Ship docs",
      },
      externalParams: {
        taskId: "task-7",
      },
    });

    localDraft.patch({
      done: true,
    });
    localDraft.assign({
      title: "Ready to ship",
    });
    signals.transaction((tx) => {
      tx.patch(localDraft, {
        status: "queued",
      });
    });
    assert.deepEqual(localDraft(), {
      title: "Ready to ship",
      done: true,
      status: "queued",
    });
    assert.throws(
      () => signals.input(1, { debugName: "primitiveCount" }).patch(2),
      /input\.patch\(\.\.\.\) requires object or array values/,
    );

    graph.writeInputs({
      draftValue: {
        title: "Ready to ship",
      },
    });
    graph.writeInput("draftValue", {
      title: "Reviewed",
    });
    graph.patchInputs({
      draftValue: {
        status: "queued",
      },
    });
    graph.patchInput("draftValue", {
      priority: "high",
    });
    graph.transaction((tx) => {
      tx.set("draftValue", {
        title: "Queued",
        status: "queued",
        priority: "high",
      });
    });
    graph.transaction((tx) => {
      tx.patch("draftValue", {
        reviewer: "Avery",
      });
    });
    graph.apply({
      writes: {
        draftValue: {
          title: "Approved",
          status: "approved",
          priority: "high",
          reviewer: "Avery",
        },
      },
      commands: {},
    });
    graph.resetInput("draftValue");

    assert.deepEqual({ ...graph.readInputs() }, {
      serverValue: {
        id: "task-7",
        title: "Ship docs",
      },
      draftValue: {
        title: "Ship docs",
      },
      externalParams: {
        taskId: "task-7",
      },
    });
    assert.equal(graph.read().effectiveValue.id, "taskAuthority.effectiveValue");
    assert.throws(
      () => graph.transaction((tx) => {
        tx.patch("serverValue", {
          title: "Nope",
        });
      }),
      /cannot patch public input `serverValue`/,
    );
  } finally {
    await cleanup();
  }
});

test("The Graph-Native Export And Restore Equivalence Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const sourceSignals = wrapSignals(createGraphExportImportRuntime());
    const count = sourceSignals.input(2, { debugName: "count" });
    const displayLabel = sourceSignals.computed(() => `Count:${count() * 2}`, { debugName: "displayLabel" });
    const namingGraph = sourceSignals.graph("naming", {
      inputs: {
        count,
      },
      outputs: {
        publicDisplayName: displayLabel,
      },
    });

    count.set(4);
    const exportedDefinition = namingGraph.exportDefinition();
    const exportedSnapshot = namingGraph.exportSnapshot();
    assert.deepEqual(namingGraph.importPosture(), {
      graphId: "naming",
      exactRestoreMode: "SameRuntimeExact",
      portableImport: "Denied",
      portableImportReason: "graph-native import currently requires the exact originating runtime envelope",
      hydrate: "Deferred",
      hydrateReason: "graph-native portable hydrate is not yet admitted on this surface",
    });
    assert.deepEqual(exportedDefinition.importPosture, namingGraph.importPosture());
    assert.deepEqual(exportedSnapshot.importPosture, namingGraph.importPosture());

    const restoredSignals = wrapSignals(createGraphExportImportRuntime());
    const restoredGraph = restoredSignals.importGraph(exportedDefinition, exportedSnapshot);

    assert.deepEqual({ ...restoredGraph.readInputs() }, { count: 4 });
    assert.deepEqual({ ...restoredGraph.read() }, { publicDisplayName: "Count:8" });
    assert.deepEqual(restoredGraph.contract(), namingGraph.contract());
    assert.deepEqual(restoredGraph.importPosture(), namingGraph.importPosture());
    assert.deepEqual(restoredGraph.contractHistory(), {
      graphId: "naming",
      current: namingGraph.contract(),
      baseline: namingGraph.contract(),
      deltas: [
        {
          graphId: "naming",
          previousGraphId: "naming",
          changed: false,
          inputs: {
            added: [],
            removed: [],
            remapped: [],
          },
          outputs: {
            added: [],
            removed: [],
            remapped: [],
          },
          inputDescriptorsChanged: [],
          outputDescriptorsChanged: [],
        },
      ],
      changedSinceBaseline: false,
      restoreMode: "SameRuntimeExact",
      importedFromGraphId: "naming",
    });
    assert.equal(
      restoredGraph.exportCompatibilityDefinition().outputs.publicDisplayName,
      "naming.publicDisplayName",
    );
    assert.deepEqual(
      restoredGraph.inspectDiagnostics().dependenciesForOutput("publicDisplayName").publicInputNames,
      ["count"],
    );
    assert.equal(
      restoredGraph.inspectHistory().output("publicDisplayName").replay.id,
      "naming.publicDisplayName",
    );

    assert.throws(
      () => restoredSignals.importGraph(
        exportedDefinition,
        { ...exportedSnapshot, id: "other" },
      ),
      /requires matching graph ids/,
    );
    assert.throws(
      () => restoredSignals.importGraph(
        {
          ...exportedDefinition,
          contract: {
            ...exportedDefinition.contract,
            outputs: {
              ...exportedDefinition.contract.outputs,
              publicDisplayName: "naming.other",
            },
          },
        },
        exportedSnapshot,
      ),
      /snapshot\.definition\.contract to match the exported graph definition/,
    );
  } finally {
    await cleanup();
  }
});

test("The Public Boundary Naming Truth Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphExportImportRuntime());
    const name = signals.input("Ada", { debugName: "name" });
    const displayLabel = signals.computed(() => name().toUpperCase(), { debugName: "displayLabel" });
    const namingGraph = signals.graph("naming", {
      inputs: {
        name,
      },
      outputs: {
        publicDisplayName: displayLabel,
      },
    });

    assert.equal(name.debugName, "name");
    assert.equal(displayLabel.debugName, "displayLabel");
    assert.equal(namingGraph.output("publicDisplayName").id, "naming.publicDisplayName");
    assert.deepEqual({
      ...namingGraph.contract(),
      inputs: { ...namingGraph.contract().inputs },
      outputs: { ...namingGraph.contract().outputs },
    }, {
      graph: namingGraph.summary(),
      inputs: {
        name: name.id,
      },
      outputs: {
        publicDisplayName: "naming.publicDisplayName",
      },
      inputDescriptors: namingGraph.inputDescriptors(),
      descriptors: namingGraph.descriptors(),
    });
    assert.deepEqual(namingGraph.descriptors(), [
      {
        outputName: "publicDisplayName",
        sourceId: displayLabel.id,
        sourceKind: "computed",
        publishedId: "naming.publicDisplayName",
        publicationKind: "synthesizedOutput",
      },
    ]);
    assert.equal(
      namingGraph.exportCompatibilityDefinition().contract.outputs.publicDisplayName,
      "naming.publicDisplayName",
    );
  } finally {
    await cleanup();
  }
});

test("The Contract Dependency Explanation Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphExportImportRuntime());
    const firstName = signals.spec.input("firstName", "Ada");
    const status = signals.spec.input("status", "ready");
    const displayLabel = signals.spec.computedCallback(
      "displayLabel",
      () => `${firstName()} (${status()})`,
    );
    const graph = signals.graph("personCard", {
      inputs: {
        firstName,
        status,
      },
      outputs: {
        publicDisplayName: displayLabel,
      },
    });

    assert.deepEqual(
      graph.inspectDiagnostics().dependenciesForOutput("publicDisplayName"),
      {
        graphId: "personCard",
        outputName: "publicDisplayName",
        publishedId: "personCard.publicDisplayName",
        sourceId: "displayLabel",
        publicInputNames: ["firstName", "status"],
        publicInputSourceIds: ["firstName", "status"],
        transitiveSignalIds: [
          "personCard.publicDisplayName",
          "displayLabel",
          "firstName",
          "status",
        ],
      },
    );
    assert.deepEqual(
      graph.inspectHistory().dependenciesForOutput("publicDisplayName"),
      graph.inspectDiagnostics().dependenciesForOutput("publicDisplayName"),
    );
  } finally {
    await cleanup();
  }
});

test("The Contract Delta And History Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const firstSignals = wrapSignals(createGraphExportImportRuntime());
    const firstName = firstSignals.spec.input("name", "Ada");
    const displayLabel = firstSignals.spec.computedCallback(
      "displayLabel",
      () => firstName().toUpperCase(),
    );
    const graphV1 = firstSignals.graph("naming", {
      inputs: {
        name: firstName,
      },
      outputs: {
        publicDisplayName: displayLabel,
      },
    });

    const secondSignals = wrapSignals(createGraphExportImportRuntime());
    const secondName = secondSignals.spec.input("name", "Ada");
    const displayNameV2 = secondSignals.spec.computedCallback(
      "displayNameV2",
      () => `Person:${secondName()}`,
    );
    const graphV2 = secondSignals.graph("naming", {
      inputs: {
        name: secondName,
      },
      outputs: {
        publicDisplayName: displayNameV2,
      },
    });

    assert.deepEqual(graphV2.contractDelta(graphV1.contract()), {
      graphId: "naming",
      previousGraphId: "naming",
      changed: true,
      inputs: {
        added: [],
        removed: [],
        remapped: [],
      },
      outputs: {
        added: [],
        removed: [],
        remapped: [],
      },
      inputDescriptorsChanged: [],
      outputDescriptorsChanged: [
        {
          outputName: "publicDisplayName",
          previousSourceId: "displayLabel",
          currentSourceId: "displayNameV2",
          previousPublishedId: "naming.publicDisplayName",
          currentPublishedId: "naming.publicDisplayName",
          previousSourceKind: "computed",
          currentSourceKind: "computed",
          previousPublicationKind: "synthesizedOutput",
          currentPublicationKind: "synthesizedOutput",
        },
      ],
    });
    assert.deepEqual(graphV1.contractHistory(), {
      graphId: "naming",
      current: graphV1.contract(),
      baseline: null,
      deltas: [],
      changedSinceBaseline: false,
      restoreMode: "LiveRuntime",
      importedFromGraphId: null,
    });
  } finally {
    await cleanup();
  }
});

test("The Ergonomic Mutation Envelope Equivalence Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const rawSignals = createGraphOperationalRuntime();
    const signals = wrapSignals(rawSignals);

    function createEditSessionController(namespace) {
      const serverItemData = namespace.input(null, { id: "serverItemData" });
      const draftEdits = namespace.input({}, { id: "draftEdits" });
      const effectiveItemData = namespace.computed(() => ({
        ...(serverItemData() ?? {}),
        ...draftEdits(),
      }), { id: "effectiveItemData" });

      return namespace.controller({
        inputs: {
          serverItemData,
          draftEdits,
        },
        outputs: {
          effectiveItemData,
        },
      });
    }

    const graph = signals.graph("itemDetail", (graphBuilder) => {
      const editSession = createEditSessionController(graphBuilder.scope("editSession"));
      return graphBuilder.expose({
        controllers: [editSession],
      });
    });
    const foreignGraph = signals.graph("otherDetail", (graphBuilder) => {
      const editSession = createEditSessionController(graphBuilder.scope("editSession"));
      return graphBuilder.expose({
        controllers: [editSession],
      });
    });

    assert.deepEqual({
      ...graph.operationalContract(),
      writes: { ...graph.operationalContract().writes },
      patches: { ...graph.operationalContract().patches },
      commands: { ...graph.operationalContract().commands },
      authorities: Object.fromEntries(
        Object.entries(graph.operationalContract().authorities).map(([inputName, authority]) => [
          inputName,
          { ...authority },
        ]),
      ),
    }, {
      graph: graph.summary(),
      writes: {
        serverItemData: "itemDetail.editSession.serverItemData",
        draftEdits: "itemDetail.editSession.draftEdits",
      },
      patches: {
        draftEdits: "itemDetail.editSession.draftEdits",
      },
      commands: {},
      authorities: {
        serverItemData: {
          inputName: "serverItemData",
          sourceId: "itemDetail.editSession.serverItemData",
          authority: "writable",
          requiredness: "required",
          supportsWrite: true,
          supportsPatch: false,
          supportsReset: true,
        },
        draftEdits: {
          inputName: "draftEdits",
          sourceId: "itemDetail.editSession.draftEdits",
          authority: "writable",
          requiredness: "required",
          supportsWrite: true,
          supportsPatch: true,
          supportsReset: true,
        },
      },
      resettableInputNames: ["serverItemData", "draftEdits"],
    });

    graph.writeInputs({
      serverItemData: {
        workflow_target_state_id: 7,
      },
    });
    assert.deepEqual({ ...graph.readInputs() }, {
      serverItemData: {
        workflow_target_state_id: 7,
      },
      draftEdits: {},
    });

    graph.patchInputs({
      draftEdits: {
        title: "Ship docs",
      },
    });
    assert.deepEqual({ ...graph.readInputs() }, {
      serverItemData: {
        workflow_target_state_id: 7,
      },
      draftEdits: {
        title: "Ship docs",
      },
    });

    graph.transaction((tx) => {
      tx.set("draftEdits", {
        title: "Ready to ship",
      });
    });
    assert.deepEqual(graph.readInputs().draftEdits, {
      title: "Ready to ship",
    });

    graph.apply({
      writes: {
        serverItemData: {
          workflow_target_state_id: 12,
        },
      },
      patches: {
        draftEdits: {
          priority: "high",
        },
      },
    });
    assert.deepEqual({ ...graph.readInputs() }, {
      serverItemData: {
        workflow_target_state_id: 12,
      },
      draftEdits: {
        title: "Ready to ship",
        priority: "high",
      },
    });

    graph.resetInputs(["draftEdits"]);
    assert.deepEqual({ ...graph.readInputs() }, {
      serverItemData: {
        workflow_target_state_id: 12,
      },
      draftEdits: {},
    });

    graph.resetInputs();
    assert.deepEqual({ ...graph.readInputs() }, {
      serverItemData: null,
      draftEdits: {},
    });

    assert.throws(
      () => graph.writeInputs({
        missingInput: 7,
      }),
      /itemDetail\.missingInput.*public input contract/,
    );
    assert.throws(
      () => graph.patchInputs({
        serverItemData: {
          title: "Nope",
        },
      }),
      /does not admit patches for it/,
    );
    assert.throws(
      () => graph.apply({
        writes: {
          draftEdits: {},
        },
        reset: ["draftEdits"],
      }),
      /cannot both write and reset public input `draftEdits`/,
    );
    assert.throws(
      () => graph.transaction((tx) => {
        tx.set(foreignGraph.inputs.draftEdits, {});
      }),
      /outside the graph contract/,
    );

    assert.deepEqual(
      rawSignals.callLog.filter(([family]) => family === "transaction").map(([, ops]) => ops),
      [
        [["set", "itemDetail.editSession.serverItemData", { workflow_target_state_id: 7 }]],
        [["set", "itemDetail.editSession.draftEdits", { title: "Ship docs" }]],
        [["set", "itemDetail.editSession.draftEdits", { title: "Ready to ship" }]],
        [
          ["set", "itemDetail.editSession.serverItemData", { workflow_target_state_id: 12 }],
          ["set", "itemDetail.editSession.draftEdits", { title: "Ready to ship", priority: "high" }],
        ],
        [["set", "itemDetail.editSession.draftEdits", {}]],
        [
          ["set", "itemDetail.editSession.serverItemData", null],
          ["set", "itemDetail.editSession.draftEdits", {}],
        ],
      ],
    );
  } finally {
    await cleanup();
  }
});

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
          subscribe() { return { free() {} }; },
          why(id) { return { id, family: "why" }; },
          health() { return null; },
          summaryNow() { return { profile: "WebDevelopment", active_node_count: 0 }; },
          historyNow() { return { history: { profile: "WebDevelopment", traced_node_count: 0, execution_record_count: 0, latest_execution_record_id: 0, reuse_origin_counts: {}, nodes: [] }, callbackNodes: [] }; },
          latestObservation() { return null; },
          latestFlow() { return null; },
          performanceSummary() { return {}; },
          latestFailure() { return null; },
          latestRollback() { return null; },
          latestFrontierExecution() { return null; },
          latestInvalidationTraceRecords() { return []; },
          recentHistory() { return []; },
        };
      },
      history() {
        return {
          replay_for(id) { return { id, family: "replay", frames: [] }; },
          lineage_for(id) { return { id, family: "lineage" }; },
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
        signals.computed(() => {
          graph.patchInputs({
            draftEdits: {
              title: "Nope",
            },
          });
          return 1;
        }, { debugName: "illegalPatch" });
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

test("wrapSignals exposes a typed specialist wrapper without dropping legacy expert methods", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const rawSignals = {
      input() {
        throw new Error("input not needed");
      },
      computedSpec() {
        throw new Error("computedSpec not needed");
      },
      computedCallback() {
        throw new Error("computedCallback not needed");
      },
      outputSpec() {
        throw new Error("outputSpec not needed");
      },
      read() {
        throw new Error("read not needed");
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
        return {
          evaluate_dirty() {
            return { touchedNodes: 3, nodesEvaluated: 2 };
          },
          graph_summary() {
            return { profile: "Development", activeNodeCount: 4 };
          },
          read_versions(ids) {
            return ids.map((id, index) => ({ id, version: index + 1 }));
          },
          free() {},
        };
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
    const specialist = signals.specialist();

    assert.equal(specialist.graphSummary().profile, "Development");
    assert.equal(specialist.graph_summary().activeNodeCount, 4);
    assert.equal(specialist.evaluateDirty().touchedNodes, 3);
    assert.equal(specialist.evaluate_dirty().nodesEvaluated, 2);
    assert.deepEqual(specialist.readVersions(["a", "b"]), [
      { id: "a", version: 1 },
      { id: "b", version: 2 },
    ]);
    assert.deepEqual(specialist.read_versions(["c"]), [{ id: "c", version: 1 }]);
  } finally {
    await cleanup();
  }
});

test("wrapSignals history wrapper accepts numeric branch ids and normalizes preview requests", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const calls = [];
    const rawHistory = {
      current_branch() {
        return { id: 7, name: "main", parent_branch_id: null, head_snapshot_id: null };
      },
      replay_for_branch(branchId) {
        calls.push(["replay_for_branch", typeof branchId, branchId]);
        return { frames: [] };
      },
      branch_snapshot(branchId) {
        calls.push(["branch_snapshot", typeof branchId, branchId]);
        return { meta: { branch_id: 7 } };
      },
      branch_snapshot_wire(branchId) {
        calls.push(["branch_snapshot_wire", typeof branchId, branchId]);
        return JSON.stringify({ meta: { branch_id: 7 } });
      },
      branch_snapshot_portable_wire(branchId) {
        calls.push(["branch_snapshot_portable_wire", typeof branchId, branchId]);
        return JSON.stringify({ meta: { branch_id: 7 } });
      },
      branch_snapshot_envelope(branchId) {
        calls.push(["branch_snapshot_envelope", typeof branchId, branchId]);
        return { snapshot: { meta: { branch_id: 7 } }, state: { sources: [], recipes: [] } };
      },
      branch_snapshot_envelope_wire(branchId) {
        calls.push(["branch_snapshot_envelope_wire", typeof branchId, branchId]);
        return JSON.stringify({ snapshot: { meta: { branch_id: 7 } }, state: { sources: [], recipes: [] } });
      },
      branch_snapshot_envelope_portable_wire(branchId) {
        calls.push(["branch_snapshot_envelope_portable_wire", typeof branchId, branchId]);
        return JSON.stringify({ snapshot: { meta: { branch_id: 7 } }, state: { sources: [], recipes: [] } });
      },
      restore_snapshot(snapshot) {
        calls.push(["restore_snapshot", snapshot.snapshot.meta.branch_id]);
      },
      restore_snapshot_wire(snapshot) {
        calls.push(["restore_snapshot_wire", JSON.parse(snapshot).snapshot.meta.branch_id]);
      },
      restore_branch_snapshot(branchId, snapshot) {
        calls.push(["restore_branch_snapshot", typeof branchId, branchId, snapshot.meta.branch_id]);
      },
      restore_branch_snapshot_wire(branchId, snapshot) {
        calls.push(["restore_branch_snapshot_wire", typeof branchId, branchId, JSON.parse(snapshot).meta.branch_id]);
      },
      restore_branch_snapshot_portable_wire(branchId, snapshot) {
        calls.push(["restore_branch_snapshot_portable_wire", typeof branchId, branchId, JSON.parse(snapshot).meta.branch_id]);
      },
      branch_state_proof(branchId) {
        calls.push(["branch_state_proof", typeof branchId, branchId]);
        return { stateDigest: "digest" };
      },
      replay_parity_proof(expectedBranchId, replayedBranchId) {
        calls.push(["replay_parity_proof", typeof expectedBranchId, expectedBranchId, replayedBranchId]);
        return { parity: true, mismatch_classes: [] };
      },
      replay_artifact_proof(expected, replayedBranchId) {
        calls.push(["replay_artifact_proof", expected.branchStateDigest, typeof replayedBranchId, replayedBranchId]);
        return { parity: true, mismatch_classes: [] };
      },
      plan_merge_policy_preview(request) {
        calls.push(["plan_merge_policy_preview", request]);
        return { source_branch_id: request.source_branch_id };
      },
      plan_merge_policy_preview_with_proof(request) {
        calls.push(["plan_merge_policy_preview_with_proof", request]);
        return { plan: { source_branch_id: request.source_branch_id }, proof: { planDigest: "plan" } };
      },
      merge_branches_policy_preview(request) {
        calls.push(["merge_branches_policy_preview", request]);
        return { target_branch: request.target_branch_id };
      },
      merge_branches_policy_preview_with_proof(request) {
        calls.push(["merge_branches_policy_preview_with_proof", request]);
        return { result: { target_branch: request.target_branch_id }, proof: { resultDigest: "result" } };
      },
      free() {},
    };

    const rawSignals = {
      input(id, initial) {
        return createRawReadableHandle(id, initial);
      },
      computedSpec(id, spec) {
        return createRawReadableHandle(id, spec);
      },
      computedCallback(id) {
        return createRawReadableHandle(id, id);
      },
      outputSpec(id, spec) {
        return createRawReadableHandle(id, spec);
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
          why() { return null; },
          health() { return null; },
          summaryNow() { return null; },
          historyNow() { return null; },
          latestObservation() { return null; },
          latestFlow() { return null; },
          performanceSummary() { return {}; },
          latestFailure() { return null; },
          latestRollback() { return null; },
          latestFrontierExecution() { return null; },
          latestInvalidationTraceRecords() { return []; },
          recentHistory() { return []; },
          subscribe() { return { free() {} }; },
        };
      },
      history() {
        return rawHistory;
      },
      specialist() {
        return {};
      },
      adapters() {
        return { free() {} };
      },
      compatibilityApp() {
        return {};
      },
      compatibilityRuntime() {
        return {};
      },
      free() {},
    };

    const signals = wrapSignals(rawSignals);
    const history = signals.history();
    const currentBranch = history.current_branch();
    const snapshot = history.branch_snapshot(currentBranch.id);
    const envelope = history.branch_snapshot_envelope(currentBranch.id);
    history.restore_exact_snapshot(envelope);
    history.restore_exact_branch_snapshot(currentBranch.id, snapshot);
    const proof = history.branch_state_proof(currentBranch.id);
    const parity = history.replay_parity_proof(currentBranch.id, currentBranch.id);
    const artifact = history.replay_artifact_proof({ branchStateDigest: proof.stateDigest }, currentBranch.id);
    const previewPlan = history.plan_merge_policy_preview({
      source_branch_id: currentBranch.id,
      target_branch_id: currentBranch.id,
    });
    const previewPlanProof = history.plan_merge_policy_preview_with_proof({
      source_branch_id: currentBranch.id,
      target_branch_id: currentBranch.id,
    });
    const previewResult = history.merge_branches_policy_preview({
      source_branch_id: currentBranch.id,
      target_branch_id: currentBranch.id,
    });
    const previewResultProof = history.merge_branches_policy_preview_with_proof({
      source_branch_id: currentBranch.id,
      target_branch_id: currentBranch.id,
    });

    assert.equal(previewPlan.source_branch_id, 7);
    assert.equal(previewPlanProof.proof.planDigest, "plan");
    assert.equal(previewResult.target_branch, 7);
    assert.equal(previewResultProof.proof.resultDigest, "result");
    assert.equal(parity.parity, true);
    assert.equal(artifact.parity, true);
    assert.equal(typeof snapshot.snapshotRestoreToken, "string");
    assert.equal(snapshot.snapshotRestoreMode, "SameRuntimeExact");
    assert.equal(typeof snapshot.snapshotPortableWire, "string");
    assert.equal(typeof envelope.snapshotEnvelopeRestoreToken, "string");
    assert.equal(envelope.snapshotEnvelopeRestoreMode, "SameRuntimeExact");
    assert.equal(typeof envelope.snapshotEnvelopePortableWire, "string");

    assert.deepEqual(calls, [
      ["branch_snapshot", "bigint", 7n],
      ["branch_snapshot_wire", "bigint", 7n],
      ["branch_snapshot_portable_wire", "bigint", 7n],
      ["branch_snapshot_envelope", "bigint", 7n],
      ["branch_snapshot_envelope_wire", "bigint", 7n],
      ["branch_snapshot_envelope_portable_wire", "bigint", 7n],
      ["restore_snapshot_wire", 7],
      ["restore_branch_snapshot_wire", "bigint", 7n, 7],
      ["branch_state_proof", "bigint", 7n],
      ["replay_parity_proof", "bigint", 7n, 7n],
      ["replay_artifact_proof", "digest", "bigint", 7n],
      ["plan_merge_policy_preview", { source_branch_id: 7, target_branch_id: 7 }],
      ["plan_merge_policy_preview_with_proof", { source_branch_id: 7, target_branch_id: 7 }],
      ["merge_branches_policy_preview", { source_branch_id: 7, target_branch_id: 7 }],
      ["merge_branches_policy_preview_with_proof", { source_branch_id: 7, target_branch_id: 7 }],
    ]);

    assert.throws(
      () => history.switch_branch(-1),
      /history\.switch_branch expects a non-negative safe integer branch id/,
    );
    assert.throws(
      () => history.plan_merge_policy_preview("bad"),
      /history\.plan_merge_policy_preview expects a merge preview request object/,
    );
    assert.throws(
      () => history.restore_exact_snapshot({ snapshot: { meta: { branch_id: 7 } }, state: { sources: [], recipes: [] } }),
      /history\.restore_exact_snapshot expects an artifact returned by history\.snapshot\(\) or history\.branch_snapshot_envelope\(\)/,
    );
    assert.throws(
      () => history.plan_merge_policy_preview({
        source_branch_id: 9007199254740992n,
        target_branch_id: currentBranch.id,
      }),
      /exceeds the safe integer range supported by merge preview requests/,
    );
  } finally {
    await cleanup();
  }
});

test("wrapSignals adapters wrapper marks same-runtime exact restore while preserving portable host-capability denial artifacts", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const rawEnvelope = {
      definitions: {
        policy: { preset: "WebDevelopment" },
        sources: [],
        recipes: [],
        sourceFamilies: [],
        recipeFamilies: [],
        unavailableCallbacks: [{
          id: "visibleLabel",
          signalKind: "computed",
          reason: "computeCallbackUnavailableForPortableExport",
          currentReads: ["count"],
          hostCapabilityReads: [{
            family: "visibility",
            registrationId: "visibility",
            compatibility: "LiveOnly",
          }],
          hostCapabilityTransports: [{
            family: "visibility",
            registrationId: "visibility",
            compatibility: "LiveOnly",
            exactRestoreOutcome: "Live",
            portableImportOutcome: "Denied",
            portableImportReason: "live-only host capabilities require the exact originating runtime",
          }],
        }],
      },
      snapshot: {
        snapshot: { meta: { branch_id: 0 } },
        state: { sources: [], recipes: [] },
      },
    };
    const calls = [];
    const rawSignals = {
      input(id, initial) {
        return createRawReadableHandle(id, initial);
      },
      computedSpec(id, spec) {
        return createRawReadableHandle(id, spec);
      },
      computedCallback(id) {
        return createRawReadableHandle(id, id);
      },
      outputSpec(id, spec) {
        return createRawReadableHandle(id, spec);
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
          why() { return null; },
          health() { return null; },
          summaryNow() { return null; },
          historyNow() { return null; },
          latestObservation() { return null; },
          latestFlow() { return null; },
          performanceSummary() { return {}; },
          latestFailure() { return null; },
          latestRollback() { return null; },
          latestFrontierExecution() { return null; },
          latestInvalidationTraceRecords() { return []; },
          recentHistory() { return []; },
          subscribe() { return { free() {} }; },
        };
      },
      history() {
        return { free() {} };
      },
      specialist() {
        return {};
      },
      adapters() {
        return {
          export_definitions() {
            return rawEnvelope.definitions;
          },
          export_runtime_envelope() {
            return structuredClone(rawEnvelope);
          },
          export_runtime_envelope_wire() {
            return "restore-token";
          },
          export_runtime_envelope_portable_wire() {
            return "{\"portable\":true}";
          },
          replace_runtime_envelope(envelope) {
            calls.push(["replace_runtime_envelope", envelope.definitions.unavailableCallbacks[0].id]);
          },
          replace_runtime_envelope_portable_wire(envelope) {
            calls.push(["replace_runtime_envelope_portable_wire", envelope]);
          },
          replace_runtime_envelope_wire(token) {
            calls.push(["replace_runtime_envelope_wire", token]);
          },
          runtime_proof_report() {
            return { proofSchemaVersion: "1" };
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

    const signals = wrapSignals(rawSignals);
    const adapters = signals.adapters();
    const diagnostics = signals.diagnostics();
    const envelope = adapters.exportRuntimeEnvelope();
    const secondEnvelope = adapters.exportRuntimeEnvelope();
    const transportReport = adapters.hostCapabilityTransportReport(envelope);
    const implicitTransportReport = adapters.hostCapabilityTransportReport();

    assert.equal(envelope.runtimeEnvelopeRestoreToken, "restore-token");
    assert.equal(secondEnvelope.runtimeEnvelopeRestoreToken, "restore-token");
    assert.equal(envelope.runtimeEnvelopeRestoreMode, "SameRuntimeExact");
    assert.equal(envelope.runtimeEnvelopePortableWire, "{\"portable\":true}");
    assert.equal(
      envelope.definitions.unavailableCallbacks[0].hostCapabilityTransports[0].portableImportOutcome,
      "Denied",
    );
    assert.equal(
      envelope.definitions.unavailableCallbacks[0].hostCapabilityTransports[0].exactRestoreOutcome,
      "Live",
    );
    assert.equal(typeof transportReport.digest, "string");
    assert.equal(transportReport.totals.unavailableArtifactCount, 1);
    assert.deepEqual(transportReport.families[0]?.deniedCallbackIds, ["visibleLabel"]);
    assert.equal(typeof implicitTransportReport.digest, "string");
    assert.equal(implicitTransportReport.totals.unavailableArtifactCount, 1);

    assert.throws(
      () => adapters.replaceRuntimeEnvelope(envelope),
      (error) => error?.code === "computeCallbackUnavailableForRuntimeEnvelopeImport" &&
        error?.message === "runtime envelope import cannot restore callback-backed nodes without live callback registrations: visibleLabel",
    );
    assert.deepEqual(diagnostics.latestHostCapabilityEvent(), {
      sequence: 1,
      kind: "PortableImportDenied",
      family: "visibility",
      registrationId: "visibility",
      compatibility: "LiveOnly",
      invalidationMode: null,
      queuedInvalidationCount: 0,
      previousState: null,
      nextState: null,
      touchedNodes: 0,
      reevaluatedNodes: 0,
      portableImportOutcome: "Denied",
      portableImportReason: "live-only host capabilities require the exact originating runtime",
      deniedCallbackIds: ["visibleLabel"],
    });
    assert.equal(diagnostics.performanceSummary().hostCapabilityCompatibilityDenialCount, 1);
    assert.equal(diagnostics.performanceSummary().hostCapabilityUnavailabilityArtifactCount, 1);
    assert.equal(diagnostics.hostCapabilityReport().totals.compatibilityDenialCount, 1);
    assert.equal(typeof diagnostics.hostCapabilityReport().lineageDigest, "string");
    assert.equal(typeof diagnostics.hostCapabilityReport().breadthDigest, "string");
    adapters.restoreExactRuntimeEnvelope(envelope);

    assert.deepEqual(calls, [["replace_runtime_envelope_wire", "restore-token"]]);
    assert.throws(
      () => adapters.restoreExactRuntimeEnvelope(rawEnvelope),
      /adapters\.restoreExactRuntimeEnvelope expects an artifact returned by adapters\.exportRuntimeEnvelope\(\)/,
    );
  } finally {
    await cleanup();
  }
});
