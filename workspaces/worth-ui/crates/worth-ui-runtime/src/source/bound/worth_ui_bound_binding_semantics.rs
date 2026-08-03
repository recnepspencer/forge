use crate::capability::{
    AdmittedCapability, CommandDescriptor, CommandId, CommandProjectionDescriptor,
    CommandProjectionId, FrozenThemeTokenEntry, FrozenViewBindingEntry, IconDescriptor, IconId,
    QueryDenialPresentation, ThemeTokenId, ViewBindingId,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiBoundIconReference {
    icon: AdmittedCapability<IconId>,
    descriptor: IconDescriptor,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiBoundCommandProjectionReference {
    command_projection: AdmittedCapability<CommandProjectionId>,
    descriptor: CommandProjectionDescriptor,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiBoundCommandSemantics {
    icon: Option<WorthUiBoundIconReference>,
    projection_eligibility: Option<WorthUiBoundCommandProjectionReference>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiBoundCommandReference {
    command: AdmittedCapability<CommandId>,
    descriptor: CommandDescriptor,
    semantics: WorthUiBoundCommandSemantics,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiBoundQueryViewSemantics {
    definition: worth_ui_query_binding::WorthUiQueryViewDefinition,
    denial_presentation: QueryDenialPresentation,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiBoundViewBindingReference {
    view_binding: AdmittedCapability<ViewBindingId>,
    entry: FrozenViewBindingEntry,
    query_semantics: WorthUiBoundQueryViewSemantics,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct WorthUiBoundSurfaceSemantics {
    icon: Option<WorthUiBoundIconReference>,
    command_slots: Vec<WorthUiBoundCommandReference>,
    view_binding: Option<WorthUiBoundViewBindingReference>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiBoundThemeTokenSemantics {
    resolved_target_theme_token: AdmittedCapability<ThemeTokenId>,
    resolved_target_entry: FrozenThemeTokenEntry,
}

impl WorthUiBoundCommandReference {
    pub(crate) fn new(
        command: AdmittedCapability<CommandId>,
        descriptor: CommandDescriptor,
        semantics: WorthUiBoundCommandSemantics,
    ) -> Self {
        Self {
            command,
            descriptor,
            semantics,
        }
    }

    pub(crate) fn command(&self) -> &AdmittedCapability<CommandId> {
        &self.command
    }

    pub(crate) fn descriptor(&self) -> &CommandDescriptor {
        &self.descriptor
    }

    pub(crate) fn semantics(&self) -> &WorthUiBoundCommandSemantics {
        &self.semantics
    }
}

impl WorthUiBoundQueryViewSemantics {
    pub(crate) fn new(
        definition: worth_ui_query_binding::WorthUiQueryViewDefinition,
        denial_presentation: QueryDenialPresentation,
    ) -> Self {
        Self {
            definition,
            denial_presentation,
        }
    }

    pub(crate) fn definition(&self) -> &worth_ui_query_binding::WorthUiQueryViewDefinition {
        &self.definition
    }

    pub(crate) fn denial_presentation(&self) -> &QueryDenialPresentation {
        &self.denial_presentation
    }
}

impl WorthUiBoundIconReference {
    pub(crate) fn new(icon: AdmittedCapability<IconId>, descriptor: IconDescriptor) -> Self {
        Self { icon, descriptor }
    }

    pub(crate) fn icon(&self) -> &AdmittedCapability<IconId> {
        &self.icon
    }

    pub(crate) fn descriptor(&self) -> &IconDescriptor {
        &self.descriptor
    }
}

impl WorthUiBoundCommandProjectionReference {
    pub(crate) fn new(
        command_projection: AdmittedCapability<CommandProjectionId>,
        descriptor: CommandProjectionDescriptor,
    ) -> Self {
        Self {
            command_projection,
            descriptor,
        }
    }

    pub(crate) fn command_projection(&self) -> &AdmittedCapability<CommandProjectionId> {
        &self.command_projection
    }

    pub(crate) fn descriptor(&self) -> &CommandProjectionDescriptor {
        &self.descriptor
    }
}

impl WorthUiBoundCommandSemantics {
    pub(crate) fn new(
        icon: Option<WorthUiBoundIconReference>,
        projection_eligibility: Option<WorthUiBoundCommandProjectionReference>,
    ) -> Self {
        Self {
            icon,
            projection_eligibility,
        }
    }

    pub(crate) fn icon(&self) -> Option<&WorthUiBoundIconReference> {
        self.icon.as_ref()
    }

    pub(crate) fn projection_eligibility(&self) -> Option<&WorthUiBoundCommandProjectionReference> {
        self.projection_eligibility.as_ref()
    }
}

impl WorthUiBoundViewBindingReference {
    pub(crate) fn new(
        view_binding: AdmittedCapability<ViewBindingId>,
        entry: FrozenViewBindingEntry,
        query_semantics: WorthUiBoundQueryViewSemantics,
    ) -> Self {
        Self {
            view_binding,
            entry,
            query_semantics,
        }
    }

    pub(crate) fn view_binding(&self) -> &AdmittedCapability<ViewBindingId> {
        &self.view_binding
    }

    pub(crate) fn entry(&self) -> &FrozenViewBindingEntry {
        &self.entry
    }

    pub(crate) fn query_semantics(&self) -> &WorthUiBoundQueryViewSemantics {
        &self.query_semantics
    }
}

impl WorthUiBoundSurfaceSemantics {
    pub(crate) fn new(
        icon: Option<WorthUiBoundIconReference>,
        command_slots: Vec<WorthUiBoundCommandReference>,
        view_binding: Option<WorthUiBoundViewBindingReference>,
    ) -> Self {
        Self {
            icon,
            command_slots,
            view_binding,
        }
    }

    pub(crate) fn icon(&self) -> Option<&WorthUiBoundIconReference> {
        self.icon.as_ref()
    }

    pub(crate) fn command_slots(&self) -> &[WorthUiBoundCommandReference] {
        &self.command_slots
    }

    pub(crate) fn view_binding(&self) -> Option<&WorthUiBoundViewBindingReference> {
        self.view_binding.as_ref()
    }
}

impl WorthUiBoundThemeTokenSemantics {
    pub(crate) fn new(
        resolved_target_theme_token: AdmittedCapability<ThemeTokenId>,
        resolved_target_entry: FrozenThemeTokenEntry,
    ) -> Self {
        Self {
            resolved_target_theme_token,
            resolved_target_entry,
        }
    }

    pub(crate) fn resolved_target_theme_token(&self) -> &AdmittedCapability<ThemeTokenId> {
        &self.resolved_target_theme_token
    }

    pub(crate) fn resolved_target_entry(&self) -> &FrozenThemeTokenEntry {
        &self.resolved_target_entry
    }
}
