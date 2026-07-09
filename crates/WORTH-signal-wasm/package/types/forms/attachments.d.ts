export interface FormAttachmentPresentationArtifact {
  readonly kind: "attachmentPresentationUpdate";
  readonly artifactId: number;
  readonly observedAtMs: number;
  readonly source: "report" | "clear";
  readonly status: "pending" | "busy" | "settling" | "ready" | "failed" | "unavailable";
  readonly target: string | null;
  readonly reason: string;
  readonly token: string | null;
  readonly section: string | null;
  readonly selectedCount: number | null;
  readonly stagedCount: number | null;
  readonly failedCount: number | null;
  readonly operation: "generic" | "select" | "stage" | "preview" | "remove" | "clear";
  readonly attachmentDigest: string;
}

export interface FormAttachmentsReport {
  readonly current: FormAttachmentPresentationArtifact | null;
  readonly history: ReadonlyArray<FormAttachmentPresentationArtifact>;
  readonly summary: {
    readonly status: "pending" | "busy" | "settling" | "ready" | "failed" | "unavailable";
    readonly selectedCount: number | null;
    readonly stagedCount: number | null;
    readonly failedCount: number | null;
    readonly activeSection: string | null;
  };
  readonly counters: {
    readonly costBasis: "attachmentPresentationHistoryScan";
    readonly incrementalStatus: "notIncremental";
    readonly updates: number;
    readonly settlingUpdates: number;
    readonly failedUpdates: number;
    readonly unavailableUpdates: number;
  };
  readonly digest: string;
}
