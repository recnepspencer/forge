use worth_ui::facade::WorthUiAppBuilder;

use super::PreparedHarnessVisualFoundation;

pub trait HarnessVisualFoundationRegistration {
    fn install_harness_visual_foundation(self, foundation: PreparedHarnessVisualFoundation)
        -> Self;
}

impl HarnessVisualFoundationRegistration for WorthUiAppBuilder {
    fn install_harness_visual_foundation(
        self,
        foundation: PreparedHarnessVisualFoundation,
    ) -> Self {
        let parts = foundation.into_parts();
        let builder = parts
            .theme_tokens
            .into_iter()
            .fold(self, WorthUiAppBuilder::register_theme_token);
        let builder = parts
            .sizing_contracts
            .into_iter()
            .fold(builder, WorthUiAppBuilder::register_mosaic_sizing_contract);
        let builder = parts
            .icons
            .into_iter()
            .fold(builder, WorthUiAppBuilder::register_icon);
        let builder = parts
            .command_projections
            .into_iter()
            .fold(builder, WorthUiAppBuilder::register_command_projection);
        parts.runtime_outcome_projections.into_iter().fold(
            builder,
            WorthUiAppBuilder::register_runtime_outcome_projection,
        )
    }
}
