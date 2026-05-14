export type ResourceMutationResponseCloseoutCategory =
  | "supportedErgonomicHappyPath"
  | "supportedPreciseDenial"
  | "supportedTypedUnavailableFallback"
  | "intentionallyOutOfScope";

export type ResourceMutationResponseCloseoutLane =
  | "saveDetailReplace"
  | "saveDetailGranular"
  | "updateRelatedCollectionItem"
  | "updateRelatedSummary"
  | "createPlacement"
  | "createIdentityMigration"
  | "deleteExactRemoval"
  | "deleteCanonicalTombstone"
  | "multiFamilyReconciliation"
  | "refetchRequired"
  | "deliveryAwaited"
  | "partialReconciliation"
  | "placementUnavailable"
  | "deletionUnavailable"
  | "identityMigrationUnavailable"
  | "overclaimedDeclarations"
  | "hiddenBestEffortMutation"
  | "fallbackPresentedAsExact"
  | "advertisingDeniedAsErgonomics";

export type ResourceMutationResponseCloseoutProofLane =
  | "runtime"
  | "typeSurface"
  | "docs"
  | "closeout";

export interface ResourceMutationResponseCloseoutEvidence {
  readonly runtimeTests: readonly string[];
  readonly typeSurface: readonly string[];
  readonly docs: readonly string[];
  readonly closeout: readonly string[];
}

export interface ResourceMutationResponseCloseoutMatrixRow {
  readonly lane: ResourceMutationResponseCloseoutLane;
  readonly category: ResourceMutationResponseCloseoutCategory;
  readonly summary: string;
  readonly runtimeProof: true;
  readonly typeSurfaceProof: true;
  readonly docsProof: true;
  readonly closeoutProof: true;
  readonly evidence: ResourceMutationResponseCloseoutEvidence;
}

export interface ResourceMutationResponseDeferredErgonomic {
  readonly lane: string;
  readonly reason: string;
}

export interface ResourceMutationResponseCloseoutMatrix {
  readonly proofLanes: readonly ResourceMutationResponseCloseoutProofLane[];
  readonly rows: readonly ResourceMutationResponseCloseoutMatrixRow[];
  readonly deferredErgonomics: readonly ResourceMutationResponseDeferredErgonomic[];
}

export interface ResourceMutationResponses {
  closeoutMatrix(): ResourceMutationResponseCloseoutMatrix;
}

export const resourceMutationResponses: ResourceMutationResponses;
