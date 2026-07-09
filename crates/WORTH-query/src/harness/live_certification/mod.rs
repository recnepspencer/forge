mod bundles;
mod canonical_rows;
mod model;
mod rejection_rows;
mod tests;

pub(crate) use model::{
    LiveBundleFamily, LiveCertificationBundle, LiveCertificationMatrix, LiveCertificationRow,
    LiveFailureClass, LiveHostileExpectation, LiveOutcomeKind, LivePerturbationClass,
    LiveRejectionBundle, LiveRejectionRow, MilestoneFiveLiveCertificationArtifact,
};

pub struct MilestoneFiveLiveCertificationAdapter;

impl MilestoneFiveLiveCertificationAdapter {
    pub fn live_promotion_convergence_and_suppression_certification_artifact(
    ) -> MilestoneFiveLiveCertificationArtifact {
        Self::live_promotion_convergence_and_suppression_test().into_milestone_five_artifact()
    }

    pub fn live_promotion_convergence_and_suppression_test() -> LiveCertificationMatrix {
        LiveCertificationMatrix {
            suite_name: "Live Promotion Convergence And Suppression Test",
            rows: canonical_rows::canonical_rows(),
            rejection_rows: rejection_rows::rejection_rows(),
        }
    }
}
