import {
  probeDeploymentCell,
  withPlaywrightPage,
} from "./browser_probe_session.mjs";
import { verdictForSpaFallbackCell } from "./cell_verdicts.mjs";
import { startViteServer } from "./vite_server_process.mjs";

export async function runSpaFallbackCell(options) {
  const {
    world,
    port,
    cellId = "spa-fallback-mainThread",
  } = options;

  const server = await startViteServer({
    cwd: world.worldRoot,
    script: "dev",
    port,
  });

  try {
    // Directly prove the adversarial host behavior even if createSignals never
    // reaches WASM init under a broader dev-server failure.
    const wasmProbeUrl =
      `${server.baseUrl}/node_modules/worth-signals-wasm/worth_signal_wasm_bg.wasm`;
    const direct = await fetch(wasmProbeUrl);
    const directBytes = new Uint8Array(await direct.arrayBuffer());
    const directPrefix = [...directBytes.slice(0, 4)];

    return await withPlaywrightPage(world.worldRoot, async (page) => {
      const probe = await probeDeploymentCell({
        page,
        baseUrl: server.baseUrl,
        deployment: "mainThreadCompatibility",
        cellId,
        timeoutMs: 180_000,
      });
      const enriched = {
        ...probe,
        directWasmProbe: {
          url: wasmProbeUrl,
          status: direct.status,
          contentType: direct.headers.get("content-type"),
          prefixHex: directPrefix
            .map((byte) => byte.toString(16).padStart(2, "0"))
            .join(" "),
          prefixClass:
            directPrefix[0] === 0x3c
              ? (directPrefix[1] === 0x21 ? "htmlDoctype" : "htmlLike")
              : "other",
        },
        world: {
          viteVersion: world.viteVersion,
          forceOptimizeInclude: world.forceOptimizeInclude,
          spaFallbackWasm: world.spaFallbackWasm,
          packageName: world.packageName,
        },
        mode: "vite-dev-spa-fallback",
      };
      // Prefer the independent direct fetch oracle for this adversarial cell.
      if (
        enriched.directWasmProbe.prefixClass === "htmlDoctype" ||
        enriched.directWasmProbe.prefixClass === "htmlLike"
      ) {
        enriched.wasm = {
          count: 1,
          urls: [enriched.directWasmProbe.url],
          prefixClasses: [enriched.directWasmProbe.prefixClass],
          statuses: [enriched.directWasmProbe.status],
          contentTypes: [enriched.directWasmProbe.contentType],
          entries: [{
            kind: "wasm",
            url: enriched.directWasmProbe.url,
            status: enriched.directWasmProbe.status,
            contentType: enriched.directWasmProbe.contentType,
            prefixHex: enriched.directWasmProbe.prefixHex,
            prefixClass: enriched.directWasmProbe.prefixClass,
            prefixError: null,
            fromServiceWorker: false,
          }],
        };
      }
      return {
        ...enriched,
        verdict: verdictForSpaFallbackCell(enriched),
      };
    });
  } finally {
    await server.stop();
  }
}
