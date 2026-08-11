import { access, copyFile, mkdir, mkdtemp, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import process from "node:process";

import { runNpm, tarballFileName } from "../verify-worth-signals-wasm-package-support.mjs";
import {
  buildConsumerPackageJson,
  buildProbeHtml,
  buildProbeModule,
  buildViteConfigSource,
} from "./consumer_world_sources.mjs";

export async function preparePackedViteConsumerWorld(options) {
  const {
    pkgDir,
    packageName,
    packageVersion,
    viteVersion,
    forceOptimizeInclude = true,
    spaFallbackWasm = false,
    assetsInjection = false,
    worldLabel,
  } = options;

  const tarballName = tarballFileName(packageName, packageVersion);
  const sourceTarballPath = path.join(pkgDir, tarballName);
  await access(sourceTarballPath);

  const worldRoot = await mkdtemp(
    path.join(tmpdir(), `worth-gate0-${worldLabel}-`),
  );

  try {
    await writeFile(
      path.join(worldRoot, "package.json"),
      buildConsumerPackageJson({ viteVersion, packageName }),
      "utf8",
    );
    await writeFile(
      path.join(worldRoot, "vite.config.js"),
      buildViteConfigSource({ forceOptimizeInclude, spaFallbackWasm }),
      "utf8",
    );
    await writeFile(path.join(worldRoot, "index.html"), buildProbeHtml(), "utf8");
    await mkdir(path.join(worldRoot, "src"), { recursive: true });
    await writeFile(
      path.join(worldRoot, "src", "probe.js"),
      buildProbeModule({ assetsInjection }),
      "utf8",
    );
    await copyFile(sourceTarballPath, path.join(worldRoot, "package.tgz"));

    await runNpm(["install"], {
      cwd: worldRoot,
      env: {
        ...process.env,
        PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD: "0",
      },
    });
    await runNpm(["exec", "playwright", "install", "chromium"], {
      cwd: worldRoot,
    });

    return {
      worldRoot,
      packageName,
      viteVersion,
      forceOptimizeInclude,
      spaFallbackWasm,
      assetsInjection,
      async dispose() {
        await rm(worldRoot, { recursive: true, force: true });
      },
    };
  } catch (error) {
    await rm(worldRoot, { recursive: true, force: true });
    throw error;
  }
}
