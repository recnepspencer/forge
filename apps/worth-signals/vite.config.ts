import path from "node:path";

import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";

const workspaceRoot = path.resolve(__dirname, "../..");

// https://vite.dev/config/
export default defineConfig({
  plugins: [react(), wasm()],
  server: {
    fs: {
      allow: [workspaceRoot],
    },
    headers: {
      "Cross-Origin-Opener-Policy": "same-origin",
      "Cross-Origin-Embedder-Policy": "require-corp",
    },
  },
  worker: {
    format: "es",
    plugins: () => [wasm()],
  },
  resolve: {
    alias: [
      {
        find: "forge-signal-wasm/react",
        replacement: path.resolve(__dirname, "../../crates/forge-signal-wasm/react/index.ts"),
      },
      {
        find: "forge-signal-wasm",
        replacement: path.resolve(__dirname, "./src/runtime/forgeSignalWasmBridge.ts"),
      },
      {
        find: "@forge/signal/wasm",
        replacement: path.resolve(__dirname, "../../crates/forge-signal-wasm/pkg/forge_signal_wasm.js"),
      },
      {
        find: "@forge/signal",
        replacement: path.resolve(__dirname, "../../packages/forge-signal/src/index.ts"),
      },
    ],
    preserveSymlinks: true,
  },
});
