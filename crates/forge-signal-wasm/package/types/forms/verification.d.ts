import type { FormActionsReport } from "./actions.js";
import type { FormAdmissionReport } from "./admission.js";
import type { FormAvailabilityReport } from "./availability.js";
import type { FormStepsReport } from "./steps.js";
import type { FormValidationReport } from "./validation.js";
import type { FormSourceAuthorityDiagnostics } from "./sources.js";

export interface FormVerificationPackage {
  readonly kind: "formVerification";
  readonly digests: {
    readonly sourceAuthorityDigest: string;
    readonly sourceAuthorityContractDigest: string;
    readonly sourceValueDigest: string;
    readonly formDeclarationDigest: string;
    readonly fieldContractDigest: string;
    readonly inputAdapterCapabilityDigest: string;
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
    readonly diagnosticsHistoryDigest: string;
  };
  readonly sourceAuthority: FormSourceAuthorityDiagnostics;
  readonly actionHistory: {
    readonly attempts: number;
    readonly digest: string;
  };
  readonly actionExecutionHistory: {
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
  readonly performanceEnvelope: {
    readonly costBasis: "derivedFullReportScan";
    readonly diagnosticsSummaryBreadth: "summaryShapedNotFullHistoryMaterialization";
    readonly actionHistoryAttempts: number;
    readonly actionExecutionOperations: number;
    readonly asyncValidationOperations: number;
    readonly canonicalizationOperations: number;
    readonly validation: FormValidationReport["counters"];
    readonly availability: FormAvailabilityReport["counters"];
    readonly admission: FormAdmissionReport["counters"];
    readonly steps: FormStepsReport["counters"];
    readonly actions: FormActionsReport["counters"];
  };
  readonly packageDigest: string;
}
