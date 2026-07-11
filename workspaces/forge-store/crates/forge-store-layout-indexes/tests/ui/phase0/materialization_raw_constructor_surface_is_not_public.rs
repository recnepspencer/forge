use forge_store_layout_indexes::access_planning::{
    S8CoverageBasisKind, S8CoverageGapWitness, S8LayoutCoverageWitness,
    S8LayoutMaterializationState, S8LayoutWatermark,
};
use forge_store_layout_indexes::layout_families::layout_declarations;

fn main() {
    let family = layout_declarations().seed_family().family();
    let gap = S8CoverageGapWitness::physical_range(family, S8CoverageBasisKind::WalLsn, 1, 2);
    let state = S8LayoutMaterializationState::exact(family);
    let watermark = S8LayoutWatermark::root_epoch(7);
    let _coverage = S8LayoutCoverageWitness::partially_covered(state, watermark, watermark, gap);
}
