import { mkdir, mkdtemp, readFile, readdir, rm, writeFile } from "node:fs/promises";
import { stripTypeScriptTypes } from "node:module";
import { tmpdir } from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";

const moduleDir = path.dirname(fileURLToPath(import.meta.url));
const packageDir = path.join(moduleDir, "..", "..", "..");
const packageSourceDir = path.join(packageDir, "..", "package-src");

async function copyProductTree(sourceDir, targetDir) {
  const entries = await readdir(sourceDir, { withFileTypes: true });
  for (const entry of entries) {
    const sourcePath = path.join(sourceDir, entry.name);
    const targetPath = path.join(
      targetDir,
      entry.name.endsWith(".ts") ? entry.name.replace(/\.ts$/, ".js") : entry.name,
    );
    if (entry.isDirectory()) {
      await mkdir(targetPath, { recursive: true });
      await copyProductTree(sourcePath, targetPath);
      continue;
    }
    if (!entry.isFile() || !entry.name.endsWith(".ts")) {
      continue;
    }
    const source = await readFile(sourcePath, "utf8");
    await writeFile(
      targetPath,
      stripTypeScriptTypes(source, { mode: "transform" }),
      "utf8",
    );
  }
}

export async function loadSignalsModule() {
  const tempDir = await mkdtemp(
    path.join(tmpdir(), "worth-signal-host-capability-"),
  );
  try {
    await mkdir(path.join(tempDir, "product"), { recursive: true });
    await copyProductTree(
      path.join(packageSourceDir, "product"),
      path.join(tempDir, "product"),
    );

    await writeFile(
      path.join(tempDir, "raw_surface.js"),
      "export function createRawSignals() { throw new Error('createRawSignals should not be used in host capability runtime tests'); }\n",
      "utf8",
    );

    const moduleUrl = new URL(
      `file:///${path.join(tempDir, "product", "signals.js").replace(/\\/g, "/")}`,
    );
    const loaded = await import(moduleUrl.href);
    return {
      ...loaded,
      cleanup: () => rm(tempDir, { recursive: true, force: true }),
    };
  } catch (error) {
    await rm(tempDir, { recursive: true, force: true });
    throw error;
  }
}
