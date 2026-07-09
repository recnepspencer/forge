use std::collections::BTreeMap;

use worth_harness::facade::DeterministicFeedStreamGenerator;

use super::{PricingMaterial, PricingProduct};

#[derive(Debug, Clone)]
pub(in crate::harness::tests) struct PricingDomainWorld {
    pub(super) generators: BTreeMap<PricingMaterial, DeterministicFeedStreamGenerator>,
    pub(super) baseline_prices_microunits: BTreeMap<PricingMaterial, i64>,
    pub(super) industrial_factor_generator: DeterministicFeedStreamGenerator,
    pub(super) energy_factor_generator: DeterministicFeedStreamGenerator,
    pub(super) current_prices_microunits: BTreeMap<PricingMaterial, i64>,
    pub(super) current_industrial_factor_microunits: i64,
    pub(super) current_energy_factor_microunits: i64,
    pub(super) products: Vec<PricingProduct>,
    pub(super) next_sequence: u64,
}

impl PricingDomainWorld {
    pub(in crate::harness::tests) fn new(seed: u64) -> Self {
        let stream_profiles = Self::reference_stream_profiles();
        let mut generators = BTreeMap::new();
        let mut baseline_prices_microunits = BTreeMap::new();
        let mut current_prices_microunits = BTreeMap::new();
        let industrial_factor_profile = Self::industrial_factor_profile();
        let energy_factor_profile = Self::energy_factor_profile();

        for (offset, (material, profile)) in stream_profiles.into_iter().enumerate() {
            let generator =
                DeterministicFeedStreamGenerator::new(profile.clone(), seed + offset as u64 + 1);
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
            current_industrial_factor_microunits: industrial_factor_profile
                .starting_value_microunits,
            current_energy_factor_microunits: energy_factor_profile.starting_value_microunits,
            products: Self::reference_catalog(),
            next_sequence: 1,
        }
    }

    pub(in crate::harness::tests) fn products(&self) -> &[PricingProduct] {
        &self.products
    }

    pub(in crate::harness::tests) fn current_material_price_microunits(
        &self,
        material: PricingMaterial,
    ) -> i64 {
        *self
            .current_prices_microunits
            .get(&material)
            .expect("material price should exist in reference world")
    }

    pub(in crate::harness::tests) fn baseline_material_price_microunits(
        &self,
        material: PricingMaterial,
    ) -> i64 {
        *self
            .baseline_prices_microunits
            .get(&material)
            .expect("baseline material price should exist in reference world")
    }

    pub(in crate::harness::tests) fn shocked_material_price_microunits(
        &self,
        material: PricingMaterial,
        multiplier_per_mille: i64,
    ) -> i64 {
        self.current_material_price_microunits(material)
            .saturating_mul(multiplier_per_mille)
            / 1000
    }
}
