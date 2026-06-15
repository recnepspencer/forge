use worth_ui::facade::WorthUi;
use worth_ui_harness::facade::{
    HarnessDensity, HarnessVisualFoundationBundle, HarnessVisualFoundationRegistration,
};

#[test]
fn theme_density_change_preserves_shell_artifact_meaning() {
    let compact = HarnessVisualFoundationBundle::vscode_like_dark()
        .with_density(HarnessDensity::CompactWorkbench)
        .prepare()
        .expect("compact visual foundation should prepare");
    let comfortable = HarnessVisualFoundationBundle::vscode_like_dark()
        .with_density(HarnessDensity::ComfortableWorkbench)
        .prepare()
        .expect("comfortable visual foundation should prepare");

    assert_eq!(theme_token_ids(&compact), theme_token_ids(&comfortable));
    assert_eq!(
        command_projection_ids(&compact),
        command_projection_ids(&comfortable)
    );
    assert_eq!(
        runtime_projection_ids(&compact),
        runtime_projection_ids(&comfortable)
    );
    assert_eq!(
        sizing_contract_ids(&compact),
        sizing_contract_ids(&comfortable)
    );
    assert_ne!(
        sizing_contract_debug_basis(&compact),
        sizing_contract_debug_basis(&comfortable),
        "density should change measurement values without changing sizing identity"
    );

    let compact_app = WorthUi::app()
        .install_harness_visual_foundation(compact)
        .freeze();
    let comfortable_app = WorthUi::app()
        .install_harness_visual_foundation(comfortable)
        .freeze();

    assert_eq!(
        compact_app.capabilities().command_projections(),
        comfortable_app.capabilities().command_projections(),
        "visual density reload must not rewrite command-backed shell surfaces"
    );
    assert_eq!(
        compact_app.capabilities().runtime_outcome_projections(),
        comfortable_app.capabilities().runtime_outcome_projections(),
        "visual density reload must not rewrite runtime outcome evidence"
    );
    assert_ne!(
        compact_app.capabilities().digest(),
        comfortable_app.capabilities().digest(),
        "density token values must remain visible in the capability artifact"
    );
}

fn theme_token_ids(
    foundation: &worth_ui_harness::facade::PreparedHarnessVisualFoundation,
) -> Vec<String> {
    foundation
        .theme_tokens()
        .iter()
        .map(|descriptor| descriptor.id().as_str().to_owned())
        .collect()
}

fn runtime_projection_ids(
    foundation: &worth_ui_harness::facade::PreparedHarnessVisualFoundation,
) -> Vec<String> {
    foundation
        .runtime_outcome_projections()
        .iter()
        .map(|descriptor| descriptor.id().as_str().to_owned())
        .collect()
}

fn command_projection_ids(
    foundation: &worth_ui_harness::facade::PreparedHarnessVisualFoundation,
) -> Vec<String> {
    foundation
        .command_projections()
        .iter()
        .map(|descriptor| descriptor.id().as_str().to_owned())
        .collect()
}

fn sizing_contract_ids(
    foundation: &worth_ui_harness::facade::PreparedHarnessVisualFoundation,
) -> Vec<String> {
    foundation
        .sizing_contracts()
        .iter()
        .map(|descriptor| descriptor.id().as_str().to_owned())
        .collect()
}

fn sizing_contract_debug_basis(
    foundation: &worth_ui_harness::facade::PreparedHarnessVisualFoundation,
) -> Vec<String> {
    foundation
        .sizing_contracts()
        .iter()
        .map(|descriptor| format!("{:?}", descriptor.named_measurement()))
        .collect()
}
