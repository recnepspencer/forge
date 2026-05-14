export interface FormMediaPresentationArtifact {
  readonly kind: "mediaPresentationUpdate";
  readonly artifactId: number;
  readonly observedAtMs: number;
  readonly source: "report" | "clear";
  readonly status: "pending" | "busy" | "settling" | "ready" | "failed" | "unavailable";
  readonly target: string | null;
  readonly reason: string;
  readonly token: string | null;
  readonly mode: "preview" | "capture" | "crop" | "annotate" | null;
  readonly surfaceId: string | null;
  readonly operation: "generic" | "open" | "replace" | "annotate" | "close";
  readonly mediaDigest: string;
}

export interface FormMediaReport {
  readonly current: FormMediaPresentationArtifact | null;
  readonly history: ReadonlyArray<FormMediaPresentationArtifact>;
  readonly summary: {
    readonly status: "pending" | "busy" | "settling" | "ready" | "failed" | "unavailable";
    readonly mode: "preview" | "capture" | "crop" | "annotate" | null;
    readonly surfaceId: string | null;
    readonly activeTarget: string | null;
  };
  readonly counters: {
    readonly costBasis: "mediaPresentationHistoryScan";
    readonly incrementalStatus: "notIncremental";
    readonly updates: number;
    readonly settlingUpdates: number;
    readonly failedUpdates: number;
    readonly unavailableUpdates: number;
  };
  readonly digest: string;
}
