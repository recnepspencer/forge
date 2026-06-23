use worth_kernel::workload_composition::WorthWorkload;

fn select_with_copied_parts(workload: &WorthWorkload) {
    let _ = workload.select_query_graph_obligations(("copied authority digest", 1usize));
}

fn main() {}
