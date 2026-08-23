use worth_ui_host_contract::{UiMountedSemanticTextMechanic, UiSemanticTextSlot};

use super::{UiMountedCollectionTextKey, UiMountedSemanticMechanicKey};

pub(super) fn row_digest(value: u64) -> u64 {
    value.wrapping_mul(0x9e37_79b1_85eb_ca87).rotate_left(19)
}

impl UiMountedSemanticMechanicKey {
    pub(super) fn for_row(row: &UiMountedSemanticTextMechanic) -> Self {
        Self {
            slot: row.slot(),
            collection: row.collection_row().map(|row| row.correlation_digest()),
        }
    }

    pub(super) fn collection(row: UiMountedCollectionTextKey, field: u16) -> Self {
        Self {
            slot: UiSemanticTextSlot::CollectionValue {
                selected_field_ordinal: field,
            },
            collection: Some(row.correlation_digest()),
        }
    }

    pub(super) const fn posture() -> Self {
        Self {
            slot: UiSemanticTextSlot::Posture,
            collection: None,
        }
    }
}
