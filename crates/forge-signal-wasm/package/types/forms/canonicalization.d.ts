import type { SignalValue } from "../model.js";
import type { ResourceEffectProfileDigest } from "../resource/resource_effect_envelope.js";
import type {
  FormResourceMutationResponseReport,
  FormResourceRollbackDigest,
  FormResourceVisibleSelectionReport,
} from "./resource_source.js";

export interface FormCanonicalizationArtifact {
  readonly kind: "canonicalization";
  readonly canonicalizationId: number;
  readonly observedAtMs: number;
  readonly operationId: number;
  readonly action: string | null;
  readonly planDigest: string | null;
  readonly previousSourceDigest: string;
  readonly previousDraftDigest: string;
  readonly previousDraftValue: SignalValue;
  readonly nextDraftDigest: string;
  readonly nextDraftValue: SignalValue;
  readonly sourceBasisDigest: string;
  readonly canonicalSourceDigest: string;
  readonly canonicalValue: SignalValue;
  readonly resourceLine: {
    readonly sourceKind: "resourceLine";
    readonly effectProfile: {
      readonly profile: ResourceEffectProfileDigest | null;
      readonly closeoutMatrixDigest: string | null;
    };
    readonly rollback: FormResourceRollbackDigest | null;
    readonly visibleSelection: FormResourceVisibleSelectionReport;
    readonly mutationResponse: FormResourceMutationResponseReport | null;
    readonly verification: {
      readonly packageDigest: string;
      readonly mutationResponseCloseoutMatrixDigest: string | null;
    };
    readonly resourceSubmissionDigest: string;
  } | null;
  readonly draftReset: boolean;
  readonly draftClearedFields: ReadonlyArray<string>;
  readonly sourceProjection:
    | "serverCanonicalUntilAuthoritativeSourceDrift"
    | "resourceMutationResponsePreservedOptimisticTruth"
    | "resourceMutationResponsePartialCanonicalTruth"
    | "resourceMutationResponseRefetchRequired"
    | "resourceMutationResponseDeliveryAwaited";
  readonly reason: string;
  readonly canonicalizationDigest: string;
}
