import {
  probeDeploymentCell,
  withPlaywrightPage,
} from "./browser_probe_session.mjs";
import { verdictForDefaultAssetCell } from "./cell_verdicts.mjs";
import { inspectVitePrebundleCache } from "./prebundle_cache_inspection.mjs";
import { startViteServer } from "./vite_server_process.mjs";

export async function runViteDevDeploymentCells(options) {
  const {
    world,
    port,
    cellPrefix,
    deployments = ["mainThreadCompatibility", "workerFirst"],
  } = options;

  const server = await startViteServer({
    cwd: world.worldRoot,
    script: "dev",
    port,
  });

  try {
    // Trigger optimizeDeps before the timed browser probe budget starts.
    await fetch(`${server.baseUrl}/src/probe.js`).catch(() => undefined);
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
          // First optimizeDeps of this package can exceed two minutes.
          timeoutMs: 300_000,
        });
        const prebundleCache = await inspectVitePrebundleCache(
          world.worldRoot,
          world.packageName,
        );
        const enriched = {
          ...probe,
          prebundleCache,
          world: describeWorld(world),
          mode: "vite-dev",
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

function describeWorld(world) {
  return {
    viteVersion: world.viteVersion,
    forceOptimizeInclude: world.forceOptimizeInclude,
    spaFallbackWasm: world.spaFallbackWasm,
    assetsInjection: world.assetsInjection === true,
    packageName: world.packageName,
  };
}
