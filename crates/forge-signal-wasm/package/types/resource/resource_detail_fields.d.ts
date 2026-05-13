import type { SignalValue } from "../model.js";
import type { ResourceEffectFieldPatchProof } from "./resource_effect_field_patch_proof.js";

declare const forgeSignalResourceDetailFieldsBrand: unique symbol;

export interface ResourceDetailField<TValue, TFieldValue = SignalValue> {
  read(value: TValue): TFieldValue;
  write(value: TValue, fieldValue: TFieldValue): TValue;
  extract?(value: TValue): {
    readonly present: boolean;
    readonly value: TFieldValue | undefined;
  };
  readonly fieldProof?: ResourceEffectFieldPatchProof;
}

export type ResourceDetailFieldMap<TValue> = Readonly<
  Record<string, ResourceDetailField<TValue, any>>
>;

export type ResourceDetailFieldValue<TField> =
  TField extends ResourceDetailField<any, infer TValue> ? TValue : never;

export interface ResourceDetailFields<
  TValue,
  TFieldMap extends ResourceDetailFieldMap<TValue> = ResourceDetailFieldMap<TValue>,
> {
  readonly definitions: TFieldMap;
  readonly [forgeSignalResourceDetailFieldsBrand]: "resourceDetailFields";
}

export type ResourceDetailObjectFieldMap<TValue> = Readonly<
  Record<string, keyof TValue & string>
>;

export type ResourceDetailObjectFieldDefinitions<
  TValue,
  TFields extends ResourceDetailObjectFieldMap<TValue>,
> = {
  readonly [TField in keyof TFields & string]: ResourceDetailField<
    TValue,
    TValue[TFields[TField]]
  >;
};
