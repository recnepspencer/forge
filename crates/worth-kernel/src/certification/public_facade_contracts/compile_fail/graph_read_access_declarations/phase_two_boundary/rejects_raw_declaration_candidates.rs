use worth_kernel::graph_read_access_declarations::current_worth_graph_read_access_declaration_catalog_closeout;
use worth_kernel::graph_read_access_inventory::WorthGraphReadDeclarationCandidate;

fn main() {
    fn misuse(candidates: &[WorthGraphReadDeclarationCandidate]) {
        let _ = current_worth_graph_read_access_declaration_catalog_closeout(candidates);
    }

    let _ = misuse;
}
