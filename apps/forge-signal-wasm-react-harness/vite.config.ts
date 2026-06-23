import { fileURLToPath, URL } from "node:url";

import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import wasm from "vite-plugin-wasm";

export default defineConfig({
  plugins: [react(), wasm()],
  resolve: {
    alias: {
      react: fileURLToPath(new URL("./node_modules/react/index.js", import.meta.url)),
      "react/jsx-runtime": fileURLToPath(
        new URL("./node_modules/react/jsx-runtime.js", import.meta.url),
      ),
      "react/jsx-dev-runtime": fileURLToPath(
        new URL("./node_modules/react/jsx-dev-runtime.js", import.meta.url),
      ),
      "react-dom": fileURLToPath(
        new URL("./node_modules/react-dom/index.js", import.meta.url),
      ),
    },
  },
  server: {
    fs: {
      allow: [fileURLToPath(new URL("../..", import.meta.url))],
    },
  },
  test: {
    environment: "jsdom",
    globals: true,
    server: {
      deps: {
        inline: ["@aust-group/forge-signal-wasm"],
      },
    },
  },
});
