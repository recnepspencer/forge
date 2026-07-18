use worth_ui::facade::app::{
    WorthUi, WorthUiApplicationPreparationDenial, WorthUiApplicationPreparationPhase,
};
use worth_ui::facade::declaration::{
    UiDeclarationArtifact, UiDeclarationContainmentIntent, UiDeclarationFamilyKind,
    UiDeclarationGraphHandoffDenial, UiDeclarationStructuralRole, UiDeclaredPostureAdmissionDenial,
    UiDeclaredPostureApplicability, UiDeclaredPostureLaneKind, UiDeclaredQueryBindingPosture,
    UiDeclaredServiceUsagePosture, UiDeclaredTouchMeaningPosture, WorthUiHostCapability,
};
use worth_ui_dsl::{
    UiDslAspectName, UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily,
    UiDslSemanticKey, UiDslSourceProvenance, UiDslStructuralToken, UiDslSupportToken,
    WorthUiDslPackage,
};

#[test]
fn public_freeze_derives_exact_graph_handoff_from_canonical_declaration_authority() {
    let app = WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.graph-handoff")
                .with_semantic_artifact_spec(control_graph_input_spec()),
        )
        .freeze()
        .expect("application preparation should succeed");
    let artifact = artifact_from_file_provenance(&app, "app/graph_handoff.wui", 0);
    let handoff = artifact
        .graph_handoff()
        .expect("control declaration should derive graph handoff");

    assert_eq!(handoff.identity(), artifact.identity());
    assert_eq!(handoff.family_kind(), UiDeclarationFamilyKind::Control);
    assert_eq!(handoff.role(), UiDeclarationStructuralRole::Control);
    assert_eq!(
        handoff.containment_intent(),
        &UiDeclarationContainmentIntent::DeclaredControlAttachment {
            control_name: "save".into(),
        },
    );
    assert_eq!(
        handoff.query_binding().applicability(),
        UiDeclaredPostureApplicability::Optional
    );
    assert_eq!(
        handoff.query_binding().admitted(),
        Some(&UiDeclaredQueryBindingPosture::AttachedViewBinding),
    );
    assert_eq!(
        handoff.service_usage().admitted(),
        Some(&UiDeclaredServiceUsagePosture::Portal),
    );
    assert_eq!(
        handoff.touch_meaning().admitted(),
        Some(&UiDeclaredTouchMeaningPosture::Press),
    );
    assert_eq!(
        handoff
            .host_capability()
            .admitted()
            .expect("host capability posture should admit")
            .required_capabilities(),
        &[WorthUiHostCapability::Ime, WorthUiHostCapability::TextInput,],
    );
}

#[test]
fn support_noise_is_out_of_graph_but_aspect_contract_is_graph_relevant() {
    let baseline = WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.graph-handoff.equivalence")
                .with_semantic_artifact_spec(control_graph_input_spec()),
        )
        .freeze()
        .expect("application preparation should succeed");
    let noisy = WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.graph-handoff.equivalence")
                .with_semantic_artifact_spec(control_graph_input_with_noise_spec()),
        )
        .freeze()
        .expect("application preparation should succeed");
    let baseline_artifact = artifact_from_file_provenance(&baseline, "app/graph_handoff.wui", 0);
    let noisy_artifact = artifact_from_file_provenance(&noisy, "app/graph_handoff.wui", 0);

    let baseline_handoff = baseline_artifact
        .graph_handoff()
        .expect("baseline control declaration should derive graph handoff");
    let noisy_handoff = noisy_artifact
        .graph_handoff()
        .expect("noisy control declaration should derive graph handoff");

    assert_ne!(baseline_handoff.identity(), noisy_handoff.identity());
    assert_eq!(baseline_handoff.family(), noisy_handoff.family());
    assert_eq!(baseline_handoff.role(), noisy_handoff.role());
    assert_eq!(
        baseline_handoff.containment_intent(),
        noisy_handoff.containment_intent()
    );
    assert_eq!(
        baseline_handoff.slot_participation_intent(),
        noisy_handoff.slot_participation_intent()
    );
    assert_eq!(
        baseline_handoff.declared_posture(),
        noisy_handoff.declared_posture()
    );
    assert_ne!(
        baseline_handoff.aspect_contract(),
        noisy_handoff.aspect_contract()
    );
}

#[test]
fn invalid_declared_posture_denies_before_graph_handoff_promotion() {
    let denial = match WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.graph-handoff.denial")
                .with_semantic_artifact_spec(invalid_graph_input_spec()),
        )
        .freeze()
    {
        Ok(_) => panic!("invalid declared posture must deny application preparation"),
        Err(denial) => denial,
    };

    assert_eq!(
        denial.phase(),
        WorthUiApplicationPreparationPhase::GraphHandoff
    );
    match denial {
        WorthUiApplicationPreparationDenial::GraphHandoff(
            UiDeclarationGraphHandoffDenial::DeclaredPostureNotAdmitted {
                denial:
                    UiDeclaredPostureAdmissionDenial::InvalidLaneClaim {
                        family,
                        lane,
                        observed,
                    },
            },
        ) => {
            assert_eq!(family, UiDeclarationFamilyKind::Control);
            assert_eq!(lane, UiDeclaredPostureLaneKind::ServiceUsage);
            assert_eq!(observed, ["service:unknown"]);
        }
        other => panic!("unexpected application-preparation denial: {other:?}"),
    }
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

fn control_graph_input_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.save"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/graph_handoff.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:save"))
    .with_structural_token(UiDslStructuralToken::new("slot:footer"))
    .with_posture_token(UiDslPostureToken::new("query-binding:attached:view"))
    .with_posture_token(UiDslPostureToken::new("service:portal"))
    .with_posture_token(UiDslPostureToken::new("touch:press"))
    .with_posture_token(UiDslPostureToken::new("measurement:hug-height"))
    .with_posture_token(UiDslPostureToken::new("host-capability:text-input"))
    .with_posture_token(UiDslPostureToken::new("host-capability:ime"))
}

fn control_graph_input_with_noise_spec() -> UiDslSemanticArtifactSpec {
    control_graph_input_spec()
        .with_published_aspect(UiDslAspectName::new("content.text"))
        .with_support_token(UiDslSupportToken::new("support:preview-only"))
}

fn invalid_graph_input_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.invalid"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/graph_handoff_denial.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:save"))
    .with_posture_token(UiDslPostureToken::new("service:unknown"))
}
