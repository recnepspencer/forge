/**
 * Track 5 bundle policy admission: stable chunk names, colocated bridge/worker,
 * and package-root relative externals for wasm glue.
 */
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { describe, it } from "node:test";
import { fileURLToPath } from "node:url";

import {
  BUNDLED_JS_FILE_CAP,
  BRIDGE_ENTRY_RELATIVE_PATH,
  CHUNK_NAME_PATTERN,
  WORKER_ENTRY_RELATIVE_PATH,
} from "./bundle_worth_signals_wasm_entries.mjs";

const scriptDir = path.dirname(fileURLToPath(import.meta.url));

describe("worth-signals-wasm Track 5 bundle policy", () => {
  it("pins stable chunk naming and colocated bridge/worker outfiles", () => {
    assert.equal(CHUNK_NAME_PATTERN, "chunks/[name]");
    assert.equal(
      WORKER_ENTRY_RELATIVE_PATH,
      "product/entrypoint/bridge/worker_runtime_bridge_worker.js",
    );
    assert.equal(
      BRIDGE_ENTRY_RELATIVE_PATH,
      "product/entrypoint/bridge/worker_runtime_bridge.js",
    );
    assert.ok(BUNDLED_JS_FILE_CAP <= 40);
  });

  it("keeps facade emit free of content-hashed chunkNames and of unsafe splitting", async () => {
    const bundleSource = await readFile(
      path.join(scriptDir, "bundle-worth-signals-wasm-product.mjs"),
      "utf8",
    );
    assert.match(bundleSource, /chunkNames:\s*CHUNK_NAME_PATTERN/u);
    assert.doesNotMatch(bundleSource, /chunk-\[hash\]/u);
    assert.doesNotMatch(bundleSource, /chunks\/chunk-\[hash\]/u);
    // Facade splitting would break package-root-relative bridge/wasm externals.
    assert.match(
      bundleSource,
      /splitting:\s*false[\s\S]*chunkNames:\s*CHUNK_NAME_PATTERN/u,
    );
  });
});
