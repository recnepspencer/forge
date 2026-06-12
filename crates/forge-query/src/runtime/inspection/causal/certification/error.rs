use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};

use super::super::identity::CausalInspectionCertificationErrorIdentity;

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
    failure_identity: CausalInspectionCertificationErrorIdentity,
}

impl CausalInspectionCertificationError {
    pub(super) fn new(
        kind: CausalInspectionCertificationErrorKind,
        message: &'static str,
        evidence: &[String],
    ) -> Self {
        let failure_identity = forge_query_evidence_identity(
            ForgeQueryEvidenceScope::CausalInspectionCertificationError,
        )
        .field_shape(ForgeQueryEvidenceTag::new("kind"), kind.as_str())
        .field_value(ForgeQueryEvidenceTag::new("message"), message)
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("evidence"),
            evidence.iter().map(String::as_str),
        )
        .seal()
        .into();
        Self {
            kind,
            message,
            failure_identity,
        }
    }

    pub fn kind(&self) -> CausalInspectionCertificationErrorKind {
        self.kind
    }

    pub fn message(&self) -> &str {
        self.message
    }

    pub fn failure_digest(&self) -> &str {
        self.failure_identity.as_str()
    }
}
