use crate::ordinary::workflow;
use crate::runtime::WorthQueryWorkspace;
use crate::session_label::WorthQuerySessionLabel;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryPreviewContext {
    pub(crate) authority: crate::runtime::WorthQueryOrdinaryAuthorityAdmission,
}

pub type WorthQueryPreviewContextStop = workflow::WorthQueryWorkflowContextStop;

pub fn for_session(
    workspace: &WorthQueryWorkspace,
    label: WorthQuerySessionLabel,
) -> Result<WorthQueryPreviewContext, WorthQueryPreviewContextStop> {
    workflow::preview(workspace, label).map(|context| WorthQueryPreviewContext {
        authority: context.authority,
    })
}
