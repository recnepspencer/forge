mod bundles;
mod canonical_rows;
mod fixtures;
mod rejection_rows;
mod tests;

use super::validation_matrix::{
    MilestoneTwoValidationCertificationArtifact, ValidationCertificationMatrix,
};

pub struct MilestoneTwoValidationCertificationAdapter;

impl MilestoneTwoValidationCertificationAdapter {
    pub fn schema_aware_rejection_and_projection_legality_certification_artifact(
    ) -> MilestoneTwoValidationCertificationArtifact {
        Self::schema_aware_rejection_and_projection_legality_test().into_milestone_two_artifact()
    }

    pub fn schema_aware_rejection_and_projection_legality_test() -> ValidationCertificationMatrix {
        ValidationCertificationMatrix {
            suite_name: "Schema-Aware Rejection And Projection Legality Test",
            rows: canonical_rows::canonical_rows(),
            rejection_rows: rejection_rows::rejection_rows(),
        }
    }
}
