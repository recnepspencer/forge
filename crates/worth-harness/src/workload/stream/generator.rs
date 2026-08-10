use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::profile::{
    FeedShiftRange, FeedStreamEventKind, FeedStreamProfile, FeedVolatilityRegime,
};
use super::sample::{FeedStreamBatch, FeedStreamSample};

#[derive(Debug, Clone, Copy)]
struct FeedSampleDeltas {
    external_factor_delta: i64,
    factor_delta: i64,
    trend_delta: i64,
    mean_reversion_delta: i64,
    idiosyncratic_noise: i64,
    jump_delta: i64,
    jump_kind: Option<FeedStreamEventKind>,
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
        let regime_changed = self.advance_sample_state();
        let deltas = self.calculate_sample_deltas(external_factor_microunits, regime_changed);
        let event_kind = self.classify_sample_event(&deltas);
        self.apply_sample_value(&deltas);
        self.build_sample(
            previous_value,
            external_factor_microunits,
            &deltas,
            event_kind,
        )
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

    fn advance_sample_state(&mut self) -> bool {
        let previous_regime = self.current_regime;
        self.advance_regime();
        let regime_changed = self.current_regime != previous_regime;

        self.advance_factor_state();
        self.advance_trend_state();
        regime_changed
    }

    fn calculate_sample_deltas(
        &mut self,
        external_factor_microunits: i64,
        regime_changed: bool,
    ) -> FeedSampleDeltas {
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
        FeedSampleDeltas {
            external_factor_delta,
            factor_delta,
            trend_delta,
            mean_reversion_delta,
            idiosyncratic_noise,
            jump_delta,
            jump_kind,
        }
    }

    fn classify_sample_event(&self, deltas: &FeedSampleDeltas) -> FeedStreamEventKind {
        deltas.jump_kind.unwrap_or_else(|| {
            let directional_magnitude = deltas.trend_delta.abs()
                + deltas.factor_delta.abs() / 2
                + deltas.external_factor_delta.abs();
            if directional_magnitude > self.profile.drift_step_microunits.max(0) / 2 {
                FeedStreamEventKind::Drift
            } else if deltas.idiosyncratic_noise != 0 {
                FeedStreamEventKind::Noise
            } else {
                FeedStreamEventKind::Stable
            }
        })
    }

    fn apply_sample_value(&mut self, deltas: &FeedSampleDeltas) {
        let next_value = self
            .current_value_microunits
            .saturating_add(deltas.mean_reversion_delta)
            .saturating_add(deltas.idiosyncratic_noise)
            .saturating_add(deltas.trend_delta)
            .saturating_add(deltas.factor_delta)
            .saturating_add(deltas.external_factor_delta)
            .saturating_add(deltas.jump_delta)
            .max(1);
        self.current_value_microunits = next_value;
    }

    fn build_sample(
        &mut self,
        previous_value: i64,
        external_factor_microunits: i64,
        deltas: &FeedSampleDeltas,
        event_kind: FeedStreamEventKind,
    ) -> FeedStreamSample {
        let mut metadata = BTreeMap::new();
        metadata.insert(
            "external_factor_microunits".to_owned(),
            external_factor_microunits.to_string(),
        );
        metadata.insert(
            "factor_delta_microunits".to_owned(),
            deltas.factor_delta.to_string(),
        );
        metadata.insert(
            "trend_delta_microunits".to_owned(),
            deltas.trend_delta.to_string(),
        );
        metadata.insert(
            "mean_reversion_delta_microunits".to_owned(),
            deltas.mean_reversion_delta.to_string(),
        );
        metadata.insert(
            "idiosyncratic_noise_microunits".to_owned(),
            deltas.idiosyncratic_noise.to_string(),
        );
        metadata.insert(
            "jump_delta_microunits".to_owned(),
            deltas.jump_delta.to_string(),
        );

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
