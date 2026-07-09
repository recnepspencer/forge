use crate::{HotnessClassificationVerdict, PlacementDemandSummary, PlacementObservationScopeClass};

pub(crate) fn summarize_window(
    window: &crate::WorkingSetObservationWindow,
) -> PlacementDemandSummary {
    let verdict = match window.scope_class() {
        PlacementObservationScopeClass::Branch => HotnessClassificationVerdict::Hot,
        PlacementObservationScopeClass::RetainedBasis => HotnessClassificationVerdict::Warm,
        PlacementObservationScopeClass::ArtifactFamily => {
            if window.observed_artifact_keys().is_empty() {
                HotnessClassificationVerdict::CoolingDebt
            } else {
                HotnessClassificationVerdict::Warm
            }
        }
    };
    PlacementDemandSummary::new(
        window.scope_class(),
        window.scope_key().to_string(),
        window.observed_artifact_keys().len() as u64,
        verdict,
    )
}
