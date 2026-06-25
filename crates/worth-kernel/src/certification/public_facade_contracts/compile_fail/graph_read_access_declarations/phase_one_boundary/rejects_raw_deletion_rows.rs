use worth_kernel::graph_read_access_declarations::current_worth_graph_read_access_declaration_phase_one_closeout_from_milestone_six;
use worth_kernel::graph_read_access_inventory::WorthGraphReadDeletionLedgerItem;

fn main() {
    fn misuse(deletion_rows: &[WorthGraphReadDeletionLedgerItem]) {
        let _ = current_worth_graph_read_access_declaration_phase_one_closeout_from_milestone_six(
            deletion_rows,
        );
    }

    let _ = misuse;
}
