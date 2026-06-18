use topology::facade::{
    PlanarBooleanLoopValidatorProofObligation, PlanarBooleanLoopValidatorRow,
    PlanarBooleanLoopValidatorRuntimeLane,
};

fn main() {
    let _ = PlanarBooleanLoopValidatorRow {
        validator_name: "FakeLoopValidator",
        runtime_lane: PlanarBooleanLoopValidatorRuntimeLane::QueryGraphInvariantPack,
        governs_topology_legality: true,
        proof_obligations: &[PlanarBooleanLoopValidatorProofObligation::RuntimeFacingDenialPathTypedAndInspectable],
    };
}
