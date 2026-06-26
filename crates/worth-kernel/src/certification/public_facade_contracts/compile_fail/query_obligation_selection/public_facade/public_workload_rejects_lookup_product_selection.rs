use worth_kernel::workload_composition::WorthWorkload;
use worth_spatial::facade::workload_vocabulary::SpatialEvidenceLookupProduct;

fn select_with_lookup_product(workload: &WorthWorkload, lookup: &SpatialEvidenceLookupProduct) {
    let _ = workload.select_query_graph_obligations(lookup);
}

fn main() {}
