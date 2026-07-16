export type FormResetMode = "acceptCanonicalValue" | "resourceRollback";
export type FormResetResultKind = "reset" | "effectRejected" | "noOp" | "unavailable";

export interface FormResetRollbackUnavailable {
  readonly kind: "unavailable";
  readonly reason: string;
  readonly detail: string;
  readonly digest: string;
}

export interface FormResetEffectRejected {
  readonly kind: "effectRejected";
  readonly effectId: string;
  readonly terminalKind: "rejectedAndRetired";
  readonly retiredEffectIds: readonly string[];
  readonly projectionKind: "derivedEffectProjectionBranch" | "canonical";
  readonly projectionDigest: string;
  readonly retirement: readonly Readonly<Record<string, unknown>>[];
  readonly retirementDigest: string;
  readonly digest: string;
}

export type FormResetRollbackArtifact =
  | FormResetRollbackUnavailable
  | FormResetEffectRejected;

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
