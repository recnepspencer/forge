use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};

use crate::runtime::WorthQueryGraphObligationRegistration;

use super::super::lookup::{
    WorthQueryGraphObligationOperatingWorldLookupKey, WorthQueryGraphObligationTouchLookupKey,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationIndexEntry {
    touch_key_kind: &'static str,
    touch_key_value: Option<String>,
    operating_world_key: &'static str,
    registration: WorthQueryGraphObligationRegistration,
    entry_digest: WorthQueryEvidenceIdentity,
}

impl WorthQueryGraphObligationIndexEntry {
    pub(super) fn new(
        touch_key: &WorthQueryGraphObligationTouchLookupKey,
        operating_world_key: WorthQueryGraphObligationOperatingWorldLookupKey,
        registration: WorthQueryGraphObligationRegistration,
    ) -> Self {
        let touch_key_value = touch_key.terminal_value_projection();
        let entry_digest =
            worth_query_evidence_identity(WorthQueryEvidenceScope::GraphObligationIndexEntry)
                .field_shape(
                    WorthQueryEvidenceTag::new("touch_key_kind"),
                    touch_key.as_kind_str(),
                )
                .optional_value(
                    WorthQueryEvidenceTag::new("touch_key_value"),
                    touch_key_value.as_deref(),
                )
                .field_shape(
                    WorthQueryEvidenceTag::new("operating_world_key"),
                    operating_world_key.as_str(),
                )
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("registration"),
                    registration.registration_evidence_digest(),
                )
                .seal();
        Self {
            touch_key_kind: touch_key.as_kind_str(),
            touch_key_value,
            operating_world_key: operating_world_key.as_str(),
            registration,
            entry_digest,
        }
    }

    pub fn touch_key_kind(&self) -> &'static str {
        self.touch_key_kind
    }

    pub fn terminal_touch_key_value_projection(&self) -> Option<&str> {
        self.touch_key_value.as_deref()
    }

    pub fn operating_world_key(&self) -> &'static str {
        self.operating_world_key
    }

    pub fn registration(&self) -> &WorthQueryGraphObligationRegistration {
        &self.registration
    }

    pub fn entry_digest(&self) -> &str {
        self.entry_digest.as_str()
    }

    pub(crate) fn entry_evidence_digest(&self) -> &WorthQueryEvidenceIdentity {
        &self.entry_digest
    }
}
