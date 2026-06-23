use forge_query::facade::{
    ForgeQueryDerivedView, ForgeQueryMutationMetadata,
    ForgeQueryRuntimeDeclarationInitializationAdapter, ForgeQueryWorkspaceError,
};

struct StringSnapshotInitializationAdapter;

impl ForgeQueryRuntimeDeclarationInitializationAdapter for StringSnapshotInitializationAdapter {
    fn declaration_initialization_metadata(
        &self,
        _view: &ForgeQueryDerivedView,
        _snapshot_token: &str,
    ) -> Result<ForgeQueryMutationMetadata, ForgeQueryWorkspaceError> {
        panic!("not executed")
    }
}

fn main() {}
