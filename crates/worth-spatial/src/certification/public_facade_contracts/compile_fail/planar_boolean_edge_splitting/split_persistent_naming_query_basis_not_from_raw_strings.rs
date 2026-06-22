use worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitPersistentNamingQueryBasis;

fn main() {
    let _ = PlanarBooleanSplitPersistentNamingQueryBasis::from_query_runtime(
        "worth.topology/current_head_authoritative",
        "persistent-name-live-view:split",
        "naming-attachment-report:split",
    );
}
