import { createSignals } from "worth-signals-wasm";
import wasmUrl from "worth-signals-wasm/wasm?url";
import workerUrl from "worth-signals-wasm/worker?worker&url";

type DemoCreateSignalsOptions = Parameters<typeof createSignals>[0];

/**
 * Demo construction uses the portable bundler asset recipe so the site matches
 * the published consumer guidance (not only Vite 8 zero-config defaults).
 */
export function createDemoSignals(options?: DemoCreateSignalsOptions) {
  const deployment = options?.deployment ?? "workerFirst";
  if (deployment === "mainThreadCompatibility") {
    return createSignals({
      ...options,
      assets: { wasmUrl },
    });
  }
  return createSignals({
    ...options,
    assets: { wasmUrl, workerUrl },
  });
}
