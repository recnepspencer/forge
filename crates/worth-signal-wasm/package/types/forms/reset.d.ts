import type { ResourceEffectRollback } from "../resource/resource_effect_envelope.js";
import type { ResourceLineStatus } from "../resource/resource_lifecycle.js";

export type FormResetMode = "acceptCanonicalValue" | "resourceRollback";
export type FormResetResultKind = "reset" | "rolledBack" | "noOp" | "unavailable";

export interface FormResetRollbackUnavailable {
  readonly kind: "unavailable";
  readonly reason: string;
  readonly detail: string;
  readonly digest: string;
}

export interface FormResetRollbackApplied {
  readonly kind: "rolledBack";
  readonly mode: "SameRuntimeBranchExact" | "CompactInversePatch";
  readonly effectId: string;
  readonly branchId: number;
  readonly snapshotId: number;
  readonly basisCurrentId: string | null;
  readonly basisAdvanceCount: number;
  readonly rollback: ResourceEffectRollback;
  readonly reloadStatus: ResourceLineStatus;
  readonly digest: string;
}

export type FormResetRollbackArtifact =
  | FormResetRollbackUnavailable
  | FormResetRollbackApplied;

export interface FormResetArtifact {
  readonly kind: "formReset";
  readonly resetId: number;
  readonly observedAtMs: number;
  readonly mode: FormResetMode;
  readonly resultKind: FormResetResultKind;
  readonly reason: string;
  readonly previousSourceDigest: string;
  readonly previousDraftDigest: string;
  readonly previousEffectiveDigest: string;
  readonly nextSourceDigest: string;
  readonly nextDraftDigest: string;
  readonly nextEffectiveDigest: string;
  readonly resourceRollback: FormResetRollbackArtifact | null;
  readonly resetDigest: string;
}
