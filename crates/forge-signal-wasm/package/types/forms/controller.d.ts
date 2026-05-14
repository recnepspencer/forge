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
import type { FormHostReport } from "./host.js";
import type { FormHostBindings } from "./host.js";
import type { FormExitPresentationArtifact, FormExitReport } from "./exit.js";
import type { FormHandoffPresentationArtifact, FormHandoffReport } from "./handoff.js";
import type { FormInputCapabilitiesReport } from "./input_capabilities.js";
import type { FormAttachmentsReport, FormAttachmentPresentationArtifact } from "./attachments.js";
import type { FormMediaReport, FormMediaPresentationArtifact } from "./media.js";
import type {
  FormCollaborationArtifact,
  FormCollaborationDeclaration,
  FormCollaborationReport,
} from "./collaboration.js";
import type { FormInteractionReport } from "./interaction.js";
import type { FormNavigationReport } from "./navigation.js";
import type { FormAccessibilityReport } from "./accessibility.js";
import type { FormLayoutReport } from "./layout.js";
import type {
  FormPresentationDeclaration,
  FormPresentationHistoryArtifact,
  FormPresentationLifecycleArtifact,
  FormPresentationLaneUpdateArtifact,
  FormPresentationReport,
  FormPresentationSettlementArtifact,
} from "./presentation.js";
import type {
  FormLayoutMeasurementDeclaration,
  FormLayoutMeasurementReport,
  FormLayoutRowMeasurement,
  FormLayoutMeasurementCause,
  FormLayoutSnapshotArtifact,
} from "./measurement.js";
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
  host?: FormHostBindings;
  collaboration?: FormCollaborationDeclaration;
  measurement?: FormLayoutMeasurementDeclaration;
  presentation?: FormPresentationDeclaration;
}

export interface FormController<
  TSource = SignalValue,
  TFieldHandles extends Record<string, FormFieldHandle> = Record<string, FormFieldHandle>,
> {
  readonly fields: TFieldHandles;
  source(): TSource;
  draft(): Partial<TSource>;
  effective(): TSource;
  host(): FormHostReport;
  inputCapabilities(): FormInputCapabilitiesReport;
  exit(): FormExitReport;
  handoff(): FormHandoffReport;
  attachments(): FormAttachmentsReport;
  media(): FormMediaReport;
  collaboration(): FormCollaborationReport;
  interaction(): FormInteractionReport;
  reportFieldInteraction(
    fieldId: string,
    event: {
      readonly kind:
        | "touch"
        | "visit"
        | "focus"
        | "blur"
        | "input"
        | "compositionStart"
        | "compositionCommit"
        | "compositionCancel";
      readonly source?: string;
      readonly rawValue?: SignalValue;
    },
  ): FormInteractionReport["history"][number];
  reportSubmitIntent(options?: {
    readonly source?: "keyboard" | "pointer" | "programmatic";
  }): FormInteractionReport["history"][number];
  clearSubmitIntent(options?: { readonly reason?: string }): FormInteractionReport["history"][number];
  navigation(): FormNavigationReport;
  accessibility(): FormAccessibilityReport;
  layout(): FormLayoutReport;
  layoutMeasurement(): FormLayoutMeasurementReport;
  presentation(): FormPresentationReport;
  presentationLifecycle(laneId?: string): FormPresentationReport | FormPresentationLifecycleArtifact | null;
  reportPresentationLane(
    laneId: "collaboration" | "exit" | "attachments" | "media" | "handoff",
    update: {
      readonly status: "pending" | "busy" | "settling" | "ready" | "failed" | "unavailable";
      readonly target?: string | null;
      readonly reason: string;
      readonly token?: string | null;
    },
  ): FormPresentationLaneUpdateArtifact;
  clearPresentationLane(
    laneId: "collaboration" | "exit" | "attachments" | "media" | "handoff",
    options?: { readonly reason?: string },
  ): FormPresentationLaneUpdateArtifact;
  reportExit(update: {
    readonly status: "pending" | "busy" | "settling" | "ready" | "failed" | "unavailable";
    readonly target?: string | null;
    readonly reason: string;
    readonly token?: string | null;
    readonly scopeKind?: "route" | "modal" | "external" | null;
    readonly surfaceId?: string | null;
    readonly operation?: "generic" | "block" | "confirm" | "dismiss" | "leave" | "stay" | "close";
    readonly unsupportedReason?: string | null;
  }): FormExitPresentationArtifact;
  clearExit(options?: { readonly reason?: string }): FormExitPresentationArtifact;
  reportHandoff(update: {
    readonly status: "pending" | "busy" | "settling" | "ready" | "failed" | "unavailable";
    readonly target?: string | null;
    readonly reason: string;
    readonly token?: string | null;
    readonly scopeKind?: "route" | "modal" | "external" | null;
    readonly surfaceId?: string | null;
    readonly operation?: "generic" | "open" | "handoff" | "dismiss" | "return" | "close";
    readonly unsupportedReason?: string | null;
  }): FormHandoffPresentationArtifact;
  clearHandoff(options?: { readonly reason?: string }): FormHandoffPresentationArtifact;
  reportAttachments(update: {
    readonly status: "pending" | "busy" | "settling" | "ready" | "failed" | "unavailable";
    readonly target?: string | null;
    readonly reason: string;
    readonly token?: string | null;
    readonly section?: string | null;
    readonly selectedCount?: number;
    readonly stagedCount?: number;
    readonly failedCount?: number;
    readonly operation?: "generic" | "select" | "stage" | "preview" | "remove" | "clear";
  }): FormAttachmentPresentationArtifact;
  clearAttachments(options?: { readonly reason?: string }): FormAttachmentPresentationArtifact;
  reportMedia(update: {
    readonly status: "pending" | "busy" | "settling" | "ready" | "failed" | "unavailable";
    readonly target?: string | null;
    readonly reason: string;
    readonly token?: string | null;
    readonly mode?: "preview" | "capture" | "crop" | "annotate" | null;
    readonly surfaceId?: string | null;
    readonly operation?: "generic" | "open" | "replace" | "annotate" | "close";
  }): FormMediaPresentationArtifact;
  clearMedia(options?: { readonly reason?: string }): FormMediaPresentationArtifact;
  reportCollaboration(update: {
    readonly posture?: "active" | "blocked" | "settling" | "unavailable";
    readonly reason?: string;
    readonly lockOwnerId?: string | null;
    readonly leasedFields?: ReadonlyArray<{ readonly field: string; readonly ownerId: string }>;
    readonly branchId?: string | null;
    readonly readOnly?: boolean;
    readonly remoteUpdateDigest?: string | null;
    readonly presence?: ReadonlyArray<{ readonly actorId: string; readonly status: "active" | "idle" | "viewing" }>;
    readonly comments?: ReadonlyArray<{ readonly id: string; readonly authorId: string; readonly target?: string | null }>;
  }): FormCollaborationArtifact;
  clearCollaboration(options?: { readonly reason?: string }): FormCollaborationArtifact;
  acknowledgePresentation(laneId: string): FormPresentationSettlementArtifact;
  timeoutPresentation(laneId: string, options?: { readonly reason?: string }): FormPresentationSettlementArtifact;
  presentationHistory(): ReadonlyArray<FormPresentationHistoryArtifact>;
  recordLayoutMeasurement(
    rows: ReadonlyArray<FormLayoutRowMeasurement>,
    options?: {
      readonly cause?: FormLayoutMeasurementCause;
      readonly frameToken?: string | number | null;
    },
  ): FormLayoutSnapshotArtifact;
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
  sourceCompatibility(): {
    readonly posture: "notDeclared" | "current" | "compatible" | "migrated" | "unavailable";
    readonly currentSchemaVersion: string | null;
    readonly draftSchemaVersion: string | null;
    readonly reason: string | null;
    readonly artifact: {
      readonly kind: "sourceCompatibility";
      readonly artifactId: number;
      readonly posture: "compatible" | "migrated" | "unavailable";
      readonly previousSchemaVersion: string | null;
      readonly currentSchemaVersion: string | null;
      readonly previousDraftDigest: string;
      readonly nextDraftDigest: string;
      readonly nextDraft: SignalValue | null;
      readonly reason: string;
      readonly resolutionKey: string;
      readonly compatibilityDigest: string;
    } | null;
    readonly counters: {
      readonly costBasis: "sourceSchemaCompatibilityDerivedScan";
      readonly incrementalStatus: "notIncremental";
      readonly schemaReads: number;
      readonly migrations: number;
      readonly compatibleDrifts: number;
      readonly unavailableDrifts: number;
      readonly historyArtifacts: number;
    };
  };
  sourceCompatibilityHistory(): ReadonlyArray<{
    readonly kind: "sourceCompatibility";
    readonly artifactId: number;
    readonly posture: "compatible" | "migrated" | "unavailable";
    readonly previousSchemaVersion: string | null;
    readonly currentSchemaVersion: string | null;
    readonly previousDraftDigest: string;
    readonly nextDraftDigest: string;
    readonly nextDraft: SignalValue | null;
    readonly reason: string;
    readonly resolutionKey: string;
    readonly compatibilityDigest: string;
  }>;
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
    readonly host: FormHostReport;
    readonly inputCapabilities: FormInputCapabilitiesReport;
    readonly exit: FormExitReport;
    readonly handoff: FormHandoffReport;
    readonly attachments: FormAttachmentsReport;
    readonly media: FormMediaReport;
    readonly collaboration: FormCollaborationReport;
    readonly interaction: FormInteractionReport;
    readonly navigation: FormNavigationReport;
    readonly accessibility: FormAccessibilityReport;
    readonly layout: FormLayoutReport;
    readonly layoutMeasurement: FormLayoutMeasurementReport;
    readonly presentation: FormPresentationReport;
    readonly sourceCompatibility: FormController<TSource, TFieldHandles>["sourceCompatibility"] extends () => infer TReport
      ? TReport
      : never;
    readonly steps: FormStepsReport;
    readonly actions: FormActionsReport;
    readonly actionHistory: ReadonlyArray<FormActionResultArtifact>;
    readonly actionExecutionHistory: ReadonlyArray<FormActionExecutionArtifact>;
    readonly presentationHistory: ReadonlyArray<FormPresentationHistoryArtifact>;
    readonly asyncValidationHistory: ReadonlyArray<FormAsyncValidationLifecycleArtifact>;
    readonly canonicalizationHistory: ReadonlyArray<FormCanonicalizationArtifact>;
    readonly sourceCompatibilityHistory: ReturnType<FormController<TSource, TFieldHandles>["sourceCompatibilityHistory"]>;
    readonly verification: FormVerificationPackage;
  };
  field(fieldId: string): FormFieldHandle;
}

export type FormFieldHandleFor<TDeclaration> =
  TDeclaration extends FormFieldDeclaration<infer TValue, infer TRaw>
    ? FormFieldHandle<TValue, TRaw>
    : FormFieldHandle;
