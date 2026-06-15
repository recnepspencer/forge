use worth_ui::facade::{
    WorthUi, WorthUiRuntimeLaunchPreparationDenial, WorthUiRuntimeSourceModule,
};

#[test]
fn empty_launch_source_package_is_rejected_before_runtime_launch() {
    let app = WorthUi::app().freeze();

    let denial = WorthUi::runtime_launch()
        .prepare_for(&app)
        .expect_err("empty launch source must not construct a runtime launch");

    assert_eq!(
        denial,
        WorthUiRuntimeLaunchPreparationDenial::EmptySourcePackage
    );
}

#[test]
fn malformed_launch_source_is_rejected_before_runtime_launch() {
    let app = WorthUi::app().freeze();

    let denial = WorthUi::runtime_launch()
        .from_source_module(WorthUiRuntimeSourceModule::new(
            "app/main.wui",
            "component Dashboard {",
        ))
        .prepare_for(&app)
        .expect_err("malformed source must not construct a runtime launch");

    assert_diagnostic_denial(denial, |denial| {
        matches!(
            denial,
            WorthUiRuntimeLaunchPreparationDenial::ParseRejected { .. }
        )
    });
}

#[test]
fn unresolved_launch_capability_is_rejected_before_runtime_launch() {
    let app = WorthUi::app().freeze();

    let denial = WorthUi::runtime_launch()
        .from_source_module(WorthUiRuntimeSourceModule::new(
            "app/main.wui",
            "component workspace.component.missing {}",
        ))
        .prepare_for(&app)
        .expect_err("unresolved source capability must not construct a runtime launch");

    assert_diagnostic_denial(denial, |denial| {
        matches!(
            denial,
            WorthUiRuntimeLaunchPreparationDenial::SnapshotResolutionRejected { .. }
        )
    });
}

#[test]
fn invalid_authoring_entry_is_rejected_before_snapshot_resolution() {
    let app = WorthUi::app().freeze();

    let denial = WorthUi::runtime_launch()
        .from_source_module(WorthUiRuntimeSourceModule::new(
            "app/main.wui",
            r#"
            app ShopifyAdminApp { workspace MissingWorkspace }
            workspace AdminWorkspace { pages [ProductsPage] }
            page ProductsPage {
                runtime ProductsRuntime
                layout ProductsLayout
                content ProductsContent
            }
            runtime ProductsRuntime {}
            layout ProductsLayout {}
            content ProductsContent {}
            "#,
        ))
        .prepare_for(&app)
        .expect_err("invalid authoring entry must not construct a runtime launch");

    assert_diagnostic_denial(denial, |denial| {
        matches!(
            denial,
            WorthUiRuntimeLaunchPreparationDenial::AuthoringEntryRejected { .. }
        )
    });
}

#[test]
fn blank_scenario_id_is_rejected_without_panicking() {
    let denial = worth_ui_harness::facade::HarnessScenario::define("   ")
        .expect_err("blank scenario id must be a typed authoring denial");

    assert_eq!(
        denial,
        worth_ui_harness::facade::HarnessScenarioIdError::Empty
    );
}

fn assert_diagnostic_denial(
    denial: WorthUiRuntimeLaunchPreparationDenial,
    expected_kind: impl FnOnce(&WorthUiRuntimeLaunchPreparationDenial) -> bool,
) {
    assert!(expected_kind(&denial), "unexpected denial kind: {denial:?}");
    assert!(
        diagnostic_count(&denial) > 0,
        "diagnostic denial should report at least one diagnostic: {denial:?}"
    );
}

fn diagnostic_count(denial: &WorthUiRuntimeLaunchPreparationDenial) -> usize {
    match denial {
        WorthUiRuntimeLaunchPreparationDenial::SourcePackageRejected { diagnostic_count }
        | WorthUiRuntimeLaunchPreparationDenial::ParseRejected { diagnostic_count }
        | WorthUiRuntimeLaunchPreparationDenial::AuthoringEntryRejected { diagnostic_count }
        | WorthUiRuntimeLaunchPreparationDenial::SnapshotResolutionRejected { diagnostic_count }
        | WorthUiRuntimeLaunchPreparationDenial::StructuralLegalityRejected { diagnostic_count }
        | WorthUiRuntimeLaunchPreparationDenial::BindingSemanticsRejected { diagnostic_count }
        | WorthUiRuntimeLaunchPreparationDenial::IdentitySeedingRejected { diagnostic_count }
        | WorthUiRuntimeLaunchPreparationDenial::ArtifactAssemblyRejected { diagnostic_count } => {
            *diagnostic_count
        }
        WorthUiRuntimeLaunchPreparationDenial::EmptySourcePackage => 0,
    }
}
