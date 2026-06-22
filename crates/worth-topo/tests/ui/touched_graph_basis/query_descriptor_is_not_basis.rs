use topology::facade::{
    topology_operator_relation_touch_descriptor, TopologyTouchedGraphBasis,
};

fn requires_basis(_: &TopologyTouchedGraphBasis) {}

fn main() {
    let descriptor = topology_operator_relation_touch_descriptor().unwrap();
    requires_basis(&descriptor);
}
