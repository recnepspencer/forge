use worth_store_layout_indexes::{
    layout_declarations, S8CoverageBasisKind, S8CoverageGapWitness, S8LayoutCoverageWitness,
    S8LayoutMaterializationState, S8LayoutWatermark,
};

fn main() {
    let family = layout_declarations().seed_family().family();
    let gap = S8CoverageGapWitness::physical_range(family, S8CoverageBasisKind::WalLsn, 1, 2);
    let state = S8LayoutMaterializationState::exact(family);
    let watermark = S8LayoutWatermark::root_epoch(7);
    let _coverage = S8LayoutCoverageWitness::partially_covered(state, watermark, watermark, gap);
}
