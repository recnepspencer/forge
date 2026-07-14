use crate::runtime::{
    WorthQueryOrdinaryAuthorityAdmission, WorthQueryOrdinaryAuthorityFamily,
    WorthQueryRuntimeError, WorthQueryWorkspace,
};
use crate::session_label::WorthQuerySessionLabel;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryWorkflowContext {
    pub(crate) authority: WorthQueryOrdinaryAuthorityAdmission,
}

#[derive(Debug)]
pub struct WorthQueryWorkflowContextStop {
    error: WorthQueryRuntimeError,
}

impl WorthQueryWorkflowContextStop {
    pub fn error(&self) -> &WorthQueryRuntimeError {
        &self.error
    }

    pub fn next_action(&self) -> super::WorthQueryWorkflowNextAction {
        super::WorthQueryWorkflowNextAction::ProvideAuthority
    }
}

pub fn preview(
    workspace: &WorthQueryWorkspace,
    label: WorthQuerySessionLabel,
) -> Result<WorthQueryWorkflowContext, WorthQueryWorkflowContextStop> {
    let authority = workspace
        .capture_ordinary_preview_authority(label)
        .map_err(|error| WorthQueryWorkflowContextStop { error })?;
    debug_assert_eq!(
        authority.family(),
        WorthQueryOrdinaryAuthorityFamily::Preview
    );
    Ok(WorthQueryWorkflowContext { authority })
}
