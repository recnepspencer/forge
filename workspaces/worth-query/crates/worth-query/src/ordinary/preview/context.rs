use crate::ordinary::workflow;
use crate::runtime::{WorthQueryEffectPolicy, WorthQueryWorkspace};
use crate::session_label::WorthQuerySessionLabel;

#[derive(Debug, Eq, PartialEq)]
pub struct WorthQueryReadOnlyPreviewContext {
    pub(crate) authority: crate::runtime::WorthQueryOrdinaryAuthorityAdmission,
}

#[derive(Debug, Eq, PartialEq)]
pub struct WorthQueryPromotionPreviewContext {
    pub(crate) authority: crate::runtime::WorthQueryOrdinaryAuthorityAdmission,
}

pub type WorthQueryPreviewContextStop = workflow::WorthQueryWorkflowContextStop;

pub fn read_only(
    workspace: &WorthQueryWorkspace,
    label: WorthQuerySessionLabel,
) -> Result<WorthQueryReadOnlyPreviewContext, WorthQueryPreviewContextStop> {
    workspace
        .capture_ordinary_preview_authority(label, WorthQueryEffectPolicy::DeriveOnly)
        .map(|authority| WorthQueryReadOnlyPreviewContext { authority })
        .map_err(workflow::WorthQueryWorkflowContextStop::from_runtime)
}

pub fn promotion(
    workspace: &WorthQueryWorkspace,
    label: WorthQuerySessionLabel,
) -> Result<WorthQueryPromotionPreviewContext, WorthQueryPreviewContextStop> {
    workflow::preview(workspace, label).map(|context| WorthQueryPromotionPreviewContext {
        authority: context.authority,
    })
}
