use super::{
    in_memory_test_runtime, WorthQueryTestBackendError, WorthQueryTestBackendErrorKind,
    WorthQueryTestBackendSchema,
};

#[test]
fn in_memory_test_runtime_validates_schema_denials_before_workspace_creation() {
    assert_schema_error(
        WorthQueryTestBackendSchema::single_collection("Task")
            .aspect("title.value", "title.value")
            .expect("first aspect should be accepted")
            .aspect("title.value", "task_title"),
        WorthQueryTestBackendErrorKind::DuplicateAspectLabel,
    );
    assert_schema_error(
        WorthQueryTestBackendSchema::single_collection("Task")
            .aspect("title.value", "title.value")
            .expect("first aspect should be accepted")
            .aspect("task_title", "title.value"),
        WorthQueryTestBackendErrorKind::DuplicateProjectionPath,
    );
    assert_schema_error(
        WorthQueryTestBackendSchema::single_collection(" ").aspect("title.value", "title.value"),
        WorthQueryTestBackendErrorKind::BlankCollectionName,
    );
    assert_schema_error(
        WorthQueryTestBackendSchema::single_collection("Task").aspect(" ", "title.value"),
        WorthQueryTestBackendErrorKind::BlankAspectLabel,
    );
    assert_schema_error(
        WorthQueryTestBackendSchema::single_collection("Task").aspect("title.value", " "),
        WorthQueryTestBackendErrorKind::BlankProjectionPath,
    );
    assert_schema_error(
        WorthQueryTestBackendSchema::single_collection("Task").aspect("title.value", "title value"),
        WorthQueryTestBackendErrorKind::InvalidProjectionPath,
    );
    assert_eq!(
        workspace_build_error_kind(
            in_memory_test_runtime().workspace("consumer-kit.test-backend.missing-schema")
        ),
        WorthQueryTestBackendErrorKind::MissingSchema
    );
    assert_eq!(
        workspace_build_error_kind(
            in_memory_test_runtime()
                .with_schema(WorthQueryTestBackendSchema::single_collection("Task"))
                .workspace("consumer-kit.test-backend.empty-schema")
        ),
        WorthQueryTestBackendErrorKind::EmptyAspectSet
    );
}

fn assert_schema_error(
    result: Result<WorthQueryTestBackendSchema, WorthQueryTestBackendError>,
    expected: WorthQueryTestBackendErrorKind,
) {
    match result {
        Ok(_) => panic!("schema declaration should fail"),
        Err(error) => assert_eq!(error.kind(), expected),
    }
}

fn workspace_build_error_kind(
    result: Result<crate::runtime::WorthQueryWorkspace, WorthQueryTestBackendError>,
) -> WorthQueryTestBackendErrorKind {
    match result {
        Ok(_) => panic!("workspace build should fail"),
        Err(error) => error.kind(),
    }
}
