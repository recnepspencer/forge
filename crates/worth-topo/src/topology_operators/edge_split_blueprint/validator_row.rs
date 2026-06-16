use super::classification::EdgeSplitValidatorRuntimeLane;
use super::proof_obligation::EdgeSplitValidatorProofObligation;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EdgeSplitValidatorRow {
    validator_name: &'static str,
    runtime_lane: EdgeSplitValidatorRuntimeLane,
    governs_topology_legality: bool,
    proof_obligations: &'static [EdgeSplitValidatorProofObligation],
}

impl EdgeSplitValidatorRow {
    pub(crate) fn new(
        validator_name: &'static str,
        runtime_lane: EdgeSplitValidatorRuntimeLane,
        governs_topology_legality: bool,
        proof_obligations: &'static [EdgeSplitValidatorProofObligation],
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

    pub fn runtime_lane(&self) -> EdgeSplitValidatorRuntimeLane {
        self.runtime_lane
    }

    pub fn governs_topology_legality(&self) -> bool {
        self.governs_topology_legality
    }

    pub fn proof_obligations(&self) -> &'static [EdgeSplitValidatorProofObligation] {
        self.proof_obligations
    }

    pub fn requires_runtime_lane(&self, runtime_lane: EdgeSplitValidatorRuntimeLane) -> bool {
        self.runtime_lane == runtime_lane
    }
}
