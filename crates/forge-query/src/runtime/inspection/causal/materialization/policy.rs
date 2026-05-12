use crate::identity::hash_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CausalInspectionArtifactKind {
    Admitted,
    Advisory,
    Denied,
}

impl CausalInspectionArtifactKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::Advisory => "advisory",
            Self::Denied => "denied",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CausalInspectionRedactionPolicy {
    PreserveDetail,
    DigestOnly,
}

impl CausalInspectionRedactionPolicy {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::PreserveDetail => "preserve_detail",
            Self::DigestOnly => "digest_only",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CausalInspectionMaterializationPolicy {
    OfflineInterpretableArtifact,
    DigestReferenceOnly,
}

impl CausalInspectionMaterializationPolicy {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::OfflineInterpretableArtifact => "offline_interpretable_artifact",
            Self::DigestReferenceOnly => "digest_reference_only",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CausalInspectionBoundaryEnvelopeCategory {
    PrimaryResult,
    StructuredWarnings,
    DecisionTrace,
    StructuralDeltas,
    IntegrityMarkers,
    PerformanceAccounting,
}

impl CausalInspectionBoundaryEnvelopeCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::PrimaryResult => "primary_result",
            Self::StructuredWarnings => "structured_warnings",
            Self::DecisionTrace => "decision_trace",
            Self::StructuralDeltas => "structural_deltas",
            Self::IntegrityMarkers => "integrity_markers",
            Self::PerformanceAccounting => "performance_accounting",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CausalInspectionMaterializationErrorKind {
    AdmissionSummaryKindMismatch,
    AdmissionSummaryDigestMismatch,
    QueryObservationBindingMissing,
    QueryObservationBindingMismatch,
    QueryObservationBindingOverclaim,
    ReplayPostureUnsupported,
    MaterializationPolicyOverclaim,
}

impl CausalInspectionMaterializationErrorKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AdmissionSummaryKindMismatch => "admission_summary_kind_mismatch",
            Self::AdmissionSummaryDigestMismatch => "admission_summary_digest_mismatch",
            Self::QueryObservationBindingMissing => "query_observation_binding_missing",
            Self::QueryObservationBindingMismatch => "query_observation_binding_mismatch",
            Self::QueryObservationBindingOverclaim => "query_observation_binding_overclaim",
            Self::ReplayPostureUnsupported => "replay_posture_unsupported",
            Self::MaterializationPolicyOverclaim => "materialization_policy_overclaim",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CausalInspectionMaterializationError {
    kind: CausalInspectionMaterializationErrorKind,
    failure_digest: String,
}

impl CausalInspectionMaterializationError {
    pub(super) fn new(kind: CausalInspectionMaterializationErrorKind, evidence: &[String]) -> Self {
        let mut parts = vec![
            "causal_inspection_materialization_error_v1".to_string(),
            kind.as_str().to_string(),
        ];
        parts.extend(evidence.iter().cloned());
        Self {
            kind,
            failure_digest: hash_parts(&parts),
        }
    }

    pub fn kind(&self) -> CausalInspectionMaterializationErrorKind {
        self.kind
    }

    pub fn failure_digest(&self) -> &str {
        &self.failure_digest
    }
}

pub(super) fn boundary_categories() -> Vec<CausalInspectionBoundaryEnvelopeCategory> {
    vec![
        CausalInspectionBoundaryEnvelopeCategory::PrimaryResult,
        CausalInspectionBoundaryEnvelopeCategory::StructuredWarnings,
        CausalInspectionBoundaryEnvelopeCategory::DecisionTrace,
        CausalInspectionBoundaryEnvelopeCategory::StructuralDeltas,
        CausalInspectionBoundaryEnvelopeCategory::IntegrityMarkers,
        CausalInspectionBoundaryEnvelopeCategory::PerformanceAccounting,
    ]
}
