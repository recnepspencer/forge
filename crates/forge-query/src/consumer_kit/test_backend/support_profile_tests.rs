use crate::runtime::{
    ForgeQueryEffectPolicy, ForgeQueryRuntimeError, ForgeQueryRuntimeFacadeFamily,
    ForgeQueryRuntimeFamilySupportStatus,
};

use super::support_profile::in_memory_test_backend_support_profile;
use super::{in_memory_test_runtime, ForgeQueryTestBackendSchema};

#[test]
fn in_memory_test_runtime_support_profile_is_honest_about_debt_rows() {
    let workspace = task_workspace();
    let matrix = workspace.public_support_matrix();

    assert_eq!(
        matrix_row_status(&matrix, ForgeQueryRuntimeFacadeFamily::Read),
        ForgeQueryRuntimeFamilySupportStatus::Supported
    );
    assert_eq!(
        matrix_row_status(&matrix, ForgeQueryRuntimeFacadeFamily::Write),
        ForgeQueryRuntimeFamilySupportStatus::Supported
    );
    assert_eq!(
        matrix_row_status(&matrix, ForgeQueryRuntimeFacadeFamily::Intent),
        ForgeQueryRuntimeFamilySupportStatus::Unsupported
    );
    assert_eq!(
        matrix_row_status(&matrix, ForgeQueryRuntimeFacadeFamily::StoreBackedExecution),
        ForgeQueryRuntimeFamilySupportStatus::DeferredDebt
    );
    assert_eq!(
        in_memory_test_backend_support_profile()
            .support_for(ForgeQueryRuntimeFacadeFamily::BranchPreview)
            .expect("branch-preview row should exist")
            .effect_policies(),
        &[
            ForgeQueryEffectPolicy::DeriveOnly,
            ForgeQueryEffectPolicy::SandboxedWriteIntent
        ]
    );
    for row in matrix.rows() {
        if row.status() != ForgeQueryRuntimeFamilySupportStatus::Supported {
            assert!(
                row.admission_fail_closed(),
                "row should fail closed: {row:?}"
            );
            assert!(
                !row.extension_rule().is_empty(),
                "row should expose extension rule: {row:?}"
            );
        }
    }
}

#[test]
fn in_memory_test_runtime_support_matrix_denies_unsupported_public_families() {
    let workspace = task_workspace();
    for family in unsupported_public_families() {
        let error = workspace
            .admit_public_api_family(family)
            .expect_err("unsupported/deferred family should fail closed");
        match error {
            ForgeQueryRuntimeError::UnsupportedFacadeFamily(denial) => {
                assert_eq!(denial.family(), family);
            }
            other => panic!("expected unsupported facade family denial, got {other:?}"),
        }
    }
}

fn task_workspace() -> crate::runtime::ForgeQueryWorkspace {
    in_memory_test_runtime()
        .with_schema(task_schema())
        .workspace("consumer-kit.test-backend.support-profile")
        .expect("in-memory test runtime should build")
}

fn task_schema() -> ForgeQueryTestBackendSchema {
    ForgeQueryTestBackendSchema::single_collection("Task")
        .aspect("identity.id", "identity.id")
        .expect("identity aspect")
        .aspect("title.value", "title.value")
        .expect("title aspect")
}

fn matrix_row_status(
    matrix: &crate::runtime::ForgeQueryRuntimePublicSupportMatrix,
    family: ForgeQueryRuntimeFacadeFamily,
) -> ForgeQueryRuntimeFamilySupportStatus {
    matrix
        .rows()
        .iter()
        .find(|row| row.facade_family() == Some(family))
        .unwrap_or_else(|| panic!("support matrix should include `{family}`"))
        .status()
}

fn unsupported_public_families() -> [ForgeQueryRuntimeFacadeFamily; 10] {
    [
        ForgeQueryRuntimeFacadeFamily::Computed,
        ForgeQueryRuntimeFacadeFamily::SharedRead,
        ForgeQueryRuntimeFacadeFamily::Replay,
        ForgeQueryRuntimeFacadeFamily::Effect,
        ForgeQueryRuntimeFacadeFamily::Intent,
        ForgeQueryRuntimeFacadeFamily::Temporal,
        ForgeQueryRuntimeFacadeFamily::AsyncResource,
        ForgeQueryRuntimeFacadeFamily::MixedCauseDelivery,
        ForgeQueryRuntimeFacadeFamily::StoreBackedExecution,
        ForgeQueryRuntimeFacadeFamily::DurableArtifacts,
    ]
}
