use worth_ui::facade::app::WorthUi;
use worth_ui::facade::declaration::{
    UiDeclarationArtifact, UiDeclarationSupportMilestoneExpectation,
    UiDeclarationSupportRowSchemaKind, UiDeclarationUnsupportedPosture,
};
use worth_ui::facade::inspection::{
    UiInspectionMilestoneExpectation, UiInspectionPosture, UiInspectionQuery, UiInspectionScope,
    UiInspectionSupportPosture, UiInspectionSupportReason, UiInspectionSupportStatus,
    UiInspectionSupportWorld, UiInspectionTarget,
};
use worth_ui_certification::{
    WorthUiCertificationBuilderExt, WorthUiRustAuthoredDeclarationFixture,
};
use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken,
};
use worth_ui_host_contract::WorthUiHostCapability;

#[test]
fn public_freeze_derives_support_snapshot_from_admitted_declaration_authority() {
    let fixture =
        WorthUiRustAuthoredDeclarationFixture::named("worth-ui.certification.declaration-support")
            .with_semantic_artifact_spec(control_spec());
    let provenance = fixture.admitted_provenance_for("workflow_editor.inspector.save");
    let app = WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(fixture)
        .freeze()
        .map(
            worth_ui_runtime::facade::entry::WorthUiCertificationApplicationTransition::activate_headless,
        )
        .expect("application preparation should succeed");
    let artifact = artifact_from_compiler_provenance(&app, &provenance);
    let snapshot = artifact
        .support_snapshot()
        .expect("control declaration should expose support snapshot on freeze path");

    assert_eq!(snapshot.rows().len(), 5);
    assert_eq!(
        snapshot
            .row(UiDeclarationSupportRowSchemaKind::HostCapability)
            .expect("host row should exist")
            .declared_host_capability_posture()
            .map(|posture| posture.required_capabilities()),
        Some(&[WorthUiHostCapability::Ime, WorthUiHostCapability::TextInput,][..]),
    );
}

#[test]
fn public_freeze_localizes_future_semantics_to_exact_support_rows() {
    let fixture = WorthUiRustAuthoredDeclarationFixture::named(
        "worth-ui.certification.declaration-support.localization",
    )
    .with_semantic_artifact_spec(page_spec());
    let provenance = fixture.admitted_provenance_for("workflow_editor.region.root");
    let app = WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(fixture)
        .freeze()
        .map(
            worth_ui_runtime::facade::entry::WorthUiCertificationApplicationTransition::activate_headless,
        )
        .expect("application preparation should succeed");
    let artifact = artifact_from_compiler_provenance(&app, &provenance);
    let snapshot = artifact
        .support_snapshot()
        .expect("page declaration should expose support snapshot on freeze path");

    for kind in [
        UiDeclarationSupportRowSchemaKind::ServiceUsage,
        UiDeclarationSupportRowSchemaKind::TouchMeaning,
        UiDeclarationSupportRowSchemaKind::HostCapability,
    ] {
        assert_eq!(
            snapshot
                .row(kind)
                .expect("future row should exist")
                .unsupported_posture(),
            Some(
                UiDeclarationUnsupportedPosture::ArchitecturallyOwnedButNotYetAdmitted {
                    expected_in: UiDeclarationSupportMilestoneExpectation::Milestone32,
                },
            ),
        );
    }

    assert_eq!(
        snapshot
            .row(UiDeclarationSupportRowSchemaKind::MeasurementPolicy)
            .expect("measurement row should exist")
            .unsupported_posture(),
        None,
    );
}

#[test]
fn public_freeze_preserves_representative_support_shapes_across_family_classes() {
    let fixture = WorthUiRustAuthoredDeclarationFixture::named(
        "worth-ui.certification.declaration-support.shapes",
    )
    .with_semantic_artifact_spec(page_spec())
    .with_semantic_artifact_spec(region_spec())
    .with_semantic_artifact_spec(control_spec())
    .with_semantic_artifact_spec(diagnostic_surface_spec());
    let page_provenance = fixture.admitted_provenance_for("workflow_editor.region.root");
    let region_provenance = fixture.admitted_provenance_for("workflow_editor.region.sidebar");
    let control_provenance = fixture.admitted_provenance_for("workflow_editor.inspector.save");
    let diagnostic_provenance =
        fixture.admitted_provenance_for("workflow_editor.diagnostic_surface.lint");
    let app = WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(fixture)
        .freeze()
        .map(
            worth_ui_runtime::facade::entry::WorthUiCertificationApplicationTransition::activate_headless,
        )
        .expect("application preparation should succeed");

    let page = artifact_from_compiler_provenance(&app, &page_provenance);
    let region = artifact_from_compiler_provenance(&app, &region_provenance);
    let control = artifact_from_compiler_provenance(&app, &control_provenance);
    let diagnostic = artifact_from_compiler_provenance(&app, &diagnostic_provenance);

    for artifact in [page, region] {
        assert_eq!(
            artifact
                .support_snapshot()
                .expect("structural family should derive support")
                .row(UiDeclarationSupportRowSchemaKind::HostCapability)
                .expect("host row should exist")
                .unsupported_posture(),
            Some(
                UiDeclarationUnsupportedPosture::ArchitecturallyOwnedButNotYetAdmitted {
                    expected_in: UiDeclarationSupportMilestoneExpectation::Milestone32,
                },
            ),
        );
    }

    assert_eq!(
        control
            .support_snapshot()
            .expect("control should derive support")
            .row(UiDeclarationSupportRowSchemaKind::HostCapability)
            .expect("host row should exist")
            .declared_host_capability_posture()
            .map(|posture| posture.required_capabilities()),
        Some(&[WorthUiHostCapability::Ime, WorthUiHostCapability::TextInput,][..]),
    );
    assert_eq!(
        diagnostic
            .support_snapshot()
            .expect("diagnostic surface should derive support")
            .row(UiDeclarationSupportRowSchemaKind::TouchMeaning)
            .expect("touch row should exist")
            .applicability(),
        worth_ui::facade::declaration::UiDeclaredPostureApplicability::DiagnosticOnly
    );
    assert_eq!(
        diagnostic
            .support_snapshot()
            .expect("diagnostic surface should derive support")
            .row(UiDeclarationSupportRowSchemaKind::QueryBinding)
            .expect("query row should exist")
            .unsupported_posture(),
        None,
    );
}

#[test]
fn public_app_inspection_surfaces_use_declaration_support_projection() {
    let app = WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(
                "worth-ui.certification.declaration-support.inspection",
            )
            .with_semantic_artifact_spec(page_spec()),
        )
        .freeze()
        .map(
            worth_ui_runtime::facade::entry::WorthUiCertificationApplicationTransition::activate_headless,
        )
        .expect("application preparation should succeed");
    let mounting_report = app.inspection_support_report(UiInspectionScope::Mounting);
    assert_eq!(
        mounting_report.status(),
        UiInspectionSupportStatus::Unsupported
    );
    assert_eq!(
        mounting_report.posture(),
        UiInspectionSupportPosture::Deferred
    );
    assert_eq!(
        mounting_report.reason(),
        Some(UiInspectionSupportReason::BelongsArchitecturallyNotYetAdmitted),
    );
    assert_eq!(
        mounting_report.expected_in(),
        Some(UiInspectionMilestoneExpectation::Milestone32),
    );

    let measurement_report = app.inspection_support_report(UiInspectionScope::Measurement);
    assert_eq!(
        measurement_report.status(),
        UiInspectionSupportStatus::Supported
    );
    assert_eq!(
        measurement_report.posture(),
        UiInspectionSupportPosture::Supported
    );
    assert_eq!(measurement_report.reason(), None);
    assert_eq!(measurement_report.expected_in(), None);
    assert_eq!(
        measurement_report.current_world(),
        UiInspectionSupportWorld::Authoritative
    );

    let receipt = app.inspect(UiInspectionQuery::new(
        UiInspectionTarget::product_root(),
        UiInspectionScope::Mounting,
    ));
    assert_eq!(receipt.support_report(), Some(mounting_report));
    assert_eq!(
        receipt.posture(),
        Some(UiInspectionPosture::deferred(
            Some(UiInspectionMilestoneExpectation::Milestone32),
            UiInspectionSupportWorld::Authoritative,
        )),
    );
}

#[test]
fn public_app_inspection_receipts_keep_diagnostic_only_support_visible() {
    let fixture = WorthUiRustAuthoredDeclarationFixture::named(
        "worth-ui.certification.declaration-support.receipt-diagnostic",
    )
    .with_semantic_artifact_spec(diagnostic_surface_spec());
    let provenance = fixture.admitted_provenance_for("workflow_editor.diagnostic_surface.lint");
    let app = WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(fixture)
        .freeze()
        .map(
            worth_ui_runtime::facade::entry::WorthUiCertificationApplicationTransition::activate_headless,
        )
        .expect("application preparation should succeed");
    let query = UiInspectionQuery::new(
        UiInspectionTarget::declared_surface(
            provenance.module_path(),
            provenance.declaration_index(),
        ),
        UiInspectionScope::Mounting,
    );
    let report = app.inspection_support_report_for(&query);
    let receipt = app.inspect(query);

    assert_eq!(report.posture(), UiInspectionSupportPosture::DiagnosticOnly);
    assert_eq!(report.status(), UiInspectionSupportStatus::Unsupported);
    assert_eq!(
        report.reason(),
        Some(UiInspectionSupportReason::DiagnosticOnly)
    );
    assert_eq!(report.expected_in(), None);
    assert_eq!(
        report.current_world(),
        UiInspectionSupportWorld::Authoritative
    );
    assert_eq!(report.expected_world(), None);

    assert_eq!(receipt.support_report(), Some(report));
    assert_eq!(
        receipt.posture(),
        Some(UiInspectionPosture::diagnostic_only(
            UiInspectionSupportWorld::Authoritative,
        )),
    );
}

#[test]
fn public_declaration_support_projection_keeps_diagnostic_only_rows_visible() {
    let fixture = WorthUiRustAuthoredDeclarationFixture::named(
        "worth-ui.certification.declaration-support.diagnostic",
    )
    .with_semantic_artifact_spec(diagnostic_surface_spec());
    let provenance = fixture.admitted_provenance_for("workflow_editor.diagnostic_surface.lint");
    let app = WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(fixture)
        .freeze()
        .map(
            worth_ui_runtime::facade::entry::WorthUiCertificationApplicationTransition::activate_headless,
        )
        .expect("application preparation should succeed");
    let artifact = artifact_from_compiler_provenance(&app, &provenance);
    let rows = artifact
        .support_snapshot()
        .expect("diagnostic surface should expose support snapshot")
        .inspection_rows(UiInspectionScope::Mounting);

    assert_eq!(rows.len(), 3);
    for row in rows.iter() {
        assert_eq!(row.posture(), UiInspectionSupportPosture::DiagnosticOnly);
        assert_eq!(row.status(), UiInspectionSupportStatus::Unsupported);
        assert_eq!(
            row.reason(),
            Some(UiInspectionSupportReason::DiagnosticOnly)
        );
        assert_eq!(row.current_world(), UiInspectionSupportWorld::Authoritative);
        assert_eq!(row.expected_in(), None);
    }
}

fn control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.save"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/declaration_support.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:save"))
    .with_posture_token(UiDslPostureToken::new("query-binding:attached:view"))
    .with_posture_token(UiDslPostureToken::new("service:portal"))
    .with_posture_token(UiDslPostureToken::new("touch:press"))
    .with_posture_token(UiDslPostureToken::new("measurement:hug-height"))
    .with_posture_token(UiDslPostureToken::new("host-capability:text-input"))
    .with_posture_token(UiDslPostureToken::new("host-capability:ime"))
}

fn page_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.region.root"),
        UiDslSemanticFamily::Region,
        UiDslSourceProvenance::file_authored("app/declaration_support.wui", 1),
    )
    .with_structural_token(UiDslStructuralToken::new("region:sidebar"))
}

fn region_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.region.sidebar"),
        UiDslSemanticFamily::Region,
        UiDslSourceProvenance::file_authored("app/declaration_support.wui", 2),
    )
    .with_structural_token(UiDslStructuralToken::new("region:sidebar"))
}

fn diagnostic_surface_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.diagnostic_surface.lint"),
        UiDslSemanticFamily::DiagnosticSurface,
        UiDslSourceProvenance::file_authored("app/declaration_support.wui", 3),
    )
    .with_structural_token(UiDslStructuralToken::new("diagnostic-surface:lint"))
}

fn artifact_from_file_provenance<'a>(
    app: &'a worth_ui::facade::app::WorthUiApp,
    module_path: &str,
    declaration_index: usize,
) -> &'a UiDeclarationArtifact {
    app.declaration_artifacts()
        .iter()
        .find(|artifact| {
            let provenance = artifact.provenance().source_provenance();
            provenance.module_path() == module_path
                && provenance.declaration_index() == declaration_index
        })
        .unwrap_or_else(|| {
            panic!(
                "expected declaration artifact for {module_path}#{declaration_index} on freeze path"
            )
        })
}

fn artifact_from_compiler_provenance<'a>(
    app: &'a worth_ui::facade::app::WorthUiApp,
    provenance: &UiDslSourceProvenance,
) -> &'a UiDeclarationArtifact {
    artifact_from_file_provenance(
        app,
        provenance.module_path(),
        provenance.declaration_index(),
    )
}
