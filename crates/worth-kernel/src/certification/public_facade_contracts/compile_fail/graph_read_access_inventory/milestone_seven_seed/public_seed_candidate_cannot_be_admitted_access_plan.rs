use forge_query::facade::ForgeQueryAdmittedGraphReadAccessPlan;
use worth_kernel::graph_read_access_inventory::WorthGraphReadAccessMilestoneSevenSeed;

fn main() {
    fn misuse(seed: &WorthGraphReadAccessMilestoneSevenSeed) {
        let candidate = seed.declaration_candidates()[0].clone();
        let _plan: ForgeQueryAdmittedGraphReadAccessPlan = candidate.clone();
    }

    let _ = misuse;
}
