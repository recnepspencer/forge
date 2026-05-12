import type { ResourceEffectJsonPathPatchProof } from "./resource_effect_json_path_patch_proof.js";
import type {
  ResourceEffectLocus,
  ResourceEffectLocusProof,
} from "./resource_effect_locus_proof.js";
export type { ResourceEffectJsonPathPatchProof } from "./resource_effect_json_path_patch_proof.js";
export type {
  ResourceEffectLocus,
  ResourceEffectLocusCostCounters,
  ResourceEffectLocusProof,
} from "./resource_effect_locus_proof.js";

export type ResourceEffectEnvelopeProvenance =
  | "localPatch"
  | "deliveredPatch"
  | "deliveredReplace"
  | "deliveryInvalidate"
  | "deliveryBasisRefresh";

export interface ResourceEffectProfileDigest {
  readonly name: string;
  readonly optimism: "branchSpeculative" | "none";
  readonly confirmation: "exact" | "serverCanonical" | "acceptedPendingDelivery";
  readonly rollback: "branchRestore" | "branchRestoreOrInverse" | "unavailable";
  readonly rebase: "nativeMergePlan" | "unavailable";
  readonly preimage: "none" | "compactInverse" | "digestOnly" | "retainedFragment";
}

export type ResourceEffectCompactInverseDescriptor =
  | {
      readonly kind: "compactPatchInverse";
      readonly mode: "CompactInversePatch";
      readonly preimage: "aspectValue";
      readonly scope: "aspect";
      readonly itemId: string;
      readonly aspect: string;
      readonly summary: null;
      readonly patch: {
        readonly kind: "itemAspect";
        readonly itemId: string;
        readonly aspect: string;
        readonly value: unknown;
      };
      readonly cost: ResourceEffectCompactInverseCost;
    }
  | {
      readonly kind: "compactPatchInverse";
      readonly mode: "CompactInversePatch";
      readonly preimage: "itemFragment";
      readonly scope: "item";
      readonly itemId: string;
      readonly aspect: null;
      readonly summary: null;
      readonly patch: {
        readonly kind: "item";
        readonly itemId: string;
        readonly nextItem: unknown;
      };
      readonly cost: ResourceEffectCompactInverseCost;
    }
  | {
      readonly kind: "compactPatchInverse";
      readonly mode: "CompactInversePatch";
      readonly preimage: "summaryValue";
      readonly scope: "summary";
      readonly itemId: null;
      readonly aspect: null;
      readonly summary: string;
      readonly patch: {
        readonly kind: "summary";
        readonly summary: string;
        readonly value: unknown;
      };
      readonly cost: ResourceEffectCompactInverseCost;
    };

export interface ResourceEffectCompactInverseCost {
  readonly retainedValueCount: 1;
  readonly retainedResponsePreimage: false;
}

export interface ResourceEffectPatchDigest {
  readonly kind: "replace" | "item" | "itemAspect" | "summary" | null;
  readonly scope: "line" | "item" | "aspect" | "summary" | null;
  readonly itemId: string | null;
  readonly aspect: string | null;
  readonly summary: string | null;
  readonly valueChanged: boolean;
  readonly jsonPath: ResourceEffectJsonPathPatchProof | null;
}

export interface ResourceEffectDeliveryDigest {
  readonly kind: "replace" | "patch" | "invalidate" | "basisRefresh";
  readonly scope: "line" | "item" | "aspect" | "summary" | "basis" | "invalidate";
  readonly packetId: string;
  readonly basisId: string | null;
  readonly nextBasisId: string | null;
}

export type ResourceEffectCommittedOnlyReason =
  "deliveryAuthority" | "profileDisablesOptimism" | "unconfigured";

export type ResourceEffectOptimismUnavailableReason =
  "unsupportedByRuntime" | "runtimeRejected" | "branchHeadUnavailable" | "restoreUnavailable";

export type ResourceEffectBranchPosture =
  | {
      readonly kind: "committedOnly";
      readonly profileName: string | null;
      readonly optimism: "branchSpeculative" | "none";
      readonly rollback: "branchRestore" | "branchRestoreOrInverse" | "unavailable";
      readonly reason: ResourceEffectCommittedOnlyReason;
      readonly detail: string;
      readonly proofBreadth: 0;
    }
  | {
      readonly kind: "speculativeBranch";
      readonly profileName: string;
      readonly optimism: "branchSpeculative";
      readonly rollback: "branchRestore" | "branchRestoreOrInverse";
      readonly rollbackMode: "SameRuntimeBranchExact" | "CompactInversePatch";
      readonly branchId: number;
      readonly snapshotId: number;
      readonly restoreMode: "SameRuntimeBranchExact" | null;
      readonly inverse: ResourceEffectCompactInverseDescriptor | null;
      readonly proofBreadth: number;
    }
  | {
      readonly kind: "optimisticUnavailable";
      readonly profileName: string;
      readonly optimism: "branchSpeculative";
      readonly rollback: "branchRestore" | "branchRestoreOrInverse";
      readonly reason: ResourceEffectOptimismUnavailableReason;
      readonly detail: string;
      readonly branchId: number | null;
      readonly snapshotId: number | null;
      readonly inverseAvailable: false;
      readonly proofBreadth: number;
    };

export type ResourceEffectServerConfirmation =
  | {
      readonly kind: "notApplicable";
      readonly detail: string;
    }
  | {
      readonly kind: "independentServerTruth";
      readonly previousEffectId: string | null;
      readonly detail: string;
    }
  | {
      readonly kind: "preservedSpeculativeTruth";
      readonly previousEffectId: string;
      readonly previousPlanId: string;
      readonly previousBranchId: number;
      readonly previousSnapshotId: number;
      readonly locusMatches: true;
      readonly detail: string;
    }
  | {
      readonly kind: "consumedCanonicalServerTruth";
      readonly previousEffectId: string;
      readonly previousPlanId: string;
      readonly previousBranchId: number;
      readonly previousSnapshotId: number;
      readonly locusMatches: boolean;
      readonly valueChanged: boolean;
      readonly detail: string;
    };

export type ResourceEffectBranchLifecycle =
  | {
      readonly kind: "selectedExistingBranch";
      readonly acquisition: "currentRuntimeBranch";
      readonly creation: "notCreatedByResourceRuntime";
      readonly reuse: "currentBranchReuse";
      readonly ownership: "signalsRuntimeOwned";
      readonly branchId: number;
      readonly snapshotId: number;
      readonly restoreMode: "SameRuntimeBranchExact" | null;
      readonly disposal: {
        readonly kind: "notOwnedByResourceRuntime";
        readonly detail: string;
      };
      readonly leakDenial: {
        readonly kind: "noResourceOwnedBranch";
        readonly detail: string;
      };
    }
  | {
      readonly kind: "unavailable";
      readonly creation: "deniedBeforeBranchCreation";
      readonly reason: ResourceEffectOptimismUnavailableReason;
      readonly detail: string;
      readonly branchId: number | null;
      readonly snapshotId: number | null;
      readonly disposal: {
        readonly kind: "notApplicable";
        readonly detail: string;
      };
      readonly leakDenial: {
        readonly kind: "optimismDeniedBeforeResourceOwnedBranch";
        readonly detail: string;
      };
    }
  | {
      readonly kind: "notApplicable";
      readonly creation: "notApplicable";
      readonly reason: ResourceEffectCommittedOnlyReason;
      readonly detail: string;
      readonly disposal: {
        readonly kind: "notApplicable";
        readonly detail: string;
      };
      readonly leakDenial: {
        readonly kind: "notApplicable";
        readonly detail: string;
      };
    };

export type ResourceEffectRollback =
  | {
      readonly kind: "exactBranchRestoreAvailable";
      readonly mode: "SameRuntimeBranchExact";
      readonly branchId: number;
      readonly snapshotId: number;
      readonly detail: string;
    }
  | {
      readonly kind: "compactInverseAvailable";
      readonly mode: "CompactInversePatch";
      readonly branchId: number;
      readonly snapshotId: number;
      readonly inverse: ResourceEffectCompactInverseDescriptor;
      readonly detail: string;
    }
  | {
      readonly kind: "unavailable";
      readonly reason: ResourceEffectOptimismUnavailableReason;
      readonly detail: string;
      readonly branchId: number | null;
      readonly snapshotId: number | null;
      readonly inverseAvailable: false;
    }
  | {
      readonly kind: "notApplicable";
      readonly reason: ResourceEffectCommittedOnlyReason;
      readonly detail: string;
    };

export type ResourceEffectOptimisticLifecycle =
  | {
      readonly kind: "applied";
      readonly admissionKind: "localPatch" | "delivery";
      readonly branchPosture: "speculativeBranch";
      readonly branchId: number;
      readonly snapshotId: number;
      readonly restoreMode: "SameRuntimeBranchExact" | null;
      readonly rollback: ResourceEffectRollback;
      readonly confirmation: "pendingServer";
      readonly detail: string;
    }
  | {
      readonly kind: "unavailable";
      readonly admissionKind: "localPatch" | "delivery";
      readonly branchPosture: "optimisticUnavailable";
      readonly reason: ResourceEffectOptimismUnavailableReason;
      readonly detail: string;
      readonly branchId: number | null;
      readonly snapshotId: number | null;
      readonly inverseAvailable: false;
      readonly rollback: ResourceEffectRollback;
    }
  | {
      readonly kind: "committed";
      readonly admissionKind: "localPatch" | "delivery";
      readonly branchPosture: "committedOnly";
      readonly reason: ResourceEffectCommittedOnlyReason;
      readonly detail: string;
      readonly rollback: ResourceEffectRollback;
      readonly confirmation: ResourceEffectServerConfirmation;
    };

export interface ResourceEffectPlanDigest {
  readonly planId: string;
  readonly admissionKind: "localPatch" | "delivery";
  readonly causalSequence: string;
  readonly retryLineageId: string | null;
  readonly branch: ResourceEffectBranchPosture;
}

export interface ResourceEffectAuthorityDigest {
  readonly version: "resource-effect-authority-v1";
  readonly runtimeEffectToken: string;
  readonly envelopeDigest: string;
}

export interface ResourceEffectEnvelope {
  readonly version: "resource-effect-envelope-v1";
  readonly effectId: string;
  readonly provenance: ResourceEffectEnvelopeProvenance;
  readonly idempotencyKey: string | null;
  readonly serverCorrelationId: string | null;
  readonly plan: ResourceEffectPlanDigest;
  readonly family: {
    readonly kind: "detail" | "collection" | "paged";
    readonly familyId: string;
  };
  readonly line: {
    readonly runtimeLineId: string;
    readonly scopeId: string;
    readonly canonicalKey: string;
  };
  readonly profile: ResourceEffectProfileDigest | null;
  readonly branchLifecycle: ResourceEffectBranchLifecycle;
  readonly optimistic: ResourceEffectOptimisticLifecycle;
  readonly request: {
    readonly correlationId: string | null;
    readonly branchId: string | number | null;
    readonly basisId: string | null;
  };
  readonly delivery: ResourceEffectDeliveryDigest | null;
  readonly locus: ResourceEffectLocus;
  readonly locusProof: ResourceEffectLocusProof | null;
  readonly patch: ResourceEffectPatchDigest;
  readonly authority: ResourceEffectAuthorityDigest;
  readonly counters: {
    readonly patchCountBefore: number;
    readonly deliveryCountBefore: number;
    readonly basisAdvanceCountBefore: number;
    readonly planningBreadth: number;
    readonly executionBreadth: number;
    readonly branchProofBreadth: number;
    readonly branchLifecycleBreadth: number;
    readonly optimisticLifecycleBreadth: number;
    readonly serverConfirmationBreadth: number;
    readonly rollbackReadinessBreadth: number;
    readonly responseLensBreadth: number;
    readonly effectLocusBreadth: number;
    readonly jsonPathTraversalBreadth: number;
    readonly jsonPathReconstructionBreadth: number;
  };
}
