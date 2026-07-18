import type {
  LocalTruthBasis,
  LocalTruthBranchReceipt,
  LocalTruthCommitOutcome,
  LocalTruthOutcome,
  LocalTruthSignalProjectionReceipt,
} from "./core.js";

export interface LocalTruthMergeScope {
  readonly entityIds: ReadonlyArray<string>;
  readonly aspectIds: ReadonlyArray<string>;
}

export interface LocalTruthMergeClassification {
  readonly entityId: string;
  readonly aspectId: string;
  readonly kind: "Unchanged" | "AdoptSource" | "PreserveTarget" | "Equivalent" | "ResolutionRequired" | "UnsupportedStructure";
  readonly selectionBasis: string;
  readonly baseValue: unknown;
  readonly sourceValue: unknown;
  readonly targetValue: unknown;
}

export interface LocalTruthConflictAlternative {
  readonly artifactFamily: "LocalTruthConflictAlternative";
  readonly id: string;
  readonly choice: "source" | "target" | "custom";
  readonly value: unknown;
  readonly conflictId?: string;
  readonly resolutionBranchId?: string;
  readonly resolutionBasis?: LocalTruthBasis;
}

export interface LocalTruthConflictRecord {
  readonly artifactFamily: "LocalTruthConflictRecord";
  readonly id: string;
  readonly entityId: string;
  readonly aspectId: string;
  readonly baseValue: unknown;
  readonly sourceValue: unknown;
  readonly targetValue: unknown;
  readonly alternatives: ReadonlyArray<LocalTruthConflictAlternative>;
}

export interface LocalTruthMergeReview {
  readonly artifactFamily: "LocalTruthMergeReview";
  readonly id: string;
  readonly authorityId: string;
  readonly schemaIdentity: string;
  readonly sourceBasis: LocalTruthBasis;
  readonly targetBasis: LocalTruthBasis;
  readonly structuralAncestorCommitId: string;
  readonly classifications: ReadonlyArray<LocalTruthMergeClassification>;
  readonly conflicts: ReadonlyArray<LocalTruthConflictRecord>;
  readonly counters: Readonly<Record<string, number>>;
  readonly digest: string;
}

export type LocalTruthMergePreviewOutcome =
  | LocalTruthOutcome<LocalTruthMergeReview>
  | { readonly posture: "reviewRequired"; readonly review: LocalTruthMergeReview };

export interface LocalTruthMergeRequest {
  readonly sourceBranchId: string;
  readonly targetBranchId: string;
  readonly expectedSourceBasis: LocalTruthBasis;
  readonly expectedTargetBasis: LocalTruthBasis;
  readonly scope?: LocalTruthMergeScope;
  readonly policy?: { readonly overlap: "review" | "preferSource" | "preferTarget" };
}

export interface LocalTruthResolutionSelection {
  readonly reviewId: string;
  readonly conflictId: string;
  readonly alternativeId: string;
}

export interface LocalTruthResolutionSubmission {
  readonly requestId: string;
  readonly reviewId: string;
  readonly selections: ReadonlyArray<LocalTruthResolutionSelection>;
}

export interface LocalTruthResolutionBranchReceipt {
  readonly artifactFamily: "LocalTruthResolutionBranchReceipt";
  readonly reviewId: string;
  readonly conflictId: string;
  readonly entityId: string;
  readonly aspectId: string;
  readonly branch: LocalTruthBranchReceipt;
  readonly targetBasis: LocalTruthBasis;
  readonly derivation?: LocalTruthSignalProjectionReceipt | null;
}

export interface CommittedLocalTruthMerge extends LocalTruthCommitOutcome {
  readonly merge: {
    readonly artifactFamily: "CommittedLocalTruthMerge";
    readonly commit: LocalTruthCommitOutcome["commit"];
    readonly decisions: ReadonlyArray<unknown>;
    readonly targetBasis: LocalTruthBasis;
    readonly counters: Readonly<Record<string, number>>;
    readonly retiredResolutionBranchIds: ReadonlyArray<string>;
  };
}
