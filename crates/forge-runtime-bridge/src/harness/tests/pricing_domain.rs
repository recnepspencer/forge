use std::collections::BTreeMap;

use forge_harness::facade::{
    DeterministicFeedStreamGenerator, ExecutionPhase, FeedShiftRange, FeedStreamEventKind,
    FeedStreamProfile, FeedVolatilityRegime,
};

use crate::facade::TruthSnapshotIdentity;
use crate::facade::SnapshotReadRecord;
use crate::harness::fixtures::SnapshotFixture;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum PricingMaterial {
    Steel,
    Aluminum,
    Copper,
    Rubber,
    PlasticResin,
    Electronics,
    Packaging,
    Labor,
    Fuel,
}

impl PricingMaterial {
    pub(super) fn key(self) -> &'static str {
        match self {
            Self::Steel => "steel",
            Self::Aluminum => "aluminum",
            Self::Copper => "copper",
            Self::Rubber => "rubber",
            Self::PlasticResin => "plastic-resin",
            Self::Electronics => "electronics",
            Self::Packaging => "packaging",
            Self::Labor => "labor",
            Self::Fuel => "fuel",
        }
    }

    fn snapshot_record_key(self) -> String {
        format!("component:{}:cost", self.key())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct MaterialRequirement {
    pub(super) material: PricingMaterial,
    pub(super) quantity_milliunits: i64,
}

impl MaterialRequirement {
    pub(super) fn new(material: PricingMaterial, quantity_milliunits: i64) -> Self {
        Self {
            material,
            quantity_milliunits,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ShippingSpec {
    pub(super) route_class: String,
    pub(super) route_distance_km: i64,
    pub(super) shipment_weight_grams: i64,
    pub(super) packaging_volume_cc: i64,
    pub(super) base_shipping_cents: i64,
    pub(super) fuel_burn_microliters_per_kg_km: i64,
}

impl ShippingSpec {
    pub(super) fn new(
        route_class: impl Into<String>,
        route_distance_km: i64,
        shipment_weight_grams: i64,
        packaging_volume_cc: i64,
        base_shipping_cents: i64,
        fuel_burn_microliters_per_kg_km: i64,
    ) -> Self {
        Self {
            route_class: route_class.into(),
            route_distance_km,
            shipment_weight_grams,
            packaging_volume_cc,
            base_shipping_cents,
            fuel_burn_microliters_per_kg_km,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ToleranceGate {
    pub(super) repricing_threshold_bps: i64,
    pub(super) margin_floor_bps: i64,
}

impl ToleranceGate {
    pub(super) fn new(repricing_threshold_bps: i64, margin_floor_bps: i64) -> Self {
        Self {
            repricing_threshold_bps,
            margin_floor_bps,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct PricingProduct {
    pub(super) sku: String,
    pub(super) family: String,
    pub(super) materials: Vec<MaterialRequirement>,
    pub(super) shipping: ShippingSpec,
    pub(super) tolerance_gate: ToleranceGate,
    pub(super) margin_bps: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ProductPriceBreakdown {
    pub(super) sku: String,
    pub(super) family: String,
    pub(super) material_cost_cents: i64,
    pub(super) shipping_cost_cents: i64,
    pub(super) policy_surcharge_cents: i64,
    pub(super) baseline_landed_cost_cents: i64,
    pub(super) landed_cost_cents: i64,
    pub(super) landed_cost_delta_cents: i64,
    pub(super) margin_cents: i64,
    pub(super) retail_price_cents: i64,
    pub(super) repricing_threshold_cents: i64,
    pub(super) repricing_triggered: bool,
    pub(super) margin_floor_breached: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct MaterialTick {
    pub(super) material: PricingMaterial,
    pub(super) event_kind: FeedStreamEventKind,
    pub(super) value_microunits: i64,
    pub(super) attribution: MaterialPriceAttribution,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct MaterialTickWave {
    pub(super) sequence: u64,
    pub(super) industrial_factor_microunits: i64,
    pub(super) energy_factor_microunits: i64,
    pub(super) changed_materials: Vec<MaterialTick>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct MaterialPriceAttribution {
    pub(super) material: PricingMaterial,
    pub(super) event_kind: FeedStreamEventKind,
    pub(super) regime: FeedVolatilityRegime,
    pub(super) previous_value_microunits: i64,
    pub(super) current_value_microunits: i64,
    pub(super) delta_microunits: i64,
    pub(super) external_factor_microunits: i64,
    pub(super) factor_delta_microunits: i64,
    pub(super) trend_delta_microunits: i64,
    pub(super) mean_reversion_delta_microunits: i64,
    pub(super) idiosyncratic_noise_microunits: i64,
    pub(super) jump_delta_microunits: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ProductPricingAttribution {
    pub(super) sku: String,
    pub(super) retail_price_cents: i64,
    pub(super) baseline_landed_cost_cents: i64,
    pub(super) landed_cost_cents: i64,
    pub(super) landed_cost_delta_cents: i64,
    pub(super) material_cost_cents: i64,
    pub(super) shipping_cost_cents: i64,
    pub(super) margin_cents: i64,
    pub(super) repricing_threshold_cents: i64,
    pub(super) repricing_triggered: bool,
    pub(super) margin_floor_breached: bool,
    pub(super) fuel_shipping_component_cents: i64,
    pub(super) packaging_surcharge_cents: i64,
    pub(super) material_contributions_cents: Vec<(PricingMaterial, i64)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct PricingCommitAttribution {
    pub(super) commit_identity: String,
    pub(super) snapshot_identity: String,
    pub(super) branch_identity: String,
    pub(super) material: PricingMaterial,
    pub(super) material_attribution: MaterialPriceAttribution,
    pub(super) shock_delta_microunits: i64,
    pub(super) shock_multiplier_per_mille: i64,
    pub(super) representative_product: ProductPricingAttribution,
}

#[derive(Debug, Clone)]
pub(super) struct PricingDomainWorld {
    generators: BTreeMap<PricingMaterial, DeterministicFeedStreamGenerator>,
    baseline_prices_microunits: BTreeMap<PricingMaterial, i64>,
    industrial_factor_generator: DeterministicFeedStreamGenerator,
    energy_factor_generator: DeterministicFeedStreamGenerator,
    current_prices_microunits: BTreeMap<PricingMaterial, i64>,
    current_industrial_factor_microunits: i64,
    current_energy_factor_microunits: i64,
    products: Vec<PricingProduct>,
    next_sequence: u64,
}

impl PricingDomainWorld {
    pub(super) fn reference_catalog() -> Vec<PricingProduct> {
        let templates = vec![
            (
                "bicycle",
                vec![
                    MaterialRequirement::new(PricingMaterial::Steel, 18_000),
                    MaterialRequirement::new(PricingMaterial::Rubber, 5_200),
                    MaterialRequirement::new(PricingMaterial::Aluminum, 2_400),
                    MaterialRequirement::new(PricingMaterial::Labor, 6_000),
                    MaterialRequirement::new(PricingMaterial::Packaging, 1_000),
                ],
                ShippingSpec::new("regional-ground", 820, 15_000, 140_000, 2_800, 130),
                ToleranceGate::new(45, 1_800),
                2_800,
            ),
            (
                "scooter",
                vec![
                    MaterialRequirement::new(PricingMaterial::Steel, 9_500),
                    MaterialRequirement::new(PricingMaterial::Rubber, 3_600),
                    MaterialRequirement::new(PricingMaterial::PlasticResin, 1_400),
                    MaterialRequirement::new(PricingMaterial::Labor, 4_500),
                    MaterialRequirement::new(PricingMaterial::Packaging, 900),
                ],
                ShippingSpec::new("metro-ground", 410, 9_000, 82_000, 1_900, 150),
                ToleranceGate::new(40, 1_600),
                2_700,
            ),
            (
                "wheelbarrow",
                vec![
                    MaterialRequirement::new(PricingMaterial::Steel, 11_000),
                    MaterialRequirement::new(PricingMaterial::Rubber, 2_500),
                    MaterialRequirement::new(PricingMaterial::PlasticResin, 1_800),
                    MaterialRequirement::new(PricingMaterial::Labor, 3_400),
                    MaterialRequirement::new(PricingMaterial::Packaging, 700),
                ],
                ShippingSpec::new("regional-ground", 760, 12_500, 120_000, 2_200, 128),
                ToleranceGate::new(35, 1_500),
                2_400,
            ),
            (
                "washer",
                vec![
                    MaterialRequirement::new(PricingMaterial::Steel, 24_000),
                    MaterialRequirement::new(PricingMaterial::Copper, 4_500),
                    MaterialRequirement::new(PricingMaterial::PlasticResin, 3_400),
                    MaterialRequirement::new(PricingMaterial::Electronics, 6_500),
                    MaterialRequirement::new(PricingMaterial::Labor, 8_500),
                    MaterialRequirement::new(PricingMaterial::Packaging, 1_400),
                ],
                ShippingSpec::new("appliance-truck", 1_150, 68_000, 380_000, 8_200, 170),
                ToleranceGate::new(30, 1_300),
                2_200,
            ),
            (
                "dryer",
                vec![
                    MaterialRequirement::new(PricingMaterial::Steel, 22_000),
                    MaterialRequirement::new(PricingMaterial::Copper, 3_900),
                    MaterialRequirement::new(PricingMaterial::PlasticResin, 2_800),
                    MaterialRequirement::new(PricingMaterial::Electronics, 5_400),
                    MaterialRequirement::new(PricingMaterial::Labor, 7_900),
                    MaterialRequirement::new(PricingMaterial::Packaging, 1_300),
                ],
                ShippingSpec::new("appliance-truck", 1_050, 62_000, 360_000, 7_800, 166),
                ToleranceGate::new(30, 1_300),
                2_100,
            ),
            (
                "e-bike",
                vec![
                    MaterialRequirement::new(PricingMaterial::Steel, 10_000),
                    MaterialRequirement::new(PricingMaterial::Aluminum, 6_000),
                    MaterialRequirement::new(PricingMaterial::Rubber, 4_800),
                    MaterialRequirement::new(PricingMaterial::Electronics, 9_000),
                    MaterialRequirement::new(PricingMaterial::Labor, 7_200),
                    MaterialRequirement::new(PricingMaterial::Packaging, 1_100),
                ],
                ShippingSpec::new("regional-ground", 910, 24_000, 180_000, 3_900, 136),
                ToleranceGate::new(35, 1_700),
                2_900,
            ),
        ];

        let mut products = Vec::new();
        for product_idx in 0..100 {
            let template = &templates[product_idx % templates.len()];
            products.push(PricingProduct {
                sku: format!("{}-{product_idx:03}", template.0),
                family: template.0.to_owned(),
                materials: template.1.clone(),
                shipping: template.2.clone(),
                tolerance_gate: template.3.clone(),
                margin_bps: template.4,
            });
        }
        products
    }

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

    fn industrial_factor_profile() -> FeedStreamProfile {
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

    fn energy_factor_profile() -> FeedStreamProfile {
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

    pub(super) fn new(seed: u64) -> Self {
        let stream_profiles = Self::reference_stream_profiles();
        let mut generators = BTreeMap::new();
        let mut baseline_prices_microunits = BTreeMap::new();
        let mut current_prices_microunits = BTreeMap::new();
        let industrial_factor_profile = Self::industrial_factor_profile();
        let energy_factor_profile = Self::energy_factor_profile();

        for (offset, (material, profile)) in stream_profiles.into_iter().enumerate() {
            let generator = DeterministicFeedStreamGenerator::new(profile.clone(), seed + offset as u64 + 1);
            baseline_prices_microunits.insert(material, profile.starting_value_microunits);
            current_prices_microunits.insert(material, profile.starting_value_microunits);
            generators.insert(material, generator);
        }

        Self {
            baseline_prices_microunits,
            industrial_factor_generator: DeterministicFeedStreamGenerator::new(
                industrial_factor_profile.clone(),
                seed + 1_000,
            ),
            energy_factor_generator: DeterministicFeedStreamGenerator::new(
                energy_factor_profile.clone(),
                seed + 2_000,
            ),
            generators,
            current_prices_microunits,
            current_industrial_factor_microunits: industrial_factor_profile.starting_value_microunits,
            current_energy_factor_microunits: energy_factor_profile.starting_value_microunits,
            products: Self::reference_catalog(),
            next_sequence: 1,
        }
    }

    pub(super) fn products(&self) -> &[PricingProduct] {
        &self.products
    }

    pub(super) fn current_material_price_microunits(&self, material: PricingMaterial) -> i64 {
        *self
            .current_prices_microunits
            .get(&material)
            .expect("material price should exist in reference world")
    }

    pub(super) fn baseline_material_price_microunits(&self, material: PricingMaterial) -> i64 {
        *self
            .baseline_prices_microunits
            .get(&material)
            .expect("baseline material price should exist in reference world")
    }

    pub(super) fn advance_material_streams(&mut self) -> MaterialTickWave {
        self.current_industrial_factor_microunits = self
            .industrial_factor_generator
            .next_sample()
            .value_microunits;
        self.current_energy_factor_microunits = self
            .energy_factor_generator
            .next_sample()
            .value_microunits;
        let industrial_factor = self.current_industrial_factor_microunits;
        let energy_factor = self.current_energy_factor_microunits;

        let mut changed_materials = Vec::new();
        for (material, generator) in &mut self.generators {
            let external_factor = Self::external_factor_for(*material, industrial_factor, energy_factor);
            let previous_value = generator.current_value_microunits();
            let sample = generator.next_sample_with_external_factor(external_factor);
            self.current_prices_microunits
                .insert(*material, sample.value_microunits);
            changed_materials.push(MaterialTick {
                material: *material,
                event_kind: sample.event_kind,
                value_microunits: sample.value_microunits,
                attribution: MaterialPriceAttribution {
                    material: *material,
                    event_kind: sample.event_kind,
                    regime: sample.regime,
                    previous_value_microunits: previous_value,
                    current_value_microunits: sample.value_microunits,
                    delta_microunits: sample.delta_microunits,
                    external_factor_microunits: parse_i64_metadata(
                        &sample.metadata,
                        "external_factor_microunits",
                    ),
                    factor_delta_microunits: parse_i64_metadata(
                        &sample.metadata,
                        "factor_delta_microunits",
                    ),
                    trend_delta_microunits: parse_i64_metadata(
                        &sample.metadata,
                        "trend_delta_microunits",
                    ),
                    mean_reversion_delta_microunits: parse_i64_metadata(
                        &sample.metadata,
                        "mean_reversion_delta_microunits",
                    ),
                    idiosyncratic_noise_microunits: parse_i64_metadata(
                        &sample.metadata,
                        "idiosyncratic_noise_microunits",
                    ),
                    jump_delta_microunits: parse_i64_metadata(
                        &sample.metadata,
                        "jump_delta_microunits",
                    ),
                },
            });
        }
        let wave = MaterialTickWave {
            sequence: self.next_sequence,
            industrial_factor_microunits: industrial_factor,
            energy_factor_microunits: energy_factor,
            changed_materials,
        };
        self.next_sequence += 1;
        wave
    }

    pub(super) fn snapshot_fixture(&self, snapshot_id: &str) -> SnapshotFixture {
        self.snapshot_fixture_with_overrides(snapshot_id, [])
    }

    pub(super) fn snapshot_fixture_with_overrides<I>(
        &self,
        snapshot_id: &str,
        overrides: I,
    ) -> SnapshotFixture
    where
        I: IntoIterator<Item = (PricingMaterial, i64)>,
    {
        let override_map = overrides.into_iter().collect::<BTreeMap<_, _>>();
        let mut records = Vec::new();
        for material in self.current_prices_microunits.keys() {
            let payload = override_map
                .get(material)
                .copied()
                .unwrap_or_else(|| self.current_material_price_microunits(*material))
                .to_string()
                .into_bytes();
            records.push(SnapshotReadRecord::new(material.snapshot_record_key(), payload));
        }

        SnapshotFixture::new(TruthSnapshotIdentity::new(snapshot_id), records)
    }

    pub(super) fn shocked_material_price_microunits(
        &self,
        material: PricingMaterial,
        multiplier_per_mille: i64,
    ) -> i64 {
        self.current_material_price_microunits(material)
            .saturating_mul(multiplier_per_mille)
            / 1000
    }

    pub(super) fn price_product(&self, product: &PricingProduct) -> ProductPriceBreakdown {
        self.price_product_with_scenario(product, BTreeMap::new(), BTreeMap::new())
    }

    pub(super) fn price_product_with_overrides(
        &self,
        product: &PricingProduct,
        overrides: BTreeMap<PricingMaterial, i64>,
    ) -> ProductPriceBreakdown {
        self.price_product_with_scenario(product, overrides, BTreeMap::new())
    }

    pub(super) fn price_product_with_scenario(
        &self,
        product: &PricingProduct,
        overrides: BTreeMap<PricingMaterial, i64>,
        family_tariff_bps: BTreeMap<String, i64>,
    ) -> ProductPriceBreakdown {
        let baseline_material_cost_cents = product
            .materials
            .iter()
            .map(|requirement| {
                self.baseline_material_price_microunits(requirement.material)
                    * requirement.quantity_milliunits
                    / 1_000_000
            })
            .sum::<i64>();
        let material_cost_cents = product
            .materials
            .iter()
            .map(|requirement| {
                self.material_price_with_overrides(requirement.material, &overrides)
                    * requirement.quantity_milliunits
                    / 1_000_000
            })
            .sum::<i64>();

        let baseline_shipping_cost_cents =
            self.shipping_cost_cents_with_prices(&product.shipping, &self.baseline_prices_microunits);
        let shipping_cost_cents = self.shipping_cost_cents_with_overrides(&product.shipping, &overrides);
        let baseline_landed_cost_cents = baseline_material_cost_cents + baseline_shipping_cost_cents;
        let pre_policy_landed_cost_cents = material_cost_cents + shipping_cost_cents;
        let tariff_bps = family_tariff_bps
            .get(&product.family)
            .copied()
            .unwrap_or(0)
            .max(0);
        let policy_surcharge_cents = pre_policy_landed_cost_cents * tariff_bps / 10_000;
        let landed_cost_cents = pre_policy_landed_cost_cents + policy_surcharge_cents;
        let landed_cost_delta_cents = (landed_cost_cents - baseline_landed_cost_cents).abs();
        let margin_cents = landed_cost_cents * product.margin_bps / 10_000;
        let retail_price_cents = landed_cost_cents + margin_cents;
        let repricing_threshold_cents =
            (baseline_landed_cost_cents * product.tolerance_gate.repricing_threshold_bps) / 10_000;
        let margin_floor_breached = product.margin_bps < product.tolerance_gate.margin_floor_bps;
        let repricing_triggered = (repricing_threshold_cents > 0
            && landed_cost_delta_cents >= repricing_threshold_cents)
            || margin_floor_breached;

        ProductPriceBreakdown {
            sku: product.sku.clone(),
            family: product.family.clone(),
            material_cost_cents,
            shipping_cost_cents,
            policy_surcharge_cents,
            baseline_landed_cost_cents,
            landed_cost_cents,
            landed_cost_delta_cents,
            margin_cents,
            retail_price_cents,
            repricing_threshold_cents,
            repricing_triggered,
            margin_floor_breached,
        }
    }

    pub(super) fn price_matrix(&self) -> Vec<ProductPriceBreakdown> {
        self.products
            .iter()
            .map(|product| self.price_product(product))
            .collect()
    }

    pub(super) fn price_matrix_with_overrides<I>(
        &self,
        overrides: I,
    ) -> Vec<ProductPriceBreakdown>
    where
        I: IntoIterator<Item = (PricingMaterial, i64)>,
    {
        let override_map = overrides.into_iter().collect::<BTreeMap<_, _>>();
        self.products
            .iter()
            .map(|product| self.price_product_with_overrides(product, override_map.clone()))
            .collect()
    }

    pub(super) fn price_matrix_with_scenario(
        &self,
        overrides: BTreeMap<PricingMaterial, i64>,
        family_tariff_bps: BTreeMap<String, i64>,
    ) -> Vec<ProductPriceBreakdown> {
        self.products
            .iter()
            .map(|product| {
                self.price_product_with_scenario(
                    product,
                    overrides.clone(),
                    family_tariff_bps.clone(),
                )
            })
            .collect()
    }

    pub(super) fn explain_product_price(&self, sku: &str) -> ProductPricingAttribution {
        let product = self
            .products
            .iter()
            .find(|product| product.sku == sku)
            .expect("product sku should exist in pricing domain");
        let breakdown = self.price_product(product);
        let material_contributions_cents = product
            .materials
            .iter()
            .map(|requirement| {
                (
                    requirement.material,
                    self.current_material_price_microunits(requirement.material)
                        * requirement.quantity_milliunits
                        / 1_000_000,
                )
            })
            .collect::<Vec<_>>();
        let override_map = BTreeMap::new();
        let (fuel_shipping_component_cents, packaging_surcharge_cents) =
            self.shipping_components_cents_with_overrides(&product.shipping, &override_map);

        ProductPricingAttribution {
            sku: breakdown.sku,
            retail_price_cents: breakdown.retail_price_cents,
            baseline_landed_cost_cents: breakdown.baseline_landed_cost_cents,
            landed_cost_cents: breakdown.landed_cost_cents,
            landed_cost_delta_cents: breakdown.landed_cost_delta_cents,
            material_cost_cents: breakdown.material_cost_cents,
            shipping_cost_cents: breakdown.shipping_cost_cents,
            margin_cents: breakdown.margin_cents,
            repricing_threshold_cents: breakdown.repricing_threshold_cents,
            repricing_triggered: breakdown.repricing_triggered,
            margin_floor_breached: breakdown.margin_floor_breached,
            fuel_shipping_component_cents,
            packaging_surcharge_cents,
            material_contributions_cents,
        }
    }

    fn shipping_cost_cents_with_overrides(
        &self,
        shipping: &ShippingSpec,
        overrides: &BTreeMap<PricingMaterial, i64>,
    ) -> i64 {
        self.shipping_cost_cents_with_prices(shipping, overrides)
    }

    fn shipping_cost_cents_with_prices(
        &self,
        shipping: &ShippingSpec,
        prices: &BTreeMap<PricingMaterial, i64>,
    ) -> i64 {
        let (fuel_component_cents, packaging_surcharge_cents) =
            self.shipping_components_cents_with_prices(shipping, prices);
        shipping.base_shipping_cents + fuel_component_cents + packaging_surcharge_cents
    }

    fn shipping_components_cents_with_overrides(
        &self,
        shipping: &ShippingSpec,
        overrides: &BTreeMap<PricingMaterial, i64>,
    ) -> (i64, i64) {
        self.shipping_components_cents_with_prices(shipping, overrides)
    }

    fn shipping_components_cents_with_prices(
        &self,
        shipping: &ShippingSpec,
        prices: &BTreeMap<PricingMaterial, i64>,
    ) -> (i64, i64) {
        let fuel_price_microunits = prices
            .get(&PricingMaterial::Fuel)
            .copied()
            .unwrap_or_else(|| self.current_material_price_microunits(PricingMaterial::Fuel));
        let weight_kg = shipping.shipment_weight_grams.max(0) / 1_000;
        let fuel_component_cents = fuel_price_microunits
            * shipping.fuel_burn_microliters_per_kg_km
            * weight_kg
            * shipping.route_distance_km
            / 1_000_000_000;
        let packaging_surcharge_cents = shipping.packaging_volume_cc / 25_000;
        (fuel_component_cents, packaging_surcharge_cents)
    }

    fn material_price_with_overrides(
        &self,
        material: PricingMaterial,
        overrides: &BTreeMap<PricingMaterial, i64>,
    ) -> i64 {
        overrides
            .get(&material)
            .copied()
            .unwrap_or_else(|| self.current_material_price_microunits(material))
    }

    fn external_factor_for(
        material: PricingMaterial,
        industrial_factor_microunits: i64,
        energy_factor_microunits: i64,
    ) -> i64 {
        match material {
            PricingMaterial::Steel => industrial_factor_microunits * 10 / 10,
            PricingMaterial::Aluminum => industrial_factor_microunits * 9 / 10,
            PricingMaterial::Copper => {
                industrial_factor_microunits * 7 / 10 + energy_factor_microunits * 3 / 10
            }
            PricingMaterial::Rubber => {
                industrial_factor_microunits * 4 / 10 + energy_factor_microunits * 6 / 10
            }
            PricingMaterial::PlasticResin => {
                industrial_factor_microunits * 3 / 10 + energy_factor_microunits * 7 / 10
            }
            PricingMaterial::Electronics => {
                industrial_factor_microunits * 8 / 10 + energy_factor_microunits * 2 / 10
            }
            PricingMaterial::Packaging => {
                industrial_factor_microunits * 2 / 10 + energy_factor_microunits * 2 / 10
            }
            PricingMaterial::Labor => industrial_factor_microunits * 2 / 10,
            PricingMaterial::Fuel => energy_factor_microunits,
        }
    }
}

fn parse_i64_metadata(metadata: &BTreeMap<String, String>, key: &str) -> i64 {
    metadata
        .get(key)
        .unwrap_or_else(|| panic!("feed stream sample metadata should include {key}"))
        .parse::<i64>()
        .unwrap_or_else(|_| panic!("feed stream sample metadata {key} should parse as i64"))
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::{PricingDomainWorld, PricingMaterial, PricingProduct};
    use forge_harness::facade::FeedStreamEventKind;

    #[test]
    fn pricing_domain_reference_catalog_is_large_shared_and_fuel_coupled() {
        let world = PricingDomainWorld::new(101);
        let products = world.products();

        assert_eq!(products.len(), 100);
        assert!(products.iter().any(|product| product.family == "bicycle"));
        assert!(products.iter().any(|product| product.family == "washer"));
        assert!(products.iter().all(|product| product.shipping.fuel_burn_microliters_per_kg_km > 0));
        assert!(products.iter().all(|product| product.tolerance_gate.repricing_threshold_bps > 0));
        assert!(products.iter().all(|product| {
            product
                .materials
                .iter()
                .any(|requirement| requirement.material == PricingMaterial::Labor)
        }));
    }

    #[test]
    fn pricing_domain_hidden_streams_advance_material_prices_and_export_snapshot_fixture() {
        let mut world = PricingDomainWorld::new(202);
        let first_wave = world.advance_material_streams();
        let second_wave = world.advance_material_streams();
        let snapshot = world.snapshot_fixture("snapshot:pricing-domain");

        assert_eq!(first_wave.sequence, 1);
        assert_eq!(second_wave.sequence, 2);
        assert_eq!(first_wave.changed_materials.len(), 9);
        assert_eq!(second_wave.changed_materials.len(), 9);
        assert_eq!(
            snapshot.identity().as_str(),
            "snapshot:pricing-domain"
        );
        assert_eq!(snapshot.records().len(), 9);
        assert!(snapshot
            .records()
            .iter()
            .any(|record| record.request_key() == "component:fuel:cost"));
        assert!(first_wave
            .changed_materials
            .iter()
            .any(|tick| tick.event_kind != FeedStreamEventKind::Stable));
    }

    #[test]
    fn pricing_domain_price_matrix_reflects_fuel_driven_shipping_and_tolerance_gates() {
        let mut world = PricingDomainWorld::new(303);
        world.advance_material_streams();
        let matrix = world.price_matrix();
        let bicycle = matrix
            .iter()
            .find(|breakdown| breakdown.sku.starts_with("bicycle-"))
            .expect("bicycle should exist in reference matrix");

        assert!(bicycle.material_cost_cents > 0);
        assert!(bicycle.shipping_cost_cents > 0);
        assert!(bicycle.baseline_landed_cost_cents > 0);
        assert!(bicycle.landed_cost_cents >= bicycle.material_cost_cents);
        assert_eq!(
            bicycle.landed_cost_delta_cents,
            (bicycle.landed_cost_cents - bicycle.baseline_landed_cost_cents).abs()
        );
        assert!(bicycle.retail_price_cents > bicycle.landed_cost_cents);
        assert_eq!(
            bicycle.repricing_triggered,
            (bicycle.repricing_threshold_cents > 0
                && bicycle.landed_cost_delta_cents >= bicycle.repricing_threshold_cents)
                || bicycle.margin_floor_breached
        );
        assert!(!bicycle.margin_floor_breached);
    }

    #[test]
    fn pricing_domain_product_breakdown_matches_independent_oracle_math() {
        let mut world = PricingDomainWorld::new(404);
        world.advance_material_streams();
        let product = world
            .products()
            .iter()
            .find(|product| product.sku.starts_with("bicycle-"))
            .expect("bicycle should exist in reference catalog");
        let mut overrides = BTreeMap::new();
        overrides.insert(PricingMaterial::Steel, world.current_material_price_microunits(PricingMaterial::Steel) + 12_500);
        overrides.insert(PricingMaterial::Fuel, world.current_material_price_microunits(PricingMaterial::Fuel) + 9_500);
        let breakdown = world.price_product_with_scenario(product, overrides.clone(), BTreeMap::new());
        let oracle = independent_breakdown_oracle(&world, product, &overrides);

        assert_eq!(breakdown.material_cost_cents, oracle.material_cost_cents);
        assert_eq!(breakdown.shipping_cost_cents, oracle.shipping_cost_cents);
        assert_eq!(breakdown.baseline_landed_cost_cents, oracle.baseline_landed_cost_cents);
        assert_eq!(breakdown.landed_cost_cents, oracle.landed_cost_cents);
        assert_eq!(breakdown.landed_cost_delta_cents, oracle.landed_cost_delta_cents);
        assert_eq!(breakdown.repricing_threshold_cents, oracle.repricing_threshold_cents);
        assert_eq!(breakdown.repricing_triggered, oracle.repricing_triggered);
    }

    #[test]
    fn pricing_domain_independent_oracle_holds_across_multiple_seeds_and_families() {
        for seed in [11_u64, 29, 47, 83, 131] {
            let mut world = PricingDomainWorld::new(seed);
            world.advance_material_streams();
            world.advance_material_streams();

            for family_prefix in ["bicycle-", "washer-", "e-bike-"] {
                let product = world
                    .products()
                    .iter()
                    .find(|product| product.sku.starts_with(family_prefix))
                    .expect("reference family should exist");
                let mut overrides = BTreeMap::new();
                overrides.insert(
                    PricingMaterial::Steel,
                    world.current_material_price_microunits(PricingMaterial::Steel) + 7_500,
                );
                overrides.insert(
                    PricingMaterial::Fuel,
                    world.current_material_price_microunits(PricingMaterial::Fuel) + 5_500,
                );
                if family_prefix == "washer-" || family_prefix == "e-bike-" {
                    overrides.insert(
                        PricingMaterial::Electronics,
                        world.current_material_price_microunits(PricingMaterial::Electronics) + 4_000,
                    );
                }

                let breakdown =
                    world.price_product_with_scenario(product, overrides.clone(), BTreeMap::new());
                let oracle = independent_breakdown_oracle(&world, product, &overrides);

                assert_eq!(breakdown.material_cost_cents, oracle.material_cost_cents);
                assert_eq!(breakdown.shipping_cost_cents, oracle.shipping_cost_cents);
                assert_eq!(
                    breakdown.baseline_landed_cost_cents,
                    oracle.baseline_landed_cost_cents
                );
                assert_eq!(breakdown.landed_cost_cents, oracle.landed_cost_cents);
                assert_eq!(breakdown.landed_cost_delta_cents, oracle.landed_cost_delta_cents);
                assert_eq!(
                    breakdown.repricing_threshold_cents,
                    oracle.repricing_threshold_cents
                );
                assert_eq!(breakdown.repricing_triggered, oracle.repricing_triggered);
            }
        }
    }

    struct OracleBreakdown {
        material_cost_cents: i64,
        shipping_cost_cents: i64,
        baseline_landed_cost_cents: i64,
        landed_cost_cents: i64,
        landed_cost_delta_cents: i64,
        repricing_threshold_cents: i64,
        repricing_triggered: bool,
    }

    fn independent_breakdown_oracle(
        world: &PricingDomainWorld,
        product: &PricingProduct,
        overrides: &BTreeMap<PricingMaterial, i64>,
    ) -> OracleBreakdown {
        let material_cost_cents = product
            .materials
            .iter()
            .map(|requirement| {
                overrides
                    .get(&requirement.material)
                    .copied()
                    .unwrap_or_else(|| world.current_material_price_microunits(requirement.material))
                    * requirement.quantity_milliunits
                    / 1_000_000
            })
            .sum::<i64>();
        let baseline_material_cost_cents = product
            .materials
            .iter()
            .map(|requirement| {
                world.baseline_material_price_microunits(requirement.material)
                    * requirement.quantity_milliunits
                    / 1_000_000
            })
            .sum::<i64>();
        let weight_kg = product.shipping.shipment_weight_grams.max(0) / 1_000;
        let packaging_surcharge_cents = product.shipping.packaging_volume_cc / 25_000;
        let fuel_price_microunits = overrides
            .get(&PricingMaterial::Fuel)
            .copied()
            .unwrap_or_else(|| world.current_material_price_microunits(PricingMaterial::Fuel));
        let shipping_cost_cents = product.shipping.base_shipping_cents
            + fuel_price_microunits
                * product.shipping.fuel_burn_microliters_per_kg_km
                * weight_kg
                * product.shipping.route_distance_km
                / 1_000_000_000
            + packaging_surcharge_cents;
        let baseline_shipping_cost_cents = product.shipping.base_shipping_cents
            + world.baseline_material_price_microunits(PricingMaterial::Fuel)
                * product.shipping.fuel_burn_microliters_per_kg_km
                * weight_kg
                * product.shipping.route_distance_km
                / 1_000_000_000
            + packaging_surcharge_cents;
        let baseline_landed_cost_cents =
            baseline_material_cost_cents + baseline_shipping_cost_cents;
        let landed_cost_cents = material_cost_cents + shipping_cost_cents;
        let landed_cost_delta_cents = (landed_cost_cents - baseline_landed_cost_cents).abs();
        let repricing_threshold_cents =
            baseline_landed_cost_cents * product.tolerance_gate.repricing_threshold_bps / 10_000;
        let margin_floor_breached = product.margin_bps < product.tolerance_gate.margin_floor_bps;

        OracleBreakdown {
            material_cost_cents,
            shipping_cost_cents,
            baseline_landed_cost_cents,
            landed_cost_cents,
            landed_cost_delta_cents,
            repricing_threshold_cents,
            repricing_triggered: (repricing_threshold_cents > 0
                && landed_cost_delta_cents >= repricing_threshold_cents)
                || margin_floor_breached,
        }
    }
}
