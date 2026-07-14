import type { SignalValue } from "../model.js";

declare const WorthSignalResourceDetailRegionsBrand: unique symbol;

export interface ResourceDetailRegion<TValue, TRegionValue = SignalValue> {
  read(value: TValue): TRegionValue;
  write(value: TValue, regionValue: TRegionValue): TValue;
  readonly identityBoundary: "inside" | "outside";
  readonly mergeGranularity: string;
}

export type ResourceDetailRegionMap<TValue> = Readonly<
  Record<string, ResourceDetailRegion<TValue, any>>
>;

export type ResourceDetailRegionValue<TRegion> =
  TRegion extends ResourceDetailRegion<any, infer TValue> ? TValue : never;

export interface ResourceDetailRegions<
  TValue,
  TRegionMap extends ResourceDetailRegionMap<TValue> = ResourceDetailRegionMap<TValue>,
> {
  readonly definitions: TRegionMap;
  readonly [WorthSignalResourceDetailRegionsBrand]: "resourceDetailRegions";
}
