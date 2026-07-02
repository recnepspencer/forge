use worth_ui::facade::compat::admission::{
    UiAdmissionBoundary, UiAdmissionTarget, UiSupportSnapshot,
};

fn main() {
    let _ = (
        None::<UiAdmissionBoundary<'static>>,
        None::<UiAdmissionTarget>,
        None::<UiSupportSnapshot>,
    );
}
