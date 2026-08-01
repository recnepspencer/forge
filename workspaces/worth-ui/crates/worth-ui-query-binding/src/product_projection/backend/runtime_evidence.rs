use worth_query::facade::{foundation, runtime};

pub(super) struct WorthUiScalarProjectionUnsupportedPreview;

impl runtime::WorthQueryRuntimePreviewBasisAdapter for WorthUiScalarProjectionUnsupportedPreview {
    fn admit_preview_basis(
        &self,
        _label: &runtime::WorthQuerySessionLabel,
        _effect_policy: runtime::WorthQueryEffectPolicy,
        _authority: &runtime::WorthQueryRuntimeEvidenceAuthority,
    ) -> Result<runtime::WorthQueryPreviewBasisAdmission, foundation::WorthQueryWorkspaceError>
    {
        Err(foundation::WorthQueryWorkspaceError::new(
            "Worth UI product projection does not advertise preview support",
        ))
    }
}

pub(super) struct WorthUiScalarProjectionUnsupportedInspection;

impl runtime::WorthQueryRuntimeInspectorEvidenceAdapter
    for WorthUiScalarProjectionUnsupportedInspection
{
    fn inspect_write_receipt(
        &self,
        _receipt: &runtime::WorthQueryWriteReceipt,
        _authority: &runtime::WorthQueryRuntimeEvidenceAuthority,
    ) -> Result<runtime::WorthQueryRuntimeInspectionEvidence, foundation::WorthQueryWorkspaceError>
    {
        Err(foundation::WorthQueryWorkspaceError::new(
            "Worth UI product projection does not advertise Query inspection support",
        ))
    }
}
