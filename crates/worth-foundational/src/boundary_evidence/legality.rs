use super::primitives::{
    FoundationalBoundaryEvidenceCategory, FoundationalBoundaryEvidenceDescriptiveRole,
    FoundationalBoundaryEvidenceExecutionPosture, FoundationalBoundaryEvidenceFreshnessPosture,
    FoundationalBoundaryEvidenceLocality,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalBoundaryEvidencePrimitiveLegalityDenial {
    SupportTruthRequiresSupportGradeRole,
    NonSupportTruthMustNotClaimSupportGradeRole,
}

pub fn evaluate_boundary_evidence_primitive_legality(
    category: FoundationalBoundaryEvidenceCategory,
    _locality: FoundationalBoundaryEvidenceLocality,
    _execution_posture: FoundationalBoundaryEvidenceExecutionPosture,
    descriptive_role: FoundationalBoundaryEvidenceDescriptiveRole,
    _freshness_posture: FoundationalBoundaryEvidenceFreshnessPosture,
) -> Result<(), FoundationalBoundaryEvidencePrimitiveLegalityDenial> {
    if category == FoundationalBoundaryEvidenceCategory::SupportTruth
        && descriptive_role != FoundationalBoundaryEvidenceDescriptiveRole::SupportGrade
    {
        return Err(
            FoundationalBoundaryEvidencePrimitiveLegalityDenial::SupportTruthRequiresSupportGradeRole,
        );
    }

    if category != FoundationalBoundaryEvidenceCategory::SupportTruth
        && descriptive_role == FoundationalBoundaryEvidenceDescriptiveRole::SupportGrade
    {
        return Err(
            FoundationalBoundaryEvidencePrimitiveLegalityDenial::NonSupportTruthMustNotClaimSupportGradeRole,
        );
    }

    Ok(())
}
