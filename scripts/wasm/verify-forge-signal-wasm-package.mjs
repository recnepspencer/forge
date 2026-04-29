import { execFile } from "node:child_process";
import { copyFile, mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import process from "node:process";
import assert from "node:assert/strict";
import { promisify } from "node:util";

const execFileAsync = promisify(execFile);

async function runNpm(args, options) {
  if (process.platform === "win32") {
    const command = `npm ${args.join(" ")}`;
    return execFileAsync(
      "cmd.exe",
      ["/d", "/s", "/c", command],
      options,
    );
  }
  return execFileAsync("npm", args, options);
}

const pkgDir = path.resolve(process.argv[2] ?? "crates/forge-signal-wasm/pkg");
const packageJsonPath = path.join(pkgDir, "package.json");

function normalizeTarEntries(stdout) {
  return stdout
    .split(/\r?\n/)
    .map((line) => line.trim())
    .filter(Boolean)
    .map((entry) => entry.replaceAll("\\", "/"));
}

function tarballFileName(packageName, version) {
  const normalizedName = packageName
    .replace(/^@/, "")
    .replace(/\//g, "-");
  return `${normalizedName}-${version}.tgz`;
}

async function installSmokeDependencies(tempDir, tarballPath) {
  const localTarballPath = path.join(tempDir, path.basename(tarballPath));
  await copyFile(tarballPath, localTarballPath);

  await runNpm(["init", "-y"], { cwd: tempDir });
  await runNpm(["pkg", "set", "type=module"], { cwd: tempDir });
  await runNpm(
    ["install", path.basename(localTarballPath), "react", "typescript"],
    { cwd: tempDir },
  );
}

async function runRuntimeSmoke(tempDir, packageName) {
  const smokeRuntimePath = path.join(tempDir, "smoke.mjs");
  const source = `import init, { createSignals } from "${packageName}";
import * as reactApi from "${packageName}/react";

const summary = {
  hasInit: typeof init === "function",
  hasCreateSignals: typeof createSignals === "function",
  reactKeys: Object.keys(reactApi).sort(),
};

console.log(JSON.stringify(summary));
`;
  await writeFile(smokeRuntimePath, source, "utf8");
  const { stdout } = await execFileAsync("node", [smokeRuntimePath], { cwd: tempDir });
  const result = JSON.parse(stdout.trim());

  assert.equal(result.hasInit, true, "root default init export should exist");
  assert.equal(result.hasCreateSignals, true, "root createSignals export should exist");
  assert.deepEqual(
    result.reactKeys,
    [
      "createReactSignalsStore",
      "useOutputValue",
      "useSignalValue",
      "useSignalsDiagnostics",
    ],
    "react subpath should export the expected public API",
  );
}

async function runTypeSmoke(tempDir, packageName) {
  const smokeTypePath = path.join(tempDir, "smoke.ts");
  const tscJsPath = path.join(tempDir, "node_modules", "typescript", "bin", "tsc");
  const source = `import { createSignals } from "${packageName}";
import { createReactSignalsStore, useSignalValue } from "${packageName}/react";

const signals = createSignals();
const count = signals.input("count", 1);
const doubled = signals.computed("doubled", () => count() * 2);
const store = createReactSignalsStore(signals);

void useSignalValue;
void count;
void doubled;
void store;
`;
  await writeFile(smokeTypePath, source, "utf8");
  const args = [
    tscJsPath,
    "--noEmit",
    "--strict",
    "--target", "ES2022",
    "--module", "NodeNext",
    "--moduleResolution", "NodeNext",
    "--skipLibCheck",
    smokeTypePath,
  ];
  await execFileAsync(process.execPath, args, { cwd: tempDir });
}

async function main() {
  const packageJson = JSON.parse(await readFile(packageJsonPath, "utf8"));
  const expectedTarballName = tarballFileName(packageJson.name, packageJson.version);
  const tarballPath = path.join(pkgDir, expectedTarballName);

  assert.equal(packageJson.main, "./index.js");
  assert.equal(packageJson.module, "./index.js");
  assert.equal(packageJson.types, "./forge_signal_wasm.d.ts");
  assert.equal(packageJson.exports["."].import, "./index.js");
  assert.equal(packageJson.exports["./react"].import, "./react/index.js");

  await rm(tarballPath, { force: true });
  await runNpm(["pack"], { cwd: pkgDir });

  const { stdout: tarStdout } = await execFileAsync("tar", ["-tf", expectedTarballName], { cwd: pkgDir });
  const entries = normalizeTarEntries(tarStdout);

  const requiredEntries = [
    "package/index.js",
    "package/index.d.ts",
    "package/raw_surface.js",
    "package/product/signals.js",
    "package/product/handles.js",
    "package/product/transactions.js",
    "package/types/model.d.ts",
    "package/types/raw_surface.d.ts",
    "package/types/callable_surface.d.ts",
    "package/react/index.js",
    "package/react/index.d.ts",
  ];

  for (const entry of requiredEntries) {
    assert(
      entries.includes(entry),
      `packed tarball is missing required entry ${entry}`,
    );
  }

  const tempDir = await mkdtemp(path.join(tmpdir(), "forge-signal-wasm-proof-"));

  try {
    await installSmokeDependencies(tempDir, tarballPath);
    await runRuntimeSmoke(tempDir, packageJson.name);
    await runTypeSmoke(tempDir, packageJson.name);
  } finally {
    await rm(tempDir, { recursive: true, force: true });
  }

  console.log(`Verified ${packageJson.name}@${packageJson.version} from ${pkgDir}`);
}

await main();
