use worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitDecisionLogDeclaration;

fn main() {
    let _ = PlanarBooleanSplitDecisionLogDeclaration::for_split_products(
        "split request",
        "split chain validation receipt",
        "split persistent naming receipt",
    );
}
