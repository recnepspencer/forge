use crate::history::data::BranchId;
use crate::identity::data::VersionId;
use crate::logic::runtime::RelationalRuntime;

pub(crate) fn authoritative_branch_for_version(
    runtime: &RelationalRuntime,
    version_id: VersionId,
) -> BranchId {
    runtime
        .history()
        .committed_version(version_id)
        .map(|summary| summary.commit().branch_id.clone())
        .or_else(|| {
            (version_id == VersionId(0)).then(|| runtime.config().history.main_branch.clone())
        })
        .expect("every visible Relational version has authoritative branch identity")
}
