import { requirePlainObject } from "../../host_capability_declarations.js";

export function normalizeCreateSignalsAssets(assets, deployment) {
  if (assets === undefined) {
    return null;
  }
  const normalizedAssets = requirePlainObject(
    assets,
    "createSignals assets must be an object when provided",
  );
  const { wasmUrl, workerUrl, ...unknownOptions } = normalizedAssets;
  const unknownKeys = Object.keys(unknownOptions);
  if (unknownKeys.length > 0) {
    throw new TypeError(
      `createSignals assets do not support: ${unknownKeys.join(", ")}`,
    );
  }
  const hasWasm = wasmUrl !== undefined;
  const hasWorker = workerUrl !== undefined;
  if (!hasWasm && !hasWorker) {
    throw new TypeError(
      "createSignals assets must include wasmUrl and/or workerUrl",
    );
  }
  if (deployment === "mainThreadCompatibility") {
    if (hasWorker) {
      throw new TypeError(
        'createSignals assets.workerUrl is only valid with deployment: "workerFirst"',
      );
    }
    return Object.freeze({
      wasmUrl: normalizeAssetUrl(wasmUrl, "assets.wasmUrl"),
      workerUrl: null,
    });
  }
  if (!hasWasm || !hasWorker) {
    throw new TypeError(
      'createSignals workerFirst assets require both wasmUrl and workerUrl',
    );
  }
  return Object.freeze({
    wasmUrl: normalizeAssetUrl(wasmUrl, "assets.wasmUrl"),
    workerUrl: normalizeAssetUrl(workerUrl, "assets.workerUrl"),
  });
}

export function normalizeAssetUrl(value, label) {
  if (value instanceof URL) {
    return value;
  }
  if (typeof value === "string") {
    if (value.length === 0) {
      throw new TypeError(`${label} must not be an empty string`);
    }
    try {
      return new URL(value);
    } catch {
      // Relative bundler-emitted paths remain legal; resolve later against the
      // receiving module's import.meta.url.
      return value;
    }
  }
  throw new TypeError(`${label} must be a string or URL`);
}
