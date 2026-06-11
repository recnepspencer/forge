use crate::harness::certification::{digest_parts, CertificationMatrix};

use super::axes::MilestoneNineFivePerturbationClass;
use super::builders;
use super::row::{
    MilestoneNineFiveHostileLaneBundle, MilestoneNineFiveHostileMatrix,
    MilestoneNineFiveHostileRejectionBundle,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MilestoneNineFiveHostileMatrixArtifact {
    pub suite_name: &'static str,
    pub certification_bundle_digest: String,
    pub coverage_matrix_digest: String,
    pub matrix: MilestoneNineFiveHostileMatrix,
}

impl MilestoneNineFiveHostileMatrix {
    pub fn into_milestone_nine_five_artifact(self) -> MilestoneNineFiveHostileMatrixArtifact {
        MilestoneNineFiveHostileMatrixArtifact {
            suite_name: self.suite_name,
            certification_bundle_digest: digest_parts(&builders::bundle_digest_parts(&self)),
            coverage_matrix_digest: digest_parts(&builders::coverage_digest_parts(&self)),
            matrix: self,
        }
    }
}

pub struct MilestoneNineFiveHostileMatrixAdapter;

impl MilestoneNineFiveHostileMatrixAdapter {
    pub fn debt_close_hostile_certification_matrix_artifact(
    ) -> MilestoneNineFiveHostileMatrixArtifact {
        Self::debt_close_hostile_certification_matrix_test().into_milestone_nine_five_artifact()
    }

    pub fn debt_close_hostile_certification_matrix_test() -> MilestoneNineFiveHostileMatrix {
        CertificationMatrix::<
            MilestoneNineFivePerturbationClass,
            MilestoneNineFiveHostileLaneBundle,
            MilestoneNineFiveHostileRejectionBundle,
        > {
            suite_name: "Milestone 9.5 Debt-Close Hostile Certification Matrix",
            rows: builders::canonical_rows(),
            rejection_rows: builders::rejection_rows(),
        }
    }
}
