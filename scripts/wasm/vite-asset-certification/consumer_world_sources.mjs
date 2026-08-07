/** Source files installed into the temporary Vite consumer world. */

import { buildProbeModule } from "./build_probe_module.mjs";

export { buildProbeModule };

export function buildProbeHtml() {
  return `<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <title>worth-signals-wasm Gate 0</title>
  </head>
  <body>
    <pre id="status">html-booted</pre>
    <script>
      window.__WORTH_GATE0__ = Object.freeze({
        phase: "html-booted",
        note: "waiting for module graph / optimizeDeps / createSignals",
      });
    </script>
    <script type="module" src="/src/probe.js"></script>
  </body>
</html>
`;
}

export function buildViteConfigSource(options) {
  const {
    forceOptimizeInclude = true,
    spaFallbackWasm = false,
  } = options;

  const optimizeBlock = forceOptimizeInclude
    ? `
  optimizeDeps: {
    include: ["worth-signals-wasm"],
  },`
    : "";

  const spaPlugin = spaFallbackWasm
    ? `
function spaFallbackWasmPlugin() {
  return {
    name: "gate0-spa-fallback-wasm",
    configureServer(server) {
      server.middlewares.use((req, res, next) => {
        const url = req.url ?? "";
        if (!url.includes(".wasm")) {
          next();
          return;
        }
        res.statusCode = 200;
        res.setHeader("Content-Type", "text/html; charset=utf-8");
        res.end("<!doctype html><html><body>gate0-spa-fallback</body></html>");
      });
    },
  };
}
`
    : "";

  const plugins = spaFallbackWasm ? "[spaFallbackWasmPlugin()]" : "[]";

  return `${spaPlugin}
import { defineConfig } from "vite";

export default defineConfig({
  plugins: ${plugins},${optimizeBlock}
  // worth-signals-wasm worker entry uses top-level await; IIFE worker builds fail.
  worker: {
    format: "es",
  },
  server: {
    strictPort: true,
    fs: {
      // Packed consumer worlds live under the OS temp directory; avoid 403s on
      // rebased node_modules asset URLs during optimizeDeps-served modules.
      strict: false,
      allow: [".."],
    },
  },
  preview: {
    strictPort: true,
  },
  build: {
    modulePreload: false,
  },
});
`;
}

export function buildConsumerPackageJson(options) {
  const { viteVersion, packageName } = options;
  return `${JSON.stringify({
    name: "worth-signals-wasm-gate0-consumer",
    private: true,
    type: "module",
    scripts: {
      dev: "vite",
      build: "vite build",
      preview: "vite preview",
    },
    dependencies: {
      [packageName]: "file:./package.tgz",
    },
    devDependencies: {
      vite: viteVersion,
      playwright: "1.51.0",
    },
  }, null, 2)}\n`;
}
