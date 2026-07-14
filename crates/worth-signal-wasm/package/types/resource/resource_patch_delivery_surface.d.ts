import type { ResourceDetailFieldMap } from "./resource_detail_fields.js";
import type { ResourceDetailJsonPathMap } from "./resource_detail_json_paths.js";
import type { ResourceDetailRegionMap } from "./resource_detail_regions.js";
import type {
  DeleteResourcePatch,
  FieldResourcePatch,
  InvalidateResourceDelivery,
  InsertResourcePatch,
  ItemAspectResourcePatch,
  ItemResourcePatch,
  JsonPathResourcePatch,
  PatchResourceDelivery,
  RegionResourcePatch,
  ReplaceResourceDelivery,
  ReplaceResourcePatch,
  ResourceCollectionShape,
  ResourceItemAspectMap,
  ResourcePatchForReconcile,
  ResourceSummaryPatchScope,
  ResourceValueSummaryMap,
  SummaryResourcePatch,
} from "./resource_reconciliation.js";

export interface ReplacedResourcePatchResult {
  readonly kind: "replaced";
  readonly scope: "line";
  readonly itemId: null;
  readonly aspect: null;
}

export interface NarrowedItemPatchResult {
  readonly kind: "narrowed";
  readonly scope: "item";
  readonly itemId: string;
  readonly aspect: null;
}

export interface NarrowedDetailFieldPatchResult {
  readonly kind: "narrowed";
  readonly scope: "field";
  readonly itemId: null;
  readonly aspect: null;
  readonly field: string;
}

export interface NarrowedDetailRegionPatchResult {
  readonly kind: "narrowed";
  readonly scope: "region";
  readonly itemId: null;
  readonly aspect: null;
  readonly region: string;
}

export interface NarrowedDetailJsonPathPatchResult {
  readonly kind: "narrowed";
  readonly scope: "jsonPath";
  readonly itemId: null;
  readonly aspect: null;
  readonly path: string;
}

export interface NarrowedItemAspectPatchResult {
  readonly kind: "narrowed";
  readonly scope: "aspect";
  readonly itemId: string;
  readonly aspect: string;
}

export interface NarrowedSummaryPatchResult {
  readonly kind: "narrowed";
  readonly scope: "summary";
  readonly itemId: null;
  readonly aspect: null;
  readonly summary: string;
}

export type ResourcePatchResult =
  | ReplacedResourcePatchResult
  | NarrowedDetailFieldPatchResult
  | NarrowedDetailRegionPatchResult
  | NarrowedDetailJsonPathPatchResult
  | NarrowedItemPatchResult
  | NarrowedItemAspectPatchResult
  | NarrowedSummaryPatchResult;

export interface ResourceLineReconciliation<
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem> = {},
  TSummaryMap extends ResourceValueSummaryMap<any> = {},
  TFieldMap extends ResourceDetailFieldMap<any> = {},
  TRegionMap extends ResourceDetailRegionMap<any> = {},
  TJsonPathMap extends ResourceDetailJsonPathMap<any> = {},
  TNarrowItem extends boolean = boolean,
  TNarrowField extends boolean = boolean,
  TNarrowRegion extends boolean = boolean,
  TNarrowJsonPath extends boolean = boolean,
  TNarrowSummary extends boolean = boolean,
> {
  readonly broadReplace: true;
  readonly narrowItem: TNarrowItem;
  readonly narrowField: TNarrowField;
  readonly narrowRegion: TNarrowRegion;
  readonly narrowJsonPath: TNarrowJsonPath;
  readonly narrowSummary: TNarrowSummary;
  readonly fieldNames: readonly (keyof TFieldMap & string)[];
  readonly regionNames: readonly (keyof TRegionMap & string)[];
  readonly jsonPathNames: readonly (keyof TJsonPathMap & string)[];
  readonly aspectNames: readonly (keyof TAspectMap & string)[];
  readonly summaryNames: readonly (keyof TSummaryMap & string)[];
}

export interface ResourcePatchFactory {
  replace<TValue>(nextValue: TValue): ReplaceResourcePatch<TValue>;
  field<TField extends string, TValue>(options: {
    field: TField;
    value: TValue;
  }): FieldResourcePatch<TField, TValue>;
  region<TRegion extends string, TValue>(options: {
    region: TRegion;
    value: TValue;
  }): RegionResourcePatch<TRegion, TValue>;
  jsonPath<TPath extends string, TValue>(options: {
    path: TPath;
    value: TValue;
  }): JsonPathResourcePatch<TPath, TValue>;
  item<TItem>(options: {
    itemId: string;
    nextItem: TItem;
  }): ItemResourcePatch<TItem>;
  delete(options: {
    itemId: string;
  }): DeleteResourcePatch;
  insert<TItem>(options: {
    itemId: string;
    placement: "append" | "prepend";
    nextItem: TItem;
  }): InsertResourcePatch<TItem>;
  itemAspect<TAspect extends string, TValue>(options: {
    itemId: string;
    aspect: TAspect;
    value: TValue;
  }): ItemAspectResourcePatch<TAspect, TValue>;
  summary<TSummary extends string, TValue>(options: {
    summary: TSummary;
    value: TValue;
  }): SummaryResourcePatch<TSummary, TValue>;
}

export interface ExternalReplaceResourceDelivery<TValue> {
  readonly kind: "replace"; readonly packetId: string;
  readonly basisId: string | null; readonly nextBasisId: string | null | undefined;
  readonly nextValue: TValue; readonly version: "worth-resource-external-delivery-v1";
  readonly contract: "basis-compat-v1";
}

export interface ExternalPatchResourceDelivery<
  TValue, TItem, TReconcile,
  TFamilyKind extends "detail" | "collection" | "paged" = "collection",
> {
  readonly kind: "patch"; readonly packetId: string;
  readonly basisId: string | null; readonly nextBasisId: string | null | undefined;
  readonly patch: ResourcePatchForReconcile<TValue, TItem, TReconcile, TFamilyKind>;
  readonly version: "worth-resource-external-delivery-v1";
  readonly contract: "basis-compat-v1";
}

export interface ExternalInvalidateResourceDelivery {
  readonly kind: "invalidate"; readonly packetId: string;
  readonly basisId: string | null; readonly nextBasisId: string | null | undefined;
  readonly version: "worth-resource-external-delivery-v1";
  readonly contract: "basis-compat-v1";
}

export interface ExternalBasisRefreshResourceDelivery {
  readonly kind: "basisRefresh"; readonly packetId: string;
  readonly basisId: string | null; readonly nextBasisId: string;
  readonly version: "worth-resource-external-delivery-v1";
  readonly contract: "basis-compat-v1";
}

export type ResourceDeliveryForReconcile<
  TValue, TItem, TReconcile,
  TFamilyKind extends "detail" | "collection" | "paged" = "collection",
> =
  | ReplaceResourceDelivery<TValue>
  | PatchResourceDelivery<TValue, TItem, TReconcile, TFamilyKind>
  | InvalidateResourceDelivery
  | ExternalReplaceResourceDelivery<TValue>
  | ExternalPatchResourceDelivery<TValue, TItem, TReconcile, TFamilyKind>
  | ExternalInvalidateResourceDelivery
  | ExternalBasisRefreshResourceDelivery;

export interface AppliedResourceDeliveryResult {
  readonly kind: "applied"; readonly deliveryKind: "replace" | "patch" | "invalidate";
  readonly scope: "line" | "field" | "region" | "jsonPath" | "item" | "aspect" | "summary" | "invalidate";
  readonly path?: string | null;
  readonly packetId: string; readonly basisId: string | null;
  readonly nextBasisId: string | null;
  readonly supersededOperation: "initialLoad" | "refresh" | "revalidate" | null;
}

export interface DuplicateIgnoredResourceDeliveryResult {
  readonly kind: "duplicateIgnored"; readonly packetId: string;
  readonly deliveryKind: "replace" | "patch" | "invalidate" | "basisRefresh";
}

export interface BasisRejectedResourceDeliveryResult {
  readonly kind: "basisRejected"; readonly packetId: string;
  readonly expectedBasisId: string | null; readonly actualBasisId: string;
}

export interface BasisRefreshedResourceDeliveryResult {
  readonly kind: "basisRefreshed"; readonly packetId: string;
  readonly basisId: string | null; readonly nextBasisId: string | null;
  readonly reloadStatus:
    | import("./resource_lifecycle.js").ResourceLinePendingStatus
    | import("./resource_lifecycle.js").ResourceLineFulfilledStatus
    | import("./resource_lifecycle.js").ResourceLineTimedOutStatus
    | import("./resource_lifecycle.js").ResourceLineRejectedStatus;
}

export type ResourceDeliveryResult =
  | AppliedResourceDeliveryResult
  | DuplicateIgnoredResourceDeliveryResult
  | BasisRejectedResourceDeliveryResult
  | BasisRefreshedResourceDeliveryResult;

export interface ResourceDeliveryFactory {
  replace<TValue>(options: {
    packetId: string; basisId?: string | null;
    nextBasisId?: string | null; nextValue: TValue;
  }): ReplaceResourceDelivery<TValue>;
  patch<
    TValue, TItem, TReconcile,
    TFamilyKind extends "detail" | "collection" | "paged" = "collection",
  >(options: {
    packetId: string; basisId?: string | null; nextBasisId?: string | null;
    patch: ResourcePatchForReconcile<TValue, TItem, TReconcile, TFamilyKind>;
  }): PatchResourceDelivery<TValue, TItem, TReconcile, TFamilyKind>;
  invalidate(options: {
    packetId: string; basisId?: string | null; nextBasisId?: string | null;
  }): InvalidateResourceDelivery;
}

export interface ResourceExternalDeliveryFactory {
  replace<TValue>(options: {
    packetId: string; basisId?: string | null;
    nextBasisId?: string | null; nextValue: TValue;
  }): ExternalReplaceResourceDelivery<TValue>;
  patch<
    TValue, TItem, TReconcile,
    TFamilyKind extends "detail" | "collection" | "paged" = "collection",
  >(options: {
    packetId: string; basisId?: string | null; nextBasisId?: string | null;
    patch: ResourcePatchForReconcile<TValue, TItem, TReconcile, TFamilyKind>;
  }): ExternalPatchResourceDelivery<TValue, TItem, TReconcile, TFamilyKind>;
  invalidate(options: {
    packetId: string; basisId?: string | null; nextBasisId?: string | null;
  }): ExternalInvalidateResourceDelivery;
  basisRefresh(options: {
    packetId: string; basisId?: string | null; nextBasisId: string;
  }): ExternalBasisRefreshResourceDelivery;
}
