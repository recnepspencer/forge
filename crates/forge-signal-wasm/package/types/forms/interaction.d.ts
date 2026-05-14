export type FormInteractionInputSource =
  | "typing"
  | "paste"
  | "drop"
  | "autofill";

export interface FormFieldInteractionArtifact {
  readonly kind: "fieldInteraction";
  readonly artifactId: number;
  readonly field: string;
  readonly interaction:
    | "touched"
    | "visited"
    | "focused"
    | "blurred"
    | "input"
    | "compositionStarted"
    | "compositionCommitted"
    | "compositionCancelled";
  readonly source: string;
  readonly rawDigest: string | null;
  readonly interactionDigest: string;
}

export interface FormSubmitIntentArtifact {
  readonly kind: "submitIntent";
  readonly artifactId: number;
  readonly source: "keyboard" | "pointer" | "programmatic" | null;
  readonly resultKind: "reported" | "cleared";
  readonly reason: string | null;
  readonly intentDigest: string;
}

export interface FormFieldInteractionState {
  readonly field: string;
  readonly path: string;
  readonly touched: boolean;
  readonly visited: boolean;
  readonly focused: boolean;
  readonly focusIntent: boolean;
  readonly blurred: boolean;
  readonly lastInputSource: FormInteractionInputSource | null;
  readonly composing: boolean;
  readonly compositionDigest: string | null;
  readonly rawInputPosture: {
    readonly posture: "supported" | "unavailable";
    readonly reason: string | null;
  };
  readonly compositionPosture: {
    readonly posture: "supported" | "unavailable";
    readonly reason: string | null;
  };
  readonly focusPosture: "supported" | "unavailable";
  readonly focusReason: string | null;
  readonly interactionDigest: string;
}

export interface FormInteractionReport {
  readonly fields: ReadonlyArray<FormFieldInteractionState>;
  readonly summary: {
    readonly fields: number;
    readonly touchedFields: number;
    readonly visitedFields: number;
    readonly focusedField: string | null;
    readonly focusIntentField: string | null;
    readonly focusPosture: "supported" | "unavailable";
    readonly composingFields: number;
    readonly rawInputUnavailableFields: number;
    readonly compositionUnavailableFields: number;
    readonly inputSources: {
      readonly typing: number;
      readonly paste: number;
      readonly drop: number;
      readonly autofill: number;
    };
    readonly submitIntent: {
      readonly active: boolean;
      readonly source: "keyboard" | "pointer" | "programmatic" | null;
      readonly count: number;
    };
  };
  readonly counters: {
    readonly costBasis: "interactionArtifactAndHostFocusScan";
    readonly incrementalStatus: "notIncremental";
    readonly fields: number;
    readonly touchedFields: number;
    readonly visitedFields: number;
    readonly focusedFields: number;
    readonly composingFields: number;
    readonly rawInputUnavailableFields: number;
    readonly compositionUnavailableFields: number;
    readonly inputArtifacts: number;
    readonly compositionArtifacts: number;
    readonly focusArtifacts: number;
    readonly submitIntentArtifacts: number;
    readonly interactionArtifacts: number;
  };
  readonly history: ReadonlyArray<FormFieldInteractionArtifact | FormSubmitIntentArtifact>;
  readonly digest: string;
}
