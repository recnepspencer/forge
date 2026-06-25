use crate::runtime::{
    WorthUiPrimitiveContentIconRenderPosture, WorthUiPrimitiveContentItem,
    WorthUiPrimitiveContentItemKind, WorthUiPrimitiveContentParticipationPosture,
    WorthUiPrimitiveContentReceipt,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPrimitiveContentAccessibilityParticipation {
    Named,
    Decorative,
    Hidden,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveContentAnatomyReceipt {
    items: Vec<WorthUiPrimitiveContentAnatomyItemReceipt>,
    accessibility: WorthUiPrimitiveContentAccessibilityParticipation,
    item_count: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveContentAnatomyItemReceipt {
    item_index: usize,
    item_kind: WorthUiPrimitiveContentItemKind,
    participation: WorthUiPrimitiveContentParticipationPosture,
    accessibility: WorthUiPrimitiveContentAccessibilityParticipation,
    native_vector: Option<WorthUiPrimitiveContentIconRenderPosture>,
    width_points: f32,
    height_points: f32,
    baseline_points: f32,
}

impl WorthUiPrimitiveContentAnatomyReceipt {
    pub(crate) fn from_content_receipt(receipt: &WorthUiPrimitiveContentReceipt) -> Self {
        let accessibility = if !receipt.participation().participates_in_accessibility() {
            WorthUiPrimitiveContentAccessibilityParticipation::Hidden
        } else if receipt.accessibility_name().is_some() {
            WorthUiPrimitiveContentAccessibilityParticipation::Named
        } else {
            WorthUiPrimitiveContentAccessibilityParticipation::Decorative
        };
        let items = receipt
            .items()
            .iter()
            .enumerate()
            .map(|(item_index, item)| {
                WorthUiPrimitiveContentAnatomyItemReceipt::from_content_item(
                    item_index,
                    item,
                    receipt.participation(),
                    accessibility,
                )
            })
            .collect::<Vec<_>>();
        Self {
            item_count: items.len(),
            items,
            accessibility,
        }
    }

    pub fn items(&self) -> &[WorthUiPrimitiveContentAnatomyItemReceipt] {
        &self.items
    }

    pub fn accessibility(&self) -> WorthUiPrimitiveContentAccessibilityParticipation {
        self.accessibility
    }

    pub fn item_count(&self) -> usize {
        self.item_count
    }
}

impl WorthUiPrimitiveContentAnatomyItemReceipt {
    fn from_content_item(
        item_index: usize,
        item: &WorthUiPrimitiveContentItem,
        participation: WorthUiPrimitiveContentParticipationPosture,
        receipt_accessibility: WorthUiPrimitiveContentAccessibilityParticipation,
    ) -> Self {
        let accessibility = match item.kind() {
            WorthUiPrimitiveContentItemKind::Icon if receipt_accessibility.is_named() => {
                WorthUiPrimitiveContentAccessibilityParticipation::Hidden
            }
            _ => receipt_accessibility,
        };
        Self {
            item_index,
            item_kind: item.kind(),
            participation,
            accessibility,
            native_vector: item.as_icon().map(|icon| icon.render_posture()),
            width_points: item.width_points(),
            height_points: item.height_points(),
            baseline_points: item.baseline_points(),
        }
    }

    pub fn item_index(&self) -> usize {
        self.item_index
    }

    pub fn item_kind(&self) -> WorthUiPrimitiveContentItemKind {
        self.item_kind
    }

    pub fn participation(&self) -> WorthUiPrimitiveContentParticipationPosture {
        self.participation
    }

    pub fn accessibility(&self) -> WorthUiPrimitiveContentAccessibilityParticipation {
        self.accessibility
    }

    pub fn native_vector(&self) -> Option<WorthUiPrimitiveContentIconRenderPosture> {
        self.native_vector
    }

    pub fn width_points(&self) -> f32 {
        self.width_points
    }

    pub fn height_points(&self) -> f32 {
        self.height_points
    }

    pub fn baseline_points(&self) -> f32 {
        self.baseline_points
    }
}

impl WorthUiPrimitiveContentAccessibilityParticipation {
    fn is_named(self) -> bool {
        self == Self::Named
    }
}
