#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UiMountedPaintCommandIdentity {
    mounted_instance: crate::UiMountedInstanceIdentity,
    family: UiMountedPaintCommandFamily,
    semantic_slot: u16,
    collection_row: Option<[u8; 32]>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum UiMountedPaintCommandFamily {
    FilledRect,
    SemanticText,
}

#[derive(Clone, Debug, PartialEq)]
pub enum UiMountedPaintCommand {
    FilledRect {
        identity: UiMountedPaintCommandIdentity,
        table_index: u16,
        mechanic: crate::UiMountedFilledRectMechanic,
    },
    SemanticText {
        identity: UiMountedPaintCommandIdentity,
        table_index: u16,
        mechanic: crate::UiMountedSemanticTextMechanic,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum UiMountedPaintCommandChange {
    Insert(UiMountedPaintCommand),
    Replace(UiMountedPaintCommand),
    Remove(UiMountedPaintCommandIdentity),
}

impl UiMountedPaintCommandIdentity {
    #[doc(hidden)]
    pub fn filled_rect(mechanic: &crate::UiMountedFilledRectMechanic) -> Self {
        Self {
            mounted_instance: mechanic.mounted_instance(),
            family: UiMountedPaintCommandFamily::FilledRect,
            semantic_slot: 0,
            collection_row: None,
        }
    }

    #[doc(hidden)]
    pub fn semantic_text(mechanic: &crate::UiMountedSemanticTextMechanic) -> Self {
        let semantic_slot = match mechanic.slot() {
            crate::UiSemanticTextSlot::Value => 0,
            crate::UiSemanticTextSlot::CollectionValue {
                selected_field_ordinal,
            } => selected_field_ordinal.saturating_add(1),
            crate::UiSemanticTextSlot::Posture => u16::MAX,
        };
        Self {
            mounted_instance: mechanic.mounted_instance(),
            family: UiMountedPaintCommandFamily::SemanticText,
            semantic_slot,
            collection_row: mechanic
                .collection_row()
                .map(|row| row.correlation_digest()),
        }
    }

    pub const fn mounted_instance(self) -> crate::UiMountedInstanceIdentity {
        self.mounted_instance
    }

    pub(super) fn order_fingerprint(self) -> u64 {
        let family = match self.family {
            UiMountedPaintCommandFamily::FilledRect => 1_u64,
            UiMountedPaintCommandFamily::SemanticText => 2_u64,
        };
        let mut digest = self
            .mounted_instance
            .diagnostic_value()
            .wrapping_mul(0x9e37_79b1_85eb_ca87)
            ^ family
            ^ u64::from(self.semantic_slot);
        if let Some(row) = self.collection_row {
            for chunk in row.chunks_exact(8) {
                digest ^= u64::from_le_bytes(chunk.try_into().expect("eight-byte chunk"));
                digest = digest.rotate_left(13).wrapping_mul(0xff51_afd7_ed55_8ccd);
            }
        }
        digest
    }
}

impl UiMountedPaintCommand {
    pub fn identity(&self) -> UiMountedPaintCommandIdentity {
        match self {
            Self::FilledRect { identity, .. } | Self::SemanticText { identity, .. } => *identity,
        }
    }

    pub const fn table_index(&self) -> u16 {
        match self {
            Self::FilledRect { table_index, .. } | Self::SemanticText { table_index, .. } => {
                *table_index
            }
        }
    }

    pub fn layer_semantic_order(&self) -> u32 {
        match self {
            Self::FilledRect { mechanic, .. } => mechanic.layer_semantic_order(),
            Self::SemanticText { mechanic, .. } => mechanic.layer_semantic_order(),
        }
    }

    pub fn bounds(&self) -> crate::UiMountedCanonicalBox {
        match self {
            Self::FilledRect { mechanic, .. } => mechanic.bounds(),
            Self::SemanticText { mechanic, .. } => mechanic.bounds(),
        }
    }

    pub fn semantic_digest(&self) -> u64 {
        match self {
            Self::FilledRect { mechanic, .. } => mechanic.semantic_digest(),
            Self::SemanticText { mechanic, .. } => mechanic.semantic_digest(),
        }
    }
}
