use worth_kernel::graph_read_access_plan_adoption::current_worth_graph_read_access_plan_adoption_phase_one_closeout;

fn main() {
    fn misuse(read_family_digest: String) {
        let _ = current_worth_graph_read_access_plan_adoption_phase_one_closeout(
            &read_family_digest,
        );
    }

    let _ = misuse;
}
