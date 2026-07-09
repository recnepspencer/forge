export interface FormStateHistoryArtifact {
  readonly kind: "formStateHistory";
  readonly artifactId: number;
  readonly entryKind: "rawInput" | "draftWrite";
  readonly observedAtMs: number;
  readonly field: string;
  readonly operation: string;
  readonly source: string | null;
  readonly reason: string | null;
  readonly rawValueDigest: string | null;
  readonly parsedValueDigest: string | null;
  readonly previousDraftDigest: string;
  readonly nextDraftDigest: string;
  readonly sourceDigest: string;
  readonly effectiveDigest: string;
  readonly dirtyDigest: string;
  readonly patchPlanDigest: string;
  readonly readinessDigest: string;
  readonly stateHistoryDigest: string;
}
