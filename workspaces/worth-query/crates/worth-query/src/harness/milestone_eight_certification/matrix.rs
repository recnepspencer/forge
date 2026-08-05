use super::*;

impl MilestoneEightCertificationMatrix {
    pub fn into_milestone_eight_artifact(self) -> MilestoneEightCertificationArtifact {
        let certification_bundle_digest = digest_parts(&bundle_digest_parts(&self));
        let coverage_matrix_digest = digest_parts(&coverage_digest_parts(&self));
        MilestoneEightCertificationArtifact {
            suite_name: self.suite_name,
            certification_bundle_digest,
            coverage_matrix_digest,
            matrix: self,
        }
    }
}

pub struct MilestoneEightCertificationAdapter;

impl MilestoneEightCertificationAdapter {
    pub fn scope_template_view_shape_semantic_parity_certification_artifact(
    ) -> MilestoneEightCertificationArtifact {
        Self::scope_template_view_shape_semantic_parity_test().into_milestone_eight_artifact()
    }

    pub fn scope_template_view_shape_semantic_parity_test() -> MilestoneEightCertificationMatrix {
        MilestoneEightCertificationMatrix {
            suite_name: "Scope / Template / View-Shape Semantic Parity Test",
            rows: canonical_rows(),
            rejection_rows: rejection_rows(),
        }
    }
}
