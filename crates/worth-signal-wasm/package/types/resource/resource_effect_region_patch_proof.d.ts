export interface ResourceEffectRegionPatchProof {
  readonly version: "resource-detail-region-proof-v1";
  readonly regionName: string;
  readonly identityBoundary: "inside" | "outside";
  readonly mergeGranularity: string;
  readonly cost: {
    readonly traversalBreadth: number;
    readonly reconstructionBreadth: number;
    readonly cloneBreadth: number;
  };
  readonly proofDigest: string;
}
