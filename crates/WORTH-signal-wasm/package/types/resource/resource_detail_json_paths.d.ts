import type { SignalValue } from "../model.js";
import type { ResourceEffectJsonPathPatchProof } from "./resource_effect_json_path_patch_proof.js";

declare const WORTHSignalResourceDetailJsonPathsBrand: unique symbol;

export type ResourceDetailJsonPathSegment = string | number;
export type ResourceDetailJsonPathPresence = "required" | "optional";

export interface ResourceDetailJsonPathDefinition {
  readonly path: ReadonlyArray<ResourceDetailJsonPathSegment>;
  readonly presence?: ResourceDetailJsonPathPresence;
}

export type ResourceDetailJsonPathDefinitionMap<TValue> = Readonly<
  Record<string, ResourceDetailJsonPathDefinition>
>;
export type ResourceDetailJsonPathDeclaration<TValue> =
  ResourceDetailJsonPathDefinition;
export type ResourceDetailJsonPathDeclarationMap<TValue> =
  ResourceDetailJsonPathDefinitionMap<TValue>;

export interface ResourceDetailJsonPath<TValue, TPathValue = SignalValue> {
  read(value: TValue): TPathValue;
  write(value: TValue, pathValue: TPathValue): TValue;
  readonly jsonPathProof: ResourceEffectJsonPathPatchProof;
}

export type ResourceDetailJsonPathMap<TValue> = Readonly<
  Record<string, ResourceDetailJsonPath<TValue, any>>
>;

export type ResourceDetailJsonPathDefinitions<
  TValue,
  TDefinitionMap extends Readonly<Record<string, unknown>>,
> = {
  readonly [TPath in keyof TDefinitionMap & string]: ResourceDetailJsonPath<
    TValue,
    SignalValue
  >;
};

export type ResourceDetailJsonPathValue<TPath> =
  TPath extends ResourceDetailJsonPath<any, infer TValue> ? TValue : never;

export interface ResourceDetailJsonPaths<
  TValue,
  TPathMap extends ResourceDetailJsonPathMap<TValue> = ResourceDetailJsonPathMap<TValue>,
> {
  readonly definitions: TPathMap;
  readonly [WORTHSignalResourceDetailJsonPathsBrand]: "resourceDetailJsonPaths";
}
