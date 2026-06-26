use topology::derived_invalidation_migrated_products::MaterializedGraphReadSource;
use topology::facade::TopologyView;

fn main() {
    fn attempt(topology: &TopologyView) {
        let _ = MaterializedGraphReadSource::from_topology_view_with_selected_prefix(topology, 0, 0);
    }

    let _ = attempt;
}
