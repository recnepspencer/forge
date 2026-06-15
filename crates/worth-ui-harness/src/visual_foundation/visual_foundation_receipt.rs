use crate::theme::{HarnessVisualThemeReceipt, HarnessVisualTokenRole};

use super::{HarnessCommandProjectionVisualRole, HarnessRuntimeOutcomeVisualRole};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HarnessVisualFoundationReceipt {
    theme: HarnessVisualThemeReceipt,
    icon_count: usize,
    command_projection_roles: Vec<HarnessCommandProjectionVisualRole>,
    runtime_outcome_roles: Vec<HarnessRuntimeOutcomeVisualRole>,
}

impl HarnessVisualFoundationReceipt {
    pub(crate) fn new(
        theme: HarnessVisualThemeReceipt,
        icon_count: usize,
        command_projection_roles: Vec<HarnessCommandProjectionVisualRole>,
        runtime_outcome_roles: Vec<HarnessRuntimeOutcomeVisualRole>,
    ) -> Self {
        Self {
            theme,
            icon_count,
            command_projection_roles,
            runtime_outcome_roles,
        }
    }

    pub fn theme(&self) -> &HarnessVisualThemeReceipt {
        &self.theme
    }

    pub fn icon_count(&self) -> usize {
        self.icon_count
    }

    pub fn command_projection_count(&self) -> usize {
        self.command_projection_roles.len()
    }

    pub fn covers_token_role(&self, role: HarnessVisualTokenRole) -> bool {
        self.theme.covers(role)
    }

    pub fn covers_command_projection_role(&self, role: HarnessCommandProjectionVisualRole) -> bool {
        self.command_projection_roles.contains(&role)
    }

    pub fn covers_runtime_outcome_role(&self, role: HarnessRuntimeOutcomeVisualRole) -> bool {
        self.runtime_outcome_roles.contains(&role)
    }
}
