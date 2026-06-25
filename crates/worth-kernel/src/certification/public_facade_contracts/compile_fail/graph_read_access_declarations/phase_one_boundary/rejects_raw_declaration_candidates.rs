use worth_kernel::graph_read_access_declarations::current_worth_graph_read_access_declaration_phase_one_closeout_from_milestone_six;
use worth_kernel::graph_read_access_inventory::WorthGraphReadDeclarationCandidate;

fn main() {
    fn misuse(candidates: &[WorthGraphReadDeclarationCandidate]) {
        let _ = current_worth_graph_read_access_declaration_phase_one_closeout_from_milestone_six(
            candidates,
        );
    }

    let _ = misuse;
}
