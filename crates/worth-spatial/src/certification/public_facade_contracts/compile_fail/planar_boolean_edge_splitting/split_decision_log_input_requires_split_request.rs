use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet,
    PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
    PlanarBooleanSplitChainValidationReceipt, PlanarBooleanSplitDecisionLogDeclaration,
    PlanarBooleanSplitDecisionLogInput, PlanarBooleanSplitEdgeFragmentSet,
    PlanarBooleanSplitPersistentNamingReceipt, PlanarBooleanSplitVertexIdentitySet,
};

fn main() {
    let declaration: PlanarBooleanSplitDecisionLogDeclaration = test_value();
    let endpoint_boundary: &PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet = test_value();
    let interval_subdivision: &PlanarBooleanIntervalSubdivisionNormalizedScheduleSet = test_value();
    let split_vertices: &PlanarBooleanSplitVertexIdentitySet = test_value();
    let split_fragments: &PlanarBooleanSplitEdgeFragmentSet = test_value();
    let chain_validation: &PlanarBooleanSplitChainValidationReceipt = test_value();
    let persistent_naming: &PlanarBooleanSplitPersistentNamingReceipt = test_value();

    let _ = PlanarBooleanSplitDecisionLogInput::from_certified_products(
        declaration,
        endpoint_boundary,
        interval_subdivision,
        split_vertices,
        split_fragments,
        chain_validation,
        persistent_naming,
    );
}

fn test_value<T>() -> T {
    unimplemented!()
}
