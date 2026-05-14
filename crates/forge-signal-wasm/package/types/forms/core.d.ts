import type { SignalValue } from "../model.js";
import type { FormValidationArtifact } from "./validation.js";
import type { FormInteractionInputSource } from "./interaction.js";

export interface CallableFormSignal<TValue = SignalValue> {
  (): TValue;
  get(): TValue;
  value(): TValue;
}

export interface FormSourceSchemaContext<TSource = SignalValue> {
  readonly previousSchemaVersion: string | null;
  readonly currentSchemaVersion: string | null;
  readonly source: TSource;
}

export type FormSourceMigrationResult =
  | true
  | null
  | undefined
  | {
    readonly kind: "compatible";
    readonly reason?: string;
  }
  | {
    readonly kind: "migrated";
    readonly draft: SignalValue;
    readonly reason?: string;
  }
  | {
    readonly kind: "unavailable";
    readonly reason?: string;
  };

export interface FormSourceDescriptor<TValue = SignalValue> {
  readonly value: FormSourceValue<TValue>;
  readonly schemaVersion?: string | number | CallableFormSignal<string | number> | (() => string | number);
  readonly migrateDraft?: (
    draft: Partial<TValue>,
    context: FormSourceSchemaContext<TValue>,
  ) => FormSourceMigrationResult;
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

export type FormSourceValue<TValue = SignalValue> =
  | TValue
  | (() => TValue)
  | CallableFormSignal<TValue>;

export type FormSource<TValue = SignalValue> =
  | FormSourceValue<TValue>
  | FormSourceDescriptor<TValue>;

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
  readonly capabilities: FormInputAdapterCapabilitySet;
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
  readonly interaction: {
    readonly field: string;
    readonly path: string;
    readonly touched: boolean;
    readonly visited: boolean;
    readonly focused: boolean;
    readonly focusIntent: boolean;
    readonly blurred: boolean;
    readonly lastInputSource: FormInteractionInputSource | null;
    readonly composing: boolean;
    readonly compositionDigest: string | null;
    readonly focusPosture: "supported" | "unavailable";
    readonly focusReason: string | null;
    readonly interactionDigest: string;
  } | null;
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
  input(rawValue: TRaw, options?: {
    commit?: boolean;
    source?: FormInteractionInputSource;
  }): FormFieldHandle<TValue, TRaw>;
  compose(rawValue: TRaw): FormFieldHandle<TValue, TRaw>;
  commitInput(parser?: (rawValue: TRaw) => TValue): FormFieldHandle<TValue, TRaw>;
  touch(): FormFieldHandle<TValue, TRaw>;
  visit(): FormFieldHandle<TValue, TRaw>;
  focus(): FormFieldHandle<TValue, TRaw>;
  blur(): FormFieldHandle<TValue, TRaw>;
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
    | "schema:drift"
    | "host:offline"
    | "host:unavailable"
    | "step:blocked"
    | "step:unavailable"
    | "step:deferred"
    | "action:deferred"
    | "navigation:notCurrentStep"
    | "navigation:noNextStep"
    | "navigation:noBackStep"
    | "navigation:removedTarget"
    | "navigation:unavailableTarget"
    | "collaboration:locked"
    | "collaboration:leased"
    | "collaboration:readOnly"
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
  readonly collaborator?: string;
  readonly schemaVersion?: string | null;
  readonly previousSchemaVersion?: string | null;
  readonly reason: string;
}
