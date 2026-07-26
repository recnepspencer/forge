use crate::facade::WorthUiRustAuthoredDeclarationFixture;
use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken,
};
use worth_ui_inspection::{
    UiInspectionMilestoneExpectation, UiInspectionScope, UiInspectionSupportPosture,
    UiInspectionSupportReason, UiInspectionSupportWorld,
};

use crate::declaration::artifact::ui_declaration_lowering::UiDeclarationLowering;
use crate::declaration::{
    UiDeclarationSupportMilestoneExpectation, UiDeclarationSupportRowSchemaKind,
    UiDeclarationSupportSnapshotAdmission, UiDeclarationUnsupportedPosture,
    UiDeclaredPostureApplicability,
};

fn lower(spec: UiDslSemanticArtifactSpec) -> crate::declaration::UiDeclarationArtifact {
    let package =
        WorthUiRustAuthoredDeclarationFixture::named("worth-ui.runtime.declaration-support.tests");
    UiDeclarationLowering::lower(package.admit_semantic_artifact(spec))
}

fn assert_support_applicability(
    artifact: &crate::declaration::UiDeclarationArtifact,
    expected: [UiDeclaredPostureApplicability; 5],
) {
    let snapshot = artifact
        .support_snapshot()
        .expect("support snapshot should derive from admitted declared posture");

    assert_eq!(
        snapshot
            .row(UiDeclarationSupportRowSchemaKind::QueryBinding)
            .expect("query-binding row should exist")
            .applicability(),
        expected[0]
    );
    assert_eq!(
        snapshot
            .row(UiDeclarationSupportRowSchemaKind::ServiceUsage)
            .expect("service row should exist")
            .applicability(),
        expected[1]
    );
    assert_eq!(
        snapshot
            .row(UiDeclarationSupportRowSchemaKind::TouchMeaning)
            .expect("touch row should exist")
            .applicability(),
        expected[2]
    );
    assert_eq!(
        snapshot
            .row(UiDeclarationSupportRowSchemaKind::MeasurementPolicy)
            .expect("measurement row should exist")
            .applicability(),
        expected[3]
    );
    assert_eq!(
        snapshot
            .row(UiDeclarationSupportRowSchemaKind::HostCapability)
            .expect("host row should exist")
            .applicability(),
        expected[4]
    );
}

fn family_spec(family: UiDslSemanticFamily, declaration_index: usize) -> UiDslSemanticArtifactSpec {
    match family {
        UiDslSemanticFamily::Page => UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.page.root"),
            family,
            UiDslSourceProvenance::file_authored(
                "app/declaration_support_matrix.wui",
                declaration_index,
            ),
        )
        .with_structural_token(UiDslStructuralToken::new("page:product-root")),
        UiDslSemanticFamily::PageSet => UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.page_set.shell"),
            family,
            UiDslSourceProvenance::file_authored(
                "app/declaration_support_matrix.wui",
                declaration_index,
            ),
        )
        .with_structural_token(UiDslStructuralToken::new("page-set:shell")),
        UiDslSemanticFamily::Region => UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.region.sidebar"),
            family,
            UiDslSourceProvenance::file_authored(
                "app/declaration_support_matrix.wui",
                declaration_index,
            ),
        )
        .with_structural_token(UiDslStructuralToken::new("region:sidebar")),
        UiDslSemanticFamily::Mosaic => UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.mosaic.workspace"),
            family,
            UiDslSourceProvenance::file_authored(
                "app/declaration_support_matrix.wui",
                declaration_index,
            ),
        )
        .with_structural_token(UiDslStructuralToken::new("mosaic:workspace")),
        UiDslSemanticFamily::LocalComposition => UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.local_composition.inspector"),
            family,
            UiDslSourceProvenance::file_authored(
                "app/declaration_support_matrix.wui",
                declaration_index,
            ),
        )
        .with_structural_token(UiDslStructuralToken::new("local-composition:inspector")),
        UiDslSemanticFamily::Control => UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.control.save"),
            family,
            UiDslSourceProvenance::file_authored(
                "app/declaration_support_matrix.wui",
                declaration_index,
            ),
        )
        .with_structural_token(UiDslStructuralToken::new("control:save")),
        UiDslSemanticFamily::QueryBinding => UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.query.selection"),
            family,
            UiDslSourceProvenance::file_authored(
                "app/declaration_support_matrix.wui",
                declaration_index,
            ),
        )
        .with_posture_token(UiDslPostureToken::new("query-binding:standalone")),
        UiDslSemanticFamily::Intent => UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.intent.selection"),
            family,
            UiDslSourceProvenance::file_authored(
                "app/declaration_support_matrix.wui",
                declaration_index,
            ),
        )
        .with_posture_token(UiDslPostureToken::new("intent:standalone")),
        UiDslSemanticFamily::DiagnosticSurface => UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.diagnostic_surface.lint"),
            family,
            UiDslSourceProvenance::file_authored(
                "app/declaration_support_matrix.wui",
                declaration_index,
            ),
        )
        .with_structural_token(UiDslStructuralToken::new("diagnostic-surface:lint")),
    }
}

#[test]
fn support_snapshot_classifies_every_row_for_admitted_families() {
    let cases = [
        (
            UiDslSemanticFamily::Page,
            [
                UiDeclaredPostureApplicability::Optional,
                UiDeclaredPostureApplicability::ArchitecturallyOwnedButNotYetAdmitted,
                UiDeclaredPostureApplicability::ArchitecturallyOwnedButNotYetAdmitted,
                UiDeclaredPostureApplicability::Optional,
                UiDeclaredPostureApplicability::ArchitecturallyOwnedButNotYetAdmitted,
            ],
        ),
        (
            UiDslSemanticFamily::PageSet,
            [
                UiDeclaredPostureApplicability::Optional,
                UiDeclaredPostureApplicability::ArchitecturallyOwnedButNotYetAdmitted,
                UiDeclaredPostureApplicability::ArchitecturallyOwnedButNotYetAdmitted,
                UiDeclaredPostureApplicability::Optional,
                UiDeclaredPostureApplicability::ArchitecturallyOwnedButNotYetAdmitted,
            ],
        ),
        (
            UiDslSemanticFamily::Region,
            [
                UiDeclaredPostureApplicability::Optional,
                UiDeclaredPostureApplicability::ArchitecturallyOwnedButNotYetAdmitted,
                UiDeclaredPostureApplicability::ArchitecturallyOwnedButNotYetAdmitted,
                UiDeclaredPostureApplicability::Optional,
                UiDeclaredPostureApplicability::ArchitecturallyOwnedButNotYetAdmitted,
            ],
        ),
        (
            UiDslSemanticFamily::Mosaic,
            [
                UiDeclaredPostureApplicability::Optional,
                UiDeclaredPostureApplicability::ArchitecturallyOwnedButNotYetAdmitted,
                UiDeclaredPostureApplicability::ArchitecturallyOwnedButNotYetAdmitted,
                UiDeclaredPostureApplicability::Optional,
                UiDeclaredPostureApplicability::ArchitecturallyOwnedButNotYetAdmitted,
            ],
        ),
        (
            UiDslSemanticFamily::LocalComposition,
            [
                UiDeclaredPostureApplicability::Optional,
                UiDeclaredPostureApplicability::ArchitecturallyOwnedButNotYetAdmitted,
                UiDeclaredPostureApplicability::ArchitecturallyOwnedButNotYetAdmitted,
                UiDeclaredPostureApplicability::Optional,
                UiDeclaredPostureApplicability::ArchitecturallyOwnedButNotYetAdmitted,
            ],
        ),
        (
            UiDslSemanticFamily::Control,
            [
                UiDeclaredPostureApplicability::Optional,
                UiDeclaredPostureApplicability::Optional,
                UiDeclaredPostureApplicability::Optional,
                UiDeclaredPostureApplicability::Optional,
                UiDeclaredPostureApplicability::Optional,
            ],
        ),
        (
            UiDslSemanticFamily::QueryBinding,
            [
                UiDeclaredPostureApplicability::Required,
                UiDeclaredPostureApplicability::NotApplicable,
                UiDeclaredPostureApplicability::NotApplicable,
                UiDeclaredPostureApplicability::NotApplicable,
                UiDeclaredPostureApplicability::NotApplicable,
            ],
        ),
        (
            UiDslSemanticFamily::Intent,
            [
                UiDeclaredPostureApplicability::NotApplicable,
                UiDeclaredPostureApplicability::NotApplicable,
                UiDeclaredPostureApplicability::NotApplicable,
                UiDeclaredPostureApplicability::NotApplicable,
                UiDeclaredPostureApplicability::NotApplicable,
            ],
        ),
        (
            UiDslSemanticFamily::DiagnosticSurface,
            [
                UiDeclaredPostureApplicability::NotApplicable,
                UiDeclaredPostureApplicability::DiagnosticOnly,
                UiDeclaredPostureApplicability::DiagnosticOnly,
                UiDeclaredPostureApplicability::Optional,
                UiDeclaredPostureApplicability::DiagnosticOnly,
            ],
        ),
    ];

    for (declaration_index, (family, expected)) in cases.into_iter().enumerate() {
        let artifact = lower(family_spec(family, declaration_index));
        assert_support_applicability(&artifact, expected);
    }

    let query_binding = lower(family_spec(UiDslSemanticFamily::QueryBinding, 100));
    assert_eq!(
        query_binding
            .support_snapshot()
            .expect("query-binding family should derive support")
            .row(UiDeclarationSupportRowSchemaKind::QueryBinding)
            .expect("query-binding row should exist")
            .declared_query_binding_posture(),
        Some(&crate::declaration::UiDeclaredQueryBindingPosture::StandaloneBinding),
    );
}

#[test]
fn support_snapshot_localizes_not_yet_admitted_posture_to_exact_rows() {
    let artifact = lower(page_spec());
    let snapshot = artifact
        .support_snapshot()
        .expect("page declaration should derive support snapshot");

    assert_eq!(
        snapshot
            .row(UiDeclarationSupportRowSchemaKind::QueryBinding)
            .expect("query-binding row should exist")
            .unsupported_posture(),
        None,
    );
    assert_eq!(
        snapshot
            .row(UiDeclarationSupportRowSchemaKind::MeasurementPolicy)
            .expect("measurement row should exist")
            .unsupported_posture(),
        None,
    );

    for kind in [
        UiDeclarationSupportRowSchemaKind::ServiceUsage,
        UiDeclarationSupportRowSchemaKind::TouchMeaning,
        UiDeclarationSupportRowSchemaKind::HostCapability,
    ] {
        let row = snapshot.row(kind).expect("future-lane row should exist");
        assert_eq!(
            row.applicability(),
            UiDeclaredPostureApplicability::ArchitecturallyOwnedButNotYetAdmitted,
        );
        assert_eq!(
            row.unsupported_posture(),
            Some(
                UiDeclarationUnsupportedPosture::ArchitecturallyOwnedButNotYetAdmitted {
                    expected_in: UiDeclarationSupportMilestoneExpectation::Milestone32,
                },
            ),
        );
    }
}

#[test]
fn support_snapshot_inspection_projection_keeps_unsupported_posture_scope_local() {
    let artifact = lower(page_spec());
    let snapshot = artifact
        .support_snapshot()
        .expect("page declaration should derive support snapshot");

    let mounting_rows = snapshot.inspection_rows(UiInspectionScope::Mounting);
    assert_eq!(mounting_rows.len(), 3);
    for row in mounting_rows.iter() {
        assert_eq!(row.scope(), UiInspectionScope::Mounting);
        assert_eq!(row.posture(), UiInspectionSupportPosture::Deferred);
        assert_eq!(
            row.reason(),
            Some(UiInspectionSupportReason::BelongsArchitecturallyNotYetAdmitted),
        );
        assert_eq!(
            row.expected_in(),
            Some(UiInspectionMilestoneExpectation::Milestone32),
        );
        assert_eq!(row.current_world(), UiInspectionSupportWorld::Authoritative);
    }

    let measurement_rows = snapshot.inspection_rows(UiInspectionScope::Measurement);
    assert_eq!(measurement_rows.len(), 1);
    assert_eq!(
        measurement_rows[0].posture(),
        UiInspectionSupportPosture::Supported
    );
    assert_eq!(measurement_rows[0].reason(), None);

    let rebind_rows = snapshot.inspection_rows(UiInspectionScope::Rebind);
    assert_eq!(rebind_rows.len(), 1);
    assert_eq!(
        rebind_rows[0].posture(),
        UiInspectionSupportPosture::Supported
    );
    assert_eq!(rebind_rows[0].reason(), None);
}

#[test]
fn support_snapshot_denial_does_not_create_fallback_authority() {
    let artifact = lower(UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.query.selection"),
        UiDslSemanticFamily::QueryBinding,
        UiDslSourceProvenance::file_authored("app/declaration_support_denial.wui", 0),
    ));

    match artifact.support_snapshot_admission() {
        UiDeclarationSupportSnapshotAdmission::Denied(_) => {}
        UiDeclarationSupportSnapshotAdmission::Admitted(_) => {
            panic!("support snapshot must not admit when declared posture is denied")
        }
    }
    assert!(artifact.support_snapshot().is_err());
}

fn page_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.page.root"),
        UiDslSemanticFamily::Page,
        UiDslSourceProvenance::file_authored("app/declaration_support.wui", 1),
    )
    .with_structural_token(UiDslStructuralToken::new("page:product-root"))
}
