#!/usr/bin/env node
/**
 * Gate 0: packed-tarball Vite asset certification for worth-signals-wasm.
 *
 * Measures whether default relative WASM/worker URLs survive Vite prebundling
 * without optimizeDeps.exclude and without createSignals({ assets }).
 *
 * This is a measurement instrument. It always writes a JSON report; process
 * exit code is non-zero only when the harness itself fails, or when
 * --require-vite8-default-pass is set and Vite 8 default cells fail.
 */

import { mkdir, writeFile } from "node:fs/promises";
import path from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";

import { runGate0Certification } from "./vite-asset-certification/run_gate0_certification.mjs";

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(scriptDir, "../..");

function parseArgs(argv) {
  const options = {
    repoRoot,
    pkgDir: "crates/worth-signal-wasm/pkg",
    includeVite7: true,
    includeVite7Assets: true,
    includePreview: true,
    includeSpaFallback: true,
    keepWorlds: false,
    buildIfMissing: false,
    requireVite8DefaultPass: false,
    requireVite7AssetsPass: false,
    reportPath: null,
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
    if (arg === "--build") {
      options.buildIfMissing = true;
      continue;
    }
    if (arg === "--keep-worlds") {
      options.keepWorlds = true;
      continue;
    }
    if (arg === "--skip-vite7") {
      options.includeVite7 = false;
      continue;
    }
    if (arg === "--skip-vite7-assets") {
      options.includeVite7Assets = false;
      continue;
    }
    if (arg === "--skip-preview") {
      options.includePreview = false;
      continue;
    }
    if (arg === "--skip-spa-fallback") {
      options.includeSpaFallback = false;
      continue;
    }
    if (arg === "--require-vite8-default-pass") {
      options.requireVite8DefaultPass = true;
      continue;
    }
    if (arg === "--require-vite7-assets-pass") {
      options.requireVite7AssetsPass = true;
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

function printHelp() {
  process.stdout.write(`Gate 0 — worth-signals-wasm Vite asset certification

Usage:
  node scripts/wasm/verify-worth-signals-wasm-vite-assets.mjs [options]

Options:
  --pkg-dir <path>              Prepared package directory (default: crates/worth-signal-wasm/pkg)
  --build                       wasm-pack + prepare when package files are missing
  --report <path>               Write JSON report to this path (default: under pkg/)
  --keep-worlds                 Retain temporary consumer worlds for inspection
  --skip-vite7                  Skip the Vite 7 historical-break cell
  --skip-vite7-assets           Skip the Vite 7 + createSignals({ assets }) proof cell
  --skip-preview                Skip Vite production build/preview cells
  --skip-spa-fallback           Skip the adversarial SPA-HTML-for-wasm cell
  --require-vite8-default-pass  Exit non-zero if Vite 8 default relative-URL cells fail
  --require-vite7-assets-pass   Exit non-zero if Vite 7 assets-injection cells fail
  --help                        Show this help

Cells:
  A  vite8-dev-mainThread
  B  vite8-dev-workerFirst
  C  vite7-dev-* (historical comparator; default relative URLs)
  D  vite8-preview-*
  E  spa-fallback-mainThread
  F  vite7-assets-* (portable createSignals({ assets }) proof)
`);
}

async function main() {
  const options = parseArgs(process.argv.slice(2));
  if (options.help) {
    printHelp();
    return;
  }

  const report = await runGate0Certification(options);
  const reportPath = path.resolve(
    options.repoRoot,
    options.reportPath ??
      path.join(options.pkgDir, "gate0-vite-asset-certification-report.json"),
  );
  const durableReportPath = path.resolve(
    options.repoRoot,
    "scripts/wasm/vite-asset-certification/last-gate0-report.json",
  );
  await mkdir(path.dirname(reportPath), { recursive: true });
  await writeFile(reportPath, `${JSON.stringify(report, null, 2)}\n`, "utf8");
  await writeFile(
    durableReportPath,
    `${JSON.stringify(report, null, 2)}\n`,
    "utf8",
  );

  process.stdout.write(`Gate 0 report written to ${reportPath}\n`);
  process.stdout.write(`Durable copy written to ${durableReportPath}\n`);
  process.stdout.write(
    `Decision: ${report.decision.recommendation}\n`,
  );
  for (const cell of report.cells) {
    process.stdout.write(
      `- ${cell.cellId}: ${cell.verdict.status} (${cell.verdict.reason})\n`,
    );
  }
  if (Array.isArray(report.decision.notes)) {
    for (const note of report.decision.notes) {
      process.stdout.write(`note: ${note}\n`);
    }
  }

  if (options.requireVite8DefaultPass) {
    const failed = report.decision.vite8DevDefaultCells.filter(
      (cell) => cell.status === "failed",
    );
    if (failed.length > 0) {
      process.exitCode = 1;
    }
  }
  if (options.requireVite7AssetsPass) {
    const failed = (report.decision.vite7AssetsCells ?? []).filter(
      (cell) => cell.status !== "passed",
    );
    if (failed.length > 0) {
      process.exitCode = 1;
    }
  }
}

main().catch((error) => {
  process.stderr.write(
    `${error instanceof Error ? error.stack ?? error.message : String(error)}\n`,
  );
  process.exitCode = 1;
});
