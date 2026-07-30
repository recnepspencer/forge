/// WUI-owned transition destinations admitted in 3.14.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum UiIntentTransitionDestination {
    NavigatePage,
    ChangeMosaic,
}

/// Runtime-service destinations reserved for the 3.15 service owner.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum UiIntentRuntimeServiceDestination {
    OpenPortal,
    ClosePortal,
    InvokeCommand,
}

/// Exactly one execution owner selected by a compiled intent definition.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum UiIntentExecutionDestination {
    ApplicationEffect,
    UiTransition(UiIntentTransitionDestination),
    RuntimeService(UiIntentRuntimeServiceDestination),
}

impl UiIntentExecutionDestination {
    pub(crate) const fn digest_basis(self) -> &'static str {
        match self {
            Self::ApplicationEffect => "application_effect",
            Self::UiTransition(UiIntentTransitionDestination::NavigatePage) => {
                "ui_transition:navigate_page"
            }
            Self::UiTransition(UiIntentTransitionDestination::ChangeMosaic) => {
                "ui_transition:change_mosaic"
            }
            Self::RuntimeService(UiIntentRuntimeServiceDestination::OpenPortal) => {
                "runtime_service:open_portal"
            }
            Self::RuntimeService(UiIntentRuntimeServiceDestination::ClosePortal) => {
                "runtime_service:close_portal"
            }
            Self::RuntimeService(UiIntentRuntimeServiceDestination::InvokeCommand) => {
                "runtime_service:invoke_command"
            }
        }
    }
}
