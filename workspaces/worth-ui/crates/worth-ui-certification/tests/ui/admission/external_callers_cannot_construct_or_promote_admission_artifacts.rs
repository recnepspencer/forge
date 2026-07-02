use worth_ui::facade::admission::{
    UiAdmissionDecision, UiAdmissionReport, UiAdmissionTarget, UiAdmissionWorld, UiSupportPosture,
    UiSupportSnapshot,
};

fn main() {
    let _ = UiSupportSnapshot::new(todo!(), UiSupportPosture::Supported);
    let support_snapshot: UiSupportSnapshot = todo!();
    let _ = UiAdmissionDecision::new(support_snapshot.clone(), todo!());
    let _ = UiAdmissionReport::from_decision(todo!());
    let _ = UiAdmissionTarget::declaration(todo!(), UiAdmissionWorld::authoritative());
}
