pub(super) fn bulk_continuity_member_identity(
    lineage_context: &crate::continuity::BridgeLineageContext,
) -> BulkContinuityMemberIdentity {
    let authority_basis = lineage_context.authority_basis();
    BulkContinuityMemberIdentity::new(digest_string(
        "bulk-continuity-member",
        &format!(
            "bulk-continuity-member|authority={}|branch={}|snapshot={}",
            authority_basis.digest(),
            authority_basis.branch_identity().as_str(),
            authority_basis.snapshot_identity().as_str()
        ),
    ))
}

pub(super) fn bulk_truth_view_member_identity(
    route: &BridgePlannedRoute,
) -> BulkTruthViewMemberIdentity {
    BulkTruthViewMemberIdentity::new(digest_string(
        "bulk-truth-view-member",
        &format!(
            "bulk-truth-view-member|branch={}|snapshot={}|commit={}",
            route.source_branch().as_str(),
            route.source_snapshot().as_str(),
            route.source_commit().as_str()
        ),
    ))
}

use super::*;
