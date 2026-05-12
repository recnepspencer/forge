import assert from "node:assert/strict";
import { copyFile, readFile, readdir } from "node:fs/promises";
import { execFile } from "node:child_process";
import process from "node:process";
import { promisify } from "node:util";
import path from "node:path";

const execFileAsync = promisify(execFile);

export async function runNpm(args, options) {
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

export function normalizeTarEntries(stdout) {
  return stdout
    .split(/\r?\n/)
    .map((line) => line.trim())
    .filter(Boolean)
    .map((entry) => entry.replaceAll("\\", "/"));
}

export function tarballFileName(packageName, version) {
  const normalizedName = packageName
    .replace(/^@/, "")
    .replace(/\//g, "-");
  return `${normalizedName}-${version}.tgz`;
}

export async function installSmokeDependencies(tempDir, tarballPath) {
  const localTarballPath = path.join(tempDir, path.basename(tarballPath));
  await copyFile(tarballPath, localTarballPath);

  await runNpm(["init", "-y"], { cwd: tempDir });
  await runNpm(["pkg", "set", "type=module"], { cwd: tempDir });
  await runNpm(
    ["install", path.basename(localTarballPath), "react", "typescript"],
    { cwd: tempDir },
  );
}

export async function assertDocsStayOnCurrentPackageStory(pkgDir, packageName) {
  const docsDir = path.join(pkgDir, "docs");
  const docNames = await readdir(docsDir);
  const docTexts = await Promise.all(
    docNames
      .filter((name) => name.endsWith(".md"))
      .map(async (name) => readFile(path.join(docsDir, name), "utf8")),
  );
  const joinedDocs = docTexts.join("\n");
  const readme = await readFile(path.join(pkgDir, "README.md"), "utf8");

  assert.equal(
    readme.includes("forge-signal-wasm-dev"),
    false,
    "package README must not refer to the obsolete forge-signal-wasm-dev package name",
  );
  assert.equal(
    joinedDocs.includes("forge-signal-wasm-dev"),
    false,
    "package docs must not refer to the obsolete forge-signal-wasm-dev package name",
  );
  assert.equal(
    readme.includes("npm install forge-signal-wasm"),
    true,
    "package README should teach the public install command",
  );
  assert.equal(
    joinedDocs.includes(packageName),
    true,
    "prepared docs should mention the package lane they are proving",
  );
}

export { execFileAsync };
