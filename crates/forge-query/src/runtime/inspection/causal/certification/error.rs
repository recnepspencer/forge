use crate::identity::hash_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CausalInspectionCertificationErrorKind {
    MissingRequiredHostileLane,
    MissingRepresentativeMatrixRow,
    RepresentativeMatrixMismatch,
    RedactionIdentityDrift,
    PublicBoundaryBypass,
    ScaleSlopeDrift,
    ProofShapeBypass,
}

impl CausalInspectionCertificationErrorKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MissingRequiredHostileLane => "missing_required_hostile_lane",
            Self::MissingRepresentativeMatrixRow => "missing_representative_matrix_row",
            Self::RepresentativeMatrixMismatch => "representative_matrix_mismatch",
            Self::RedactionIdentityDrift => "redaction_identity_drift",
            Self::PublicBoundaryBypass => "public_boundary_bypass",
            Self::ScaleSlopeDrift => "scale_slope_drift",
            Self::ProofShapeBypass => "proof_shape_bypass",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CausalInspectionCertificationError {
    kind: CausalInspectionCertificationErrorKind,
    message: &'static str,
    failure_digest: String,
}

impl CausalInspectionCertificationError {
    pub(super) fn new(
        kind: CausalInspectionCertificationErrorKind,
        message: &'static str,
        evidence: &[String],
    ) -> Self {
        let mut parts = vec![
            "causal_inspection_certification_error_v1".to_string(),
            kind.as_str().to_string(),
            message.to_string(),
        ];
        parts.extend(evidence.iter().cloned());
        Self {
            kind,
            message,
            failure_digest: hash_parts(&parts),
        }
    }

    pub fn kind(&self) -> CausalInspectionCertificationErrorKind {
        self.kind
    }

    pub fn message(&self) -> &str {
        self.message
    }

    pub fn failure_digest(&self) -> &str {
        &self.failure_digest
    }
}
