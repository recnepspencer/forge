import path from "node:path";

import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";

import { gearCodeEvidencePlugin } from "./build/gear_code_evidence_plugin";
import { workerBundleIntegrityPlugin } from "./build/worker_bundle_integrity_plugin";

const workspaceRoot = path.resolve(__dirname, "../..");

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    gearCodeEvidencePlugin(__dirname),
    react(),
    wasm(),
    workerBundleIntegrityPlugin(),
  ],
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
