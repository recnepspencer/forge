use worth_store_layout_indexes::access_planning::{
    CoverageBasisKind, CoverageGapWitness, LayoutCoverageWitness, LayoutMaterializationState,
    LayoutWatermark,
};
use worth_store_layout_indexes::declarations::layout_declarations;

fn main() {
    let family = layout_declarations().seed_family().family();
    let gap = CoverageGapWitness::physical_range(family, CoverageBasisKind::WalLsn, 1, 2);
    let state = LayoutMaterializationState::exact(family);
    let watermark = LayoutWatermark::root_epoch(7);
    let _coverage = LayoutCoverageWitness::partially_covered(state, watermark, watermark, gap);
}
