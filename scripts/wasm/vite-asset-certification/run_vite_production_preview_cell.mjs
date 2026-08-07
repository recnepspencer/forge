import {
  probeDeploymentCell,
  withPlaywrightPage,
} from "./browser_probe_session.mjs";
import { verdictForDefaultAssetCell } from "./cell_verdicts.mjs";
import { runViteBuild, startViteServer } from "./vite_server_process.mjs";

export async function runViteProductionPreviewCells(options) {
  const {
    world,
    port,
    cellPrefix = "vite8-preview",
    deployments = ["mainThreadCompatibility", "workerFirst"],
  } = options;

  try {
    await runViteBuild(world.worldRoot);
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    return deployments.map((deployment) => {
      const shortName =
        deployment === "mainThreadCompatibility" ? "mainThread" : "workerFirst";
      return buildHarnessFailureCell({
        cellId: `${cellPrefix}-${shortName}`,
        deployment,
        world,
        mode: "vite-preview",
        reason: "viteProductionBuildFailed",
        message,
      });
    });
  }

  const server = await startViteServer({
    cwd: world.worldRoot,
    script: "preview",
    port,
    readyPattern: /Local:\s+http:\/\/127\.0\.0\.1/u,
  });

  try {
    return await withPlaywrightPage(world.worldRoot, async (page) => {
      const results = [];
      for (const deployment of deployments) {
        const shortName =
          deployment === "mainThreadCompatibility" ? "mainThread" : "workerFirst";
        const cellId = `${cellPrefix}-${shortName}`;
        const probe = await probeDeploymentCell({
          page,
          baseUrl: server.baseUrl,
          deployment,
          cellId,
          timeoutMs: 300_000,
        });
        const enriched = {
          ...probe,
          world: {
            viteVersion: world.viteVersion,
            forceOptimizeInclude: world.forceOptimizeInclude,
            spaFallbackWasm: world.spaFallbackWasm,
            packageName: world.packageName,
          },
          mode: "vite-preview",
        };
        results.push({
          ...enriched,
          verdict: verdictForDefaultAssetCell(enriched),
        });
      }
      return results;
    });
  } finally {
    await server.stop();
  }
}

function buildHarnessFailureCell({ cellId, deployment, world, mode, reason, message }) {
  return {
    cellId,
    deployment,
    targetUrl: null,
    construction: {
      phase: "failed",
      deployment,
      errorName: "Gate0HarnessFailure",
      errorMessage: message,
      artifactFamily: null,
      moduleUrl: null,
    },
    wasm: { count: 0, urls: [], prefixClasses: [], statuses: [], contentTypes: [], entries: [] },
    worker: { count: 0, urls: [], prefixClasses: [], statuses: [], contentTypes: [], entries: [] },
    prebundleEvidence: {
      probeModuleUrl: null,
      probeAppearsFromViteDeps: false,
      wasmRequestedFromViteDeps: false,
      wasmRequestedWithPackagePath: false,
    },
    consoleMessages: [],
    pageErrors: [],
    world: {
      viteVersion: world.viteVersion,
      forceOptimizeInclude: world.forceOptimizeInclude,
      spaFallbackWasm: world.spaFallbackWasm,
      packageName: world.packageName,
    },
    mode,
    verdict: {
      claim: "defaultRelativeAssetsLoadUnderVitePrebundle",
      status: "failed",
      reason,
      constructionPhase: "failed",
      constructionError: message,
      wasmPrefixClasses: [],
      workerPrefixClasses: [],
      prebundleEvidence: null,
      prebundleCache: null,
      prebundleConfirmed: false,
    },
  };
}
