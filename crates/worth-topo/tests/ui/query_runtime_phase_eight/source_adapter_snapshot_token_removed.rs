use forge_query::facade::{
    DeclarativeLiveQueryRequest, ForgeQueryEntity, ForgeQueryLivePatch, ForgeQueryLiveViewHandle,
    ForgeQueryMutationReceipt, ForgeQueryRuntimeSourceAdapter, ForgeQueryWorkspaceError,
    QuerySchemaView,
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

    fn live_entities(&self, _view_name: &str) -> Vec<ForgeQueryEntity> {
        Vec::new()
    }

    fn drain_live_patches(&mut self, _view_name: &str) -> Vec<ForgeQueryLivePatch> {
        Vec::new()
    }

    fn affected_live_view_ids(&self, _receipt: &ForgeQueryMutationReceipt) -> Vec<String> {
        Vec::new()
    }

    fn snapshot_token(&self) -> String {
        "snapshot-1".to_string()
    }
}

fn main() {}
