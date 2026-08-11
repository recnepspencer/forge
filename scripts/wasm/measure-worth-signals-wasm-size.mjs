#!/usr/bin/env node
/**
 * Gate 6.0 / Track 6 oracle: measure published WASM + tarball size.
 */
import { execFile } from "node:child_process";
import { access, mkdir, readFile, stat, writeFile } from "node:fs/promises";
import path from "node:path";
import process from "node:process";
import { promisify } from "node:util";
import { fileURLToPath, pathToFileURL } from "node:url";

import { runNpm, tarballFileName } from "./verify-worth-signals-wasm-package-support.mjs";
import {
  BASELINE_WASM_BYTES,
  MAX_WASM_BYTES,
  WASM_BINARY_RELATIVE_PATH,
  assertWasmMagicPrefix,
} from "./wasm_size_policy.mjs";

const execFileAsync = promisify(execFile);
const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(scriptDir, "../..");

function parseArgs(argv) {
  const options = {
    pkgDir: "crates/worth-signal-wasm/pkg",
    reportPath: null,
    pack: true,
  };
  for (let index = 0; index < argv.length; index += 1) {
    const arg = argv[index];
    if (arg === "--pkg-dir") {
      options.pkgDir = argv[++index];
      continue;
    }
    if (arg === "--report") {
      options.reportPath = argv[++index];
      continue;
    }
    if (arg === "--no-pack") {
      options.pack = false;
      continue;
    }
    if (arg === "--help" || arg === "-h") {
      options.help = true;
      continue;
    }
    throw new Error(`Unknown argument: ${arg}`);
  }
  return options;
}

async function toolVersion(command, args = ["--version"]) {
  try {
    const { stdout, stderr } = await execFileAsync(command, args, {
      windowsHide: true,
    });
    return String(stdout || stderr).trim().split(/\r?\n/u)[0] ?? null;
  } catch {
    return null;
  }
}

async function resolveWasmOptVersion() {
  return (
    (await toolVersion("wasm-opt")) ??
    (await toolVersion("wasm-opt.exe"))
  );
}

export async function measureWasmSize(pkgDir, options = {}) {
  const pack = options.pack !== false;
  const packageJson = JSON.parse(
    await readFile(path.join(pkgDir, "package.json"), "utf8"),
  );
  const wasmPath = path.join(pkgDir, WASM_BINARY_RELATIVE_PATH);
  const wasmBytes = await readFile(wasmPath);
  assertWasmMagicPrefix(wasmBytes);
  const wasmInfo = await stat(wasmPath);

  let tarball = null;
  if (pack) {
    const tarballName = tarballFileName(packageJson.name, packageJson.version);
    const tarballPath = path.join(pkgDir, tarballName);
    await runNpm(["pack"], { cwd: pkgDir });
    const tarballInfo = await stat(tarballPath);
    tarball = {
      name: tarballName,
      bytes: tarballInfo.size,
    };
  }

  return {
    gate: "gate6-wasm-size",
    generatedAt: new Date().toISOString(),
    package: {
      name: packageJson.name,
      version: packageJson.version,
      exportsWasm: packageJson.exports?.["./wasm"] ?? null,
    },
    policy: {
      baselineWasmBytes: BASELINE_WASM_BYTES,
      maxWasmBytes: MAX_WASM_BYTES,
    },
    bytes: {
      wasm: wasmInfo.size,
      tarball: tarball?.bytes ?? null,
    },
    tarball,
    magic: "00 61 73 6d",
    tools: {
      wasmPack: await toolVersion("wasm-pack"),
      wasmOpt: await resolveWasmOptVersion(),
      rustc: await toolVersion("rustc"),
    },
  };
}

async function main() {
  const options = parseArgs(process.argv.slice(2));
  if (options.help) {
    process.stdout.write(
      `Usage: node scripts/wasm/measure-worth-signals-wasm-size.mjs [--pkg-dir path] [--report path] [--no-pack]\n`,
    );
    return;
  }
  const pkgDir = path.resolve(repoRoot, options.pkgDir);
  await access(path.join(pkgDir, "package.json"));
  const report = await measureWasmSize(pkgDir, { pack: options.pack });
  const reportPath = path.resolve(
    repoRoot,
    options.reportPath ??
      "scripts/wasm/wasm_size_certification/baseline-pre-track6.json",
  );
  await mkdir(path.dirname(reportPath), { recursive: true });
  await writeFile(reportPath, `${JSON.stringify(report, null, 2)}\n`, "utf8");
  process.stdout.write(`WASM size report written to ${reportPath}\n`);
  process.stdout.write(
    `wasmBytes=${report.bytes.wasm} tarballBytes=${report.bytes.tarball} baseline=${BASELINE_WASM_BYTES}\n`,
  );
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
