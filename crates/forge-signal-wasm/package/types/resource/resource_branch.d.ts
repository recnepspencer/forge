import type { MergePolicyPreviewRequest } from "../diagnostics.js";
import type { ResourceEffectEnvelope } from "./resource_effect_envelope.js";
import type {
  ResourceEffectLocus,
  ResourceEffectLocusProof,
} from "./resource_effect_locus_proof.js";

export interface ResourceBranchMergePlanSummary {
  readonly kind: "planned";
  readonly sourceBranchId: number;
  readonly targetBranchId: number;
  readonly mergeKind: string;
  readonly selectedSemantics: {
    readonly strategy: string;
    readonly mergeBase: string;
    readonly conflictPolicy: string;
    readonly conflictIsolation: string;
    readonly identityMatcher: string;
    readonly sourceOnlyPolicy: string;
    readonly deletionPolicy: string;
  };
  readonly breadth: {
    readonly nodeMapCount: number;
    readonly nodePlanCount: number;
    readonly adoptionPlanCount: number;
    readonly conflictRecordCount: number;
  };
  readonly conflicts: ResourceBranchNativeConflictSummary;
  readonly proof: {
    readonly proofSchemaVersion: string;
    readonly planDigest: string;
    readonly semanticsDigest: string;
    readonly selectedStrategyDigest: string;
    readonly selectedMergeBaseDigest: string;
    readonly selectedConflictPolicyDigest: string;
    readonly selectedConflictIsolationDigest: string;
    readonly selectedIdentityMatcherDigest: string;
    readonly selectedSourceOnlyPolicyDigest: string;
    readonly selectedDeletionPolicyDigest: string;
  };
}

export type ResourceBranchNativeConflictSummary =
  | {
      readonly kind: "none";
      readonly divergence: null;
      readonly records: readonly [];
    }
  | {
      readonly kind: "nativeConflicts";
      readonly divergence: string;
      readonly records: ReadonlyArray<ResourceBranchNativeConflictRecord>;
    };

export interface ResourceBranchNativeConflictRecord {
  readonly sourceNode: string;
  readonly targetNode: string;
  readonly requiredResolution: ReadonlyArray<string>;
  readonly supportedStrategies: ReadonlyArray<string>;
}

export interface ResourceBranchMergePlanDenial {
  readonly kind: "denied";
  readonly reason: "mergePlanUnavailable";
  readonly detail: string;
}

export type ResourceBranchMergePlanResult =
  | ResourceBranchMergePlanSummary
  | ResourceBranchMergePlanDenial;

export interface ResourceBranchMergeExecutionSummary {
  readonly kind: "merged";
  readonly sourceBranchId: number;
  readonly targetBranchId: number;
  readonly mergeKind: string;
  readonly selectedSemantics: ResourceBranchMergePlanSummary["selectedSemantics"];
  readonly breadth: {
    readonly recordCount: number;
    readonly sourceOnlyCount: number;
    readonly targetOnlyCount: number;
    readonly conflictRecordCount: number;
  };
  readonly conflicts: ResourceBranchNativeConflictSummary;
  readonly proof: {
    readonly proofSchemaVersion: string;
    readonly resultDigest: string;
    readonly semanticsDigest: string;
    readonly lineageDigest: string;
    readonly selectedStrategyDigest: string;
    readonly selectedMergeBaseDigest: string;
    readonly selectedConflictPolicyDigest: string;
    readonly selectedConflictIsolationDigest: string;
    readonly selectedIdentityMatcherDigest: string;
    readonly selectedSourceOnlyPolicyDigest: string;
    readonly selectedDeletionPolicyDigest: string;
  };
}

export interface ResourceBranchMergeExecutionDenial {
  readonly kind: "denied";
  readonly reason: "mergeExecutionUnavailable";
  readonly detail: string;
}

export type ResourceBranchMergeExecutionResult =
  | ResourceBranchMergeExecutionSummary
  | ResourceBranchMergeExecutionDenial;

export interface ResourceEffectMergePlanSummary extends ResourceBranchMergePlanSummary {
  readonly resourceEffect: {
    readonly effectId: string;
    readonly provenance: ResourceEffectEnvelope["provenance"];
    readonly family: ResourceEffectEnvelope["family"];
    readonly line: ResourceEffectEnvelope["line"];
    readonly locus: ResourceEffectLocus;
    readonly topology: ResourceEffectLocusProof["topology"] | null;
    readonly effectLocus: ResourceEffectLocusProof["locus"] | ResourceEffectLocus["kind"];
    readonly rebase: "nativeMergePlan";
    readonly conflictIsolation: string;
    readonly policyBinding: ResourceEffectMergePolicyBinding;
    readonly rebaseArtifact: ResourceEffectRebaseArtifact;
    readonly proof: {
      readonly planDigest: string;
      readonly semanticsDigest: string;
      readonly effectLocusDigest: string | null;
      readonly compiledLensDigest: string | null;
    };
  };
}

export interface ResourceEffectMergeExecutionSummary extends ResourceBranchMergeExecutionSummary {
  readonly resourceEffect: {
    readonly effectId: string;
    readonly provenance: ResourceEffectEnvelope["provenance"];
    readonly family: ResourceEffectEnvelope["family"];
    readonly line: ResourceEffectEnvelope["line"];
    readonly locus: ResourceEffectLocus;
    readonly topology: ResourceEffectLocusProof["topology"] | null;
    readonly effectLocus: ResourceEffectLocusProof["locus"] | ResourceEffectLocus["kind"];
    readonly rebase: "nativeMergePlan";
    readonly conflictIsolation: string;
    readonly policyBinding: ResourceEffectMergePolicyBinding;
    readonly mergeArtifact: ResourceEffectMergeExecutionArtifact;
    readonly proof: {
      readonly resultDigest: string;
      readonly semanticsDigest: string;
      readonly lineageDigest: string;
      readonly effectLocusDigest: string | null;
      readonly compiledLensDigest: string | null;
    };
  };
}

export type ResourceEffectRebaseArtifact =
  | {
      readonly kind: "rebaseAvailable";
      readonly conflictCount: 0;
      readonly conflicts: readonly [];
      readonly proof: ResourceEffectRebaseProof;
    }
  | {
      readonly kind: "conflict";
      readonly conflictCount: number;
      readonly conflicts: ReadonlyArray<ResourceEffectMergeConflictArtifact>;
      readonly proof: ResourceEffectRebaseProof;
    }
  | {
      readonly kind: "mappingUnavailable";
      readonly reason: "resourceTopologyMappingUnavailable";
      readonly conflictCount: number;
      readonly conflicts: readonly [];
      readonly native: ResourceEffectMergeUnavailableNativeEvidence;
      readonly resource: ResourceEffectMergeConflictResourceSummary;
      readonly detail: string;
      readonly proof: ResourceEffectRebaseProof;
    };

export interface ResourceEffectRebaseProof {
  readonly nativeMergePlanDigest: string;
  readonly nativeMergeSemanticsDigest: string;
  readonly resourceLocusDigest: string;
  readonly aspectPolicyDigest: string;
  readonly policyBindingDigest: string;
  readonly conflictIsolationDigest: string;
}

export type ResourceEffectMergeExecutionArtifact =
  | {
      readonly kind: "merged";
      readonly conflictCount: 0;
      readonly conflicts: readonly [];
      readonly proof: ResourceEffectMergeExecutionProof;
    }
  | {
      readonly kind: "mergedWithConflictRecords";
      readonly conflictCount: number;
      readonly conflicts: ReadonlyArray<ResourceEffectMergeExecutionConflictArtifact>;
      readonly proof: ResourceEffectMergeExecutionProof;
    }
  | {
      readonly kind: "mappingUnavailable";
      readonly reason: "resourceTopologyMappingUnavailable";
      readonly conflictCount: number;
      readonly conflicts: readonly [];
      readonly native: ResourceEffectMergeUnavailableNativeEvidence;
      readonly resource: ResourceEffectMergeConflictResourceSummary;
      readonly detail: string;
      readonly proof: ResourceEffectMergeExecutionProof;
    };

export interface ResourceEffectMergeExecutionProof {
  readonly nativeMergeResultDigest: string;
  readonly nativeMergeSemanticsDigest: string;
  readonly nativeMergeLineageDigest: string;
  readonly resourceLocusDigest: string;
  readonly aspectPolicyDigest: string;
  readonly policyBindingDigest: string;
  readonly conflictIsolationDigest: string;
}

export interface ResourceEffectMergePolicyBinding {
  readonly source: "resourceEffectLocus";
  readonly locusKind: ResourceEffectLocus["kind"];
  readonly aspect: string | null;
  readonly hostRegion: ResourceEffectMergeHostRegion | null;
  readonly resourceGranularity:
    | "hostRegion"
    | "resourceAspect"
    | "resourceItem"
    | "resourceLine";
  readonly nativeIsolationGranularity: "nativeNode";
  readonly nativeMapping:
    | "hostRegionMappedToNativeNode"
    | "resourceAspectMappedToNativeNode"
    | "resourceLocusMappedToNativeNode";
  readonly conflictPolicyName: string;
  readonly conflictIsolationPolicyName: string;
}

export interface ResourceEffectMergeHostRegion {
  readonly source: "responseLocusProofCost";
  readonly topology: ResourceEffectLocusProof["topology"];
  readonly lookup: string;
  readonly traversal: string;
  readonly reconstruction: string;
}

export interface ResourceEffectMergeConflictArtifact {
  readonly kind: "resourceMergeConflict";
  readonly native: ResourceBranchNativeConflictRecord;
  readonly resource: ResourceEffectMergeConflictResourceSummary;
  readonly proof: ResourceEffectRebaseProof;
}

export interface ResourceEffectMergeConflictResourceSummary {
  readonly effectId: string;
  readonly family: ResourceEffectEnvelope["family"];
  readonly line: ResourceEffectEnvelope["line"];
  readonly locus: ResourceEffectLocus;
  readonly topology: ResourceEffectLocusProof["topology"] | null;
  readonly effectLocus: ResourceEffectLocusProof["locus"] | ResourceEffectLocus["kind"];
}

export interface ResourceEffectMergeExecutionConflictArtifact {
  readonly kind: "resourceMergeConflict";
  readonly native: ResourceBranchNativeConflictRecord;
  readonly resource: ResourceEffectMergeConflictArtifact["resource"];
  readonly proof: ResourceEffectMergeExecutionProof;
}

export interface ResourceEffectMergeUnavailableNativeEvidence {
  readonly sourceBranchId: number;
  readonly targetBranchId: number;
  readonly divergence: string;
  readonly records: ReadonlyArray<ResourceBranchNativeConflictRecord>;
}

export interface ResourceEffectMergeRequest {
  readonly merge: MergePolicyPreviewRequest;
  readonly effect: ResourceEffectEnvelope;
}

export type ResourceEffectMergePlanRequest = ResourceEffectMergeRequest;

export interface ResourceEffectMergeDenial {
  readonly kind: "denied";
  readonly reason: "resourceEffectMergeUnavailable";
  readonly detail: string;
}

export type ResourceEffectMergePlanDenial = ResourceEffectMergeDenial;

export type ResourceEffectMergePlanResult =
  | ResourceEffectMergePlanSummary
  | ResourceBranchMergePlanDenial
  | ResourceEffectMergeDenial;

export type ResourceEffectMergeExecutionResult =
  | ResourceEffectMergeExecutionSummary
  | ResourceBranchMergeExecutionDenial
  | ResourceEffectMergeDenial;

export interface ResourceBranchNamespace {
  planMerge(
    request: MergePolicyPreviewRequest,
  ): ResourceBranchMergePlanResult | Promise<ResourceBranchMergePlanResult>;
  planEffectMerge(
    request: ResourceEffectMergeRequest,
  ): ResourceEffectMergePlanResult | Promise<ResourceEffectMergePlanResult>;
  mergeEffect(
    request: ResourceEffectMergeRequest,
  ): ResourceEffectMergeExecutionResult | Promise<ResourceEffectMergeExecutionResult>;
}
