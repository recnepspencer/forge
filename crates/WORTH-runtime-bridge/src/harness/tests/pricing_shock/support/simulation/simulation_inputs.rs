use super::*;

pub(in crate::harness::tests::pricing_shock) fn simulation_candidate_materials(
) -> &'static [PricingMaterial] {
    &[
        PricingMaterial::Steel,
        PricingMaterial::Aluminum,
        PricingMaterial::Copper,
        PricingMaterial::Rubber,
        PricingMaterial::PlasticResin,
        PricingMaterial::Electronics,
        PricingMaterial::Packaging,
        PricingMaterial::Labor,
        PricingMaterial::Fuel,
    ]
}

pub(in crate::harness::tests::pricing_shock) fn base_shock_multiplier_per_mille(
    material: PricingMaterial,
) -> i64 {
    match material {
        PricingMaterial::Rubber => 2_350,
        PricingMaterial::Fuel => 2_200,
        PricingMaterial::Steel => 1_650,
        PricingMaterial::Copper => 1_800,
        PricingMaterial::Electronics => 1_700,
        PricingMaterial::Aluminum => 1_500,
        PricingMaterial::PlasticResin => 1_450,
        PricingMaterial::Packaging => 1_180,
        PricingMaterial::Labor => 1_120,
    }
}

pub(in crate::harness::tests::pricing_shock) fn regime_pressure_per_mille(
    regime: FeedVolatilityRegime,
) -> i64 {
    match regime {
        FeedVolatilityRegime::Calm => 0,
        FeedVolatilityRegime::Normal => 120,
        FeedVolatilityRegime::Volatile => 320,
        FeedVolatilityRegime::Stressed => 720,
    }
}

pub(in crate::harness::tests::pricing_shock) fn event_pressure_per_mille(
    event_kind: FeedStreamEventKind,
) -> i64 {
    match event_kind {
        FeedStreamEventKind::Stable => 0,
        FeedStreamEventKind::Noise => 35,
        FeedStreamEventKind::Drift => 80,
        FeedStreamEventKind::MinorShift => 180,
        FeedStreamEventKind::MajorShift => 420,
        FeedStreamEventKind::RegimeShift => 760,
    }
}

pub(in crate::harness::tests::pricing_shock) fn natural_shock_multiplier_per_mille(
    material: PricingMaterial,
    attribution: &MaterialPriceAttribution,
    branch_index: usize,
    iteration_index: usize,
) -> i64 {
    let branch_variation = ((branch_index as i64 * 67) + (iteration_index as i64 * 29)) % 240;
    let jump_pressure = (attribution.jump_delta_microunits.abs() / 2_000).clamp(0, 500);
    let factor_pressure = (attribution.external_factor_microunits.abs() / 3_000).clamp(0, 260);
    (base_shock_multiplier_per_mille(material)
        + regime_pressure_per_mille(attribution.regime)
        + event_pressure_per_mille(attribution.event_kind)
        + branch_variation
        + jump_pressure
        + factor_pressure)
        .clamp(1_100, 4_500)
}

pub(in crate::harness::tests::pricing_shock) fn family_tariff_bps_for_material(
    material: PricingMaterial,
    branch_index: usize,
    iteration_index: usize,
) -> BTreeMap<String, i64> {
    let pulse = 40 + ((branch_index as i64 * 13 + iteration_index as i64 * 11) % 140);
    match material {
        PricingMaterial::Fuel => BTreeMap::from([
            ("washer".to_owned(), 320 + pulse),
            ("dryer".to_owned(), 300 + pulse),
            ("e-bike".to_owned(), 180 + (pulse / 2)),
        ]),
        PricingMaterial::Electronics => BTreeMap::from([
            ("e-bike".to_owned(), 360 + pulse),
            ("washer".to_owned(), 240 + (pulse / 2)),
            ("dryer".to_owned(), 220 + (pulse / 2)),
        ]),
        PricingMaterial::Steel | PricingMaterial::Copper => BTreeMap::from([
            ("washer".to_owned(), 220 + pulse),
            ("dryer".to_owned(), 210 + pulse),
        ]),
        PricingMaterial::Rubber => BTreeMap::from([
            ("bicycle".to_owned(), 120 + (pulse / 3)),
            ("scooter".to_owned(), 140 + (pulse / 3)),
            ("e-bike".to_owned(), 160 + (pulse / 3)),
        ]),
        _ => BTreeMap::new(),
    }
}

pub(in crate::harness::tests::pricing_shock) fn sum_retail_cents(
    entries: &[ProductPriceBreakdown],
) -> i64 {
    entries.iter().map(|entry| entry.retail_price_cents).sum()
}
