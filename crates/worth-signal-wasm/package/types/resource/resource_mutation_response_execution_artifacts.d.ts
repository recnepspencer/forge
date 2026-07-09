import type {
  ResourceMutationResponseSubmittedTargetBasis,
  ResourceMutationResponseTargetStaleness,
} from "./resource_mutation_response_target_basis.js";
import type { ResourceMutationResponseTargetEffectProof } from "./resource_mutation_response_lifecycle_proof.js";

export interface ResourceMutationResponsePartialReconciliationArtifact {
  readonly kind: "missingResponseField";
  readonly field: string;
  readonly digest: string;
}

export interface ResourceMutationResponseFallbackExecutionArtifact {
  readonly artifactId: string;
  readonly targetId: string;
  readonly kind: "fallback";
  readonly fallback: import("./resource_mutation_response.js").ResourceMutationResponseFallbackKind;
  readonly familyKind: "detail" | "collection" | "paged";
  readonly familyId: string;
  readonly canonicalKey: string;
  readonly runtimeLineId: string | null;
  readonly residency: "declared" | "resident";
  readonly submittedTarget: ResourceMutationResponseSubmittedTargetBasis | null;
  readonly staleness: ResourceMutationResponseTargetStaleness | null;
  readonly partial: ResourceMutationResponsePartialReconciliationArtifact | null;
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

export interface ResourceMutationResponseExactDetailInvalidationExecutionArtifact {
  readonly artifactId: string;
  readonly targetId: string;
  readonly kind: "exactDetailInvalidation";
  readonly scope: "line";
  readonly familyKind: "detail";
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
  readonly summary: null;
  readonly outcomeKind?: "applied";
  readonly deliveryKind?: "invalidate" | null;
  readonly deliveryScope?: "invalidate" | null;
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
  readonly placement: null;
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

export interface ResourceMutationResponseExactCollectionTombstoneExecutionArtifact {
  readonly artifactId: string;
  readonly targetId: string;
  readonly kind: "exactCollectionTombstone";
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
  readonly placement: null;
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

export interface ResourceMutationResponseExactCollectionInsertExecutionArtifact {
  readonly artifactId: string;
  readonly targetId: string;
  readonly kind: "exactCollectionInsert";
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
  readonly placement: "append" | "prepend";
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

export interface ResourceMutationResponseExactCollectionDeleteExecutionArtifact {
  readonly artifactId: string;
  readonly targetId: string;
  readonly kind: "exactCollectionDelete";
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
  readonly placement: null;
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

export type ResourceMutationResponseExecutionArtifact =
  | ResourceMutationResponseFallbackExecutionArtifact
  | ResourceMutationResponseExactDetailExecutionArtifact
  | ResourceMutationResponseExactDetailInvalidationExecutionArtifact
  | ResourceMutationResponseExactCollectionDeleteExecutionArtifact
  | ResourceMutationResponseExactCollectionItemExecutionArtifact
  | ResourceMutationResponseExactCollectionTombstoneExecutionArtifact
  | ResourceMutationResponseExactCollectionInsertExecutionArtifact
  | ResourceMutationResponseExactSummaryExecutionArtifact;
