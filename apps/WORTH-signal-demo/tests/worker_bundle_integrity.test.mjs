import assert from "node:assert/strict";
import test from "node:test";

import { assertWorkerBundleIntegrity } from "../build/worker_bundle_integrity_plugin.ts";

test("production rejects a worker copied without its dependency graph", () => {
  assert.throws(
    () => assertWorkerBundleIntegrity({
      "assets/worker_runtime_bridge_worker.js": {
        fileName: "assets/worker_runtime_bridge_worker.js",
        source: 'import init from "../../../raw_surface.js";',
        type: "asset",
      },
    }),
    /references missing output .*raw_surface\.js/u,
  );
});

test("production accepts a bundled worker whose emitted dependencies exist", () => {
  assert.doesNotThrow(() => assertWorkerBundleIntegrity({
    "assets/worker_runtime_bridge_worker.js": {
      fileName: "assets/worker_runtime_bridge_worker.js",
      source: 'import "./worker_support.js";',
      type: "asset",
    },
    "assets/worker_support.js": {
      fileName: "assets/worker_support.js",
      source: "",
      type: "asset",
    },
  }));
});
