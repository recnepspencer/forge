use worth_kernel::workload_composition::WorthWorkload;

fn select_with_copied_text(workload: &WorthWorkload) {
    let _ = workload.select_query_graph_obligations("copied touch descriptor");
}

fn main() {}
