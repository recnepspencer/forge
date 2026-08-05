use crate::harness::certification::digest_parts;
use crate::harness::milestone_nine_certification::bundles::MilestoneNineCertificationArtifact;
use crate::harness::milestone_nine_certification::digests::bundle_digest_parts;
use crate::harness::milestone_nine_certification::digests::coverage_digest_parts;
use crate::harness::milestone_nine_certification::phase_four_support::MilestoneNinePhaseFourSupportReport;
use crate::harness::milestone_nine_certification::rows::canonical_rows;
use crate::harness::milestone_nine_certification::rows::rejection_rows;

impl MilestoneNineCertificationMatrix {
    pub fn into_milestone_nine_artifact(self) -> MilestoneNineCertificationArtifact {
        let certification_bundle_digest = digest_parts(&bundle_digest_parts(&self));
        let coverage_matrix_digest = digest_parts(&coverage_digest_parts(&self));
        MilestoneNineCertificationArtifact {
            suite_name: self.suite_name,
            certification_bundle_digest,
            coverage_matrix_digest,
            matrix: self,
        }
    }

    pub fn phase_four_support_report(&self) -> MilestoneNinePhaseFourSupportReport {
        MilestoneNinePhaseFourSupportReport::new(self)
    }
}

pub struct MilestoneNineCertificationAdapter;

impl MilestoneNineCertificationAdapter {
    pub fn policy_tenant_context_admission_certification_artifact(
    ) -> MilestoneNineCertificationArtifact {
        Self::policy_tenant_context_admission_test().into_milestone_nine_artifact()
    }

    pub fn policy_tenant_context_admission_test() -> MilestoneNineCertificationMatrix {
        MilestoneNineCertificationMatrix {
            suite_name: "Policy And Tenant Context Admission Test",
            rows: canonical_rows(),
            rejection_rows: rejection_rows(),
        }
    }
}
use crate::harness::milestone_nine_certification::bundles::MilestoneNineCertificationMatrix;
