use worth_ui::facade::WorthUiRuntimeLaunch;

#[derive(Debug)]
pub enum HarnessScenarioOperation {
    LaunchRuntime { launch: WorthUiRuntimeLaunch },
    ObserveVisibleFrame,
    AttemptAppLocalShellStateInjection,
}

impl HarnessScenarioOperation {
    pub fn launch_runtime(launch: WorthUiRuntimeLaunch) -> Self {
        Self::LaunchRuntime { launch }
    }

    pub fn observe_visible_frame() -> Self {
        Self::ObserveVisibleFrame
    }

    pub fn attempt_app_local_shell_state_injection() -> Self {
        Self::AttemptAppLocalShellStateInjection
    }

    pub fn identity_text(&self) -> &'static str {
        match self {
            Self::LaunchRuntime { .. } => "harness.operation.launch_runtime",
            Self::ObserveVisibleFrame => "harness.operation.observe_visible_frame",
            Self::AttemptAppLocalShellStateInjection => {
                "harness.operation.attempt_app_local_shell_state_injection"
            }
        }
    }
}
