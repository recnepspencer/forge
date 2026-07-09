use worth_query::facade::{
    WorthQueryDerivedView, WorthQueryMutationMetadata,
    WorthQueryRuntimeDeclarationInitializationAdapter, WorthQueryWorkspaceError,
};

struct StringSnapshotInitializationAdapter;

impl WorthQueryRuntimeDeclarationInitializationAdapter for StringSnapshotInitializationAdapter {
    fn declaration_initialization_metadata(
        &self,
        _view: &WorthQueryDerivedView,
        _snapshot_token: &str,
    ) -> Result<WorthQueryMutationMetadata, WorthQueryWorkspaceError> {
        panic!("not executed")
    }
}

fn main() {}
