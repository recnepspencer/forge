use forge_query::facade::{
    DeclarativeLiveQueryRequest, ForgeQueryEntity, ForgeQueryLivePatch, ForgeQueryLiveViewHandle,
    ForgeQueryLiveArtifactTarget, ForgeQueryMutationReceipt, ForgeQueryRuntimeSourceAdapter,
    ForgeQueryWorkspaceError, QuerySchemaView,
};

struct SnapshotTokenSource;

impl ForgeQueryRuntimeSourceAdapter for SnapshotTokenSource {
    fn declare_live_view(
        &mut self,
        _name: String,
        _request: DeclarativeLiveQueryRequest,
        _schema_view: QuerySchemaView,
    ) -> Result<ForgeQueryLiveViewHandle, ForgeQueryWorkspaceError> {
        panic!("not executed")
    }

    fn live_entities_for_target(&self, _target: &ForgeQueryLiveArtifactTarget) -> Vec<ForgeQueryEntity> {
        Vec::new()
    }

    fn drain_live_patches_for_target(&mut self, _target: &ForgeQueryLiveArtifactTarget) -> Vec<ForgeQueryLivePatch> {
        Vec::new()
    }

    fn affected_live_view_targets(
        &self,
        _receipt: &ForgeQueryMutationReceipt,
    ) -> Vec<ForgeQueryLiveArtifactTarget> {
        Vec::new()
    }

    fn snapshot_token(&self) -> String {
        "snapshot-1".to_string()
    }
}

fn main() {}
