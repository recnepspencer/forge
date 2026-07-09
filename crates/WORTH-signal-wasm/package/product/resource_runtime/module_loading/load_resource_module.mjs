import { rmSync } from "node:fs";
import { mkdir, mkdtemp, readdir, readFile, rm, writeFile } from "node:fs/promises";
import { stripTypeScriptTypes } from "node:module";
import { tmpdir } from "node:os";
import path from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const supportDir = path.dirname(fileURLToPath(import.meta.url));
const resourceRuntimeDir = path.dirname(supportDir);
const productDir = path.dirname(resourceRuntimeDir);
const packageDir = path.dirname(productDir);
const packageSourceDir = path.join(packageDir, "..", "package-src");
const apiSourceDir = path.join(packageSourceDir, "product", "api");
const resourceSourceDir = path.join(packageSourceDir, "product", "resource");
const resourceModuleGlobal = globalThis;
const cachedResourceModuleLoad =
  resourceModuleGlobal.__WORTHCachedResourceModuleLoad ?? { promise: null };
resourceModuleGlobal.__WORTHCachedResourceModuleLoad = cachedResourceModuleLoad;
const cachedResourceModuleTempDirs =
  resourceModuleGlobal.__WORTHCachedResourceModuleTempDirs ?? new Set();
resourceModuleGlobal.__WORTHCachedResourceModuleTempDirs = cachedResourceModuleTempDirs;

if (!resourceModuleGlobal.__WORTHCachedResourceModuleCleanupInstalled) {
  process.once("exit", () => {
    for (const tempDir of cachedResourceModuleTempDirs) {
      rmSync(tempDir, { recursive: true, force: true });
    }
  });
  resourceModuleGlobal.__WORTHCachedResourceModuleCleanupInstalled = true;
}

async function writeConvertedResourceTree(tempDir, sourceDir, outputDir) {
  const entries = await readdir(sourceDir, { withFileTypes: true });
  await mkdir(outputDir, { recursive: true });
  for (const entry of entries) {
    const sourcePath = path.join(sourceDir, entry.name);
    const outputPath = path.join(outputDir, replaceTsWithJs(entry.name));
    if (entry.isDirectory()) {
      await writeConvertedResourceTree(tempDir, sourcePath, outputPath);
      continue;
    }
    const source = await readFile(sourcePath, "utf8");
    const transformed = stripTypeScriptTypes(source, { mode: "transform" });
    await writeFile(outputPath, transformed, "utf8");
  }
}

function replaceTsWithJs(name) {
  return name.endsWith(".ts") ? `${name.slice(0, -3)}.js` : name;
}

async function loadResourceModule() {
  if (cachedResourceModuleLoad.promise !== null) {
    return cachedResourceModuleLoad.promise;
  }
  cachedResourceModuleLoad.promise = loadResourceModuleIntoCachedTempDir();
  return cachedResourceModuleLoad.promise;
}

async function loadResourceModuleIntoCachedTempDir() {
  const tempDir = await mkdtemp(path.join(tmpdir(), "worth-signal-resource-"));
  cachedResourceModuleTempDirs.add(tempDir);
  try {
    await writeConvertedResourceTree(
      tempDir,
      apiSourceDir,
      path.join(tempDir, "product", "api"),
    );
    await writeConvertedResourceTree(
      tempDir,
      resourceSourceDir,
      path.join(tempDir, "product", "resource"),
    );

    const moduleUrl = pathToFileURL(
      path.join(tempDir, "product", "resource", "facade.js"),
    ).href;
    const loaded = await import(moduleUrl);
    return {
      ...loaded,
      cleanup: async () => {},
    };
  } catch (error) {
    cachedResourceModuleLoad.promise = null;
    cachedResourceModuleTempDirs.delete(tempDir);
    await rm(tempDir, { recursive: true, force: true });
    throw error;
  }
}

export { loadResourceModule };
