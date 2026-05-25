import type { SignalValue } from "../model.js";
import type { FormAdmissionCapability } from "./admission.js";
import type { FormHostRequiredCapability } from "./host.js";
import type { ResourceEffectProfile } from "../resource/resource_effect_profiles.js";

export type FormActionKind = "submit" | "custom" | "step";
export type FormActionPatchPolicy = "requiresNonEmpty" | "allowEmpty" | "ignore";
export type FormActionIdempotency = "none" | "collapse" | "supersede" | "queue" | "deny";
export type FormActionEffectPolicy = "deferred" | "none" | "controllerLocal";
export type FormStepActionCommand = "next" | "back" | "jump" | "skip" | "revisit" | "custom";
export type FormActionResultKind =
  | "accepted"
  | "denied"
  | "unavailable"
  | "cancelled"
  | "superseded"
  | "rejected"
  | "fulfilled"
  | "noOp";

export interface FormActionDeclarationOptions {
  readonly label?: string;
  readonly kind?: FormActionKind;
  readonly patchPolicy?: FormActionPatchPolicy;
  readonly admissionCapability?: FormAdmissionCapability;
  readonly destructive?: boolean;
  readonly idempotency?: FormActionIdempotency;
  readonly effectPolicy?: FormActionEffectPolicy;
  readonly hostEffect?: string;
  readonly hostRequirements?: ReadonlyArray<FormHostRequiredCapability>;
  readonly resourceAction?: never;
  readonly resourceEffectProfile?: ResourceEffectProfile;
  readonly schema?: SignalValue;
}

export interface FormResourcePatchActionDeclaration {
  readonly kind: "patchPlan";
  readonly fields?: ReadonlyArray<string>;
}

export interface FormResourceRefreshActionDeclaration { readonly kind: "refresh"; }
export interface FormResourceRevalidateActionDeclaration { readonly kind: "revalidate"; }
export interface FormResourceReplayExactActionDeclaration { readonly kind: "replayExact"; }
export interface FormResourceRestoreExactActionDeclaration { readonly kind: "restoreExact"; }
export interface FormResourceRollbackLastEffectActionDeclaration { readonly kind: "rollbackLastEffect"; }

export type FormResourceActionDeclaration =
  | FormResourcePatchActionDeclaration
  | FormResourceRefreshActionDeclaration
  | FormResourceRevalidateActionDeclaration
  | FormResourceReplayExactActionDeclaration
  | FormResourceRestoreExactActionDeclaration
  | FormResourceRollbackLastEffectActionDeclaration;

export interface FormResourceBackedPatchActionDeclarationOptions extends Omit<
  FormActionDeclarationOptions,
  "patchPolicy" | "effectPolicy" | "hostEffect" | "resourceAction"
> {
  readonly patchPolicy?: "requiresNonEmpty";
  readonly effectPolicy?: "deferred";
  readonly hostEffect?: never;
  readonly resourceAction: FormResourcePatchActionDeclaration;
}

export interface FormResourceBackedLifecycleActionDeclarationOptions extends Omit<
  FormActionDeclarationOptions,
  "patchPolicy" | "effectPolicy" | "hostEffect" | "resourceEffectProfile" | "resourceAction"
> {
  readonly patchPolicy?: "ignore";
  readonly effectPolicy?: "deferred";
  readonly hostEffect?: never;
  readonly resourceEffectProfile?: never;
  readonly resourceAction: FormResourceRefreshActionDeclaration | FormResourceRevalidateActionDeclaration;
}

export interface FormResourceBackedRecoveryActionDeclarationOptions extends Omit<
  FormActionDeclarationOptions,
  "patchPolicy" | "effectPolicy" | "hostEffect" | "resourceEffectProfile" | "resourceAction"
> {
  readonly patchPolicy?: "ignore";
  readonly effectPolicy?: "deferred";
  readonly hostEffect?: never;
  readonly resourceEffectProfile?: never;
  readonly resourceAction:
    | FormResourceReplayExactActionDeclaration
    | FormResourceRestoreExactActionDeclaration
    | FormResourceRollbackLastEffectActionDeclaration;
}

export interface FormStepActionDeclarationOptions extends FormActionDeclarationOptions {
  readonly kind?: "step";
  readonly routeCoupled?: boolean;
}

export interface FormActionDeclaration {
  readonly id: string;
  readonly kind: FormActionKind;
}

export interface FormActionsFactory {
  submit(options?: FormActionDeclarationOptions): FormActionDeclaration;
  action(
    actionId: string,
    options?:
      | FormActionDeclarationOptions
      | FormResourceBackedPatchActionDeclarationOptions
      | FormResourceBackedLifecycleActionDeclarationOptions
      | FormResourceBackedRecoveryActionDeclarationOptions,
  ): FormActionDeclaration;
  step(
    actionId: string,
    stepId: string,
    command: FormStepActionCommand,
    options?: FormStepActionDeclarationOptions,
  ): FormActionDeclaration;
}

export type FormActionsBuilder =
  (factory: FormActionsFactory) => Record<string, unknown>;
