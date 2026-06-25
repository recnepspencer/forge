use crate::runtime::WorthUiCompositionNodeKind;

pub(super) fn composition_node_identity(kind: WorthUiCompositionNodeKind, id: &str) -> String {
    match kind {
        WorthUiCompositionNodeKind::Control => format!("live_view.control.{id}"),
        WorthUiCompositionNodeKind::Interaction => format!("live_view.interaction.{id}"),
        _ => id.to_owned(),
    }
}
