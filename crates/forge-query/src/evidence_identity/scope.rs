#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryEvidenceScope {
    RuntimePublicSupportMatrixRow,
    RuntimePublicSupportMatrix,
    RuntimePublicApiFamilyContract,
    RuntimePublicApiContract,
    RuntimePublicApiTranscriptEvidence,
    RuntimeHostileCertificationArtifact,
    RuntimeStateSnapshot,
    SessionLabelIdentity,
    BasisAdmissionEvidenceRow,
    PreviewBasisAdmission,
    BranchBasisAdmission,
    PreviewIntentAdmission,
    PreviewIntentReceipt,
    BranchIntentAdmission,
    BranchIntentReceipt,
    IntentDenialEvidence,
    PreviewCloseoutEvidence,
    PreviewPromotionDenialEvidence,
    PreviewExecutionEvidence,
    PreviewPromotionRebinding,
    PreviewWriteReceiptIdentity,
    PreviewBindingInspectionArtifact,
    PreviewOutcomeInspectionArtifact,
    CausalObservationReceipt,
    CausalObservationQuery,
    CausalObservationBasis,
    CausalObservationTarget,
    CausalResultShapeContext,
    CausalQueryObservationReceipt,
    CausalObservationAnchor,
    CausalObservationAnchorCounters,
    CausalObservationAnchorFailure,
    CausalEvidenceReference,
    CausalEvidenceReferenceReceipt,
    CausalEvidenceReferenceResolutionCounters,
    CausalEvidenceReferenceResolutionDenial,
    CausalEvidenceReferenceIndex,
    CausalEvidenceReferenceIndexRecord,
    CausalEvidenceReferenceIndexError,
    CausalInspectionTarget,
    CausalInspectionRequest,
    CausalInspectionRequestFailure,
    CausalInspectionAdmissionSubject,
    CausalInspectionAdmissionDecision,
    CausalInspectionDecisionTraceRow,
    CausalInspectionDecisionTraceIndex,
    CausalInspectionAdmissionCounters,
    CausalInspectionAdmissionReceipt,
    CausalInspectionOutcome,
    CausalInspectionMaterializedDetail,
    CausalInspectionDeniedArtifactDetail,
    CausalInspectionArtifact,
    CausalInspectionArtifactIdentity,
    CausalInspectionPerformanceSnapshot,
    CausalInspectionPerformanceSlope,
    CausalInspectionPerformanceScaleSlope,
    CausalInspectionPerformanceCertificationBundle,
    RuntimePublicApiNamingRow,
    RuntimePublicApiNamingContract,
    GraphCompositionDomainInvariantDenial,
    GraphCompositionInvariantViolation,
    ReadDomainInvariantDenial,
    ReadInvariantViolation,
    ApplicationSupportSectionPosture,
    ApplicationSupportReport,
    ApplicationEvidenceIdentityBoundaryClosure,
    ApplicationStopClassBoundaryClosure,
    ApplicationSessionLabelBoundaryClosure,
    ApplicationIdentityBoundaryClosure,
}

impl ForgeQueryEvidenceScope {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RuntimePublicSupportMatrixRow => "runtime-public-support-matrix-row",
            Self::RuntimePublicSupportMatrix => "runtime-public-support-matrix",
            Self::RuntimePublicApiFamilyContract => "runtime-public-api-family-contract",
            Self::RuntimePublicApiContract => "runtime-public-api-contract",
            Self::RuntimePublicApiTranscriptEvidence => "runtime-public-api-transcript-evidence",
            Self::RuntimeHostileCertificationArtifact => "runtime-hostile-certification-artifact",
            Self::RuntimeStateSnapshot => "runtime-state-snapshot",
            Self::SessionLabelIdentity => "session-label-identity",
            Self::BasisAdmissionEvidenceRow => "basis-admission-evidence-row",
            Self::PreviewBasisAdmission => "preview-basis-admission",
            Self::BranchBasisAdmission => "branch-basis-admission",
            Self::PreviewIntentAdmission => "preview-intent-admission",
            Self::PreviewIntentReceipt => "preview-intent-receipt",
            Self::BranchIntentAdmission => "branch-intent-admission",
            Self::BranchIntentReceipt => "branch-intent-receipt",
            Self::IntentDenialEvidence => "intent-denial-evidence",
            Self::PreviewCloseoutEvidence => "preview-closeout-evidence",
            Self::PreviewPromotionDenialEvidence => "preview-promotion-denial-evidence",
            Self::PreviewExecutionEvidence => "preview-execution-evidence",
            Self::PreviewPromotionRebinding => "preview-promotion-rebinding",
            Self::PreviewWriteReceiptIdentity => "preview-write-receipt-identity",
            Self::PreviewBindingInspectionArtifact => "preview-binding-inspection-artifact",
            Self::PreviewOutcomeInspectionArtifact => "preview-outcome-inspection-artifact",
            Self::CausalObservationReceipt => "causal-observation-receipt",
            Self::CausalObservationQuery => "causal-observation-query",
            Self::CausalObservationBasis => "causal-observation-basis",
            Self::CausalObservationTarget => "causal-observation-target",
            Self::CausalResultShapeContext => "causal-result-shape-context",
            Self::CausalQueryObservationReceipt => "causal-query-observation-receipt",
            Self::CausalObservationAnchor => "causal-observation-anchor",
            Self::CausalObservationAnchorCounters => "causal-observation-anchor-counters",
            Self::CausalObservationAnchorFailure => "causal-observation-anchor-failure",
            Self::CausalEvidenceReference => "causal-evidence-reference",
            Self::CausalEvidenceReferenceReceipt => "causal-evidence-reference-receipt",
            Self::CausalEvidenceReferenceResolutionCounters => {
                "causal-evidence-reference-resolution-counters"
            }
            Self::CausalEvidenceReferenceResolutionDenial => {
                "causal-evidence-reference-resolution-denial"
            }
            Self::CausalEvidenceReferenceIndex => "causal-evidence-reference-index",
            Self::CausalEvidenceReferenceIndexRecord => "causal-evidence-reference-index-record",
            Self::CausalEvidenceReferenceIndexError => "causal-evidence-reference-index-error",
            Self::CausalInspectionTarget => "causal-inspection-target",
            Self::CausalInspectionRequest => "causal-inspection-request",
            Self::CausalInspectionRequestFailure => "causal-inspection-request-failure",
            Self::CausalInspectionAdmissionSubject => "causal-inspection-admission-subject",
            Self::CausalInspectionAdmissionDecision => "causal-inspection-admission-decision",
            Self::CausalInspectionDecisionTraceRow => "causal-inspection-decision-trace-row",
            Self::CausalInspectionDecisionTraceIndex => "causal-inspection-decision-trace-index",
            Self::CausalInspectionAdmissionCounters => "causal-inspection-admission-counters",
            Self::CausalInspectionAdmissionReceipt => "causal-inspection-admission-receipt",
            Self::CausalInspectionOutcome => "causal-inspection-outcome",
            Self::CausalInspectionMaterializedDetail => "causal-inspection-materialized-detail",
            Self::CausalInspectionDeniedArtifactDetail => {
                "causal-inspection-denied-artifact-detail"
            }
            Self::CausalInspectionArtifact => "causal-inspection-artifact",
            Self::CausalInspectionArtifactIdentity => "causal-inspection-artifact-identity",
            Self::CausalInspectionPerformanceSnapshot => "causal-inspection-performance-snapshot",
            Self::CausalInspectionPerformanceSlope => "causal-inspection-performance-slope",
            Self::CausalInspectionPerformanceScaleSlope => {
                "causal-inspection-performance-scale-slope"
            }
            Self::CausalInspectionPerformanceCertificationBundle => {
                "causal-inspection-performance-certification-bundle"
            }
            Self::RuntimePublicApiNamingRow => "runtime-public-api-naming-row",
            Self::RuntimePublicApiNamingContract => "runtime-public-api-naming-contract",
            Self::GraphCompositionDomainInvariantDenial => {
                "graph-composition-domain-invariant-denial"
            }
            Self::GraphCompositionInvariantViolation => "graph-composition-invariant-violation",
            Self::ReadDomainInvariantDenial => "read-domain-invariant-denial",
            Self::ReadInvariantViolation => "read-invariant-violation",
            Self::ApplicationSupportSectionPosture => "application-support-section-posture",
            Self::ApplicationSupportReport => "application-support-report",
            Self::ApplicationEvidenceIdentityBoundaryClosure => {
                "application-evidence-identity-boundary-closure"
            }
            Self::ApplicationStopClassBoundaryClosure => "application-stop-class-boundary-closure",
            Self::ApplicationSessionLabelBoundaryClosure => {
                "application-session-label-boundary-closure"
            }
            Self::ApplicationIdentityBoundaryClosure => "application-identity-boundary-closure",
        }
    }
}
