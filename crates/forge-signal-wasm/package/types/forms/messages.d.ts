import type { FormMessageArtifact } from "./validation.js";

export interface FormMessagePresentationArtifact {
  readonly kind: "messagePresentationUpdate";
  readonly artifactId: number;
  readonly observedAtMs: number;
  readonly source: "report" | "clear";
  readonly status: "pending" | "busy" | "settling" | "ready" | "failed" | "unavailable";
  readonly target: string | null;
  readonly reason: string;
  readonly token: string | null;
  readonly scope: "field" | "section" | "action" | "control" | "wholeForm" | "step";
  readonly channel: "inline" | "summary" | "banner" | "toast";
  readonly audience: "user" | "developer" | "system";
  readonly visibleCount: number | null;
  readonly operation: "show" | "update" | "dismiss" | "clear";
  readonly messageDigest: string;
}

export interface FormMessagesReport {
  readonly current: FormMessagePresentationArtifact | null;
  readonly history: ReadonlyArray<FormMessagePresentationArtifact>;
  readonly semantic: {
    readonly total: number;
    readonly visible: number;
    readonly summary: number;
    readonly blocked: number;
  };
  readonly summary: {
    readonly status: "pending" | "busy" | "settling" | "ready" | "failed" | "unavailable";
    readonly activeChannel: "inline" | "summary" | "banner" | "toast" | null;
    readonly activeAudience: "user" | "developer" | "system" | null;
    readonly activeTarget: string | null;
    readonly semanticVisibleCount: number;
    readonly externalVisibleCount: number | null;
  };
  readonly counters: {
    readonly costBasis: "messagePresentationHistoryScan";
    readonly incrementalStatus: "notIncremental";
    readonly semanticVisibleMessages: number;
    readonly updates: number;
    readonly settlingUpdates: number;
    readonly failedUpdates: number;
    readonly unavailableUpdates: number;
  };
  readonly digest: string;
}
