#!/usr/bin/env node
/**
 * Track 6: fail-closed Binaryen size pass for worth_signal_wasm_bg.wasm.
 * Owns wasm-opt for the publish lane (wasm-pack implicit opt is disabled).
 */
import { execFile, spawnSync } from "node:child_process";
import { copyFile, mkdtemp, readFile, rm, stat, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import process from "node:process";
import { promisify } from "node:util";
import { fileURLToPath, pathToFileURL } from "node:url";

import {
  BASELINE_WASM_BYTES,
  WASM_BINARY_RELATIVE_PATH,
  assertWasmMagicPrefix,
} from "./wasm_size_policy.mjs";

const execFileAsync = promisify(execFile);
const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(scriptDir, "../..");

function resolveWasmOptBinary() {
  const fromEnv = process.env.WORTH_SIGNAL_WASM_OPT?.trim();
  if (fromEnv) {
    return fromEnv;
  }
  const which = spawnSync(
    process.platform === "win32" ? "where.exe" : "which",
    ["wasm-opt"],
    { encoding: "utf8" },
  );
  if (which.status === 0) {
    const first = String(which.stdout)
      .split(/\r?\n/u)
      .map((line) => line.trim())
      .find((line) => line.length > 0);
    if (first) {
      return first;
    }
  }
  return null;
}

function missingWasmOptError() {
  return new Error(
    "Track 6 requires Binaryen `wasm-opt` on PATH (or WORTH_SIGNAL_WASM_OPT). " +
      "Install Binaryen, then re-run optimize-worth-signals-wasm.mjs. " +
      "Publish must not skip this step.",
  );
}

export async function optimizeWorthSignalsWasm(pkgDir) {
  const wasmOpt = resolveWasmOptBinary();
  if (!wasmOpt) {
    throw missingWasmOptError();
  }

  const wasmPath = path.join(pkgDir, WASM_BINARY_RELATIVE_PATH);
  const before = await readFile(wasmPath);
  assertWasmMagicPrefix(before);
  const beforeBytes = before.byteLength;

  const tempRoot = await mkdtemp(path.join(tmpdir(), "worth-wasm-opt-"));
  const inputCopy = path.join(tempRoot, "input.wasm");
  const outputPath = path.join(tempRoot, "output.wasm");
  try {
    await writeFile(inputCopy, before);
    // -Oz size; --strip-debug removes DWARF. Feature flags match rustc/wasm-bindgen
    // output (bulk memory copies require --enable-bulk-memory-opt on Binaryen 123+).
    const primaryArgs = [
      inputCopy,
      "-o",
      outputPath,
      "-Oz",
      "--strip-debug",
      "--strip-producers",
      "--enable-bulk-memory",
      "--enable-bulk-memory-opt",
      "--enable-nontrapping-float-to-int",
      "--enable-sign-ext",
      "--enable-mutable-globals",
      "--enable-reference-types",
    ];
    try {
      await execFileAsync(wasmOpt, primaryArgs, { windowsHide: true });
    } catch (error) {
      const detail = String(error?.stderr ?? error?.message ?? error);
      const unknownFlag =
        /unknown argument|unexpected argument|Did you mean|unrecognized/iu.test(
          detail,
        );
      if (!unknownFlag) {
        throw error;
      }
      // Older Binaryen: retry without flags it does not recognize.
      const fallbackArgs = [
        inputCopy,
        "-o",
        outputPath,
        "-Oz",
        "--strip-debug",
        "--enable-bulk-memory",
        "--enable-nontrapping-float-to-int",
        "--enable-sign-ext",
        "--enable-mutable-globals",
        "--enable-reference-types",
      ];
      await execFileAsync(wasmOpt, fallbackArgs, { windowsHide: true });
      process.stderr.write(
        "wasm-opt: retried without newer Binaryen flags after flag rejection\n",
      );
    }

    const after = await readFile(outputPath);
    assertWasmMagicPrefix(after);
    const afterBytes = after.byteLength;
    if (afterBytes > beforeBytes) {
      throw new Error(
        `wasm-opt increased WASM size from ${beforeBytes} to ${afterBytes}`,
      );
    }
    await copyFile(outputPath, wasmPath);
    const finalInfo = await stat(wasmPath);
    process.stdout.write(
      `Optimized ${WASM_BINARY_RELATIVE_PATH}: ${beforeBytes} -> ${finalInfo.size} bytes ` +
        `(baseline ${BASELINE_WASM_BYTES})\n`,
    );
    return {
      beforeBytes,
      afterBytes: finalInfo.size,
      wasmOpt,
    };
  } finally {
    await rm(tempRoot, { recursive: true, force: true });
  }
}

async function main() {
  const pkgDir = path.resolve(
    repoRoot,
    process.argv[2] ?? "crates/worth-signal-wasm/pkg",
  );
  await optimizeWorthSignalsWasm(pkgDir);
}

const isMain = process.argv[1]
  && import.meta.url === pathToFileURL(path.resolve(process.argv[1])).href;
if (isMain) {
  main().catch((error) => {
    process.stderr.write(
      `${error instanceof Error ? error.stack ?? error.message : String(error)}\n`,
    );
    process.exitCode = 1;
  });
}
