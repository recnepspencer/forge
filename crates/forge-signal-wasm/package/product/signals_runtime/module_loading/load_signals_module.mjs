import { mkdir, mkdtemp, readdir, readFile, rm, writeFile } from "node:fs/promises";
import { stripTypeScriptTypes } from "node:module";
import { tmpdir } from "node:os";
import path from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const moduleDir = path.dirname(fileURLToPath(import.meta.url));
const packageDir = path.join(moduleDir, "..", "..", "..");
const packageSourceDir = path.join(packageDir, "..", "package-src");
const apiSourceDir = path.join(packageSourceDir, "product", "api");
const formsSourceDir = path.join(packageSourceDir, "product", "forms");
const resourceSourceDir = path.join(packageSourceDir, "product", "resource");

export async function loadSignalsModule(options = {}) {
  const tempDir = await mkdtemp(path.join(tmpdir(), "forge-signal-product-"));
  try {
    const filesToCopy = [
      [
        "product/authoring_option_validation.ts",
        "product/authoring_option_validation.js",
      ],
      ["product/signals.ts", "product/signals.js"],
      ["product/callback_frames.ts", "product/callback_frames.js"],
      ["product/controllers.ts", "product/controllers.js"],
      ["product/diagnostics.ts", "product/diagnostics.js"],
      [
        "product/graph_authoring_support.ts",
        "product/graph_authoring_support.js",
      ],
      ["product/graph_support.ts", "product/graph_support.js"],
      ["product/graphs.ts", "product/graphs.js"],
      [
        "product/host_capability_declarations.ts",
        "product/host_capability_declarations.js",
      ],
      [
        "product/host_capability_registrations.ts",
        "product/host_capability_registrations.js",
      ],
      [
        "product/host_capability_reports.ts",
        "product/host_capability_reports.js",
      ],
      ["product/host_capabilities.ts", "product/host_capabilities.js"],
      ["product/history.ts", "product/history.js"],
      ["product/handles.ts", "product/handles.js"],
      ["product/linked.ts", "product/linked.js"],
      ["product/output_projection_ids.ts", "product/output_projection_ids.js"],
      ["product/public_inputs.ts", "product/public_inputs.js"],
      [
        "product/reserved_authoring_ids.ts",
        "product/reserved_authoring_ids.js",
      ],
      ["product/scopes.ts", "product/scopes.js"],
      ["product/specialist.ts", "product/specialist.js"],
      ["product/transactions.ts", "product/transactions.js"],
      ["product/symbols.ts", "product/symbols.js"],
    ];

    for (const [sourceRelativePath, outputRelativePath] of filesToCopy) {
      const sourcePath = path.join(packageSourceDir, sourceRelativePath);
      const targetPath = path.join(tempDir, outputRelativePath);
      await mkdir(path.dirname(targetPath), { recursive: true });
      const source = await readFile(sourcePath, "utf8");
      await writeFile(
        targetPath,
        stripTypeScriptTypes(source, { mode: "transform" }),
        "utf8",
      );
    }

    await writeConvertedTree(
      apiSourceDir,
      path.join(tempDir, "product", "api"),
    );
    await writeConvertedTree(
      formsSourceDir,
      path.join(tempDir, "product", "forms"),
    );
    await writeConvertedTree(
      resourceSourceDir,
      path.join(tempDir, "product", "resource"),
    );

    const rawSurfacePath = path.join(tempDir, "raw_surface.js");
    if (options.rawSurface === "real") {
      const realRawSurfaceUrl = pathToFileURL(
        path.join(packageDir, "..", "pkg", "raw_surface.js"),
      ).href;
      await writeFile(
        rawSurfacePath,
        `export * from ${JSON.stringify(realRawSurfaceUrl)};\n`,
        "utf8",
      );
    } else {
      await writeFile(
        rawSurfacePath,
        "export function createRawSignals() { throw new Error('createRawSignals should not be used in signals product runtime tests'); }\n",
        "utf8",
      );
    }

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

async function writeConvertedTree(sourceDir, outputDir) {
  const entries = await readdir(sourceDir, { withFileTypes: true });
  await mkdir(outputDir, { recursive: true });
  for (const entry of entries) {
    const sourcePath = path.join(sourceDir, entry.name);
    const outputPath = path.join(outputDir, replaceTsWithJs(entry.name));
    if (entry.isDirectory()) {
      await writeConvertedTree(sourcePath, outputPath);
      continue;
    }
    const source = await readFile(sourcePath, "utf8");
    await writeFile(
      outputPath,
      stripTypeScriptTypes(source, { mode: "transform" }),
      "utf8",
    );
  }
}

function replaceTsWithJs(name) {
  return name.endsWith(".ts") ? `${name.slice(0, -3)}.js` : name;
}
