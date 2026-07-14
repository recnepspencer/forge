import assert from "node:assert/strict";

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
      return (expr.args ?? []).reduce(
        (acc, nextExpr) => acc + Number(evaluateExpr(nextExpr, values) ?? 0),
        0,
      );
    case "multiply":
      return (expr.args ?? []).reduce(
        (acc, nextExpr) => acc * Number(evaluateExpr(nextExpr, values) ?? 1),
        1,
      );
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
      capture?.__WorthSignalCallbackCapture,
      true,
      `computed callback ${node.id} must return a Worth callback capture artifact`,
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
      let reevaluatedNodes = 0;
      for (const node of computedNodes.values()) {
        if (![...node.deps].some((depId) => frontier.has(depId))) {
          continue;
        }
        touchedComputedNodes.add(node.id);
        if (recomputeNode(node)) {
          values.set(node.id, node.value);
          frontier.add(node.id);
          reevaluatedNodes += 1;
        }
      }

      calls.push(["transaction", pendingSets]);
      for (const id of frontier) {
        emitSignalWatch(id);
      }

      latestObservation = frontier.size === 0
        ? latestObservation
        : { observation: { frontier: [...frontier].sort() }, callbackNodes: buildCallbackNodes() };
      latestFlow = {
        callbackNodes: buildCallbackNodes(),
        frontier: [...frontier].sort(),
        touchedComputedNodes: [...touchedComputedNodes].sort(),
        reevaluatedNodes,
      };
      emitDiagnostics();
      return {
        touchedNodes: frontier.size,
        reevaluatedNodes,
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
        why() {
          return null;
        },
        health() {
          return null;
        },
        summaryNow() {
          return { profile: "Development" };
        },
        historyNow() {
          return { history: {}, callbackNodes: [] };
        },
        latestObservation() {
          return latestObservation;
        },
        latestFlow() {
          return latestFlow;
        },
        performanceSummary() {
          return { activeHandleCount: 0 };
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
      return {};
    },
    adapters() {
      return options.adaptersFactory ? options.adaptersFactory() : {};
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
    calls,
    values,
  };
}
