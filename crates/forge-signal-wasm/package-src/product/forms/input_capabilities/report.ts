import { stableValueDigest } from "../values/value_paths.js";

export function readInputCapabilitiesReport(fieldDeclarations) {
  const fields = Object.freeze(fieldDeclarations.map((field) => {
    const unavailableCapabilities = Object.freeze(field.inputAdapter.unavailable.map((entry) => Object.freeze({
      capability: entry.capability,
      reason: entry.reason,
    })));
    const posture = unavailableCapabilities.length === 0 ? "ready" : "unavailable";
    return Object.freeze({
      field: field.id,
      path: field.path,
      tier: field.inputAdapter.tier,
      posture,
      reason: unavailableCapabilities.length === 0
        ? `${field.inputAdapter.tier} adapter supports the declared input capability surface`
        : `${field.inputAdapter.tier} adapter cannot honor ${unavailableCapabilities.map((entry) => entry.capability).join(", ")}`,
      capabilities: Object.freeze(field.inputAdapter.capabilities),
      unavailableCapabilities,
      capabilityDigest: stableValueDigest({
        field: field.id,
        tier: field.inputAdapter.tier,
        capabilities: field.inputAdapter.capabilities,
        unavailableCapabilities,
      }),
    });
  }));
  const summary = Object.freeze({
    total: fields.length,
    unavailableFields: fields.filter((field) => field.posture === "unavailable").length,
    rawInputUnavailableFields: fields.filter((field) => field.capabilities.reportsRawInput === false).length,
    commitBoundaryUnavailableFields: fields.filter((field) => field.capabilities.reportsCommitBoundary === false).length,
    compositionUnavailableFields: fields.filter((field) => field.capabilities.reportsComposition === false).length,
    focusUnavailableFields: fields.filter((field) => field.capabilities.reportsFocus === false).length,
    labelTrackUnavailableFields: fields.filter((field) => field.capabilities.supportsLabelTrack === false).length,
    helpTrackUnavailableFields: fields.filter((field) => field.capabilities.supportsHelpTrack === false).length,
    messageTrackUnavailableFields: fields.filter((field) => field.capabilities.supportsMessageTrack === false).length,
    minHeightSyncUnavailableFields: fields.filter((field) => field.capabilities.supportsMinHeightSync === false).length,
    responsiveTokenUnavailableFields: fields.filter((field) => field.capabilities.supportsResponsiveTokens === false).length,
  });
  const counters = Object.freeze({
    costBasis: "declaredInputAdapterCapabilityScan",
    incrementalStatus: "notIncremental",
    fields: fields.length,
    signalNativeFields: fields.filter((field) => field.tier === "signalNative").length,
    signalBridgeFields: fields.filter((field) => field.tier === "signalBridge").length,
    externalImperativeFields: fields.filter((field) => field.tier === "externalImperative").length,
    unavailableFields: summary.unavailableFields,
    unavailableCapabilities: fields.reduce((total, field) => total + field.unavailableCapabilities.length, 0),
  });
  return Object.freeze({
    fields,
    summary,
    counters,
    digest: stableValueDigest({ fields, summary, counters }),
  });
}
