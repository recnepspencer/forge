use std::collections::BTreeMap;

use crate::construction::tests::support::compound_required_inventory::PrimitiveConstructionCorpusRequiredScenarioInventory;
use worth_geom::facade::PrimitiveRealizationExhaustionWitnessKind;

use super::super::cases::compound_scenarios;
use super::super::schema::{
    PrimitiveConstructionCompoundGrazingKind, PrimitiveConstructionCompoundMotionKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PrimitiveConstructionCompoundParityRegistry {
    required_scenario_ids: Vec<String>,
    motion_inventory: BTreeMap<String, PrimitiveConstructionCompoundMotionKind>,
    grazing_inventory: BTreeMap<String, PrimitiveConstructionCompoundGrazingKind>,
    exhaustion_inventory: BTreeMap<String, PrimitiveRealizationExhaustionWitnessKind>,
}

impl PrimitiveConstructionCompoundParityRegistry {
    pub(crate) fn motion_inventory(
        &self,
    ) -> &BTreeMap<String, PrimitiveConstructionCompoundMotionKind> {
        &self.motion_inventory
    }

    pub(crate) fn grazing_inventory(
        &self,
    ) -> &BTreeMap<String, PrimitiveConstructionCompoundGrazingKind> {
        &self.grazing_inventory
    }

    pub(crate) fn exhaustion_inventory(
        &self,
    ) -> &BTreeMap<String, PrimitiveRealizationExhaustionWitnessKind> {
        &self.exhaustion_inventory
    }

    pub(crate) fn required_scenario_inventory(
        &self,
    ) -> PrimitiveConstructionCorpusRequiredScenarioInventory {
        PrimitiveConstructionCorpusRequiredScenarioInventory::new(
            self.required_scenario_ids.clone(),
        )
    }
}

pub(crate) fn compound_parity_registry() -> PrimitiveConstructionCompoundParityRegistry {
    let scenarios = compound_scenarios();
    let required_scenario_ids = scenarios
        .iter()
        .map(|scenario| scenario.scenario_id().to_string())
        .collect::<Vec<_>>();
    let motion_inventory = scenarios
        .iter()
        .filter_map(|scenario| {
            scenario
                .motion_kind()
                .map(|kind| (scenario.scenario_id().to_string(), kind))
        })
        .collect::<BTreeMap<_, _>>();
    let grazing_inventory = scenarios
        .iter()
        .filter_map(|scenario| {
            scenario
                .grazing_kind()
                .map(|kind| (scenario.scenario_id().to_string(), kind))
        })
        .collect::<BTreeMap<_, _>>();
    let exhaustion_inventory = scenarios
        .iter()
        .filter_map(|scenario| {
            exhaustion_witness_kind_for(scenario.scenario_id())
                .map(|kind| (scenario.scenario_id().to_string(), kind))
        })
        .collect::<BTreeMap<_, _>>();
    PrimitiveConstructionCompoundParityRegistry {
        required_scenario_ids,
        motion_inventory,
        grazing_inventory,
        exhaustion_inventory,
    }
}

pub(crate) fn exhaustion_witness_kind_for(
    scenario_id: &str,
) -> Option<PrimitiveRealizationExhaustionWitnessKind> {
    match scenario_id {
        "pyramid_semantic_exhaustion" => {
            Some(PrimitiveRealizationExhaustionWitnessKind::ZeroRadiusPyramidSupportCollapse)
        }
        "simplex_world_collapsed_explicit_exhaustion" => {
            Some(PrimitiveRealizationExhaustionWitnessKind::AltitudeSqueezedSimplexSupportCollapse)
        }
        _ => None,
    }
}
