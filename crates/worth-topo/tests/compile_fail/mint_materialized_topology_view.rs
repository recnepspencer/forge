use worth_topo::facade::{
    MaterializationBreadthReport, MaterializationReport, MaterializedTopologyView,
    WorthTopologyView,
};

fn main() {
    let topology = WorthTopologyView::default();
    let report = MaterializationReport {
        breadth: MaterializationBreadthReport {
            entity_count: 0,
            relation_count: 0,
            topology_entity_count: 0,
            topology_relation_count: 0,
        },
        whole_view_materialization: true,
        fallback_class: None,
    };

    let _forged = MaterializedTopologyView { topology, report };
}
