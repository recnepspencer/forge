use crate::commit_strategies::data::StrategyCommitArtifactBundle;
use crate::replay::data::{
    ReplayMismatch, ReplayMismatchClass, ReplayObservableSurface, ReplayVerificationLayer,
};

pub(super) fn strategy_mismatch(
    class: ReplayMismatchClass,
    detail: String,
    expected_artifacts: &StrategyCommitArtifactBundle,
    observed: Option<String>,
) -> ReplayMismatch {
    ReplayMismatch {
        class,
        history_drift_class: None,
        surface: ReplayObservableSurface::Strategy,
        verification_layer: ReplayVerificationLayer::DeepArtifactParity,
        detail,
        expected: Some(format!("{:?}", expected_artifacts.replay_descriptor())),
        observed,
    }
}
