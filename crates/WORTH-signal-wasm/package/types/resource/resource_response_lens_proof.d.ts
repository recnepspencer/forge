declare const WORTHSignalResourceResponseLensProofBrand: unique symbol;

export type ResourceResponseLensTopology =
  | "directArray"
  | "objectItems"
  | "customCollection"
  | "connection"
  | "discriminatedTuple"
  | "groupedCollection"
  | "entityStore"
  | "mapCollection"
  | "namedCollection"
  | "recursiveTree"
  | "sparsePage"
  | "detail"
  | "summary";

export type ResourceResponseLensLocus =
  | "broadResponse"
  | "detailResponse"
  | "detailField"
  | "detailRegion"
  | "detailJsonPath"
  | "summaryResponse"
  | "membership"
  | "connection"
  | "discriminatedTuple"
  | "entityStore"
  | "groupedCollection"
  | "mapCollection"
  | "namedCollection"
  | "recursiveTree"
  | "sparsePage"
  | "itemAspect"
  | "jsonItemAspect"
  | "summary";

export interface ResourceResponseLensCapabilityRow {
  readonly locus: ResourceResponseLensLocus;
  readonly patchScope: "line" | "field" | "region" | "jsonPath" | "item" | "aspect" | "summary";
  readonly admitted: boolean;
  readonly summaryPatchScope: "line" | "pageWindow" | null;
}

export interface ResourceResponseLensProof {
  readonly version: "resource-response-lens-proof-v1";
  readonly source: string;
  readonly topology: ResourceResponseLensTopology;
  readonly itemField: string | null;
  readonly declarationDigest: string;
  readonly capabilityDigest: string;
  readonly compiledLensDigest: string;
  readonly parityDigest: string;
  readonly compileBoundaryDigest: string;
  readonly capabilityRows: readonly ResourceResponseLensCapabilityRow[];
  readonly fieldNames: readonly string[];
  readonly regionNames: readonly string[];
  readonly jsonPathNames: readonly string[];
  readonly aspectNames: readonly string[];
  readonly jsonAspectNames: readonly string[];
  readonly summaryNames: readonly string[];
  readonly summaryPatchScope: "line" | "pageWindow" | null;
  readonly [WORTHSignalResourceResponseLensProofBrand]: "resourceResponseLensProof";
}

export interface ResourceResponseLensDenialProof {
  readonly version: "resource-response-lens-denial-proof-v1";
  readonly lensVersion: "resource-response-lens-proof-v1";
  readonly lensSource: string;
  readonly declarationDigest: string;
  readonly capabilityDigest: string;
  readonly compiledLensDigest: string;
  readonly parityDigest: string;
  readonly compileBoundaryDigest: string;
  readonly requestedLocus: ResourceResponseLensLocus | string;
  readonly requestedPatchScope: "line" | "field" | "region" | "jsonPath" | "item" | "aspect" | "summary" | null;
  readonly field: string | null;
  readonly region?: string | null;
  readonly path?: string | null;
  readonly aspect: string | null;
  readonly summary: string | null;
  readonly reason:
    | "unsupportedCapability"
    | "undeclaredField"
    | "undeclaredRegion"
    | "undeclaredJsonPath"
    | "undeclaredAspect"
    | "undeclaredJsonAspect"
    | "undeclaredSummary"
    | "pagedSummaryScopeMismatch"
    | "listSummaryScopeMismatch";
  readonly denialDigest: string;
}
