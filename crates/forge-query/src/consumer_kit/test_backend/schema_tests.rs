use super::{
    in_memory_test_runtime, ForgeQueryTestBackendError, ForgeQueryTestBackendErrorKind,
    ForgeQueryTestBackendSchema,
};

#[test]
fn in_memory_test_runtime_validates_schema_denials_before_workspace_creation() {
    assert_schema_error(
        ForgeQueryTestBackendSchema::single_collection("Task")
            .aspect("title.value", "title.value")
            .expect("first aspect should be accepted")
            .aspect("title.value", "task_title"),
        ForgeQueryTestBackendErrorKind::DuplicateAspectLabel,
    );
    assert_schema_error(
        ForgeQueryTestBackendSchema::single_collection("Task")
            .aspect("title.value", "title.value")
            .expect("first aspect should be accepted")
            .aspect("task_title", "title.value"),
        ForgeQueryTestBackendErrorKind::DuplicateProjectionPath,
    );
    assert_schema_error(
        ForgeQueryTestBackendSchema::single_collection(" ").aspect("title.value", "title.value"),
        ForgeQueryTestBackendErrorKind::BlankCollectionName,
    );
    assert_schema_error(
        ForgeQueryTestBackendSchema::single_collection("Task").aspect(" ", "title.value"),
        ForgeQueryTestBackendErrorKind::BlankAspectLabel,
    );
    assert_schema_error(
        ForgeQueryTestBackendSchema::single_collection("Task").aspect("title.value", " "),
        ForgeQueryTestBackendErrorKind::BlankProjectionPath,
    );
    assert_eq!(
        workspace_build_error_kind(
            in_memory_test_runtime().workspace("consumer-kit.test-backend.missing-schema")
        ),
        ForgeQueryTestBackendErrorKind::MissingSchema
    );
    assert_eq!(
        workspace_build_error_kind(
            in_memory_test_runtime()
                .with_schema(ForgeQueryTestBackendSchema::single_collection("Task"))
                .workspace("consumer-kit.test-backend.empty-schema")
        ),
        ForgeQueryTestBackendErrorKind::EmptyAspectSet
    );
}

fn assert_schema_error(
    result: Result<ForgeQueryTestBackendSchema, ForgeQueryTestBackendError>,
    expected: ForgeQueryTestBackendErrorKind,
) {
    match result {
        Ok(_) => panic!("schema declaration should fail"),
        Err(error) => assert_eq!(error.kind(), expected),
    }
}

fn workspace_build_error_kind(
    result: Result<crate::runtime::ForgeQueryWorkspace, ForgeQueryTestBackendError>,
) -> ForgeQueryTestBackendErrorKind {
    match result {
        Ok(_) => panic!("workspace build should fail"),
        Err(error) => error.kind(),
    }
}
