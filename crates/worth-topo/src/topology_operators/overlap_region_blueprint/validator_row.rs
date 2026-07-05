use super::classification::PlanarBooleanOverlapValidatorRuntimeLane;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapValidatorRow {
    validator_name: &'static str,
    runtime_lane: PlanarBooleanOverlapValidatorRuntimeLane,
    governs_topology_legality: bool,
}

impl PlanarBooleanOverlapValidatorRow {
    pub(crate) const fn new(
        validator_name: &'static str,
        runtime_lane: PlanarBooleanOverlapValidatorRuntimeLane,
        governs_topology_legality: bool,
    ) -> Self {
        Self {
            validator_name,
            runtime_lane,
            governs_topology_legality,
        }
    }

    pub fn validator_name(&self) -> &'static str {
        self.validator_name
    }

    pub fn runtime_lane(&self) -> PlanarBooleanOverlapValidatorRuntimeLane {
        self.runtime_lane
    }

    pub fn governs_topology_legality(&self) -> bool {
        self.governs_topology_legality
    }
}
