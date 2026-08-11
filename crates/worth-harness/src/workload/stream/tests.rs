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
