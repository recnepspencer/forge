import type { SignalValue } from "../model.js";

export type FormFieldPath = string | ReadonlyArray<string | number>;

export type FormInputAdapterTier =
  | "signalNative"
  | "signalBridge"
  | "externalImperative";

export interface FormInputAdapterOptions {
  tier?: FormInputAdapterTier;
  reportsRawInput?: boolean;
  reportsCommitBoundary?: boolean;
  reportsComposition?: boolean;
  reportsFocus?: boolean;
  supportsLabelTrack?: boolean;
  supportsHelpTrack?: boolean;
  supportsMessageTrack?: boolean;
  supportsMinHeightSync?: boolean;
  supportsResponsiveTokens?: boolean;
}

export interface FormInputAdapterCapabilitySet {
  readonly reportsRawInput: boolean;
  readonly reportsCommitBoundary: boolean;
  readonly reportsComposition: boolean;
  readonly reportsFocus: boolean;
  readonly supportsLabelTrack: boolean;
  readonly supportsHelpTrack: boolean;
  readonly supportsMessageTrack: boolean;
  readonly supportsMinHeightSync: boolean;
  readonly supportsResponsiveTokens: boolean;
}

export interface FormFieldAccessibilityOptions {
  readonly label?: string;
  readonly description?: string;
  readonly summaryLabel?: string;
  readonly describedBy?: ReadonlyArray<string>;
  readonly readingOrder?: number;
  readonly focusOrder?: number;
  readonly summaryOrder?: number;
}

export interface FormFieldLayoutOptions {
  readonly row?: string;
  readonly column?: string;
  readonly density?: "compact" | "comfortable" | "spacious";
  readonly alignment?: "start" | "center" | "stretch";
  readonly minHeight?: number;
  readonly grow?: boolean;
  readonly wrap?: boolean;
  readonly responsive?: ReadonlyArray<string>;
}

export interface FormFieldOptions<TValue = SignalValue, TRaw = TValue> {
  id?: string;
  adapter?: FormInputAdapterOptions;
  inputAdapter?: FormInputAdapterOptions;
  parse?: (rawValue: TRaw) => TValue;
  label?: string;
  description?: string;
  summaryLabel?: string;
  describedBy?: ReadonlyArray<string>;
  readingOrder?: number;
  focusOrder?: number;
  summaryOrder?: number;
  accessibility?: FormFieldAccessibilityOptions;
  row?: string;
  column?: string;
  density?: "compact" | "comfortable" | "spacious";
  alignment?: "start" | "center" | "stretch";
  minHeight?: number;
  grow?: boolean;
  wrap?: boolean;
  responsive?: ReadonlyArray<string>;
  layout?: FormFieldLayoutOptions;
  resourceLocus?: FormValueResourceLocus;
}

export interface FormValueResourceLocusField {
  readonly kind: "field";
  readonly field: string;
}

export interface FormValueResourceLocusJsonPath {
  readonly kind: "jsonPath";
  readonly path: string;
}

export interface FormValueResourceLocusRegion {
  readonly kind: "region";
  readonly region: string;
}

export interface FormValueResourceLocusItemAspect {
  readonly kind: "itemAspect";
  readonly itemId: string;
  readonly aspect: string;
}

export interface FormValueResourceLocusSummary {
  readonly kind: "summary";
  readonly summary: string;
}

export type FormValueResourceLocus =
  | FormValueResourceLocusField
  | FormValueResourceLocusJsonPath
  | FormValueResourceLocusRegion
  | FormValueResourceLocusItemAspect
  | FormValueResourceLocusSummary;

export interface FormRepeatedResourceLocus {
  readonly kind: "collectionItems";
  readonly placement?: "append" | "prepend";
}

export interface FormRepeatedFieldOptions<TValue = SignalValue, TRaw = TValue>
  extends FormFieldOptions<TValue, TRaw> {
  readonly resourceLocus?: FormRepeatedResourceLocus;
}

export type FormFieldFamily = "scalar" | "repeated" | "attachment" | "evidence";

export interface FormFieldDeclaration<
  TValue = SignalValue,
  TRaw = TValue,
  TFamily extends FormFieldFamily = FormFieldFamily,
> {
  readonly path: string;
  readonly family?: TFamily;
  readonly __formFieldValue?: TValue;
  readonly __formFieldRaw?: TRaw;
  readonly __formFieldFamily?: TFamily;
}

export interface FormFieldFactory {
  field<TValue = SignalValue, TRaw = TValue>(
    path: FormFieldPath,
    options?: FormFieldOptions<TValue, TRaw>,
  ): FormFieldDeclaration<TValue, TRaw, "scalar">;
  repeated<TValue = SignalValue, TRaw = TValue>(
    path: FormFieldPath,
    options: FormRepeatedFieldOptions<TValue, TRaw> & FormRepeatedIdentityOptions<TValue>,
  ): FormFieldDeclaration<TValue, TRaw, "repeated">;
  attachment<TValue = SignalValue, TRaw = TValue>(
    path: FormFieldPath,
    options: FormFieldOptions<TValue, TRaw> & FormAttachmentIdentityOptions<TValue>,
  ): FormFieldDeclaration<TValue, TRaw, "attachment">;
  evidence<TValue = SignalValue, TRaw = TValue>(
    path: FormFieldPath,
    options: FormFieldOptions<TValue, TRaw> & FormAttachmentIdentityOptions<TValue>,
  ): FormFieldDeclaration<TValue, TRaw, "evidence">;
}

export type FormRepeatedIdentityOptions<TValue = SignalValue> =
  | { readonly itemIdentity: string | ((item: FormRepeatedItem<TValue>) => string); readonly key?: never }
  | { readonly key: string | ((item: FormRepeatedItem<TValue>) => string); readonly itemIdentity?: never };

export type FormRepeatedItem<TValue> =
  TValue extends ReadonlyArray<infer TItem> ? TItem : SignalValue;

export type FormAttachmentIdentityOptions<TValue = SignalValue> =
  | {
      readonly attachmentIdentity: string | ((attachment: TValue) => string);
      readonly digest?: never;
      readonly metadata?: Readonly<Record<string, SignalValue>>;
    }
  | {
      readonly digest: string | ((attachment: TValue) => string);
      readonly attachmentIdentity?: never;
      readonly metadata?: Readonly<Record<string, SignalValue>>;
    };

export type FormFieldsBuilder<
  TFields extends Record<string, FormFieldDeclaration<SignalValue, SignalValue, FormFieldFamily>>,
> = (factory: FormFieldFactory) => TFields;
