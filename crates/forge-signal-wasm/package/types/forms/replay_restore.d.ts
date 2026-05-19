import type { ResourceLineStatus } from "../resource/resource_lifecycle.js";

export type FormReplayRestoreMode =
  | "resourceReplayExact"
  | "resourceRestoreExact";

export type FormReplayRestoreResultKind =
  | "replayed"
  | "restored"
  | "unavailable";

export interface FormReplayRestoreUnavailable {
  readonly kind: "unavailable";
  readonly reason:
    | "resourceSourceUnavailable"
    | "exactHistoryUnavailable"
    | "runtimeRejected"
    | "identityMigrationUnavailable"
    | "branchHeadUnavailable";
  readonly detail: string;
  readonly basisCurrentId: string | null;
  readonly basisAdvanceCount: number;
  readonly digest: string;
}

export interface FormReplayRestoreReplayed {
  readonly kind: "replayed";
  readonly mode: "SameRuntimeSignalExact";
  readonly signalId: string;
  readonly basisCurrentId: string | null;
  readonly basisAdvanceCount: number;
  readonly reloadStatus: ResourceLineStatus;
  readonly digest: string;
}

export interface FormReplayRestoreRestored {
  readonly kind: "restored";
  readonly mode: "SameRuntimeBranchExact";
  readonly branchId: number;
  readonly snapshotId: number;
  readonly basisCurrentId: string | null;
  readonly basisAdvanceCount: number;
  readonly reloadStatus: ResourceLineStatus;
  readonly digest: string;
}

export type FormReplayRestoreResourceArtifact =
  | FormReplayRestoreUnavailable
  | FormReplayRestoreReplayed
  | FormReplayRestoreRestored;

export interface FormReplayRestoreArtifact {
  readonly kind: "formReplayRestore";
  readonly replayRestoreId: number;
  readonly observedAtMs: number;
  readonly mode: FormReplayRestoreMode;
  readonly resultKind: FormReplayRestoreResultKind;
  readonly reason: string;
  readonly previousSourceDigest: string;
  readonly previousDraftDigest: string;
  readonly previousEffectiveDigest: string;
  readonly nextSourceDigest: string;
  readonly nextDraftDigest: string;
  readonly nextEffectiveDigest: string;
  readonly resourceReplayRestore: FormReplayRestoreResourceArtifact;
  readonly replayRestoreDigest: string;
}
