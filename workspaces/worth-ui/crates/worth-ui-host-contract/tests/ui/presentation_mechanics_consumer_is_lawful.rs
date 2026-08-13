use worth_ui_host_contract::{UiMountedFrameConsumptionView, UiMountedPresentationWorkView};

fn consume_inert_mechanics(view: &UiMountedFrameConsumptionView<'_>) {
    match view.presentation_work() {
        UiMountedPresentationWorkView::Initial(initial) => {
            let _ = (initial.commands(), initial.order(), initial.damage());
        }
        UiMountedPresentationWorkView::Delta(delta) => {
            let _ = (delta.changes(), delta.order(), delta.damage());
        }
        UiMountedPresentationWorkView::Reconstruction(work) => {
            let _ = (work.commands(), work.order(), work.damage());
        }
        UiMountedPresentationWorkView::Unchanged(unchanged) => {
            let _ = unchanged.affinity();
        }
    }
}

fn main() {
    let _ = consume_inert_mechanics;
}
