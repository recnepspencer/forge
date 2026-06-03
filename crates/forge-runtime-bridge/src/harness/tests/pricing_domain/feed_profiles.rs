use std::collections::BTreeMap;

use forge_harness::facade::{ExecutionPhase, FeedShiftRange, FeedStreamProfile};

use super::{PricingDomainWorld, PricingMaterial};

impl PricingDomainWorld {
    pub(super) fn reference_stream_profiles() -> BTreeMap<PricingMaterial, FeedStreamProfile> {
        BTreeMap::from([
            (
                PricingMaterial::Steel,
                FeedStreamProfile::new("material:steel", 100_000)
                    .with_phase(ExecutionPhase::Ingest)
                    .with_stability_band(450)
                    .with_drift_step(120)
                    .with_mean_reversion_per_mille(180)
                    .with_factor_process(240, 930, 650)
                    .with_regime_process(930, 80, 120, 40)
                    .with_shift_probabilities(40, 10, 1)
                    .with_shift_ranges(
                        FeedShiftRange::new(800, 1_600),
                        FeedShiftRange::new(3_000, 6_000),
                        FeedShiftRange::new(8_000, 15_000),
                    ),
            ),
            (
                PricingMaterial::Rubber,
                FeedStreamProfile::new("material:rubber", 40_000)
                    .with_phase(ExecutionPhase::Ingest)
                    .with_stability_band(350)
                    .with_drift_step(90)
                    .with_mean_reversion_per_mille(140)
                    .with_factor_process(220, 915, 500)
                    .with_regime_process(910, 70, 130, 55)
                    .with_shift_probabilities(25, 12, 2)
                    .with_shift_ranges(
                        FeedShiftRange::new(600, 1_400),
                        FeedShiftRange::new(4_000, 8_000),
                        FeedShiftRange::new(12_000, 22_000),
                    ),
            ),
            (
                PricingMaterial::Fuel,
                FeedStreamProfile::new("material:fuel", 15_000)
                    .with_phase(ExecutionPhase::Ingest)
                    .with_stability_band(220)
                    .with_drift_step(75)
                    .with_mean_reversion_per_mille(110)
                    .with_factor_process(260, 940, 800)
                    .with_regime_process(905, 55, 160, 90)
                    .with_shift_probabilities(45, 18, 3)
                    .with_shift_ranges(
                        FeedShiftRange::new(500, 1_100),
                        FeedShiftRange::new(2_000, 4_500),
                        FeedShiftRange::new(5_000, 9_000),
                    ),
            ),
            (
                PricingMaterial::Aluminum,
                FeedStreamProfile::new("material:aluminum", 62_000)
                    .with_phase(ExecutionPhase::Ingest)
                    .with_stability_band(310)
                    .with_drift_step(80)
                    .with_mean_reversion_per_mille(150)
                    .with_factor_process(210, 925, 620)
                    .with_regime_process(930, 85, 115, 35)
                    .with_shift_probabilities(32, 9, 1)
                    .with_shift_ranges(
                        FeedShiftRange::new(650, 1_300),
                        FeedShiftRange::new(2_400, 4_800),
                        FeedShiftRange::new(7_000, 12_000),
                    ),
            ),
            (
                PricingMaterial::Copper,
                FeedStreamProfile::new("material:copper", 84_000)
                    .with_phase(ExecutionPhase::Ingest)
                    .with_stability_band(410)
                    .with_drift_step(110)
                    .with_mean_reversion_per_mille(170)
                    .with_factor_process(260, 930, 700)
                    .with_regime_process(920, 70, 135, 45)
                    .with_shift_probabilities(34, 11, 2)
                    .with_shift_ranges(
                        FeedShiftRange::new(700, 1_500),
                        FeedShiftRange::new(2_800, 5_500),
                        FeedShiftRange::new(9_000, 14_000),
                    ),
            ),
            (
                PricingMaterial::PlasticResin,
                FeedStreamProfile::new("material:plastic-resin", 28_000)
                    .with_phase(ExecutionPhase::Ingest)
                    .with_stability_band(180)
                    .with_drift_step(60)
                    .with_mean_reversion_per_mille(160)
                    .with_factor_process(150, 900, 520)
                    .with_regime_process(935, 95, 95, 25)
                    .with_shift_probabilities(20, 6, 1)
                    .with_shift_ranges(
                        FeedShiftRange::new(300, 800),
                        FeedShiftRange::new(1_200, 2_700),
                        FeedShiftRange::new(3_000, 6_000),
                    ),
            ),
            (
                PricingMaterial::Electronics,
                FeedStreamProfile::new("material:electronics", 120_000)
                    .with_phase(ExecutionPhase::Ingest)
                    .with_stability_band(550)
                    .with_drift_step(140)
                    .with_mean_reversion_per_mille(90)
                    .with_factor_process(320, 920, 760)
                    .with_regime_process(905, 55, 145, 70)
                    .with_shift_probabilities(18, 14, 3)
                    .with_shift_ranges(
                        FeedShiftRange::new(900, 1_900),
                        FeedShiftRange::new(4_000, 9_000),
                        FeedShiftRange::new(10_000, 18_000),
                    ),
            ),
            (
                PricingMaterial::Packaging,
                FeedStreamProfile::new("material:packaging", 9_000)
                    .with_phase(ExecutionPhase::Ingest)
                    .with_stability_band(70)
                    .with_drift_step(20)
                    .with_mean_reversion_per_mille(200)
                    .with_factor_process(50, 960, 260)
                    .with_regime_process(955, 120, 45, 10)
                    .with_shift_probabilities(8, 2, 0)
                    .with_shift_ranges(
                        FeedShiftRange::new(100, 240),
                        FeedShiftRange::new(350, 700),
                        FeedShiftRange::new(0, 0),
                    ),
            ),
            (
                PricingMaterial::Labor,
                FeedStreamProfile::new("material:labor", 55_000)
                    .with_phase(ExecutionPhase::Ingest)
                    .with_stability_band(90)
                    .with_drift_step(25)
                    .with_mean_reversion_per_mille(240)
                    .with_factor_process(60, 970, 180)
                    .with_regime_process(970, 140, 35, 5)
                    .with_shift_probabilities(6, 2, 0)
                    .with_shift_ranges(
                        FeedShiftRange::new(120, 260),
                        FeedShiftRange::new(500, 900),
                        FeedShiftRange::new(0, 0),
                    ),
            ),
        ])
    }

    pub(super) fn industrial_factor_profile() -> FeedStreamProfile {
        FeedStreamProfile::new("factor:industrial", 0)
            .with_phase(ExecutionPhase::Ingest)
            .with_stability_band(180)
            .with_drift_step(80)
            .with_mean_reversion_per_mille(120)
            .with_factor_process(120, 960, 0)
            .with_regime_process(920, 60, 150, 55)
            .with_shift_probabilities(14, 6, 1)
            .with_shift_ranges(
                FeedShiftRange::new(250, 550),
                FeedShiftRange::new(800, 1_600),
                FeedShiftRange::new(2_000, 4_000),
            )
    }

    pub(super) fn energy_factor_profile() -> FeedStreamProfile {
        FeedStreamProfile::new("factor:energy", 0)
            .with_phase(ExecutionPhase::Ingest)
            .with_stability_band(220)
            .with_drift_step(100)
            .with_mean_reversion_per_mille(90)
            .with_factor_process(160, 955, 0)
            .with_regime_process(900, 45, 170, 85)
            .with_shift_probabilities(18, 10, 2)
            .with_shift_ranges(
                FeedShiftRange::new(300, 700),
                FeedShiftRange::new(1_000, 2_100),
                FeedShiftRange::new(2_500, 5_000),
            )
    }
}
