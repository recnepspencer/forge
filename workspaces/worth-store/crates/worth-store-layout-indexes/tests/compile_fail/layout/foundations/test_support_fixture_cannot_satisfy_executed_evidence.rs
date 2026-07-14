use worth_store_layout_indexes::S8ExecutedAccessEvidence;
use worth_store_test_support::NativeStoreAspectFixture;

fn require_executed(_: S8ExecutedAccessEvidence) {}

fn main() {
    let support: NativeStoreAspectFixture = todo!();
    require_executed(support);
}
