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

export interface FormPresentationLanePolicy {
  readonly scope?: FormPresentationScope;
  readonly delayedBusyRevealMs?: number;
  readonly minimumBusyMs?: number;
  readonly settlementAcknowledgement?: "none" | "required";
  readonly settlementTimeoutMs?: number;
  readonly supersessionHandoff?: "replace" | "handoff";
  readonly unavailableAcknowledgement?: "none" | "required";
}

export interface FormPresentationDeclaration {
  readonly entry?: FormPresentationLanePolicy;
  readonly interaction?: FormPresentationLanePolicy;
  readonly availability?: FormPresentationLanePolicy;
  readonly messages?: FormPresentationLanePolicy;
  readonly layout?: FormPresentationLanePolicy;
  readonly action?: FormPresentationLanePolicy;
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
  readonly lane: "collaboration" | "exit" | "attachments" | "media" | "handoff";
  readonly scope: FormPresentationScope;
  readonly status: FormPresentationStatus;
  readonly target: string | null;
  readonly reason: string;
  readonly token: string | null;
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
  };
  readonly acknowledgement: {
    readonly required: boolean;
    readonly status: "pending" | "acknowledged" | "timedOut" | "ignored" | "noOp";
    readonly settlementDigest: string | null;
  };
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
