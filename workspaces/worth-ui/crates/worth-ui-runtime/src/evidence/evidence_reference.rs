use super::{
    UiEvidenceAuthorityBinding, UiEvidenceFamily, UiEvidenceHandle, UiEvidenceIdentity,
    UiEvidenceMaterializationPosture, UiEvidenceRetentionPosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiEvidenceRef {
    family: UiEvidenceFamily,
    identity: UiEvidenceIdentity,
    authority_binding: UiEvidenceAuthorityBinding,
    materialization_posture: UiEvidenceMaterializationPosture,
    retention_posture: UiEvidenceRetentionPosture,
    handle: UiEvidenceHandle,
}

impl UiEvidenceRef {
    pub(crate) fn new(
        family: UiEvidenceFamily,
        identity: UiEvidenceIdentity,
        authority_binding: UiEvidenceAuthorityBinding,
        materialization_posture: UiEvidenceMaterializationPosture,
        retention_posture: UiEvidenceRetentionPosture,
        handle: UiEvidenceHandle,
    ) -> Self {
        Self {
            family,
            identity,
            authority_binding,
            materialization_posture,
            retention_posture,
            handle,
        }
    }

    pub fn family(&self) -> UiEvidenceFamily {
        self.family
    }

    pub fn identity(&self) -> UiEvidenceIdentity {
        self.identity
    }

    pub fn authority_binding(&self) -> UiEvidenceAuthorityBinding {
        self.authority_binding
    }

    pub fn authority_generation(&self) -> worth_ui_inspection::UiEvidenceAuthorityGeneration {
        self.authority_binding.authority_generation()
    }

    pub fn materialization_posture(&self) -> UiEvidenceMaterializationPosture {
        self.materialization_posture
    }

    pub fn retention_posture(&self) -> UiEvidenceRetentionPosture {
        self.retention_posture
    }

    pub fn handle(&self) -> UiEvidenceHandle {
        self.handle
    }
}
