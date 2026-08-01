#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum UiIntentConcurrencyScope {
    TargetRouteSingleFlight,
    DeclarationSingleFlight,
    DefinitionSingleFlight,
    ApplicationSingleFlight,
}

impl UiIntentConcurrencyScope {
    pub(crate) const fn into_dsl(self) -> worth_ui_dsl::WorthUiIntentConcurrencyScope {
        match self {
            Self::TargetRouteSingleFlight => {
                worth_ui_dsl::WorthUiIntentConcurrencyScope::TargetRouteSingleFlight
            }
            Self::DeclarationSingleFlight => {
                worth_ui_dsl::WorthUiIntentConcurrencyScope::DeclarationSingleFlight
            }
            Self::DefinitionSingleFlight => {
                worth_ui_dsl::WorthUiIntentConcurrencyScope::DefinitionSingleFlight
            }
            Self::ApplicationSingleFlight => {
                worth_ui_dsl::WorthUiIntentConcurrencyScope::ApplicationSingleFlight
            }
        }
    }

    pub(crate) const fn from_dsl(scope: worth_ui_dsl::WorthUiIntentConcurrencyScope) -> Self {
        match scope {
            worth_ui_dsl::WorthUiIntentConcurrencyScope::TargetRouteSingleFlight => {
                Self::TargetRouteSingleFlight
            }
            worth_ui_dsl::WorthUiIntentConcurrencyScope::DeclarationSingleFlight => {
                Self::DeclarationSingleFlight
            }
            worth_ui_dsl::WorthUiIntentConcurrencyScope::DefinitionSingleFlight => {
                Self::DefinitionSingleFlight
            }
            worth_ui_dsl::WorthUiIntentConcurrencyScope::ApplicationSingleFlight => {
                Self::ApplicationSingleFlight
            }
        }
    }
}
