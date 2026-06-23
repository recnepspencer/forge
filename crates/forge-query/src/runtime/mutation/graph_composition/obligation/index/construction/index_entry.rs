use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

use crate::runtime::ForgeQueryGraphObligationRegistration;

use super::super::lookup::{
    ForgeQueryGraphObligationOperatingWorldLookupKey, ForgeQueryGraphObligationTouchLookupKey,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphObligationIndexEntry {
    touch_key_kind: &'static str,
    touch_key_value: Option<String>,
    operating_world_key: &'static str,
    registration: ForgeQueryGraphObligationRegistration,
    entry_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryGraphObligationIndexEntry {
    pub(super) fn new(
        touch_key: &ForgeQueryGraphObligationTouchLookupKey,
        operating_world_key: ForgeQueryGraphObligationOperatingWorldLookupKey,
        registration: ForgeQueryGraphObligationRegistration,
    ) -> Self {
        let touch_key_value = touch_key.terminal_value_projection();
        let entry_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::GraphObligationIndexEntry)
                .field_shape(
                    ForgeQueryEvidenceTag::new("touch_key_kind"),
                    touch_key.as_kind_str(),
                )
                .optional_value(
                    ForgeQueryEvidenceTag::new("touch_key_value"),
                    touch_key_value.as_deref(),
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("operating_world_key"),
                    operating_world_key.as_str(),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("registration"),
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

    pub fn registration(&self) -> &ForgeQueryGraphObligationRegistration {
        &self.registration
    }

    pub fn entry_digest(&self) -> &str {
        self.entry_digest.as_str()
    }

    pub(crate) fn entry_evidence_digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.entry_digest
    }
}
