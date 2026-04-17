import { execFile } from "node:child_process";
import { copyFile, mkdir, readdir, readFile, writeFile } from "node:fs/promises";
import { promisify } from "node:util";
import path from "node:path";
import process from "node:process";

const execFileAsync = promisify(execFile);
const scope = process.env.FORGE_SIGNAL_WASM_SCOPE ?? "aust-group";

const normalizedScope = scope.toLowerCase();
const repoUrl = process.env.FORGE_SIGNAL_WASM_REPOSITORY_URL
  ?? "https://github.com/AuST-Group/forge.git";
const pkgDir = path.resolve(
  process.argv[2] ?? "crates/forge-signal-wasm/pkg",
);
const typedDeclarationsPath = path.resolve(
  "crates/forge-signal-wasm/package/forge_signal_wasm.d.ts",
);
const readmePath = path.resolve("crates/forge-signal-wasm/README.md");
const docsDirPath = path.resolve("crates/forge-signal-wasm/docs");
const reactTypeDeclarationsPath = path.resolve(
  "crates/forge-signal-wasm/react/index.d.ts",
);
const reactTsConfigPath = path.resolve("crates/forge-signal-wasm/tsconfig.react.json");
const reactCrateDir = path.resolve("crates/forge-signal-wasm");
const reactTscBinaryPath = path.resolve(
  process.platform === "win32"
    ? "crates/forge-signal-wasm/node_modules/typescript/bin/tsc"
    : "crates/forge-signal-wasm/node_modules/typescript/bin/tsc",
);
const packageJsonPath = path.join(pkgDir, "package.json");
const packageJson = JSON.parse(await readFile(packageJsonPath, "utf8"));

async function copyDirectoryRecursive(sourceDir, destinationDir) {
  await mkdir(destinationDir, { recursive: true });
  const entries = await readdir(sourceDir, { withFileTypes: true });
  for (const entry of entries) {
    const sourcePath = path.join(sourceDir, entry.name);
    const destinationPath = path.join(destinationDir, entry.name);
    if (entry.isDirectory()) {
      await copyDirectoryRecursive(sourcePath, destinationPath);
      continue;
    }
    await copyFile(sourcePath, destinationPath);
  }
}

packageJson.name = `@${normalizedScope}/forge-signal-wasm`;
packageJson.license = "UNLICENSED";
packageJson.repository = {
  type: "git",
  url: repoUrl,
};
packageJson.publishConfig = {
  registry: "https://npm.pkg.github.com",
};
packageJson.files = [
  "*.js",
  "*.d.ts",
  "*.wasm",
  "README.md",
  "docs/**/*.md",
  "react/*.js",
  "react/*.d.ts",
];
packageJson.exports = {
  ".": {
    types: "./forge_signal_wasm.d.ts",
    import: "./forge_signal_wasm.js",
  },
  "./react": {
    types: "./react/index.d.ts",
    import: "./react/index.js",
  },
};
packageJson.peerDependencies = {
  react: ">=18.0.0",
};
packageJson.peerDependenciesMeta = {
  react: {
    optional: true,
  },
};

await writeFile(
  packageJsonPath,
  `${JSON.stringify(packageJson, null, 2)}\n`,
  "utf8",
);

const noticePath = path.join(pkgDir, "PROPRIETARY.md");
const notice = `Proprietary Software Notice

This package is unpublished for general public use and is distributed only through private agreement.

No license is granted except as expressly provided in a separate written agreement with the rights holder.
`;
await writeFile(noticePath, notice, "utf8");

const npmrcPath = path.join(pkgDir, ".npmrc");
const npmrc = `@${normalizedScope}:registry=https://npm.pkg.github.com
//npm.pkg.github.com/:_authToken=\${NODE_AUTH_TOKEN}
`;
await writeFile(npmrcPath, npmrc, "utf8");

await copyFile(
  typedDeclarationsPath,
  path.join(pkgDir, "forge_signal_wasm.d.ts"),
);
await copyFile(readmePath, path.join(pkgDir, "README.md"));
await copyDirectoryRecursive(docsDirPath, path.join(pkgDir, "docs"));
await mkdir(path.join(pkgDir, "react"), { recursive: true });
await execFileAsync(
  process.execPath,
  [
    reactTscBinaryPath,
    "-p",
    reactTsConfigPath,
  ],
  { cwd: reactCrateDir },
);
await copyFile(
  reactTypeDeclarationsPath,
  path.join(pkgDir, "react", "index.d.ts"),
);

console.log(`Prepared ${packageJson.name} in ${pkgDir}`);
