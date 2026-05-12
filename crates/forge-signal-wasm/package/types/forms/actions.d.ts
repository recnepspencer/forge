import type { SignalValue } from "../model.js";
import type { FormAdmissionCapability, FormAdmissionReport } from "./admission.js";
import type { FormAvailabilityReport } from "./availability.js";
import type { FormPatchOperation, FormReadinessBlocker } from "./core.js";
import type { FormValidationReport } from "./validation.js";

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
  readonly schema?: SignalValue;
}

export interface FormStepActionDeclarationOptions extends FormActionDeclarationOptions {
  readonly kind?: "step";
}

export interface FormActionDeclaration {
  readonly id: string;
  readonly kind: FormActionKind;
}

export interface FormActionsFactory {
  submit(options?: FormActionDeclarationOptions): FormActionDeclaration;
  action(actionId: string, options?: FormActionDeclarationOptions): FormActionDeclaration;
  step(
    actionId: string,
    stepId: string,
    command: FormStepActionCommand,
    options?: FormStepActionDeclarationOptions,
  ): FormActionDeclaration;
}

export type FormActionsBuilder =
  (factory: FormActionsFactory) => Record<string, unknown>;

export interface FormActionCatalogEntry {
  readonly id: string;
  readonly name: string;
  readonly kind: FormActionKind;
  readonly label: string;
  readonly patchPolicy: FormActionPatchPolicy;
  readonly admissionCapability: FormAdmissionCapability;
  readonly destructive: boolean;
  readonly idempotency: FormActionIdempotency;
  readonly effectPolicy: FormActionEffectPolicy;
  readonly hostEffect: string | null;
  readonly schema: SignalValue | null;
  readonly step: {
    readonly stepId: string;
    readonly command: FormStepActionCommand;
    readonly routeCoupled: false;
  } | null;
}

export type FormActionRecoveryKind =
  | "retry"
  | "editField"
  | "resetField"
  | "acceptCanonicalValue"
  | "revealSection"
  | "focusFirstActionableBlocker";

export interface FormActionRecovery {
  readonly kind: FormActionRecoveryKind;
  readonly field?: string;
  readonly action?: string;
  readonly control?: string;
  readonly group?: string;
  readonly section?: string;
  readonly blockerKind: FormReadinessBlocker["kind"];
  readonly reason: string;
}

export interface FormActionPlan extends FormActionCatalogEntry {
  readonly status: "accepted" | "denied";
  readonly resultKind: FormActionResultKind;
  readonly readiness: {
    readonly action: string;
    readonly canRun: boolean;
    readonly blockers: ReadonlyArray<FormReadinessBlocker>;
  };
  readonly recoveryActions: ReadonlyArray<FormActionRecovery>;
  readonly patch: {
    readonly policy: FormActionPatchPolicy;
    readonly empty: boolean;
    readonly semanticDirty?: boolean;
    readonly operations: ReadonlyArray<FormPatchOperation>;
    readonly blocked?: ReadonlyArray<FormReadinessBlocker>;
    readonly equivalenceDigest: string;
  };
  readonly validation: {
    readonly summary: FormValidationReport["summary"];
    readonly artifactCount: number;
  };
  readonly availability: {
    readonly summary: FormAvailabilityReport["summary"];
    readonly artifactCount: number;
  };
  readonly admission: {
    readonly summary: FormAdmissionReport["summary"];
    readonly artifactCount: number;
  };
  readonly proof: {
    readonly sourceDigest: string;
    readonly draftDigest: string;
    readonly effectiveDigest: string;
    readonly patchDigest: string;
    readonly schemaDigest: string;
    readonly actionSchemaDigest: string;
    readonly effectDigest: string;
    readonly bindingDigest: string;
  };
  readonly planDigest: string;
  readonly regulatedActionBindings: ReadonlyArray<{
    readonly admissionId: string;
    readonly capability: FormAdmissionCapability;
    readonly posture: string;
    readonly actorDigest?: string;
    readonly policyDigest?: string;
    readonly admissionBindingDigest: string | null;
    readonly actionPlanDigest: string;
    readonly attestationDigest: string;
  }>;
  readonly diagnostics: {
    readonly deniedBeforeEffects: boolean;
    readonly consumesLoweredPlan: true;
    readonly routeSemantics: "controllerLocalOnly" | "notStepNavigation";
    readonly repeatedAttemptPolicy: FormActionIdempotency;
  };
}

export interface FormActionResultArtifact {
  readonly kind: "actionResult";
  readonly attemptId: number;
  readonly action: string;
  readonly actionKind: FormActionKind;
  readonly resultKind: FormActionResultKind;
  readonly planDigest: string;
  readonly idempotency: FormActionIdempotency;
  readonly destructive: boolean;
  readonly reason: string;
  readonly blockers: ReadonlyArray<FormReadinessBlocker>;
  readonly recoveryActions: ReadonlyArray<FormActionRecovery>;
  readonly proof: FormActionPlan["proof"];
  readonly patch: FormActionPlan["patch"];
  readonly collapsedIntoAttemptId?: number;
  readonly supersededAttemptId?: number;
  readonly supersededByAttemptId?: number;
  readonly queuePosition?: number;
  readonly repeatedAttempt: FormActionIdempotency;
  readonly resultDigest: string;
}

export type FormActionExecutionResultKind =
  | "pending"
  | "accepted"
  | "denied"
  | "noOp"
  | "fulfilled"
  | "rejected"
  | "cancelled"
  | "timedOut"
  | "superseded"
  | "staleCompletion";

export interface FormServerMessageArtifact {
  readonly code: string;
  readonly target: string | null;
  readonly scope: string;
  readonly severity: string;
  readonly source: "server";
}

export interface FormActionExecutionArtifact {
  readonly kind: "actionExecution";
  readonly operationId: number;
  readonly targetOperationId?: number;
  readonly targetAction?: string | null;
  readonly targetPlanDigest?: string | null;
  readonly targetExecutionDigest?: string | null;
  readonly action: string | null;
  readonly actionKind: FormActionKind | null;
  readonly attemptId: number | null;
  readonly attemptResultKind: FormActionResultKind | null;
  readonly resultKind: FormActionExecutionResultKind;
  readonly planDigest: string | null;
  readonly attemptDigest: string | null;
  readonly effectStarted: boolean;
  readonly stale: boolean;
  readonly reason: string;
  readonly proof?: FormActionPlan["proof"];
  readonly planSnapshot?: FormActionPlan;
  readonly attempt?: FormActionResultArtifact;
  readonly serverMessages: ReadonlyArray<FormServerMessageArtifact>;
  readonly canonicalValue?: SignalValue;
  readonly retryOfOperationId?: number;
  readonly supersededOperationId?: number;
  readonly supersededByOperationId?: number;
  readonly executionDigest: string;
}

export interface FormActionsReport {
  readonly catalog: ReadonlyArray<FormActionCatalogEntry>;
  readonly plans: ReadonlyArray<FormActionPlan>;
  readonly summary: {
    readonly total: number;
    readonly accepted: number;
    readonly denied: number;
    readonly unavailable: number;
    readonly cancelled: number;
    readonly superseded: number;
    readonly rejected: number;
    readonly fulfilled: number;
    readonly noOp: number;
    readonly destructive: number;
    readonly step: number;
  };
  readonly counters: {
    readonly costBasis: "derivedFullReportScan";
    readonly incrementalStatus: "notIncremental";
    readonly declarations: number;
    readonly plans: number;
    readonly deniedPlans: number;
    readonly destructivePlans: number;
    readonly stepPlans: number;
    readonly nonEmptyPatchPlans: number;
  };
  readonly digests: {
    readonly catalogDigest: string;
    readonly readinessAdmissionDigest: string;
    readonly planDigestSetDigest: string;
    readonly submitPlanDigest: string | null;
    readonly planDigests: Readonly<Record<string, string>>;
  };
}
