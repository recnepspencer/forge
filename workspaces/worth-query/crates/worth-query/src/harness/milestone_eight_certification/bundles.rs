use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MilestoneEightCertificationBundle {
    pub query_digest: String,
    pub plan_digest: String,
    pub result_shape_digest: String,
    pub delivery_digest: String,
    pub counter_snapshot_digest: String,
    pub artifact_binding_matrix_digest: String,
    pub support_profile_digest: String,
    pub identity_consumption_digest: String,
    pub inspector_identity_digest: String,
    pub inspector_identity_classification: String,
}

impl MilestoneEightCertificationBundle {
    pub(super) fn has_required_outputs(&self) -> bool {
        !self.query_digest.is_empty()
            && !self.plan_digest.is_empty()
            && !self.result_shape_digest.is_empty()
            && !self.delivery_digest.is_empty()
            && !self.counter_snapshot_digest.is_empty()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MilestoneEightRejectionBundle {
    pub failure_class: MilestoneEightFailureClass,
    pub failure_digest: String,
    pub counter_snapshot_digest: String,
}

pub type MilestoneEightCertificationRow =
    CanonicalCertificationRow<MilestoneEightPerturbationClass, MilestoneEightCertificationBundle>;
pub type MilestoneEightRejectionRow = RejectionCertificationRow<
    MilestoneEightPerturbationClass,
    MilestoneEightCertificationBundle,
    MilestoneEightRejectionBundle,
>;
pub type MilestoneEightCertificationMatrix = CertificationMatrix<
    MilestoneEightPerturbationClass,
    MilestoneEightCertificationBundle,
    MilestoneEightRejectionBundle,
>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MilestoneEightCertificationArtifact {
    pub suite_name: &'static str,
    pub certification_bundle_digest: String,
    pub coverage_matrix_digest: String,
    pub matrix: MilestoneEightCertificationMatrix,
}
