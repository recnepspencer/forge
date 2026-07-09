use worth_query::facade::{
    DeclarativeLiveQueryRequest, WorthQueryEntity, WorthQueryLiveArtifactTarget,
    WorthQueryLivePatch, WorthQueryLiveViewHandle, WorthQueryMutationReceipt,
    WorthQueryRuntimeSourceAdapter, WorthQueryWorkspaceError, QuerySchemaView,
};

struct StringSnapshotSource;

impl WorthQueryRuntimeSourceAdapter for StringSnapshotSource {
    fn declare_live_view(
        &mut self,
        _name: String,
        _request: DeclarativeLiveQueryRequest,
        _schema_view: QuerySchemaView,
    ) -> Result<WorthQueryLiveViewHandle, WorthQueryWorkspaceError> {
        panic!("not executed")
    }

    fn live_entities_for_target(
        &self,
        _target: &WorthQueryLiveArtifactTarget,
    ) -> Vec<WorthQueryEntity> {
        Vec::new()
    }

    fn drain_live_patches_for_target(
        &mut self,
        _target: &WorthQueryLiveArtifactTarget,
    ) -> Vec<WorthQueryLivePatch> {
        Vec::new()
    }

    fn affected_live_view_ids(&self, _receipt: &WorthQueryMutationReceipt) -> Vec<String> {
        Vec::new()
    }

    fn snapshot_token(&self) -> String {
        "snapshot-1".to_string()
    }
}

fn main() {}
