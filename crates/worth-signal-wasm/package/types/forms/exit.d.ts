export interface FormExitPresentationArtifact {
  readonly kind: "exitPresentationUpdate";
  readonly artifactId: number;
  readonly observedAtMs: number;
  readonly source: "report" | "clear";
  readonly status: "pending" | "busy" | "settling" | "ready" | "failed" | "unavailable";
  readonly target: string | null;
  readonly reason: string;
  readonly token: string | null;
  readonly scopeKind: "route" | "modal" | "external" | null;
  readonly surfaceId: string | null;
  readonly operation: "generic" | "block" | "confirm" | "dismiss" | "leave" | "stay" | "close";
  readonly unavailableReason: string | null;
  readonly exitDigest: string;
}

export interface FormExitReport {
  readonly current: FormExitPresentationArtifact | null;
  readonly history: ReadonlyArray<FormExitPresentationArtifact>;
  readonly summary: {
    readonly status: "pending" | "busy" | "settling" | "ready" | "failed" | "unavailable";
    readonly scopeKind: "route" | "modal" | "external" | null;
    readonly surfaceId: string | null;
    readonly activeTarget: string | null;
    readonly unavailableReason: string | null;
    readonly guardKind: "clean" | "dirty" | "pendingAction" | "sourceUnavailable";
    readonly pendingActions: number;
    readonly requiresConfirmation: boolean;
  };
  readonly counters: {
    readonly costBasis: "exitPresentationDerivedAndHistoryScan";
    readonly incrementalStatus: "notIncremental";
    readonly updates: number;
    readonly routeScopeUpdates: number;
    readonly modalScopeUpdates: number;
    readonly externalScopeUpdates: number;
    readonly settlingUpdates: number;
    readonly failedUpdates: number;
    readonly unavailableUpdates: number;
    readonly pendingActions: number;
    readonly dirtyGuardActivations: number;
    readonly sourceUnavailableGuards: number;
  };
  readonly digest: string;
}
