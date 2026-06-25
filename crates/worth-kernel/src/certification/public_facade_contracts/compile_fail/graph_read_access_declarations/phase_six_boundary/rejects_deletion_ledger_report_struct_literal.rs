use worth_kernel::graph_read_access_declarations::WorthGraphReadDeclarationDeletionLedgerReport;

fn main() {
    let _ = WorthGraphReadDeclarationDeletionLedgerReport {
        rows: Vec::new(),
        deleted_count: 0,
        capped_residue_count: 0,
        report_digest: String::new(),
    };
}
