export interface ResourceMutationResponseSubmittedTargetBasis {
  readonly targetId: string;
  readonly familyKind: "detail" | "collection" | "paged";
  readonly familyId: string;
  readonly canonicalKey: string;
  readonly runtimeLineId: string | null;
  readonly residency: "declared" | "resident";
  readonly basisId: string | null;
  readonly visibleValueVersion: number | null;
  readonly digest: string;
}

export interface ResourceMutationResponseTargetStaleness {
  readonly kind: "staleTarget";
  readonly reason:
    | "canonicalKeyChanged"
    | "basisChanged"
    | "visibleValueVersionChanged";
  readonly submittedBasisId: string | null;
  readonly currentBasisId: string | null;
  readonly submittedVisibleValueVersion: number | null;
  readonly currentVisibleValueVersion: number | null;
  readonly submittedCanonicalKey: string;
  readonly currentCanonicalKey: string;
  readonly detail: string;
}
