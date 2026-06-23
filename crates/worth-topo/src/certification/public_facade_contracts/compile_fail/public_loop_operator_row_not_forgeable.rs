use topology::facade::{
    PlanarBooleanLoopOperatorClassification, PlanarBooleanLoopOperatorRow,
    PlanarBooleanLoopOperatorTruthAuthority, PlanarBooleanLoopRequiredQuerySurface,
};

fn main() {
    let _ = PlanarBooleanLoopOperatorRow {
        operator_name: "FakeLoopOperator",
        classification: PlanarBooleanLoopOperatorClassification::PreparedSpatialOnly,
        truth_authority: PlanarBooleanLoopOperatorTruthAuthority::WorthSpatialPrepared,
        required_query_surface: PlanarBooleanLoopRequiredQuerySurface::None,
        topology_precedent: None,
        proof_obligations: &[],
        support_warning: None,
    };
}
