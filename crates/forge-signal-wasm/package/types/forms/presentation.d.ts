import type { FormSourceBootstrapArtifact } from "./core.js";

export type FormPresentationScope =
  | "field"
  | "section"
  | "action"
  | "control"
  | "wholeForm"
  | "step"
  | "modal"
  | "route"
  | "externalHandoff";

export type FormPresentationStatus =
  | "pending"
  | "busy"
  | "settling"
  | "ready"
  | "failed"
  | "unavailable";

export type FormPresentationLane =
  | "entry"
  | "interaction"
  | "availability"
  | "messages"
  | "layout"
  | "action"
  | "canonicalization"
  | "resourceDrift"
  | "collaboration"
  | "attachments"
  | "media"
  | "handoff"
  | "navigation"
  | "exit";

export type FormActionPresentationSettlementDependency =
  | "canonicalization"
  | "messages"
  | "focusTarget"
  | "layout"
  | "navigation"
  | "handoff";

export type FormEntryBootstrapDependency =
  | "sourceAdmission"
  | "draftRestore"
  | "sourceCompatibility"
  | "validation"
  | "readiness"
  | "hostFacts"
  | "inputCapabilities"
  | "focusTarget"
  | "layoutMeasurement";

export interface FormPresentationLanePolicy {
  readonly scope?: FormPresentationScope;
  readonly delayedBusyRevealMs?: number;
  readonly minimumBusyMs?: number;
  readonly settlementAcknowledgement?: "none" | "required";
  readonly settlementTimeoutMs?: number;
  readonly supersessionHandoff?: "replace" | "handoff";
  readonly unavailableAcknowledgement?: "none" | "required";
}

export interface FormActionPresentationLanePolicy extends FormPresentationLanePolicy {
  readonly settleOn?: ReadonlyArray<FormActionPresentationSettlementDependency>;
}

export interface FormEntryPresentationLanePolicy extends FormPresentationLanePolicy {
  readonly bootstrap?: FormEntryBootstrapPolicy;
}

export interface FormPresentationDependencyArtifact {
  readonly dependency: FormActionPresentationSettlementDependency | FormEntryBootstrapDependency;
  readonly status: FormPresentationStatus;
  readonly target: string | null;
  readonly reason: string;
  readonly digest: string | null;
}

export interface FormEntryBootstrapPolicy {
  readonly sourceAdmission?: boolean;
  readonly draftRestore?: boolean;
  readonly sourceCompatibility?: boolean;
  readonly validation?: boolean;
  readonly readiness?: boolean;
  readonly hostFacts?: boolean;
  readonly inputCapabilities?: boolean;
  readonly focusTarget?: boolean;
  readonly layoutMeasurement?: boolean;
}

export interface FormEntryBootstrapArtifact {
  readonly posture: "ready" | "pending" | "unavailable";
  readonly reason: string;
  readonly requirements: {
    readonly sourceAdmission: boolean;
    readonly draftRestore: boolean;
    readonly sourceCompatibility: boolean;
    readonly validation: boolean;
    readonly readiness: boolean;
    readonly hostFacts: boolean;
    readonly inputCapabilities: boolean;
    readonly focusTarget: boolean;
    readonly layoutMeasurement: boolean;
  };
  readonly sourceAdmission: FormSourceBootstrapArtifact | null;
  readonly draftRestore: FormSourceBootstrapArtifact | null;
  readonly hostUnavailableFacts: ReadonlyArray<string>;
  readonly inputUnavailableFields: ReadonlyArray<string>;
  readonly focusTarget: {
    readonly posture: "ready" | "unavailable" | "none";
    readonly field: string | null;
    readonly target: string | null;
    readonly reason: string;
  } | null;
  readonly layoutMeasurementPending: boolean;
  readonly dependencies: {
    readonly required: ReadonlyArray<FormPresentationDependencyArtifact>;
    readonly blocking: ReadonlyArray<FormPresentationDependencyArtifact>;
    readonly unavailable: ReadonlyArray<FormPresentationDependencyArtifact>;
    readonly satisfied: ReadonlyArray<FormPresentationDependencyArtifact>;
    readonly digest: string;
  };
  readonly digest: string;
}

export interface FormPresentationDeclaration {
  readonly entry?: FormEntryPresentationLanePolicy;
  readonly interaction?: FormPresentationLanePolicy;
  readonly availability?: FormPresentationLanePolicy;
  readonly messages?: FormPresentationLanePolicy;
  readonly layout?: FormPresentationLanePolicy;
  readonly action?: FormActionPresentationLanePolicy;
  readonly canonicalization?: FormPresentationLanePolicy;
  readonly resourceDrift?: FormPresentationLanePolicy;
  readonly collaboration?: FormPresentationLanePolicy;
  readonly attachments?: FormPresentationLanePolicy;
  readonly media?: FormPresentationLanePolicy;
  readonly handoff?: FormPresentationLanePolicy;
  readonly navigation?: FormPresentationLanePolicy;
  readonly exit?: FormPresentationLanePolicy;
}

export interface FormPresentationLaneUpdateArtifact {
  readonly kind: "presentationLaneUpdate";
  readonly artifactId: number;
  readonly observedAtMs: number;
  readonly laneId: string;
  readonly lane: "messages" | "collaboration" | "exit" | "attachments" | "media" | "handoff";
  readonly scope: FormPresentationScope;
  readonly status: FormPresentationStatus;
  readonly target: string | null;
  readonly reason: string;
  readonly token: string | null;
  readonly section: string | null;
  readonly scopeKind: "route" | "modal" | "external" | null;
  readonly surfaceId: string | null;
  readonly supersededByToken: string | null;
  readonly source: "report" | "clear" | "handoff";
  readonly presentationDigest: string;
}

export interface FormPresentationLifecycleArtifact {
  readonly id: string;
  readonly lane: FormPresentationLane;
  readonly scope: FormPresentationScope;
  readonly target: string | null;
  readonly status: FormPresentationStatus;
  readonly reason: string;
  readonly token: string | null;
  readonly policy: {
    readonly scope: FormPresentationScope;
    readonly delayedBusyRevealMs: number;
    readonly minimumBusyMs: number;
    readonly settlementAcknowledgement: "none" | "required";
    readonly settlementTimeoutMs: number;
    readonly supersessionHandoff: "replace" | "handoff";
    readonly unavailableAcknowledgement: "none" | "required";
    readonly settleOn: ReadonlyArray<FormActionPresentationSettlementDependency> | null;
    readonly bootstrap: FormEntryBootstrapPolicy | null;
  };
  readonly acknowledgement: {
    readonly required: boolean;
    readonly status: "pending" | "acknowledged" | "timedOut" | "ignored" | "noOp";
    readonly settlementDigest: string | null;
  };
  readonly bootstrap: FormEntryBootstrapArtifact | null;
  readonly dependencies: {
    readonly required: ReadonlyArray<FormPresentationDependencyArtifact>;
    readonly blocking: ReadonlyArray<FormPresentationDependencyArtifact>;
    readonly unavailable: ReadonlyArray<FormPresentationDependencyArtifact>;
    readonly satisfied: ReadonlyArray<FormPresentationDependencyArtifact>;
    readonly digest: string;
  } | null;
}

export interface FormPresentationSettlementArtifact {
  readonly kind: "presentationSettlement";
  readonly artifactId: number;
  readonly observedAtMs: number;
  readonly laneId: string;
  readonly lane: FormPresentationLane;
  readonly scope: FormPresentationScope;
  readonly token: string | null;
  readonly resultKind: "acknowledged" | "timedOut" | "ignored" | "noOp";
  readonly reason: string;
  readonly settlementDigest: string;
}

export type FormPresentationHistoryArtifact =
  | FormPresentationSettlementArtifact
  | FormPresentationLaneUpdateArtifact;

export interface FormPresentationReport {
  readonly lanes: ReadonlyArray<FormPresentationLifecycleArtifact>;
  readonly summary: {
    readonly total: number;
    readonly pending: number;
    readonly busy: number;
    readonly settling: number;
    readonly ready: number;
    readonly failed: number;
    readonly unavailable: number;
    readonly acknowledgementRequired: number;
  };
  readonly acknowledgements: {
    readonly required: number;
    readonly pending: number;
    readonly acknowledged: number;
    readonly timedOut: number;
    readonly ignored: number;
    readonly noOp: number;
    readonly digest: string | null;
  };
  readonly counters: {
    readonly costBasis: "derivedPresentationLifecycleScan";
    readonly incrementalStatus: "notIncremental";
    readonly lanes: number;
    readonly actionLanes: number;
    readonly navigationLanes: number;
    readonly resourceDriftLanes: number;
    readonly settlingLanes: number;
    readonly unavailableLanes: number;
    readonly requiredAcknowledgements: number;
    readonly settlementArtifacts: number;
    readonly interactionLanes: number;
    readonly canonicalizationLanes: number;
    readonly externalLanes: number;
  };
  readonly history: ReadonlyArray<FormPresentationHistoryArtifact>;
  readonly digest: string;
}
