use super::appearance_component_session_test_support::{
    appearance_component_builder, appearance_fixture, APPEARANCE_TOKEN,
};

pub(crate) fn ownerless_focus_consumer_app() -> crate::facade::WorthUiApp {
    let role = focus_background_role();
    appearance_component_builder(&role)
        .with_rust_authored_declaration_fixture(appearance_fixture(&role))
        .freeze()
        .map(crate::facade::entry::WorthUiCertificationApplicationTransition::activate_builder_host)
        .expect("ownerless Focus consumer should prepare before launch admission")
}

pub(crate) fn focus_background_role() -> worth_ui_dsl::UiAppearanceRoleDeclaration {
    axis_background_role(
        worth_ui_dsl::UiAppearanceStateAxis::Focus,
        "test.focus-background",
    )
}

fn axis_background_role(
    axis: worth_ui_dsl::UiAppearanceStateAxis,
    identity: &str,
) -> worth_ui_dsl::UiAppearanceRoleDeclaration {
    let contract = worth_ui_dsl::UiAppearanceAspectContract::component(
        [worth_ui_dsl::UiAppearanceAspect::Background],
        [],
    )
    .unwrap();
    let result = worth_ui_dsl::UiAppearanceDecisionResult::theme_slot(
        worth_ui_dsl::UiThemeSlotIdentity::new(APPEARANCE_TOKEN).unwrap(),
        worth_ui_dsl::UiThemeValueKind::Color,
    );
    let partition = worth_ui_dsl::UiAppearanceDecisionPartition::compile(
        [worth_ui_dsl::UiAppearanceAxisDomain::complete(axis)],
        [worth_ui_dsl::UiAppearanceDecisionRule::new(
            [worth_ui_dsl::UiAppearanceAxisPredicate::any(axis)],
            result,
        )],
    )
    .unwrap();
    worth_ui_dsl::UiAppearanceRoleDeclaration::admit(
        worth_ui_dsl::UiAppearanceRoleIdentity::new(identity).unwrap(),
        worth_ui_dsl::UiAppearanceRoleRevision::new(1).unwrap(),
        &contract,
        [(worth_ui_dsl::UiAppearanceAspect::Background, partition)],
    )
    .unwrap()
}
