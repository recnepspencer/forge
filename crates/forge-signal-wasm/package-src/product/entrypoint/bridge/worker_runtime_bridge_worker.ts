import init, { SignalWorkerRuntime } from "../../../raw_surface.js";
import { createWorkerRuntimeMirror } from "./worker_runtime_bridge_worker_mirror.js";

await init();

const runtime = new SignalWorkerRuntime();
const branchState = createWorkerBranchState();
const mirror = createWorkerRuntimeMirror();
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
      return resolveBranchSnapshotId(args[0]);
    case "createBranch":
      return resolveCreateBranch(runtime, args[0]);
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
  const result = runtime.publishPortableGraph(publication);
  mirror.publishPortableGraph(publication);
  return result;
}

function resolveApplyTransaction(runtime, transactionOps) {
  const result = runtime.applyTransaction(transactionOps);
  mirror.applyTransaction(transactionOps);
  return result;
}

function resolveHostCapabilityIngress(runtime, batch) {
  const result = runtime.admitHostCapabilityIngress(batch);
  mirror.admitHostCapabilityIngress(batch);
  return result;
}

function resolveBrowserHistoryIngress(runtime, ingress) {
  const result = runtime.admitBrowserHistoryIngress(ingress);
  mirror.admitBrowserHistoryIngress(ingress);
  return result;
}

function resolveReadSignals(runtime, request) {
  if (typeof runtime.readSignals === "function") {
    return runtime.readSignals(request);
  }
  return mirror.readSignals(request);
}

function createWorkerBranchState() {
  const rootBranch = {
    id: 1,
    name: "main",
    parent_branch_id: null,
    head_snapshot_id: 1,
  };
  return {
    currentBranchId: rootBranch.id,
    nextBranchId: 2,
    branches: new Map([[rootBranch.id, rootBranch]]),
    snapshotIds: new Map([[rootBranch.id, 1]]),
  };
}

function resolveCurrentBranch(runtime) {
  if (typeof runtime.currentBranch === "function") {
    return runtime.currentBranch();
  }
  return cloneBranchHandle(branchState.branches.get(branchState.currentBranchId) ?? null);
}

function resolveBranches(runtime) {
  if (typeof runtime.branches === "function") {
    return runtime.branches();
  }
  return [...branchState.branches.values()].map(cloneBranchHandle);
}

function resolveBranchSnapshotId(branchId) {
  return branchState.snapshotIds.get(Number(branchId)) ?? 1;
}

function resolveCreateBranch(runtime, name) {
  const runtimeBranch =
    typeof runtime.createBranch === "function"
      ? runtime.createBranch(name)
      : typeof runtime.createWorkerBranch === "function"
        ? runtime.createWorkerBranch(name)
        : null;
  const normalizedBranch = normalizeBranchHandle(
    runtimeBranch,
    name,
    branchState.currentBranchId,
    branchState.nextBranchId,
  );
  branchState.branches.set(normalizedBranch.id, normalizedBranch);
  branchState.snapshotIds.set(normalizedBranch.id, normalizedBranch.head_snapshot_id ?? 1);
  branchState.nextBranchId = Math.max(branchState.nextBranchId, normalizedBranch.id + 1);
  return cloneBranchHandle(normalizedBranch);
}

function resolveSwitchBranch(runtime, branchId) {
  if (typeof runtime.switchBranch === "function") {
    runtime.switchBranch(branchId);
  } else if (typeof runtime.switchWorkerBranch === "function") {
    runtime.switchWorkerBranch(branchId);
  }
  const normalizedId = Number(branchId);
  if (!branchState.branches.has(normalizedId)) {
    throw new TypeError(`unknown worker branch ${branchId}`);
  }
  branchState.currentBranchId = normalizedId;
  return undefined;
}

function normalizeBranchHandle(branch, fallbackName, parentBranchId, fallbackId) {
  if (branch && typeof branch === "object") {
    return {
      id: Number(branch.id),
      name: typeof branch.name === "string" ? branch.name : fallbackName,
      parent_branch_id:
        branch.parent_branch_id === null || branch.parent_branch_id === undefined
          ? parentBranchId ?? null
          : Number(branch.parent_branch_id),
      head_snapshot_id:
        branch.head_snapshot_id === null || branch.head_snapshot_id === undefined
          ? 1
          : Number(branch.head_snapshot_id),
    };
  }
  return {
    id: fallbackId,
    name: fallbackName,
    parent_branch_id: parentBranchId ?? null,
    head_snapshot_id: 1,
  };
}

function cloneBranchHandle(branch) {
  if (branch === null) {
    return null;
  }
  return {
    id: branch.id,
    name: branch.name,
    parent_branch_id: branch.parent_branch_id,
    head_snapshot_id: branch.head_snapshot_id,
  };
}

async function resolveWorkerPort() {
  const nodePort = await resolveNodeWorkerPort();
  if (nodePort) {
    return nodePort;
  }
  return {
    listen(handler) {
      globalThis.onmessage = (event) => {
        handler(event.data);
      };
    },
    postMessage(message) {
      globalThis.postMessage(message);
    },
  };
}

async function resolveNodeWorkerPort() {
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
