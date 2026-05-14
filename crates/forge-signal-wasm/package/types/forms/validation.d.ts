import type { SignalValue } from "../model.js";
import type {
  FormFieldDirtyState,
  FormFieldLocus,
} from "./core.js";
import type { FormHostReport } from "./host.js";

export interface FormMessageArtifact {
  readonly code: string;
  readonly message?: string;
  readonly severity: "info" | "warning" | "error";
  readonly target?: string;
  readonly audience: "user" | "developer" | "system";
  readonly visibility: "hidden" | "visible" | "summary" | "blocked";
  readonly accessibility?: {
    readonly describedBy?: ReadonlyArray<string>;
    readonly announce?: "off" | "polite" | "assertive";
    readonly focusTarget?: string;
  };
  readonly recovery?: ReadonlyArray<unknown>;
}

export type FormValidationArtifact =
  | { readonly kind: "valid"; readonly field?: string; readonly digest: string }
  | { readonly kind: "warning"; readonly field?: string; readonly message: FormMessageArtifact }
  | { readonly kind: "invalid"; readonly field?: string; readonly message: FormMessageArtifact }
  | {
      readonly kind: "pending";
      readonly field?: string;
      readonly asyncValidationId: string;
      readonly operationId?: number;
      readonly basisDigest?: string;
    }
  | { readonly kind: "blocked"; readonly field?: string; readonly reason: string; readonly blockers: ReadonlyArray<string> }
  | { readonly kind: "unavailable"; readonly field?: string; readonly reason: string; readonly detail: string }
  | { readonly kind: "parseFailure"; readonly field?: string; readonly message: FormMessageArtifact; readonly rawDigest: string };

export type FormAsyncValidationTrigger =
  | "input"
  | "blur"
  | "idle"
  | "debounce"
  | "explicit"
  | "action"
  | "submit";

export interface FormAsyncValidationTriggerPolicy {
  readonly triggers: ReadonlyArray<FormAsyncValidationTrigger | string>;
  readonly debounceMs: number | null;
}

export type FormAsyncValidationResultKind =
  | "pending"
  | "fulfilled"
  | "rejected"
  | "cancelled"
  | "timedOut"
  | "superseded"
  | "staleCompletion";

export interface FormAsyncValidationLifecycleArtifact {
  readonly kind: "asyncValidation";
  readonly operationId: number;
  readonly targetOperationId?: number;
  readonly supersededByOperationId?: number;
  readonly validationId: string | null;
  readonly targetValidationId?: string | null;
  readonly field: string | null;
  readonly dependencies: ReadonlyArray<string>;
  readonly triggerPolicy: FormAsyncValidationTriggerPolicy | null;
  readonly basisDigest: string | null;
  readonly targetBasisDigest?: string | null;
  readonly resultKind: FormAsyncValidationResultKind;
  readonly stale: boolean;
  readonly reason: string;
  readonly lifecycleDigest: string;
}

export interface FormValidationFieldReadView<TValue = SignalValue> {
  readonly id: string;
  readonly path: string;
  locus(): FormFieldLocus;
  sourceValue(): TValue;
  draftValue(): TValue | undefined;
  effectiveValue(): TValue;
  value(): TValue;
  dirty(): FormFieldDirtyState;
}

export interface FormValidationReadView<TSource = SignalValue> {
  source(): TSource;
  draft(): Partial<TSource>;
  effective(): TSource;
  host(): FormHostReport;
  field<TValue = SignalValue>(fieldId: string): FormValidationFieldReadView<TValue>;
}

export interface FormValidationContext<TValue = SignalValue> {
  readonly form: FormValidationReadView;
  readonly field?: FormValidationFieldReadView<TValue>;
  readonly sourceValue?: TValue;
  readonly dependencies?: ReadonlyArray<string>;
}

export interface FormValidationFactory {
  field<TValue = SignalValue>(
    fieldId: string,
    validator: (
      value: TValue,
      context: FormValidationContext<TValue>,
    ) => FormValidationArtifact | ReadonlyArray<FormValidationArtifact> | true | null | undefined,
    options?: { id?: string },
  ): unknown;
  asyncField(
    fieldId: string,
    options?: {
      readonly id?: string;
      readonly triggers?: ReadonlyArray<FormAsyncValidationTrigger | string>;
      readonly debounceMs?: number;
    },
  ): unknown;
  form(
    id: string,
    dependencies: ReadonlyArray<string>,
    validator: (
      values: Record<string, SignalValue>,
      context: FormValidationContext,
    ) => FormValidationArtifact | ReadonlyArray<FormValidationArtifact> | true | null | undefined,
  ): unknown;
}

export type FormValidationBuilder =
  (factory: FormValidationFactory) => Record<string, unknown>;

export interface FormValidationReport {
  readonly artifacts: ReadonlyArray<FormValidationArtifact>;
  readonly host: FormHostReport;
  readonly summary: {
    readonly valid: number;
    readonly warning: number;
    readonly invalid: number;
    readonly pending: number;
    readonly blocked: number;
    readonly unavailable: number;
    readonly parseFailure: number;
  };
  readonly counters: {
    readonly fieldLocal: number;
    readonly dependencyRegion: number;
    readonly wholeForm: number;
  };
  readonly dependencyBreadth: ReadonlyArray<{
    readonly id: string;
    readonly kind?: "sync" | "async";
    readonly breadth: "field" | "dependencyRegion" | "wholeForm";
    readonly dependencies: ReadonlyArray<string>;
    readonly triggerPolicy?: FormAsyncValidationTriggerPolicy;
  }>;
}
