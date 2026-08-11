import assert from "node:assert/strict";
import http from "node:http";
import test from "node:test";
import { createReadStream, existsSync, statSync } from "node:fs";
import path from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadStoreModule } from "../../../host_capabilities_certification/module_loading/load_store_module.mjs";
import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

const moduleDir = path.dirname(fileURLToPath(import.meta.url));
const pkgDir = path.resolve(moduleDir, "../../../../../pkg");
const wasmPath = path.join(pkgDir, "worth_signal_wasm_bg.wasm");
const workerPath = path.join(
  pkgDir,
  "product",
  "entrypoint",
  "bridge",
  "worker_runtime_bridge_worker.js",
);

async function serveWasmAsset() {
  const server = http.createServer((request, response) => {
    if ((request.url ?? "/") !== "/wasm" || !existsSync(wasmPath)) {
      response.writeHead(404);
      response.end();
      return;
    }
    response.writeHead(200, {
      "content-type": "application/wasm",
      "content-length": statSync(wasmPath).size,
    });
    createReadStream(wasmPath).pipe(response);
  });
  await new Promise((resolve) => server.listen(0, "127.0.0.1", resolve));
  const { port } = server.address();
  return {
    wasmUrl: new URL(`http://127.0.0.1:${port}/wasm`),
    async close() {
      await new Promise((resolve, reject) => {
        server.close((error) => (error ? reject(error) : resolve()));
      });
    },
  };
}

test("worker-first assets URLs + createReactSignalsStore attach on real pkg binaries", async (t) => {
  if (!existsSync(wasmPath) || !existsSync(workerPath)) {
    t.skip(
      "pkg wasm + product worker bridge must exist (run package prepare / publish dry-run)",
    );
    return;
  }

  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const wasmServer = await serveWasmAsset();
  const { createSignals, cleanup: cleanupSignals } = await loadSignalsModule({
    rawSurface: "real",
  });
  const { createReactSignalsStore, cleanup: cleanupStore } = await loadStoreModule();
  let signals = null;
  try {
    signals = await createSignals({
      deployment: "workerFirst",
      assets: {
        wasmUrl: wasmServer.wasmUrl,
        // Worker must keep a real module URL so its relative imports resolve.
        workerUrl: pathToFileURL(workerPath),
      },
    });
    const store = createReactSignalsStore(signals);
    const quantity = signals.input(4, { debugName: "assets.react.quantity" });
    assert.equal(store.getSignalSnapshot(quantity), 4);
    store.dispose();
  } finally {
    if (signals) {
      await signals.terminate();
    }
    await cleanupStore();
    await cleanupSignals();
    await wasmServer.close();
    globalThis.Worker = previousWorker;
  }
});
