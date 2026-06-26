use forge_query::facade::runtime::ForgeQueryReadReceipt;
use worth_kernel::graph_read_access_declarations::current_worth_graph_read_access_declaration_closeout;

fn main() {
    fn misuse(receipt: &ForgeQueryReadReceipt) {
        let _ = current_worth_graph_read_access_declaration_closeout(receipt);
    }

    let _ = misuse;
}
