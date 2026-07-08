use crate::evidence::shared::evidence_handle::UiEvidenceHandle;
use crate::evidence::shared::evidence_identity::UiEvidenceIdentity;
use worth_ui_inspection::UiEvidenceFamily;

pub(crate) fn evidence_identity(family: UiEvidenceFamily, digest: u64) -> UiEvidenceIdentity {
    UiEvidenceIdentity::new(family, digest)
}

pub(crate) fn evidence_handle(
    family: UiEvidenceFamily,
    identity: UiEvidenceIdentity,
    handle_digest: u64,
) -> UiEvidenceHandle {
    UiEvidenceHandle::new(family, identity, handle_digest)
}