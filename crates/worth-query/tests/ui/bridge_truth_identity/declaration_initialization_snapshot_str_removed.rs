use worth_query::facade::foundation::WorthQueryWorkspaceError;
use worth_query::facade::policy::WorthQueryDerivedView;
use worth_query::facade::runtime::{WorthQueryMutationMetadata, WorthQueryRuntimeDeclarationInitializationAdapter};

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
