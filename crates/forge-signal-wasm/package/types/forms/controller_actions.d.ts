import type { SignalValue } from "../model.js";
import type {
  FormActionExecutionArtifact,
  FormActionPlan,
  FormActionResultArtifact,
} from "./actions.js";
import type { FormAsyncValidationLifecycleArtifact, FormValidationArtifact } from "./validation.js";
import type { FormCanonicalizationArtifact } from "./canonicalization.js";
import type { FormReplayRestoreArtifact } from "./replay_restore.js";
import type { FormResetArtifact } from "./reset.js";

export interface FormControllerActionBindings {
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
  reset(options?: { readonly reason?: string }): FormResetArtifact;
  rollbackLastResourceEffect(options?: { readonly reason?: string }): FormResetArtifact;
  resetHistory(): ReadonlyArray<FormResetArtifact>;
  replayExactResourceSource(options?: { readonly reason?: string }): FormReplayRestoreArtifact;
  restoreExactResourceSource(options?: { readonly reason?: string }): FormReplayRestoreArtifact;
  replayRestoreHistory(): ReadonlyArray<FormReplayRestoreArtifact>;
}
