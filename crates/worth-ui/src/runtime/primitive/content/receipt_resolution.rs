use crate::capability::{IconId, IconSourceDescriptor};
use crate::runtime::{
    WorthUiPrimitiveContentAdmissionReceipt, WorthUiPrimitiveContentIconRenderPosture,
    WorthUiPrimitiveContentItem, WorthUiPrimitiveContentItemKind, WorthUiPrimitiveContentReceipt,
    WorthUiRuntimeFactId, WorthUiRuntimeHost,
};

use super::digest::primitive_content_receipt_digest;
use super::report::WorthUiValidatedPrimitiveContentPropSet;

impl WorthUiPrimitiveContentAdmissionReceipt {
    pub(crate) fn resolved_receipt(
        &self,
        runtime: &WorthUiRuntimeHost,
    ) -> WorthUiPrimitiveContentReceipt {
        let prop_set = self.prop_set();
        let mut items = Vec::new();
        for kind in prop_set.order() {
            match kind {
                WorthUiPrimitiveContentItemKind::Text if !prop_set.text().is_empty() => {
                    items.push(WorthUiPrimitiveContentItem::text(
                        prop_set.text(),
                        prop_set.text_size_token(),
                        prop_set.text_size_points(),
                    ));
                }
                WorthUiPrimitiveContentItemKind::Icon => {
                    if let Some(icon_id) = prop_set.icon_id() {
                        if let Some(item) = icon_item(runtime, icon_id, prop_set) {
                            items.push(item);
                        }
                    }
                }
                WorthUiPrimitiveContentItemKind::Spacer => {
                    items.push(WorthUiPrimitiveContentItem::spacer(
                        prop_set.spacer_size_token(),
                        prop_set.spacer_size_points(),
                    ));
                }
                WorthUiPrimitiveContentItemKind::Badge => {
                    if let Some(text) = prop_set.badge_text() {
                        items.push(WorthUiPrimitiveContentItem::badge(
                            text,
                            prop_set.text_size_token(),
                            prop_set.text_size_points(),
                        ));
                    }
                }
                WorthUiPrimitiveContentItemKind::Divider => {
                    items.push(WorthUiPrimitiveContentItem::divider(
                        prop_set.divider_thickness_token(),
                        prop_set.divider_thickness_points(),
                    ));
                }
                _ => {}
            }
        }
        let mut receipt = WorthUiPrimitiveContentReceipt::new(
            prop_set.kind(),
            items,
            prop_set.accessibility_name().map(str::to_owned),
            WorthUiRuntimeFactId::primitive_content(self.surface_id()),
            0,
        );
        let digest = primitive_content_receipt_digest(self.admission_digest(), &receipt);
        receipt = WorthUiPrimitiveContentReceipt::new(
            receipt.kind(),
            receipt.items().to_vec(),
            receipt.accessibility_name().map(str::to_owned),
            receipt.dependency_fact().clone(),
            digest,
        );
        receipt
    }
}

fn icon_item(
    runtime: &WorthUiRuntimeHost,
    icon_id: &IconId,
    prop_set: &WorthUiValidatedPrimitiveContentPropSet,
) -> Option<WorthUiPrimitiveContentItem> {
    let descriptor = runtime.active_capability_snapshot().icons().get(icon_id)?;
    let source = descriptor.source()?;
    Some(WorthUiPrimitiveContentItem::icon(
        icon_id.as_str(),
        format!("{:?}", source.kind()),
        source.provider(),
        source.source_key(),
        render_posture(source),
        prop_set.icon_size_token(),
        prop_set.icon_size_points(),
        prop_set.icon_stroke_token(),
        prop_set.icon_stroke_width_points(),
    ))
}

fn render_posture(source: &IconSourceDescriptor) -> WorthUiPrimitiveContentIconRenderPosture {
    if source.native_vector_support().supports_native_vector() {
        WorthUiPrimitiveContentIconRenderPosture::NativeVector
    } else {
        WorthUiPrimitiveContentIconRenderPosture::SymbolFallback
    }
}
