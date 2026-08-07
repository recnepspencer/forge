#!/usr/bin/env node
/**
 * Gate 5.0 / Track 5 oracle: measure published JS footprint of worth-signals-wasm.
 */
import { access, mkdir, readdir, readFile, stat, writeFile } from "node:fs/promises";
import path from "node:path";
import process from "node:process";
import { fileURLToPath, pathToFileURL } from "node:url";

import { runNpm, tarballFileName } from "./verify-worth-signals-wasm-package-support.mjs";

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

async function listFilesRecursive(rootDir) {
  const collected = [];
  async function walk(currentDir) {
    const entries = await readdir(currentDir, { withFileTypes: true });
    for (const entry of entries) {
      const absolutePath = path.join(currentDir, entry.name);
      if (entry.isDirectory()) {
        await walk(absolutePath);
        continue;
      }
      collected.push(absolutePath);
    }
  }
  await walk(rootDir);
  return collected;
}

function toPosixRelative(rootDir, absolutePath) {
  return path.relative(rootDir, absolutePath).split(path.sep).join("/");
}

export async function measureJsFootprint(pkgDir, options = {}) {
  const pack = options.pack !== false;
  const packageJson = JSON.parse(
    await readFile(path.join(pkgDir, "package.json"), "utf8"),
  );
  const files = await listFilesRecursive(pkgDir);
  const jsFiles = [];
  for (const absolutePath of files) {
    if (!absolutePath.endsWith(".js")) {
      continue;
    }
    const relativePath = toPosixRelative(pkgDir, absolutePath);
    if (relativePath.includes("node_modules/")) {
      continue;
    }
    if (relativePath.endsWith(".tgz") || relativePath.includes(".tgz")) {
      continue;
    }
    const info = await stat(absolutePath);
    jsFiles.push({
      path: relativePath,
      bytes: info.size,
    });
  }
  jsFiles.sort((left, right) => right.bytes - left.bytes);

  const productJs = jsFiles.filter((file) => file.path.startsWith("product/"));
  const reactJs = jsFiles.filter((file) => file.path.startsWith("react/"));
  const chunkJs = jsFiles.filter((file) => file.path.startsWith("chunks/"));
  const bridgeJs = jsFiles.filter((file) =>
    file.path.startsWith("product/entrypoint/bridge/")
  );

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
    gate: "gate5-js-footprint",
    generatedAt: new Date().toISOString(),
    package: {
      name: packageJson.name,
      version: packageJson.version,
      exports: packageJson.exports ?? null,
      sideEffects: packageJson.sideEffects ?? null,
    },
    counts: {
      jsFiles: jsFiles.length,
      productJsFiles: productJs.length,
      reactJsFiles: reactJs.length,
      chunkJsFiles: chunkJs.length,
      bridgeJsFiles: bridgeJs.length,
    },
    bytes: {
      jsTotal: jsFiles.reduce((sum, file) => sum + file.bytes, 0),
      productJsTotal: productJs.reduce((sum, file) => sum + file.bytes, 0),
      tarball: tarball?.bytes ?? null,
    },
    tarball,
    files: jsFiles.map((file) => ({
      path: file.path,
      bytes: file.bytes,
    })),
    topJsFiles: jsFiles.slice(0, 20),
  };
}

async function main() {
  const options = parseArgs(process.argv.slice(2));
  if (options.help) {
    process.stdout.write(
      `Usage: node scripts/wasm/measure-worth-signals-wasm-js-footprint.mjs [--pkg-dir path] [--report path] [--no-pack]\n`,
    );
    return;
  }
  const pkgDir = path.resolve(repoRoot, options.pkgDir);
  await access(path.join(pkgDir, "package.json"));
  const report = await measureJsFootprint(pkgDir, { pack: options.pack });
  const reportPath = path.resolve(
    repoRoot,
    options.reportPath ??
      "scripts/wasm/js-chunk-certification/baseline-pre-track5.json",
  );
  await mkdir(path.dirname(reportPath), { recursive: true });
  await writeFile(reportPath, `${JSON.stringify(report, null, 2)}\n`, "utf8");
  process.stdout.write(`JS footprint report written to ${reportPath}\n`);
  process.stdout.write(
    `jsFiles=${report.counts.jsFiles} productJs=${report.counts.productJsFiles} tarballBytes=${report.bytes.tarball}\n`,
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
