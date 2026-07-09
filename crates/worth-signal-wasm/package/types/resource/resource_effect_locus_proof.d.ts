export type ResourceEffectLocus =
  | { readonly kind: "line" }
  | { readonly kind: "broadResponse" }
  | { readonly kind: "detailResponse" }
  | { readonly kind: "detailField"; readonly field: string | null }
  | { readonly kind: "detailRegion"; readonly region: string | null }
  | { readonly kind: "detailJsonPath"; readonly path: string | null }
  | { readonly kind: "summaryResponse" }
  | { readonly kind: "membership"; readonly itemId: string | null }
  | { readonly kind: "connection"; readonly itemId: string | null }
  | { readonly kind: "discriminatedTuple"; readonly itemId: string | null }
  | { readonly kind: "entityStore"; readonly itemId: string | null }
  | { readonly kind: "groupedCollection"; readonly itemId: string | null }
  | { readonly kind: "mapCollection"; readonly itemId: string | null }
  | { readonly kind: "namedCollection"; readonly itemId: string | null }
  | { readonly kind: "recursiveTree"; readonly itemId: string | null }
  | { readonly kind: "sparsePage"; readonly itemId: string | null }
  | { readonly kind: "item"; readonly itemId: string | null }
  | {
      readonly kind: "itemAspect";
      readonly itemId: string | null;
      readonly aspect: string | null;
    }
  | {
      readonly kind: "jsonItemAspect";
      readonly itemId: string | null;
      readonly aspect: string | null;
    }
  | { readonly kind: "summary"; readonly summary: string | null }
  | { readonly kind: "basis" }
  | { readonly kind: "invalidation" };

export interface ResourceEffectLocusProof {
  readonly version: "resource-effect-locus-proof-v1";
  readonly lensVersion: "resource-response-lens-proof-v1";
  readonly lensSource: string;
  readonly declarationDigest: string;
  readonly capabilityDigest: string;
  readonly compiledLensDigest: string;
  readonly parityDigest: string;
  readonly compileBoundaryDigest: string;
  readonly capabilityRowDigest: string;
  readonly effectLocusDigest: string;
  readonly topology:
    | "directArray"
    | "objectItems"
    | "customCollection"
    | "connection"
    | "discriminatedTuple"
    | "entityStore"
    | "groupedCollection"
    | "mapCollection"
    | "namedCollection"
    | "recursiveTree"
    | "sparsePage"
    | "detail"
    | "summary";
  readonly itemField: string | null;
  readonly locus:
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
  readonly patchScope: "line" | "field" | "region" | "jsonPath" | "item" | "aspect" | "summary";
  readonly field: string | null;
  readonly region?: string | null;
  readonly path?: string | null;
  readonly aspect: string | null;
  readonly summary: string | null;
  readonly summaryPatchScope: "line" | "pageWindow" | null;
  readonly cost: ResourceEffectLocusCostCounters;
  readonly proofBreadth: 1;
}

export interface ResourceEffectLocusCostCounters {
  readonly lookup: string;
  readonly lookupBreadth: number;
  readonly traversal: string;
  readonly traversalBreadth: number;
  readonly reconstruction: string;
  readonly reconstructionBreadth: number;
}
