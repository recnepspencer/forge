import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

import {
  BASELINE_WASM_BYTES,
  MAX_WASM_BYTES,
  WASM_BINARY_RELATIVE_PATH,
  WASM_GLUE_RELATIVE_PATH,
  assertWasmMagicPrefix,
  pathLeakNeedles,
} from "./wasm_size_policy.mjs";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../..");

function findPathLeak(haystackUtf8, needles) {
  for (const needle of needles) {
    if (haystackUtf8.includes(needle)) {
      return needle;
    }
  }
  return null;
}

export async function assertPublishedWasmSizeContract(pkgDir) {
  const wasmPath = path.join(pkgDir, WASM_BINARY_RELATIVE_PATH);
  const wasmBytes = await readFile(wasmPath);
  assertWasmMagicPrefix(wasmBytes);
  assert.ok(
    wasmBytes.byteLength < BASELINE_WASM_BYTES,
    `WASM ${wasmBytes.byteLength} bytes must beat baseline ${BASELINE_WASM_BYTES}`,
  );
  assert.ok(
    wasmBytes.byteLength <= MAX_WASM_BYTES,
    `WASM ${wasmBytes.byteLength} bytes exceeds Track 6 cap ${MAX_WASM_BYTES}`,
  );

  const repoRootPosix = repoRoot.replaceAll("\\", "/");
  const needles = pathLeakNeedles(repoRootPosix);
  const wasmText = wasmBytes.toString("latin1");
  const wasmLeak = findPathLeak(wasmText, needles);
  assert.equal(
    wasmLeak,
    null,
    `WASM binary must not embed host path needle ${wasmLeak}`,
  );

  const gluePath = path.join(pkgDir, WASM_GLUE_RELATIVE_PATH);
  const glueText = await readFile(gluePath, "utf8");
  const glueLeak = findPathLeak(glueText, needles);
  assert.equal(
    glueLeak,
    null,
    `WASM glue must not embed host path needle ${glueLeak}`,
  );

  return {
    wasmBytes: wasmBytes.byteLength,
    maxWasmBytes: MAX_WASM_BYTES,
    baselineWasmBytes: BASELINE_WASM_BYTES,
  };
}
