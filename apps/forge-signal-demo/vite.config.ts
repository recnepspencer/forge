import path from "node:path";

import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";

// https://vite.dev/config/
export default defineConfig({
  plugins: [react(), wasm()],
  worker: {
    format: "es",
    plugins: () => [wasm()],
  },
  resolve: {
    alias: [
      {
        find: "@forge/signal/wasm",
        replacement: path.resolve(__dirname, "../../packages/forge-signal/pkg/forge_signal_wasm.js"),
      },
      {
        find: "@forge/signal",
        replacement: path.resolve(__dirname, "../../packages/forge-signal/src/index.js"),
      },
    ],
    preserveSymlinks: true,
  },
});
