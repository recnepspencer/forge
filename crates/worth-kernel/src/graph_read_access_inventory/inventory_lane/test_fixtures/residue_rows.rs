use super::super::{
    WorthGraphReadAccessCappedResidueBuilder, WorthGraphReadAccessCappedResidueRow,
    WorthGraphReadAccessOwner,
};

pub(crate) fn capped_residue_row() -> WorthGraphReadAccessCappedResidueBuilder {
    WorthGraphReadAccessCappedResidueRow::builder()
        .source_path("crates/worth-kernel/src/query_adoption/graph_read_access")
        .owner(WorthGraphReadAccessOwner::WorthKernel)
        .current_count(1)
        .must_not_exceed_count(1)
        .blocker("Milestone 7 declaration seed must replace old graph-read adoption")
        .removal_trigger("Milestone 7 declaration candidate ledger owns this path")
}
