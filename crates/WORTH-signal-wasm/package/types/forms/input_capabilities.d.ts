import type { FormInputAdapterCapabilitySet, FormInputAdapterTier } from "./core.js";

export interface FormInputCapabilityArtifact {
  readonly field: string;
  readonly path: string;
  readonly tier: FormInputAdapterTier;
  readonly posture: "ready" | "unavailable";
  readonly reason: string;
  readonly capabilities: FormInputAdapterCapabilitySet;
  readonly unavailableCapabilities: ReadonlyArray<{
    readonly capability: string;
    readonly reason: string;
  }>;
  readonly capabilityDigest: string;
}

export interface FormInputCapabilitiesReport {
  readonly fields: ReadonlyArray<FormInputCapabilityArtifact>;
  readonly summary: {
    readonly total: number;
    readonly unavailableFields: number;
    readonly rawInputUnavailableFields: number;
    readonly commitBoundaryUnavailableFields: number;
    readonly compositionUnavailableFields: number;
    readonly focusUnavailableFields: number;
    readonly labelTrackUnavailableFields: number;
    readonly helpTrackUnavailableFields: number;
    readonly messageTrackUnavailableFields: number;
    readonly minHeightSyncUnavailableFields: number;
    readonly responsiveTokenUnavailableFields: number;
  };
  readonly counters: {
    readonly costBasis: "declaredInputAdapterCapabilityScan";
    readonly incrementalStatus: "notIncremental";
    readonly fields: number;
    readonly signalNativeFields: number;
    readonly signalBridgeFields: number;
    readonly externalImperativeFields: number;
    readonly unavailableFields: number;
    readonly unavailableCapabilities: number;
  };
  readonly digest: string;
}
