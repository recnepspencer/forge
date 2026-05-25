import type { FormActionsReport } from "./actions.js";
import type { FormAdmissionReport } from "./admission.js";
import type { FormAvailabilityReport } from "./availability.js";
import type { FormHostReport } from "./host.js";
import type { FormInteractionReport } from "./interaction.js";
import type { FormNavigationReport } from "./navigation.js";
import type { FormPresentationReport } from "./presentation.js";
import type { FormReadinessBlocker } from "./core.js";
import type { FormStepsReport } from "./steps.js";

export interface FormDiagnosticsSummaryReport {
  readonly kind: "formDiagnosticsSummary";
  readonly fieldCount: number;
  readonly dirty: {
    readonly isDirty: boolean;
    readonly semanticDirty: boolean;
    readonly changedFields: number;
    readonly omittedFields: number;
    readonly clearedFields: number;
    readonly digest: string;
  };
  readonly patch: {
    readonly empty: boolean;
    readonly semanticDirty: boolean;
    readonly operationCount: number;
    readonly blockerCount: number;
    readonly broadReplacement: boolean;
    readonly digest: string;
  };
  readonly readiness: {
    readonly canSubmit: boolean;
    readonly blockerCount: number;
    readonly blockerKinds: ReadonlyArray<FormReadinessBlocker["kind"]>;
    readonly digest: string;
  };
  readonly validation: {
    readonly summary: {
      readonly valid: number;
      readonly invalid: number;
      readonly pending: number;
      readonly blocked: number;
      readonly unavailable: number;
    };
    readonly digest: string;
  };
  readonly availability: {
    readonly summary: FormAvailabilityReport["summary"];
    readonly digest: string;
  };
  readonly admission: {
    readonly summary: FormAdmissionReport["summary"];
    readonly digest: string;
  };
  readonly resourceSource: {
    readonly present: boolean;
    readonly digest: string | null;
    readonly settlementKind: string | null;
    readonly lifecycleActivity: string | null;
  };
  readonly host: {
    readonly summary: FormHostReport["summary"];
    readonly digest: string;
  };
  readonly interaction: {
    readonly summary: FormInteractionReport["summary"];
    readonly digest: string;
  };
  readonly navigation: {
    readonly summary: FormNavigationReport["summary"];
    readonly digest: string;
  };
  readonly presentation: {
    readonly summary: FormPresentationReport["summary"];
    readonly digest: string;
  };
  readonly routeAuthority: {
    readonly authorityAvailable: boolean;
    readonly continuity: FormRouteAuthorityReport["summary"]["continuity"];
    readonly transitionKind: FormRouteAuthorityReport["summary"]["transitionKind"];
    readonly handoff: null | {
      readonly posture: NonNullable<FormRouteAuthorityReport["summary"]["handoff"]>["posture"];
      readonly routeCoupledBehavior:
        NonNullable<FormRouteAuthorityReport["summary"]["handoff"]>["routeCoupledBehavior"];
      readonly draftDisposition:
        NonNullable<FormRouteAuthorityReport["summary"]["handoff"]>["draftDisposition"];
    };
    readonly draftContinuity: null | {
      readonly posture:
        NonNullable<FormRouteAuthorityReport["summary"]["draftContinuity"]>["posture"];
      readonly authorityChange:
        NonNullable<FormRouteAuthorityReport["summary"]["draftContinuity"]>["authorityChange"];
      readonly draftResolution:
        NonNullable<FormRouteAuthorityReport["summary"]["draftContinuity"]>["draftResolution"];
      readonly draftChanged:
        NonNullable<FormRouteAuthorityReport["summary"]["draftContinuity"]>["draftChanged"];
    };
    readonly continuityAudit: {
      readonly kind: "routeAuthorityContinuityAudit";
      readonly handoffPosture:
        NonNullable<FormRouteAuthorityReport["summary"]["handoff"]>["posture"] | null;
      readonly routeCoupledBehavior:
        NonNullable<FormRouteAuthorityReport["summary"]["handoff"]>["routeCoupledBehavior"] | null;
      readonly draftDisposition:
        NonNullable<FormRouteAuthorityReport["summary"]["handoff"]>["draftDisposition"] | null;
      readonly draftResolution:
        NonNullable<FormRouteAuthorityReport["summary"]["draftContinuity"]>["draftResolution"] | null;
      readonly transitionKind: FormRouteAuthorityReport["summary"]["transitionKind"];
      readonly authorityAvailable: boolean;
      readonly routeCoupledSteps: {
        readonly total: number;
        readonly active: number;
        readonly unavailable: number;
      };
      readonly routeCoupledActions: {
        readonly total: number;
        readonly accepted: number;
        readonly denied: number;
      };
      readonly blockingReason: string | null;
      readonly digest: string;
    };
    readonly digest: string;
  };
  readonly sourceCompatibility: {
    readonly posture: "notDeclared" | "current" | "compatible" | "migrated" | "unavailable";
    readonly digest: string;
  };
  readonly steps: {
    readonly summary: FormStepsReport["summary"];
    readonly digest: string;
  };
  readonly actions: {
    readonly summary: FormActionsReport["summary"];
    readonly digest: string;
  };
  readonly histories: {
    readonly actionAttempts: number;
    readonly actionExecutions: number;
    readonly asyncValidations: number;
    readonly canonicalizations: number;
    readonly resets: number;
    readonly stateTransitions: number;
    readonly replayRestores: number;
    readonly sourceCompatibility: number;
    readonly presentations: number;
  };
  readonly digest: string;
}

export interface FormDiagnosticsHistoryArtifact {
  readonly kind: "formDiagnosticsHistory";
  readonly artifactId: number;
  readonly observedAtMs: number;
  readonly summaryDigest: string;
  readonly diagnosticsStateDigest: string;
  readonly sourceAuthorityDigest: string;
  readonly patchPlanDigest: string;
  readonly readinessDigest: string;
  readonly validationDigest: string;
  readonly availabilityDigest: string;
  readonly admissionDigest: string;
  readonly actionPlanDigestSetDigest: string;
  readonly actionLifecycleDigest: string;
  readonly actionExecutionLifecycleDigest: string;
  readonly asyncValidationDigest: string;
  readonly canonicalizationDigest: string;
  readonly sourceCompatibilityDigest: string;
  readonly sourceCompatibilityHistoryDigest: string;
  readonly routeAuthorityDigest: string;
  readonly routeAuthorityTransitionKind: FormRouteAuthorityReport["summary"]["transitionKind"];
  readonly routeAuthorityHandoffPosture:
    FormRouteAuthorityReport["summary"]["handoff"] extends infer THandoff
      ? THandoff extends null
        ? never
        : NonNullable<THandoff>["posture"] | null
      : never;
  readonly routeAuthorityRouteCoupledBehavior:
    FormRouteAuthorityReport["summary"]["handoff"] extends infer THandoff
      ? THandoff extends null
        ? never
        : NonNullable<THandoff>["routeCoupledBehavior"] | null
      : never;
  readonly routeAuthorityDraftResolution:
    FormRouteAuthorityReport["summary"]["draftContinuity"] extends infer TContinuity
      ? TContinuity extends null
        ? never
        : NonNullable<TContinuity>["draftResolution"] | null
      : never;
  readonly routeAuthorityContinuityAuditDigest: string;
  readonly resourceSourceDigest: string | null;
  readonly collaborationDigest: string;
  readonly interactionDigest: string;
  readonly navigationDigest: string;
  readonly presentationDigest: string;
  readonly historyCounts: FormDiagnosticsSummaryReport["histories"];
  readonly diagnosticsDigest: string;
}
