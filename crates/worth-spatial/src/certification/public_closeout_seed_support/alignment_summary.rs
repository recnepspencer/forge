use crate::facade::evidence_lookup_public_closeout::current_evidence_lookup_public_closeout;
use crate::facade::spatial_compiled_product_family::SpatialCompiledProductFamilyIdentity;
use crate::workload_platform::evidence_lookup_public_closeout::EvidenceLookupPublicCloseoutDisposition;
use crate::workload_platform::selected_equivalence_family::current_spatial_selected_equivalence_family_catalog;
use crate::workload_platform::selected_equivalence_family::{
    SpatialFreshnessRequirementPosture, SpatialRenderedOutputComparisonPosture,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialPublicCloseoutSeedSupportError {
    detail: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialPublicCloseoutAlignmentSummary {
    public_closeout_digest: String,
    compiled_product_family_digest: String,
    family_stage_row_digest: String,
    compiled_product_identity_digest: String,
    equivalence_policy_identity_digest: String,
    selected_equivalence_family_identity: String,
    freshness_requirement_posture: SpatialPublicCloseoutFreshnessRequirementPosture,
    rendered_output_comparison_posture: SpatialPublicCloseoutRenderedOutputComparisonPosture,
    receipt_proof_row_count: usize,
    non_ordinary_residue_row_count: usize,
    query_residue_row_count: usize,
    spatial_deletion_row_count: usize,
    spatial_deletion_residue_row_count: usize,
    residue_audit_digest: String,
    query_boundary_support_digest: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialPublicCloseoutFreshnessRequirementPosture {
    SameAdmittedAuthorityAndLocalityRequired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialPublicCloseoutRenderedOutputComparisonPosture {
    NotPartOfBasis,
}

pub fn current_spatial_public_closeout_alignment_summary(
) -> Result<SpatialPublicCloseoutAlignmentSummary, SpatialPublicCloseoutSeedSupportError> {
    let closeout = current_evidence_lookup_public_closeout().map_err(|error| {
        SpatialPublicCloseoutSeedSupportError::new(format!(
            "spatial public closeout seed support failed to load current closeout: {:?}",
            error.kind()
        ))
    })?;
    let first_receipt_row = closeout
        .family_stage_rows()
        .iter()
        .find(|row| {
            matches!(
                row.disposition(),
                EvidenceLookupPublicCloseoutDisposition::ReceiptProof { .. }
            )
        })
        .ok_or_else(|| {
            SpatialPublicCloseoutSeedSupportError::new(
                "spatial public closeout seed support requires at least one receipt-proof row",
            )
        })?;
    let catalog = current_spatial_selected_equivalence_family_catalog();
    let family = catalog
        .family_for_compiled_product(SpatialCompiledProductFamilyIdentity::EvidenceLookupDerivedSupport)
        .ok_or_else(|| {
            SpatialPublicCloseoutSeedSupportError::new(
                "spatial public closeout seed support could not resolve evidence-lookup equivalence family",
            )
        })?;
    Ok(SpatialPublicCloseoutAlignmentSummary {
        public_closeout_digest: closeout.closeout_digest().to_string(),
        compiled_product_family_digest: closeout
            .spatial_compiled_product_family_digest()
            .to_string(),
        family_stage_row_digest: first_receipt_row.row_digest().to_string(),
        compiled_product_identity_digest: first_receipt_row
            .spatial_compiled_product_identity_digest()
            .ok_or_else(|| {
                SpatialPublicCloseoutSeedSupportError::new(
                    "spatial public closeout seed support requires compiled-product identity",
                )
            })?
            .to_string(),
        equivalence_policy_identity_digest: first_receipt_row
            .spatial_equivalence_policy_identity_digest()
            .ok_or_else(|| {
                SpatialPublicCloseoutSeedSupportError::new(
                    "spatial public closeout seed support requires equivalence-policy identity",
                )
            })?
            .to_string(),
        selected_equivalence_family_identity: family.identity().as_str().to_string(),
        freshness_requirement_posture: family.freshness_requirement_posture().into(),
        rendered_output_comparison_posture: family.rendered_output_comparison_posture().into(),
        receipt_proof_row_count: closeout.counters().receipt_proof_row_count(),
        non_ordinary_residue_row_count: closeout.counters().non_ordinary_residue_row_count(),
        query_residue_row_count: closeout.counters().query_residue_row_count(),
        spatial_deletion_row_count: closeout.counters().spatial_deletion_row_count(),
        spatial_deletion_residue_row_count: closeout
            .counters()
            .spatial_deletion_residue_row_count(),
        residue_audit_digest: closeout.residue_audit_digest().to_string(),
        query_boundary_support_digest: closeout.query_boundary_support_digest().to_string(),
    })
}

impl SpatialPublicCloseoutAlignmentSummary {
    pub fn public_closeout_digest(&self) -> &str {
        &self.public_closeout_digest
    }

    pub fn compiled_product_family_digest(&self) -> &str {
        &self.compiled_product_family_digest
    }

    pub fn family_stage_row_digest(&self) -> &str {
        &self.family_stage_row_digest
    }

    pub fn compiled_product_identity_digest(&self) -> &str {
        &self.compiled_product_identity_digest
    }

    pub fn equivalence_policy_identity_digest(&self) -> &str {
        &self.equivalence_policy_identity_digest
    }

    pub fn selected_equivalence_family_identity(&self) -> &str {
        &self.selected_equivalence_family_identity
    }

    pub const fn freshness_requirement_posture(
        &self,
    ) -> SpatialPublicCloseoutFreshnessRequirementPosture {
        self.freshness_requirement_posture
    }

    pub const fn rendered_output_comparison_posture(
        &self,
    ) -> SpatialPublicCloseoutRenderedOutputComparisonPosture {
        self.rendered_output_comparison_posture
    }

    pub const fn receipt_proof_row_count(&self) -> usize {
        self.receipt_proof_row_count
    }

    pub const fn non_ordinary_residue_row_count(&self) -> usize {
        self.non_ordinary_residue_row_count
    }

    pub const fn query_residue_row_count(&self) -> usize {
        self.query_residue_row_count
    }

    pub const fn spatial_deletion_row_count(&self) -> usize {
        self.spatial_deletion_row_count
    }

    pub const fn spatial_deletion_residue_row_count(&self) -> usize {
        self.spatial_deletion_residue_row_count
    }

    pub fn residue_audit_digest(&self) -> &str {
        &self.residue_audit_digest
    }

    pub fn query_boundary_support_digest(&self) -> &str {
        &self.query_boundary_support_digest
    }
}

impl SpatialPublicCloseoutSeedSupportError {
    pub(crate) fn new(detail: impl Into<String>) -> Self {
        Self {
            detail: detail.into(),
        }
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

impl From<SpatialFreshnessRequirementPosture> for SpatialPublicCloseoutFreshnessRequirementPosture {
    fn from(value: SpatialFreshnessRequirementPosture) -> Self {
        match value {
            SpatialFreshnessRequirementPosture::SameAdmittedAuthorityAndLocalityRequired => {
                Self::SameAdmittedAuthorityAndLocalityRequired
            }
        }
    }
}

impl From<SpatialRenderedOutputComparisonPosture>
    for SpatialPublicCloseoutRenderedOutputComparisonPosture
{
    fn from(value: SpatialRenderedOutputComparisonPosture) -> Self {
        match value {
            SpatialRenderedOutputComparisonPosture::NotPartOfBasis => Self::NotPartOfBasis,
        }
    }
}
