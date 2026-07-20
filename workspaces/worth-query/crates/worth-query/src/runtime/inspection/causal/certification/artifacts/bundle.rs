use crate::identity_authority::{
    admit_query_causal_inspection_authority_identity, QueryCausalInspectionAuthorityIdentity,
    QueryCausalInspectionIdentityKind,
};
use crate::WorthQueryEvidenceIdentity;

use super::performance::CausalInspectionPerformanceCertificationBundle;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CausalInspectionCertificationScope {
    boundary_audit_digest: String,
    representative_matrix_digest: String,
    performance_certification: CausalInspectionPerformanceCertificationBundle,
    bridge_readmission_proof_digest: String,
    scale_slope_digest: String,
    anchor_derivation_slope_digest: String,
    reference_resolution_slope_digest: String,
    admission_slope_digest: String,
    bridge_envelope_slope_digest: String,
    materialization_slope_digest: String,
    artifact_serialization_slope_digest: String,
    proof_shape_digest: String,
    phase_progression_digest: String,
    witness_authority_digest: String,
    certification_row_count: usize,
    hostile_row_count: usize,
    representative_row_count: usize,
    scale_fixture_row_count: usize,
    scope_digest: String,
}

impl CausalInspectionCertificationScope {
    #[allow(clippy::too_many_arguments)]
    pub(in crate::runtime::inspection::causal::certification) fn from_parts(
        boundary_audit_digest: &str,
        representative_matrix_digest: &str,
        performance_certification: CausalInspectionPerformanceCertificationBundle,
        bridge_readmission_proof_digest: &str,
        scale_slope_digest: &str,
        anchor_derivation_slope_digest: &str,
        reference_resolution_slope_digest: &str,
        admission_slope_digest: &str,
        bridge_envelope_slope_digest: &str,
        materialization_slope_digest: &str,
        artifact_serialization_slope_digest: &str,
        proof_shape_digest: &str,
        phase_progression_digest: &str,
        witness_authority_digest: &str,
        certification_row_count: usize,
        hostile_row_count: usize,
        representative_row_count: usize,
        scale_fixture_row_count: usize,
        scope_digest: String,
    ) -> Self {
        Self {
            boundary_audit_digest: boundary_audit_digest.to_string(),
            representative_matrix_digest: representative_matrix_digest.to_string(),
            performance_certification,
            bridge_readmission_proof_digest: bridge_readmission_proof_digest.to_string(),
            scale_slope_digest: scale_slope_digest.to_string(),
            anchor_derivation_slope_digest: anchor_derivation_slope_digest.to_string(),
            reference_resolution_slope_digest: reference_resolution_slope_digest.to_string(),
            admission_slope_digest: admission_slope_digest.to_string(),
            bridge_envelope_slope_digest: bridge_envelope_slope_digest.to_string(),
            materialization_slope_digest: materialization_slope_digest.to_string(),
            artifact_serialization_slope_digest: artifact_serialization_slope_digest.to_string(),
            proof_shape_digest: proof_shape_digest.to_string(),
            phase_progression_digest: phase_progression_digest.to_string(),
            witness_authority_digest: witness_authority_digest.to_string(),
            certification_row_count,
            hostile_row_count,
            representative_row_count,
            scale_fixture_row_count,
            scope_digest,
        }
    }

    pub fn scope_digest(&self) -> &str {
        &self.scope_digest
    }

    pub fn certification_row_count(&self) -> usize {
        self.certification_row_count
    }

    pub fn hostile_row_count(&self) -> usize {
        self.hostile_row_count
    }

    pub fn representative_row_count(&self) -> usize {
        self.representative_row_count
    }

    pub fn scale_fixture_row_count(&self) -> usize {
        self.scale_fixture_row_count
    }

    pub fn boundary_audit_digest(&self) -> &str {
        &self.boundary_audit_digest
    }

    pub fn representative_matrix_digest(&self) -> &str {
        &self.representative_matrix_digest
    }

    pub fn proof_shape_digest(&self) -> &str {
        &self.proof_shape_digest
    }

    pub fn bridge_readmission_proof_digest(&self) -> &str {
        &self.bridge_readmission_proof_digest
    }

    pub fn scale_slope_digest(&self) -> &str {
        &self.scale_slope_digest
    }

    pub fn anchor_derivation_slope_digest(&self) -> &str {
        &self.anchor_derivation_slope_digest
    }

    pub fn reference_resolution_slope_digest(&self) -> &str {
        &self.reference_resolution_slope_digest
    }

    pub fn admission_slope_digest(&self) -> &str {
        &self.admission_slope_digest
    }

    pub fn bridge_envelope_slope_digest(&self) -> &str {
        &self.bridge_envelope_slope_digest
    }

    pub fn materialization_slope_digest(&self) -> &str {
        &self.materialization_slope_digest
    }

    pub fn artifact_serialization_slope_digest(&self) -> &str {
        &self.artifact_serialization_slope_digest
    }

    pub fn phase_progression_digest(&self) -> &str {
        &self.phase_progression_digest
    }

    pub fn witness_authority_digest(&self) -> &str {
        &self.witness_authority_digest
    }

    pub fn performance_certification(&self) -> &CausalInspectionPerformanceCertificationBundle {
        &self.performance_certification
    }

    pub(in crate::runtime::inspection::causal::certification) fn into_bundle_parts(
        self,
    ) -> CausalInspectionCertificationBundleParts {
        CausalInspectionCertificationBundleParts {
            certification_scope_digest: self.scope_digest,
            performance_certification_digest: self
                .performance_certification
                .performance_certification_digest()
                .to_string(),
            bridge_readmission_proof_digest: self.bridge_readmission_proof_digest,
            scale_slope_digest: self.scale_slope_digest,
            anchor_derivation_slope_digest: self.anchor_derivation_slope_digest,
            reference_resolution_slope_digest: self.reference_resolution_slope_digest,
            admission_slope_digest: self.admission_slope_digest,
            bridge_envelope_slope_digest: self.bridge_envelope_slope_digest,
            materialization_slope_digest: self.materialization_slope_digest,
            artifact_serialization_slope_digest: self.artifact_serialization_slope_digest,
            boundary_audit_digest: self.boundary_audit_digest,
            representative_matrix_digest: self.representative_matrix_digest,
            proof_shape_digest: self.proof_shape_digest,
            phase_progression_digest: self.phase_progression_digest,
            witness_authority_digest: self.witness_authority_digest,
            certification_row_count: self.certification_row_count,
            hostile_row_count: self.hostile_row_count,
            representative_row_count: self.representative_row_count,
            scale_fixture_row_count: self.scale_fixture_row_count,
        }
    }
}

pub(in crate::runtime::inspection::causal::certification) struct CausalInspectionCertificationBundleParts
{
    pub certification_scope_digest: String,
    pub performance_certification_digest: String,
    pub bridge_readmission_proof_digest: String,
    pub scale_slope_digest: String,
    pub anchor_derivation_slope_digest: String,
    pub reference_resolution_slope_digest: String,
    pub admission_slope_digest: String,
    pub bridge_envelope_slope_digest: String,
    pub materialization_slope_digest: String,
    pub artifact_serialization_slope_digest: String,
    pub boundary_audit_digest: String,
    pub representative_matrix_digest: String,
    pub proof_shape_digest: String,
    pub phase_progression_digest: String,
    pub witness_authority_digest: String,
    pub certification_row_count: usize,
    pub hostile_row_count: usize,
    pub representative_row_count: usize,
    pub scale_fixture_row_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CausalInspectionCertificationBundle {
    certification_bundle_authority: QueryCausalInspectionAuthorityIdentity<
        WorthQueryEvidenceIdentity,
        QueryCausalInspectionIdentityKind,
    >,
    certification_scope_digest: String,
    performance_certification_digest: String,
    bridge_readmission_proof_digest: String,
    scale_slope_digest: String,
    anchor_derivation_slope_digest: String,
    reference_resolution_slope_digest: String,
    admission_slope_digest: String,
    bridge_envelope_slope_digest: String,
    materialization_slope_digest: String,
    artifact_serialization_slope_digest: String,
    boundary_audit_digest: String,
    representative_matrix_digest: String,
    proof_shape_digest: String,
    phase_progression_digest: String,
    witness_authority_digest: String,
    certification_row_count: usize,
    hostile_row_count: usize,
    representative_row_count: usize,
    scale_fixture_row_count: usize,
}

impl CausalInspectionCertificationBundle {
    #[allow(clippy::too_many_arguments)]
    pub(in crate::runtime::inspection::causal::certification) fn from_parts(
        certification_bundle_identity: WorthQueryEvidenceIdentity,
        certification_scope_digest: String,
        performance_certification_digest: String,
        bridge_readmission_proof_digest: String,
        scale_slope_digest: String,
        anchor_derivation_slope_digest: String,
        reference_resolution_slope_digest: String,
        admission_slope_digest: String,
        bridge_envelope_slope_digest: String,
        materialization_slope_digest: String,
        artifact_serialization_slope_digest: String,
        boundary_audit_digest: String,
        representative_matrix_digest: String,
        proof_shape_digest: String,
        phase_progression_digest: String,
        witness_authority_digest: String,
        certification_row_count: usize,
        hostile_row_count: usize,
        representative_row_count: usize,
        scale_fixture_row_count: usize,
    ) -> Self {
        Self {
            certification_bundle_authority: admit_query_causal_inspection_authority_identity(
                certification_bundle_identity,
            ),
            certification_scope_digest,
            performance_certification_digest,
            bridge_readmission_proof_digest,
            scale_slope_digest,
            anchor_derivation_slope_digest,
            reference_resolution_slope_digest,
            admission_slope_digest,
            bridge_envelope_slope_digest,
            materialization_slope_digest,
            artifact_serialization_slope_digest,
            boundary_audit_digest,
            representative_matrix_digest,
            proof_shape_digest,
            phase_progression_digest,
            witness_authority_digest,
            certification_row_count,
            hostile_row_count,
            representative_row_count,
            scale_fixture_row_count,
        }
    }

    pub fn certification_bundle_digest(&self) -> &str {
        self.certification_bundle_authority.value().as_str()
    }

    pub fn certification_bundle_identity(&self) -> &WorthQueryEvidenceIdentity {
        self.certification_bundle_authority.value()
    }

    pub fn certification_bundle_authority(
        &self,
    ) -> &QueryCausalInspectionAuthorityIdentity<
        WorthQueryEvidenceIdentity,
        QueryCausalInspectionIdentityKind,
    > {
        &self.certification_bundle_authority
    }

    pub fn certification_scope_digest(&self) -> &str {
        &self.certification_scope_digest
    }

    pub fn performance_certification_digest(&self) -> &str {
        &self.performance_certification_digest
    }

    pub fn bridge_readmission_proof_digest(&self) -> &str {
        &self.bridge_readmission_proof_digest
    }

    pub fn scale_slope_digest(&self) -> &str {
        &self.scale_slope_digest
    }

    pub fn anchor_derivation_slope_digest(&self) -> &str {
        &self.anchor_derivation_slope_digest
    }

    pub fn reference_resolution_slope_digest(&self) -> &str {
        &self.reference_resolution_slope_digest
    }

    pub fn admission_slope_digest(&self) -> &str {
        &self.admission_slope_digest
    }

    pub fn bridge_envelope_slope_digest(&self) -> &str {
        &self.bridge_envelope_slope_digest
    }

    pub fn materialization_slope_digest(&self) -> &str {
        &self.materialization_slope_digest
    }

    pub fn artifact_serialization_slope_digest(&self) -> &str {
        &self.artifact_serialization_slope_digest
    }

    pub fn boundary_audit_digest(&self) -> &str {
        &self.boundary_audit_digest
    }

    pub fn representative_matrix_digest(&self) -> &str {
        &self.representative_matrix_digest
    }

    pub fn proof_shape_digest(&self) -> &str {
        &self.proof_shape_digest
    }

    pub fn phase_progression_digest(&self) -> &str {
        &self.phase_progression_digest
    }

    pub fn witness_authority_digest(&self) -> &str {
        &self.witness_authority_digest
    }

    pub fn certification_row_count(&self) -> usize {
        self.certification_row_count
    }

    pub fn hostile_row_count(&self) -> usize {
        self.hostile_row_count
    }

    pub fn representative_row_count(&self) -> usize {
        self.representative_row_count
    }

    pub fn scale_fixture_row_count(&self) -> usize {
        self.scale_fixture_row_count
    }
}
