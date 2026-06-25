use worth_kernel::graph_read_access_declarations::current_worth_graph_read_declaration_deletion_firewall_closeout;
use worth_kernel::graph_read_access_inventory::WorthGraphReadDeletionLedgerItem;

fn main() {
    fn misuse(rows: &[WorthGraphReadDeletionLedgerItem]) {
        let _ = current_worth_graph_read_declaration_deletion_firewall_closeout(rows);
    }

    let _ = misuse;
}
