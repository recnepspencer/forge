use std::marker::PhantomData;

use topology::facade::TopologyTouchedGraphBasis;
use worth_spatial::facade::workload_vocabulary::lower_spatial_touch_authority_to_query_descriptor;

fn main() {
    let topology_basis = PhantomData::<TopologyTouchedGraphBasis>;
    let _ = lower_spatial_touch_authority_to_query_descriptor(&topology_basis, &topology_basis);
}
