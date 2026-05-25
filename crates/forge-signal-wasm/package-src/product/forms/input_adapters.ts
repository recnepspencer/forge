import { FormDeclarationError } from "./form_errors.js";

const ADAPTER_TIERS = new Set(["signalNative", "signalBridge", "externalImperative"]);

export function normalizeInputAdapter(options = {}) {
  const nestedInput = normalizeNestedInputOptions(options);
  const nestedAdapter = nestedInput?.adapter;
  if (nestedAdapter !== undefined && (options.inputAdapter !== undefined || options.adapter !== undefined)) {
    throw new FormDeclarationError(
      "form field input adapter should be declared in either input.adapter or adapter/inputAdapter, not both",
    );
  }
  const adapter = nestedAdapter ?? options.inputAdapter ?? options.adapter ?? {};
  if (adapter == null || typeof adapter !== "object") {
    throw new FormDeclarationError("form field input adapter must be an object", { adapter });
  }
  const tier = adapter.tier ?? options.tier ?? "signalNative";
  if (!ADAPTER_TIERS.has(tier)) {
    throw new FormDeclarationError("form field input adapter tier is not supported", { tier });
  }
  const capabilities = {
    reportsRawInput: adapter.reportsRawInput !== false,
    reportsCommitBoundary: adapter.reportsCommitBoundary !== false,
    reportsComposition: adapter.reportsComposition !== false,
    reportsFocus: adapter.reportsFocus !== false,
    supportsLabelTrack: adapter.supportsLabelTrack !== false,
    supportsHelpTrack: adapter.supportsHelpTrack !== false,
    supportsMessageTrack: adapter.supportsMessageTrack !== false,
    supportsMinHeightSync: adapter.supportsMinHeightSync !== false,
    supportsResponsiveTokens: adapter.supportsResponsiveTokens !== false,
  };
  const unavailable = Object.entries(capabilities)
    .filter(([, available]) => !available)
    .map(([capability]) => ({
      capability,
      reason: `${tier} adapter did not declare ${capability}`,
    }));
  return Object.freeze({
    tier,
    capabilities: Object.freeze(capabilities),
    unavailable: Object.freeze(unavailable),
  });
}

export function readFieldParse(options = {}) {
  const nestedInput = normalizeNestedInputOptions(options);
  if (nestedInput?.parse !== undefined && options.parse !== undefined) {
    throw new FormDeclarationError(
      "form field parse should be declared in either input.parse or parse, not both",
    );
  }
  const parse = nestedInput?.parse ?? options.parse ?? null;
  if (parse !== null && typeof parse !== "function") {
    throw new FormDeclarationError("form field parse must be a function", { parse });
  }
  return parse;
}

function normalizeNestedInputOptions(options) {
  const input = options.input;
  if (input === undefined) {
    return null;
  }
  if (input === null || typeof input !== "object" || Array.isArray(input)) {
    throw new FormDeclarationError("form field input metadata must be an object", { input });
  }
  return input;
}

