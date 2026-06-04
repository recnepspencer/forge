import { createRawSignals } from "../../../../crates/forge-signal-wasm/pkg/raw_surface.js";
import { resourcePolicyProfiles, wrapSignals } from "../../../../crates/forge-signal-wasm/package-src/product/signals.ts";

interface CompatibilityCreateSignalsOptions {
  deployment?: "workerFirst" | "mainThreadCompatibility";
  hostCapabilities?: unknown;
}

export { resourcePolicyProfiles };

export async function createSignals(
  options?: CompatibilityCreateSignalsOptions,
): Promise<ReturnType<typeof wrapSignals>> {
  const normalizedOptions = options ?? {};
  const { deployment = "workerFirst", hostCapabilities, ...unknownOptions } = normalizedOptions;
  const unknownKeys = Object.keys(unknownOptions);
  if (unknownKeys.length > 0) {
    throw new TypeError(`createSignals options do not support: ${unknownKeys.join(", ")}`);
  }
  if (deployment !== "mainThreadCompatibility" && deployment !== "workerFirst") {
    throw new TypeError('createSignals deployment must be "workerFirst" or "mainThreadCompatibility"');
  }

  if (deployment === "workerFirst") {
    return hostCapabilities === undefined
      ? wrapSignals(createRawSignals())
      : wrapSignals(createRawSignals(), { hostCapabilities });
  }

  return hostCapabilities === undefined
    ? wrapSignals(createRawSignals())
    : wrapSignals(createRawSignals(), { hostCapabilities });
}
