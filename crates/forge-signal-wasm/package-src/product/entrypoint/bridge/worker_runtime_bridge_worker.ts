import init, { SignalWorkerRuntime } from "../../../raw_surface.js";

const earlyBrowserMessages = [];
let browserMessageHandler = null;

if (typeof globalThis.addEventListener === "function") {
  globalThis.addEventListener("message", (event) => {
    if (browserMessageHandler) {
      browserMessageHandler(event.data);
      return;
    }
    earlyBrowserMessages.push(event.data);
  });
}

await init();

const runtime = new SignalWorkerRuntime();
const port = await resolveWorkerPort();

port.listen(async (message) => {
  if (!message || typeof message !== "object") {
    return;
  }
  const { id, method, args = [] } = message;
  try {
    const value = resolveRuntimeMethod(runtime, method, args);
    port.postMessage({
      id,
      ok: true,
      value: await Promise.resolve(value),
    });
  } catch (error) {
    port.postMessage({
      id,
      ok: false,
      error: serializeError(error),
    });
  }
});

function serializeError(error) {
  if (error instanceof Error) {
    return {
      name: error.name,
      message: error.message,
      stack: error.stack ?? null,
    };
  }
  if (error && typeof error === "object") {
    return {
      name: typeof error.name === "string" ? error.name : "WorkerRuntimeBridgeError",
      message: typeof error.message === "string" ? error.message : JSON.stringify(error),
      code: typeof error.code === "string" ? error.code : null,
      stack: null,
    };
  }
  return {
    name: "WorkerRuntimeBridgeError",
    message: String(error),
    stack: null,
  };
}

function resolveRuntimeMethod(runtime, method, args) {
  switch (method) {
    case "publishPortableGraph":
      return resolvePublishPortableGraph(runtime, args[0]);
    case "applyTransaction":
      return resolveApplyTransaction(runtime, args[0]);
    case "admitHostCapabilityIngress":
      return resolveHostCapabilityIngress(runtime, args[0]);
    case "admitBrowserHistoryIngress":
      return resolveBrowserHistoryIngress(runtime, args[0]);
    case "readSignals":
      return resolveReadSignals(runtime, args[0]);
    case "currentBranch":
      return resolveCurrentBranch(runtime);
    case "branches":
      return resolveBranches(runtime);
    case "branchSnapshotId":
      return invokeRequiredRuntimeMethod(runtime, "branchSnapshotId", args);
    case "createBranch":
      return resolveCreateBranch(runtime, args[0]);
    case "workerBranchBasis":
    case "forkBranch":
    case "applyTransactionToBranch":
    case "retireBranch":
    case "retireBranches":
    case "closeoutEffectBranch":
      return invokeRequiredRuntimeMethod(runtime, method, args);
    case "switchBranch":
      return resolveSwitchBranch(runtime, args[0]);
    default:
      if (typeof runtime[method] === "function") {
        return runtime[method](...args);
      }
      throw new TypeError(`unsupported worker runtime method ${method}`);
  }
}

function resolvePublishPortableGraph(runtime, publication) {
  return invokeRequiredRuntimeMethod(runtime, "publishPortableGraph", [publication]);
}

function resolveApplyTransaction(runtime, transactionOps) {
  return invokeRequiredRuntimeMethod(runtime, "applyTransaction", [transactionOps]);
}

function resolveHostCapabilityIngress(runtime, batch) {
  return invokeRequiredRuntimeMethod(runtime, "admitHostCapabilityIngress", [batch]);
}

function resolveBrowserHistoryIngress(runtime, ingress) {
  return invokeRequiredRuntimeMethod(runtime, "admitBrowserHistoryIngress", [ingress]);
}

function resolveReadSignals(runtime, request) {
  return invokeRequiredRuntimeMethod(runtime, "readSignals", [request]);
}

function resolveCurrentBranch(runtime) {
  return invokeRequiredRuntimeMethod(runtime, "currentBranch", []);
}

function resolveBranches(runtime) {
  return invokeRequiredRuntimeMethod(runtime, "branches", []);
}

function resolveCreateBranch(runtime, name) {
  return invokeRequiredRuntimeMethod(runtime, "createBranch", [name]);
}

function resolveSwitchBranch(runtime, branchId) {
  return invokeRequiredRuntimeMethod(runtime, "switchBranch", [branchId]);
}

function invokeRequiredRuntimeMethod(runtime, method, args) {
  if (typeof runtime[method] !== "function") {
    throw new TypeError(
      `worker runtime method ${method} is unavailable; worker-first execution does not fall back to JavaScript authority`,
    );
  }
  return runtime[method](...args);
}

async function resolveWorkerPort() {
  const nodePort = await resolveNodeWorkerPort();
  if (nodePort) {
    return nodePort;
  }
  return {
    listen(handler) {
      browserMessageHandler = handler;
      for (const message of earlyBrowserMessages.splice(0)) {
        handler(message);
      }
    },
    postMessage(message) {
      globalThis.postMessage(message);
    },
  };
}

async function resolveNodeWorkerPort() {
  if (typeof globalThis.process !== "object") {
    return null;
  }
  try {
    const workerThreads = await import("node:worker_threads");
    if (!workerThreads.parentPort) {
      return null;
    }
    return {
      listen(handler) {
        workerThreads.parentPort.on("message", handler);
      },
      postMessage(message) {
        workerThreads.parentPort.postMessage(message);
      },
    };
  } catch {
    return null;
  }
}
