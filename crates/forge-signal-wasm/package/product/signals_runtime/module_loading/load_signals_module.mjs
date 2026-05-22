import { rmSync } from "node:fs";
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
const signalsModuleGlobal = globalThis;
const cachedSignalsModuleLoads =
  signalsModuleGlobal.__forgeCachedSignalsModuleLoads ?? new Map();
signalsModuleGlobal.__forgeCachedSignalsModuleLoads = cachedSignalsModuleLoads;
const cachedSignalsModuleTempDirs =
  signalsModuleGlobal.__forgeCachedSignalsModuleTempDirs ?? new Set();
signalsModuleGlobal.__forgeCachedSignalsModuleTempDirs = cachedSignalsModuleTempDirs;

if (!signalsModuleGlobal.__forgeCachedSignalsModuleCleanupInstalled) {
  process.once("exit", () => {
    for (const tempDir of cachedSignalsModuleTempDirs) {
      rmSync(tempDir, { recursive: true, force: true });
    }
  });
  signalsModuleGlobal.__forgeCachedSignalsModuleCleanupInstalled = true;
}

export async function loadSignalsModule(options = {}) {
  const cacheKey = options.rawSurface === "real" ? "real" : "stub";
  const cachedLoad = cachedSignalsModuleLoads.get(cacheKey);
  if (cachedLoad !== undefined) {
    return cachedLoad;
  }
  const loadPromise = loadSignalsModuleIntoCachedTempDir(options, cacheKey);
  cachedSignalsModuleLoads.set(cacheKey, loadPromise);
  return loadPromise;
}

async function loadSignalsModuleIntoCachedTempDir(options, cacheKey) {
  const tempDir = await mkdtemp(path.join(tmpdir(), "forge-signal-product-"));
  cachedSignalsModuleTempDirs.add(tempDir);
  try {
    const filesToCopy = [
      [
        "product/authoring_option_validation.ts",
        "product/authoring_option_validation.js",
      ],
      [
        "product/canonical_diagnostic_digest.ts",
        "product/canonical_diagnostic_digest.js",
      ],
      [
        "product/entrypoint/construction/entrypoint_construction.ts",
        "product/entrypoint/construction/entrypoint_construction.js",
      ],
      [
        "product/entrypoint/bridge/worker_runtime_bridge.ts",
        "product/entrypoint/bridge/worker_runtime_bridge.js",
      ],
      [
        "product/entrypoint/bridge/worker_runtime_envelope_normalization.ts",
        "product/entrypoint/bridge/worker_runtime_envelope_normalization.js",
      ],
      [
        "product/entrypoint/bridge/worker_runtime_bridge_worker.ts",
        "product/entrypoint/bridge/worker_runtime_bridge_worker.js",
      ],
      [
        "product/entrypoint/worker_first_callable_signals.ts",
        "product/entrypoint/worker_first_callable_signals.js",
      ],
      [
        "product/entrypoint/worker_first_async_transaction.ts",
        "product/entrypoint/worker_first_async_transaction.js",
      ],
      [
        "product/entrypoint/worker_first_async_input.ts",
        "product/entrypoint/worker_first_async_input.js",
      ],
      [
        "product/entrypoint/worker_first_async_linked.ts",
        "product/entrypoint/worker_first_async_linked.js",
      ],
      [
        "product/entrypoint/worker_first_async_readable.ts",
        "product/entrypoint/worker_first_async_readable.js",
      ],
      [
        "product/entrypoint/worker_first_callback_tracking.ts",
        "product/entrypoint/worker_first_callback_tracking.js",
      ],
      [
        "product/entrypoint/worker_first_async_recipe.ts",
        "product/entrypoint/worker_first_async_recipe.js",
      ],
      [
        "product/entrypoint/worker_first_declarative_expr.ts",
        "product/entrypoint/worker_first_declarative_expr.js",
      ],
      [
        "product/entrypoint/worker_first_sync_authoring.ts",
        "product/entrypoint/worker_first_sync_authoring.js",
      ],
      [
        "product/entrypoint/worker_first_scope_handle.ts",
        "product/entrypoint/worker_first_scope_handle.js",
      ],
      [
        "product/entrypoint/worker_first_scope_identity.ts",
        "product/entrypoint/worker_first_scope_identity.js",
      ],
      [
        "product/entrypoint/worker_first_public_input_support.ts",
        "product/entrypoint/worker_first_public_input_support.js",
      ],
      [
        "product/entrypoint/worker_first_form_factory.ts",
        "product/entrypoint/worker_first_form_factory.js",
      ],
      [
        "product/entrypoint/worker_first_resource_namespace.ts",
        "product/entrypoint/worker_first_resource_namespace.js",
      ],
      [
        "product/entrypoint/worker_first_explicit_spec_namespace.ts",
        "product/entrypoint/worker_first_explicit_spec_namespace.js",
      ],
      [
        "product/entrypoint/worker_first_authoring_namespace.ts",
        "product/entrypoint/worker_first_authoring_namespace.js",
      ],
      [
        "product/entrypoint/worker_first_root_graph.ts",
        "product/entrypoint/worker_first_root_graph.js",
      ],
      [
        "product/entrypoint/worker_first_root_graph_mutation.ts",
        "product/entrypoint/worker_first_root_graph_mutation.js",
      ],
      [
        "product/entrypoint/worker_first_root_graph_support.ts",
        "product/entrypoint/worker_first_root_graph_support.js",
      ],
      [
        "product/entrypoint/worker_first_root_history_lifecycle.ts",
        "product/entrypoint/worker_first_root_history_lifecycle.js",
      ],
      [
        "product/entrypoint/worker_first_root_mutation.ts",
        "product/entrypoint/worker_first_root_mutation.js",
      ],
      [
        "product/entrypoint/worker_first_root_runtime_replacement.ts",
        "product/entrypoint/worker_first_root_runtime_replacement.js",
      ],
      [
        "product/entrypoint/worker_first_host_capabilities.ts",
        "product/entrypoint/worker_first_host_capabilities.js",
      ],
      [
        "product/entrypoint/worker_first_denied_host_capabilities.ts",
        "product/entrypoint/worker_first_denied_host_capabilities.js",
      ],
      [
        "product/entrypoint/worker_first_host_capability_events.ts",
        "product/entrypoint/worker_first_host_capability_events.js",
      ],
      [
        "product/entrypoint/worker_first_host_dependency_refresh.ts",
        "product/entrypoint/worker_first_host_dependency_refresh.js",
      ],
      [
        "product/entrypoint/worker_first_persistence_host_capability.ts",
        "product/entrypoint/worker_first_persistence_host_capability.js",
      ],
      [
        "product/entrypoint/worker_first_root_session.ts",
        "product/entrypoint/worker_first_root_session.js",
      ],
      [
        "product/entrypoint/worker_first_root_observations.ts",
        "product/entrypoint/worker_first_root_observations.js",
      ],
      [
        "product/entrypoint/worker_first_root_imported_graph.ts",
        "product/entrypoint/worker_first_root_imported_graph.js",
      ],
      [
        "product/entrypoint/worker_first_root_cached_facades.ts",
        "product/entrypoint/worker_first_root_cached_facades.js",
      ],
      [
        "product/entrypoint/worker_first_root_history.ts",
        "product/entrypoint/worker_first_root_history.js",
      ],
      [
        "product/entrypoint/worker_first_projection_session.ts",
        "product/entrypoint/worker_first_projection_session.js",
      ],
      [
        "product/entrypoint/worker_first_diagnostics.ts",
        "product/entrypoint/worker_first_diagnostics.js",
      ],
      [
        "product/entrypoint/worker_first_adapters.ts",
        "product/entrypoint/worker_first_adapters.js",
      ],
      [
        "product/entrypoint/worker_first_history.ts",
        "product/entrypoint/worker_first_history.js",
      ],
      [
        "product/entrypoint/sessions/support/worker_cached_value.ts",
        "product/entrypoint/sessions/support/worker_cached_value.js",
      ],
      [
        "product/entrypoint/sessions/support/authored/worker_first_authored_input_state.ts",
        "product/entrypoint/sessions/support/authored/worker_first_authored_input_state.js",
      ],
      [
        "product/entrypoint/sessions/support/authored/worker_first_authored_readable_state.ts",
        "product/entrypoint/sessions/support/authored/worker_first_authored_readable_state.js",
      ],
      [
        "product/entrypoint/sessions/support/authored/worker_first_authored_callback_authoring.ts",
        "product/entrypoint/sessions/support/authored/worker_first_authored_callback_authoring.js",
      ],
      [
        "product/entrypoint/sessions/support/authored/worker_first_host_dependency_records.ts",
        "product/entrypoint/sessions/support/authored/worker_first_host_dependency_records.js",
      ],
      [
        "product/entrypoint/sessions/support/authored/worker_first_host_dependency_report.ts",
        "product/entrypoint/sessions/support/authored/worker_first_host_dependency_report.js",
      ],
      [
        "product/entrypoint/sessions/support/authored/worker_first_authored_readable_refresh.ts",
        "product/entrypoint/sessions/support/authored/worker_first_authored_readable_refresh.js",
      ],
      [
        "product/entrypoint/sessions/support/worker_first_graph_inspection.ts",
        "product/entrypoint/sessions/support/worker_first_graph_inspection.js",
      ],
      [
        "product/entrypoint/sessions/support/worker_first_imported_graph_support.ts",
        "product/entrypoint/sessions/support/worker_first_imported_graph_support.js",
      ],
      [
        "product/entrypoint/sessions/support/worker_first_history_proofs.ts",
        "product/entrypoint/sessions/support/worker_first_history_proofs.js",
      ],
      [
        "product/entrypoint/sessions/support/authored/worker_first_root_authored_runtime.ts",
        "product/entrypoint/sessions/support/authored/worker_first_root_authored_runtime.js",
      ],
      [
        "product/entrypoint/sessions/support/worker_first_root_import_context.ts",
        "product/entrypoint/sessions/support/worker_first_root_import_context.js",
      ],
      [
        "product/entrypoint/worker_first_imported_graph.ts",
        "product/entrypoint/worker_first_imported_graph.js",
      ],
      [
        "product/entrypoint/sessions/support/worker_first_published_graph_definition.ts",
        "product/entrypoint/sessions/support/worker_first_published_graph_definition.js",
      ],
      [
        "product/entrypoint/worker_first_published_graph.ts",
        "product/entrypoint/worker_first_published_graph.js",
      ],
      [
        "product/entrypoint/worker_first_published_graph_surface.ts",
        "product/entrypoint/worker_first_published_graph_surface.js",
      ],
      [
        "product/entrypoint/worker_first_published_graph_transaction.ts",
        "product/entrypoint/worker_first_published_graph_transaction.js",
      ],
      [
        "product/entrypoint/worker_first_root_graph_inspection.ts",
        "product/entrypoint/worker_first_root_graph_inspection.js",
      ],
      [
        "product/entrypoint/sessions/support/worker_first_published_graph_mutation.ts",
        "product/entrypoint/sessions/support/worker_first_published_graph_mutation.js",
      ],
      [
        "product/imported_graph_surface_support.ts",
        "product/imported_graph_surface_support.js",
      ],
      ["product/imported_graphs.ts", "product/imported_graphs.js"],
      ["product/published_graphs.ts", "product/published_graphs.js"],
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
      ["product/linked_definition.ts", "product/linked_definition.js"],
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
        `export { default } from ${JSON.stringify(realRawSurfaceUrl)};\nexport * from ${JSON.stringify(realRawSurfaceUrl)};\n`,
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
    const [loadedSignals, loadedEntrypointConstruction, loadedWorkerRuntimeBridge] =
      await Promise.all([
        import(moduleUrl.href),
        import(
          pathToFileURL(
            path.join(
              tempDir,
              "product",
              "entrypoint",
              "construction",
              "entrypoint_construction.js",
            ),
          ).href
        ),
        import(
          pathToFileURL(
            path.join(
              tempDir,
              "product",
              "entrypoint",
              "bridge",
              "worker_runtime_bridge.js",
            ),
          ).href
        ),
      ]);
    return {
      ...loadedSignals,
      ...loadedEntrypointConstruction,
      ...loadedWorkerRuntimeBridge,
      importProductModule(relativePath) {
        return import(
          pathToFileURL(
            path.join(tempDir, "product", relativePath),
          ).href
        );
      },
      cleanup: async () => {},
    };
  } catch (error) {
    cachedSignalsModuleLoads.delete(cacheKey);
    cachedSignalsModuleTempDirs.delete(tempDir);
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
