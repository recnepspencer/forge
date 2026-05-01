import assert from "node:assert/strict";
import { mkdir, mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { stripTypeScriptTypes } from "node:module";
import { tmpdir } from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";

const productDir = path.dirname(fileURLToPath(import.meta.url));
const packageDir = path.dirname(productDir);
const packageSourceDir = path.join(packageDir, "..", "package-src");
const reactDir = path.join(packageDir, "..", "react");

export function flushMicrotasks() {
  return new Promise((resolve) => queueMicrotask(resolve));
}

export function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

function stableClone(value) {
  if (Array.isArray(value)) {
    return value.map(stableClone);
  }
  if (value && typeof value === "object") {
    return Object.keys(value)
      .sort()
      .reduce((acc, key) => {
        acc[key] = stableClone(value[key]);
        return acc;
      }, {});
  }
  return value;
}

export function digestValue(value) {
  const serialized = JSON.stringify(stableClone(value));
  let hash = 2166136261;
  for (let index = 0; index < serialized.length; index += 1) {
    hash ^= serialized.charCodeAt(index);
    hash = Math.imul(hash, 16777619);
  }
  return `f1a-${(hash >>> 0).toString(16).padStart(8, "0")}`;
}

export async function loadSignalsModule() {
  const tempDir = await mkdtemp(path.join(tmpdir(), "forge-signal-host-cert-"));
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
      await writeFile(targetPath, await readFile(sourcePath, "utf8"), "utf8");
    }

    await writeFile(
      path.join(tempDir, "raw_surface.js"),
      "export function createRawSignals() { throw new Error('createRawSignals should not be used in host capability certification tests'); }\n",
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

export async function loadStoreModule() {
  const tempDir = await mkdtemp(path.join(tmpdir(), "forge-signal-react-store-"));
  const sourceFiles = [
    ["model.ts", "model.js"],
    ["store.ts", "store.js"],
  ];
  try {
    for (const [sourceName, outputName] of sourceFiles) {
      const sourcePath = path.join(reactDir, sourceName);
      const source = await readFile(sourcePath, "utf8");
      const transformed = stripTypeScriptTypes(source, { mode: "transform" });
      await writeFile(path.join(tempDir, outputName), transformed, "utf8");
    }
    const moduleUrl = new URL(`file:///${path.join(tempDir, "store.js").replace(/\\/g, "/")}`);
    const loaded = await import(moduleUrl.href);
    return { ...loaded, cleanup: () => rm(tempDir, { recursive: true, force: true }) };
  } catch (error) {
    await rm(tempDir, { recursive: true, force: true });
    throw error;
  }
}

function createReadableHandle(id, reader) {
  return {
    id,
    get: reader,
    peek: reader,
    free() {},
  };
}

function evaluateExpr(expr, values) {
  switch (expr?.kind) {
    case "read":
      return values.get(expr.id);
    case "value":
      return expr.value;
    case "object":
      return Object.fromEntries(
        (expr.fields ?? []).map(([key, nextExpr]) => [key, evaluateExpr(nextExpr, values)]),
      );
    case "add":
      return (expr.args ?? []).reduce((acc, nextExpr) => acc + Number(evaluateExpr(nextExpr, values) ?? 0), 0);
    case "multiply":
      return (expr.args ?? []).reduce((acc, nextExpr) => acc * Number(evaluateExpr(nextExpr, values) ?? 1), 1);
    default:
      return undefined;
  }
}

export function createReactiveRawSignals(options = {}) {
  const values = new Map();
  const computedNodes = new Map();
  const signalSubscribers = new Map();
  const diagnosticsSubscribers = new Set();
  const calls = [];
  let latestObservation = null;
  let latestFlow = { callbackNodes: [] };

  function emitSignalWatch(id) {
    const callbacks = signalSubscribers.get(id);
    if (!callbacks) {
      return 0;
    }
    for (const callback of callbacks) {
      callback({
        triggerMatched: true,
        meaningfulChange: true,
        touched: [id],
        recomputed: true,
      });
    }
    return callbacks.size;
  }

  function emitDiagnostics() {
    for (const callback of diagnosticsSubscribers) {
      callback();
    }
  }

  function buildCallbackNodes() {
    return [...computedNodes.values()].map((node) => ({
      id: node.id,
      currentReads: [...node.deps].sort(),
      hostCapabilityReads: [...node.hostCapabilityReads],
    }));
  }

  function recomputeNode(node) {
    const capture = node.callback();
    assert.equal(
      capture?.__forgeSignalCallbackCapture,
      true,
      `computed callback ${node.id} must return a Forge callback capture artifact`,
    );
    const previousValue = node.value;
    node.value = capture.value;
    node.deps = new Set(capture.reads ?? []);
    node.hostCapabilityReads = [...(capture.hostCapabilityReads ?? [])];
    return previousValue !== node.value;
  }

  const rawSignals = {
    input(id, initial) {
      values.set(id, initial);
      calls.push(["input", id, initial]);
      return createReadableHandle(id, () => values.get(id));
    },
    computedSpec(id, spec) {
      calls.push(["computedSpec", id, spec]);
      return createReadableHandle(id, () => evaluateExpr(spec?.expr ?? spec, values));
    },
    computedCallback(id, callback) {
      calls.push(["computedCallback", id, "registered"]);
      const node = {
        id,
        callback,
        value: undefined,
        deps: new Set(),
        hostCapabilityReads: [],
      };
      computedNodes.set(id, node);
      recomputeNode(node);
      values.set(id, node.value);
      return createReadableHandle(id, () => node.value);
    },
    outputSpec(id, spec) {
      calls.push(["outputSpec", id, spec]);
      return createReadableHandle(id, () => evaluateExpr(spec?.expr ?? spec, values));
    },
    read(target) {
      return typeof target === "string" ? values.get(target) : target.get();
    },
    watch(target, callback) {
      const id = typeof target === "string" ? target : target.id;
      const callbacks = signalSubscribers.get(id) ?? new Set();
      callbacks.add(callback);
      signalSubscribers.set(id, callbacks);
      return {
        free() {
          callbacks.delete(callback);
          if (callbacks.size === 0) {
            signalSubscribers.delete(id);
          }
        },
      };
    },
    effect(target, callback) {
      return this.watch(target, callback);
    },
    transaction(callback) {
      const pendingSets = [];
      callback({
        set(target, value) {
          pendingSets.push([target.id, value]);
        },
        free() {},
      });

      const frontier = new Set();
      for (const [id, nextValue] of pendingSets) {
        if (values.get(id) !== nextValue) {
          values.set(id, nextValue);
          frontier.add(id);
        }
      }

      const touchedComputedNodes = new Set();
      const changedComputedNodes = new Set();
      let currentFrontier = [...frontier];
      while (currentFrontier.length > 0) {
        const nextFrontier = [];
        for (const node of computedNodes.values()) {
          if (!currentFrontier.some((id) => node.deps.has(id))) {
            continue;
          }
          touchedComputedNodes.add(node.id);
          const changed = recomputeNode(node);
          values.set(node.id, node.value);
          if (changed) {
            changedComputedNodes.add(node.id);
            nextFrontier.push(node.id);
          }
        }
        currentFrontier = nextFrontier;
      }

      latestFlow = {
        callbackNodes: buildCallbackNodes(),
      };
      let deliveredEventCount = 0;
      for (const id of [...frontier, ...changedComputedNodes]) {
        deliveredEventCount += emitSignalWatch(id);
      }
      latestObservation = {
        observation: {
          delivered_event_count: deliveredEventCount,
        },
        callbackNodes: buildCallbackNodes(),
      };
      emitDiagnostics();
      return {
        touchedNodes: touchedComputedNodes.size,
        nodesRecomputed: changedComputedNodes.size,
      };
    },
    batch(callback) {
      return this.transaction(callback);
    },
    nuke() {
      return true;
    },
    diagnostics() {
      return {
        latestObservation() {
          return latestObservation;
        },
        latestFlow() {
          return latestFlow;
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
        historyNow() {
          return { history: {}, callbackNodes: buildCallbackNodes() };
        },
        why(id) {
          return buildCallbackNodes().find((node) => node.id === id) ?? null;
        },
        health() {
          return null;
        },
        summaryNow() {
          return { profile: "Development" };
        },
        performanceSummary() {
          return {
            activeHandleCount: values.size,
          };
        },
        subscribe(callback) {
          diagnosticsSubscribers.add(callback);
          return {
            free() {
              diagnosticsSubscribers.delete(callback);
            },
          };
        },
        free() {},
      };
    },
    history() {
      return {};
    },
    specialist() {
      return {
        graphSummary() {
          return { profile: "Development" };
        },
        evaluateDirty() {
          return { touchedNodes: 0 };
        },
      };
    },
    adapters() {
      return typeof options.adaptersFactory === "function"
        ? options.adaptersFactory({ values, computedNodes })
        : {};
    },
    compatibilityApp() {
      return {};
    },
    compatibilityRuntime() {
      return {};
    },
    free() {},
  };

  return {
    rawSignals,
    values,
    calls,
    callbackNodes() {
      return buildCallbackNodes();
    },
  };
}
