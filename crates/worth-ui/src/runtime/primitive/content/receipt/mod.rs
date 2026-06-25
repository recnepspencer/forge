use crate::runtime::WorthUiRuntimeFactId;

use super::anatomy::WorthUiPrimitiveContentAnatomyReceipt;
use super::graph_consumption::WorthUiPrimitiveProvedContentAnatomy;
use super::participation::WorthUiPrimitiveContentParticipationPosture;
use super::{WorthUiPrimitiveContentKind, WorthUiPrimitiveContentRole};
pub use icon_paint_command::WorthUiPrimitiveContentIconPaintCommand;
pub use image_asset_receipt::WorthUiPrimitiveImageAssetReceipt;
pub use item::{
    WorthUiPrimitiveBadgeContentItem, WorthUiPrimitiveContentIconRenderPosture,
    WorthUiPrimitiveContentItem, WorthUiPrimitiveContentItemKind,
    WorthUiPrimitiveDividerContentItem, WorthUiPrimitiveIconContentItem,
    WorthUiPrimitiveImageContentItem, WorthUiPrimitiveSpacerContentItem,
    WorthUiPrimitiveTextContentItem,
};

mod icon_paint_command;
mod image_asset_receipt;
mod item;
mod resolution;

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveContentReceipt {
    kind: WorthUiPrimitiveContentKind,
    items: Vec<WorthUiPrimitiveContentItem>,
    participation: WorthUiPrimitiveContentParticipationPosture,
    role: WorthUiPrimitiveContentRole,
    accessibility_name: Option<String>,
    dependency_fact: WorthUiRuntimeFactId,
    receipt_digest: u64,
}

impl WorthUiPrimitiveContentReceipt {
    pub(crate) fn new(
        kind: WorthUiPrimitiveContentKind,
        items: Vec<WorthUiPrimitiveContentItem>,
        participation: WorthUiPrimitiveContentParticipationPosture,
        role: WorthUiPrimitiveContentRole,
        accessibility_name: Option<String>,
        dependency_fact: WorthUiRuntimeFactId,
        receipt_digest: u64,
    ) -> Self {
        Self {
            kind,
            items,
            participation,
            role,
            accessibility_name,
            dependency_fact,
            receipt_digest,
        }
    }

    pub fn items(&self) -> &[WorthUiPrimitiveContentItem] {
        &self.items
    }

    pub fn kind(&self) -> WorthUiPrimitiveContentKind {
        self.kind
    }

    pub fn accessibility_name(&self) -> Option<&str> {
        self.accessibility_name.as_deref()
    }

    pub fn participation(&self) -> WorthUiPrimitiveContentParticipationPosture {
        self.participation
    }

    pub fn role(&self) -> WorthUiPrimitiveContentRole {
        self.role
    }

    pub fn dependency_fact(&self) -> &WorthUiRuntimeFactId {
        &self.dependency_fact
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }

    pub fn text(&self) -> &str {
        self.items
            .iter()
            .find_map(|item| item.as_text().map(|text| text.text()))
            .unwrap_or("")
    }

    pub fn anatomy_receipt(&self) -> WorthUiPrimitiveContentAnatomyReceipt {
        WorthUiPrimitiveContentAnatomyReceipt::from_content_receipt(self)
    }

    pub fn proved_anatomy(&self) -> WorthUiPrimitiveProvedContentAnatomy {
        WorthUiPrimitiveProvedContentAnatomy::from_content_receipt(self)
    }
}
