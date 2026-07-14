export type FormLayoutMeasurementCause =
  | "resizeObserver"
  | "fontLoad"
  | "viewport"
  | "contentGrowth"
  | "asyncMessage"
  | "textareaGrowth"
  | "animationFrame";

export interface FormLayoutMeasurementDeclaration {
  readonly observe?: ReadonlyArray<FormLayoutMeasurementCause>;
  readonly maxRetainedSnapshots?: number;
}

export interface FormLayoutRowMeasurement {
  readonly row: string;
  readonly labelHeight?: number;
  readonly controlHeight?: number;
  readonly helpHeight?: number;
  readonly messageHeight?: number;
}

export interface FormLayoutSnapshotArtifact {
  readonly kind: "layoutSnapshot";
  readonly snapshotId: number;
  readonly frameToken: string | number | null;
  readonly causes: ReadonlyArray<FormLayoutMeasurementCause>;
  readonly rows: ReadonlyArray<{
    readonly row: string;
    readonly labelHeight: number | null;
    readonly controlHeight: number | null;
    readonly helpHeight: number | null;
    readonly messageHeight: number | null;
  }>;
  readonly layoutDigest: string;
  readonly accessibilityDigest: string;
  readonly hostDigest: string;
  readonly semanticDigests: {
    readonly validationDigest: string;
    readonly readinessDigest: string;
    readonly actionPlanDigestSetDigest: string;
  };
  readonly snapshotDigest: string;
}

export interface FormLayoutMeasurementReport {
  readonly posture: "supported";
  readonly policy: {
    readonly observe: ReadonlyArray<FormLayoutMeasurementCause>;
    readonly batching: "animationFrameCoalesced";
    readonly maxRetainedSnapshots: number;
  };
  readonly latestSnapshot: FormLayoutSnapshotArtifact | null;
  readonly snapshots: ReadonlyArray<FormLayoutSnapshotArtifact>;
  readonly counters: {
    readonly costBasis: "imperativeLayoutMeasurementEventStream";
    readonly incrementalStatus: "frameCoalesced";
    readonly retainedSnapshots: number;
    readonly coalescedWrites: number;
    readonly observedCauseCount: number;
    readonly measuredRows: number;
  };
  readonly digest: string;
}
