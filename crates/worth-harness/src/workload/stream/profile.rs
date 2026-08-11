use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::timeline::ExecutionPhase;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FeedStreamEventKind {
    Stable,
    Noise,
    Drift,
    MinorShift,
    MajorShift,
    RegimeShift,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FeedVolatilityRegime {
    Calm,
    Normal,
    Volatile,
    Stressed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeedShiftRange {
    pub min_delta_microunits: i64,
    pub max_delta_microunits: i64,
}

impl FeedShiftRange {
    pub fn new(min_delta_microunits: i64, max_delta_microunits: i64) -> Self {
        assert!(
            min_delta_microunits <= max_delta_microunits,
            "shift range min must not exceed max"
        );
        Self {
            min_delta_microunits,
            max_delta_microunits,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeedStreamProfile {
    pub feed_name: String,
    pub phase: Option<ExecutionPhase>,
    pub baseline_value_microunits: i64,
    pub starting_value_microunits: i64,
    pub stability_band_microunits: i64,
    pub drift_step_microunits: i64,
    pub mean_reversion_per_mille: u16,
    pub factor_band_microunits: i64,
    pub factor_persistence_per_mille: u16,
    pub factor_loading_per_mille: i16,
    pub regime_persistence_per_mille: u16,
    pub calm_regime_probability_per_mille: u16,
    pub volatile_regime_probability_per_mille: u16,
    pub stressed_regime_probability_per_mille: u16,
    pub minor_shift_probability_per_mille: u16,
    pub major_shift_probability_per_mille: u16,
    pub regime_shift_probability_per_mille: u16,
    pub minor_shift_range: FeedShiftRange,
    pub major_shift_range: FeedShiftRange,
    pub regime_shift_range: FeedShiftRange,
    pub metadata: BTreeMap<String, String>,
}

impl FeedStreamProfile {
    pub fn new(feed_name: impl Into<String>, baseline_value_microunits: i64) -> Self {
        Self {
            feed_name: feed_name.into(),
            phase: None,
            baseline_value_microunits,
            starting_value_microunits: baseline_value_microunits,
            stability_band_microunits: 0,
            drift_step_microunits: 0,
            mean_reversion_per_mille: 0,
            factor_band_microunits: 0,
            factor_persistence_per_mille: 0,
            factor_loading_per_mille: 0,
            regime_persistence_per_mille: 950,
            calm_regime_probability_per_mille: 50,
            volatile_regime_probability_per_mille: 50,
            stressed_regime_probability_per_mille: 10,
            minor_shift_probability_per_mille: 0,
            major_shift_probability_per_mille: 0,
            regime_shift_probability_per_mille: 0,
            minor_shift_range: FeedShiftRange::new(0, 0),
            major_shift_range: FeedShiftRange::new(0, 0),
            regime_shift_range: FeedShiftRange::new(0, 0),
            metadata: BTreeMap::new(),
        }
    }

    pub fn with_phase(mut self, phase: ExecutionPhase) -> Self {
        self.phase = Some(phase);
        self
    }

    pub fn with_starting_value(mut self, starting_value_microunits: i64) -> Self {
        self.starting_value_microunits = starting_value_microunits;
        self
    }

    pub fn with_stability_band(mut self, stability_band_microunits: i64) -> Self {
        self.stability_band_microunits = stability_band_microunits.max(0);
        self
    }

    pub fn with_drift_step(mut self, drift_step_microunits: i64) -> Self {
        self.drift_step_microunits = drift_step_microunits.max(0);
        self
    }

    pub fn with_mean_reversion_per_mille(mut self, mean_reversion_per_mille: u16) -> Self {
        self.mean_reversion_per_mille = mean_reversion_per_mille.min(1000);
        self
    }

    pub fn with_factor_process(
        mut self,
        factor_band_microunits: i64,
        factor_persistence_per_mille: u16,
        factor_loading_per_mille: i16,
    ) -> Self {
        self.factor_band_microunits = factor_band_microunits.max(0);
        self.factor_persistence_per_mille = factor_persistence_per_mille.min(1000);
        self.factor_loading_per_mille = factor_loading_per_mille.clamp(-1000, 1000);
        self
    }

    pub fn with_regime_process(
        mut self,
        regime_persistence_per_mille: u16,
        calm_regime_probability_per_mille: u16,
        volatile_regime_probability_per_mille: u16,
        stressed_regime_probability_per_mille: u16,
    ) -> Self {
        self.regime_persistence_per_mille = regime_persistence_per_mille.min(1000);
        self.calm_regime_probability_per_mille = calm_regime_probability_per_mille.min(1000);
        self.volatile_regime_probability_per_mille =
            volatile_regime_probability_per_mille.min(1000);
        self.stressed_regime_probability_per_mille =
            stressed_regime_probability_per_mille.min(1000);
        self
    }

    pub fn with_shift_probabilities(
        mut self,
        minor_shift_probability_per_mille: u16,
        major_shift_probability_per_mille: u16,
        regime_shift_probability_per_mille: u16,
    ) -> Self {
        self.minor_shift_probability_per_mille = minor_shift_probability_per_mille.min(1000);
        self.major_shift_probability_per_mille = major_shift_probability_per_mille.min(1000);
        self.regime_shift_probability_per_mille = regime_shift_probability_per_mille.min(1000);
        self
    }

    pub fn with_shift_ranges(
        mut self,
        minor_shift_range: FeedShiftRange,
        major_shift_range: FeedShiftRange,
        regime_shift_range: FeedShiftRange,
    ) -> Self {
        self.minor_shift_range = minor_shift_range;
        self.major_shift_range = major_shift_range;
        self.regime_shift_range = regime_shift_range;
        self
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}
