export type ResourceEffectCompactInverseDescriptor =
  | {
      readonly kind: "compactPatchInverse";
      readonly mode: "CompactInversePatch";
      readonly preimage: "aspectValue";
      readonly scope: "aspect";
      readonly itemId: string;
      readonly field: null;
      readonly aspect: string;
      readonly summary: null;
      readonly patch: {
        readonly kind: "itemAspect";
        readonly itemId: string;
        readonly aspect: string;
        readonly value: unknown;
      };
      readonly cost: ResourceEffectCompactInverseCost;
    }
  | {
      readonly kind: "compactPatchInverse";
      readonly mode: "CompactInversePatch";
      readonly preimage: "itemFragment";
      readonly scope: "item";
      readonly itemId: string;
      readonly field: null;
      readonly aspect: null;
      readonly summary: null;
      readonly patch: {
        readonly kind: "item";
        readonly itemId: string;
        readonly nextItem: unknown;
      };
      readonly cost: ResourceEffectCompactInverseCost;
    }
  | {
      readonly kind: "compactPatchInverse";
      readonly mode: "CompactInversePatch";
      readonly preimage: "detailRegionValue";
      readonly scope: "region";
      readonly itemId: null;
      readonly field: null;
      readonly aspect: null;
      readonly summary: null;
      readonly region: string;
      readonly patch: {
        readonly kind: "region";
        readonly region: string;
        readonly value: unknown;
      };
      readonly cost: ResourceEffectCompactInverseCost;
    }
  | {
      readonly kind: "compactPatchInverse";
      readonly mode: "CompactInversePatch";
      readonly preimage: "detailJsonPathValue";
      readonly scope: "jsonPath";
      readonly itemId: null;
      readonly field: null;
      readonly aspect: null;
      readonly summary: null;
      readonly path: string;
      readonly patch: {
        readonly kind: "jsonPath";
        readonly path: string;
        readonly value: unknown;
      };
      readonly cost: ResourceEffectCompactInverseCost;
    }
  | {
      readonly kind: "compactPatchInverse";
      readonly mode: "CompactInversePatch";
      readonly preimage: "detailFieldValue";
      readonly scope: "field";
      readonly itemId: null;
      readonly aspect: null;
      readonly summary: null;
      readonly field: string;
      readonly patch: {
        readonly kind: "field";
        readonly field: string;
        readonly value: unknown;
      };
      readonly cost: ResourceEffectCompactInverseCost;
    }
  | {
      readonly kind: "compactPatchInverse";
      readonly mode: "CompactInversePatch";
      readonly preimage: "summaryValue";
      readonly scope: "summary";
      readonly itemId: null;
      readonly aspect: null;
      readonly summary: string;
      readonly patch: {
        readonly kind: "summary";
        readonly summary: string;
        readonly value: unknown;
      };
      readonly cost: ResourceEffectCompactInverseCost;
    };

export interface ResourceEffectCompactInverseCost {
  readonly retainedValueCount: 1;
  readonly retainedResponsePreimage: false;
}
