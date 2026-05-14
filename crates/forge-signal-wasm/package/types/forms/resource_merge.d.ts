import type { MergePolicyPreviewRequest } from "../diagnostics.js";
import type { FormReadinessBlocker } from "./core.js";
import type { FormMessageArtifact } from "./validation.js";

export type FormResourceMergeStatus =
  | "ready"
  | "conflict"
  | "unavailable";

export interface FormResourceMergeArtifact {
  readonly kind: "resourceMergePreview";
  readonly artifactId: number;
  readonly observedAtMs: number;
  readonly source: "preview" | "clear";
  readonly sourceKind: "resourceLine" | "form";
  readonly status: FormResourceMergeStatus;
  readonly stale: boolean;
  readonly request: MergePolicyPreviewRequest | null;
  readonly effectDigest: string | null;
  readonly sourceBranchId: number | null;
  readonly targetBranchId: number | null;
  readonly reason: string;
  readonly conflictCount: number;
  readonly projectedFields: ReadonlyArray<string>;
  readonly projectedSections: ReadonlyArray<string>;
  readonly blockers: ReadonlyArray<FormReadinessBlocker>;
  readonly messages: ReadonlyArray<FormMessageArtifact>;
  readonly proofDigest: string | null;
  readonly resultDigest: string;
}

export interface FormResourceMergeReport {
  readonly current: FormResourceMergeArtifact | null;
  readonly history: ReadonlyArray<FormResourceMergeArtifact>;
  readonly summary: {
    readonly status: FormResourceMergeStatus | "ready";
    readonly stale: boolean;
    readonly conflictCount: number;
    readonly blockerCount: number;
    readonly messageCount: number;
    readonly fieldCount: number;
    readonly sectionCount: number;
  };
  readonly counters: {
    readonly costBasis: "resourceMergePreviewHistoryScan";
    readonly incrementalStatus: "notIncremental";
    readonly previews: number;
    readonly conflictPreviews: number;
    readonly unavailablePreviews: number;
    readonly stalePreviews: number;
    readonly projectedFields: number;
    readonly projectedSections: number;
    readonly blockers: number;
    readonly messages: number;
  };
  readonly digest: string;
}
