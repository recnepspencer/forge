use forge_query::facade::consumer_kit::ForgeQueryGraphObligationExecutionBackedAdoptionProof;
use worth_spatial::facade::query_adoption::WorthSpatialQueryConsumerKitAdoptionStatus;

fn require_query_graph_authority(_: ForgeQueryGraphObligationExecutionBackedAdoptionProof) {}

fn main() {
    let spatial_status: WorthSpatialQueryConsumerKitAdoptionStatus = panic!();

    require_query_graph_authority(spatial_status);
}
