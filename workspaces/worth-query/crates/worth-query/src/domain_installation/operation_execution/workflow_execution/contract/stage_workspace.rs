use crate::runtime::WorthQueryWorkspace;

use super::WorthQueryWorkflowEffectEvidence;

/// The workflow-stage workspace surface. Its only executable doors live on
/// the stage context and therefore enforce declared reads and effects first.
pub struct WorthQueryWorkflowStageWorkspace<'a> {
    pub(super) workspace: &'a mut WorthQueryWorkspace,
    pub(super) installed_read_executions: usize,
    pub(super) executed_effects: Vec<WorthQueryWorkflowEffectEvidence>,
}

impl<'a> WorthQueryWorkflowStageWorkspace<'a> {
    pub(crate) fn new(workspace: &'a mut WorthQueryWorkspace) -> Self {
        Self {
            workspace,
            installed_read_executions: 0,
            executed_effects: Vec::new(),
        }
    }

    pub(crate) fn installed_read_executions(&self) -> usize {
        self.installed_read_executions
    }

    pub(crate) fn into_executed_effects(self) -> Vec<WorthQueryWorkflowEffectEvidence> {
        self.executed_effects
    }
}
