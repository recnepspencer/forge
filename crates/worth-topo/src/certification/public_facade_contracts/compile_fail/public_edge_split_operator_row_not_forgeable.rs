use topology::facade::{
    EdgeSplitOperatorClassification, EdgeSplitOperatorRow, EdgeSplitOperatorTruthAuthority,
    EdgeSplitRequiredQuerySurface,
};

fn main() {
    let _ = EdgeSplitOperatorRow {
        operator_name: "FakeSplitEdge",
        classification: EdgeSplitOperatorClassification::PreparedSpatialOnly,
        truth_authority: EdgeSplitOperatorTruthAuthority::WorthSpatialPrepared,
        required_query_surface: EdgeSplitRequiredQuerySurface::None,
        topology_precedent: None,
        proof_obligations: &[],
        support_warning: None,
    };
}
