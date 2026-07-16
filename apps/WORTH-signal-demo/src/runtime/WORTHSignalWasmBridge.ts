import initializeRawSignals, * as rawSurface from "../../../../crates/worth-signal-wasm/pkg/raw_surface.js";
import type { Signals as RawSignals } from "../../../../crates/worth-signal-wasm/pkg/raw_surface.js";
import { resourcePolicyProfiles, wrapSignals } from "../../../../crates/worth-signal-wasm/package-src/product/signals.ts";

interface CompatibilityCreateSignalsOptions {
  deployment?: "workerFirst" | "mainThreadCompatibility";
  hostCapabilities?: unknown;
}

export { resourcePolicyProfiles };

const { createRawSignals } = rawSurface as unknown as {
  createRawSignals: () => RawSignals;
};

let initialization: Promise<undefined> | null = null;

async function ensureRuntimeInitialized(): Promise<void> {
  initialization ??= initializeRawSignals();
  await initialization;
}

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

  await ensureRuntimeInitialized();

  const productOptions = hostCapabilities === undefined ? undefined : { hostCapabilities };
  return wrapSignals(createRawSignals(), productOptions);
}
