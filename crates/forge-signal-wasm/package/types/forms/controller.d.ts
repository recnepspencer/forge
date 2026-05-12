import type { SignalValue } from "../model.js";
import type {
  FormDirtyState,
  FormFieldDeclaration,
  FormFieldHandle,
  FormFieldWritePosture,
  FormFieldsBuilder,
  FormPatchPlan,
  FormReadinessBlocker,
  FormSource,
} from "./core.js";
import type {
  FormActionsBuilder,
  FormActionExecutionArtifact,
  FormActionPlan,
  FormActionResultArtifact,
  FormActionsReport,
} from "./actions.js";
import type { FormCanonicalizationArtifact } from "./canonicalization.js";
import type {
  FormAdmissionBuilder,
  FormAdmissionReport,
} from "./admission.js";
import type {
  FormAvailabilityBuilder,
  FormAvailabilityReport,
} from "./availability.js";
import type {
  FormMessageArtifact,
  FormAsyncValidationLifecycleArtifact,
  FormValidationArtifact,
  FormValidationBuilder,
  FormValidationReport,
} from "./validation.js";
import type {
  FormStepsBuilder,
  FormStepsReport,
} from "./steps.js";
import type { FormVerificationPackage } from "./verification.js";

export interface FormDeclaration<
  TSource = SignalValue,
  TFields extends Record<string, FormFieldDeclaration> = Record<string, FormFieldDeclaration>,
> {
  source: FormSource<TSource>;
  fields: TFields | FormFieldsBuilder<TFields>;
  validation?: Record<string, unknown> | FormValidationBuilder;
  availability?: Record<string, unknown> | FormAvailabilityBuilder;
  admission?: Record<string, unknown> | FormAdmissionBuilder;
  steps?: Record<string, unknown> | FormStepsBuilder;
  actions?: Record<string, unknown> | FormActionsBuilder;
}

export interface FormController<
  TSource = SignalValue,
  TFieldHandles extends Record<string, FormFieldHandle> = Record<string, FormFieldHandle>,
> {
  readonly fields: TFieldHandles;
  source(): TSource;
  draft(): Partial<TSource>;
  effective(): TSource;
  dirty(): FormDirtyState;
  patchPlan(): FormPatchPlan;
  validation(): FormValidationReport;
  availability(): FormAvailabilityReport;
  admission(): FormAdmissionReport;
  steps(): FormStepsReport;
  actions(): FormActionsReport;
  actionPlan(actionId: string): FormActionPlan;
  attemptAction(actionId: string): FormActionResultArtifact;
  actionHistory(): ReadonlyArray<FormActionResultArtifact>;
  executeAction(actionId: string): FormActionExecutionArtifact;
  fulfillAction(operationId: number, payload?: {
    readonly reason?: string;
    readonly messages?: ReadonlyArray<{
      readonly code: string;
      readonly target?: string;
      readonly scope?: string;
      readonly severity?: string;
    }>;
    readonly canonicalValue?: SignalValue;
  }): FormActionExecutionArtifact;
  rejectAction(operationId: number, payload?: {
    readonly reason?: string;
    readonly messages?: ReadonlyArray<{
      readonly code: string;
      readonly target?: string;
      readonly scope?: string;
      readonly severity?: string;
    }>;
  }): FormActionExecutionArtifact;
  cancelAction(operationId: number, payload?: { readonly reason?: string }): FormActionExecutionArtifact;
  timeoutAction(operationId: number, payload?: { readonly reason?: string }): FormActionExecutionArtifact;
  retryAction(operationId: number): FormActionExecutionArtifact;
  actionExecutionHistory(): ReadonlyArray<FormActionExecutionArtifact>;
  startAsyncValidation(validationId: string): FormAsyncValidationLifecycleArtifact;
  fulfillAsyncValidation(operationId: number, payload?: {
    readonly reason?: string;
    readonly artifact?: FormValidationArtifact;
  }): FormAsyncValidationLifecycleArtifact;
  rejectAsyncValidation(operationId: number, payload?: {
    readonly reason?: string;
    readonly code?: string;
    readonly artifact?: FormValidationArtifact;
  }): FormAsyncValidationLifecycleArtifact;
  cancelAsyncValidation(operationId: number, payload?: { readonly reason?: string }): FormAsyncValidationLifecycleArtifact;
  timeoutAsyncValidation(operationId: number, payload?: { readonly reason?: string }): FormAsyncValidationLifecycleArtifact;
  asyncValidationHistory(): ReadonlyArray<FormAsyncValidationLifecycleArtifact>;
  canonicalizationHistory(): ReadonlyArray<FormCanonicalizationArtifact>;
  verification(): FormVerificationPackage;
  visibleMessages(): ReadonlyArray<FormMessageArtifact>;
  actionReadiness(actionId: string): {
    readonly action: string;
    readonly canRun: boolean;
    readonly blockers: ReadonlyArray<FormReadinessBlocker>;
  };
  fieldWritePosture(fieldId: string, capability?: "edit" | "patch"): FormFieldWritePosture;
  readiness(): {
    readonly canSubmit: boolean;
    readonly blockers: ReadonlyArray<FormReadinessBlocker>;
    readonly patchPlan: FormPatchPlan;
  };
  diagnostics(): {
    readonly kind: "form";
    readonly fieldCount: number;
    readonly dirty: FormDirtyState;
    readonly patchPlan: FormPatchPlan;
    readonly validation: FormValidationReport;
    readonly availability: FormAvailabilityReport;
    readonly admission: FormAdmissionReport;
    readonly steps: FormStepsReport;
    readonly actions: FormActionsReport;
    readonly actionHistory: ReadonlyArray<FormActionResultArtifact>;
    readonly actionExecutionHistory: ReadonlyArray<FormActionExecutionArtifact>;
    readonly asyncValidationHistory: ReadonlyArray<FormAsyncValidationLifecycleArtifact>;
    readonly canonicalizationHistory: ReadonlyArray<FormCanonicalizationArtifact>;
    readonly verification: FormVerificationPackage;
    readonly inputAdapters: ReadonlyArray<unknown>;
  };
  field(fieldId: string): FormFieldHandle;
}

export type FormFieldHandleFor<TDeclaration> =
  TDeclaration extends FormFieldDeclaration<infer TValue, infer TRaw>
    ? FormFieldHandle<TValue, TRaw>
    : FormFieldHandle;
