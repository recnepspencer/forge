import { FormDeclarationError } from "./form_errors.js";

const ADAPTER_TIERS = new Set(["signalNative", "signalBridge", "externalImperative"]);

export function normalizeInputAdapter(options = {}) {
  const adapter = options.inputAdapter ?? options.adapter ?? {};
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

