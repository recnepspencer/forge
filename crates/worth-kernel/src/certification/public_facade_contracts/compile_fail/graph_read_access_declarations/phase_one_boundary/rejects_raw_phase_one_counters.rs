use worth_kernel::graph_read_access_declarations::{
    current_worth_graph_read_access_declaration_phase_one_closeout_from_milestone_six,
    WorthGraphReadAccessDeclarationPhaseOneCounters,
};

fn main() {
    fn misuse(counters: &WorthGraphReadAccessDeclarationPhaseOneCounters) {
        let _ = current_worth_graph_read_access_declaration_phase_one_closeout_from_milestone_six(
            counters,
        );
    }

    let _ = misuse;
}
