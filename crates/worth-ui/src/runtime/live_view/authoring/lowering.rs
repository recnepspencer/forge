use crate::runtime::WorthUiLiveViewTargetBinding;

use super::super::{
    WorthUiLiveViewDeclaration, WorthUiLiveViewStateBindingDeclaration, WorthUiLiveViewStateFactId,
};
use super::document::WorthUiAuthoredLiveViewDeclaration;
use super::tokens::{authored_live_view_access, authored_live_view_value_kind};

pub(super) fn lower_authored_live_view_declaration(
    authored: &WorthUiAuthoredLiveViewDeclaration,
    target_binding: WorthUiLiveViewTargetBinding,
) -> WorthUiLiveViewDeclaration {
    authored.bindings().iter().fold(
        WorthUiLiveViewDeclaration::new(authored.live_view_id(), target_binding),
        |declaration, binding| {
            declaration.with_state_binding(WorthUiLiveViewStateBindingDeclaration::new(
                binding.binding_id(),
                WorthUiLiveViewStateFactId::new(binding.state_fact())
                    .expect("authored state fact was admitted before lowering"),
                authored_live_view_value_kind(binding.value_kind()),
                authored_live_view_access(binding.access()),
            ))
        },
    )
}
