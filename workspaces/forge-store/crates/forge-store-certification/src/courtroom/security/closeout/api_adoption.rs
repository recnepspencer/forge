use forge_foundational::{
    FoundationalBoundaryEvidenceAttachmentTargetKind,
    FoundationalBoundaryEvidenceMaterializationProfile,
};
use forge_store_readiness::PhysicalFoundationEvidenceField;

use super::{
    S51CertificationCloseoutDenial, S51CloseoutBoundaryEvidencePublication,
    S51CloseoutPerformanceReceipts,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S51CloseoutApiAdoptionEvidence {
    native_aspect_values_used: bool,
    canonicalization_used: bool,
    boundary_artifact_used: bool,
    boundary_evidence_used: bool,
    profile_used: bool,
    performance_lane_used: bool,
}

impl S51CloseoutApiAdoptionEvidence {
    pub(crate) fn from_boundary_publication(
        boundary_evidence: &S51CloseoutBoundaryEvidencePublication,
        performance_receipts: &S51CloseoutPerformanceReceipts,
    ) -> Result<Self, S51CertificationCloseoutDenial> {
        let package = boundary_evidence.package();
        let native_aspect_values_used = package.native_aspect_evidence_rows()
            == package.consumed_lower_store_evidence_rows()
            && package.consumed_lower_store_evidence_rows() > 0;
        let canonicalization_used = !package
            .performance_canonical_basis()
            .payload()
            .entries()
            .is_empty()
            && !package
                .boundary_canonical_basis()
                .payload()
                .entries()
                .is_empty()
            && package
                .carries_field(PhysicalFoundationEvidenceField::FoundationalCanonicalBasisBundle)
            && package.receipt_counter_family_count()
                == performance_receipts
                    .counter_backed_receipt()
                    .counter_rows()
                    .len() as u64;
        let boundary_artifact_used = package.attachment_bundle().target_kind()
            == FoundationalBoundaryEvidenceAttachmentTargetKind::BoundaryArtifact
            && package
                .carries_field(PhysicalFoundationEvidenceField::FoundationalProvenanceSupportTruth);
        let boundary_evidence_used = package.attachment_bundle().provenance().is_some()
            && package.attachment_bundle().receipt().is_some()
            && package
                .carries_field(PhysicalFoundationEvidenceField::FoundationalBoundaryEvidenceBundle);
        let profile_used = package.materialized_bundle().materialization_profile()
            == FoundationalBoundaryEvidenceMaterializationProfile::ElideDiagnostics
            && package.carries_field(
                PhysicalFoundationEvidenceField::FoundationalProfileMaterializationPlan,
            );
        let performance_lane_used = package.receipt_counter_family_count() > 0
            && package.carries_field(
                PhysicalFoundationEvidenceField::FoundationalCounterBackedPerformanceReceipt,
            )
            && performance_receipts.all_counter_backed();
        Ok(Self {
            native_aspect_values_used,
            canonicalization_used,
            boundary_artifact_used,
            boundary_evidence_used,
            profile_used,
            performance_lane_used,
        })
    }

    pub const fn native_aspect_values_used(self) -> bool {
        self.native_aspect_values_used
    }

    pub const fn canonicalization_used(self) -> bool {
        self.canonicalization_used
    }

    pub const fn boundary_artifact_used(self) -> bool {
        self.boundary_artifact_used
    }

    pub const fn boundary_evidence_used(self) -> bool {
        self.boundary_evidence_used
    }

    pub const fn profile_used(self) -> bool {
        self.profile_used
    }

    pub const fn performance_lane_used(self) -> bool {
        self.performance_lane_used
    }

    pub const fn uses_required_s5_1_foundational_lanes(self) -> bool {
        self.native_aspect_values_used
            && self.canonicalization_used
            && self.boundary_artifact_used
            && self.boundary_evidence_used
            && self.profile_used
            && self.performance_lane_used
    }
}
