use forge_query::facade::ForgeQueryReadReceipt;
use worth_kernel::graph_read_access_plan_adoption::current_worth_graph_read_access_plan_adoption_phase_two_closeout;

fn main() {
    fn misuse(receipt: &ForgeQueryReadReceipt) {
        let _ = current_worth_graph_read_access_plan_adoption_phase_two_closeout(receipt);
    }

    let _ = misuse;
}
