#!/usr/bin/env node
/**
 * Track 4: prove WORTH-signal-demo builds against an npm-packed tarball of
 * worth-signals-wasm (not file:…/pkg + preserveSymlinks).
 *
 * The temp world recreates the apps/ + crates/…/docs layout the demo uses for
 * public documentation imports, while installing the package from package.tgz.
 */
import { cp, mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";

import { ensurePreparedPackage } from "./vite-asset-certification/ensure_prepared_package.mjs";
import {
  execFileAsync,
  runNpm,
  tarballFileName,
} from "./verify-worth-signals-wasm-package-support.mjs";

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(scriptDir, "../..");
const demoDir = path.join(repoRoot, "apps", "WORTH-signal-demo");
const docsDir = path.join(
  repoRoot,
  "crates",
  "worth-signal-wasm",
  "docs",
);

function parseArgs(argv) {
  const options = {
    repoRoot,
    pkgDir: "crates/worth-signal-wasm/pkg",
    buildIfMissing: false,
    keepWorld: false,
  };
  for (let index = 0; index < argv.length; index += 1) {
    const arg = argv[index];
    if (arg === "--pkg-dir") {
      options.pkgDir = argv[++index];
      continue;
    }
    if (arg === "--build") {
      options.buildIfMissing = true;
      continue;
    }
    if (arg === "--keep-world") {
      options.keepWorld = true;
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
  process.stdout.write(`Verify WORTH-signal-demo against a packed worth-signals-wasm tarball

Usage:
  node scripts/wasm/verify-worth-signals-demo-packed.mjs [options]

Options:
  --pkg-dir <path>   Prepared package directory
  --build            wasm-pack + prepare when package files are missing
  --keep-world       Retain the temporary demo world for inspection
  --help             Show this help
`);
}

function shouldCopyDemoPath(sourcePath) {
  const relative = path.relative(demoDir, sourcePath);
  if (relative.startsWith("..")) {
    return true;
  }
  const parts = relative.split(path.sep);
  return !parts.some((part) =>
    part === "node_modules" ||
    part === "dist" ||
    part === ".tmp" ||
    part === "coverage"
  );
}

async function rewriteDemoPackageJson(demoWorld, packageName) {
  const packageJsonPath = path.join(demoWorld, "package.json");
  const packageJson = JSON.parse(await readFile(packageJsonPath, "utf8"));
  packageJson.dependencies = {
    ...packageJson.dependencies,
    [packageName]: "file:./package.tgz",
  };
  delete packageJson.scripts?.["refresh:signal"];
  await writeFile(
    packageJsonPath,
    `${JSON.stringify(packageJson, null, 2)}\n`,
    "utf8",
  );
}

async function rewriteViteConfigForPackedInstall(demoWorld) {
  const viteConfigPath = path.join(demoWorld, "vite.config.ts");
  const source = await readFile(viteConfigPath, "utf8");
  if (!source.includes("preserveSymlinks: true")) {
    throw new Error(
      "demo vite.config.ts no longer sets preserveSymlinks: true; update packed verify",
    );
  }
  // Packed installs must not rely on symlink preservation. Workspace-only
  // aliases to local pkg/packages are unused by the demo source tree.
  const rewritten = source
    .replace("preserveSymlinks: true", "preserveSymlinks: false")
    .replace(
      /resolve:\s*\{[\s\S]*?\n  \},/u,
      `resolve: {
    preserveSymlinks: false,
  },`,
    );
  await writeFile(viteConfigPath, rewritten, "utf8");
}

async function assertDemoUsesPortableAssetsRecipe(demoWorld) {
  const platformPath = path.join(
    demoWorld,
    "src",
    "platform",
    "createDemoSignals.ts",
  );
  const source = await readFile(platformPath, "utf8");
  if (!source.includes("worth-signals-wasm/wasm?url")) {
    throw new Error("packed demo missing wasm?url asset import");
  }
  if (!source.includes("worth-signals-wasm/worker?worker&url")) {
    throw new Error("packed demo missing worker?worker&url asset import");
  }
  if (!source.includes("assets:")) {
    throw new Error("packed demo createDemoSignals must pass assets");
  }
}

async function runViteBuild(demoWorld) {
  const viteEntrypoint = path.join(
    demoWorld,
    "node_modules",
    "vite",
    "bin",
    "vite.js",
  );
  await execFileAsync(process.execPath, [viteEntrypoint, "build"], {
    cwd: demoWorld,
    env: process.env,
  });
}

async function main() {
  const options = parseArgs(process.argv.slice(2));
  if (options.help) {
    printHelp();
    return;
  }

  const pkgDir = path.resolve(options.repoRoot, options.pkgDir);
  const prepared = await ensurePreparedPackage({
    repoRoot: options.repoRoot,
    pkgDir,
    buildIfMissing: options.buildIfMissing,
  });

  const tarballName = tarballFileName(
    prepared.packageName,
    prepared.packageVersion,
  );
  const sourceTarballPath = path.join(prepared.pkgDir, tarballName);
  const worldRoot = await mkdtemp(path.join(tmpdir(), "worth-demo-packed-"));
  const demoWorld = path.join(worldRoot, "apps", "WORTH-signal-demo");
  const docsWorld = path.join(
    worldRoot,
    "crates",
    "worth-signal-wasm",
    "docs",
  );

  try {
    await cp(demoDir, demoWorld, {
      recursive: true,
      filter: shouldCopyDemoPath,
    });
    await cp(docsDir, docsWorld, { recursive: true });
    await cp(sourceTarballPath, path.join(demoWorld, "package.tgz"));
    await rewriteDemoPackageJson(demoWorld, prepared.packageName);
    await rewriteViteConfigForPackedInstall(demoWorld);
    await assertDemoUsesPortableAssetsRecipe(demoWorld);

    await runNpm(["install"], { cwd: demoWorld });
    await runViteBuild(demoWorld);

    process.stdout.write(
      `Packed demo consumer build succeeded at ${demoWorld}\n`,
    );
    process.stdout.write(
      `Install shape: npm pack tarball (${tarballName}) with preserveSymlinks=false\n`,
    );
  } finally {
    if (!options.keepWorld) {
      await rm(worldRoot, { recursive: true, force: true });
    } else {
      process.stdout.write(`Kept packed demo world at ${worldRoot}\n`);
    }
  }
}

main().catch((error) => {
  process.stderr.write(
    `${error instanceof Error ? error.stack ?? error.message : String(error)}\n`,
  );
  process.exitCode = 1;
});
