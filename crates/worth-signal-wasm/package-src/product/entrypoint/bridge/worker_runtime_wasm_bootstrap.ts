export const WORKER_RUNTIME_WASM_BOOTSTRAP = "workerRuntimeWasmBootstrap";

export function createWorkerRuntimeWasmBootstrapMessage(wasmUrl) {
  return Object.freeze({
    artifactFamily: WORKER_RUNTIME_WASM_BOOTSTRAP,
    wasmUrl: wasmUrl == null ? null : stringifyAssetUrl(wasmUrl),
  });
}

export function isWorkerRuntimeWasmBootstrapMessage(message) {
  return Boolean(
    message &&
      typeof message === "object" &&
      message.artifactFamily === WORKER_RUNTIME_WASM_BOOTSTRAP,
  );
}

export function stringifyAssetUrl(value) {
  if (value instanceof URL) {
    return value.href;
  }
  if (typeof value === "string") {
    return value;
  }
  throw new TypeError("wasmUrl must be a string or URL");
}
