use worth_query_execution::facade::primary_graph::WorthQueryApplicationCommitReceipt;

fn cannot_splice_exact_evidence(
    left: WorthQueryApplicationCommitReceipt,
    right: WorthQueryApplicationCommitReceipt,
) {
    let _detached_axes = (
        left.mutation_work,
        left.retained_preimage,
        right.committed_dispatch_outbox,
    );
}

fn main() {}
