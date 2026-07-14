use crate::evidence::shared::evidence_handle::UiEvidenceHandle;
use crate::evidence::shared::evidence_identity::UiEvidenceIdentity;
use crate::evidence::shared::evidence_reference::UiEvidenceRef;
use worth_ui_inspection::{
    UiEvidenceAuthorityBinding, UiEvidenceFamily, UiEvidenceMaterializationPosture,
    UiEvidenceRetentionPosture,
};

pub(crate) fn evidence_ref(
    family: UiEvidenceFamily,
    identity: UiEvidenceIdentity,
    authority_binding: UiEvidenceAuthorityBinding,
    materialization_posture: UiEvidenceMaterializationPosture,
    retention_posture: UiEvidenceRetentionPosture,
    handle: UiEvidenceHandle,
) -> UiEvidenceRef {
    UiEvidenceRef::new(
        family,
        identity,
        authority_binding,
        materialization_posture,
        retention_posture,
        handle,
    )
}

pub(crate) fn with_retention_posture(
    evidence_ref: UiEvidenceRef,
    retention_posture: UiEvidenceRetentionPosture,
) -> UiEvidenceRef {
    UiEvidenceRef::new(
        evidence_ref.family(),
        evidence_ref.identity(),
        evidence_ref.authority_binding(),
        evidence_ref.materialization_posture(),
        retention_posture,
        evidence_ref.handle(),
    )
}
