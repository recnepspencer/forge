use super::super::required_phase_2_validator_lanes::REQUIRED_PHASE_2_VALIDATOR_LANES;
use super::super::validator_row::PlanarBooleanOverlapValidatorRow;

pub(crate) fn phase_2_validators() -> Vec<PlanarBooleanOverlapValidatorRow> {
    REQUIRED_PHASE_2_VALIDATOR_LANES
        .iter()
        .map(|(name, runtime_lane, governs_topology_legality)| {
            PlanarBooleanOverlapValidatorRow::new(name, *runtime_lane, *governs_topology_legality)
        })
        .collect()
}
