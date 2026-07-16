import type { ResourceEffectEnvelope } from "./resource_effect_envelope.js";

export interface ResourceOpenEffectSummary {
  readonly effectId: string;
  readonly envelope: ResourceEffectEnvelope;
  readonly lifecycle: "Pending" | "ResponseRecorded" | "Retired";
  readonly branchId: number;
  readonly nativeParentBranchId: number;
  readonly dependencyBasisBranchId: number | null;
  readonly dependencyEffectIds: readonly string[];
  readonly dependencyCloseoutPolicy:
    | "independent"
    | "cancelOnDependencyRejection";
  readonly locus: Readonly<Record<string, unknown>>;
  readonly admissionSequence: number;
  readonly terminal: ResourceEffectTerminalSummary | null;
}

export interface ResourceEffectTerminalSummary {
  readonly kind:
    | "merged"
    | "supersededAndRetired"
    | "rejectedAndRetired"
    | "dependencyCancelled";
  readonly closeout?: Readonly<Record<string, unknown>>;
  readonly retirement?: Readonly<Record<string, unknown>>;
}

export interface ResourceOptimisticProjectionReceipt<TValue = unknown> {
  readonly kind: "derivedEffectProjectionBranch" | "canonical";
  readonly projectedValue: TValue;
  readonly projectionDigest: string;
  readonly canonicalAuthority: false;
  readonly affectedEffectIds?: readonly string[];
  readonly plan: {
    readonly strategy: "affectedLocusRebuild" | "measuredBroadRebuild";
    readonly affectedLocusKeys: readonly string[];
    readonly counters: {
      readonly openEffectLookupCount: number;
      readonly dependencyTraversalCount: number;
      readonly affectedEffectCount: number;
      readonly affectedLocusCount: number;
      readonly reconstructionCount: number;
      readonly fallbackBreadth: number;
    };
  };
}

export interface ResourceEffectSettlementOptions {
  readonly serverPatch?: object | null;
  readonly serverRevision?: number | null;
  readonly responseId?: string | null;
}

export interface ResourceEffectRejectionOptions {
  readonly responseId?: string | null;
}

export interface ResourceEffectSettledResult<TValue = unknown> {
  readonly kind:
    | "merged"
    | "supersededAndRetired"
    | "rejectedAndRetired";
  readonly effectId: string;
  readonly canonicalValue: TValue;
  readonly projection: ResourceOptimisticProjectionReceipt<TValue>;
  readonly automaticallySettled?: readonly ResourceEffectSettlementResult<TValue>[];
  readonly reconciliation?: Readonly<Record<string, unknown>>;
  readonly retirement?: Readonly<Record<string, unknown>>;
  readonly retired?: readonly Readonly<Record<string, unknown>>[];
  readonly closeout?: Readonly<Record<string, unknown>>;
  readonly closeoutPlan?: Readonly<Record<string, unknown>>;
}

export interface ResourceEffectResponseRecordedResult<TValue = unknown> {
  readonly kind: "responseRecorded";
  readonly effectId: string;
  readonly canonicalValue: TValue;
  readonly projection: ResourceOptimisticProjectionReceipt<TValue>;
  readonly waitingOnDependencyEffectIds: readonly string[];
}

export interface ResourceEffectDuplicateSettlementResult<TValue = unknown> {
  readonly kind: "duplicateSettlement";
  readonly effectId: string;
  readonly responseId: string;
  readonly originalKind:
    | "merged"
    | "supersededAndRetired"
    | "rejectedAndRetired"
    | "responseRecorded"
    | "dependencyCancelled"
    | "responseInFlight";
  readonly originalReceipt: ResourceEffectSettlementResult<TValue> | null;
}

export type ResourceEffectSettlementResult<TValue = unknown> =
  | ResourceEffectSettledResult<TValue>
  | ResourceEffectResponseRecordedResult<TValue>
  | ResourceEffectDuplicateSettlementResult<TValue>;

export interface ResourceEffectDagCounters {
  readonly effectLookupCount: number;
  readonly pendingAdmissionCount: number;
  readonly openEffectCount: number;
  readonly dependencyIndexKeyCount: number;
  readonly locusIndexKeyCount: number;
  readonly retryLineageIndexKeyCount: number;
}

export interface ResourceLineEffects<TValue = unknown> {
  get(effectId: string): ResourceOpenEffectSummary | null;
  open(): readonly ResourceOpenEffectSummary[];
  reject(
    effectId: string,
    options?: ResourceEffectRejectionOptions,
  ): Promise<ResourceEffectSettlementResult<TValue>>;
  confirm(
    effectId: string,
    options?: ResourceEffectSettlementOptions,
  ): Promise<ResourceEffectSettlementResult<TValue>>;
  projection(): ResourceOptimisticProjectionReceipt<TValue> | null;
  rebuildProjection(): Promise<ResourceOptimisticProjectionReceipt<TValue>>;
  counters(): ResourceEffectDagCounters;
}
