use super::*;

pub(crate) fn current_graph_request(
    partition_scope: Option<Vec<PartitionId>>,
    relation_kind_scope: Option<Vec<KindId>>,
    summary_only: bool,
) -> crate::facade::inspection::GraphInspectionRequest {
    crate::facade::inspection::GraphInspectionRequest {
        scope: crate::facade::inspection::InspectionScope::Current,
        partition_scope,
        relation_kind_scope,
        summary_only,
    }
}

pub(crate) fn version_graph_request(
    version_id: crate::facade::identity::VersionId,
    partition_scope: Option<Vec<PartitionId>>,
    relation_kind_scope: Option<Vec<KindId>>,
    summary_only: bool,
) -> crate::facade::inspection::GraphInspectionRequest {
    crate::facade::inspection::GraphInspectionRequest {
        scope: crate::facade::inspection::InspectionScope::Version(version_id),
        partition_scope,
        relation_kind_scope,
        summary_only,
    }
}

pub(crate) fn snapshot_graph_request(
    scope: crate::facade::inspection::InspectionScope,
    partition_scope: Option<Vec<PartitionId>>,
    relation_kind_scope: Option<Vec<KindId>>,
    summary_only: bool,
) -> crate::facade::inspection::GraphInspectionRequest {
    crate::facade::inspection::GraphInspectionRequest {
        scope,
        partition_scope,
        relation_kind_scope,
        summary_only,
    }
}

pub(crate) fn connectivity_request(
    scope: crate::facade::inspection::InspectionScope,
    partition_scope: Option<Vec<PartitionId>>,
    relation_kind_scope: Option<Vec<KindId>>,
    include_members: bool,
) -> crate::facade::inspection::ConnectivityInspectionRequest {
    crate::facade::inspection::ConnectivityInspectionRequest {
        scope,
        partition_scope,
        relation_kind_scope,
        include_members,
    }
}

pub(crate) fn retained_record_inspection(
    runtime: &RelationalRuntime,
    branch_id: &BranchId,
    version_id: crate::facade::identity::VersionId,
    target: RecordRef,
) -> crate::facade::inspection::HistoricalRecordInspection {
    runtime.inspection_access().inspect_historical_record(
        branch_id,
        version_id,
        target,
        crate::facade::inspection::HistoricalInspectionMode::RetainedOnly,
    )
}

pub(crate) fn reconstructed_record_inspection(
    runtime: &RelationalRuntime,
    branch_id: &BranchId,
    version_id: crate::facade::identity::VersionId,
    target: RecordRef,
) -> crate::facade::inspection::HistoricalRecordInspection {
    runtime.inspection_access().inspect_historical_record(
        branch_id,
        version_id,
        target,
        crate::facade::inspection::HistoricalInspectionMode::AllowCanonicalReconstruction,
    )
}

pub(crate) fn recent_commit_window(
    runtime: &RelationalRuntime,
    branch_id: &BranchId,
    limit: usize,
) -> crate::facade::inspection::RecentCommitInspectionWindow {
    runtime
        .inspection_access()
        .inspect_recent_commits(&crate::facade::inspection::RecentCommitInspectionRequest {
            branch_id: Some(branch_id.clone()),
            limit,
        })
}
