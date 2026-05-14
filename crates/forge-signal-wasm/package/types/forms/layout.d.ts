export interface FormLayoutFieldHint {
  readonly field: string;
  readonly path: string;
  readonly section: string | null;
  readonly row: string;
  readonly column: string;
  readonly tracks: {
    readonly label: "declared";
    readonly control: "declared";
    readonly help: "declared" | "omitted";
    readonly message: "declared" | "omitted";
  };
  readonly density: "compact" | "comfortable" | "spacious";
  readonly alignment: "start" | "center" | "stretch";
  readonly minHeight: number | null;
  readonly grow: boolean;
  readonly wrap: boolean;
  readonly responsive: ReadonlyArray<string>;
  readonly capabilityPosture: {
    readonly posture: "supported" | "unavailable";
    readonly unavailableCapabilities: ReadonlyArray<string>;
    readonly reason: string | null;
  };
}

export interface FormLayoutSectionHint {
  readonly id: string;
  readonly group: string | null;
  readonly order: number;
  readonly density: "compact" | "comfortable" | "spacious";
  readonly alignment: "start" | "center" | "stretch";
  readonly responsive: ReadonlyArray<string>;
  readonly fields: ReadonlyArray<string>;
  readonly rows: ReadonlyArray<string>;
}

export interface FormLayoutRowHint {
  readonly id: string;
  readonly section: string | null;
  readonly fields: ReadonlyArray<string>;
  readonly columns: ReadonlyArray<string>;
  readonly maxMinHeight: number | null;
  readonly growFields: ReadonlyArray<string>;
  readonly wrap: boolean;
}

export interface FormLayoutReport {
  readonly sections: ReadonlyArray<FormLayoutSectionHint>;
  readonly rows: ReadonlyArray<FormLayoutRowHint>;
  readonly fields: ReadonlyArray<FormLayoutFieldHint>;
  readonly summary: {
    readonly sections: number;
    readonly rows: number;
    readonly fields: number;
    readonly unavailableFields: number;
    readonly responsiveFields: number;
    readonly messageTrackFields: number;
  };
  readonly counters: {
    readonly costBasis: "derivedLayoutHintScan";
    readonly incrementalStatus: "notIncremental";
    readonly sections: number;
    readonly rows: number;
    readonly fields: number;
    readonly responsiveTokens: number;
    readonly minHeightHints: number;
    readonly growFields: number;
    readonly wrapRows: number;
    readonly unavailableFields: number;
  };
  readonly digest: string;
}
