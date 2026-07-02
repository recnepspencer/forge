use worth_ui_runtime::facade::{UiAdmissionBoundary, UiAdmissionTarget, UiSupportSnapshot};

fn main() {
    let _ = (
        None::<UiAdmissionBoundary<'static>>,
        None::<UiAdmissionTarget>,
        None::<UiSupportSnapshot>,
    );
}
