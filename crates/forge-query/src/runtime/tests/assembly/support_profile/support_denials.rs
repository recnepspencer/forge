use super::super::super::support::*;

#[test]
fn runtime_support_denies_unsupported_write_family_before_execution() {
    let mut runtime = bridge_runtime_with_support(
        ForgeQueryRuntimeSupportProfile::scaffold_backend_profile().with_family_support(
            ForgeQueryRuntimeFamilySupport::unsupported(
                ForgeQueryRuntimeFacadeFamily::Write,
                "test backend disabled write authority",
            ),
        ),
    );

    let error = runtime
        .write(insert_command(
            "Task",
            [
                ("identity.id", json!("external-1")),
                ("title.value", json!("Should not write")),
            ],
        ))
        .expect_err("unsupported write family should deny before write authority");

    match error {
        ForgeQueryRuntimeError::UnsupportedFacadeFamily(denial) => {
            assert_eq!(denial.family(), ForgeQueryRuntimeFacadeFamily::Write);
            assert_eq!(denial.reason(), "test backend disabled write authority");
        }
        other => panic!("expected unsupported facade family denial, got {other:?}"),
    }
}

#[test]
fn runtime_builder_rejects_support_profiles_that_overclaim_unimplemented_families() {
    let profile = ForgeQueryRuntimeSupportProfile::scaffold_backend_profile().with_family_support(
        ForgeQueryRuntimeFamilySupport::supported(
            ForgeQueryRuntimeFacadeFamily::Intent,
            [ForgeQueryAuthorityLane::PendingWriteIntent],
            [ForgeQueryEffectPolicy::AuthoritativeAllowed],
            ["fake-intent-adapter"],
        ),
    );

    let error = complete_backend_from_parts_builder()
        .support_profile(profile)
        .build_backend_from_parts()
        .build();
    let error = match error {
        Ok(_) => panic!("support profile must not claim unimplemented facade support"),
        Err(error) => error,
    };

    match error {
        ForgeQueryRuntimeError::UnsupportedFacadeFamily(denial) => {
            assert_eq!(denial.family(), ForgeQueryRuntimeFacadeFamily::Intent);
            assert!(denial.reason().contains("intent authority adapter"));
        }
        other => panic!("expected unsupported facade family denial, got {other:?}"),
    }
}

#[test]
fn runtime_support_denies_unsupported_computed_family_before_registration() {
    let mut runtime = bridge_runtime_with_support(
        ForgeQueryRuntimeSupportProfile::scaffold_backend_profile().with_family_support(
            ForgeQueryRuntimeFamilySupport::unsupported(
                ForgeQueryRuntimeFacadeFamily::Computed,
                "test backend disabled computed resources",
            ),
        ),
    );

    let error = runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("task_titles.unsupported", ["title".to_string()]),
            TitleListMaintainer,
        )
        .expect_err("unsupported computed family should deny before registration");

    match error {
        ForgeQueryRuntimeError::UnsupportedFacadeFamily(denial) => {
            assert_eq!(denial.family(), ForgeQueryRuntimeFacadeFamily::Computed);
            assert_eq!(denial.reason(), "test backend disabled computed resources");
        }
        other => panic!("expected unsupported facade family denial, got {other:?}"),
    }
}

#[test]
fn runtime_support_denies_unsupported_preview_and_branch_sessions_without_panicking() {
    let mut runtime = bridge_runtime_with_support(
        ForgeQueryRuntimeSupportProfile::scaffold_backend_profile().with_family_support(
            ForgeQueryRuntimeFamilySupport::unsupported(
                ForgeQueryRuntimeFacadeFamily::BranchPreview,
                "test backend disabled branch and preview sessions",
            ),
        ),
    );

    let preview_error = match runtime.preview(test_session_label("unsupported preview")) {
        Ok(_) => panic!("unsupported preview should return a typed denial"),
        Err(error) => error,
    };
    let branch_error = match runtime.branch(test_session_label("unsupported branch")) {
        Ok(_) => panic!("unsupported branch should return a typed denial"),
        Err(error) => error,
    };

    for error in [preview_error, branch_error] {
        match error {
            ForgeQueryRuntimeError::UnsupportedFacadeFamily(denial) => {
                assert_eq!(
                    denial.family(),
                    ForgeQueryRuntimeFacadeFamily::BranchPreview
                );
                assert_eq!(
                    denial.reason(),
                    "test backend disabled branch and preview sessions"
                );
            }
            other => panic!("expected unsupported preview family denial, got {other:?}"),
        }
    }
}
