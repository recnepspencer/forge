import assert from "node:assert/strict";
import { access, copyFile, mkdtemp, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";

import { execFileAsync, runNpm } from "./verify-worth-signals-wasm-package-support.mjs";

async function createConsumer(tarballPath, dependencies) {
  const tempDir = await mkdtemp(path.join(tmpdir(), "worth-signals-consumer-"));
  const localTarballPath = path.join(tempDir, path.basename(tarballPath));
  await copyFile(tarballPath, localTarballPath);
  await runNpm(["init", "-y"], { cwd: tempDir });
  await runNpm(["pkg", "set", "type=module"], { cwd: tempDir });
  await runNpm(
    ["install", path.basename(localTarballPath), ...dependencies],
    { cwd: tempDir },
  );
  return tempDir;
}

async function runImportSmoke(tempDir, source) {
  const smokePath = path.join(tempDir, "consumer-smoke.mjs");
  await writeFile(smokePath, source, "utf8");
  await execFileAsync(process.execPath, [smokePath], { cwd: tempDir });
}

async function packageExists(tempDir, packagePath) {
  try {
    await access(path.join(tempDir, "node_modules", ...packagePath));
    return true;
  } catch {
    return false;
  }
}

async function verifyRootWithoutReact(tarballPath, packageName) {
  const tempDir = await createConsumer(tarballPath, []);
  try {
    await runImportSmoke(
      tempDir,
      `import { createSignals } from ${JSON.stringify(packageName)};\n`
        + `if (typeof createSignals !== "function") process.exit(2);\n`,
    );
    assert.equal(
      await packageExists(tempDir, ["react", "package.json"]),
      false,
      "a root-only consumer must not install the optional React peer",
    );
    assert.equal(
      await packageExists(tempDir, ["@types", "react", "package.json"]),
      false,
      "a root-only consumer must not receive an unsolicited React type version",
    );
  } finally {
    await rm(tempDir, { recursive: true, force: true });
  }
}

async function verifyReact19(tarballPath, packageName) {
  const tempDir = await createConsumer(tarballPath, ["react@19.2.0"]);
  try {
    await runImportSmoke(
      tempDir,
      `import * as api from ${JSON.stringify(`${packageName}/react`)};\n`
        + `if (typeof api.createReactSignalsStore !== "function") process.exit(2);\n`,
    );
    assert.equal(
      await packageExists(tempDir, ["@types", "react", "package.json"]),
      false,
      "the React adapter must not install a type package into a JavaScript consumer",
    );
  } finally {
    await rm(tempDir, { recursive: true, force: true });
  }
}

export async function verifyAdditionalConsumers(tarballPath, packageName) {
  await verifyRootWithoutReact(tarballPath, packageName);
  await verifyReact19(tarballPath, packageName);
}
