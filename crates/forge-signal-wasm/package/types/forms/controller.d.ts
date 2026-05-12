import type { SignalValue } from "../model.js";
import type {
  FormDirtyState,
  FormFieldDeclaration,
  FormFieldHandle,
  FormInputAdapterDiagnostics,
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
import type {
  FormSourceAuthorityDiagnostics,
  FormSourceDeclaration,
  FormSourceFactory,
} from "./sources.js";
import type { FormVerificationPackage } from "./verification.js";

export interface FormDeclaration<
  TSource = SignalValue,
  TFields extends Record<string, FormFieldDeclaration> = Record<string, FormFieldDeclaration>,
> {
  id?: string;
  contract?: string;
  source: FormSource<TSource> | FormSourceDeclaration<TSource>;
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
  declaration(): {
    readonly formId: string;
    readonly contract: string;
    readonly source: {
      readonly kind: string;
      readonly sourceId: string;
    };
    readonly fieldFamilies: {
      readonly scalar: number;
      readonly repeated: number;
      readonly attachment: number;
    };
    readonly fieldCount: number;
  };
  source(): TSource;
  sourceAuthority(): FormSourceAuthorityDiagnostics;
  fieldContract(): ReadonlyArray<{
    readonly id: string;
    readonly name: string;
    readonly family: "scalar" | "repeated" | "attachment";
    readonly path: string;
    readonly collectionIdentity: null | {
      readonly kind: "field" | "resolver";
      readonly field: string | null;
      readonly posture: string;
    };
    readonly attachment: null | {
      readonly identityKind: "field" | "resolver";
      readonly identityField: string | null;
      readonly metadata: Readonly<Record<string, SignalValue>>;
      readonly posture: string;
    };
  }>;
  inputAdapters(): ReadonlyArray<{
    readonly field: string;
    readonly path: string;
    readonly family: "scalar" | "repeated" | "attachment";
    readonly tier: FormInputAdapterDiagnostics["tier"];
    readonly capabilities: FormInputAdapterDiagnostics["capabilities"];
    readonly unavailable: FormInputAdapterDiagnostics["unavailable"];
  }>;
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
    readonly declaration: ReturnType<FormController["declaration"]>;
    readonly fieldCount: number;
    readonly sourceAuthority: FormSourceAuthorityDiagnostics;
    readonly fieldContract: ReturnType<FormController["fieldContract"]>;
    readonly inputAdapters: ReturnType<FormController["inputAdapters"]>;
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
  };
  field(fieldId: string): FormFieldHandle;
}

export type FormFieldHandleFor<TDeclaration> =
  TDeclaration extends FormFieldDeclaration<infer TValue, infer TRaw, infer TFamily>
    ? FormFieldHandle<TValue, TRaw, TFamily>
    : FormFieldHandle;

export interface FormFactory {
  <
    TSource = SignalValue,
    TFields extends Record<string, FormFieldDeclaration> = Record<string, FormFieldDeclaration>,
  >(declaration: FormDeclaration<TSource, TFields>): FormController<
    TSource,
    { [K in keyof TFields]: FormFieldHandleFor<TFields[K]> }
  >;
  readonly source: FormSourceFactory;
}
