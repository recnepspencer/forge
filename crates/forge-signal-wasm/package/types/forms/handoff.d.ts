export interface FormHandoffPresentationArtifact {
  readonly kind: "handoffPresentationUpdate";
  readonly artifactId: number;
  readonly observedAtMs: number;
  readonly source: "report" | "clear" | "handoff";
  readonly status: "pending" | "busy" | "settling" | "ready" | "failed" | "unavailable";
  readonly target: string | null;
  readonly reason: string;
  readonly token: string | null;
  readonly supersededByToken: string | null;
  readonly scopeKind: "route" | "modal" | "external" | null;
  readonly surfaceId: string | null;
  readonly operation: "generic" | "open" | "handoff" | "dismiss" | "return" | "close";
  readonly unsupportedReason: string | null;
  readonly handoffDigest: string;
}

export interface FormHandoffReport {
  readonly current: FormHandoffPresentationArtifact | null;
  readonly history: ReadonlyArray<FormHandoffPresentationArtifact>;
  readonly summary: {
    readonly status: "pending" | "busy" | "settling" | "ready" | "failed" | "unavailable";
    readonly scopeKind: "route" | "modal" | "external" | null;
    readonly surfaceId: string | null;
    readonly activeTarget: string | null;
    readonly unsupportedReason: string | null;
  };
  readonly counters: {
    readonly costBasis: "handoffPresentationHistoryScan";
    readonly incrementalStatus: "notIncremental";
    readonly updates: number;
    readonly routeScopeUpdates: number;
    readonly modalScopeUpdates: number;
    readonly externalScopeUpdates: number;
    readonly settlingUpdates: number;
    readonly failedUpdates: number;
    readonly unavailableUpdates: number;
  };
  readonly digest: string;
}
