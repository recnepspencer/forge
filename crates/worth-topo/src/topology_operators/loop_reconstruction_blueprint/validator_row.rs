use super::classification::PlanarBooleanLoopValidatorRuntimeLane;
use super::proof_obligation::PlanarBooleanLoopValidatorProofObligation;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopValidatorRow {
    validator_name: &'static str,
    runtime_lane: PlanarBooleanLoopValidatorRuntimeLane,
    governs_topology_legality: bool,
    proof_obligations: &'static [PlanarBooleanLoopValidatorProofObligation],
}

impl PlanarBooleanLoopValidatorRow {
    pub(crate) fn new(
        validator_name: &'static str,
        runtime_lane: PlanarBooleanLoopValidatorRuntimeLane,
        governs_topology_legality: bool,
        proof_obligations: &'static [PlanarBooleanLoopValidatorProofObligation],
    ) -> Self {
        Self {
            validator_name,
            runtime_lane,
            governs_topology_legality,
            proof_obligations,
        }
    }

    pub fn validator_name(&self) -> &'static str {
        self.validator_name
    }

    pub fn runtime_lane(&self) -> PlanarBooleanLoopValidatorRuntimeLane {
        self.runtime_lane
    }

    pub fn governs_topology_legality(&self) -> bool {
        self.governs_topology_legality
    }

    pub fn proof_obligations(&self) -> &'static [PlanarBooleanLoopValidatorProofObligation] {
        self.proof_obligations
    }

    pub fn requires_runtime_lane(
        &self,
        runtime_lane: PlanarBooleanLoopValidatorRuntimeLane,
    ) -> bool {
        self.runtime_lane == runtime_lane
    }
}
