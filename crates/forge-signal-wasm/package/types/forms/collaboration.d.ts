import type { FormResourceVisibleSelectionKind } from "./resource_source.js";

export type FormCollaborationMode =
  | "singleWriterLock"
  | "fieldLease"
  | "branchPerActor"
  | "optimisticMerge"
  | "reviewerCommentOnly"
  | "unavailable";

export interface FormCollaborationDeclaration {
  readonly mode: FormCollaborationMode;
  readonly actorId?: string;
  readonly supportsPresence?: boolean;
  readonly supportsComments?: boolean;
}

export interface FormCollaborationLease {
  readonly field: string;
  readonly ownerId: string;
}

export interface FormCollaborationPresence {
  readonly actorId: string;
  readonly status: "active" | "idle" | "viewing";
}

export interface FormCollaborationComment {
  readonly id: string;
  readonly authorId: string;
  readonly target: string | null;
}

export interface FormCollaborationResourceProof {
  readonly required: boolean;
  readonly admitted: boolean;
  readonly sourceKind: "resourceLine" | null;
  readonly visibleSelectionKind: FormResourceVisibleSelectionKind | null;
  readonly branchId: string | number | null;
  readonly reason: string | null;
  readonly digest: string;
}

export type FormCollaborationEventKind =
  | "postureChange"
  | "lockChange"
  | "leaseChange"
  | "branchChange"
  | "readOnlyChange"
  | "remoteUpdateChange"
  | "presenceChange"
  | "commentChange";

export interface FormCollaborationEvent {
  readonly kind: FormCollaborationEventKind;
  readonly artifactId: number;
  readonly source: "report" | "clear";
  readonly previousArtifactId: number | null;
  readonly mode: FormCollaborationMode | null;
  readonly posture: "active" | "blocked" | "settling" | "unavailable";
  readonly reason: string;
  readonly lockOwnerId: string | null;
  readonly leasedFields: ReadonlyArray<FormCollaborationLease>;
  readonly branchId: string | number | null;
  readonly readOnly: boolean;
  readonly remoteUpdateDigest: string | null;
  readonly presence: ReadonlyArray<FormCollaborationPresence>;
  readonly comments: ReadonlyArray<FormCollaborationComment>;
  readonly previousDigest: string | null;
  readonly nextDigest: string;
  readonly digest: string;
}

export interface FormCollaborationArtifact {
  readonly kind: "collaboration";
  readonly artifactId: number;
  readonly source: "report" | "clear";
  readonly posture: "active" | "blocked" | "settling" | "unavailable";
  readonly reason: string;
  readonly mode: FormCollaborationMode | null;
  readonly actorId: string | null;
  readonly lockOwnerId: string | null;
  readonly leasedFields: ReadonlyArray<FormCollaborationLease>;
  readonly branchId: string | number | null;
  readonly readOnly: boolean;
  readonly remoteUpdateDigest: string | null;
  readonly presence: ReadonlyArray<FormCollaborationPresence>;
  readonly comments: ReadonlyArray<FormCollaborationComment>;
  readonly collaborationDigest: string;
}

export interface FormCollaborationReport {
  readonly declared: boolean;
  readonly mode: FormCollaborationMode | "notDeclared";
  readonly actorId: string | null;
  readonly posture: "notDeclared" | "active" | "blocked" | "settling" | "unavailable";
  readonly reason: string;
  readonly lockOwnerId: string | null;
  readonly leasedFields: ReadonlyArray<FormCollaborationLease>;
  readonly branchId: string | number | null;
  readonly readOnly: boolean;
  readonly remoteUpdateDigest: string | null;
  readonly presence: ReadonlyArray<FormCollaborationPresence>;
  readonly comments: ReadonlyArray<FormCollaborationComment>;
  readonly resourceProof: FormCollaborationResourceProof;
  readonly history: ReadonlyArray<FormCollaborationArtifact>;
  readonly events: ReadonlyArray<FormCollaborationEvent>;
  readonly eventsDigest: string;
  readonly counters: {
    readonly costBasis: "derivedCollaborationPostureScan";
    readonly incrementalStatus: "notIncremental";
    readonly blockingFields: number;
    readonly presenceActors: number;
    readonly commentArtifacts: number;
    readonly historyArtifacts: number;
    readonly eventArtifacts: number;
    readonly postureChanges: number;
    readonly lockChanges: number;
    readonly leaseChanges: number;
    readonly branchChanges: number;
    readonly presenceChanges: number;
    readonly commentChanges: number;
    readonly blocked: number;
    readonly settling: number;
    readonly unavailable: number;
    readonly resourceProofRequired: number;
    readonly resourceProofUnavailable: number;
  };
  readonly digest: string;
}
