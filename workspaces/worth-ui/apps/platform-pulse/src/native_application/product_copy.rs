use worth_ui::facade::app::WorthUiNativeApplicationShell;
use worth_ui_native_platform::{
    UiNativeApplicationProgramDenial, UiNativeComponentSemanticTextChange,
};
use worth_ui_platform_pulse::product_world::PlatformPulseStaticCopy;

pub(super) fn install(
    shell: &mut WorthUiNativeApplicationShell,
) -> Result<(), UiNativeApplicationProgramDenial> {
    let changes = PlatformPulseStaticCopy::ALL
        .into_iter()
        .map(|copy| {
            UiNativeComponentSemanticTextChange::new(
                copy.component().authored_semantic_identity(),
                copy.text(),
            )
            .expect("Pulse product copy has bounded, valid semantic content")
        })
        .collect::<Vec<_>>();
    shell.apply_component_semantic_text(&changes)
}
