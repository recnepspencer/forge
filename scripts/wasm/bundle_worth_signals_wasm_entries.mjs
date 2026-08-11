/**
 * Named entry graph and externals policy for Track 5 publish-time bundling.
 */
import path from "node:path";

export const BUNDLED_JS_FILE_CAP = 40;

/** Stable publish chunk path policy (no content hashes). */
export const CHUNK_NAME_PATTERN = "chunks/[name]";

export const WORKER_ENTRY_RELATIVE_PATH =
  "product/entrypoint/bridge/worker_runtime_bridge_worker.js";

export const BRIDGE_ENTRY_RELATIVE_PATH =
  "product/entrypoint/bridge/worker_runtime_bridge.js";

export function buildProductEntryPoints(transpileRoot) {
  // Keys are posix-style outdir-relative paths (esbuild object entry form).
  return {
    index: path.join(transpileRoot, "index.js"),
    raw_surface: path.join(transpileRoot, "raw_surface.js"),
    "product/entrypoint/bridge/worker_runtime_bridge": path.join(
      transpileRoot,
      BRIDGE_ENTRY_RELATIVE_PATH,
    ),
    "product/entrypoint/bridge/worker_runtime_bridge_worker": path.join(
      transpileRoot,
      WORKER_ENTRY_RELATIVE_PATH,
    ),
  };
}

export function buildReactEntryPoints(reactEmitRoot) {
  return {
    "react/index": path.join(reactEmitRoot, "index.js"),
  };
}

export const PRODUCT_BUNDLE_EXTERNALS = [
  "./worth_signal_wasm.js",
  "./worth_signal_wasm_bg.js",
  "worth_signal_wasm.js",
  "worth_signal_wasm_bg.js",
  "node:worker_threads",
  "node:fs/promises",
  "node:fs",
  "node:path",
  "node:url",
  "node:module",
  "node:os",
  "node:process",
];

export const REACT_BUNDLE_EXTERNALS = [
  "react",
  "react/jsx-runtime",
  "react/jsx-dev-runtime",
];

export function createNodeBuiltinExternalPlugin() {
  return {
    name: "worth-external-node-builtins",
    setup(build) {
      build.onResolve({ filter: /^node:/ }, (args) => ({
        path: args.path,
        external: true,
      }));
    },
  };
}

export function createWasmGlueExternalPlugin() {
  return {
    name: "worth-external-wasm-glue",
    setup(build) {
      build.onResolve({ filter: /worth_signal_wasm(_bg)?\.js$/ }, (args) => {
        const normalized = args.path.replaceAll("\\", "/");
        if (
          normalized === "./worth_signal_wasm.js" ||
          normalized === "./worth_signal_wasm_bg.js" ||
          normalized.endsWith("/worth_signal_wasm.js") ||
          normalized.endsWith("/worth_signal_wasm_bg.js") ||
          normalized === "worth_signal_wasm.js" ||
          normalized === "worth_signal_wasm_bg.js"
        ) {
          // Package-root relative imports from raw_surface / workers.
          const basename = normalized.split("/").pop();
          return {
            path: `./${basename}`,
            external: true,
          };
        }
        return undefined;
      });
    },
  };
}
