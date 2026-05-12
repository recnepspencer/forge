import type { SignalValue } from "../model.js";
import type { FormValidationArtifact } from "./validation.js";

export interface CallableFormSignal<TValue = SignalValue> {
  (): TValue;
  get(): TValue;
  value(): TValue;
}

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
}

export interface FormFieldOptions<TValue = SignalValue, TRaw = TValue> {
  id?: string;
  adapter?: FormInputAdapterOptions;
  inputAdapter?: FormInputAdapterOptions;
  parse?: (rawValue: TRaw) => TValue;
}

export interface FormFieldDeclaration<TValue = SignalValue, TRaw = TValue> {
  readonly path: string;
  readonly __formFieldValue?: TValue;
  readonly __formFieldRaw?: TRaw;
}

export interface FormFieldFactory {
  field<TValue = SignalValue, TRaw = TValue>(
    path: FormFieldPath,
    options?: FormFieldOptions<TValue, TRaw>,
  ): FormFieldDeclaration<TValue, TRaw>;
}

export type FormFieldsBuilder<TFields extends Record<string, FormFieldDeclaration>> =
  (factory: FormFieldFactory) => TFields;

export type FormSource<TValue = SignalValue> =
  | TValue
  | (() => TValue)
  | CallableFormSignal<TValue>;

export interface FormFieldLocus {
  readonly field: string;
  readonly path: string;
  readonly segments: ReadonlyArray<string>;
}

export interface FormFieldDirtyState {
  readonly field: string;
  readonly path: string;
  readonly isDirty: boolean;
  readonly semanticDirty: boolean;
  readonly equality: FormSemanticEqualityCounters;
}

export interface FormSemanticEqualityCounters {
  readonly costBasis: "fieldLocusStructuralCompare" | "derivedFieldLocusStructuralCompare";
  readonly incrementalStatus?: "notIncremental";
  readonly fieldComparisons?: number;
  readonly deepCollectionFields?: number;
  readonly valueComparisons: number;
  readonly objectKeyReads: number;
  readonly arrayEntries: number;
  readonly maxDepth: number;
}

export interface FormInputAdapterDiagnostics {
  readonly tier: FormInputAdapterTier;
  readonly capabilities: Readonly<Record<string, boolean>>;
  readonly unavailable: ReadonlyArray<{
    readonly capability: string;
    readonly reason: string;
  }>;
}

export interface FormFieldDiagnostics {
  readonly locus: FormFieldLocus;
  readonly dirty: FormFieldDirtyState;
  readonly pendingRawInput: boolean;
  readonly parseFailure: FormValidationArtifact | null;
  readonly writePosture: FormFieldWritePosture;
  readonly inputAdapter: FormInputAdapterDiagnostics;
}

export interface FormFieldWritePosture {
  readonly field: string;
  readonly capability: "edit" | "patch";
  readonly canWrite: boolean;
  readonly blockers: ReadonlyArray<FormReadinessBlocker>;
  readonly reason: string;
}

export interface FormFieldHandle<TValue = SignalValue, TRaw = TValue> {
  readonly id: string;
  readonly path: string;
  locus(): FormFieldLocus;
  sourceValue(): TValue;
  draftValue(): TValue | undefined;
  effectiveValue(): TValue;
  value(): TValue;
  set(value: TValue): FormFieldHandle<TValue, TRaw>;
  clearDraft(): FormFieldHandle<TValue, TRaw>;
  input(rawValue: TRaw, options?: { commit?: boolean }): FormFieldHandle<TValue, TRaw>;
  commitInput(parser?: (rawValue: TRaw) => TValue): FormFieldHandle<TValue, TRaw>;
  dirty(): FormFieldDirtyState;
  diagnostics(): FormFieldDiagnostics;
}

export interface FormDirtyState {
  readonly isDirty: boolean;
  readonly semanticDirty: boolean;
  readonly fields: ReadonlyArray<{
    readonly field: string;
    readonly path: string;
    readonly sourceDigest: string;
    readonly effectiveDigest: string;
    readonly equality: FormSemanticEqualityCounters;
  }>;
  readonly equality: FormSemanticEqualityCounters;
  readonly breadth: {
    readonly declaredFields: number;
    readonly comparedFields: number;
    readonly changedFields: number;
    readonly omittedFields: number;
    readonly clearedFields: number;
    readonly sourceSnapshots: number;
    readonly effectiveSnapshots: number;
  };
}

export interface FormPatchOperation<TValue = SignalValue> {
  readonly kind: "set";
  readonly field: string;
  readonly locus: FormFieldLocus;
  readonly value: TValue;
  readonly valueDigest: string;
  readonly equality: FormSemanticEqualityCounters;
}

export interface FormPatchPlan {
  readonly semanticDirty: boolean;
  readonly empty: boolean;
  readonly operations: ReadonlyArray<FormPatchOperation>;
  readonly blocked: ReadonlyArray<FormReadinessBlocker>;
  readonly broadReplacement: false;
  readonly equality: FormSemanticEqualityCounters;
  readonly breadth: {
    readonly declaredFields: number;
    readonly comparedFields: number;
    readonly changedFields: number;
    readonly skippedRawInputFields: number;
    readonly omittedFields: number;
    readonly clearedFields: number;
    readonly sourceSnapshots: number;
    readonly effectiveSnapshots: number;
  };
  readonly equivalenceDigest: string;
}

export interface FormReadinessBlocker {
  readonly kind:
    | "unchanged"
    | "uncommittedRawInput"
    | "validation:invalid"
    | "validation:pending"
    | "validation:blocked"
    | "validation:unavailable"
    | "validation:parseFailure"
    | "availability:blocked"
    | "availability:unavailable"
    | "step:blocked"
    | "step:unavailable"
    | "idempotency:duplicate"
    | "admission:denied"
    | "admission:blocked"
    | "admission:unavailable"
    | "admission:requiresApproval"
    | "admission:requiresSignature"
    | "admission:requiresReview"
    | "admission:requiresReason";
  readonly field?: string;
  readonly action?: string;
  readonly control?: string;
  readonly group?: string;
  readonly section?: string;
  readonly fields?: ReadonlyArray<string>;
  readonly capability?: string;
  readonly reason: string;
}
