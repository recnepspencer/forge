import type {
  FormController,
  FormDeclaration,
  FormFieldHandleFor,
  FormFactory,
} from "../package/types/forms/controller.js";
import type {
  FormFieldDeclaration,
  FormFieldDiagnostics,
  FormFieldDirtyState,
  FormFieldHandle,
  FormFieldWritePosture,
} from "../package/types/forms/core.js";
import type { FormFieldInteractionState } from "../package/types/forms/interaction.js";
import type { FormMessageArtifact } from "../package/types/forms/validation.js";
import type { SignalValue } from "../package/types/model.js";
import type { SignalsLike } from "./model.js";

export type RuntimeFormController<
  TSource = unknown,
  TFieldHandles extends Record<string, unknown> = Record<string, unknown>,
> = Omit<FormController<TSource, Record<string, FormFieldHandle>>, "fields"> & {
  readonly fields: TFieldHandles;
};

export type RuntimeFormDeclaration<
  TSource = unknown,
  TFields extends Record<string, FormFieldDeclaration> = Record<string, FormFieldDeclaration>,
> = FormDeclaration<TSource, TFields>;

export type RuntimeFormFieldHandleFor<TDeclaration> = FormFieldHandleFor<TDeclaration>;

export interface SignalsWithFormLike extends SignalsLike {
  readonly form: FormFactory;
}

export interface SignalsFormOption<TValue = unknown> {
  readonly label: string;
  readonly value: TValue;
  readonly disabled?: boolean;
}

export interface SignalsFormFieldState<TValue = unknown, TRaw = TValue> {
  readonly name: string;
  readonly value: TValue;
  readonly disabled: boolean;
  readonly readOnly: boolean;
  readonly field: FormFieldHandleReactLike<TValue, TRaw>;
  readonly dirty: FormFieldDirtyState;
  readonly diagnostics: FormFieldDiagnostics;
  readonly messages: readonly FormMessageArtifact[];
  readonly interaction: FormFieldInteractionState | null;
  readonly writePosture: FormFieldWritePosture;
}

export interface SignalsFormFieldBinding<TValue = unknown, TRaw = TValue>
  extends SignalsFormFieldState<TValue, TRaw> {
  readonly binding: FormBoundInputReactLike<TValue, TRaw>;
  onChange(next: unknown): void;
  onBlur(): void;
  onFocus(): void;
}

export interface SignalsFormCheckboxBinding<TValue = boolean> {
  readonly name: string;
  readonly checked: boolean;
  readonly disabled: boolean;
  readonly readOnly: boolean;
  readonly dirty: FormFieldDirtyState;
  readonly diagnostics: FormFieldDiagnostics;
  readonly messages: readonly FormMessageArtifact[];
  readonly interaction: FormFieldInteractionState | null;
  readonly writePosture: FormFieldWritePosture;
  onChange(next: unknown): void;
  onBlur(): void;
  onFocus(): void;
}

export interface SignalsFormSelectBinding<TValue = unknown, TRaw = TValue>
  extends SignalsFormFieldBinding<TValue, TRaw> {
  readonly options: readonly SignalsFormOption<TValue>[];
}

export interface SignalsFormMultiSelectBinding<TValue = string> {
  readonly name: string;
  readonly value: readonly TValue[];
  readonly disabled: boolean;
  readonly readOnly: boolean;
  readonly dirty: FormFieldDirtyState;
  readonly diagnostics: FormFieldDiagnostics;
  readonly messages: readonly FormMessageArtifact[];
  readonly interaction: FormFieldInteractionState | null;
  readonly writePosture: FormFieldWritePosture;
  readonly options: readonly SignalsFormOption<TValue>[];
  onChange(next: unknown): void;
  onBlur(): void;
  onFocus(): void;
}

export interface SignalsFormActionBinding<
  TForm extends FormControllerReactLike = FormControllerReactLike,
> {
  readonly plan: ReturnType<TForm["actionPlan"]>;
  readonly debug: ReturnType<TForm["debugAction"]>;
  readonly disabled: boolean;
  readonly pending: boolean;
  readonly latestExecution: ReturnType<TForm["debugAction"]>["latestExecution"];
  readonly resultKind: string | null;
  execute(): ReturnType<TForm["executeAction"]>;
}

export interface SignalsFormBinding<
  TFieldIds extends string = string,
  TActionIds extends string = string,
  TForm extends FormControllerReactLike = FormControllerReactLike,
> {
  readonly controller: TForm;
  readonly source: ReturnType<TForm["source"]>;
  readonly draft: ReturnType<TForm["draft"]>;
  readonly effective: ReturnType<TForm["effective"]>;
  readonly dirty: ReturnType<TForm["dirty"]>;
  readonly patchPlan: ReturnType<TForm["patchPlan"]>;
  readonly readiness: ReturnType<TForm["readiness"]>;
  readonly visibleMessages: ReturnType<TForm["visibleMessages"]>;
  readonly actions: Readonly<Record<TActionIds, SignalsFormActionBinding<TForm>>>;
  fieldState<TValue = unknown, TRaw = TValue>(
    fieldId: TFieldIds,
  ): SignalsFormFieldState<TValue, TRaw>;
  field<TValue = unknown, TRaw = TValue>(
    fieldId: TFieldIds,
    options?: { readonly input?: unknown },
  ): SignalsFormFieldBinding<TValue, TRaw>;
  checkbox<TValue = boolean>(
    fieldId: TFieldIds,
    options?: { readonly input?: unknown },
  ): SignalsFormCheckboxBinding<TValue>;
  select<TValue = unknown, TRaw = TValue>(
    fieldId: TFieldIds,
    fieldOptions: readonly SignalsFormOption<TValue>[],
    options?: { readonly input?: unknown },
  ): SignalsFormSelectBinding<TValue, TRaw>;
  multiSelect<TValue = string>(
    fieldId: TFieldIds,
    fieldOptions: readonly SignalsFormOption<TValue>[],
    options?: { readonly input?: unknown },
  ): SignalsFormMultiSelectBinding<TValue>;
  action(
    actionId: TActionIds,
  ): SignalsFormActionBinding<TForm>;
  reset(options?: { readonly reason?: string }): ReturnType<TForm["reset"]>;
}

export interface FormBoundInputReactLike<TValue = unknown, TRaw = TValue> {
  input(rawValue: TRaw, options?: { readonly commit?: boolean; readonly source?: string }): void;
  focus(): void;
  blur(): void;
  touch(): void;
  visit(): void;
  set(value: TValue): void;
  clearDraft(): void;
}

export interface FormFieldHandleReactLike<TValue = unknown, TRaw = TValue> {
  readonly id: string;
  readonly path: string;
  value(): TValue;
  dirty(): FormFieldDirtyState;
  diagnostics(): FormFieldDiagnostics;
}

export interface FormActionPlanReactLike {
  readonly status: string;
  readonly readiness: {
    readonly canRun: boolean;
    readonly blockers: readonly unknown[];
  };
}

export interface FormActionDebugReactLike {
  readonly pending: boolean;
  readonly latestExecution: unknown;
}

export type FormVisibleMessageReactLike = FormMessageArtifact;

export type FormInteractionFieldReactLike = FormFieldInteractionState;

export interface FormControllerReactLike {
  bindInput<TValue = unknown, TRaw = TValue>(
    fieldId: string,
    options?: unknown,
  ): FormBoundInputReactLike<TValue, TRaw>;
  field(fieldId: string): FormFieldHandleReactLike;
  source(): unknown;
  draft(): unknown;
  effective(): unknown;
  patchPlan(): unknown;
  summarySignal(): { readonly id: string; get(): unknown };
  visibleMessages(): readonly FormVisibleMessageReactLike[];
  interaction(): {
    readonly fields: readonly FormInteractionFieldReactLike[];
  };
  fieldWritePosture(fieldId: string, capability?: "edit" | "patch"): FormFieldWritePosture;
  actionPlan(actionId: string): FormActionPlanReactLike;
  debugAction(actionId: string): FormActionDebugReactLike;
  executeAction(actionId: string): unknown;
  dirty(): unknown;
  readiness(): unknown;
  reset(options?: { readonly reason?: string }): unknown;
}
