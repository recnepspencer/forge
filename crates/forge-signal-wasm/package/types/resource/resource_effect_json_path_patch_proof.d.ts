export interface ResourceEffectJsonPathPatchProof {
  readonly version: "resource-json-path-aspect-proof-v1";
  readonly aspect: string;
  readonly field: string;
  readonly path: readonly (string | number)[];
  readonly parsedPathDigest: string;
  readonly policy: {
    readonly presence: "required" | "optional";
    readonly absence: "deny" | "readAsNull";
    readonly containerWrite: "immutableCopy";
    readonly extensibility: "immutableCopy";
    readonly objectPrototype: "plainOrNull";
    readonly prototypeReconstruction: "plainOrNullCopy";
    readonly arrayIndex: "explicitExistingIndex";
    readonly accessor: "denyWithoutInvocation";
  };
  readonly cost: {
    readonly traversalBreadth: number;
    readonly reconstructionBreadth: number;
    readonly cloneBreadth: number;
  };
  readonly proofDigest: string;
}
