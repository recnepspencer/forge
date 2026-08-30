pub(in crate::native_application) fn latest_command_transition(
    shell: &worth_ui::facade::app::WorthUiNativeApplicationShell,
) -> Option<worth_ui_platform_pulse::observation_contract::PlatformPulseCommandTransitionInspection>
{
    shell
        .why_command_won()
        .map(
            worth_ui_platform_pulse::observation_contract::PlatformPulseCommandTransitionInspection::from_inspection,
        )
}
