import { access, readFile } from "node:fs/promises";
import path from "node:path";
import process from "node:process";

import {
  execFileAsync,
  runNpm,
  tarballFileName,
} from "../verify-worth-signals-wasm-package-support.mjs";

export async function ensurePreparedPackage(options) {
  const {
    repoRoot,
    pkgDir,
    buildIfMissing = false,
  } = options;

  const packageJsonPath = path.join(pkgDir, "package.json");
  const wasmPath = path.join(pkgDir, "worth_signal_wasm_bg.wasm");
  const indexPath = path.join(pkgDir, "index.js");

  const missing = [];
  for (const candidate of [packageJsonPath, wasmPath, indexPath]) {
    try {
      await access(candidate);
    } catch {
      missing.push(candidate);
    }
  }

  if (missing.length > 0) {
    if (!buildIfMissing) {
      throw new Error(
        "Gate 0 requires a prepared package at " +
          `${pkgDir}. Missing: ${missing.join(", ")}. ` +
          "Re-run with --build, or run publish-worth-signals-wasm.ps1 -SkipPublish first.",
      );
    }
    await buildAndPreparePackage({ repoRoot, pkgDir });
  }

  const packageJson = JSON.parse(await readFile(packageJsonPath, "utf8"));
  if (!packageJson.exports?.["."] || !packageJson.name) {
    throw new Error(
      `Package at ${pkgDir} does not look prepared (missing exports or name). ` +
        "Run prepare-worth-signals-wasm-package.mjs.",
    );
  }

  const tarballName = tarballFileName(packageJson.name, packageJson.version);
  const tarballPath = path.join(pkgDir, tarballName);
  try {
    await access(tarballPath);
  } catch {
    await runNpm(["pack"], { cwd: pkgDir });
  }

  return {
    pkgDir,
    packageName: packageJson.name,
    packageVersion: packageJson.version,
    tarballPath: path.join(pkgDir, tarballName),
  };
}

async function buildAndPreparePackage({ repoRoot, pkgDir }) {
  const crateDir = path.join(repoRoot, "crates", "worth-signal-wasm");
  const outDir = path.basename(pkgDir);
  await execFileAsync(
    "wasm-pack",
    [
      "build",
      crateDir,
      "--target",
      "bundler",
      "--profile",
      "release-wasm",
      "--no-opt",
      "--out-dir",
      outDir,
    ],
    { cwd: repoRoot },
  );
  await execFileAsync(
    process.execPath,
    [
      path.join(repoRoot, "scripts", "wasm", "optimize-worth-signals-wasm.mjs"),
      pkgDir,
    ],
    { cwd: repoRoot },
  );
  await execFileAsync(
    process.execPath,
    [
      path.join(repoRoot, "scripts", "wasm", "prepare-worth-signals-wasm-package.mjs"),
      pkgDir,
    ],
    {
      cwd: repoRoot,
      env: {
        ...process.env,
        WORTH_SIGNAL_WASM_PACKAGE_NAME: "worth-signals-wasm",
      },
    },
  );
}
