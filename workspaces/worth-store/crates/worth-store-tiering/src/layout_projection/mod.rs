mod cold_recall;
mod placement;
mod recall_amplification;
#[cfg(all(test, feature = "certification-test-authority"))]
mod tests;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TierLayoutTraversal {
    BoundedScan,
}

pub use cold_recall::{
    ColdRecallAccessBudget, ColdRecallInterferencePosture, ColdRecallLayoutReport,
};
pub use placement::{
    TierPlacementAccessBudget, TierPlacementInterferencePosture, TierPlacementLayoutReport,
};
pub use recall_amplification::{
    RecallAmplificationAccessBudget, RecallAmplificationInterferencePosture,
    RecallAmplificationLayoutReport,
};
