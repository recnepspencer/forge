use super::super::super::support::*;

#[test]
fn runtime_support_denies_unsupported_write_family_before_execution() {
    let mut runtime = bridge_runtime_with_support(
        WorthQueryRuntimeSupportProfile::scaffold_backend_profile().with_family_support(
            WorthQueryRuntimeFamilySupport::unsupported(
                WorthQueryRuntimeFacadeFamily::Write,
                "test backend disabled write authority",
            ),
        ),
    );

    let error = runtime
        .write(insert_command(
            "Task",
            [
                ("identity.id", test_string_aspect_value("external-1")),
                ("title.value", test_string_aspect_value("Should not write")),
            ],
        ))
        .expect_err("unsupported write family should deny before write authority");

    match error {
        WorthQueryRuntimeError::UnsupportedFacadeFamily(denial) => {
            assert_eq!(denial.family(), WorthQueryRuntimeFacadeFamily::Write);
            assert_eq!(denial.reason(), "test backend disabled write authority");
        }
        other => panic!("expected unsupported facade family denial, got {other:?}"),
    }
}

#[test]
fn runtime_builder_rejects_support_profiles_that_overclaim_unimplemented_families() {
    let profile = WorthQueryRuntimeSupportProfile::scaffold_backend_profile().with_family_support(
        WorthQueryRuntimeFamilySupport::supported(
            WorthQueryRuntimeFacadeFamily::Intent,
            [WorthQueryAuthorityLane::PendingWriteIntent],
            [WorthQueryEffectPolicy::AuthoritativeAllowed],
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
        WorthQueryRuntimeError::UnsupportedFacadeFamily(denial) => {
            assert_eq!(denial.family(), WorthQueryRuntimeFacadeFamily::Intent);
            assert!(denial.reason().contains("intent authority adapter"));
        }
        other => panic!("expected unsupported facade family denial, got {other:?}"),
    }
}

#[test]
fn runtime_support_denies_unsupported_computed_family_before_registration() {
    let mut runtime = bridge_runtime_with_support(
        WorthQueryRuntimeSupportProfile::scaffold_backend_profile().with_family_support(
            WorthQueryRuntimeFamilySupport::unsupported(
                WorthQueryRuntimeFacadeFamily::Computed,
                "test backend disabled computed resources",
            ),
        ),
    );

    let error = runtime
        .declare_maintained_derived_view::<WorthQueryUnrefinedLiveShape>(
            WorthQueryDerivedView::new("task_titles.unsupported", [test_aspect_touch("title")]),
            TitleListMaintainer,
        )
        .expect_err("unsupported computed family should deny before registration");

    match error {
        WorthQueryRuntimeError::UnsupportedFacadeFamily(denial) => {
            assert_eq!(denial.family(), WorthQueryRuntimeFacadeFamily::Computed);
            assert_eq!(denial.reason(), "test backend disabled computed resources");
        }
        other => panic!("expected unsupported facade family denial, got {other:?}"),
    }
}

#[test]
fn runtime_support_denies_unsupported_preview_and_branch_sessions_without_panicking() {
    let mut runtime = bridge_runtime_with_support(
        WorthQueryRuntimeSupportProfile::scaffold_backend_profile().with_family_support(
            WorthQueryRuntimeFamilySupport::unsupported(
                WorthQueryRuntimeFacadeFamily::BranchPreview,
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
            WorthQueryRuntimeError::UnsupportedFacadeFamily(denial) => {
                assert_eq!(
                    denial.family(),
                    WorthQueryRuntimeFacadeFamily::BranchPreview
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
