import initializeRawSignals, * as rawSurface from "../../../../crates/worth-signal-wasm/pkg/raw_surface.js";
import type { Signals as RawSignals } from "../../../../crates/worth-signal-wasm/pkg/raw_surface.js";
import { createSignals as createPackagedSignals } from "../../../../crates/worth-signal-wasm/pkg/index.js";
import * as packagedSignalsModule from "../../../../crates/worth-signal-wasm/pkg/index.js";
import type {
  CallableSignals,
  CreateSignalsOptions,
} from "../../../../crates/worth-signal-wasm/pkg/index.js";
import type {
  resourcePatch as typedResourcePatch,
  resourcePolicyProfiles as typedResourcePolicyProfiles,
  wrapSignals as typedWrapSignals,
} from "../../../../crates/worth-signal-wasm/package-src/product/signals.ts";

// Every product export must come from the packaged pkg/ build. Mixing pkg with
// package-src modules creates a second brand-symbol instance, and packaged
// runtimes then reject package-src values (e.g. resourcePatch.dependsOn).
const { resourcePatch, resourcePolicyProfiles, wrapSignals } =
  packagedSignalsModule as unknown as {
    resourcePatch: typeof typedResourcePatch;
    resourcePolicyProfiles: typeof typedResourcePolicyProfiles;
    wrapSignals: typeof typedWrapSignals;
  };

export { resourcePatch, resourcePolicyProfiles };

const { createRawSignals } = rawSurface as unknown as {
  createRawSignals: () => RawSignals;
};

let initialization: Promise<undefined> | null = null;

async function ensureRuntimeInitialized(): Promise<void> {
  initialization ??= initializeRawSignals();
  await initialization;
}

export async function createSignals(
  options?: CreateSignalsOptions,
): Promise<CallableSignals> {
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
    return createPackagedSignals(normalizedOptions);
  }

  await ensureRuntimeInitialized();
  const productOptions = hostCapabilities === undefined ? undefined : { hostCapabilities };
  return wrapSignals(createRawSignals(), productOptions) as unknown as CallableSignals;
}
