use forge_store_layout_indexes::S8ExecutedAccessEvidence;
use forge_store_physical_certification::harness::by_milestone::s8_layout_access::S8LayoutAccessHarness;

fn require_executed(_: S8ExecutedAccessEvidence) {}

fn main() {
    let harness: S8LayoutAccessHarness = todo!();
    require_executed(harness);
}
