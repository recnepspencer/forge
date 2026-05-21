import init, { SignalWorkerRuntime } from "../../../raw_surface.js";

await init();

const runtime = new SignalWorkerRuntime();
const port = await resolveWorkerPort();

port.listen(async (message) => {
  if (!message || typeof message !== "object") {
    return;
  }
  const { id, method, args = [] } = message;
  try {
    const value = runtime[method](...args);
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
