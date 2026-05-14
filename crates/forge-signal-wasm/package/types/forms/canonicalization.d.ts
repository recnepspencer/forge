import type { SignalValue } from "../model.js";
import type { ResourceLineVisibleSelection } from "../resource/resource_line_diagnostics.js";
import type { ResourceEffectProfileDigest } from "../resource/resource_effect_envelope.js";
import type {
  FormResourceMutationResponseReport,
  FormResourceRollbackDigest,
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
  readonly sourceBasisDigest: string;
  readonly canonicalSourceDigest: string;
  readonly canonicalValue: SignalValue;
  readonly resourceBacked: {
    readonly sourceKind: "resourceLine";
    readonly effectProfile: {
      readonly profile: ResourceEffectProfileDigest | null;
      readonly closeoutMatrixDigest: string | null;
    };
    readonly rollback: FormResourceRollbackDigest | null;
    readonly visibleSelection: ResourceLineVisibleSelection;
    readonly mutationResponse: FormResourceMutationResponseReport | null;
    readonly verification: {
      readonly packageDigest: string;
      readonly mutationResponseCloseoutMatrixDigest: string | null;
    };
    readonly resourceSubmissionDigest: string;
  } | null;
  readonly draftReset: true;
  readonly sourceProjection:
    | "serverCanonicalUntilAuthoritativeSourceDrift"
    | "resourceMutationResponsePreservedOptimisticTruth"
    | "resourceMutationResponsePartialCanonicalTruth"
    | "resourceMutationResponseRefetchRequired"
    | "resourceMutationResponseDeliveryAwaited";
  readonly reason: string;
  readonly canonicalizationDigest: string;
}
