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
        find: "worth-signal-wasm/react",
        replacement: path.resolve(__dirname, "../../crates/worth-signal-wasm/react/index.ts"),
      },
      {
        find: "worth-signal-wasm",
        replacement: path.resolve(__dirname, "./src/runtime/WORTHSignalWasmBridge.ts"),
      },
      {
        find: "@WORTH/signal/wasm",
        replacement: path.resolve(__dirname, "../../crates/worth-signal-wasm/pkg/worth_signal_wasm.js"),
      },
      {
        find: "@WORTH/signal",
        replacement: path.resolve(__dirname, "../../packages/worth-signal/src/index.ts"),
      },
    ],
    preserveSymlinks: true,
  },
});
