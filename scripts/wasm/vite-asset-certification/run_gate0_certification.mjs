import path from "node:path";
import process from "node:process";

import { allocateLoopbackPort } from "./allocate_loopback_port.mjs";
import { ensurePreparedPackage } from "./ensure_prepared_package.mjs";
import { preparePackedViteConsumerWorld } from "./prepare_packed_vite_consumer_world.mjs";
import { runSpaFallbackCell } from "./run_spa_fallback_cell.mjs";
import { runViteDevDeploymentCells } from "./run_vite_dev_cells.mjs";
import { runViteProductionPreviewCells } from "./run_vite_production_preview_cell.mjs";
import { summarizeGateDecision } from "./cell_verdicts.mjs";

const VITE8 = "^8.0.1";
const VITE7 = "7.1.12";

export async function runGate0Certification(cliOptions = {}) {
  const repoRoot = cliOptions.repoRoot ?? process.cwd();
  const pkgDir = path.resolve(
    repoRoot,
    cliOptions.pkgDir ?? "crates/worth-signal-wasm/pkg",
  );
  const includeVite7 = cliOptions.includeVite7 !== false;
  const includeVite7Assets = cliOptions.includeVite7Assets !== false;
  const includePreview = cliOptions.includePreview !== false;
  const includeSpaFallback = cliOptions.includeSpaFallback !== false;
  const keepWorlds = cliOptions.keepWorlds === true;
  const buildIfMissing = cliOptions.buildIfMissing === true;
  let retainWorldsForInspection = keepWorlds;

  const prepared = await ensurePreparedPackage({
    repoRoot,
    pkgDir,
    buildIfMissing,
  });

  const cellResults = [];
  const worlds = [];

  try {
    const vite8World = await preparePackedViteConsumerWorld({
      pkgDir: prepared.pkgDir,
      packageName: prepared.packageName,
      packageVersion: prepared.packageVersion,
      viteVersion: VITE8,
      forceOptimizeInclude: true,
      spaFallbackWasm: false,
      worldLabel: "vite8",
    });
    worlds.push(vite8World);

    cellResults.push(
      ...(await runCapturedCells("vite8-dev", async () =>
        runViteDevDeploymentCells({
          world: vite8World,
          port: await allocateLoopbackPort(),
          cellPrefix: "vite8-dev",
        }),
      )),
    );

    if (includePreview) {
      cellResults.push(
        ...(await runCapturedCells("vite8-preview", async () =>
          runViteProductionPreviewCells({
            world: vite8World,
            port: await allocateLoopbackPort(),
            cellPrefix: "vite8-preview",
          }),
        )),
      );
    }

    if (includeSpaFallback) {
      const spaWorld = await preparePackedViteConsumerWorld({
        pkgDir: prepared.pkgDir,
        packageName: prepared.packageName,
        packageVersion: prepared.packageVersion,
        viteVersion: VITE8,
        forceOptimizeInclude: true,
        spaFallbackWasm: true,
        worldLabel: "spa",
      });
      worlds.push(spaWorld);
      cellResults.push(
        ...(await runCapturedCells("spa-fallback", async () => [
          await runSpaFallbackCell({
            world: spaWorld,
            port: await allocateLoopbackPort(),
          }),
        ])),
      );
    }

    if (includeVite7) {
      const vite7World = await preparePackedViteConsumerWorld({
        pkgDir: prepared.pkgDir,
        packageName: prepared.packageName,
        packageVersion: prepared.packageVersion,
        viteVersion: VITE7,
        forceOptimizeInclude: true,
        spaFallbackWasm: false,
        assetsInjection: false,
        worldLabel: "vite7",
      });
      worlds.push(vite7World);
      cellResults.push(
        ...(await runCapturedCells("vite7-dev", async () =>
          runViteDevDeploymentCells({
            world: vite7World,
            port: await allocateLoopbackPort(),
            cellPrefix: "vite7-dev",
          }),
        )),
      );
    }

    if (includeVite7Assets) {
      const vite7AssetsWorld = await preparePackedViteConsumerWorld({
        pkgDir: prepared.pkgDir,
        packageName: prepared.packageName,
        packageVersion: prepared.packageVersion,
        viteVersion: VITE7,
        forceOptimizeInclude: true,
        spaFallbackWasm: false,
        assetsInjection: true,
        worldLabel: "vite7-assets",
      });
      worlds.push(vite7AssetsWorld);
      cellResults.push(
        ...(await runCapturedCells("vite7-assets", async () =>
          runViteDevDeploymentCells({
            world: vite7AssetsWorld,
            port: await allocateLoopbackPort(),
            cellPrefix: "vite7-assets",
          }),
        )),
      );
    }
  } finally {
    const hasFailedCell = cellResults.some(
      (cell) => cell.verdict?.status === "failed",
    );
    retainWorldsForInspection = keepWorlds || hasFailedCell;
    if (!retainWorldsForInspection) {
      for (const world of worlds) {
        await world.dispose();
      }
    }
  }

  return {
    gate: "gate0-vite-asset-certification",
    generatedAt: new Date().toISOString(),
    package: {
      name: prepared.packageName,
      version: prepared.packageVersion,
      pkgDir: prepared.pkgDir,
      tarballPath: prepared.tarballPath,
    },
    matrix: {
      includeVite7,
      includeVite7Assets,
      includePreview,
      includeSpaFallback,
      forceOptimizeInclude: true,
      defaultAssetsInjection: false,
      vite7AssetsInjection: includeVite7Assets,
      installShape: "npm-pack-tarball",
      workerFormat: "es",
    },
    cells: cellResults.map(serializeCell),
    decision: summarizeGateDecision(cellResults),
    keptWorlds: retainWorldsForInspection
      ? worlds.map((world) => world.worldRoot)
      : [],
  };
}

async function runCapturedCells(groupId, run) {
  try {
    return await run();
  } catch (error) {
    const message = error instanceof Error ? error.stack ?? error.message : String(error);
    return [{
      cellId: `${groupId}-harness-failure`,
      deployment: null,
      targetUrl: null,
      construction: {
        phase: "failed",
        deployment: null,
        errorName: "Gate0HarnessFailure",
        errorMessage: message,
        artifactFamily: null,
        moduleUrl: null,
      },
      wasm: emptyAssetSummary(),
      worker: emptyAssetSummary(),
      prebundleEvidence: {
        probeModuleUrl: null,
        probeAppearsFromViteDeps: false,
        wasmRequestedFromViteDeps: false,
        wasmRequestedWithPackagePath: false,
      },
      consoleMessages: [],
      pageErrors: [],
      world: null,
      mode: "harness",
      verdict: {
        claim: "harnessExecutedCellGroup",
        status: "failed",
        reason: "harnessThrew",
        constructionPhase: "failed",
        constructionError: message,
        wasmPrefixClasses: [],
        workerPrefixClasses: [],
        prebundleEvidence: null,
        prebundleCache: null,
        prebundleConfirmed: false,
      },
    }];
  }
}

function emptyAssetSummary() {
  return {
    count: 0,
    urls: [],
    prefixClasses: [],
    statuses: [],
    contentTypes: [],
    entries: [],
  };
}

function serializeCell(cell) {
  return {
    cellId: cell.cellId,
    mode: cell.mode,
    deployment: cell.deployment,
    world: cell.world,
    targetUrl: cell.targetUrl,
    construction: cell.construction,
    wasm: {
      count: cell.wasm.count,
      urls: cell.wasm.urls,
      prefixClasses: cell.wasm.prefixClasses,
      statuses: cell.wasm.statuses,
      contentTypes: cell.wasm.contentTypes,
      entries: cell.wasm.entries,
    },
    worker: {
      count: cell.worker.count,
      urls: cell.worker.urls,
      prefixClasses: cell.worker.prefixClasses,
      statuses: cell.worker.statuses,
      contentTypes: cell.worker.contentTypes,
      entries: cell.worker.entries,
    },
    prebundleEvidence: cell.prebundleEvidence,
    prebundleCache: cell.prebundleCache ?? null,
    verdict: cell.verdict,
    pageErrors: cell.pageErrors ?? [],
    pageStatusText: cell.pageStatusText ?? null,
    waitError: cell.waitError ?? null,
    failedResponses: (cell.failedResponses ?? []).slice(0, 50),
    consoleMessages: (cell.consoleMessages ?? []).slice(0, 50),
  };
}
