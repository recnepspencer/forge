use worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitEdgeChainLedgerDeclaration;

fn main() {
    let _declaration = PlanarBooleanSplitEdgeChainLedgerDeclaration::from_product_identities(
        "request",
        "validation",
        "naming",
        "decisions",
    );
}
