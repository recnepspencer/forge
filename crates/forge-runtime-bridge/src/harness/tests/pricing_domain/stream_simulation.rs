use std::collections::BTreeMap;

use super::{
    MaterialPriceAttribution, MaterialTick, MaterialTickWave, PricingDomainWorld, PricingMaterial,
};

impl PricingDomainWorld {
    pub(in crate::harness::tests) fn advance_material_streams(&mut self) -> MaterialTickWave {
        self.current_industrial_factor_microunits = self
            .industrial_factor_generator
            .next_sample()
            .value_microunits;
        self.current_energy_factor_microunits =
            self.energy_factor_generator.next_sample().value_microunits;
        let industrial_factor = self.current_industrial_factor_microunits;
        let energy_factor = self.current_energy_factor_microunits;

        let mut changed_materials = Vec::new();
        for (material, generator) in &mut self.generators {
            let external_factor =
                Self::external_factor_for(*material, industrial_factor, energy_factor);
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
