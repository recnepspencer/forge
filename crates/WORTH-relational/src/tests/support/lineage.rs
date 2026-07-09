use crate::facade::history::BranchId;
use crate::facade::identity::LineageId;
use crate::facade::lineage::CorrespondenceCandidate;
use crate::facade::runtime::RelationalRuntime;

pub(crate) fn record_lineage_candidate(
    runtime: &mut RelationalRuntime,
    branch_id: BranchId,
    sources: Vec<LineageId>,
    targets: Vec<LineageId>,
    note: &str,
) -> CorrespondenceCandidate {
    runtime
        .lineage_authority()
        .record_correspondence_candidate(branch_id, sources, targets, note)
}
