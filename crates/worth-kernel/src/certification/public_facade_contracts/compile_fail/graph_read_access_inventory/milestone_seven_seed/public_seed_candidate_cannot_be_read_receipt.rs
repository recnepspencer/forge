use forge_query::facade::ForgeQueryReadReceipt;
use worth_kernel::graph_read_access_inventory::WorthGraphReadAccessMilestoneSevenSeed;

fn main() {
    fn misuse(seed: &WorthGraphReadAccessMilestoneSevenSeed) {
        let candidate = seed.declaration_candidates()[0].clone();
        let _receipt: ForgeQueryReadReceipt = candidate.clone();
    }

    let _ = misuse;
}
