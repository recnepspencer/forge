mod contradiction_basis;
mod counters;
mod denial;
mod duplicate_key;
mod grouping;
mod identity;
mod normalization;
mod normalized_cut;
mod normalized_cut_builder;
mod retained_interval_entry;
#[cfg(test)]
mod tests;
#[cfg(test)]
pub(crate) mod tests_support;

pub use counters::PlanarBooleanNormalizedEdgeSplitScheduleCounters;
pub use denial::{
    PlanarBooleanDuplicateSplitNormalizationDenial,
    PlanarBooleanDuplicateSplitNormalizationDenialKind,
};
#[cfg(test)]
pub(crate) use normalized_cut::PlanarBooleanNormalizedEndpointAuthority;
pub use normalized_cut::{
    PlanarBooleanNormalizedEdgeSplitSchedule, PlanarBooleanNormalizedEdgeSplitScheduleSet,
    PlanarBooleanNormalizedSplitCut,
};
pub use retained_interval_entry::PlanarBooleanRetainedIntervalSplitEntry;
