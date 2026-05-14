export type ResourceMutationResponseIdentityAtomicity =
  | "allOrNone"
  | "partialAllowed";

export type ResourceMutationResponseAtomicity =
  | "allOrNone"
  | "partialAllowed";

export interface ResourceMutationResponseIdentitySummaryTargetScope {
  readonly kind: "summary";
  readonly summary: string;
}

export interface ResourceMutationResponseIdentitySelectionTargetScope {
  readonly kind: "visibleSelection";
}

export interface ResourceMutationResponseIdentityDetailChildTargetScope {
  readonly kind: "detailChild";
  readonly region: string;
}

export type ResourceMutationResponseDetailReconciliationDeclaration =
  | {
      readonly kind: "replace";
    }
  | {
      readonly kind: "invalidate";
    }
  | {
      readonly kind: "field";
      readonly field: string;
    }
  | {
      readonly kind: "region";
      readonly region: string;
    }
  | {
      readonly kind: "jsonPath";
      readonly path: string;
    };

export type ResourceMutationResponseCollectionReconciliationDeclaration =
  | {
      readonly kind: "item";
    }
  | {
      readonly kind: "delete";
      readonly itemId?: (
        mutationParams: any,
        responseValue: any,
      ) => string;
    }
  | {
      readonly kind: "insert";
      readonly placement: "append" | "prepend";
    };

export interface ResourceMutationResponseSummaryReconciliationDeclaration {
  readonly kind: "summary";
  readonly summary: string;
}

export interface ResourceMutationResponseDiagnosticDeclaration {
  readonly kind: "validation" | "warnings";
  readonly field: string;
}

export interface ResourceMutationResponseTargetFamily {
  invalidate(params: unknown): boolean;
  invalidateAll(): number;
  line(params: unknown): {
    descriptor(): {
      readonly family: {
        readonly familyId: string;
        readonly kind: "detail" | "collection" | "paged";
      };
      readonly canonicalParams: {
        readonly canonicalKey: string;
      };
      readonly runtimeLineId: string;
    };
  };
}

export type ResourceMutationResponseTargetFamilyKind<
  TFamily extends ResourceMutationResponseTargetFamily,
> = TFamily extends {
  line(params: unknown): {
    descriptor(): {
      readonly family: {
        readonly kind: infer TKind;
      };
    };
  };
}
  ? TKind
  : never;

export type ResourceMutationResponseTargetFamilyParams<
  TFamily extends ResourceMutationResponseTargetFamily,
> = TFamily extends {
  line(params: infer TParams): {
    descriptor(): unknown;
  };
}
  ? TParams
  : never;

interface ResourceMutationResponseBaseIdentityTargetDeclaration<
  TMutationParams,
  TResponseValue,
  TFamily extends ResourceMutationResponseTargetFamily,
> {
  readonly family: TFamily;
  readonly params: (
    mutationParams: TMutationParams,
  ) => ResourceMutationResponseTargetFamilyParams<TFamily>;
  readonly canonicalParams?: (
    mutationParams: TMutationParams,
    responseValue: TResponseValue,
    canonicalIdentity: string,
    responseIdentity: string | null,
  ) => ResourceMutationResponseTargetFamilyParams<TFamily>;
  readonly fallback: import("./resource_mutation_response.js").ResourceMutationResponseIdentityFallbackKind;
}

export type ResourceMutationResponseResidentLineIdentityTargetDeclaration<
  TMutationParams,
  TResponseValue,
  TFamily extends ResourceMutationResponseTargetFamily,
> = ResourceMutationResponseBaseIdentityTargetDeclaration<
  TMutationParams,
  TResponseValue,
  TFamily
> & {
  readonly summary?: never;
  readonly selection?: never;
  readonly detailChild?: never;
};

export type ResourceMutationResponseSummaryIdentityTargetDeclaration<
  TMutationParams,
  TResponseValue,
  TFamily extends ResourceMutationResponseTargetFamily,
> = ResourceMutationResponseTargetFamilyKind<TFamily> extends
  | "collection"
  | "paged"
  ? ResourceMutationResponseBaseIdentityTargetDeclaration<
      TMutationParams,
      TResponseValue,
      TFamily
    > & {
      readonly summary: ResourceMutationResponseIdentitySummaryTargetScope;
      readonly selection?: never;
      readonly detailChild?: never;
    }
  : never;

export type ResourceMutationResponseSelectionIdentityTargetDeclaration<
  TMutationParams,
  TResponseValue,
  TFamily extends ResourceMutationResponseTargetFamily,
> = ResourceMutationResponseBaseIdentityTargetDeclaration<
  TMutationParams,
  TResponseValue,
  TFamily
> & {
  readonly summary?: never;
  readonly selection: ResourceMutationResponseIdentitySelectionTargetScope;
  readonly detailChild?: never;
};

export type ResourceMutationResponseDetailChildIdentityTargetDeclaration<
  TMutationParams,
  TResponseValue,
  TFamily extends ResourceMutationResponseTargetFamily,
> = ResourceMutationResponseTargetFamilyKind<TFamily> extends "detail"
  ? Omit<
      ResourceMutationResponseBaseIdentityTargetDeclaration<
        TMutationParams,
        TResponseValue,
        TFamily
      >,
      "canonicalParams"
    > & {
      readonly canonicalParams?: never;
      readonly summary?: never;
      readonly selection?: never;
      readonly detailChild: ResourceMutationResponseIdentityDetailChildTargetScope;
    }
  : never;

export type ResourceMutationResponseIdentityTargetDeclaration<
  TMutationParams,
  TResponseValue,
  TFamily extends ResourceMutationResponseTargetFamily,
> =
  | ResourceMutationResponseResidentLineIdentityTargetDeclaration<
      TMutationParams,
      TResponseValue,
      TFamily
    >
  | ResourceMutationResponseSummaryIdentityTargetDeclaration<
      TMutationParams,
      TResponseValue,
      TFamily
    >
  | ResourceMutationResponseSelectionIdentityTargetDeclaration<
      TMutationParams,
      TResponseValue,
      TFamily
    >
  | ResourceMutationResponseDetailChildIdentityTargetDeclaration<
      TMutationParams,
      TResponseValue,
      TFamily
    >;

export type ResourceMutationResponseAnyIdentityTargetDeclaration<
  TMutationParams,
  TResponseValue,
> =
  | ResourceMutationResponseResidentLineIdentityTargetDeclaration<
      TMutationParams,
      TResponseValue,
      ResourceMutationResponseTargetFamily
    >
  | (ResourceMutationResponseBaseIdentityTargetDeclaration<
      TMutationParams,
      TResponseValue,
      ResourceMutationResponseTargetFamily
    > & {
      readonly summary: ResourceMutationResponseIdentitySummaryTargetScope;
      readonly selection?: never;
      readonly detailChild?: never;
    })
  | ResourceMutationResponseSelectionIdentityTargetDeclaration<
      TMutationParams,
      TResponseValue,
      ResourceMutationResponseTargetFamily
    >
  | (Omit<
      ResourceMutationResponseBaseIdentityTargetDeclaration<
        TMutationParams,
        TResponseValue,
        ResourceMutationResponseTargetFamily
      >,
      "canonicalParams"
    > & {
      readonly canonicalParams?: never;
      readonly summary?: never;
      readonly selection?: never;
      readonly detailChild: ResourceMutationResponseIdentityDetailChildTargetScope;
    });

export interface ResourceMutationResponseIdentityDeclaration<
  TMutationParams,
  TResponseValue,
> {
  readonly submitted: (mutationParams: TMutationParams) => string;
  readonly response?: (responseValue: TResponseValue) => string;
  readonly canonical: (
    responseValue: TResponseValue,
    responseIdentity: string | null,
  ) => string;
  readonly atomicity?: ResourceMutationResponseIdentityAtomicity;
  readonly targets?: readonly ResourceMutationResponseAnyIdentityTargetDeclaration<
    TMutationParams,
    TResponseValue
  >[];
}

export interface ResourceMutationResponseTargetDeclaration<
  TMutationParams,
  TFamily extends ResourceMutationResponseTargetFamily,
> {
  readonly family: TFamily;
  readonly params: (
    mutationParams: TMutationParams,
  ) => ResourceMutationResponseTargetFamilyParams<TFamily>;
  readonly fallback: import("./resource_mutation_response.js").ResourceMutationResponseFallbackKind;
  readonly detail?: ResourceMutationResponseDetailReconciliationDeclaration;
  readonly collection?: ResourceMutationResponseCollectionReconciliationDeclaration;
  readonly summary?: ResourceMutationResponseSummaryReconciliationDeclaration;
}

export type ResourceMutationResponseAnyTargetDeclaration<TMutationParams> =
  ResourceMutationResponseTargetDeclaration<
    TMutationParams,
    ResourceMutationResponseTargetFamily
  >;

export type ResourceMutationResponseCreateTargetDeclaration<
  TMutationParams,
  TFamily extends ResourceMutationResponseTargetFamily,
> =
  | ResourceMutationResponseFallbackTargetDeclaration<TMutationParams>
  | (ResourceMutationResponseTargetFamilyKind<TFamily> extends "detail"
      ? Omit<
          ResourceMutationResponseTargetDeclaration<TMutationParams, TFamily>,
          "summary" | "collection"
        > & {
          readonly detail: ResourceMutationResponseDetailReconciliationDeclaration;
          readonly collection?: never;
          readonly summary?: never;
        }
      : never)
  | (Omit<
      ResourceMutationResponseTargetDeclaration<TMutationParams, TFamily>,
      "detail" | "summary" | "collection"
    > & {
      readonly detail?: never;
      readonly summary?: never;
      readonly collection: {
        readonly kind: "insert";
        readonly placement: "append" | "prepend";
      };
    })
  | (Omit<
      ResourceMutationResponseTargetDeclaration<TMutationParams, TFamily>,
      "detail" | "summary" | "collection"
    > & {
      readonly detail?: never;
      readonly collection?: never;
      readonly summary: ResourceMutationResponseSummaryReconciliationDeclaration;
    });

export type ResourceMutationResponseAnyCreateTargetDeclaration<TMutationParams> =
  ResourceMutationResponseCreateTargetDeclaration<
    TMutationParams,
    ResourceMutationResponseTargetFamily
  >;

export type ResourceMutationResponseFallbackTargetDeclaration<TMutationParams> =
  Omit<
    ResourceMutationResponseAnyTargetDeclaration<TMutationParams>,
    "detail" | "collection" | "summary"
  > & {
    readonly detail?: never;
    readonly collection?: never;
    readonly summary?: never;
  };
