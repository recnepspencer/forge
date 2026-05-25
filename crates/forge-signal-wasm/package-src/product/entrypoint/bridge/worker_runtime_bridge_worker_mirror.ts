function createWorkerRuntimeMirror() {
  let truthRevision = 0;
  let publication = null;
  const sources = new Map();
  const recipes = new Map();

  return Object.freeze({
    publishPortableGraph(nextPublication) {
      publication = nextPublication;
      sources.clear();
      recipes.clear();
      for (const source of nextPublication.sources ?? []) {
        sources.set(source.id, cloneValue(source.initial));
      }
      for (const recipe of nextPublication.recipes ?? []) {
        recipes.set(recipe.id, recipe);
      }
      truthRevision += 1;
    },
    applyTransaction(transactionOps) {
      let changed = false;
      for (const op of transactionOps) {
        if (!op || typeof op !== "object") {
          continue;
        }
        if (op.kind === "set" && typeof op.id === "string") {
          sources.set(op.id, cloneValue(op.value));
          changed = true;
        }
      }
      if (changed) {
        truthRevision += 1;
      }
    },
    admitHostCapabilityIngress(batch) {
      let changed = false;
      for (const update of batch?.updates ?? []) {
        if (
          update
          && typeof update.runtimeSourceId === "string"
          && "runtimeValue" in update
        ) {
          sources.set(update.runtimeSourceId, cloneValue(update.runtimeValue));
          changed = true;
        }
      }
      if (changed) {
        truthRevision += 1;
      }
    },
    admitBrowserHistoryIngress(ingress) {
      let changed = false;
      if (typeof ingress?.runtimeRouteSourceId === "string" && "routeValue" in ingress) {
        sources.set(ingress.runtimeRouteSourceId, cloneValue(ingress.routeValue));
        changed = true;
      }
      if (
        typeof ingress?.runtimeContinuitySourceId === "string"
        && "continuityValue" in ingress
      ) {
        sources.set(
          ingress.runtimeContinuitySourceId,
          cloneValue(ingress.continuityValue),
        );
        changed = true;
      }
      if (changed) {
        truthRevision += 1;
      }
    },
    readSignals(request) {
      if (!publication) {
        throw new TypeError(
          "worker runtime bridge readSignals(...) requires publishPortableGraph(...) first",
        );
      }
      const signalIds = normalizeSignalIds(request?.signalIds);
      const signals = signalIds.map((id) => {
        const value = evaluateSignal(id, sources, recipes);
        return Object.freeze({
          id,
          value,
          payloadByteCount: measurePayloadBytes(value),
        });
      });
      const signalPayloadByteCount = signals.reduce(
        (sum, signal) => sum + signal.payloadByteCount,
        0,
      );
      return Object.freeze({
        envelopeFamily: "signalReadback",
        readbackMode: "CommittedSignalReadback",
        runtimeAuthority: "workerOwnedRuntime",
        signalReadbackPacketCount: 1,
        signalReadbackBreadth: signals.length,
        signalPayloadByteCount,
        workerFirstTruthDigest: `worker-first-truth:${truthRevision}`,
        signalDigest: createMirrorDigest("signal-readback", {
          truthRevision,
          signalIds,
          signalPayloadByteCount,
        }),
        boundaryPerformance: Object.freeze({
          totalNanos: "0",
          evaluationNanos: "0",
          commitNanos: "0",
        }),
        packetDigest: createMirrorDigest("signal-readback-packet", {
          truthRevision,
          signals: signals.map((signal) => ({
            id: signal.id,
            payloadByteCount: signal.payloadByteCount,
          })),
        }),
        signals: Object.freeze(signals),
      });
    },
  });
}

function normalizeSignalIds(signalIds) {
  if (!Array.isArray(signalIds)) {
    throw new TypeError("worker runtime bridge readSignals(...) requires signalIds as an array");
  }
  const seen = new Set();
  return signalIds.map((id, index) => {
    if (typeof id !== "string" || id.trim().length === 0) {
      throw new TypeError(`worker runtime bridge readSignals(...) signalIds[${index}] must be a non-empty string`);
    }
    if (seen.has(id)) {
      throw new TypeError(`worker runtime bridge readSignals(...) rejects duplicate signal id \`${id}\``);
    }
    seen.add(id);
    return id;
  });
}

function evaluateSignal(id, sources, recipes) {
  if (sources.has(id)) {
    return cloneValue(sources.get(id));
  }
  const recipe = recipes.get(id);
  if (!recipe) {
    throw new TypeError(`worker runtime bridge mirror has no committed signal \`${id}\``);
  }
  return evaluateExpr(recipe.expr, sources, recipes);
}

function evaluateExpr(expr, sources, recipes) {
  if (!expr || typeof expr !== "object") {
    throw new TypeError("worker runtime bridge mirror encountered a missing portable expr");
  }
  switch (expr.kind) {
    case "read":
      return evaluateSignal(expr.id, sources, recipes);
    case "sum":
      return (expr.args ?? []).reduce(
        (sum, arg) => sum + Number(evaluateExpr(arg, sources, recipes)),
        0,
      );
    default:
      throw new TypeError(
        `worker runtime bridge mirror does not support portable expr kind \`${expr.kind}\``,
      );
  }
}

function cloneValue(value) {
  if (typeof globalThis.structuredClone === "function") {
    try {
      return globalThis.structuredClone(value);
    } catch {}
  }
  if (Array.isArray(value)) {
    return value.slice();
  }
  if (value && typeof value === "object") {
    return { ...value };
  }
  return value;
}

function measurePayloadBytes(value) {
  return new TextEncoder().encode(JSON.stringify(value)).length;
}

function createMirrorDigest(label, value) {
  return `forge-worker-bridge:${label}:${JSON.stringify(value)}`;
}

export {
  createWorkerRuntimeMirror,
};
