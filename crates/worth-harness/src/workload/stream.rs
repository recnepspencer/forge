use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::timeline::{ExecutionPhase, FeedBatch};

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeedStreamSample {
    pub feed_name: String,
    pub sequence: u64,
    pub value_microunits: i64,
    pub delta_microunits: i64,
    pub event_kind: FeedStreamEventKind,
    pub regime: FeedVolatilityRegime,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeedStreamBatch {
    pub feed_name: String,
    pub phase: Option<ExecutionPhase>,
    pub sequence_start: u64,
    pub sequence_end: u64,
    pub samples: Vec<FeedStreamSample>,
    pub metadata: BTreeMap<String, String>,
}

impl FeedStreamBatch {
    pub fn as_feed_batch(&self) -> FeedBatch {
        let mut feed_batch = FeedBatch::new(
            self.feed_name.clone(),
            self.sequence_start,
            self.sequence_end,
        );
        if let Some(phase) = self.phase {
            feed_batch = feed_batch.with_phase(phase);
        }
        feed_batch.metadata.extend(self.metadata.clone());
        feed_batch
            .metadata
            .insert("sample_count".to_owned(), self.samples.len().to_string());
        if let Some(last) = self.samples.last() {
            feed_batch.metadata.insert(
                "last_value_microunits".to_owned(),
                last.value_microunits.to_string(),
            );
            feed_batch.metadata.insert(
                "last_event_kind".to_owned(),
                format!("{:?}", last.event_kind),
            );
            feed_batch
                .metadata
                .insert("last_regime".to_owned(), format!("{:?}", last.regime));
        }
        feed_batch
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeterministicFeedStreamGenerator {
    profile: FeedStreamProfile,
    seed: u64,
    current_value_microunits: i64,
    current_trend_microunits: i64,
    current_factor_microunits: i64,
    current_regime: FeedVolatilityRegime,
    next_sequence: u64,
}

impl DeterministicFeedStreamGenerator {
    pub fn new(profile: FeedStreamProfile, seed: u64) -> Self {
        Self {
            current_value_microunits: profile.starting_value_microunits,
            current_trend_microunits: 0,
            current_factor_microunits: 0,
            current_regime: FeedVolatilityRegime::Normal,
            next_sequence: 1,
            profile,
            seed: seed.max(1),
        }
    }

    pub fn profile(&self) -> &FeedStreamProfile {
        &self.profile
    }

    pub fn current_value_microunits(&self) -> i64 {
        self.current_value_microunits
    }

    pub fn current_regime(&self) -> FeedVolatilityRegime {
        self.current_regime
    }

    pub fn next_sample(&mut self) -> FeedStreamSample {
        self.next_sample_with_external_factor(0)
    }

    pub fn next_sample_with_external_factor(
        &mut self,
        external_factor_microunits: i64,
    ) -> FeedStreamSample {
        let previous_value = self.current_value_microunits;
        let previous_regime = self.current_regime;
        self.advance_regime();
        let regime_changed = self.current_regime != previous_regime;

        self.advance_factor_state();
        self.advance_trend_state();

        let regime_volatility_multiplier = self.regime_volatility_multiplier_per_mille();
        let idiosyncratic_noise = self.next_gaussian_delta(
            self.profile.stability_band_microunits * regime_volatility_multiplier / 1000,
        );
        let mean_reversion_delta = self.compute_mean_reversion_delta();
        let external_factor_delta =
            external_factor_microunits * self.profile.factor_loading_per_mille as i64 / 1000;
        let factor_delta = self.current_factor_microunits;
        let trend_delta = self.current_trend_microunits;

        let (jump_delta, jump_kind) = self.next_jump_delta(regime_changed);
        let event_kind = jump_kind.unwrap_or_else(|| {
            let directional_magnitude =
                trend_delta.abs() + factor_delta.abs() / 2 + external_factor_delta.abs();
            if directional_magnitude > self.profile.drift_step_microunits.max(0) / 2 {
                FeedStreamEventKind::Drift
            } else if idiosyncratic_noise != 0 {
                FeedStreamEventKind::Noise
            } else {
                FeedStreamEventKind::Stable
            }
        });

        let next_value = self
            .current_value_microunits
            .saturating_add(mean_reversion_delta)
            .saturating_add(idiosyncratic_noise)
            .saturating_add(trend_delta)
            .saturating_add(factor_delta)
            .saturating_add(external_factor_delta)
            .saturating_add(jump_delta)
            .max(1);
        self.current_value_microunits = next_value;

        let mut metadata = BTreeMap::new();
        metadata.insert(
            "external_factor_microunits".to_owned(),
            external_factor_microunits.to_string(),
        );
        metadata.insert(
            "factor_delta_microunits".to_owned(),
            factor_delta.to_string(),
        );
        metadata.insert("trend_delta_microunits".to_owned(), trend_delta.to_string());
        metadata.insert(
            "mean_reversion_delta_microunits".to_owned(),
            mean_reversion_delta.to_string(),
        );
        metadata.insert(
            "idiosyncratic_noise_microunits".to_owned(),
            idiosyncratic_noise.to_string(),
        );
        metadata.insert("jump_delta_microunits".to_owned(), jump_delta.to_string());

        let sample = FeedStreamSample {
            feed_name: self.profile.feed_name.clone(),
            sequence: self.next_sequence,
            value_microunits: self.current_value_microunits,
            delta_microunits: self.current_value_microunits - previous_value,
            event_kind,
            regime: self.current_regime,
            metadata,
        };
        self.next_sequence += 1;
        sample
    }

    pub fn next_batch(&mut self, sample_count: usize) -> FeedStreamBatch {
        self.next_batch_with_external_factor(sample_count, 0)
    }

    pub fn next_batch_with_external_factor(
        &mut self,
        sample_count: usize,
        external_factor_microunits: i64,
    ) -> FeedStreamBatch {
        let mut samples = Vec::with_capacity(sample_count);
        for _ in 0..sample_count {
            samples.push(self.next_sample_with_external_factor(external_factor_microunits));
        }
        let (sequence_start, sequence_end) = match (samples.first(), samples.last()) {
            (Some(first), Some(last)) => (first.sequence, last.sequence),
            _ => (self.next_sequence, self.next_sequence),
        };

        let mut metadata = self.profile.metadata.clone();
        metadata.insert("seed".to_owned(), self.seed.to_string());
        metadata.insert(
            "ending_value_microunits".to_owned(),
            self.current_value_microunits.to_string(),
        );
        metadata.insert(
            "external_factor_microunits".to_owned(),
            external_factor_microunits.to_string(),
        );
        metadata.insert(
            "ending_regime".to_owned(),
            format!("{:?}", self.current_regime),
        );

        FeedStreamBatch {
            feed_name: self.profile.feed_name.clone(),
            phase: self.profile.phase,
            sequence_start,
            sequence_end,
            samples,
            metadata,
        }
    }

    fn advance_regime(&mut self) {
        let persistence_roll = self.next_u64() % 1000;
        if persistence_roll < self.profile.regime_persistence_per_mille as u64 {
            return;
        }

        let roll = self.next_u64() % 1000;
        let calm = self.profile.calm_regime_probability_per_mille as u64;
        let volatile = self.profile.volatile_regime_probability_per_mille as u64;
        let stressed = self.profile.stressed_regime_probability_per_mille as u64;

        self.current_regime = if roll < calm {
            FeedVolatilityRegime::Calm
        } else if roll < calm + volatile {
            FeedVolatilityRegime::Volatile
        } else if roll < calm + volatile + stressed {
            FeedVolatilityRegime::Stressed
        } else {
            FeedVolatilityRegime::Normal
        };
    }

    fn advance_factor_state(&mut self) {
        let persistence = self.profile.factor_persistence_per_mille as i64;
        let retained = self.current_factor_microunits * persistence / 1000;
        let innovation = self.next_gaussian_delta(
            self.profile.factor_band_microunits * self.regime_volatility_multiplier_per_mille()
                / 1000,
        );
        self.current_factor_microunits = retained.saturating_add(innovation);
    }

    fn advance_trend_state(&mut self) {
        let retained = self.current_trend_microunits * 850 / 1000;
        let innovation = self.next_gaussian_delta(
            self.profile.drift_step_microunits * self.regime_volatility_multiplier_per_mille()
                / 1000,
        );
        self.current_trend_microunits = retained.saturating_add(innovation);
    }

    fn next_jump_delta(&mut self, regime_changed: bool) -> (i64, Option<FeedStreamEventKind>) {
        if regime_changed {
            let jump = self.scaled_shift_delta(
                self.profile.regime_shift_range,
                self.regime_jump_multiplier_per_mille(),
            );
            return (jump, Some(FeedStreamEventKind::RegimeShift));
        }

        let jump_multiplier = self.regime_jump_multiplier_per_mille();
        let roll = self.next_u64() % 1000;
        let major_probability =
            (self.profile.major_shift_probability_per_mille as u64 * jump_multiplier as u64 / 1000)
                .min(1000);
        let minor_probability =
            (self.profile.minor_shift_probability_per_mille as u64 * jump_multiplier as u64 / 1000)
                .min(1000);

        if roll < major_probability {
            (
                self.scaled_shift_delta(self.profile.major_shift_range, jump_multiplier),
                Some(FeedStreamEventKind::MajorShift),
            )
        } else if roll < major_probability + minor_probability {
            (
                self.scaled_shift_delta(self.profile.minor_shift_range, jump_multiplier),
                Some(FeedStreamEventKind::MinorShift),
            )
        } else {
            (0, None)
        }
    }

    fn scaled_shift_delta(&mut self, range: FeedShiftRange, multiplier_per_mille: i64) -> i64 {
        let scaled_min = range.min_delta_microunits * multiplier_per_mille / 1000;
        let scaled_max = range.max_delta_microunits * multiplier_per_mille / 1000;
        let span = (scaled_max - scaled_min).max(0);
        let magnitude = scaled_min + (self.next_u64() % (span as u64 + 1)) as i64;
        if self.next_bool() {
            magnitude
        } else {
            -magnitude
        }
    }

    fn compute_mean_reversion_delta(&self) -> i64 {
        if self.profile.mean_reversion_per_mille == 0 {
            return 0;
        }

        let distance_from_baseline =
            self.profile.baseline_value_microunits - self.current_value_microunits;
        distance_from_baseline * self.profile.mean_reversion_per_mille as i64 / 1000
    }

    fn regime_volatility_multiplier_per_mille(&self) -> i64 {
        match self.current_regime {
            FeedVolatilityRegime::Calm => 650,
            FeedVolatilityRegime::Normal => 1000,
            FeedVolatilityRegime::Volatile => 1600,
            FeedVolatilityRegime::Stressed => 2400,
        }
    }

    fn regime_jump_multiplier_per_mille(&self) -> i64 {
        match self.current_regime {
            FeedVolatilityRegime::Calm => 500,
            FeedVolatilityRegime::Normal => 1000,
            FeedVolatilityRegime::Volatile => 1750,
            FeedVolatilityRegime::Stressed => 2600,
        }
    }

    fn next_gaussian_delta(&mut self, band_microunits: i64) -> i64 {
        if band_microunits <= 0 {
            return 0;
        }

        // Sum-of-uniforms gives a deterministic, dependency-free bell shape that is
        // materially more realistic than a flat bounded jitter for workload feeds.
        let mut total = 0i64;
        for _ in 0..6 {
            total += (self.next_u64() % 1001) as i64;
        }
        let centered_per_mille = total - 3000;
        centered_per_mille * band_microunits / 1000
    }

    fn next_bool(&mut self) -> bool {
        self.next_u64() & 1 == 0
    }

    fn next_u64(&mut self) -> u64 {
        // Xorshift64* keeps feed generation deterministic and dependency-free.
        let mut value = self.seed;
        value ^= value >> 12;
        value ^= value << 25;
        value ^= value >> 27;
        self.seed = value;
        value.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
}

#[cfg(test)]
mod tests {
    use crate::timeline::ExecutionPhase;

    use super::{
        DeterministicFeedStreamGenerator, FeedShiftRange, FeedStreamEventKind, FeedStreamProfile,
        FeedVolatilityRegime,
    };

    #[test]
    fn deterministic_feed_stream_generator_replays_identically_for_same_seed() {
        let profile = FeedStreamProfile::new("steel", 100_000)
            .with_phase(ExecutionPhase::Ingest)
            .with_stability_band(500)
            .with_drift_step(150)
            .with_mean_reversion_per_mille(200)
            .with_factor_process(200, 880, 350)
            .with_regime_process(920, 90, 120, 40)
            .with_shift_probabilities(30, 10, 2)
            .with_shift_ranges(
                FeedShiftRange::new(800, 2_000),
                FeedShiftRange::new(3_000, 8_000),
                FeedShiftRange::new(10_000, 20_000),
            );
        let mut left = DeterministicFeedStreamGenerator::new(profile.clone(), 42);
        let mut right = DeterministicFeedStreamGenerator::new(profile, 42);

        let left_batch = left.next_batch_with_external_factor(16, 175);
        let right_batch = right.next_batch_with_external_factor(16, 175);

        assert_eq!(left_batch, right_batch);
        assert_eq!(left_batch.sequence_start, 1);
        assert_eq!(left_batch.sequence_end, 16);
        assert_eq!(
            left_batch.as_feed_batch().phase,
            Some(ExecutionPhase::Ingest)
        );
    }

    #[test]
    fn deterministic_feed_stream_generator_emits_shift_events_under_nonzero_probabilities() {
        let profile = FeedStreamProfile::new("fuel", 50_000)
            .with_stability_band(250)
            .with_drift_step(80)
            .with_factor_process(240, 900, 0)
            .with_regime_process(800, 60, 180, 120)
            .with_shift_probabilities(120, 80, 30)
            .with_shift_ranges(
                FeedShiftRange::new(400, 900),
                FeedShiftRange::new(1_200, 2_500),
                FeedShiftRange::new(3_000, 5_000),
            );
        let mut generator = DeterministicFeedStreamGenerator::new(profile, 7);
        let batch = generator.next_batch(96);

        assert!(batch.samples.iter().any(|sample| {
            matches!(
                sample.event_kind,
                FeedStreamEventKind::MinorShift
                    | FeedStreamEventKind::MajorShift
                    | FeedStreamEventKind::RegimeShift
            )
        }));
        assert!(batch
            .samples
            .iter()
            .any(|sample| sample.regime != FeedVolatilityRegime::Normal));
    }

    #[test]
    fn deterministic_feed_stream_generator_mean_reverts_toward_baseline() {
        let profile = FeedStreamProfile::new("rubber", 100_000)
            .with_starting_value(130_000)
            .with_mean_reversion_per_mille(400);
        let mut generator = DeterministicFeedStreamGenerator::new(profile, 99);

        let first = generator.next_sample();
        let second = generator.next_sample();

        assert!(first.value_microunits < 130_000);
        assert!(second.value_microunits <= first.value_microunits);
    }

    #[test]
    fn deterministic_feed_stream_generator_external_factor_can_move_shared_feeds_together() {
        let profile = FeedStreamProfile::new("aluminum", 62_000)
            .with_stability_band(100)
            .with_drift_step(40)
            .with_factor_process(60, 950, 600)
            .with_mean_reversion_per_mille(80);
        let mut generator = DeterministicFeedStreamGenerator::new(profile, 321);

        let calm = generator.next_sample_with_external_factor(0);
        let stressed = generator.next_sample_with_external_factor(2_000);

        assert!(stressed.value_microunits >= calm.value_microunits);
        assert_eq!(
            stressed
                .metadata
                .get("external_factor_microunits")
                .map(String::as_str),
            Some("2000")
        );
    }
}
