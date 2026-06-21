use schema::facade::platform::authority::{MutationOrigin, RawTopologyIntent};
use topology::facade::TopologyTouchedGraphBasis;

fn requires_basis(_: &TopologyTouchedGraphBasis) {}

fn main() {
    let intent = RawTopologyIntent::new(Vec::new(), MutationOrigin::LocalEdit);
    requires_basis(&intent);
}
