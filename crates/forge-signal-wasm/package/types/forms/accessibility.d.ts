export type FormAccessibilityAnnouncementPriority =
  | "off"
  | "polite"
  | "assertive";

export interface FormAccessibilityFieldArtifact {
  readonly kind: "fieldAccessibility";
  readonly field: string;
  readonly path: string;
  readonly label: string;
  readonly description: string | null;
  readonly summaryLabel: string;
  readonly describedBy: ReadonlyArray<string>;
  readonly messageIds: ReadonlyArray<string>;
  readonly summaryMessageIds: ReadonlyArray<string>;
  readonly required: boolean;
  readonly invalid: boolean;
  readonly disabled: boolean;
  readonly readonly: boolean;
  readonly hidden: boolean;
  readonly readingOrder: number;
  readonly focusOrder: number;
  readonly summaryOrder: number;
  readonly announcementPriority: FormAccessibilityAnnouncementPriority;
  readonly focusCapability: {
    readonly posture: "supported" | "unavailable";
    readonly reason: string | null;
  };
}

export interface FormAccessibilityMessageArtifact {
  readonly id: string;
  readonly code: string;
  readonly target: string | null;
  readonly visibility: "visible" | "summary" | "blocked";
  readonly severity: "info" | "warning" | "error";
  readonly announce: FormAccessibilityAnnouncementPriority;
  readonly describedBy: ReadonlyArray<string>;
  readonly summaryOrder: number;
  readonly focusTarget: string | null;
}

export interface FormAccessibilitySectionArtifact {
  readonly id: string;
  readonly group: string | null;
  readonly order: number;
  readonly posture: string;
  readonly fields: ReadonlyArray<string>;
}

export interface FormAccessibilityReport {
  readonly fields: ReadonlyArray<FormAccessibilityFieldArtifact>;
  readonly messages: ReadonlyArray<FormAccessibilityMessageArtifact>;
  readonly sections: ReadonlyArray<FormAccessibilitySectionArtifact>;
  readonly focusTarget: {
    readonly posture: "ready" | "unavailable" | "none";
    readonly field: string | null;
    readonly target: string | null;
    readonly reason: string;
  };
  readonly orderHints: {
    readonly readingOrder: ReadonlyArray<string>;
    readonly focusOrder: ReadonlyArray<string>;
    readonly sectionOrder: ReadonlyArray<string>;
    readonly summaryOrder: ReadonlyArray<string>;
  };
  readonly orderDigest: string | null;
  readonly summary: {
    readonly fields: number;
    readonly invalidFields: number;
    readonly requiredFields: number;
    readonly readonlyFields: number;
    readonly disabledFields: number;
    readonly hiddenFields: number;
    readonly messages: number;
    readonly summaryMessages: number;
  };
  readonly counters: {
    readonly costBasis: "derivedAccessibilityArtifactScan";
    readonly incrementalStatus: "notIncremental";
    readonly declaredFields: number;
    readonly describedRelationships: number;
    readonly invalidFields: number;
    readonly requiredFields: number;
    readonly readonlyFields: number;
    readonly disabledFields: number;
    readonly hiddenFields: number;
    readonly summaryMessages: number;
    readonly sections: number;
    readonly focusUnavailableFields: number;
  };
  readonly digest: string;
}
