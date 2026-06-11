use worth_spatial::facade::bindings::PrimitiveRebindingRetainedFactSource;
use worth_spatial::facade::neighborhood::{
    primitive_rebinding_neighborhood_replacement_facts,
    primitive_rebinding_neighborhood_replacement_source, topology_neighborhood_replacement_entry,
    PrimitiveRebindingNeighborhoodReplacementSource,
    TopologyNeighborhoodReplacementDeclarationFamily, TopologyNeighborhoodReplacementEntry,
    TopologyNeighborhoodReplacementFactReceipt,
};
use worth_spatial::facade::projection::{
    geometry_projection_consumption_entry, primitive_rebinding_geometry_projection_consumption,
    GeometryProjectionConsumptionDeclarationFamily, GeometryProjectionConsumptionEntry,
    GeometryProjectionConsumptionReceipt,
};
use worth_spatial::facade::rebinding::PrimitiveRebindingQueryWorld;

#[test]
fn spatial_public_facade_exports_neighborhood_and_projection_family_surfaces() {
    let _: TopologyNeighborhoodReplacementDeclarationFamily =
        TopologyNeighborhoodReplacementDeclarationFamily;
    let _: GeometryProjectionConsumptionDeclarationFamily =
        GeometryProjectionConsumptionDeclarationFamily;

    let _: fn(
        PrimitiveRebindingNeighborhoodReplacementSource,
    ) -> TopologyNeighborhoodReplacementEntry = topology_neighborhood_replacement_entry;
    let _ = primitive_rebinding_neighborhood_replacement_source::<PrimitiveRebindingQueryWorld>;
    let _: fn(PrimitiveRebindingRetainedFactSource) -> GeometryProjectionConsumptionEntry =
        geometry_projection_consumption_entry;

    let _: Option<TopologyNeighborhoodReplacementFactReceipt> = None;
    let _: Option<GeometryProjectionConsumptionReceipt> = None;

    let _ = primitive_rebinding_neighborhood_replacement_facts::<PrimitiveRebindingQueryWorld>;
    let _ = primitive_rebinding_geometry_projection_consumption::<PrimitiveRebindingQueryWorld>;
}
