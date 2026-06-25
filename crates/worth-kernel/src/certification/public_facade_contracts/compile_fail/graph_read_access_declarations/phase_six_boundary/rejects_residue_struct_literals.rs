use worth_kernel::graph_read_access_declarations::WorthGraphReadDeclarationCappedResidueRow;

fn main() {
    let _ = WorthGraphReadDeclarationCappedResidueRow {
        source_path: String::new(),
        owner: String::new(),
        blocker: String::new(),
        removal_trigger: String::new(),
        current_count: 0,
        must_not_exceed_count: 0,
        row_digest: String::new(),
    };
}
