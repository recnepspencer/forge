use crate::runtime::tests::source_ingress_boundary_test_support::{
    lower_rust_submission, source_backed_package_component, source_backed_package_region,
    source_backed_package_sizing,
};

const ACTIVE_COMPONENT: &str = "workspace.component.active_session_current";
const CANDIDATE_COMPONENT: &str = "workspace.component.active_session_candidate";
const APPEARANCE_TOKEN: &str = "theme.appearance_consumer";

pub(crate) fn source_backed_static_paint_consumer_session(
) -> crate::facade::WorthUiActiveApplicationSession {
    let role = validation_background_role(APPEARANCE_TOKEN);
    appearance_component_builder(&role)
        .with_rust_authored_declaration_fixture(appearance_fixture(&role))
        .freeze()
        .map(crate::facade::entry::WorthUiCertificationApplicationTransition::activate_builder_host)
        .expect("appearance consumer source application should prepare")
        .launch()
        .expect("appearance consumer source application should launch")
}

pub(crate) fn source_backed_static_paint_role_capable_session(
    role: &worth_ui_dsl::UiAppearanceRoleDeclaration,
) -> crate::facade::WorthUiActiveApplicationSession {
    appearance_component_builder(role)
        .with_rust_authored_declaration_fixture(appearance_fixture_without_attachment())
        .freeze()
        .map(crate::facade::entry::WorthUiCertificationApplicationTransition::activate_builder_host)
        .expect("appearance-capable source application should prepare")
        .launch()
        .expect("appearance-capable source application should launch")
}

pub(crate) fn appearance_candidate_submission(
    session: &crate::facade::WorthUiActiveApplicationSession,
    source_name: &str,
    attachment: Option<&worth_ui_dsl::UiAppearanceRoleDeclaration>,
) -> crate::runtime::WorthUiWatchedCandidateSubmission {
    let declaration = worth_ui_dsl::WorthUiSemanticArtifactDeclaration::new(
        worth_ui_dsl::UiDslSemanticKey::new(ACTIVE_COMPONENT),
        worth_ui_dsl::UiDslSemanticFamily::Control,
    )
    .with_structural_token(worth_ui_dsl::UiDslStructuralToken::new(
        "control:appearance-consumer",
    ))
    .with_component_reference(worth_ui_dsl::UiDslComponentReference::new(ACTIVE_COMPONENT).unwrap())
    .unwrap();
    let declaration = attachment.map_or(declaration.clone(), |role| {
        declaration
            .clone()
            .with_appearance_role_attachment(
                worth_ui_dsl::UiAppearanceRoleAttachmentDeclaration::new(
                    role.role().clone(),
                    role.revision(),
                ),
            )
            .unwrap()
    });
    let input = worth_ui_dsl::WorthUiRustAuthoredArtifactInput::from_modules([
        worth_ui_dsl::WorthUiRustAuthoredArtifactInputModule::new("appearance/consumer")
            .with_semantic_declaration(declaration),
    ]);
    lower_rust_submission(
        crate::runtime::WorthUiSourceProvider::rust_authored(source_name)
            .with_rust_authored_input(input),
        [crate::runtime::WorthUiWatcherEvent::provider_revision(
            source_name,
        )],
        session.capabilities(),
    )
}

fn appearance_component_builder(
    role: &worth_ui_dsl::UiAppearanceRoleDeclaration,
) -> crate::facade::entry::WorthUiApplicationBuilder {
    let (_, _, world_profile) =
        crate::evidence::measurement::projection::fact_test_support::display_field_projection_context(
            "appearance-consumer-active-session",
        );
    let token = crate::capability::ThemeTokenId::new(APPEARANCE_TOKEN).unwrap();
    crate::facade::WorthUi::app()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .with_graph_world_profile(world_profile)
        .register_component(static_paint_component(ACTIVE_COMPONENT, token.clone()))
        .register_component(static_paint_component(CANDIDATE_COMPONENT, token.clone()))
        .register_appearance_role(role.clone())
        .unwrap()
        .register_theme_token(crate::capability::ThemeTokenDescriptor::define(
            token,
            crate::capability::ThemeTokenFamily::surface(),
            crate::capability::ThemeTokenSource::application(),
            crate::capability::ThemeTokenValue::color(
                crate::capability::ThemeColorValue::hex("#112233").unwrap(),
            ),
        ))
        .register_mosaic_region_kind(source_backed_package_region())
        .register_mosaic_sizing_contract(source_backed_package_sizing())
}

pub(crate) fn static_paint_component(
    identity: &str,
    token: crate::capability::ThemeTokenId,
) -> crate::capability::ComponentDescriptor {
    let appearance_contract = worth_ui_dsl::UiAppearanceAspectContract::component(
        [worth_ui_dsl::UiAppearanceAspect::Background],
        [],
    )
    .unwrap();
    static_paint_component_with_contract(identity, token, appearance_contract).unwrap()
}

fn static_paint_component_with_contract(
    identity: &str,
    token: crate::capability::ThemeTokenId,
    appearance_contract: worth_ui_dsl::UiAppearanceAspectContract,
) -> Result<
    crate::capability::ComponentDescriptor,
    crate::capability::ComponentAppearanceAspectContractDenial,
> {
    source_backed_package_component(identity)
        .with_static_paint(
            crate::capability::ComponentStaticPaintContract::opaque_fill(
                token,
                crate::capability::ComponentStaticPaintOrder::back_to_front(0),
            ),
            crate::capability::ComponentAllocationMeasurementContract::fill_viewport(),
        )
        .with_appearance_aspect_contract(appearance_contract)
}

#[test]
fn component_descriptor_rejects_the_actual_backdrop_contract() {
    let token = crate::capability::ThemeTokenId::new(APPEARANCE_TOKEN).unwrap();
    let result = static_paint_component_with_contract(
        ACTIVE_COMPONENT,
        token,
        worth_ui_dsl::UiAppearanceAspectContract::backdrop(),
    );
    assert_eq!(
        result,
        Err(
            crate::capability::ComponentAppearanceAspectContractDenial::BackdropContractOnComponent
        )
    );
}

fn appearance_fixture(
    role: &worth_ui_dsl::UiAppearanceRoleDeclaration,
) -> crate::facade::WorthUiRustAuthoredDeclarationFixture {
    let attachment = worth_ui_dsl::UiAppearanceRoleAttachmentDeclaration::new(
        role.role().clone(),
        role.revision(),
    );
    crate::facade::WorthUiRustAuthoredDeclarationFixture::named("appearance-consumer-current")
        .with_semantic_artifact_spec(
            worth_ui_dsl::UiDslSemanticArtifactSpec::new(
                worth_ui_dsl::UiDslSemanticKey::new(ACTIVE_COMPONENT),
                worth_ui_dsl::UiDslSemanticFamily::Control,
                worth_ui_dsl::UiDslSourceProvenance::rust_authored("appearance/consumer", 0),
            )
            .with_structural_token(worth_ui_dsl::UiDslStructuralToken::new(
                "control:appearance-consumer",
            ))
            .with_component_reference(
                worth_ui_dsl::UiDslComponentReference::new(ACTIVE_COMPONENT).unwrap(),
            )
            .unwrap()
            .with_appearance_role_attachment(attachment)
            .unwrap(),
        )
}

fn appearance_fixture_without_attachment() -> crate::facade::WorthUiRustAuthoredDeclarationFixture {
    crate::facade::WorthUiRustAuthoredDeclarationFixture::named("appearance-capable-current")
        .with_semantic_artifact_spec(
            worth_ui_dsl::UiDslSemanticArtifactSpec::new(
                worth_ui_dsl::UiDslSemanticKey::new(ACTIVE_COMPONENT),
                worth_ui_dsl::UiDslSemanticFamily::Control,
                worth_ui_dsl::UiDslSourceProvenance::rust_authored("appearance/consumer", 0),
            )
            .with_structural_token(worth_ui_dsl::UiDslStructuralToken::new(
                "control:appearance-consumer",
            ))
            .with_component_reference(
                worth_ui_dsl::UiDslComponentReference::new(ACTIVE_COMPONENT).unwrap(),
            )
            .unwrap(),
        )
}

pub(crate) fn validation_background_role(slot: &str) -> worth_ui_dsl::UiAppearanceRoleDeclaration {
    let contract = worth_ui_dsl::UiAppearanceAspectContract::component(
        [worth_ui_dsl::UiAppearanceAspect::Background],
        [],
    )
    .unwrap();
    let result = worth_ui_dsl::UiAppearanceDecisionResult::theme_slot(
        worth_ui_dsl::UiThemeSlotIdentity::new(slot).unwrap(),
        worth_ui_dsl::UiThemeValueKind::Color,
    );
    let partition = worth_ui_dsl::UiAppearanceDecisionPartition::compile(
        [worth_ui_dsl::UiAppearanceAxisDomain::complete(
            worth_ui_dsl::UiAppearanceStateAxis::Validation,
        )],
        [worth_ui_dsl::UiAppearanceDecisionRule::new(
            [worth_ui_dsl::UiAppearanceAxisPredicate::any(
                worth_ui_dsl::UiAppearanceStateAxis::Validation,
            )],
            result,
        )],
    )
    .unwrap();
    worth_ui_dsl::UiAppearanceRoleDeclaration::admit(
        worth_ui_dsl::UiAppearanceRoleIdentity::new("test.validation-background").unwrap(),
        worth_ui_dsl::UiAppearanceRoleRevision::new(1).unwrap(),
        &contract,
        [(worth_ui_dsl::UiAppearanceAspect::Background, partition)],
    )
    .unwrap()
}
