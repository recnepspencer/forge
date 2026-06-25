use topology::derived_invalidation_migrated_products::TraversalViewsReadSource;
use topology::facade::TopologyView;

fn main() {
    fn attempt(topology: &TopologyView) {
        let _ = TraversalViewsReadSource::from_topology_view_with_selected_prefix(topology, 0);
    }

    let _ = attempt;
}
