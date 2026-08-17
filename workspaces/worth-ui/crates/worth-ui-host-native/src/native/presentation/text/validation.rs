//! Honest pre-effect denial while glyph-run presentation remains a later gate.

use worth_ui_host_contract::{
    UiHostSurfacePresentationDenial, UiMountedPaintCommand, UiMountedPaintCommandChange,
    UiMountedPresentationWorkView,
};

pub(crate) fn semantic_text_before_effects_denial(
    work: UiMountedPresentationWorkView<'_>,
) -> Option<UiHostSurfacePresentationDenial> {
    work_contains_semantic_text(work)
        .then_some(UiHostSurfacePresentationDenial::TextAtlasPresentationDeferred)
}

fn work_contains_semantic_text(work: UiMountedPresentationWorkView<'_>) -> bool {
    match work {
        UiMountedPresentationWorkView::Initial(initial) => {
            !initial.projection().semantic_text().rows().is_empty()
                || initial.commands().iter().any(is_semantic_text_command)
        }
        UiMountedPresentationWorkView::Reconstruction(reconstruction) => {
            !reconstruction
                .projection()
                .semantic_text()
                .rows()
                .is_empty()
                || reconstruction
                    .commands()
                    .iter()
                    .any(is_semantic_text_command)
        }
        UiMountedPresentationWorkView::Delta(delta) => {
            delta.changes().iter().any(change_inserts_semantic_text)
        }
        UiMountedPresentationWorkView::Unchanged(_) => false,
    }
}

fn change_inserts_semantic_text(change: &UiMountedPaintCommandChange) -> bool {
    match change {
        UiMountedPaintCommandChange::Insert(command)
        | UiMountedPaintCommandChange::Replace(command) => is_semantic_text_command(command),
        UiMountedPaintCommandChange::Remove(_) => false,
    }
}

fn is_semantic_text_command(command: &UiMountedPaintCommand) -> bool {
    matches!(command, UiMountedPaintCommand::SemanticText { .. })
}
