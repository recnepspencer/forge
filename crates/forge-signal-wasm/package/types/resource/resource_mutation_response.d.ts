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
}

export type ResourceMutationResponseAnyTargetDeclaration<TMutationParams> =
  ResourceMutationResponseTargetDeclaration<
    TMutationParams,
    ResourceMutationResponseTargetFamily
  >;

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
  readonly kind: "replace" | "field" | "region" | "jsonPath";
  readonly field: string | null;
  readonly region: string | null;
  readonly path: string | null;
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
  readonly field: string | null;
  readonly region: string | null;
  readonly path: string | null;
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
  readonly execution:
    | ResourceMutationResponseFallbackExecutionArtifact
    | ResourceMutationResponseExactDetailExecutionArtifact;
  readonly targetDigest: string;
}

export type ResourceMutationResponseExecutionArtifact =
  | ResourceMutationResponseFallbackExecutionArtifact
  | ResourceMutationResponseExactDetailExecutionArtifact;

export interface ResourceMutationResponsePlan {
  readonly version: "resource-mutation-response-plan-v1";
  readonly source: string;
  readonly planId: string;
  readonly route: string;
  readonly method: "POST" | "PUT" | "DELETE";
  readonly line: ResourceMutationResponseLineDigest;
  readonly request: ResourceMutationResponseRequestDigest;
  readonly response: ResourceMutationResponsePayloadDigest;
  readonly targets: readonly ResourceMutationResponseTargetDigest[];
  readonly targetCount: number;
  readonly atomicity: "zeroTargets" | "singleTarget" | "allOrNone";
  readonly targetDigest: string;
  readonly fallbackDigest: string;
  readonly executionArtifacts: readonly ResourceMutationResponseExecutionArtifact[];
  readonly executionDigest: string;
  readonly counters: ResourceMutationResponseCounters;
}
