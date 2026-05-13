import type {
  ResourceMutationResponseSubmittedTargetBasis,
  ResourceMutationResponseTargetStaleness,
} from "./resource_mutation_response_target_basis.js";
import type {
  ResourceMutationResponseLifecycleProof,
  ResourceMutationResponseTargetEffectProof,
} from "./resource_mutation_response_lifecycle_proof.js";
export type {
  ResourceMutationResponseSubmittedTargetBasis,
  ResourceMutationResponseTargetStaleness,
} from "./resource_mutation_response_target_basis.js";
export type {
  ResourceMutationResponseLifecycleProof,
  ResourceMutationResponseLifecycleProofEntry,
  ResourceMutationResponseMergeRebaseProof,
  ResourceMutationResponseRollbackProof,
  ResourceMutationResponseTargetEffectProof,
} from "./resource_mutation_response_lifecycle_proof.js";

export interface ResourceMutationResponseLineDigest {
  readonly familyId: string;
  readonly runtimeLineId: string;
  readonly canonicalKey: string;
}

export interface ResourceMutationResponseRequestDigest {
  readonly correlationId: string | null;
  readonly branchId: string | number | null;
  readonly basisId: string | null;
  readonly requestPath: string | null;
  readonly url: string | null;
}

export interface ResourceMutationResponsePayloadDigest {
  readonly topology: string;
  readonly readResponseLensSource: string;
  readonly readResponseLensDigest: string;
  readonly mutationResponseLensDigest: string;
  readonly payloadDigest: string;
}

export interface ResourceMutationResponseCounters {
  readonly planningBreadth: number;
  readonly responseExtractionBreadth: number;
  readonly targetLookupBreadth: number;
  readonly targetFanoutBreadth: number;
  readonly fallbackBreadth: number;
  readonly executionBreadth: number;
  readonly diagnosticExtractionBreadth: number;
  readonly confirmationClassificationBreadth: number;
  readonly lifecycleProofBreadth: number;
  readonly targetBasisSnapshotBreadth: number;
  readonly staleTargetDenialBreadth: number;
  readonly appliedTargetBreadth?: number;
}

export type ResourceMutationResponseFallbackKind =
  | "refetchRequired"
  | "deliveryAwaited"
  | "partialReconciliation"
  | "unsupportedTarget";

export type ResourceMutationResponseDetailReconciliationDeclaration =
  | {
      readonly kind: "replace";
    }
  | {
      readonly kind: "field";
      readonly field: string;
    }
  | {
      readonly kind: "region";
      readonly region: string;
    }
  | {
      readonly kind: "jsonPath";
      readonly path: string;
    };

export interface ResourceMutationResponseCollectionReconciliationDeclaration {
  readonly kind: "item";
}

export interface ResourceMutationResponseSummaryReconciliationDeclaration {
  readonly kind: "summary";
  readonly summary: string;
}

export interface ResourceMutationResponseDiagnosticDeclaration {
  readonly kind: "validation" | "warnings";
  readonly field: string;
}

export interface ResourceMutationResponseTargetFamily {
  invalidate(params: unknown): boolean;
  invalidateAll(): number;
  line(params: unknown): {
    descriptor(): {
      readonly family: {
        readonly familyId: string;
        readonly kind: "detail" | "collection" | "paged";
      };
      readonly canonicalParams: {
        readonly canonicalKey: string;
      };
      readonly runtimeLineId: string;
    };
  };
}

export type ResourceMutationResponseTargetFamilyParams<
  TFamily extends ResourceMutationResponseTargetFamily,
> = TFamily extends {
  line(params: infer TParams): {
    descriptor(): unknown;
  };
}
  ? TParams
  : never;

export interface ResourceMutationResponseTargetDeclaration<
  TMutationParams,
  TFamily extends ResourceMutationResponseTargetFamily,
> {
  readonly family: TFamily;
  readonly params: (
    mutationParams: TMutationParams,
  ) => ResourceMutationResponseTargetFamilyParams<TFamily>;
  readonly fallback: ResourceMutationResponseFallbackKind;
  readonly detail?: ResourceMutationResponseDetailReconciliationDeclaration;
  readonly collection?: ResourceMutationResponseCollectionReconciliationDeclaration;
  readonly summary?: ResourceMutationResponseSummaryReconciliationDeclaration;
}

export type ResourceMutationResponseAnyTargetDeclaration<TMutationParams> =
  ResourceMutationResponseTargetDeclaration<
    TMutationParams,
    ResourceMutationResponseTargetFamily
  >;

export type ResourceMutationResponseFallbackTargetDeclaration<TMutationParams> =
  Omit<
    ResourceMutationResponseAnyTargetDeclaration<TMutationParams>,
    "detail" | "collection" | "summary"
  > & {
    readonly detail?: never;
    readonly collection?: never;
    readonly summary?: never;
  };

export interface ResourceMutationResponseTargetLineDigest {
  readonly familyKind: "detail" | "collection" | "paged";
  readonly familyId: string;
  readonly canonicalKey: string;
  readonly runtimeLineId: string | null;
  readonly residency: "declared" | "resident";
}

export interface ResourceMutationResponseTargetFallbackDigest {
  readonly kind: ResourceMutationResponseFallbackKind;
  readonly detail: string;
}

export interface ResourceMutationResponseTargetReconciliationDigest {
  readonly kind: "replace" | "field" | "region" | "jsonPath" | "item" | "summary";
  readonly itemId: string | null;
  readonly field: string | null;
  readonly region: string | null;
  readonly path: string | null;
  readonly summary: string | null;
  readonly targetDigest: string;
}

export interface ResourceMutationResponseFallbackExecutionArtifact {
  readonly artifactId: string;
  readonly targetId: string;
  readonly kind: "fallback";
  readonly fallback: ResourceMutationResponseFallbackKind;
  readonly familyKind: "detail" | "collection" | "paged";
  readonly familyId: string;
  readonly canonicalKey: string;
  readonly runtimeLineId: string | null;
  readonly residency: "declared" | "resident";
  readonly submittedTarget: ResourceMutationResponseSubmittedTargetBasis | null;
  readonly staleness: ResourceMutationResponseTargetStaleness | null;
  readonly detail: string;
}

export interface ResourceMutationResponseExactDetailExecutionArtifact {
  readonly artifactId: string;
  readonly targetId: string;
  readonly kind: "exactDetail";
  readonly scope: "line" | "field" | "region" | "jsonPath";
  readonly familyKind: "detail" | "collection" | "paged";
  readonly familyId: string;
  readonly canonicalKey: string;
  readonly runtimeLineId: string | null;
  readonly residency: "declared" | "resident";
  readonly packetId: string;
  readonly submittedTarget: ResourceMutationResponseSubmittedTargetBasis | null;
  readonly staleness: null;
  readonly itemId: null;
  readonly field: string | null;
  readonly region: string | null;
  readonly path: string | null;
  readonly summary: null;
  readonly outcomeKind?: "applied";
  readonly deliveryKind?: "replace" | "patch" | null;
  readonly deliveryScope?:
    | "line"
    | "field"
    | "region"
    | "jsonPath"
    | "item"
    | "aspect"
    | "summary"
    | "basis"
    | "invalidate"
    | null;
  readonly effectId?: string | null;
  readonly effectProof?: ResourceMutationResponseTargetEffectProof | null;
  readonly targetVisibleValueVersion?: number;
}

export interface ResourceMutationResponseExactCollectionItemExecutionArtifact {
  readonly artifactId: string;
  readonly targetId: string;
  readonly kind: "exactCollectionItem";
  readonly scope: "item";
  readonly familyKind: "collection" | "paged";
  readonly familyId: string;
  readonly canonicalKey: string;
  readonly runtimeLineId: string | null;
  readonly residency: "declared" | "resident";
  readonly packetId: string;
  readonly submittedTarget: ResourceMutationResponseSubmittedTargetBasis | null;
  readonly staleness: null;
  readonly itemId: string;
  readonly field: null;
  readonly region: null;
  readonly path: null;
  readonly summary: null;
  readonly outcomeKind?: "applied";
  readonly deliveryKind?: "patch" | null;
  readonly deliveryScope?: "item" | null;
  readonly effectId?: string | null;
  readonly effectProof?: ResourceMutationResponseTargetEffectProof | null;
  readonly targetVisibleValueVersion?: number;
}

export interface ResourceMutationResponseExactSummaryExecutionArtifact {
  readonly artifactId: string;
  readonly targetId: string;
  readonly kind: "exactSummary";
  readonly scope: "summary";
  readonly familyKind: "collection" | "paged";
  readonly familyId: string;
  readonly canonicalKey: string;
  readonly runtimeLineId: string | null;
  readonly residency: "declared" | "resident";
  readonly packetId: string;
  readonly submittedTarget: ResourceMutationResponseSubmittedTargetBasis | null;
  readonly staleness: null;
  readonly itemId: null;
  readonly field: null;
  readonly region: null;
  readonly path: null;
  readonly summary: string;
  readonly summaryScope: "line" | "pageWindow" | null;
  readonly outcomeKind?: "applied";
  readonly deliveryKind?: "patch" | null;
  readonly deliveryScope?: "summary" | null;
  readonly effectId?: string | null;
  readonly effectProof?: ResourceMutationResponseTargetEffectProof | null;
  readonly targetVisibleValueVersion?: number;
}

export interface ResourceMutationResponseTargetDigest {
  readonly targetId: string;
  readonly family: {
    readonly kind: "detail" | "collection" | "paged";
    readonly familyId: string;
  };
  readonly line: ResourceMutationResponseTargetLineDigest;
  readonly fallback: ResourceMutationResponseTargetFallbackDigest;
  readonly reconciliation: ResourceMutationResponseTargetReconciliationDigest | null;
  readonly submittedTarget: ResourceMutationResponseSubmittedTargetBasis | null;
  readonly execution:
    | ResourceMutationResponseFallbackExecutionArtifact
    | ResourceMutationResponseExactDetailExecutionArtifact
    | ResourceMutationResponseExactCollectionItemExecutionArtifact
    | ResourceMutationResponseExactSummaryExecutionArtifact;
  readonly targetDigest: string;
}

export interface ResourceMutationResponseDiagnosticFact {
  readonly diagnosticId: string;
  readonly kind: "validation" | "warnings";
  readonly field: string;
  readonly value: unknown;
  readonly valueDigest: string;
}

export interface ResourceMutationResponseDiagnosticsDigest {
  readonly entries: readonly ResourceMutationResponseDiagnosticFact[];
  readonly count: number;
  readonly digest: string;
}

export type ResourceMutationResponseConfirmationKind =
  | "preservedOptimisticTruth"
  | "consumedCanonicalTruth"
  | "partialCanonicalTruth"
  | "refetchRequired"
  | "deliveryAwaited";

export interface ResourceMutationResponseConfirmationClassification {
  readonly kind: ResourceMutationResponseConfirmationKind;
  readonly detail: string;
  readonly exactTargetCount: number;
  readonly fallbackTargetCount: number;
  readonly diagnosticCount: number;
  readonly fallbackKinds: readonly ResourceMutationResponseFallbackKind[];
  readonly digest: string;
}

export type ResourceMutationResponseExecutionArtifact =
  | ResourceMutationResponseFallbackExecutionArtifact
  | ResourceMutationResponseExactDetailExecutionArtifact
  | ResourceMutationResponseExactCollectionItemExecutionArtifact
  | ResourceMutationResponseExactSummaryExecutionArtifact;

export interface ResourceMutationResponsePlan {
  readonly version: "resource-mutation-response-plan-v1";
  readonly source: string;
  readonly planId: string;
  readonly route: string;
  readonly method: "POST" | "PUT" | "DELETE";
  readonly line: ResourceMutationResponseLineDigest;
  readonly request: ResourceMutationResponseRequestDigest;
  readonly submittedTargets: readonly ResourceMutationResponseSubmittedTargetBasis[];
  readonly response: ResourceMutationResponsePayloadDigest;
  readonly confirmation: ResourceMutationResponseConfirmationClassification;
  readonly lifecycleProof: ResourceMutationResponseLifecycleProof;
  readonly diagnostics: ResourceMutationResponseDiagnosticsDigest;
  readonly targets: readonly ResourceMutationResponseTargetDigest[];
  readonly targetCount: number;
  readonly atomicity: "zeroTargets" | "singleTarget" | "allOrNone";
  readonly targetDigest: string;
  readonly fallbackDigest: string;
  readonly executionArtifacts: readonly ResourceMutationResponseExecutionArtifact[];
  readonly executionDigest: string;
  readonly counters: ResourceMutationResponseCounters;
}
