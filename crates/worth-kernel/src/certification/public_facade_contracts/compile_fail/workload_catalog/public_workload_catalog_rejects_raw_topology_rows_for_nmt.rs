use topology::facade::TopologyView;
use worth_kernel::workload_composition::WorkloadCatalog;

fn main() {
    let raw_topology_rows = TopologyView::default();
    let _ = WorkloadCatalog::from_topology_construction(raw_topology_rows);
}
