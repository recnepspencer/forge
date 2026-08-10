use super::super::classification_error;
use super::super::{SubscriptionSupportFamilyId, SubscriptionSupportFamilyKind};
use crate::failure::StoreError;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportFamilyVersionWindow {
    family_id: SubscriptionSupportFamilyId,
    family_kind: SubscriptionSupportFamilyKind,
    minimum_reader_version: u16,
    maximum_payload_version: u16,
}

#[allow(dead_code)]
impl SupportFamilyVersionWindow {
    pub(crate) fn new(
        family_id: SubscriptionSupportFamilyId,
        family_kind: SubscriptionSupportFamilyKind,
        minimum_reader_version: u16,
        maximum_payload_version: u16,
    ) -> Result<Self, StoreError> {
        if minimum_reader_version == 0 || maximum_payload_version == 0 {
            return Err(classification_error(
                "subscription-support compatibility version windows require non-zero versions",
            ));
        }
        if minimum_reader_version > maximum_payload_version {
            return Err(classification_error(
                "subscription-support compatibility version windows cannot require a reader newer than the payload",
            ));
        }
        Ok(Self {
            family_id,
            family_kind,
            minimum_reader_version,
            maximum_payload_version,
        })
    }

    pub fn family_id(&self) -> &SubscriptionSupportFamilyId {
        &self.family_id
    }

    pub fn family_kind(&self) -> SubscriptionSupportFamilyKind {
        self.family_kind
    }

    pub fn minimum_reader_version(&self) -> u16 {
        self.minimum_reader_version
    }

    pub fn maximum_payload_version(&self) -> u16 {
        self.maximum_payload_version
    }
}
