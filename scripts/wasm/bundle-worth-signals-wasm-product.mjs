/**
 * Track 5: multi-entry esbuild emit for worth-signals-wasm product + react.
 *
 * Bridge and worker are bundled without code-splitting so
 * `new URL("./worker_runtime_bridge_worker.js", import.meta.url)` keeps executing
 * in a module colocated with the worker file.
 */
import { createRequire } from "node:module";
import path from "node:path";
import { fileURLToPath } from "node:url";

import {
  BUNDLED_JS_FILE_CAP,
  BRIDGE_ENTRY_RELATIVE_PATH,
  CHUNK_NAME_PATTERN,
  PRODUCT_BUNDLE_EXTERNALS,
  REACT_BUNDLE_EXTERNALS,
  WORKER_ENTRY_RELATIVE_PATH,
  createNodeBuiltinExternalPlugin,
  createWasmGlueExternalPlugin,
} from "./bundle_worth_signals_wasm_entries.mjs";

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(scriptDir, "../..");
const crateRequire = createRequire(
  path.join(repoRoot, "crates/worth-signal-wasm/package.json"),
);

export { BUNDLED_JS_FILE_CAP, BRIDGE_ENTRY_RELATIVE_PATH, WORKER_ENTRY_RELATIVE_PATH };

const PACKAGE_BRIDGE_IMPORT = "./product/entrypoint/bridge/worker_runtime_bridge.js";
const PACKAGE_WORKER_IMPORT = "./product/entrypoint/bridge/worker_runtime_bridge_worker.js";

function createBridgeExternalPlugin() {
  return {
    name: "worth-external-bridge-shell",
    setup(build) {
      build.onResolve({ filter: /worker_runtime_bridge(_worker)?\.js$/ }, (args) => {
        const normalized = args.path.replaceAll("\\", "/");
        if (normalized.includes("worker_runtime_bridge_worker.js")) {
          return {
            path: PACKAGE_WORKER_IMPORT,
            external: true,
          };
        }
        if (normalized.includes("worker_runtime_bridge.js")) {
          return {
            path: PACKAGE_BRIDGE_IMPORT,
            external: true,
          };
        }
        return undefined;
      });
    },
  };
}

function createRawSurfaceExternalPlugin() {
  return {
    name: "worth-external-raw-surface",
    setup(build) {
      // Keep the importer's relative path so the worker entry still resolves
      // ../../../raw_surface.js from product/entrypoint/bridge/.
      build.onResolve({ filter: /raw_surface\.js$/ }, (args) => ({
        path: args.path,
        external: true,
      }));
    },
  };
}

export async function bundleWorthSignalsWasmProduct(options) {
  const {
    transpileRoot,
    reactEmitRoot,
    pkgDir,
  } = options;
  const esbuild = crateRequire("esbuild");
  const sharedPlugins = [
    createNodeBuiltinExternalPlugin(),
    createWasmGlueExternalPlugin(),
  ];

  const workerResult = await esbuild.build({
    absWorkingDir: transpileRoot,
    entryPoints: [path.join(transpileRoot, WORKER_ENTRY_RELATIVE_PATH)],
    bundle: true,
    splitting: false,
    format: "esm",
    platform: "browser",
    target: ["es2022"],
    outfile: path.join(pkgDir, WORKER_ENTRY_RELATIVE_PATH),
    legalComments: "none",
    // Track 5 deferred minify; Track 6 QA takes the publish-size win.
    minify: true,
    sourcemap: false,
    logLevel: "warning",
    external: PRODUCT_BUNDLE_EXTERNALS,
    plugins: [...sharedPlugins, createRawSurfaceExternalPlugin()],
    write: true,
    metafile: true,
  });

  const bridgeResult = await esbuild.build({
    absWorkingDir: transpileRoot,
    entryPoints: [path.join(transpileRoot, BRIDGE_ENTRY_RELATIVE_PATH)],
    bundle: true,
    splitting: false,
    format: "esm",
    platform: "browser",
    target: ["es2022"],
    outfile: path.join(pkgDir, BRIDGE_ENTRY_RELATIVE_PATH),
    legalComments: "none",
    minify: true,
    sourcemap: false,
    logLevel: "warning",
    external: PRODUCT_BUNDLE_EXTERNALS,
    plugins: sharedPlugins,
    write: true,
    metafile: true,
  });

  // Facade entries stay at package root and externalize bridge shells / wasm glue
  // with package-root-relative import paths. Code-splitting into chunks/ would make
  // those `./product/...` and `./worth_signal_wasm.js` rewrite paths resolve beside
  // the chunk instead of the package root, so splitting stays off until a chunk-aware
  // relative-external strategy exists. Stable chunkNames remain the Track 5 policy
  // if splitting is re-enabled later.
  const facadeResult = await esbuild.build({
    absWorkingDir: transpileRoot,
    entryPoints: {
      index: path.join(transpileRoot, "index.js"),
      raw_surface: path.join(transpileRoot, "raw_surface.js"),
    },
    bundle: true,
    splitting: false,
    format: "esm",
    platform: "browser",
    target: ["es2022"],
    outdir: pkgDir,
    chunkNames: CHUNK_NAME_PATTERN,
    legalComments: "none",
    minify: true,
    sourcemap: false,
    logLevel: "warning",
    external: PRODUCT_BUNDLE_EXTERNALS,
    plugins: [...sharedPlugins, createBridgeExternalPlugin()],
    write: true,
    metafile: true,
  });

  const reactResult = await esbuild.build({
    absWorkingDir: path.resolve(reactEmitRoot),
    entryPoints: {
      "react/index": path.join(path.resolve(reactEmitRoot), "index.js"),
    },
    bundle: true,
    splitting: false,
    format: "esm",
    platform: "browser",
    target: ["es2022"],
    outdir: pkgDir,
    legalComments: "none",
    minify: true,
    sourcemap: false,
    logLevel: "warning",
    external: REACT_BUNDLE_EXTERNALS,
    jsx: "automatic",
    write: true,
    metafile: true,
  });

  return {
    workerMetafile: workerResult.metafile,
    bridgeMetafile: bridgeResult.metafile,
    facadeMetafile: facadeResult.metafile,
    reactMetafile: reactResult.metafile,
  };
}
