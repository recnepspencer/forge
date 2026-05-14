import type { FormActionsReport } from "./actions.js";
import type { FormAdmissionReport } from "./admission.js";
import type { FormAvailabilityReport } from "./availability.js";
import type { FormStepsReport } from "./steps.js";
import type { FormValidationReport } from "./validation.js";
import type { FormHostReport } from "./host.js";
import type { FormExitReport } from "./exit.js";
import type { FormHandoffReport } from "./handoff.js";
import type { FormAttachmentsReport } from "./attachments.js";
import type { FormMediaReport } from "./media.js";
import type { FormCollaborationReport } from "./collaboration.js";
import type { FormInteractionReport } from "./interaction.js";
import type { FormNavigationReport } from "./navigation.js";
import type { FormAccessibilityReport } from "./accessibility.js";
import type { FormLayoutReport } from "./layout.js";
import type { FormLayoutMeasurementReport } from "./measurement.js";
import type { FormPresentationReport } from "./presentation.js";
import type { FormInputCapabilitiesReport } from "./input_capabilities.js";

export interface FormVerificationPackage {
  readonly kind: "formVerification";
  readonly digests: {
    readonly sourceAuthorityDigest: string;
    readonly hostFactDigest: string;
    readonly inputCapabilityDigest: string;
    readonly exitDigest: string;
    readonly handoffDigest: string;
    readonly attachmentDigest: string;
    readonly mediaDigest: string;
    readonly collaborationDigest: string;
    readonly interactionDigest: string;
    readonly interactionHistoryDigest: string;
    readonly navigationDigest: string;
    readonly navigationHistoryDigest: string;
    readonly accessibilityDigest: string;
    readonly presentationOrderHintDigest: string | null;
    readonly layoutDigest: string;
    readonly layoutMeasurementDigest: string;
    readonly presentationDigest: string;
    readonly presentationSettlementAcknowledgementDigest: string | null;
    readonly sourceCompatibilityDigest: string;
    readonly draftDigest: string;
    readonly effectiveValueDigest: string;
    readonly semanticEqualityDigest: string;
    readonly patchPlanDigest: string;
    readonly readinessDigest: string;
    readonly validationDigest: string;
    readonly availabilityDependencyDigest: string;
    readonly stepDeclarationProgressDigest: string;
    readonly admissionPolicyDigest: string;
    readonly regulatedBindingDigest: string;
    readonly actionCatalogDigest: string;
    readonly actionReadinessAdmissionDigest: string;
    readonly actionPlanDigestSetDigest: string;
    readonly submitPlanDigest: string | null;
    readonly actionLifecycleDigest: string;
    readonly actionExecutionLifecycleDigest: string;
    readonly asyncValidationLifecycleDigest: string;
    readonly canonicalizationDigest: string;
    readonly sourceCompatibilityHistoryDigest: string;
    readonly presentationHistoryDigest: string;
    readonly diagnosticsHistoryDigest: string;
  };
  readonly actionHistory: {
    readonly attempts: number;
    readonly digest: string;
  };
  readonly actionExecutionHistory: {
    readonly operations: number;
    readonly digest: string;
  };
  readonly interactionHistory: {
    readonly operations: number;
    readonly digest: string;
  };
  readonly navigationHistory: {
    readonly operations: number;
    readonly digest: string;
  };
  readonly asyncValidationHistory: {
    readonly operations: number;
    readonly digest: string;
  };
  readonly canonicalizationHistory: {
    readonly operations: number;
    readonly digest: string;
  };
  readonly presentationHistory: {
    readonly operations: number;
    readonly digest: string;
  };
  readonly sourceCompatibilityHistory: {
    readonly operations: number;
    readonly digest: string;
  };
  readonly performanceEnvelope: {
    readonly costBasis: "derivedFullReportScan";
    readonly diagnosticsSummaryBreadth: "summaryShapedNotFullHistoryMaterialization";
    readonly actionHistoryAttempts: number;
    readonly actionExecutionOperations: number;
    readonly asyncValidationOperations: number;
    readonly canonicalizationOperations: number;
    readonly interactionOperations: number;
    readonly navigationOperations: number;
    readonly sourceCompatibilityOperations: number;
    readonly hostFacts: FormHostReport["counters"];
    readonly inputCapabilities: FormInputCapabilitiesReport["counters"];
    readonly exit: FormExitReport["counters"];
    readonly handoff: FormHandoffReport["counters"];
    readonly attachments: FormAttachmentsReport["counters"];
    readonly media: FormMediaReport["counters"];
    readonly collaboration: FormCollaborationReport["counters"];
    readonly interaction: FormInteractionReport["counters"];
    readonly navigation: FormNavigationReport["counters"];
    readonly accessibility: FormAccessibilityReport["counters"];
    readonly layout: FormLayoutReport["counters"];
    readonly layoutMeasurement: FormLayoutMeasurementReport["counters"];
    readonly presentation: FormPresentationReport["counters"];
    readonly sourceCompatibility: {
      readonly costBasis: "sourceSchemaCompatibilityDerivedScan";
      readonly incrementalStatus: "notIncremental";
      readonly schemaReads: number;
      readonly migrations: number;
      readonly compatibleDrifts: number;
      readonly unavailableDrifts: number;
      readonly historyArtifacts: number;
    };
    readonly validation: FormValidationReport["counters"];
    readonly availability: FormAvailabilityReport["counters"];
    readonly admission: FormAdmissionReport["counters"];
    readonly steps: FormStepsReport["counters"];
    readonly actions: FormActionsReport["counters"];
  };
  readonly packageDigest: string;
}
