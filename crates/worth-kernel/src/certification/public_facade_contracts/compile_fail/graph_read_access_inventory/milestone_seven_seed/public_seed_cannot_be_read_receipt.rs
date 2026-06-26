use forge_query::facade::ForgeQueryReadReceipt;
use worth_kernel::graph_read_access_inventory::WorthGraphReadAccessMilestoneSevenSeed;

fn main() {
    fn misuse(seed: WorthGraphReadAccessMilestoneSevenSeed) {
        let _receipt: ForgeQueryReadReceipt = seed;
    }

    let _ = misuse;
}
