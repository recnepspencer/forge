/// Built-in capability families a plugin slot may admit.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PluginContributionFamily {
    kind: PluginContributionFamilyKind,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum PluginContributionFamilyKind {
    Command,
    Component,
    Surface,
    Setting,
    ViewBinding,
    ThemeToken,
    Icon,
    CommandProjection,
    TaskPresentation,
    RuntimeOutcomeProjection,
    NativeCapabilityRequest,
    UnsupportedForDiagnostics(String),
    GlobalMutationHookForDiagnostics,
}

impl PluginContributionFamily {
    pub fn command() -> Self {
        Self::known(PluginContributionFamilyKind::Command)
    }

    pub fn component() -> Self {
        Self::known(PluginContributionFamilyKind::Component)
    }

    pub fn surface() -> Self {
        Self::known(PluginContributionFamilyKind::Surface)
    }

    pub fn setting() -> Self {
        Self::known(PluginContributionFamilyKind::Setting)
    }

    pub fn view_binding() -> Self {
        Self::known(PluginContributionFamilyKind::ViewBinding)
    }

    pub fn theme_token() -> Self {
        Self::known(PluginContributionFamilyKind::ThemeToken)
    }

    pub fn icon() -> Self {
        Self::known(PluginContributionFamilyKind::Icon)
    }

    pub fn command_projection() -> Self {
        Self::known(PluginContributionFamilyKind::CommandProjection)
    }

    pub fn task_presentation() -> Self {
        Self::known(PluginContributionFamilyKind::TaskPresentation)
    }

    pub fn runtime_outcome_projection() -> Self {
        Self::known(PluginContributionFamilyKind::RuntimeOutcomeProjection)
    }

    pub fn native_capability_request() -> Self {
        Self::known(PluginContributionFamilyKind::NativeCapabilityRequest)
    }

    pub fn unsupported_for_diagnostics(family: impl Into<String>) -> Self {
        Self::known(PluginContributionFamilyKind::UnsupportedForDiagnostics(
            family.into(),
        ))
    }

    pub fn arbitrary_global_mutation_hook_for_diagnostics() -> Self {
        Self::known(PluginContributionFamilyKind::GlobalMutationHookForDiagnostics)
    }

    pub(crate) fn is_supported(&self) -> bool {
        !matches!(
            self.kind,
            PluginContributionFamilyKind::UnsupportedForDiagnostics(_)
                | PluginContributionFamilyKind::GlobalMutationHookForDiagnostics
        )
    }

    pub(crate) fn is_global_mutation_hook(&self) -> bool {
        matches!(
            self.kind,
            PluginContributionFamilyKind::GlobalMutationHookForDiagnostics
        )
    }

    pub(crate) fn digest_basis(&self) -> String {
        match &self.kind {
            PluginContributionFamilyKind::Command => "command".to_owned(),
            PluginContributionFamilyKind::Component => "component".to_owned(),
            PluginContributionFamilyKind::Surface => "surface".to_owned(),
            PluginContributionFamilyKind::Setting => "setting".to_owned(),
            PluginContributionFamilyKind::ViewBinding => "view_binding".to_owned(),
            PluginContributionFamilyKind::ThemeToken => "theme_token".to_owned(),
            PluginContributionFamilyKind::Icon => "icon".to_owned(),
            PluginContributionFamilyKind::CommandProjection => "command_projection".to_owned(),
            PluginContributionFamilyKind::TaskPresentation => "task_presentation".to_owned(),
            PluginContributionFamilyKind::RuntimeOutcomeProjection => {
                "runtime_outcome_projection".to_owned()
            }
            PluginContributionFamilyKind::NativeCapabilityRequest => {
                "native_capability_request".to_owned()
            }
            PluginContributionFamilyKind::UnsupportedForDiagnostics(family) => {
                format!("unsupported:{family}")
            }
            PluginContributionFamilyKind::GlobalMutationHookForDiagnostics => {
                "global_mutation_hook".to_owned()
            }
        }
    }

    fn known(kind: PluginContributionFamilyKind) -> Self {
        Self { kind }
    }
}
