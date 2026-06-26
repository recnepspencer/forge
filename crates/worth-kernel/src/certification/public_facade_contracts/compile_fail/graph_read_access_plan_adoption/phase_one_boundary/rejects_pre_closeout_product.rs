use worth_kernel::graph_read_access_declarations::WorthGraphReadAccessDeclarationCloseout;
use worth_kernel::graph_read_access_plan_adoption::current_worth_graph_read_access_plan_adoption_phase_one_closeout;

fn main() {
    fn misuse(declaration_closeout: &WorthGraphReadAccessDeclarationCloseout) {
        let _ =
            current_worth_graph_read_access_plan_adoption_phase_one_closeout(declaration_closeout);
    }

    let _ = misuse;
}
