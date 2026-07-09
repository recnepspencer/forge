export interface ResourceEffectFieldPatchProof {
  readonly version: "resource-detail-field-proof-v1";
  readonly fieldName: string;
  readonly cost: {
    readonly traversalBreadth: number;
    readonly reconstructionBreadth: number;
    readonly cloneBreadth: number;
  };
  readonly proofDigest: string;
}
